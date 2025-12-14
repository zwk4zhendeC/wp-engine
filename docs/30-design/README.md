# 设计文档索引（Design）
<!-- 最近验证：2025-12-11 -->

本目录包含 wp-engine 的技术设计文档，面向开发者和架构师。

## 文档列表

| 文档 | 说明 |
|------|------|
| [01-architecture.md](01-architecture.md) | 架构概览（精简版）：数据流、模块分层、启动顺序、设计原则 |
| [02-event_prehook.md](02-event_prehook.md) | DSourceEvent + EventPreHook 设计：面向并行与背压的接口演进 |
| [03-pick_policy.md](03-pick_policy.md) | ActPicker 与 Policy 设计：调度策略器（Post/Pull Policy）实现 |
| [04-pick_tuning.md](04-pick_tuning.md) | 批/队列/拉取策略调优参数：内存/吞吐折中的四种档位 |
| [05-tcp_source.md](05-tcp_source.md) | TCP Source 设计：多实例 SourceFactory 接口草案 |
| [06-zero_copy.md](06-zero_copy.md) | Zero-Copy Data 设计：基于 Arc 的零拷贝消息传递 |
| [07-pipe-processor.md](07-pipe-processor.md) | PipeProcessor 插件设计：WPL 管道处理器扩展机制 |
| [08-knowledge_db.md](08-knowledge_db.md) | KnowDB 运行形态 ADR：Authority DB vs In-Memory 取舍 |
| [09-rescue.md](09-rescue.md) | 救急文件结构化回放方案：RescueEntry V1 设计 |

## 快速导航

### 数据流相关
- 架构概览 → [01-architecture.md](01-architecture.md)
- 事件预处理 → [02-event_prehook.md](02-event_prehook.md)
- 零拷贝传输 → [06-zero_copy.md](06-zero_copy.md)

### 调度与性能
- Picker 策略 → [03-pick_policy.md](03-pick_policy.md)
- 调优参数 → [04-pick_tuning.md](04-pick_tuning.md)

### 插件与扩展
- Source 工厂 → [05-tcp_source.md](05-tcp_source.md)
- Pipe 处理器 → [07-pipe-processor.md](07-pipe-processor.md)

### 数据存储
- 知识库 → [08-knowledge_db.md](08-knowledge_db.md)
- 救急恢复 → [09-rescue.md](09-rescue.md)

## 相关文档

- 用户配置：`../10-user/02-config/`
- 开发指南：`../20-dev/`
