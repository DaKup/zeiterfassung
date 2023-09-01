use rfd::AsyncFileDialog;

use crate::platform;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct MainApp {}

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
        let Self {} = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |_ui| {});
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {});
    }
}
