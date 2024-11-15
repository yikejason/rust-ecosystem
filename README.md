### 日志生态
- tracing
- tracing-subcriber
- opentelemetry opentelemetry_otp opentelemetry_sdk tracing-opentelemetry

### 常用 macro 生态
- derive_builder 省去些一个 struct 的 new 方法  字段多的时候非常方便
- derive_more 引入常见的 trait 例如 From Into 减少一些手动实现  标准库trait的自动实现
- strum: enum 相关trait的自动实现

### tokio  std::thread
tokio 用于 io 密集型的操作  读文件  写文件
 std::thread 用于 cpu 密集型计算操作
