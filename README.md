# 错误日志库使用示例

## 应用层

推荐使用 `anyhow` 库进行错误日志记录，借助Context记录完整调用链路。

## Lib 库

推荐使用 `thiserror` 库进行错误类型定义，如果 `Error` 比较大时可考虑拆分 `Error`。
注意 `thiserror` 当前不支持调用链的回溯。
