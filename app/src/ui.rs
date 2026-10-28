//! The egui window: input form, verdict panel and disclaimer footer.

use eframe::egui::{self, Color32, RichText};

use crate::model::{
    self, Contribution, Education, Gender, Goal, HEIGHT_RANGE_CM, IncomeBand, MAX_AGE, MIN_AGE,
    Profile, ProfileError, Region, Verdict,
};
use crate::{gauge, memes};

/// Footer text shown on every frame.
const DISCLAIMER: &str = "For fun only. Not science, not advice.";

/// Colour for validation errors.
const ERROR_COLOR: Color32 = Color32::from_rgb(220, 90, 60);

/// Colour for contributions that raised the score.
const GAIN_COLOR: Color32 = Color32::from_rgb(80, 180, 100);

/// Colour for contributions that lowered the score.
const LOSS_COLOR: Color32 = Color32::from_rgb(220, 100, 80);

/// Height pre-filled when the user ticks the height box.
const DEFAULT_HEIGHT_CM: u16 = 172;

/// Application state.
pub struct MeterApp {
    /// What the user has entered so far.
    profile: Profile,
    /// Whether the optional height field is used.
    include_height: bool,
    /// Height value kept even while the box is unticked.
    height_cm: u16,
    /// Result of the last button press, if any.
    outcome: Option<Result<Verdict, ProfileError>>,
    /// Scroll the verdict into view on the next frame (set by a press, so the
    /// result is visible even in a small or tiled window).
    scroll_to_verdict: bool,
}

impl Default for MeterApp {
    fn default() -> Self {
        MeterApp {
            profile: Profile::default(),
            include_height: false,
            height_cm: DEFAULT_HEIGHT_CM,
            outcome: None,
            scroll_to_verdict: false,
        }
    }
}

impl MeterApp {
    /// An app whose form is pre-filled with `profile` and already scored, as
    /// if the button had been pressed (used by `--demo`).
    pub fn scored(profile: Profile) -> Self {
        MeterApp {
            include_height: profile.height_cm.is_some(),
            height_cm: profile.height_cm.unwrap_or(DEFAULT_HEIGHT_CM),
            outcome: Some(model::evaluate(&profile)),
            scroll_to_verdict: true,
            profile,
        }
    }
}

/// A combo box over a closed set of enum values.
fn enum_combo<T: Copy + PartialEq>(
    ui: &mut egui::Ui,
    id: &str,
    value: &mut T,
    all: &[T],
    label: fn(T) -> &'static str,
) {
    egui::ComboBox::from_id_salt(id)
        .selected_text(label(*value))
        .width(200.0)
        .show_ui(ui, |ui| {
            for &option in all {
                ui.selectable_value(value, option, label(option));
            }
        });
}

impl MeterApp {
    /// The input form. Returns `true` when the button was pressed.
    fn form(&mut self, ui: &mut egui::Ui) -> bool {
        egui::Grid::new("profile_form")
            .num_columns(2)
            .spacing([16.0, 8.0])
            .show(ui, |ui| {
                ui.label("Age");
                ui.add(egui::DragValue::new(&mut self.profile.age).range(MIN_AGE..=MAX_AGE));
                ui.end_row();

                ui.label("Gender");
                enum_combo(
                    ui,
                    "gender",
                    &mut self.profile.gender,
                    &Gender::ALL,
                    Gender::label,
                );
                ui.end_row();

                ui.label("Education");
                enum_combo(
                    ui,
                    "education",
                    &mut self.profile.education,
                    &Education::ALL,
                    Education::label,
                );
                ui.end_row();

                ui.label("Income band");
                enum_combo(
                    ui,
                    "income",
                    &mut self.profile.income,
                    &IncomeBand::ALL,
                    IncomeBand::label,
                );
                ui.end_row();

                ui.label("Height (optional)");
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.include_height, "");
                    ui.add_enabled(
                        self.include_height,
                        egui::DragValue::new(&mut self.height_cm)
                            .range(HEIGHT_RANGE_CM.0..=HEIGHT_RANGE_CM.1)
                            .suffix(" cm"),
                    );
                });
                ui.end_row();

                ui.label("Country / region");
                enum_combo(
                    ui,
                    "region",
                    &mut self.profile.region,
                    &Region::ALL,
                    Region::label,
                );
                ui.end_row();

                ui.label("Relationship goal");
                enum_combo(ui, "goal", &mut self.profile.goal, &Goal::ALL, Goal::label);
                ui.end_row();
            });

        ui.add_space(8.0);
        ui.button(RichText::new("Consult the market").size(18.0))
            .clicked()
    }

    /// The verdict panel: what to show before, after an error, or after a
    /// successful press.
    fn verdict(&self, ui: &mut egui::Ui) {
        match &self.outcome {
            None => {
                gauge::draw(ui, None);
                ui.label("Fill in the form and consult the market. It is very wise (it is not).");
            }
            Some(Err(err)) => {
                gauge::draw(ui, None);
                ui.colored_label(ERROR_COLOR, err.to_string());
            }
            Some(Ok(verdict)) => show_verdict(ui, verdict),
        }
    }
}

/// Gauge, headline, meme card and the inputs that moved the score.
fn show_verdict(ui: &mut egui::Ui, verdict: &Verdict) {
    gauge::draw(ui, Some(verdict.score));
    ui.heading(RichText::new(verdict.tier.headline()).strong().size(24.0));
    ui.label(RichText::new(verdict.tier.tagline()).italics());
    ui.add_space(8.0);
    memes::draw(ui, verdict.tier, verdict.gender);
    ui.add_space(8.0);
    ui.label(format!(
        "Market index: {:.1} / 100 (starts at {:.0}). What moved it:",
        verdict.score,
        model::BASE_SCORE
    ));
    for contribution in &verdict.contributions {
        contribution_row(ui, contribution);
    }
}

/// One line of the explanation: signed points, factor name, joke note.
fn contribution_row(ui: &mut egui::Ui, c: &Contribution) {
    let color = delta_color(ui, c.delta);
    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new(format!("{:+5.1}", c.delta))
                .monospace()
                .color(color),
        );
        ui.label(RichText::new(c.factor.label()).strong());
        ui.label(c.note);
    });
}

/// Green for points gained, red for points lost, grey for no effect.
fn delta_color(ui: &egui::Ui, delta: f32) -> Color32 {
    if delta > 0.0 {
        GAIN_COLOR
    } else if delta < 0.0 {
        LOSS_COLOR
    } else {
        ui.visuals().weak_text_color()
    }
}

impl eframe::App for MeterApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::bottom("disclaimer").show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new(DISCLAIMER).small().weak());
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(RichText::new("Hypergamy Meter").size(30.0).strong());
                    ui.label("Is hypergamy real? Probably a bit. Is this meter real? No.");
                });
                ui.separator();

                if self.form(ui) {
                    self.profile.height_cm = self.include_height.then_some(self.height_cm);
                    self.outcome = Some(model::evaluate(&self.profile));
                    self.scroll_to_verdict = true;
                }

                ui.separator();
                if std::mem::take(&mut self.scroll_to_verdict) {
                    ui.scroll_to_cursor(Some(egui::Align::TOP));
                }
                ui.vertical_centered(|ui| self.verdict(ui));
            });
        });
    }
}
