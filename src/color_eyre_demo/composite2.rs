#![allow(unused)]

use std::fs::read_to_string;

use color_eyre::{Result, eyre::WrapErr};

fn read_file() -> Result<()> {
    let content = read_to_string("path/to/file").wrap_err("Failed to read file")?;
    println!("{}", content);
    Ok(())
}

fn msg1() -> Result<()> {
    read_file().context("msg1 failed")?;
    println!("this is a msg1");
    Ok(())
}

fn msg2() -> Result<()> {
    msg1().context("msg2 failed")?;
    println!("this is a msg2");
    Ok(())
}

fn msg3() -> Result<()> {
    msg2().context("msg3 failed")?;
    println!("this is a msg3");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_color_eyre() -> color_eyre::Result<()> {
        color_eyre::install()?;

        if let Err(e) = msg3() {
            println!("{}", e);
            println!("====================================");
            println!("{:?}", e);
            println!("====================================");
            println!("{:#?}", e);
        }

        Ok(())
    }

    #[test]
    fn it_disable_backtrace() -> color_eyre::Result<()> {
        color_eyre::config::HookBuilder::default()
            .display_location_section(true) // 不显示位置信息
            .display_env_section(false) // 不显示环境信息
            .capture_span_trace_by_default(false) // 不捕获完整的调用链
            .panic_section(false) // 禁用 panic 信息
            .add_default_filters() // 默认过滤器
            .add_frame_filter(Box::new(|frames| {
                // 过滤掉匹配的函数帧
                frames.retain(|_frame| false);
            })) // 禁用所有调用链帧
            .install()?;

        if let Err(e) = msg3() {
            println!("{:?}", e);
        }

        Ok(())
    }

    #[test]
    fn it_disable_std_backtrace() -> color_eyre::Result<()> {
        color_eyre::config::HookBuilder::default()
            .display_location_section(true) // 不显示位置信息
            .display_env_section(false) // 不显示环境信息
            .capture_span_trace_by_default(false) // 不捕获完整的调用链
            .panic_section(false) // 禁用 panic 信息
            .add_default_filters() // 默认过滤器
            .add_frame_filter(Box::new(|frames| {
                // 定义需要过滤的函数名列表
                let filters = &["std::", "test::", "<core::", "core::", "<E as eyre::"];

                // 过滤掉匹配的函数帧
                frames.retain(|frame| {
                    !filters.iter().any(|f| {
                        if let Some(name) = frame.name.as_ref() {
                            name.as_str().starts_with(f)
                        } else {
                            false
                        }
                    })
                });
            })) // 过滤指定函数
            .install()?;

        if let Err(e) = msg3() {
            println!("{:?}", e);
        }

        Ok(())
    }
}
