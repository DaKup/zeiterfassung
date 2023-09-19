#![warn(clippy::all)]
#![allow(unused)]

use crate::gui::UpdateOutputsTrait;
use crate::processing::{AsMyStringTrait, ProjectType, TimeframeTrait, Update};
use crate::MainApp;
use chrono::{Duration, Timelike};
use egui::Ui;

use egui::plot::{Bar, BarChart, BoxElem, BoxPlot, BoxSpread, Legend, Line, Plot, PlotPoints};

pub fn plot_timeframes(
    app: &mut MainApp,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    ui: &mut Ui,
    width: f32,
    height: f32,
    rounded: bool,
) {
    let bars: Vec<BoxElem> = app
        .state
        .tasks
        .iter()
        .enumerate()
        .map(|(i, task)| {
            let project_id = i;
            let project_id = 0;

            // let lower_whisker= task.timeframe.begin().timestamp() as f64;
            // let upper_whisker= task.timeframe.end().timestamp() as f64;

            // let (lower_whisker, upper_whisker) = match app.state.rounded_plots {
            //     false => (task.timeframe.begin().timestamp() as f64, task.timeframe.end().timestamp() as f64),
            //     true => (task.timeframe.round().begin().timestamp() as f64, task.timeframe.round().end().timestamp() as f64)
            // };

            let (begin, end) = match app.state.rounded_plots {
                false => (task.timeframe.begin(), task.timeframe.end()),
                true => (task.timeframe.round().begin(), task.timeframe.round().end()),
            };

            let lower_whisker = (begin.hour() * 60 + begin.minute()) as f64;
            let upper_whisker = (end.hour() * 60 + end.minute()) as f64;

            let (lower_whisker, upper_whisker) = match app.state.log_scale {
                false => (lower_whisker, upper_whisker),
                true => (lower_whisker.log10(), upper_whisker.log10()),
            };

            let quartille1 = lower_whisker;
            let median = (lower_whisker + upper_whisker) / 2.0;
            let quartille3 = upper_whisker;

            BoxElem::new(
                project_id as f64,
                BoxSpread::new(lower_whisker, quartille1, median, quartille3, upper_whisker),
            )
            .name(task.description.clone())
        })
        .collect();

    // let data_aspect = match app.state.log_scale {
    //     false => 1.01f64,
    //     true => (1.0f64).log10(),
    // };

    Plot::new("Timeframes")
        .view_aspect(0.5)
        .width(width)
        .height(height)
        .legend(Legend::default())
        .data_aspect(0.1)
        .show(ui, |plot_ui| {
            for box_elem in bars.iter() {
                let box_elem = box_elem.clone();
                let boxplot = BoxPlot::new(vec![box_elem]).horizontal();
                plot_ui.box_plot(boxplot);
            }
        });
}

