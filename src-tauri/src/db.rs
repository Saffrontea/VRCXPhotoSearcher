use crate::model::search::SearchFolder;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use image::GenericImageView;
use std::collections::{HashMap, HashSet};
// サムネイル生成用
use chrono::{DateTime, Utc};
use png::Decoder;
use rusqlite::{params, params_from_iter, Connection, OpenFlags, OptionalExtension, Result};
use serde_json::{Number, Value};
use std::fs;
use std::fs::{create_dir_all, File};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::Mutex;
use tokio::task;
use walkdir::WalkDir;

// Queries構造体をインポート
mod query;
use query::Queries;

// グローバルでクエリを一度読み込む
lazy_static::lazy_static! {
    static ref SQL_QUERIES: Queries = Queries::load();
}

/// データベースの初期化
pub fn init_db(app: &AppHandle) -> Result<Connection> {
    // データベースファイルのパス
    if !app
        .path()
        .app_data_dir()
        .unwrap_or(PathBuf::from("."))
        .exists()
    {
        create_dir_all(app.path().app_data_dir().unwrap_or(PathBuf::from(".")))
            .expect("Failed to create directory")
    }
    let db_path = app
        .path()
        .app_data_dir()
        .unwrap_or(std::path::PathBuf::from("."))
        .join("search_folders.db");
    // ディレクトリ作成（存在しない場合）
    // if let Some(parent) = db_path.parent() {
    //        match std::fs::::create(parent) {
    //            Ok(_) => {},
    //            Err(_) => {eprintln!("configフォルダがありません。")    }
    //        }
    // }

    // データベースに接続
    let conn = Connection::open(db_path)?;

    // 必要なテーブルを作成
    conn.execute_batch(SQL_QUERIES.create_tables)?; // クエリを使用

    Ok(conn)
}

fn connect_index_db(app: &AppHandle, uuid: &str) -> Result<Connection> {
    let db_path = app
        .path()
        .app_data_dir()
        .unwrap_or(std::path::PathBuf::from("."))
        .join(uuid);
    let conn = Connection::open(db_path)?;
    conn.execute_batch(SQL_QUERIES.create_sub_index)?; // クエリを使用
    Ok(conn)
}

fn connect_index_db_ro(app: &AppHandle, uuid: &str) -> Result<Connection> {
    let db_path = app
        .path()
        .app_data_dir()
        .unwrap_or(std::path::PathBuf::from("."))
        .join(uuid);
    let conn = Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    Ok(conn)
}

