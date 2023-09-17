#![warn(clippy::all)]
#![allow(unused)]

use crate::processing::{TimeframeTrait, Update};
use crate::MainApp;
use egui::Ui;

pub fn central_panel(
    app: &mut MainApp,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    ui: &mut Ui,
) {
    let show_debug = true;

    // screen size:
    let available_width = ui.available_width();
    let available_height = ui.available_height();

    app.state.update();

    let mut lines_of_interest = app.state.log_lines.join("\n");

    let mut rounded_timestamps = app
        .state
        .rounded_timestamp_tasks
        .iter()
        .map(|x| {
            let (a, _) = x;
            a.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut timestamps = app
        .state
        .timestamp_tasks
        .iter()
        .map(|x| {
            let (a, _) = x;
            a.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut tasks = app
        .state
        .timestamp_tasks
        .iter()
        .map(|x| {
            let (_, b) = x;
            b.clone()
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut durations = app
        .state
        .durations
        .iter()
        .map(|duration| {
            let time = duration.num_hours() as f32 + (duration.num_minutes() % 60) as f32 / 60.0;
            format!("{:.2}h", time)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut rounded_durations = app
        .state
        .rounded_durations
        .iter()
        .map(|duration| {
            let time = duration.num_hours() as f32 + (duration.num_minutes() % 60) as f32 / 60.0;
            format!("{:.2}h", time)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let total_duration: f32 = app
        .state
        .durations
        .iter()
        .map(|duration| duration.num_hours() as f32 + (duration.num_minutes() % 60) as f32 / 60.0)
        .sum();
    let total_duration = format!("{:.2}h", total_duration);

    let total_rounded_duration: f32 = app
        .state
        .rounded_durations
        .iter()
        .map(|duration| duration.num_hours() as f32 + (duration.num_minutes() % 60) as f32 / 60.0)
        .sum();
    let total_rounded_duration = format!("{:.2}h", total_rounded_duration);

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
                            ui.label("Parsing");

                            ui.label("");
                            ui.label("Markdown Input:");
                            ui.add(
                                egui::TextEdit::multiline(&mut app.state.markdown_input)
                                    .desired_rows(1)
                                    .desired_width(available_width / 3.0),
                            );

                            // parser debug output:
                            if show_debug {
                                ui.label("");
                                ui.label("Lines of Interest:");
                                ui.add(
                                    egui::TextEdit::multiline(&mut lines_of_interest)
                                        .desired_rows(1)
                                        .desired_width(available_width / 3.0),
                                );
                                ui.label("");
                                ui.label("Timestamps:");
                                ui.add(
                                    egui::TextEdit::multiline(&mut timestamps)
                                        .desired_rows(1)
                                        .desired_width(available_width / 3.0),
                                );
                                ui.label("");
                                ui.label("Tasks:");
                                ui.add(
                                    egui::TextEdit::multiline(&mut tasks)
                                        .desired_rows(1)
                                        .desired_width(available_width / 3.0),
                                );
                            }
                        });

                        // second column: processing
                        ui.vertical(|ui| {
                            ui.label("Processing");

                            ui.label("");
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label("Timestamps:");
                                    ui.add(
                                        egui::TextEdit::multiline(&mut timestamps)
                                            .desired_rows(1)
                                            .desired_width(available_width / 3.0 / 2.0),
                                    );
                                });
                                ui.vertical(|ui| {
                                    ui.label("Durations:");
                                    ui.add(
                                        egui::TextEdit::multiline(&mut durations)
                                            .desired_rows(1)
                                            .desired_width(available_width / 3.0 / 2.0),
                                    );
                                });
                            });
                            ui.label(format!("total: {}", total_duration));

                            ui.label("");
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label("Rounded Timestamps:");
                                    ui.add(
                                        egui::TextEdit::multiline(&mut rounded_timestamps)
                                            .desired_rows(1)
                                            .desired_width(available_width / 3.0 / 2.0),
                                    );
                                });
                                ui.vertical(|ui| {
                                    ui.label("Rounded Durations:");
                                    ui.add(
                                        egui::TextEdit::multiline(&mut rounded_durations)
                                            .desired_rows(1)
                                            .desired_width(available_width / 3.0 / 2.0),
                                    );
                                });
                            });
                            ui.label(format!("total: {}", total_rounded_duration));
                        });

                        // third column: results
                        ui.vertical(|ui| {
                            ui.label("Results");

                            ui.label("");
                            ui.label("Results:");
                            ui.add(
                                egui::TextEdit::multiline(&mut rounded_timestamps)
                                    .desired_rows(1)
                                    .desired_width(available_width / 3.0),
                            );

                            // app.state.tasks[0].description;
                            // app.state.tasks[0].project;
                            // app.state.tasks[0].timeframe;

                            for (_i, e) in app.state.tasks.iter().enumerate() {
                                ui.label(e.timeframe.round().begin().to_string());
                                ui.label(e.timeframe.round().end().to_string());
                                ui.label(e.timeframe.round().duration().to_string());
                                ui.label(&e.project);
                                ui.label(&e.description);
                            }

                            ui.label("");
                            ui.label("Durations:");
                            ui.add(
                                egui::TextEdit::multiline(&mut rounded_timestamps)
                                    .desired_rows(1)
                                    .desired_width(available_width / 3.0),
                            );
                        });
                    });
            });
        });
}
