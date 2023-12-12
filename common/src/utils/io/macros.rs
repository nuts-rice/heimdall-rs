#[macro_export]
macro_rules! fatal {
    ($message:expr) => {
        // if let verbosity
        $crate::utils::io::logging::Logger::default().fatal($message);
    };
    ($message:expr, $($arg:tt)*) => {
        $crate::utils::io::logging::Logger::default().fatal(&format!($message, $($arg)*))
    };
}

#[macro_export]
macro_rules! success {
    ($message:expr) => {
        $crate::utils::io::logging::Logger::default().success($message);
    };
    ($message:expr, $($arg:tt)*) => {
        $crate::utils::io::logging::Logger::default().success(&format!($message, $($arg)*))
    };
}

#[macro_export]
macro_rules! error {
    ($message:expr) => {

        let level = std::env::var("RUST_LOG").expect("RUST_LOG not set");
             match level.to_lowercase().as_str()  {
                 "error"  => {
                     $crate::utils::io::logging::Logger::default().error($message);
                },
                 "silent" => {
                     return;
                 }
                _ => {
                    return;
                }

            }
        };
    ($message:expr, $($arg:tt)*) => {
        let level = std::env::var("RUST_LOG").expect("RUST_LOG not set");
        match level.to_lowercase().as_str()  {
            "error"  => {
                $crate::utils::io::logging::Logger::default().error(&format!($message, $($arg)*))
            },
                _ => {
                    return;
                }

        }
    };
}

#[macro_export]
macro_rules! warn {
    ($message:expr) => {
        let level = std::env::var("RUST_LOG").expect("RUST_LOG not set");
        match level.to_lowercase().as_str()  {
            "error" | "warn"   => {    
                $crate::utils::io::logging::Logger::default().warn($message);
            },
                _ => {
                    return;
                }               
        }
    };
    ($message:expr, $($arg:tt)*) => {
        let level = std::env::var("RUST_LOG").expect("RUST_LOG not set");
        match level.to_lowercase().as_str()  {
            "error" | "warn"   => {    
        $crate::utils::io::logging::Logger::default().warn(&format!($message, $($arg)*))
            },
            _ => {
                return;
            }
        }
    };
}

#[macro_export]
macro_rules! info {
    ($message:expr) => {
        $crate::utils::io::logging::Logger::default().info($message);
    };
    ($message:expr, $($arg:tt)*) => {
        $crate::utils::io::logging::Logger::default().info(&format!($message, $($arg)*))
    };
}

#[macro_export]
macro_rules! debug_max {
    ($message:expr) => {
        $crate::utils::io::logging::Logger::default().debug_max($message);
    };
    ($message:expr, $($arg:tt)*) => {
        $crate::utils::io::logging::Logger::default().debug_max(&format!($message, $($arg)*))
    };
}

#[macro_export]
macro_rules! debug {
    ($message:expr) => {
        $crate::utils::io::logging::Logger::default().debug($message);
    };
    ($message:expr, $($arg:tt)*) => {
        $crate::utils::io::logging::Logger::default().debug(&format!($message, $($arg)*))
    };
}

#[macro_export]
macro_rules! trace {
    ($message:expr) => {
        $crate::utils::io::logging::Logger::default().trace($message);
    };
    ($message:expr, $($arg:tt)*) => {
        $crate::utils::io::logging::Logger::default().trace(&format!($message, $($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use crate::utils::io::logging::set_logger_env;

    use super::*;
    #[test]
    fn test_fatal_macro() {
        fatal!("This is a test for fatal! macro");
        fatal!("This is a test for fatal! macro with arg: {}", 1);
    }
    #[test]
    fn test_success_macro() {
        success!("This is a test for success! macro");
        success!("This is a test for success! macro with arg: {}", 1);
    }
    #[test]
    fn test_error_macro() {
        error!("log");
        error!("log, arg: {}", 1);
    }
    #[test]
    fn test_warn_macro() {
        warn!("This is a test for warn! macro");
        warn!("This is a test for warn! macro with arg: {}", 1);
    }
    #[test]
    fn test_info_macro() {
        info!("This is a test for info! macro");
        info!("This is a test for info! macro with arg: {}", 1);
    }
    #[test]
    fn test_debug_macro() {
        debug!("This is a test for debug! macro");
        debug!("This is a test for debug! macro with arg: {}", 1);
    }
    #[test]
    fn test_debug_max_macro() {
        debug_max!("This is a test for debug_max! macro");
        debug_max!("This is a test for debug_max! macro with arg: {}", 1);
    }
}
