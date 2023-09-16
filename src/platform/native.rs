#![cfg(not(target_arch = "wasm32"))]

use rfd::AsyncFileDialog;

pub async fn save_file_dialog(bytes: impl AsRef<[u8]>) {
    let file_handle = AsyncFileDialog::new()
        .add_filter("Markdown", &["md"])
        .save_file()
        .await;

    if file_handle.is_some() {
        let path = file_handle.unwrap();
        let path = path.path();

        let mut file = std::fs::File::create(path).unwrap();
        std::io::Write::write_all(&mut file, bytes.as_ref()).unwrap();
    }
}
