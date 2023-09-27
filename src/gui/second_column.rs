#![warn(clippy::all)]
#![allow(unused)]

use crate::gui::{plot_durations, plot_timeframes};
use crate::MainApp;
use egui::Ui;

pub fn second_column(
    app: &mut MainApp,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    ui: &mut Ui,
    available_width: f32,
    available_height: f32,
    num_columns: u32,
) {
    // second column: processing
    ui.vertical(|ui| {
        // ui.label("Processing");

        // ui.selectable_label(bla, "test this");
        // ui.selectable_label(app.state.rounded_plots, "round").highlight();
        // ui.
        ui.checkbox(&mut app.state.rounded_plots, "round")
            .highlight();

        ui.checkbox(&mut app.state.log_scale, "log scale")
            .highlight();

        ui.label("");
        ui.label("Durations");
        plot_durations(
            app,
            ctx,
            frame,
            ui,
            available_width / num_columns as f32,
            available_height / 3.0,
        );

        ui.label("");
        ui.label("");
        ui.label("Timeframes:");

        plot_timeframes(
            app,
            ctx,
            frame,
            ui,
            available_width / num_columns as f32,
            available_height / 3.0,
            app.state.rounded_plots,
        );
    });
}
