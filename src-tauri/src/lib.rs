mod config;
mod db;
mod model;

use db::*;
use std::path::PathBuf;
use tauri::Manager;

use config::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        // 使用するTauriプラグインを追加
        .plugin(tauri_plugin_opener::init())
        // フォルダ操作・画像スキャン関連コマンドを追加
        .invoke_handler(tauri::generate_handler![
            add_folder,               // フォルダ追加
            get_all_folders,          // 全フォルダ取得
            delete_folder,            // フォルダ削除
            search_files_in_folders,  // フォルダ内画像検索
            scan_and_register_images, // 画像スキャン＆登録
            generate_and_get_thumbnails,
            get_image_metadata,
            scan_and_register_images_with_progress,
            get_config,
            set_config,
            add_ignore_folder,      // フォルダ追加
            delete_ignore_folder,   // フォルダ削除
            get_all_ignore_folders, // 全フォルダ取得
            search_images
        ])
        // Tauriイベントのサンプルフックセット
        .setup(|app| {
            let app_handle = app.handle();

            let config_path = app_handle
                .path()
                .app_data_dir()
                .unwrap_or(PathBuf::from("."));
            let _config = load_config(&config_path);

            #[cfg(debug_assertions)]
            app.get_webview_window("main").unwrap().open_devtools();
            // 初期処理（例：DBや必要フォルダの作成）があればここに追加
            init_db(app_handle).unwrap();
            println!("Tauri application is starting!");
            Ok(())
        })
        // ランタイムエントリーポイント
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
