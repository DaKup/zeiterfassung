#![warn(clippy::all)]
#![allow(unused)]

use crate::gui::{plot_durations, plot_timeframes, UpdateOutputsTrait};
use crate::processing::{AsMyStringTrait, ProjectType, TimeframeTrait, Update};
use crate::MainApp;
use chrono::{Duration, Timelike};
use egui::Ui;

use egui::plot::{Bar, BarChart, BoxElem, BoxPlot, BoxSpread, Legend, Line, Plot, PlotPoints};



pub fn central_panel(
    app: &mut MainApp,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    ui: &mut Ui,
) {
    // screen size:
    let available_width = ui.available_width();
    let available_height = ui.available_height();

    // layout:
    let num_columns = 3u32;

    app.state.update();
    app.outputs.update(&app.state);

    // ui:
    egui::ScrollArea::horizontal()
        .min_scrolled_width(available_width)
        .show(ui, |ui| {
            ui.style_mut().wrap = Some(false);

            ui.horizontal(|ui| {
                egui::ScrollArea::vertical()
                    .min_scrolled_height(available_height)
                    .show(ui, |ui| {
                        ui.style_mut().wrap = Some(false);

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
                                    egui::TextEdit::multiline(
                                        &mut app.outputs.parser.lines_of_interest,
                                    )
                                    .desired_rows(1)
                                    .desired_width(available_width / num_columns as f32),
                                );
                                ui.label("");
                                ui.label("Timestamps:").highlight();
                                ui.add(
                                    egui::TextEdit::multiline(
                                        &mut app.outputs.parser.parsed_timestamps,
                                    )
                                    .desired_rows(1)
                                    .desired_width(available_width / num_columns as f32),
                                );
                                ui.label("");
                                ui.label("Task Descriptions:").highlight();
                                ui.add(
                                    egui::TextEdit::multiline(
                                        &mut app.outputs.parser.parsed_descriptions,
                                    )
                                    .desired_rows(1)
                                    .desired_width(available_width / num_columns as f32),
                                );
                            }
                        });

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
                                        ui.selectable_value(
                                            &mut s.project_type,
                                            ProjectType::Unknown,
                                            "Unknown",
                                        );
                                        ui.selectable_value(
                                            &mut s.project_type,
                                            ProjectType::Break,
                                            "Break",
                                        );
                                        ui.selectable_value(
                                            &mut s.project_type,
                                            ProjectType::Id(0),
                                            "[Project_Name]",
                                        );
                                    });
                            }
                            ui.separator();

                            // ui.label(format!("Total: {} => {}", &sum, &rounded_sum)).highlight();
                            ui.label(format!("Total: {} => {}", &sum, &rounded_sum))
                                .highlight();

                        });
                    });
            });
        });
}