/// フォルダの追加（INSERT）
#[tauri::command]
pub fn add_folder(app: AppHandle, path: String) -> Result<(), String> {
    let conn = init_db(&app).map_err(|e| e.to_string())?;
    conn.execute(
        SQL_QUERIES.insert_folder, // クエリを使用
        params![path, uuid::Uuid::new_v4().to_string()],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// フォルダの追加（INSERT）
#[tauri::command]
pub fn add_ignore_folder(app: AppHandle, path: String) -> Result<(), String> {
    let conn = init_db(&app).map_err(|e| e.to_string())?;
    conn.execute(
        SQL_QUERIES.insert_ignore_folder, // クエリを使用
        params![path, uuid::Uuid::new_v4().to_string()],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// フォルダの取得（SELECT）
#[tauri::command]
pub fn get_all_folders(app: AppHandle) -> Result<Vec<SearchFolder>, String> {
    let conn = init_db(&app).map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(SQL_QUERIES.select_all_folders) // クエリを使用
        .map_err(|e| e.to_string())?;

    let folders = stmt
        .query_map([], |row| {
            Ok(SearchFolder {
                id: row.get(0)?,
                path: row.get(1)?,
                uuid: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())? // エラー処理
        .filter_map(Result::ok)
        .collect();

    Ok(folders)
}

#[tauri::command]
pub fn get_all_ignore_folders(app: AppHandle) -> Result<Vec<SearchFolder>, String> {
    let conn = init_db(&app).map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(SQL_QUERIES.select_all_ignore_folders) // クエリを使用
        .map_err(|e| e.to_string())?;

    let folders = stmt
        .query_map([], |row| {
            Ok(SearchFolder {
                id: row.get(0)?,
                path: row.get(1)?,
                uuid: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())? // エラー処理
        .filter_map(Result::ok)
        .collect();

    Ok(folders)
}

/// フォルダの削除（DELETE）
#[tauri::command]
pub fn delete_folder(app: AppHandle, id: i32) -> Result<(), String> {
    let conn = init_db(&app).map_err(|e| e.to_string())?;
    let uuid: String = conn
        .prepare("SELECT uuid From search_folders where id = ?")
        .map_err(|e| e.to_string())?
        .query_map(params![id], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .next()
        .unwrap();
    let db_path = app
        .path()
        .app_data_dir()
        .unwrap_or(std::path::PathBuf::from("."))
        .join(uuid);
    match fs::remove_file(db_path.clone()) {
        Ok(_) => {}
        Err(err) => eprintln!(
            "ファイル '{}' の削除中にエラーが発生しました: {}",
            db_path.display(),
            err
        ),
    }
    conn.execute(
        SQL_QUERIES.delete_folder, // クエリを使用
        params![id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_ignore_folder(app: AppHandle, id: i32) -> Result<(), String> {
    let conn = init_db(&app).map_err(|e| e.to_string())?;
    conn.execute(
        SQL_QUERIES.delete_ignore_folder, // クエリを使用
        params![id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// 全てのフォルダで検索し、ファイルリストを返す
#[tauri::command]
pub fn search_files_in_folders(app: AppHandle) -> Result<Vec<(String, String)>, String> {
    let conn = init_db(&app).map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(SQL_QUERIES.select_all_folders) // クエリを使用
        .map_err(|e| e.to_string())?;
    // フォルダパスのリストを取得
    let paths: Vec<(String, String)> = stmt
        .query_map([], |row| {
            let folder: String = row.get(1)?; // フォルダパス
            let uuid: String = row.get(2)?; // UUID
            Ok((folder, uuid))
        })
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();
    stmt = conn
        .prepare(SQL_QUERIES.select_all_ignore_folders) // クエリを使用
        .map_err(|e| e.to_string())?;
    // フォルダパスのリストを取得
    let ignore_paths: Vec<(String, String)> = stmt
        .query_map([], |row| {
            let folder: String = row.get(1)?; // フォルダパス
            let uuid: String = row.get(2)?; // UUID
            Ok((folder, uuid))
        })
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();

    let mut all_files = Vec::new();

    for (path, uuid) in paths {
        for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_file()
                && is_image_file(entry.path())
                && !is_ignored(
                    entry.path(),
                    &ignore_paths
                        .iter()
                        .map(|p| Path::new(&p.0).to_path_buf())
                        .collect(),
                )
            {
                all_files.push((entry.path().to_string_lossy().to_string(), uuid.clone()));
            }
        }
    }

    Ok(all_files)
}

fn is_ignored(path: &Path, ignore_dirs: &HashSet<PathBuf>) -> bool {
    ignore_dirs
        .iter()
        .any(|ignore_dir| path.starts_with(ignore_dir))
}

/// 指定されたフォルダ内の画像ファイルを検索し、サムネイルを生成してデータベースに登録
#[tauri::command]
pub fn scan_and_register_images(app: AppHandle) -> Result<(), String> {
    let conn = init_db(&app).map_err(|e| e.to_string())?;

    // 検索フォルダを取得
    let mut stmt = conn
        .prepare(SQL_QUERIES.select_all_folders)
        .map_err(|e| e.to_string())?;

    let folders: Vec<(String, String)> = stmt
        .query_map([], |row| {
            let folder: String = row.get(1)?; // フォルダパス
            let uuid: String = row.get(2)?; // UUID
            Ok((folder, uuid))
        })
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();

    let mut stmt = conn
        .prepare(SQL_QUERIES.select_all_ignore_folders)
        .map_err(|e| e.to_string())?;

    let ignore_paths: Vec<(String, String)> = stmt
        .query_map([], |row| {
            let folder: String = row.get(1)?; // フォルダパス
            let uuid: String = row.get(2)?; // UUID
            Ok((folder, uuid))
        })
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();

    // 各フォルダ内を再帰探索
    for (folder, uuid) in folders {
        let path = Path::new(&folder);

        // `WalkDir`を使用して再帰的にディレクトリ内を探索
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok()) // 有効なエントリのみ処理
            .filter(|e| {
                e.path().is_file()
                    && !is_ignored(
                        e.path(),
                        &ignore_paths
                            .iter()
                            .map(|p| Path::new(&p.0).to_path_buf())
                            .collect(),
                    )
            })
        // ファイルのみ処理
        {
            let file_path = entry.path();

            if is_image_file(file_path) {
                // 画像ファイルを処理する
                match process_image_file(&app, file_path, &uuid) {
                    Ok(_) => println!("登録成功: {:?}", file_path),
                    Err(e) => eprintln!("登録失敗: {:?}, エラー: {}", file_path, e),
                }
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn scan_and_register_images_with_progress(
    app: AppHandle,
    folder_list: Vec<String>,
    event_id: String,
) -> Result<(), String> {
    let con = init_db(&app).map_err(|e| e.to_string())?;
    let conn = Arc::new(Mutex::new(con));
    if folder_list.is_empty() {
        return Err("フォルダがありません。".to_string());
    }

    let placeholders = folder_list
        .iter()
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(", ");
    let folders: Vec<(String, String)> = conn
        .lock()
        .await
        .prepare(&format!(
            "SELECT path, uuid FROM search_folders WHERE path IN ({})",
            placeholders
        ))
        .map_err(|e| e.to_string())?
        .query_map(params_from_iter(folder_list.iter()), |row| {
            let folder: String = row.get(0)?; // フォルダパス
            let uuid: String = row.get(1)?; // UUID
            Ok((folder, uuid))
        })
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();

    let ignore_paths: Vec<(String, String)> = conn
        .lock()
        .await
        .prepare(SQL_QUERIES.select_all_ignore_folders)
        .map_err(|e| e.to_string())?
        .query_map([], |row| {
            let folder: String = row.get(1)?; // フォルダパス
            let uuid: String = row.get(2)?; // UUID
            Ok((folder, uuid))
        })
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();

    println!("{:?}", folders);
    let mut i = 1;

    for (folder, uuid) in folders {
        let path = PathBuf::from(&folder);
        if path.is_dir() {
            // `WalkDir`を使用して再帰的に画像ファイルを検索
            let image_files: Vec<PathBuf> = WalkDir::new(&path)
                .into_iter()
                .filter_map(|entry| entry.ok()) // 有効なエントリのみ処理
                .filter(|entry| {
                    entry.path().is_file()
                        && is_image_file(entry.path())
                        && !is_ignored(
                            entry.path(),
                            &ignore_paths
                                .iter()
                                .map(|p| Path::new(&p.0).to_path_buf())
                                .collect(),
                        )
                }) // 画像ファイルのみ処理
                .map(|entry| entry.into_path())
                .collect();

            let progress_message = format!("フォルダ解析中...");
            let mut event_data = Value::Object(
                vec![
                    ("event_id".to_string(), Value::String(event_id.clone())),
                    ("progress".to_string(), Value::Number(Number::from(i))),
                    (
                        "message".to_string(),
                        Value::String(progress_message.clone()),
                    ),
                ]
                .into_iter()
                .collect(),
            );

            let app_clone = app.clone();
            let uuid_clone = uuid.clone();

            let res =
                process_images_in_transaction_async(image_files, uuid_clone, Arc::new(app_clone))
                    .await;
            // トランザクションを使用してフォルダ内の画像を一括登録
            if let Err(e) = res {
                eprintln!("フォルダ処理失敗: {} - エラー: {}", folder, e);
                event_data = Value::Object(
                    vec![
                        ("event_id".to_string(), Value::String(event_id.clone())),
                        ("progress".to_string(), Value::Number(Number::from(i))),
                        (
                            "message".to_string(),
                            Value::String(format!("フォルダ処理失敗: {} - エラー: {}", folder, e)),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                );
                continue;
            } else {
                let q = res.unwrap();
                i += q;
            }

            // 進捗を通知
            app.emit("scan_progress", &event_data)
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

/// ファイルが画像かどうかを判定
fn is_image_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        matches!(
            ext.to_string_lossy().to_lowercase().as_str(),
            "jpg" | "jpeg" | "png"
        )
    } else {
        false
    }
}

fn process_image_file(app: &AppHandle, file_path: &Path, uuid: &str) -> Result<(), String> {
    let conn = connect_index_db(app, uuid).expect("sub index db not found");

    let db_meta = conn
        .prepare("SELECT file_size,length(thumbnail),updated_at FROM images WHERE file_path = ?")
        .map_err(|e| e.to_string())?
        .query_row([file_path.to_string_lossy().to_string()], |row| {
            let file_size: i32 = row.get(0)?;
            let thumbnail_size: i32 = row.get(1)?;
            let updated_at: String = row.get(2)?;
            Ok((file_size, thumbnail_size, updated_at))
        })
        .optional()
        .map_err(|e| e.to_string());

    let metadata = fs::metadata(file_path).map_err(|e| e.to_string())?;
    let file_size = metadata.len() as i32;
    match db_meta {
        Ok(Some(db_meta)) => {
            if file_size == db_meta.0
                && db_meta.1 > 0
                && SystemTime::UNIX_EPOCH
                    .checked_add(Duration::from_secs(
                        db_meta.2.parse::<DateTime<Utc>>().unwrap().timestamp() as u64,
                    ))
                    .unwrap()
                    <= metadata.modified().unwrap()
            {
                return Ok(());
            }
        }
        Ok(None) => {}
        Err(e) => return Err(e.to_string()),
    }

    // サムネイル生成
    let thumbnail = generate_thumbnail(app, file_path).map_err(|e| e.to_string())?;

    // 画像の幅と高さを取得
    let image = image::open(file_path).map_err(|e| e.to_string())?;
    let (width, height) = image.dimensions();

    // iTXtチャンクからメタデータを取得
    let metadata_json = extract_metadata(file_path).unwrap_or(None);
    let file_created_at_time: DateTime<Utc> = fs::metadata(file_path)
        .map_err(|e| {
            format!(
                "Failed to get metadata for file {}: {}",
                file_path.to_string_lossy(),
                e
            )
        })?
        .created()
        .map_err(|e| {
            format!(
                "Failed to get created date for file {}: {}",
                file_path.to_string_lossy(),
                e
            )
        })?
        .into();
    let file_created_at = file_created_at_time.to_rfc3339();
    // 現在時刻を取得
    let created_at = Utc::now().to_rfc3339();
    let updated_at = created_at.clone();

    // 保存処理を実行
    conn.execute(
        SQL_QUERIES.insert_image,
        params![
            file_path.to_string_lossy().to_string(),
            thumbnail,
            width as i32,
            height as i32,
            file_size,
            metadata_json.unwrap_or_default(), // JSONデータがない場合は空文字列
            file_created_at,
            created_at,
            updated_at,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

async fn process_images_in_transaction_async(
    file_paths: Vec<PathBuf>,
    uuid: String,
    app: Arc<AppHandle>,
) -> std::result::Result<u32, String> {
    // スレッドブロッキング部分
    let transaction_result = task::spawn_blocking(move || {
        let mut i = 1;
        // 各ファイルの画像処理
        let chunk_size = 20; // トランザクションごとに処理する件数
        for chunk in file_paths.chunks(chunk_size) {
            // トランザクション開始
            let mut conn = connect_index_db(app.as_ref(), &uuid).map_err(|e| e.to_string())?;
            let transaction = conn.transaction().map_err(|e| e.to_string())?;

            for file_path in chunk {
                // データベースのメタデータを取得
                let db_meta = transaction
                    .prepare("SELECT file_size,length(thumbnail),updated_at FROM images WHERE file_path = ?")
                    .map_err(|e| e.to_string())?
                    .query_row([file_path.to_string_lossy().to_string()], |row| {
                        let file_size: i32 = row.get(0)?;
                        let thumbnail_size: i32 = row.get(1)?;
                        let updated_at: String = row.get(2)?;
                        Ok((file_size, thumbnail_size, updated_at))
                    })
                    .optional()
                    .map_err(|e| e.to_string());

                let metadata = fs::metadata(file_path).map_err(|e| e.to_string())?;
                let file_size = metadata.len() as i32;

                match db_meta {
                    Ok(Some(db_meta)) => {
                        if file_size == db_meta.0
                            && db_meta.1 > 0
                            && SystemTime::UNIX_EPOCH
                            .checked_add(Duration::from_secs(db_meta.2.parse::<DateTime<Utc>>().unwrap().timestamp() as u64))
                            .unwrap()
                            <= metadata.modified().unwrap()
                        {
                            continue;
                        }
                    }
                    Ok(None) => {}
                    Err(e) => {
                        return Err(e.to_string());
                    }
                }

                // サムネイルの生成およびデータの収集
                let thumbnail = base64::encode(generate_thumbnail(&app, file_path).map_err(|e| e.to_string())?);
                let image = image::open(file_path).map_err(|e| e.to_string())?;
                let (width, height) = image.dimensions();
                let metadata_json = extract_metadata(file_path).unwrap_or(None);
                let file_created_at_time: DateTime<Utc> = fs::metadata(file_path.clone())
                    .map_err(|e| format!("Failed to get metadata for file {}: {}", file_path.to_string_lossy(), e))?
                    .created()
                    .map_err(|e| format!("Failed to get created date for file {}: {}", file_path.to_string_lossy(), e))?
                    .into();
                let file_created_at = file_created_at_time.to_rfc3339();
                let created_at = Utc::now().to_rfc3339();
                let updated_at = created_at.clone();

                // データベースに挿入
                transaction.execute(
                    SQL_QUERIES.insert_image,
                    params![
                file_path.to_string_lossy(),
                thumbnail,
                width as i32,
                height as i32,
                file_size,
                metadata_json.unwrap_or_default(),
                file_created_at,
                created_at,
                updated_at
            ],
                )
                    .map_err(|e| e.to_string())?;
                println!("{:?}",file_path.to_string_lossy());
                i+=1;
            }

            // トランザクションのコミット
            transaction.commit().map_err(|e| e.to_string())?;
        }
        Ok::<u32, String>(i)
    })
        .await
        .map_err(|e| format!("トランザクションエラー: {:?}", e));
    transaction_result?
}

fn extract_metadata(file_path: &Path) -> Result<Option<String>, String> {
    // ファイルを開く
    let file = File::open(file_path).map_err(|e| format!("ファイルを開けませんでした: {}", e))?;

    // PNGデコーダーを作成
    let decoder = Decoder::new(file);
    let reader = decoder
        .read_info()
        .map_err(|e| format!("PNG解析エラー: {}", e))?;

    // PNGのメタデータチャンク（iTXt）を検索
    if let Some(itxt) = reader
        .info()
        .utf8_text
        .iter()
        .find(|t| t.keyword == "Description")
    {
        // iTXtの内容を取得
        let data = itxt.get_text().unwrap_or("".parse().unwrap());
        // JSONとしてパース可能な場合はパース
        match serde_json::from_str::<Value>(&data) {
            Ok(parsed_json) => {
                Ok(Some(parsed_json.to_string())) // JSON文字列として返却
            }
            Err(_) => {
                // JSONでない場合は元データをそのまま返す
                Ok(Some(data.clone()))
            }
        }
    } else {
        // iTXtが存在しない場合
        Ok(None)
    }
}

/// サムネイル生成
fn generate_thumbnail(_app: &AppHandle, file_path: &Path) -> Result<Vec<u8>> {
    let image = image::open(file_path)
        .map_err(|_| "画像を開けませんでした")
        .unwrap();
    let thumbnail = image.thumbnail(256, 256); // サムネイルサイズは256に縮小

    let mut buffer = Vec::new();
    thumbnail
        .write_to(
            &mut std::io::Cursor::new(&mut buffer),
            image::ImageFormat::Png,
        )
        .map_err(|_| "サムネイルのエンコードに失敗しました")
        .expect("");
    //TODO: ERR
    Ok(buffer)
}

#[tauri::command]
pub fn generate_and_get_thumbnails(
    app: AppHandle,
    file_paths: Vec<(String, String)>,
) -> Result<Vec<(String, String, String)>, String> {
    let mut results = Vec::new();
    for (file_path, uuid) in file_paths {
        let path = Path::new(&file_path);
        if !path.exists() {
            continue;
        }
        if !is_image_file(path) {
            continue;
        }
        let conn = connect_index_db_ro(&app, &uuid).map_err(|e| e.to_string())?; // サムネイルがデータベースに存在するか確認
        let mut stmt = conn
            .prepare("SELECT thumbnail FROM images WHERE file_path = ?")
            .map_err(|e| format!("クエリ準備エラー: {}", e))?;

        let existing_thumbnail: Option<String> = stmt
            .query_map([file_path.clone()], |row| row.get(0))
            .map_err(|e| format!("クエリ実行エラー: {}", e))?
            .filter_map(Result::ok)
            .next();
        if let Some(thumbnail) = existing_thumbnail {
            let mime_type = "image/png";
            let base64_formatted = format!("data:{};base64,{}", mime_type, thumbnail);
            // サムネイルが存在すれば結果に追加
            results.push((file_path.clone(), base64_formatted, uuid.clone()));
            continue;
        }
    }
    Ok(results)
}

#[tauri::command]
pub fn get_image_metadata(
    app: AppHandle,
    uuid: String,
    file_path: String,
) -> Result<Value, String> {
    // ファイルの存在をチェック
    let path = Path::new(&file_path);
    if !path.exists() || !path.is_file() {
        return Err("指定されたファイルが存在しません。".to_string());
    }

    // UUIDに基づきデータベースに接続
    let conn = connect_index_db(&app, &uuid).map_err(|e| {
        let msg = format!(
            "サブデータベース接続エラー: {} (UUID: {}, Path: {:?})",
            e,
            uuid,
            app.path().app_data_dir()
        );
        msg
    })?;

    // DBからメタデータ取得
    let (metadata_json, file_created_at): (Option<String>, Option<String>) = conn
        .query_row(
            "SELECT metadata_json, file_created_at FROM images WHERE file_path = ?",
            params![file_path],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(|e| format!("クエリエラー: {}", e))?
        .unwrap_or((None, None));

    if let Some(metadata) = metadata_json {
        // ファイルを読み取りBase64エンコード
        return match fs::read(path) {
            Ok(file_data) => {
                let base64_content = STANDARD.encode(&file_data);
                let mime_type = mime_guess::from_path(&file_path)
                    .first_or_octet_stream()
                    .essence_str()
                    .to_string();
                let data_url = format!("data:{};base64,{}", mime_type, base64_content);

                let metadata_value: Value = serde_json::from_str(&metadata).unwrap_or(Value::Null);
                let result = Value::Object(
                    vec![
                        ("metadata".to_string(), metadata_value),
                        ("data_url".to_string(), Value::String(data_url)),
                        (
                            "file_created_at".to_string(),
                            Value::String(file_created_at.unwrap_or("".to_string())),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                );
                Ok(result)
            }
            Err(e) => Err(format!("ファイル読み取りエラー: {}", e)),
        };
    }
    Err("指定されたファイルのメタデータが見つかりません。".to_string())
}

#[tauri::command]
pub fn search_images(
    app: AppHandle,
    conditions: Vec<HashMap<String, String>>,
) -> Result<Vec<(String, String, String)>, String> {
    let con = init_db(&app).map_err(|e| e.to_string())?;
    let subdb_list: Vec<(String, String)> = con
        .prepare(SQL_QUERIES.select_all_folders)
        .map_err(|e| e.to_string())?
        .query_map([], |row| {
            let folder: String = row.get(1)?; // フォルダパス
            let uuid: String = row.get(2)?; // UUID
            Ok((folder, uuid))
        })
        .map_err(|e| e.to_string())? // エラー処理
        .filter_map(Result::ok)
        .collect();
    let mut query = String::from(
        "SELECT DISTINCT file_path, thumbnail FROM images WHERE 1=1 AND json_valid(metadata_json) = 1",
    ); // ベースクエリ
    let mut params: Vec<String> = vec![]; // バインド用パラメータリスト

    let mut condition_segments: Vec<String> = vec![]; // 個別の条件を保存
    let mut player_count = 0;

    for condition in conditions.clone() {
        let logic = match condition
            .get("logic")
            .unwrap_or(&"AND".to_string())
            .to_uppercase()
            .as_str()
        {
            "AND" => "AND",
            "OR" => "OR",
            _ => "AND",
        }
        .to_string();
        let default_field = "".to_string();
        let field = match condition.get("field").unwrap_or(&default_field).as_str() {
            "player" => "player",
            "world" => "world",
            "created_at" => "created_at",
            _ => "file_path",
        }
        .to_string();
        let default_operator = "=".to_string();
        let operator = match condition
            .get("operator")
            .unwrap_or(&default_operator)
            .to_uppercase()
            .as_str()
        {
            "EQ" => "=",
            "NE" => "!=",
            "GT" => ">",
            "GE" => ">=",
            "LT" => "<",
            "LE" => "<=",
            "LIKE" => "LIKE",
            _ => "=",
        };
        let default_value = "".to_string();
        let value = condition.get("value").unwrap_or(&default_value);
        if value.is_empty() {
            continue; // 値のない条件は無視
        }

        let mut segment = match field.as_str() {
            "player" => {
                player_count += 1;
                let player_alias = format!("player{}", player_count);

                format!(
                    "EXISTS (SELECT 1 FROM json_each(metadata_json, '$.players') AS {} WHERE json_extract({}.value, '$.displayName') {} ?)",
                    player_alias, player_alias, operator
                )
            }
            "world" => {
                format!("JSON_EXTRACT(metadata_json, '$.world.name') {} ?", operator)
            }
            "created_at" => {
                format!("file_created_at {} ?", operator)
            }
            _ => "file_path LIKE ?".to_string(),
        };

        // プレースホルダーの値を追加
        params.push(if field == "player" || field == "world" {
            if operator.to_uppercase().contains("LIKE") {
                format!("%{}%", value.clone())
            } else {
                value.clone()
            }
        } else {
            value.clone()
        });

        // 条件を適切にロジックに基づき追加
        if !condition_segments.is_empty() {
            segment = format!("{} {}", logic, segment);
        }
        condition_segments.push(segment);
    }

    // 条件をすべて結合
    if !condition_segments.is_empty() {
        query.push_str(" AND (");
        query.push_str(&condition_segments.join(" "));
        query.push_str(")");
    }

    let mut results = Vec::new();
    // SQLを実行し、結果を取得
    for (_, uuid) in subdb_list {
        let conn = connect_index_db_ro(&app, &uuid).map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare(&query)
            .map_err(|_e| String::from("検索クエリエラー"))?;
        let params = params.clone();
        let rows = stmt.query_map(params_from_iter(params.iter()), |row| {
            let file_path: String = row.get(0)?;
            let thumbnail: String = row.get(1)?;
            let mime_type = "image/png";
            let base64_formatted = format!("data:{};base64,{}", mime_type, thumbnail);
            Ok((file_path, base64_formatted, uuid.clone()))
        });
        // 検索結果を Vec に格納して返却
        for row in rows.map_err(|e| e.to_string())? {
            match row {
                Ok(row) => {
                    results.push(row);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    }

    Ok(results)
}
