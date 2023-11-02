#![warn(clippy::all)]

use crate::{platform, MainApp};
use egui::TextBuffer;
use rfd::AsyncFileDialog;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

pub trait OnClickedButtonTrait {
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
