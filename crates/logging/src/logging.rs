use crate::bindings::exports;
use crate::bindings::wasi::logging::logging::{self, Level};

impl From<exports::wasi::logging::logging::Level> for Level {
    fn from(value: exports::wasi::logging::logging::Level) -> Self {
        match value {
            exports::wasi::logging::logging::Level::Trace => Self::Trace,
            exports::wasi::logging::logging::Level::Debug => Self::Debug,
            exports::wasi::logging::logging::Level::Info => Self::Info,
            exports::wasi::logging::logging::Level::Warn => Self::Warn,
            exports::wasi::logging::logging::Level::Error => Self::Error,
            exports::wasi::logging::logging::Level::Critical => Self::Critical,
        }
    }
}

impl From<Level> for exports::wasi::logging::logging::Level {
    fn from(value: Level) -> Self {
        match value {
            Level::Trace => Self::Trace,
            Level::Debug => Self::Debug,
            Level::Info => Self::Info,
            Level::Warn => Self::Warn,
            Level::Error => Self::Error,
            Level::Critical => Self::Critical,
        }
    }
}

impl exports::wasi::logging::logging::Guest for () {
    fn log(level: exports::wasi::logging::logging::Level, context: String, message: String) {
        logging::log(level.into(), &context, &message)
    }
}
