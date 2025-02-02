mod mc_server;
mod utils;
mod mc_commands;

#[tauri::command]
async fn greet()->Result<String, String> {

    Ok(("every ok").to_string())
    
}




#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
