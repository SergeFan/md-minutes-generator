use directories::UserDirs;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub fn get_desktop_dir() -> String {
    if let Some(user_dirs) = UserDirs::new() {
        if let Some(desktop_dir) = user_dirs.desktop_dir() {
            return desktop_dir
                .to_path_buf()
                .into_os_string()
                .into_string()
                .unwrap();
        }
    }

    String::new()
}

#[tauri::command]
pub async fn select_input(app_handle: AppHandle) -> String {
    if let Some(user_dirs) = UserDirs::new() {
        let file_path = app_handle
            .dialog()
            .file()
            .set_directory(user_dirs.home_dir())
            .add_filter("Excel", &["xlsx", "xls"])
            .blocking_pick_file();

        return match file_path {
            None => String::new(),
            Some(file_path) => file_path.to_string(),
        };
    };

    String::new()
}

#[tauri::command]
pub async fn select_output(app_handle: AppHandle) -> String {
    if let Some(user_dirs) = UserDirs::new() {
        let file_path = app_handle
            .dialog()
            .file()
            .set_directory(user_dirs.home_dir())
            .blocking_pick_folder();

        return match file_path {
            None => String::new(),
            Some(file_path) => file_path.to_string(),
        };
    };

    String::new()
}
