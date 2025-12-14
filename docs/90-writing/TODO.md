# 文档后续 TODO（迭代清单）

优先级 P0（近期）
- 参考页表格化：为 `docs/reference/params/*` 与 `docs/reference/schemas/*` 增加表格（字段/类型/默认/约束/示例），覆盖所有内置 Source/Sink 类型。
- 链接完整性校验：新增脚本（如 `tools/docs-link-check.sh`）并在 CI 执行，防止后续改动引入断链。
- 排障升级：在 `docs/user/guides/troubleshooting.md` 增补常见错误的“报错原文 + 修复指引”，包含：
  - tags 校验（key/value 违规、数量超限）
  - connectors 未找到（起始目录/向上查找）
  - allow_override 违规（覆写超出白名单/嵌套 params）
  - feature 未启用（kafka/db 等）
- 用例 README 收敛：统一 `usecase/*/README.md` 文案到新结构（入口、路径、命令），减少重复叙述。

优先级 P1（下一期）
- 连接器复用方案（profiles/inherit）：在概念与配置中补充设计与示例；若落地实现，完善 `wpgen_output_connectors.md`。
- Sinks tags 注入示例：在 `docs/user/config/sinks.md` 增补“注入前后对比”的数据样例与注意事项（Raw 不注入）。
- 高级示例：
  - Kafka（TLS/SASL）、Elasticsearch（云端/鉴权）、ClickHouse（鉴权/数据库），以“连接器 + routes”组合形式给出可参考模板。
- 术语对齐表：在 `docs/README.md` 增补术语表（Source/Sink/Connector/ResolvedSpec/Factory/Group/OML）。

优先级 P2（可选）
- 文档站点化：评估 mkdocs 或 mdBook，生成侧边导航与全文搜索（不改变仓库原文结构）。
- 版本与发布节奏：在 `docs/README.md` 增补“文档版本说明”和“更新记录”约定。
- 贡献指南（文档向）：在 `docs/dev/contributing.md` 附带“文档提交规范”（命名、链接、示例最小化）。

执行建议
- 每次改动先本地运行链接检查脚本；提交后由 CI 再验一遍。
- 参考页更新时，优先从源码注释与实现默认值中提取，确保与代码一致。
