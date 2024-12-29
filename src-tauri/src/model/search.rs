use serde::Serialize;

/// 検索フォルダのデータ構造
#[derive(Serialize)]
pub struct SearchFolder {
    pub id: i32,
    pub path: String,
    pub uuid: String,
}
