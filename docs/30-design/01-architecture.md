# 架构概览（精简版）
<!-- 角色：技术决策者 | 最近验证：2025-10-15 -->

数据流（核心路径）
- Source（采集）→ Parser/Rule（解析与转换）→ Sink（输出）
- Sources/Sinks 经 Factory + ResolvedSpec 构建，解耦实现细节与配置形状。

模块与配置分层
- connectors：外部系统连接定义（source.d/sink.d）
- models：业务模型与路由（models/sources/wpsrc.toml、models/sinks/business.d/infra.d、defaults.toml）
- conf：引擎主配置（conf/wparse.toml）

运行模式（简述）
- daemon：常驻；适合网络接入/服务端组件；启动 acceptor。
- batch：读完即退；文件源等离线场景；不启动 acceptor。
- 详见：`../10-user/01-cli/02-run_modes.md`

启动顺序（要点）
1) Monitor（统计）先启动，统一获取 `MonSend`；
2) Parser 启动并准备好消费通道；
3) Infra 与业务 Sink 先于上游启动，持有各自接收端，避免首次发送遇到 closed；
4) Picker（采集）最后启动；batch 下以 picker 组为主组，全部结束即优雅退出。

设计原则
- 先有消费者，后有生产者；
- 通道的生产/消费端均由任务组持有（TaskManager 管理生命周期）；
- 统一监控通道（单一发送端），避免无人消费导致的发送失败。

人因与接口简化原则
- 面向使用者的配置尽量保持“少而稳”的开关：提供必要的布尔/枚举开关，避免暴露细粒度的可调参数；
- 面向实现细节的调参（阈值/步进/采样）统一在代码内部以常量+自适应策略实现，减少人为失误；
- 示例：TCP Sink 的“发送队列感知 backoff”仅提供 `max_backoff` 一个布尔开关；仅在“无限速”场景缺省开启，有限速强制关闭；具体目标水位/采样周期/退让时长由代码常量与自适应闭环控制，无需用户干预。

延伸阅读
- 角色入口与统一阅读：`README.md`（选型/架构），`../50-dev/README.md`（代码流转）
- 配置实践：`../10-user/02-config/02-sources.md`、`../10-user/02-config/04-sinks_config.md`
