use serde::Serialize;

/// 画像情報のデータ構造
#[derive(Serialize)]
pub struct ImageData {
    pub id: i32,
    pub file_path: String,
    pub thumbnail: String,
    pub width: i32,
    pub height: i32,
    pub file_size: i32,
    pub metadata: String,
    pub file_created_at: String,
    pub created_at: String,
    pub updated_at: String,
}
