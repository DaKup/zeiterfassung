#![warn(clippy::all)]
#![allow(unused)]

use crate::gui::{plot_durations, plot_timeframes};
use crate::processing::{AsMyStringTrait, ProjectType, TimeframeTrait};
use crate::MainApp;
use egui::Ui;

pub fn third_column(
    app: &mut MainApp,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    ui: &mut Ui,
    available_width: f32,
    available_height: f32,
    num_columns: u32,
) {
    // third column: results
    ui.vertical(|ui| {
        ui.label("Summary").highlight();

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
            ui.separator();
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
        ui.separator();

        // ui.label(format!("Total: {} => {}", &sum, &rounded_sum)).highlight();
        ui.label(format!("Total: {} => {}", &sum, &rounded_sum))
            .highlight();
    });
}
