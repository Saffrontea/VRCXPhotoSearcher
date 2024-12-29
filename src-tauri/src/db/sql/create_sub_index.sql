CREATE TABLE IF NOT EXISTS images (
                                      id INTEGER PRIMARY KEY AUTOINCREMENT,
                                      file_path TEXT NOT NULL UNIQUE,
                                      thumbnail TEXT,
                                      width INTEGER,
                                      height INTEGER,
                                      file_size INTEGER,
                                      metadata_json TEXT,
                                      file_created_at TEXT NOT NULL,
                                      created_at TEXT NOT NULL,
                                      updated_at TEXT NOT NULL
);