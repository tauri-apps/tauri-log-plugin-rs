#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
use tauri::api::{
  config::get as get_config,
  path::{resolve_path, BaseDirectory},
};
use tauri_log_plugin::{Logger, RotationStrategy};

fn main() -> anyhow::Result<()> {
  let config = get_config()?;

  let log_dir = resolve_path(
    config.tauri.bundle.identifier.clone(),
    Some(BaseDirectory::Cache),
  )?;
  if !log_dir.exists() {
    std::fs::create_dir_all(&log_dir)?;
  }

  let logger = Logger::new(log_dir, RotationStrategy::KeepAll)?;

  tauri::AppBuilder::new()
    .plugin(logger)
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            MyCustomCommand { argument } => {
              //  your command code
              println!("{}", argument);
            }
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
  Ok(())
}
