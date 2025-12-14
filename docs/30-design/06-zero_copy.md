# Zero-Copy Data Design
<!-- 角色：开发者 | 最近验证：2025-12-11 -->

**接口位置**：`src/sources/tcp/zc/types.rs`、`src/sources/net/tcp/`。

## 目标

- 减少 TCP 数据接收路径上的内存拷贝，提升高吞吐场景下的性能。
- 通过 `Arc<Vec<u8>>` 共享 payload，避免在 Source → Parser → Sink 链路上重复克隆数据。
- 提供统一的批处理配置（`BatchConfig`），平衡延迟与吞吐。

## 架构概述

### 核心类型

```rust
/// 零拷贝消息，使用 Arc 共享 payload
pub struct ZcpMessage {
    client_ip: IpAddr,
    is_valid_ip: bool,
    payload: Arc<Vec<u8>>,      // 共享负载
    timestamp_ns: u64,
    payload_len: usize,
}

/// 零拷贝配置
pub struct ZcpConfig {
    pub buffer_size: usize,      // 默认 8KB
    pub pool_capacity: usize,    // 默认 1024
    pub clear_on_return: bool,   // 安全：归还时清空
}

/// 批处理配置
pub struct BatchConfig {
    pub max_batch_size: usize,   // 默认 16
    pub batch_timeout_ms: u64,   // 默认 1ms
    pub batch_capacity: usize,   // 默认 32
}
```

### 数据流

```
TCP 连接
    ↓
TcpConnection::try_read_batch()
    ↓ 拆帧 + 构建 SourceEvent
    ↓ payload 使用 Arc<Vec<u8>> 或 Bytes
SourceBatch (Vec<SourceEvent>)
    ↓
ActPicker 分发
    ↓
ActParser 解析
    ↓ RawData::ArcBytes 保持零拷贝
Sink 输出
```

### 关键实现

1. **连接层**（`src/sources/tcp/conn/connection.rs`）：
   - `TcpConnection` 负责从 socket 读取数据并拆帧
   - `BatchBuilder` 按事件数/字节数构建批次
   - 空闲时自动收缩缓冲区（`SHRINK_HIGH_WATER_BYTES` → `SHRINK_TARGET_BYTES`）

2. **消息类型**（`src/sources/tcp/zc/types.rs`）：
   - `ZcpMessage` 封装零拷贝消息
   - `MessageBatch = Vec<ZcpMessage>` 用于批量传输

3. **通用 TCP 服务**（`src/sources/net/tcp/`）：
   - `TcpServer` 提供通用的零拷贝 TCP 服务框架
   - `BatchProcessor` 处理批次聚合逻辑
   - 支持 Syslog 等协议复用

### 与 SourceEvent 的集成

```rust
// 从 ZcpMessage 构建 SourceEvent
pub fn build_zero_copy_frame(&self, msg: ZcpMessage) -> SourceEvent {
    let payload = RawData::ArcBytes(msg.clone_payload_arc());
    SourceEvent {
        src_key: self.source_key.clone(),
        payload,
        tags: self.base_tags.clone(),
        ups_ip: Some(msg.client_ip()),
        preproc: self.preproc.clone(),
    }
}
```

## 配置参数

| 参数 | 默认值 | 说明 |
|------|--------|------|
| `buffer_size` | 8KB | 单个缓冲区大小 |
| `pool_capacity` | 1024 | 缓冲池容量 |
| `max_batch_size` | 16 | 批次最大消息数 |
| `batch_timeout_ms` | 1ms | 不完整批次的最大等待时间 |
| `SHRINK_HIGH_WATER_BYTES` | 1MB | 触发收缩的容量阈值 |
| `SHRINK_TARGET_BYTES` | 256KB | 收缩后的目标容量 |

## 错误处理

```rust
pub enum ZeroCopyError {
    PoolExhausted { available: usize },
    InvalidIndex { index: usize },
    BufferOverflow { attempted: usize, capacity: usize },
    Config(String),
    Io(std::io::Error),
}
```

## 性能收益

- 高吞吐场景下减少 30-50% 的内存分配
- 降低 GC 压力（通过 Arc 引用计数）
- 批处理减少系统调用次数
