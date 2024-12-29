-- INSERT OR IGNORE INTO images (file_path, thumbnail, width, height, file_size,metadata_json,file_created_at, created_at, updated_at)
--         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7,?8,?9);

INSERT INTO images (file_path, thumbnail, width, height, file_size, metadata_json, file_created_at, created_at,
                    updated_at)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9) ON CONFLICT(file_path) DO
UPDATE SET
    thumbnail = excluded.thumbnail,
    updated_at = excluded.updated_at;