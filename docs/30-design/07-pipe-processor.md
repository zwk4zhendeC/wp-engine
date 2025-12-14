# PipeProcessor 插件设计
<!-- 角色：开发者 | 最近验证：2025-11-27 -->

**接口位置**：`crates/wp-lang/src/eval/builtins/registry.rs`、`crates/wp-lang/src/traits.rs`。

## 概述

PipeProcessor 插件扩展 wp-lang 的数据处理管道，为 WPL 解析添加自定义的数据转换功能。插件处理器可以串联在解析管道中，对字段数据进行解码、格式化、验证等操作。

## 注册宏方案

### 单个处理器注册

```rust
use wp_lang::register_wpl_pipe;

// 注册单个处理器
register_wpl_pipe!("my-processor", my_builder_function);
```

### 批量处理器注册

```rust
use wp_lang::eval::builtins::registry::register_wpl_pipe_batch;

// 批量注册多个处理器
register_wpl_pipe_batch([
    ("text/uppercase", build_uppercase_processor as _),
    ("text/lowercase", build_lowercase_processor as _),
    ("format/json-pretty", build_json_formatter as _),
    ("decode/custom-base64", build_custom_base64_processor as _),
]);
```

## 在 WPL 中使用

注册后，处理器可以在 WPL 规则中使用：

```wpl
rule my_rule {
    plg_pipe(text/uppercase) {
        (chars@message)
    }
}
```

或者使用简化语法：

```wpl
rule my_rule {
    (text/uppercase) (chars@message)
}
```

多个处理器可以串联使用：

```wpl
rule process_log {
    plg_pipe(unquote/unescape) plg_pipe(text/uppercase) {
        (chars@message)
    }
}
```

## 完整示例：自定义处理器插件

### 1. 项目结构

```
my-wp-lang-processors/
├── Cargo.toml
└── src/
    ├── lib.rs
    └── processors.rs
```

### 2. Cargo.toml

```toml
[package]
name = "my-wp-lang-processors"
version = "0.1.0"
edition = "2021"

[dependencies]
wp-lang = "1.0"
base64 = "0.21"
serde_json = "1.0"
orion-error = "0.1"
bytes = "1.0"
```

### 3. processors.rs - 实现自定义处理器

```rust
use std::sync::Arc;
use wp_lang::traits::{PipeProcessor, PipeHold};
use wp_parse_api::{RawData, WparseResult};

// === 文本转换处理器 ===

#[derive(Debug)]
pub struct UpperCaseProcessor;

impl PipeProcessor for UpperCaseProcessor {
    fn process(&self, data: RawData) -> WparseResult<RawData> {
        match data {
            RawData::String(s) => {
                let upper = s.to_uppercase();
                Ok(RawData::from_string(upper))
            },
            RawData::Bytes(b) => {
                let upper = String::from_utf8_lossy(&b).to_uppercase();
                Ok(RawData::from_string(upper))
            },
            RawData::ArcBytes(b) => {
                let upper = String::from_utf8_lossy(&b).to_uppercase();
                Ok(RawData::from_string(upper))
            },
        }
    }

    fn name(&self) -> &'static str {
        "text/uppercase"
    }
}

pub fn build_uppercase_processor() -> PipeHold {
    Arc::new(UpperCaseProcessor)
}

#[derive(Debug)]
pub struct LowerCaseProcessor;

impl PipeProcessor for LowerCaseProcessor {
    fn process(&self, data: RawData) -> WparseResult<RawData> {
        match data {
            RawData::String(s) => {
                let lower = s.to_lowercase();
                Ok(RawData::from_string(lower))
            },
            RawData::Bytes(b) => {
                let lower = String::from_utf8_lossy(&b).to_lowercase();
                Ok(RawData::from_string(lower))
            },
            RawData::ArcBytes(b) => {
                let lower = String::from_utf8_lossy(&b).to_lowercase();
                Ok(RawData::from_string(lower))
            },
        }
    }

    fn name(&self) -> &'static str {
        "text/lowercase"
    }
}

pub fn build_lowercase_processor() -> PipeHold {
    Arc::new(LowerCaseProcessor)
}

// === 解码处理器 ===

#[derive(Debug)]
pub struct CustomBase64Processor;

impl PipeProcessor for CustomBase64Processor {
    fn process(&self, data: RawData) -> WparseResult<RawData> {
        match data {
            RawData::String(s) => {
                let decoded = general_purpose::STANDARD
                    .decode(s.as_bytes())
                    .owe_data()
                    .want("base64 decode")?;
                let vstring = String::from_utf8(decoded).owe_data().want("to-json")?;
                Ok(RawData::from_string(vstring))
            },
            RawData::Bytes(b) => {
                let decoded = general_purpose::STANDARD
                    .decode(b.as_ref())
                    .owe_data()
                    .want("base64 decode")?;
                Ok(RawData::Bytes(Bytes::from(decoded)))
            },
            RawData::ArcBytes(b) => {
                let decoded = general_purpose::STANDARD
                    .decode(b.as_ref())
                    .owe_data()
                    .want("base64 decode")?;
                Ok(RawData::ArcBytes(Arc::from(decoded)))
            },
        }
    }

    fn name(&self) -> &'static str {
        "decode/custom-base64"
    }
}

pub fn build_custom_base64_processor() -> PipeHold {
    Arc::new(CustomBase64Processor)
}

// === 格式化处理器 ===

#[derive(Debug)]
pub struct JsonPrettyFormatter;

impl PipeProcessor for JsonPrettyFormatter {
    fn process(&self, data: RawData) -> WparseResult<RawData> {
        let text = match &data {
            RawData::String(s) => s.clone(),
            RawData::Bytes(b) => String::from_utf8_lossy(b).into_owned(),
            RawData::ArcBytes(b) => String::from_utf8_lossy(b).into_owned(),
        };

        // 尝试解析并重新格式化 JSON
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) {
            let formatted = serde_json::to_string_pretty(&parsed)
                .map_err(|e| WparseError::from(WparseReason::LineProc(
                    format!("JSON format error: {}", e)
                )))?;
            Ok(RawData::from_string(formatted))
        } else {
            // 如果不是有效的 JSON，返回原始数据
            Ok(data)
        }
    }

    fn name(&self) -> &'static str {
        "format/json-pretty"
    }
}

pub fn build_json_formatter() -> PipeHold {
    Arc::new(JsonPrettyFormatter)
}

// === 验证处理器 ===

#[derive(Debug)]
pub struct EmailValidator;

impl PipeProcessor for EmailValidator {
    fn process(&self, data: RawData) -> WparseResult<RawData> {
        let text = match &data {
            RawData::String(s) => s.clone(),
            RawData::Bytes(b) => String::from_utf8_lossy(b).into_owned(),
            RawData::ArcBytes(b) => String::from_utf8_lossy(b).into_owned(),
        };

        // 简单的邮箱验证
        if text.contains('@') && text.contains('.') {
            Ok(data) // 有效邮箱，保持原样
        } else {
            Err(WparseError::from(WparseReason::LineProc(
                format!("Invalid email format: {}", text)
            )))
        }
    }

    fn name(&self) -> &'static str {
        "validate/email"
    }
}

pub fn build_email_validator() -> PipeHold {
    Arc::new(EmailValidator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wp_lang::wp_parse_api::RawData;

    #[test]
    fn test_uppercase_processor() {
        let processor = UpperCaseProcessor;
        let input = RawData::from_string("hello".to_string());
        let result = processor.process(input).unwrap();

        assert_eq!(processor.name(), "text/uppercase");
        assert_eq!(
            wp_lang::eval::builtins::raw_to_utf8_string(&result),
            "HELLO"
        );
    }

    #[test]
    fn test_base64_processor() {
        let processor = CustomBase64Processor;
        let input = RawData::from_string("SGVsbG8gV29ybGQ=".to_string()); // "Hello World"
        let result = processor.process(input).unwrap();

        assert_eq!(processor.name(), "decode/custom-base64");
        assert_eq!(
            wp_lang::eval::builtins::raw_to_utf8_string(&result),
            "Hello World"
        );
    }
}
```