pub fn plot_durations(
    app: &mut MainApp,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    ui: &mut Ui,
    width: f32,
    height: f32,
) {
    let bars: Vec<Bar> = app
        .state
        .tasks
        .iter()
        .enumerate()
        .map(|(i, task)| {
            let project_id = i;
            let mut height = 0f64;

            let duration = match app.state.rounded_plots {
                true => task.timeframe.round().duration().num_minutes(),
                false => task.timeframe.duration().num_minutes(),
            };

            if duration > 0 && app.state.log_scale {
                height = (duration as f64).log10();
            }

            Bar::new(project_id as f64, height).name(task.description.clone())
        })
        .collect();

    Plot::new("Durations")
        .view_aspect(0.5)
        .width(width)
        .height(height)
        .legend(Legend::default())
        .data_aspect(0.3)
        .show(ui, |plot_ui| {
            for bar in bars.iter() {
                let barchart = BarChart::new(vec![bar.clone()]);
                plot_ui.bar_chart(barchart);
            }
        });
}

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

    // let mut lines_of_interest = app.state.log_lines.join("\n");
    //
    // let mut rounded_timestamps = app
    //     .state
    //     .rounded_timestamp_tasks
    //     .iter()
    //     .map(|x| {
    //         let (a, _) = x;
    //         a.to_string()
    //     })
    //     .collect::<Vec<_>>()
    //     .join("\n");
    //
    // let mut timestamps = app
    //     .state
    //     .timestamp_tasks
    //     .iter()
    //     .map(|x| {
    //         let (a, _) = x;
    //         a.to_string()
    //     })
    //     .collect::<Vec<_>>()
    //     .join("\n");
    //
    // let mut tasks = app
    //     .state
    //     .timestamp_tasks
    //     .iter()
    //     .map(|x| {
    //         let (_, b) = x;
    //         b.clone()
    //     })
    //     .collect::<Vec<_>>()
    //     .join("\n");
    //
    // let mut durations = app
    //     .state
    //     .durations
    //     .iter()
    //     .map(|duration| {
    //         let time = duration.num_hours() as f32 + (duration.num_minutes() % 60) as f32 / 60.0;
    //         format!("{:.2}h", time)
    //     })
    //     .collect::<Vec<_>>()
    //     .join("\n");
    //
    // let mut rounded_durations = app
    //     .state
    //     .rounded_durations
    //     .iter()
    //     .map(|duration| {
    //         let time = duration.num_hours() as f32 + (duration.num_minutes() % 60) as f32 / 60.0;
    //         format!("{:.2}h", time)
    //     })
    //     .collect::<Vec<_>>()
    //     .join("\n");
    //
    // let total_duration: f32 = app
    //     .state
    //     .durations
    //     .iter()
    //     .map(|duration| duration.num_hours() as f32 + (duration.num_minutes() % 60) as f32 / 60.0)
    //     .sum();
    // let total_duration = format!("{:.2}h", total_duration);
    //
    // let total_rounded_duration: f32 = app
    //     .state
    //     .rounded_durations
    //     .iter()
    //     .map(|duration| duration.num_hours() as f32 + (duration.num_minutes() % 60) as f32 / 60.0)
    //     .sum();
    // let total_rounded_duration = format!("{:.2}h", total_rounded_duration);

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

                            // let test = Bar::new(3.0, 1.0);
                            // let bars1: Vec<Bar> = vec![
                            //     Bar::new(1.0, 2.0).name("test1"),
                            //     Bar::new(1.0, 4.0).name("test2"),
                            // ];
                            // let bars2: Vec<Bar> = vec![Bar::new(3.0, 1.0), Bar::new(2.0, 4.0)];
                            // let bars3: Vec<Bar> = vec![Bar::new(3.0, 1.0), Bar::new(2.0, 4.0)];
                            // let bar_chart1 = egui::plot::BarChart::new(bars1).horizontal();
                            // let bar_chart2 = egui::plot::BarChart::new(bars2).horizontal();
                            // let bar_chart3 = egui::plot::BarChart::new(bars3).horizontal();
                            //
                            // Plot::new("my_plot")
                            //     .view_aspect(0.5)
                            //     .width(available_width / num_columns as f32)
                            //     .legend(Legend::default())
                            //     // .data_aspect(1.0)
                            //     .show(ui, |plot_ui| {
                            //         plot_ui.bar_chart(bar_chart1);
                            //         plot_ui.bar_chart(bar_chart2);
                            //         plot_ui.bar_chart(bar_chart3);
                            //     });

                            // let bla: bool = false;

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

                            // ui.label("");
                            // ui.horizontal(|ui| {
                            //     ui.vertical(|ui| {
                            //         ui.label("Timestamps:");
                            //         ui.add(
                            //             egui::TextEdit::multiline(
                            //                 &mut app.outputs.parser.parsed_timestamps,
                            //             )
                            //             .desired_rows(1)
                            //             .desired_width(available_width / num_columns as f32 / 2.0),
                            //         );
                            //     });
                            //     ui.vertical(|ui| {
                            //         ui.label("Durations:");
                            //         ui.add(
                            //             egui::TextEdit::multiline(
                            //                 &mut app.outputs.processing.durations_tasks,
                            //             )
                            //             .desired_rows(1)
                            //             .desired_width(available_width / num_columns as f32 / 2.0),
                            //         );
                            //     });
                            // });
                            // ui.label(format!("total: {}", app.outputs.results.total_durations));
                            //
                            // ui.label("");
                            // ui.horizontal(|ui| {
                            //     ui.vertical(|ui| {
                            //         ui.label("Rounded Timestamps:");
                            //         ui.add(
                            //             egui::TextEdit::multiline(
                            //                 &mut app.outputs.results.rounded_timestamp_descriptions,
                            //             )
                            //             .desired_rows(1)
                            //             .desired_width(available_width / num_columns as f32 / 2.0),
                            //         );
                            //     });
                            //     ui.vertical(|ui| {
                            //         ui.label("Rounded Durations:");
                            //         ui.add(
                            //             egui::TextEdit::multiline(
                            //                 &mut app.outputs.results.rounded_durations_tasks,
                            //             )
                            //             .desired_rows(1)
                            //             .desired_width(available_width / num_columns as f32 / 2.0),
                            //         );
                            //     });
                            // });
                            // ui.label(format!(
                            //     "total: {}",
                            //     app.outputs.results.total_rounded_durations
                            // ));
                        });

                        // third column: results
                        ui.vertical(|ui| {
                            ui.label("Summary").highlight();

                            ui.label("");
                            // ui.label("Results:");
                            // ui.add(
                            //     egui::TextEdit::multiline(
                            //         &mut app.outputs.results.rounded_timestamp_descriptions,
                            //     )
                            //     .desired_rows(1)
                            //     .desired_width(available_width / num_columns as f32),
                            // );

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

                            // ui.label("");
                            // ui.label("Durations:");
                            // ui.add(
                            //     egui::TextEdit::multiline(
                            //         &mut app.outputs.results.rounded_timestamp_descriptions,
                            //     )
                            //     .desired_rows(1)
                            //     .desired_width(available_width / num_columns as f32),
                            // );
                        });
                    });
            });
        });
}
