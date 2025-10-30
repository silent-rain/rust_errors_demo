//! 日志初始化
#![allow(unused)]

use time::format_description::FormatItem;
use time::{UtcOffset, format_description::BorrowedFormatItem};
use tracing::subscriber::DefaultGuard;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    Registry,
    fmt::{self, time::OffsetTime, writer::MakeWriterExt},
    layer::{Layer, SubscriberExt},
    registry::LookupSpan,
};

// log库, 无日志输出
pub fn init_logger() {
    // 初始化 LogTracer 以转发 log 事件到 tracing
    tracing_log::LogTracer::init().expect("Failed to set logger");

    let mut layers = Vec::new();
    let layer = layer();
    layers.push(layer);

    // 日志订阅器
    let subscriber = Registry::default()
        .with(ErrorLayer::default())
        // .with(console_layer)
        .with(layers);

    // 注册全局日志订阅器
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

pub fn init_logger2() -> DefaultGuard {
    // 初始化 LogTracer 以转发 log 事件到 tracing
    tracing_log::LogTracer::init().expect("Failed to set logger");

    let layer = layer();

    let subscriber = tracing_subscriber::registry().with(layer);

    tracing::subscriber::set_default(subscriber)
}

// log库, 无日志输出
pub fn init_logger3() -> DefaultGuard {
    // 初始化 LogTracer 以转发 log 事件到 tracing
    tracing_log::LogTracer::init().expect("Failed to set logger");

    let mut layers = Vec::new();
    let layer = layer();
    layers.push(layer);

    // 日志订阅器
    let subscriber = Registry::default()
        .with(ErrorLayer::default())
        // .with(console_layer)
        .with(layers);

    tracing::subscriber::set_default(subscriber)
}

/// 输出到控制台中
pub fn layer<S>() -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: SubscriberExt,
    S: for<'a> LookupSpan<'a>,
{
    // 本地时间
    let timer = local_time();

    // Shared configuration regardless of where logs are output to.
    let layer = fmt::layer()
        .compact()
        .with_ansi(true)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_timer(timer)
        .with_thread_names(false)
        .log_internal_errors(true)
        .with_writer(std::io::stderr);
    Box::new(layer)
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
