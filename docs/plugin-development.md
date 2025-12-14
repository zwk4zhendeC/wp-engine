# WPL Plugin Development Guide

This guide shows how to create plugins for the Warp Parse Language (WPL) system using the minimal `wp-parse-api` crate.

## Overview

WPL plugins implement the `Parsable` trait to process raw data and produce structured results. The plugin API is intentionally minimal to reduce migration costs and maintain compatibility.

## Quick Start

### 1. Create a New Plugin Project

```bash
cargo new --lib my-wpl-plugin
cd my-wpl-plugin
```

### 2. Add Dependencies

Add to your `Cargo.toml`:

```toml
[dependencies]
wp-parse-api = "1.0"  # Use the same version as wp-engine
wpl = "2.0"            # For RawData and DataResult types
```

### 3. Implement the Plugin Trait

```rust
use wp_parse_api::{Parsable, RawData, DataResult};
use wpl::WparseError;

pub struct MyParser {
    name: String,
}

impl MyParser {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Parsable for MyParser {
    fn parse(&self, data: &RawData, _successful_others: usize) -> DataResult {
        // Your parsing logic here
        // data contains the raw input to process

        // Example: simple success case
        // Ok(data.successful_item(your_parsed_data))

        // Example: error case
        Err(WparseError::from(wpl::WparseReason::Plugin(
            format!("{}: parsing failed", self.name)
        )))
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// Factory function for plugin registration
pub fn build_parser(
    name: String,
    _vm: wpl::WplEvaluator,
) -> Box<dyn Parsable + Send + Sync> {
    Box::new(MyParser::new(name))
}
```

## Plugin Architecture

### Core Components

1. **Parsable Trait**: Main interface that all plugins must implement
2. **RawData**: Input data structure containing raw bytes/information
3. **DataResult**: Result type for successful parsing or errors
4. **ParserHandle**: Boxed trait object for dynamic dispatch

### Trait Methods

#### `proc(&self, data: &RawData, successful_others: usize) -> DataResult`

Process raw input data and return a structured result.

- `data`: Raw input data to be parsed
- `successful_others`: Number of other successful parses (useful for coordination)
- Returns: `DataResult` with either parsed data or error information

#### `name(&self) -> &str`

Return the plugin's identifier name used for logging and debugging.

## Integration with wp-engine

### Feature-Based Integration

To integrate your plugin with wp-engine:

1. Add your plugin as a workspace dependency in `wp-engine/Cargo.toml`
2. Add a feature flag for your plugin
3. Use conditional compilation to include your plugin

```rust
// In wp-engine/src/lib.rs
#[cfg(feature = "my-plugin")]
pub mod my_plugin;

// In wp-engine/src/core/parser/plugins/factory.rs
#[cfg(feature = "my-plugin")]
use crate::my_plugin::build_parser as my_builder;
```

### Plugin Registration

Plugins are typically registered through factory functions that create boxed trait objects:

```rust
pub fn build_parser(
    name: String,
    vm: wpl::WplEvaluator,
) -> Box<dyn Parsable + Send + Sync> {
    // Your plugin instance creation logic
}
```

## Example: Simple Log Parser

Here's a complete example of a plugin that parses Apache-style log entries:

```rust
use wp_parse_api::{Parsable, RawData, DataResult};
use wpl::{WparseError, WparseReason, WplEvaluator};
use std::collections::HashMap;

pub struct ApacheLogParser {
    name: String,
}

impl ApacheLogParser {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    fn parse_apache_log(&self, line: &str) -> Result<HashMap<String, String>, String> {
        // Simple Apache log parsing
        // Format: IP - - [timestamp] "method path version" status size
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 7 {
            return Err("Invalid log format".to_string());
        }

        let mut result = HashMap::new();
        result.insert("ip".to_string(), parts[0].to_string());
        result.insert("method".to_string(), parts[3].trim_matches('"').to_string());
        result.insert("path".to_string(), parts[4].to_string());
        result.insert("status".to_string(), parts[5].to_string());
        result.insert("size".to_string(), parts[6].to_string());

        Ok(result)
    }
}

impl Parsable for ApacheLogParser {
    fn parse(&self, data: &RawData, _successful_others: usize) -> DataResult {
        // Extract string data from RawData
        let input = String::from_utf8_lossy(&data.key);

        match self.parse_apache_log(&input) {
            Ok(parsed) => {
                // Convert parsed HashMap to the expected DataResult format
                // This depends on your specific data model
                data.successful_item(parsed)
            }
            Err(e) => Err(WparseError::from(WparseReason::Plugin(
                format!("{}: {}", self.name, e)
            )))
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub fn build_parser(
    name: String,
    _vm: wpl::WplEvaluator,
) -> Box<dyn Parsable + Send + Sync> {
    Box::new(ApacheLogParser::new(name))
}
```

## Migration from Old API

If you have existing plugins using the deprecated `wpl::Parsable`, updating is straightforward:

1. Update imports:
   ```rust
   // Old
   use wpl::{Parsable, RawData, DataResult};

   // New
   use wp_parse_api::{Parsable, RawData, DataResult};
   ```

2. Update your `Cargo.toml` to include `wp-plugin-api`

3. The trait interface remains identical, so no code changes are needed

## Best Practices

1. **Error Handling**: Provide descriptive error messages that help with debugging
2. **Performance**: Avoid unnecessary allocations in hot paths
3. **Thread Safety**: Ensure your implementation is `Send + Sync`
4. **Logging**: Use the plugin name for consistent log identification
5. **Testing**: Write comprehensive tests for various input scenarios

## Testing Your Plugin

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wpl::{RawData, PkgID};

    #[test]
    fn test_plugin_basic() {
        let parser = MyParser::new("test".to_string());

        // Create test data
        let pkg_id = PkgID::new("test");
        let raw_data = RawData::new("test input", &pkg_id);

        // Test processing
        let result = parser.proc(&raw_data, 0);

        // Assert results based on your plugin's behavior
        assert!(result.is_ok() || result.is_err());
    }
}
```

## API Reference

### Types

- `ParserHandle`: `Box<dyn Parsable + Send + Sync>`
- `RawData`: Input data container (re-exported from `wpl`)
- `DataResult`: Result type (re-exported from `wpl`)

### Trait

```rust
pub trait Parsable: Send + Sync {
    fn parse(&self, data: &RawData, successful_others: usize) -> DataResult;
    fn name(&self) -> &str;
}
```

## Support

For questions or issues with plugin development:

1. Check existing plugin implementations in the `wp-engine` codebase
2. Refer to the core parsing logic in `crates/wp-lang`
3. Consult the main documentation for the Warp Parse system
