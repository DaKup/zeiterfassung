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
        ui.label("Projects:").highlight();
        ui.add(
            egui::TextEdit::multiline(&mut app.state.projects_input)
                .desired_rows(1)
                .desired_width(available_width / num_columns as f32),
        );

        ui.label("Markdown Input:").highlight();

        let mut theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
        // ui.collapsing("Theme", |ui| {
        //     ui.group(|ui| {
        //         theme.ui(ui);
        //         theme.clone().store_in_memory(ui.ctx());
        //     });
        // });

        let language = "md";

        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            let mut layout_job =
                egui_extras::syntax_highlighting::highlight(ui.ctx(), &theme, string, language);
            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };

        ui.add(
            egui::TextEdit::multiline(&mut app.state.markdown_input)
                .code_editor()
                .desired_rows(1)
                .desired_width(available_width / num_columns as f32)
                .layouter(&mut layouter),
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
