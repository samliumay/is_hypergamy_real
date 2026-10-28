//! Meme cards: one per verdict tier, drawn by the app itself. Only the lowest
//! tier differs by gender, after the internet's own "it's over" memes.
//!
//! Classic layout: a coloured background with a big emoji "picture", bold
//! white TOP TEXT and BOTTOM TEXT with a black outline. No images are
//! embedded or downloaded; everything is egui shapes and text.

use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Pos2, Sense, Vec2};

use crate::model::{Gender, Tier};

/// Everything needed to draw one meme card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Meme {
    /// Caption at the top of the card (drawn in capitals).
    pub top: &'static str,
    /// Caption at the bottom of the card (drawn in capitals).
    pub bottom: &'static str,
    /// The "picture": an emoji drawn large in the middle.
    pub emoji: &'static str,
    /// Background colour as RGB.
    pub background: [u8; 3],
    /// Short "haha" line shown under the card.
    pub joke: &'static str,
}

/// The meme table: one card per tier (the lowest one per gender). Exhaustive
/// on purpose, so a new tier or gender cannot ship without a meme.
pub const fn meme_for(tier: Tier, gender: Gender) -> Meme {
    match tier {
        Tier::TopOfTheMarket => Meme {
            top: "Top of the market",
            bottom: "Stonks only go up",
            emoji: "😎",
            background: [40, 120, 70],
            joke: "Haha, enjoy it. The index resets when you leave the dishes in the sink.",
        },
        Tier::HighlyCompetitive => Meme {
            top: "Strong buy",
            bottom: "Analysts are blushing",
            emoji: "🚀",
            background: [40, 90, 150],
            joke: "Haha, a rocket. Remember to text back within the same business day.",
        },
        Tier::SolidContender => Meme {
            top: "Not bad",
            bottom: "Not bad at all",
            emoji: "👍",
            background: [110, 90, 160],
            joke: "Haha, the average is where most happy couples live anyway.",
        },
        Tier::NicheButLoved => Meme {
            top: "Acquired taste",
            bottom: "Like olives. Or jazz.",
            emoji: "🎷",
            background: [170, 110, 40],
            joke: "Haha, you only need one buyer. This is not an index fund.",
        },
        Tier::ClearanceAisle => Meme {
            top: "Clearance aisle",
            bottom: "Still a bargain",
            emoji: "🛒",
            background: [160, 70, 60],
            joke: "Haha, discounted by a model that has never heard you tell a joke.",
        },
        Tier::LifeIsOver => rock_bottom(gender),
    }
}

/// The lowest-tier card. Men get the classic "it's over"; women get the
/// "single mother arc" version of the same meme. The joke line always
/// undercuts it, because the model is five inputs and a sum.
const fn rock_bottom(gender: Gender) -> Meme {
    match gender {
        Gender::Man => Meme {
            top: "It's over",
            bottom: "...just kidding, go touch grass :D",
            emoji: "🌱",
            background: [60, 60, 60],
            joke: "Haha, a few dropdowns and an additive model cannot end anything. Go outside.",
        },
        Gender::Woman => Meme {
            top: "Single mother arc",
            bottom: "...or it's over. Kidding, go touch grass :D",
            emoji: "🍼",
            background: [90, 60, 90],
            joke: "Haha, the forums say so. The forums also think the 80/20 rule is real.",
        },
        Gender::NonBinary => Meme {
            top: "It's over (all genders edition)",
            bottom: "...just kidding, go touch grass :D",
            emoji: "🌱",
            background: [60, 60, 60],
            joke: "Haha, the model has no data on you, so even its doom is made up.",
        },
    }
}

/// Draws `text` centred at `pos` in white with a black outline (the classic
/// meme font look), by stamping the text in black at small offsets first.
fn outlined_text(painter: &egui::Painter, pos: Pos2, anchor: Align2, text: &str, size: f32) {
    let font = FontId::proportional(size);
    let width = (size / 14.0).max(1.0);
    for (dx, dy) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)] {
        let offset = Vec2::new(dx * width, dy * width);
        painter.text(pos + offset, anchor, text, font.clone(), Color32::BLACK);
    }
    painter.text(pos, anchor, text, font, Color32::WHITE);
}

/// Font size that makes `text` fit on one line of a card `width` wide.
/// Uses a rough average glyph width of 0.62 em for bold-ish capitals and
/// caps the size so short captions do not become huge.
fn caption_size(width: f32, text: &str) -> f32 {
    let chars = text.chars().count().max(1) as f32;
    (width * 0.92 / (chars * 0.62)).min(width * 0.08)
}

/// Draws the meme card for `tier` and `gender`, then its joke line underneath.
pub fn draw(ui: &mut egui::Ui, tier: Tier, gender: Gender) {
    let meme = meme_for(tier, gender);
    let width = ui.available_width().min(420.0);
    let size = Vec2::new(width, width * 0.6);
    let (response, painter) = ui.allocate_painter(size, Sense::hover());
    let rect = response.rect;
    let [r, g, b] = meme.background;

    painter.rect_filled(rect, CornerRadius::same(8), Color32::from_rgb(r, g, b));
    painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        meme.emoji,
        FontId::proportional(rect.height() * 0.4),
        Color32::WHITE,
    );

    let top = meme.top.to_uppercase();
    let bottom = meme.bottom.to_uppercase();
    let margin = width * 0.03;
    outlined_text(
        &painter,
        rect.center_top() + Vec2::new(0.0, margin),
        Align2::CENTER_TOP,
        &top,
        caption_size(width, &top),
    );
    outlined_text(
        &painter,
        rect.center_bottom() - Vec2::new(0.0, margin),
        Align2::CENTER_BOTTOM,
        &bottom,
        caption_size(width, &bottom),
    );

    ui.label(egui::RichText::new(meme.joke).italics());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_tier_has_a_distinct_meme() {
        let memes: Vec<Meme> = Tier::THRESHOLDS
            .iter()
            .map(|(t, _)| meme_for(*t, Gender::Man))
            .collect();
        for (i, a) in memes.iter().enumerate() {
            assert!(!a.top.is_empty() && !a.bottom.is_empty() && !a.joke.is_empty());
            assert!(memes[i + 1..].iter().all(|b| b.top != a.top));
        }
    }

    #[test]
    fn long_captions_get_smaller_fonts() {
        let short = caption_size(400.0, "NOT BAD");
        let long = caption_size(400.0, "...JUST KIDDING, GO TOUCH GRASS :D");
        assert!(long < short);
        assert!(short <= 400.0 * 0.08);
    }

    #[test]
    fn lowest_tier_is_an_obvious_joke() {
        for gender in Gender::ALL {
            let meme = meme_for(Tier::LifeIsOver, gender);
            assert!(meme.bottom.to_lowercase().contains("kidding"));
        }
    }

    #[test]
    fn lowest_tier_meme_depends_on_gender() {
        assert_eq!(meme_for(Tier::LifeIsOver, Gender::Man).top, "It's over");
        assert_eq!(
            meme_for(Tier::LifeIsOver, Gender::Woman).top,
            "Single mother arc"
        );
        // Every other tier is the same for everyone.
        assert_eq!(
            meme_for(Tier::SolidContender, Gender::Man),
            meme_for(Tier::SolidContender, Gender::Woman)
        );
    }
}
