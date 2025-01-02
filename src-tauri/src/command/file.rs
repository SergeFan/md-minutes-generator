use tauri::AppHandle;

use crate::tool::excel;

#[tauri::command]
pub fn generate_markdown(
    app_handle: AppHandle,
    input: &str,
    output: Option<&str>,
    sheet: Option<&str>,
) -> bool {
    if output.is_none() || sheet.is_none() {
        return false;
    }

    excel::generate_markdown(app_handle, input, output.unwrap(), sheet.unwrap()).is_ok()
}

#[tauri::command]
pub fn read_excel(input: &str, _: Option<&str>, _: Option<&str>) -> Vec<String> {
    if let Ok(sheet_names) = excel::read_excel(input) {
        return sheet_names;
    }

    vec![]
}