### 4. lib.rs - 插件入口和注册

```rust
pub mod processors;

// 提供初始化函数，在应用启动时调用
pub fn init() {
    use wp_lang::eval::builtins::registry::register_wpl_pipe_batch;

    register_wpl_pipe_batch([
        ("text/uppercase", processors::build_uppercase_processor as _),
        ("text/lowercase", processors::build_lowercase_processor as _),
        ("decode/custom-base64", processors::build_custom_base64_processor as _),
        ("format/json-pretty", processors::build_json_formatter as _),
        ("validate/email", processors::build_email_validator as _),
    ]);
}

// 可选：导出处理器类型供外部直接使用
pub use processors::{
    UpperCaseProcessor, LowerCaseProcessor, CustomBase64Processor,
    JsonPrettyFormatter, EmailValidator
};
```

### 5. 在主应用中使用

```rust
// 在你的主应用中
use my_wp_lang_processors::init;

fn main() {
    // 初始化自定义处理器
    init();

    // 现在可以在 WPL 规则中使用这些处理器
    let wpl_rule = r#"
        rule process_data {
            plg_pipe(text/uppercase) {
                plg_pipe(format/json-pretty) {
                    (chars@json_data)
                }
            }
        }
    "#;

    // 解析和使用规则...
}
```

### 6. 管理已注册的处理器

```rust
use wp_lang::eval::builtins::registry::{
    create_pipe_unit,
    list_pipe_units
};

// 列出所有已注册的处理器
let processors = list_pipe_units();
println!("Available processors: {:?}", processors);

// 动态创建处理器实例
if let Some(processor) = create_pipe_unit("text/uppercase") {
    let input = RawData::from_string("hello world".to_string());
    let result = processor.process(input)?;
    println!("Result: {:?}", result);
}
```

## 最佳实践

### 命名约定
- 使用清晰的命名空间：`category/action`
- 常见类别：`decode/`、`encode/`、`format/`、`text/`、`validate/`
- 示例：`decode/base64`、`format/json-pretty`、`text/lowercase`

### 错误处理
```rust
.owe_data()
.want("descriptive operation name")
```

### 类型处理
- 正确处理所有 `RawData` 类型：`String`、`Bytes`、`ArcBytes`
- 保持输入类型语义，除非转换是必要的
- 对于字符串输入，优先尝试保持字符串输出

通过这种方式，你可以创建功能强大、易于维护的 PipeProcessor 插件，为 wp-lang 添加丰富的数据处理能力。
