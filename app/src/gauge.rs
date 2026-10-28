//! The semicircular "market meter", drawn with the egui painter.

use eframe::egui::{self, Align2, Color32, FontId, Pos2, Sense, Shape, Stroke, Vec2};
use std::f32::consts::PI;

use crate::model::SCORE_RANGE;

/// Number of coloured slices in the dial.
const SEGMENTS: usize = 60;

/// Colour for a position along the dial, `t` in `0..=1` (red to green).
fn dial_color(t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    // Red -> amber -> green, linear in two halves.
    let (r, g) = if t < 0.5 {
        (230.0, 70.0 + 300.0 * t)
    } else {
        (230.0 - 360.0 * (t - 0.5), 220.0)
    };
    Color32::from_rgb(r as u8, g as u8, 80)
}

/// Point on a circle around `center` at `angle` (radians, 0 = right, PI = left,
/// measured upwards because screen y grows downwards).
fn polar(center: Pos2, radius: f32, angle: f32) -> Pos2 {
    center + Vec2::new(radius * angle.cos(), -radius * angle.sin())
}

/// Maps a score to the needle angle: lowest score points left (PI), highest
/// points right (0).
pub fn needle_angle(score: f32) -> f32 {
    let (lo, hi) = SCORE_RANGE;
    let t = ((score - lo) / (hi - lo)).clamp(0.0, 1.0);
    PI * (1.0 - t)
}

/// Draws the meter with the needle at `score` (or with no needle for `None`).
pub fn draw(ui: &mut egui::Ui, score: Option<f32>) {
    let width = ui.available_width().min(420.0);
    let (response, painter) = ui.allocate_painter(Vec2::new(width, width * 0.58), Sense::hover());
    let rect = response.rect;
    let radius = width * 0.42;
    let center = Pos2::new(rect.center().x, rect.top() + radius + 16.0);
    let thickness = radius * 0.22;

    for i in 0..SEGMENTS {
        let t0 = i as f32 / SEGMENTS as f32;
        let t1 = (i + 1) as f32 / SEGMENTS as f32;
        let a0 = PI * (1.0 - t0);
        let a1 = PI * (1.0 - t1);
        let quad = vec![
            polar(center, radius, a0),
            polar(center, radius, a1),
            polar(center, radius - thickness, a1),
            polar(center, radius - thickness, a0),
        ];
        painter.add(Shape::convex_polygon(
            quad,
            dial_color((t0 + t1) / 2.0),
            Stroke::NONE,
        ));
    }

    let text_color = ui.visuals().text_color();
    let small = FontId::proportional(12.0);
    for (label, t, align) in [
        ("doomed", 0.0, Align2::RIGHT_CENTER),
        ("meh", 0.5, Align2::CENTER_BOTTOM),
        ("iconic", 1.0, Align2::LEFT_CENTER),
    ] {
        let pos = polar(center, radius + 12.0, PI * (1.0 - t));
        painter.text(pos, align, label, small.clone(), text_color);
    }

    if let Some(score) = score {
        let tip = polar(center, radius - thickness * 0.4, needle_angle(score));
        painter.line_segment([center, tip], Stroke::new(4.0, text_color));
        painter.circle_filled(center, 8.0, text_color);
        painter.text(
            center + Vec2::new(0.0, -radius * 0.35),
            Align2::CENTER_CENTER,
            format!("{score:.0}"),
            FontId::proportional(radius * 0.32),
            text_color,
        );
    } else {
        painter.circle_filled(center, 8.0, text_color);
        painter.text(
            center + Vec2::new(0.0, -radius * 0.35),
            Align2::CENTER_CENTER,
            "?",
            FontId::proportional(radius * 0.32),
            text_color,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn needle_spans_the_half_circle() {
        assert!((needle_angle(0.0) - PI).abs() < 1e-6);
        assert!((needle_angle(50.0) - PI / 2.0).abs() < 1e-6);
        assert!(needle_angle(100.0).abs() < 1e-6);
        assert!(needle_angle(-5.0) <= PI && needle_angle(500.0) >= 0.0);
    }
}
