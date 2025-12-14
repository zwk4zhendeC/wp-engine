# wprescue CLI 概览
<!-- 角色：使用配置者 | 最近验证：2025-12-11 -->

wprescue 是恢复/补写专用工具，复用引擎的解析与 Sink 体系，用于将救援目录中的数据按照工程内的 Sink 路由输出到目标。

- 仅支持批处理模式（batch）；不支持 daemon。
- 启动时会注册内置 Sink 工厂（file/null/test_rescue）。
- 常见用法与 `wparse batch` 类似，主要差异是使用 `rescue_root` 作为输入根目录。

## 常用命令

- 批处理运行（推荐）
```bash
wprescue batch --work-root <WR> -n 3000 --stat 3 --print_stat
```

- 工作目录结构（与 wparse 共用）：
  - `conf/wparse.toml`：主配置（其中 `rescue_root` 指向救援数据根）。
  - `models/sinks/{business.d,infra.d,defaults.toml}`：路由与默认项。
  - `connectors/sink.d/*.toml`：连接器定义。

## 退出与日志

- batch 模式下，读取完救援数据后优雅退出；关键日志与 wparse 一致。
- 日志目录默认为 `./logs/`（可配置），格式 `{时间} [LEVEL] [target] {message}`，超过 10MB 自动滚动（10 份，gz）。

## 实现参考

- CLI 入口：apps/wprescue/main.rs
- 运行模式：仅处理 WParseCLI::Batch 分支。
- 工厂注册：`wp_engine::sinks::register_builtin_sinks()`。
