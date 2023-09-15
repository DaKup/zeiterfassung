use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;

use rfd::AsyncFileDialog;

use crate::platform;
use crate::processing::parse_date;
use crate::processing::{self, extract_log_lines, parse_log_lines, round_timestamp_tasks};
// use async_trait::async_trait;

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

trait OnClickedOpen {
    fn on_clicked_open(&mut self);
}

impl OnClickedOpen for MainApp {
    fn on_clicked_open(&mut self) {
        async fn run(
            /*Arc<Mutex<String>> markdown_content*/
            markdown_content: Arc<Mutex<String>>,
            overwrite_input: Arc<AtomicBool>,
        ) {
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
            self.state.markdown_content_backbuffer.clone(), /*self.state.markdown_text.clone() */
            self.state.overwrite_input.clone(),
        ));
    }
}

fn _on_clicked_save() {
    async fn run() {
        let data = "";
        // serde_jsonc::to_string_pretty(&gpc.lock().unwrap().geometry_layer.filename).unwrap();

        #[cfg(not(target_arch = "wasm32"))]
        {
            let file_handle = AsyncFileDialog::new()
                .add_filter("gpc", &["gpc.json"])
                .save_file()
                .await;

            if file_handle.is_some() {
                let path = file_handle.unwrap();
                let path = path.path();

                let mut file = std::fs::File::create(path).unwrap();
                std::io::Write::write_all(&mut file, data.as_bytes()).unwrap();
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            platform::web::download_bytes(data.as_bytes(), "test.gpc.json");
        }
    }

    platform::spawn_async(run());
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
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // new files were opened:
            if self.state.overwrite_input.load(Ordering::Relaxed) {
                self.state.overwrite_input.store(false, Ordering::Relaxed);
                self.state.markdown_input = self
                    .state
                    .markdown_content_backbuffer
                    .lock()
                    .unwrap()
                    .to_string();
            }

            // screen size:
            let available_width = ui.available_width();
            let available_height = ui.available_height();

            // parse:
            // input = self.state.markdown_input;
            self.state.log_lines = extract_log_lines(&self.state.markdown_input);
            self.state.timestamp_tasks = parse_log_lines(&self.state.log_lines);
            self.state.rounded_timestamp_tasks = round_timestamp_tasks(&self.state.timestamp_tasks);

            let rounded_timestamps: Vec<String> = self
                .state
                .rounded_timestamp_tasks
                .iter()
                .map(|x| {
                    let (a, _) = x;

                    a.to_string()
                })
                .collect();

            let mut rounded_timestamps = rounded_timestamps.join("\n");

            // parse old:
            let mut parsed_markdown =
                processing::parse_markdown(self.state.markdown_input.as_str());
            let (mut timestamps, mut tasks) = processing::parse_log(parsed_markdown.as_str());
            let mut duration = parse_date(&timestamps);

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
                                ui.vertical(|ui| {
                                    ui.add(
                                        egui::TextEdit::multiline(&mut self.state.markdown_input)
                                            .desired_rows(1)
                                            .desired_width(available_width / 3.0),
                                    );
                                    ui.add(
                                        egui::TextEdit::multiline(&mut parsed_markdown)
                                            .desired_rows(1)
                                            .desired_width(available_width / 3.0),
                                    );
                                    ui.add(
                                        egui::TextEdit::multiline(&mut timestamps)
                                            .desired_rows(1)
                                            .desired_width(available_width / 3.0),
                                    );
                                    ui.add(
                                        egui::TextEdit::multiline(&mut tasks)
                                            .desired_rows(1)
                                            .desired_width(available_width / 3.0),
                                    );
                                });

                                ui.add(
                                    egui::TextEdit::multiline(&mut duration)
                                        .desired_rows(1)
                                        .desired_width(available_width / 3.0),
                                );
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
}
