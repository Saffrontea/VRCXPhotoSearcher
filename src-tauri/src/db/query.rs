pub struct Queries {
    pub create_tables: &'static str,
    pub create_sub_index: &'static str,
    pub insert_folder: &'static str,
    pub insert_ignore_folder: &'static str,
    pub select_all_folders: &'static str,
    pub select_all_ignore_folders: &'static str,
    pub delete_folder: &'static str,
    pub delete_ignore_folder: &'static str,
    pub insert_image: &'static str,
}

/// クエリの初期化用マクロ
#[cfg(windows)]
impl Queries {
    pub fn load() -> Self {
        Self {
            create_tables: include_str!("sql\\create_tables.sql"), // パス修正
            create_sub_index: include_str!("sql\\create_sub_index.sql"), // パス修正
            insert_folder: include_str!("sql\\insert_folder.sql"), // パス修正
            insert_ignore_folder: include_str!("sql\\insert_ignore_folder.sql"), // パス修正
            select_all_folders: include_str!("sql\\select_all_folders.sql"), // パス修正
            select_all_ignore_folders: include_str!("sql\\select_all_ignore_folders.sql"), // パス修正
            delete_folder: include_str!("sql\\delete_folder.sql"), // パス修正
            delete_ignore_folder: include_str!("sql\\delete_ignore_folder.sql"), // パス修正
            insert_image: include_str!("sql\\insert_image.sql"),   // パス修正
        }
    }
}
#[cfg(not(windows))]
impl Queries {
    pub fn load() -> Self {
        Self {
            create_tables: include_str!("sql/create_tables.sql"),
            create_sub_index: include_str!("sql/create_sub_index.sql"), // パス修正
            insert_folder: include_str!("sql/insert_folder.sql"),
            insert_ignore_folder: include_str!("sql/insert_ignore_folder.sql"),
            select_all_folders: include_str!("sql/select_all_folders.sql"),
            select_all_ignore_folders: include_str!("sql/select_all_ignore_folders.sql"),
            delete_folder: include_str!("sql/delete_folder.sql"),
            delete_ignore_folder: include_str!("sql/delete_ignore_folder.sql"),
            insert_image: include_str!("sql/insert_image.sql"),
        }
    }
}
