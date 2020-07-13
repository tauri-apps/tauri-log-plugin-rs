use anyhow::Result;
use byte_unit::Byte;
use log::{debug, error, info, trace, warn};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use tauri::api::config::get as get_config;

use std::fs::{self, File};
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
enum LogCmd {
  TauriPluginLog { level: LogLevel, message: String },
}

/// The available verbosity levels of the logger.
#[derive(Deserialize_repr, Debug)]
#[repr(u16)]
pub enum LogLevel {
  Trace = 1,
  Debug,
  Info,
  Warn,
  Error,
}

pub enum RotationStrategy {
  KeepOne,
  KeepAll,
}

const DEFAULT_MAX_FILE_SIZE: u128 = 40000;

fn get_max_file_size() -> u128 {
  if let Ok(config) = get_config() {
    if let Some(plugin_config) = config.plugin_config("log") {
      return plugin_config
        .get("maxFileSize")
        .map(|val| {
          Byte::from_str(
            val
              .as_str()
              .expect("maxFileSize must be a string. example: 2MB"),
          )
          .expect("failed to parse maxFileSize")
          .get_bytes()
        })
        .unwrap_or(DEFAULT_MAX_FILE_SIZE);
    }
  }
  DEFAULT_MAX_FILE_SIZE
}

fn get_log_file_path<P: AsRef<Path>>(
  dir: P,
  rotation_strategy: RotationStrategy,
) -> Result<PathBuf> {
  let path = dir.as_ref().join("app.log");
  if path.exists() {
    let log_size = File::open(&path)?.metadata()?.len() as u128;
    if log_size > get_max_file_size() {
      match rotation_strategy {
        RotationStrategy::KeepAll => {
          fs::rename(
            &path,
            dir.as_ref().join(format!(
              "{}.log",
              chrono::Local::now().format("app-%Y-%m-%d")
            )),
          )?;
        }
        RotationStrategy::KeepOne => {
          fs::remove_file(&path)?;
        }
      }
    }
  }

  Ok(path)
}

/// The logger.
pub struct Logger;

impl Logger {
  pub fn new<P: AsRef<Path>>(path: P, rotation_strategy: RotationStrategy) -> Result<Self> {
    fern::Dispatch::new()
      // Perform allocation-free log formatting
      .format(|out, message, record| {
        out.finish(format_args!(
          "{}[{}][{}] {}",
          chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
          record.target(),
          record.level(),
          message
        ))
      })
      .level(log::LevelFilter::Trace)
      .chain(std::io::stdout())
      .chain(fern::log_file(get_log_file_path(path, rotation_strategy)?)?)
      .apply()?;
    let logger = Self {};
    Ok(logger)
  }

  pub fn log(&self, level: LogLevel, message: String) {
    match level {
      LogLevel::Trace => trace!("{}", message),
      LogLevel::Debug => debug!("{}", message),
      LogLevel::Info => info!("{}", message),
      LogLevel::Warn => warn!("{}", message),
      LogLevel::Error => error!("{}", message),
    }
  }
}

impl tauri::plugin::Plugin for Logger {
  fn extend_api(&self, _: &mut tauri::WebView<'_, ()>, payload: &str) -> Result<bool, String> {
    match serde_json::from_str(payload) {
      Err(e) => Err(e.to_string()),
      Ok(command) => {
        match command {
          LogCmd::TauriPluginLog { level, message } => self.log(level, message),
        }
        Ok(true)
      }
    }
  }
}
