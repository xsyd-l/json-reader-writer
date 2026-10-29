# json-reader-writer

一个使用 Rust 编写的轻量级 JSON 解析与序列化练习项目。

## 功能

- 将 JSON 文本解析为 `Json` 枚举值，支持对象、数组、字符串、整数、浮点数、布尔值和 `null`。
- 将 `Json` 值序列化为 JSON 文本。
- 从文件读取 JSON，以及将 JSON 值写入文件。

> 当前实现用于学习和简单场景，尚未覆盖完整 JSON 规范。请勿将其视为经过充分验证的通用 JSON 库。

## 环境要求

- Rust 工具链（包含 Cargo）

## 构建与运行

在项目根目录执行：

```bash
cargo build
cargo run
```

当前示例程序从 `D:\MyProject\json\test.json` 读取 JSON 并将解析后的内容打印到标准输出。若在其他位置运行，请相应修改 `src/main.rs` 中的文件路径。

## 代码接口

核心实现位于 `src/json.rs`。`Json` 提供以下方法：

- `Json::from(&str)`：从 JSON 文本创建 `Json` 值。
- `to_string()`：将值序列化为 JSON 文本。
- `Json::read(&str)`：从文件路径读取 JSON。
- `write(&str)`：将值写入文件路径。

## 许可证

本项目采用 [MIT License](./LICENSE)。
