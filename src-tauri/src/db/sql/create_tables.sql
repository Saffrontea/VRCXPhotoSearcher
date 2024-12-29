-- テーブル「search_folders」を作成
CREATE TABLE IF NOT EXISTS search_folders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    uuid TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS ignore_folders (
                                              id INTEGER PRIMARY KEY AUTOINCREMENT,
                                              path TEXT NOT NULL UNIQUE,
                                              uuid TEXT NOT NULL UNIQUE
);