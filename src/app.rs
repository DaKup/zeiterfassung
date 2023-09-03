use rfd::AsyncFileDialog;

use crate::platform;
use crate::processing;
use crate::processing::parse_date;

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

fn _on_clicked_open() {
    async fn run() {
        let file_handle = AsyncFileDialog::new()
            .add_filter("gpc", &["gpc.json"])
            .pick_file()
            // .pick_files()
            .await;

        let data = match file_handle {
            Some(x) => x.read().await,
            None => return,
        };

        let _data = String::from_utf8_lossy(&data).to_string();
        // *gpc.lock().unwrap() = parse_gpc(&data);
    }

    platform::spawn_async(run());
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
        let Self { state } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |_ui| {});
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let num_lines1 = state.text.lines().count();

            let mut parsed_markdown = processing::parse_markdown(state.text.as_str());
            let (mut timestamps, mut tasks) = processing::parse_log(parsed_markdown.as_str());

            let num_lines2 = parsed_markdown.lines().count();
            let num_lines3 = timestamps.lines().count();
            let num_lines4 = tasks.lines().count();
            let width = ui.available_width() / 3.0;

            let mut duration = parse_date(&timestamps);

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut state.text)
                            // .desired_rows(60)
                            .desired_rows(num_lines1 + 1)
                            // .desired_width(600.0),
                            .desired_width(width),
                    );
                    ui.add(
                        egui::TextEdit::multiline(&mut parsed_markdown)
                            .desired_rows(num_lines2)
                            .desired_width(width),
                    );
                    ui.add(
                        egui::TextEdit::multiline(&mut timestamps)
                            .desired_rows(num_lines3)
                            .desired_width(width),
                    );
                    ui.add(
                        egui::TextEdit::multiline(&mut tasks)
                            .desired_rows(num_lines4)
                            .desired_width(width),
                    );
                });

                ui.add(
                    egui::TextEdit::multiline(&mut duration)
                        .desired_rows(3)
                        .desired_width(width),
                );
            });
        });
    }
}
