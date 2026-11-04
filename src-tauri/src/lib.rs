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
fn startup(app: AppHandle) {
    std::thread::spawn(move || {
        let state = app.state::<AppState>();
        drop(state.scanning.lock().unwrap());
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            scanning: std::sync::Mutex::new(false),
        })
        .invoke_handler(tauri::generate_handler![get_status])
        .setup(|app| {
            startup(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
