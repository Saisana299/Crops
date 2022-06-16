#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn fetch_messages() -> Vec<String> {
    vec!["hoge".into(), "huga".into(), "piyo".into(), "yaba".into()]
}

fn main() {
  tauri::Builder::default()
	.invoke_handler(tauri::generate_handler![fetch_messages])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
