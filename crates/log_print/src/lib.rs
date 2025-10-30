use log::{debug, error, info, trace, warn};

pub fn log_info() {
    trace!("lib, 这是一条 trace 日志");
    debug!("lib, 这是一条 debug 日志");
    info!("lib, 这是一条 info 日志");
    warn!("lib, 这是一条 warn 日志");
    error!("lib, 这是一条 error 日志");
}
