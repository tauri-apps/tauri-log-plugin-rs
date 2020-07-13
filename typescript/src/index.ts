import { invoke } from 'tauri/api/tauri'

enum LogLevel {
  Trace = 1,
  Debug,
  Info,
  Warn,
  Error
}

function log(level: LogLevel, message: string) {
  invoke({
    cmd: 'tauriPluginLog',
    level,
    message
  })
}

function trace(message: string) {
  log(LogLevel.Trace, message)
}

function debug(message: string) {
  log(LogLevel.Debug, message)
}

function info(message: string) {
  log(LogLevel.Info, message)
}

function warn(message: string) {
  log(LogLevel.Warn, message)
}

function error(message: string) {
  log(LogLevel.Error, message)
}

export {
  trace,
  debug,
  info,
  warn,
  error
}
