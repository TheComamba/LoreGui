use std::path::PathBuf;

fn current_path() -> PathBuf {
    std::env::current_dir().unwrap_or_default()
}

pub(super) fn new() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_file_name("new_lore_database.db")
        .set_directory(current_path())
        .save_file()
}

pub(super) fn open() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("Lore Database (.db)", &["db"])
        .add_filter("Any", &["*"])
        .set_directory(current_path())
        .pick_file()
}
