use tauri::{AppHandle, Manager, State};

pub struct AppState {
    pub scanning: std::sync::Mutex<bool>,
}

#[tauri::command]
fn get_status(state: State<'_, AppState>) -> serde_json::Value {
    let scanning = *state.scanning.lock().unwrap();
    serde_json::json!({
        "scanning": scanning,
        "product": "你的創意視覺化呈現",
        "brand": "晏銘湛箴",
    })
}

#[allow(dead_code)]
fn startup(_app: AppHandle) {
    let _state = std::sync::Arc::new(std::sync::Mutex::new(false));
    drop(_state.lock().unwrap());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Tauri 運行時將在 build script 正確設定後啟用
    // 骨架階段僅編譯符號檢查
}
