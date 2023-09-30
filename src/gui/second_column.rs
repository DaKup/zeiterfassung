#![warn(clippy::all)]
#![allow(unused)]

use crate::gui::{plot_durations, plot_timeframes};
use crate::processing::{AsMyStringTrait, ProjectType, TimeframeTrait};
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

        // ui.checkbox(&mut app.state.rounded_plots, "edit")
        //     .highlight();

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
            available_height / 8.0,
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
            available_height / 8.0,
            app.state.rounded_plots,
        );

        ui.label("Tasks").highlight();

        ui.label("");

        let mut sum = 0;
        let mut rounded_sum = 0;

        let num_tasks = app.state.tasks.len();

        for (_i, (e, s)) in app
            .state
            .tasks
            .iter()
            .zip(app.state.task_states.iter_mut())
            .enumerate()
        {
            // ui.label("");
            // ui.separator();
            // ui.add(
            //     egui::widgets::Separator::default().spacing(available_width / num_columns as f32)
            // );

            ui.label(format!(
                "{} => {}",
                e.timeframe.begin(),
                e.timeframe.round().begin()
            ));
            ui.label(format!(
                "{} => {}",
                e.timeframe.end(),
                e.timeframe.round().end()
            ));
            ui.label(format!(
                "{} => {}",
                e.timeframe.duration().to_my_string(),
                e.timeframe.round().duration().to_my_string()
            ));
            ui.label(format!("{}: {}", &e.project, &e.description))
                .highlight();

            ui.label("");
            // let mut selected = ProjectType::Unknown;
            egui::ComboBox::from_label(e.description.clone())
                .selected_text(format!("{:?}", s.project_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut s.project_type, ProjectType::Unknown, "Unknown");
                    ui.selectable_value(&mut s.project_type, ProjectType::Break, "Break");
                    ui.selectable_value(&mut s.project_type, ProjectType::Id(0), "[Project_Name]");
                });
        }
        // ui.separator();

        // ui.label(format!("Total: {} => {}", &sum, &rounded_sum)).highlight();
        ui.label(format!("Total: {} => {}", &sum, &rounded_sum))
            .highlight();

        ui.label("");
    });
}
