#![warn(clippy::all)]
#![allow(unused)]

use crate::MainApp;
use egui::Ui;

pub fn first_column(
    app: &mut MainApp,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    ui: &mut Ui,
    available_width: f32,
    available_height: f32,
    num_columns: u32,
) {
    // first column: input
    ui.vertical(|ui| {
        ui.label("Markdown Input:").highlight();
        ui.add(
            egui::TextEdit::multiline(&mut app.state.markdown_input)
                .desired_rows(1)
                .desired_width(available_width / num_columns as f32),
        );

        // parser debug output:
        ui.add(egui::Checkbox::new(&mut app.state.show_debug, "Show Debug"));
        if app.state.show_debug {
            ui.label("");
            ui.label("Lines of Interest:").highlight();
            ui.add(
                egui::TextEdit::multiline(&mut app.outputs.parser.lines_of_interest)
                    .desired_rows(1)
                    .desired_width(available_width / num_columns as f32),
            );
            ui.label("");
            ui.label("Timestamps:").highlight();
            ui.add(
                egui::TextEdit::multiline(&mut app.outputs.parser.parsed_timestamps)
                    .desired_rows(1)
                    .desired_width(available_width / num_columns as f32),
            );
            ui.label("");
            ui.label("Task Descriptions:").highlight();
            ui.add(
                egui::TextEdit::multiline(&mut app.outputs.parser.parsed_descriptions)
                    .desired_rows(1)
                    .desired_width(available_width / num_columns as f32),
            );
        }
    });
}
