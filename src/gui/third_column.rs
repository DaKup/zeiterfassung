#![warn(clippy::all)]

use crate::MainApp;
use egui::Ui;

pub fn third_column(
    app: &mut MainApp,
    _ctx: &egui::Context,
    _frame: &mut eframe::Frame,
    ui: &mut Ui,
    available_width: f32,
    _available_height: f32,
    num_columns: u32,
) {
    // third column: results
    ui.vertical(|ui| {
        ui.label("Summary:").highlight();
        ui.add(
            egui::TextEdit::multiline(&mut app.state.summary)
                .desired_rows(1)
                .desired_width(available_width / num_columns as f32),
        );
    });
}
