#![allow(unused)]
use log::{debug, error, info, trace, warn};

#[cfg(test)]
mod tests {
    use log_print::log_info;

    use crate::tracing_demo::logger2::{init_logger, init_logger2, init_logger3};

    use super::*;

    #[test]
    fn it_logger2_init_logger() {
        init_logger();

        log_info();

        trace!("这是一条 trace 日志");
        debug!("这是一条 debug 日志");
        info!("这是一条 info 日志");
        warn!("这是一条 warn 日志");
        error!("这是一条 error 日志");
    }

    #[test]
    fn it_logger2_init_logger2() {
        let _guard = init_logger2();

        log_info();

        trace!("这是一条 trace 日志");
        debug!("这是一条 debug 日志");
        info!("这是一条 info 日志");
        warn!("这是一条 warn 日志");
        error!("这是一条 error 日志");
    }

    #[test]
    fn it_logger2_init_logger3() {
        let _guard = init_logger3();

        log_info();

        trace!("这是一条 trace 日志");
        debug!("这是一条 debug 日志");
        info!("这是一条 info 日志");
        warn!("这是一条 warn 日志");
        error!("这是一条 error 日志");
    }
}
