//! 日志初始化
#![allow(unused)]
use std::io;

use time::{UtcOffset, format_description::FormatItem};
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::{EnvFilter, fmt};

pub fn init_logger() {
    // 本地时间
    let timer = local_time();

    fmt::fmt()
        .compact()
        .with_ansi(true)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_timer(timer)
        // .with_timer(fmt::time::LocalTime::rfc_3339())
        .with_thread_names(false)
        .log_internal_errors(false)
        .with_writer(io::stderr)
        .with_env_filter(EnvFilter::new("debug")) // 设置过滤规则
        .init();
}

/// 获取本地时间
pub fn local_time() -> OffsetTime<Vec<FormatItem<'static>>> {
    let time_format = time::format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]",
    )
    .expect("format string should be valid!");

    let offset = UtcOffset::from_hms(8, 0, 0).unwrap();
    // let offset = UtcOffset::current_local_offset().expect("should get local offset!");

    // 本地时间
    OffsetTime::new(offset, time_format)
}
