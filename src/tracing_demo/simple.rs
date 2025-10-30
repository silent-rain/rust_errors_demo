#![allow(unused)]
use log::{debug, error, info, trace, warn};

#[cfg(test)]
mod tests {
    use log_print::log_info;

    use crate::tracing_demo::logger::init_logger;

    use super::*;

    #[test]
    fn it_simple() {
        init_logger();

        log_info();

        trace!("这是一条 trace 日志");
        debug!("这是一条 debug 日志");
        info!("这是一条 info 日志");
        warn!("这是一条 warn 日志");
        error!("这是一条 error 日志");
    }
}
