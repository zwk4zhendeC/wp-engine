# 开发者（Dev）索引
<!-- 角色：开发者 | 最近验证：2025-10-15 -->

你关心：实现/扩展 Sink、Factory 接口、运行期装配与调试。

建议阅读
- 代码流转（config → resolved → factory → runtime）：`docs/dev/sinks_code_flow.md`
- SinkFactory API 与内置：`src/sinks/builtin_factories.rs`、`crates/apis/wp-sink-api/src/lib.rs`
- 配置装配（合并/校验）：`crates/wp-config/src/assemble/sinks/build.rs`、`crates/wp-config/src/assemble/sinks/types.rs`
- 组结构：`crates/wp-config/src/structure/group.rs`
- 运行期派发：`src/services/sink/act_sink.rs`、`src/conf/sinks.rs`
