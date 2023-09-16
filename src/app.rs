use egui::TextBuffer;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;

use rfd::AsyncFileDialog;

use crate::platform;
use crate::processing::{self};
use crate::processing::{TimeframeTrait, Update};

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct MainApp {
    #[serde(skip)]
    state: processing::State,
}

impl MainApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

trait OnClickedButtonTrait {
    fn on_clicked_open(&mut self);
    fn on_clicked_save(&mut self);
}

impl OnClickedButtonTrait for MainApp {
    fn on_clicked_open(&mut self) {
        async fn run(markdown_content: Arc<Mutex<String>>, overwrite_input: Arc<AtomicBool>) {
            let file_handle = AsyncFileDialog::new()
                .add_filter("Markdown", &["md"])
                .pick_files()
                .await;

            if file_handle.is_none() {
                return;
            }

            let mut all_data = String::new();

            let file_handles = file_handle.unwrap();
            for file_handle in file_handles {
                let data = file_handle.read().await;
                let data = String::from_utf8_lossy(&data).to_string();
                all_data.push_str(&data);
                all_data.push_str("\n---\n");
            }
            *markdown_content.lock().unwrap() = all_data;
            overwrite_input.store(true, Ordering::Relaxed);
        }

        platform::spawn_async(run(
            self.state.markdown_content_backbuffer.clone(),
            self.state.overwrite_input.clone(),
        ));
    }

    fn on_clicked_save(&mut self) {
        async fn run() {
            platform::save_file("dummy_content".as_str().as_bytes(), "zeiterfassung.md").await;
        }

        platform::spawn_async(run());
    }
}

impl eframe::App for MainApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let Self { state } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        self.on_clicked_open();
                    }
                    if ui.button("Save").clicked() {
                        self.on_clicked_save();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let show_debug = true;

            // screen size:
            let available_width = ui.available_width();
            let available_height = ui.available_height();

            self.state.update();

            let mut lines_of_interest = self.state.log_lines.join("\n");

            let mut rounded_timestamps = self
                .state
                .rounded_timestamp_tasks
                .iter()
                .map(|x| {
                    let (a, _) = x;
                    a.to_string()
                })
                .collect::<Vec<_>>()
                .join("\n");

            let mut timestamps = self
                .state
                .timestamp_tasks
                .iter()
                .map(|x| {
                    let (a, _) = x;
                    a.to_string()
                })
                .collect::<Vec<_>>()
                .join("\n");

            let mut tasks = self
                .state
                .timestamp_tasks
                .iter()
                .map(|x| {
                    let (_, b) = x;
                    b.clone()
                })
                .collect::<Vec<_>>()
                .join("\n");

            let mut durations = self
                .state
                .durations
                .iter()
                .map(|duration| {
                    let time =
                        duration.num_hours() as f32 + (duration.num_minutes() % 60) as f32 / 60.0;
                    format!("{:.2}h", time)
                })
                .collect::<Vec<_>>()
                .join("\n");

            let mut rounded_durations = self
                .state
                .rounded_durations
                .iter()
                .map(|duration| {
                    let time =
                        duration.num_hours() as f32 + (duration.num_minutes() % 60) as f32 / 60.0;
                    format!("{:.2}h", time)
                })
                .collect::<Vec<_>>()
                .join("\n");

            let total_duration: f32 = self
                .state
                .durations
                .iter()
                .map(|duration| {
                    duration.num_hours() as f32 + (duration.num_minutes() % 60) as f32 / 60.0
                })
                .sum();
            let total_duration = format!("{:.2}h", total_duration);

            let total_rounded_duration: f32 = self
                .state
                .rounded_durations
                .iter()
                .map(|duration| {
                    duration.num_hours() as f32 + (duration.num_minutes() % 60) as f32 / 60.0
                })
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
                                        egui::TextEdit::multiline(&mut self.state.markdown_input)
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

                                    // self.state.tasks[0].description;
                                    // self.state.tasks[0].project;
                                    // self.state.tasks[0].timeframe;

                                    for (_i, e) in self.state.tasks.iter().enumerate() {
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
        });
    }
}
