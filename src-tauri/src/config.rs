use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

// カスタム設定の構造体
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    feature_flags: FeatureFlags,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureFlags {
    update_db_when_startup: bool,
    pub language: String,
}

fn default_config() -> Config {
    Config {
        feature_flags: FeatureFlags {
            update_db_when_startup: false,
            language: "ja".to_string(),
        },
    }
}

pub(crate) fn load_config(config_path: &PathBuf) -> Config {
    let path = config_path.join("config.json");
    if path.exists() {
        let config_file = fs::read_to_string(path).expect("Failed to read config file");
        serde_json::from_str(&config_file).expect("Failed to parse config file")
    } else {
        let default = default_config();
        save_config(config_path, &default);
        default
    }
}
pub fn save_config(config_path: &PathBuf, config: &Config) {
    let path = config_path.join("config.json");
    // 必要ならディレクトリを作成
    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir).expect("Failed to create config directory");
    }

    // JSON文字列として保存
    let content = serde_json::to_string_pretty(config).expect("Failed to serialize config");
    fs::write(path, content).expect("Failed to write config.json");
}

#[tauri::command]
pub fn get_config(app_handle: AppHandle) -> Config {
    load_config(
        &app_handle
            .path()
            .app_data_dir()
            .unwrap_or(PathBuf::from(".")),
    )
}
#[tauri::command]
pub fn set_config(app_handle: AppHandle, config: Config) {
    save_config(
        &app_handle
            .path()
            .app_data_dir()
            .unwrap_or(PathBuf::from(".")),
        &config,
    );
}
