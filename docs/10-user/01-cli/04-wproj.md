# wproj CLI
<!-- 角色：使用配置者 | 最近验证：2025-12-11 -->

本文档定义 wproj 的命令布局、输出与退出码规范，并给出从旧命令到新结构的迁移建议。目标是让 wproj 聚焦“规则/解析/工程工具”，并统一承载 sink 侧的离线统计/校验与联通性测试（含 Kafka 工具、模型脚手架）。

## 设计目标
- 聚焦职责：wproj 专注“规则/解析/工程工具”，并收敛 sink 工具与 Kafka/模型工具，减少多 CLI 分裂。
- 一致体验：统一 `--work-root/--json`、表格/JSON 输出、失败即非 0 退出。
- 平滑迁移：旧命令继续可用（兼容期）；新结构在 help 中清晰呈现。

## 命令总览
- `wproj rule-verify` 规则校验（等价旧 `wproj verify`）
- `wproj parse-run` 离线解析运行（当前等价 analyse 流程）
- `wproj conf …` 配置初始化/清理/校验
- `wproj conf init|clean|check` 工作目录初始化/清理/配置检查（承接旧 `wparse conf`）
  - 多项目检查：`wproj conf check -w <ROOT>` 只在 `<ROOT>` 下递归搜索项目（识别 `<proj>/conf/wparse.toml` 与 `<proj>/wparse.toml`），逐个校验，并输出表格与汇总；存在失败时返回非零退出码。
- `wproj sources check -w <WR>` 检查 `models/sources/wpsrc.toml`（connectors/source.d 白名单与插件校验）
- `wproj sinks check -w <WR>` 检查 `models/sinks/{business.d,infra.d,defaults.toml}`（connectors/sink.d + 路由结构/字段校验）
- `wproj model check -w <WR> --what wpl|oml|all` 检查 WPL/OML 语法与基本索引
- `wproj project check -w <ROOT> --what conf,sources,sinks,wpl,oml,all [--console] [--fail-fast]`
  - 只在 `<ROOT>` 下递归搜索项目，逐一执行对应检查项，并输出表格与汇总；有失败则非零退出码。
- `wproj project …` 工程/环境工具（仅保留体检）
  - `wproj project doctor` 环境体检（等价旧 `wproj os info`）
- `wproj connectors …` 连接器模板
  - `wproj connectors init` 生成最小 connectors 模板（sources/sinks），无参数
  - `wproj connectors lint` Lint/检查 connectors 命名与基础规范（返回非 0 表示存在错误）
- `wproj stat …` 离线统计（保留）
  - `wproj stat src-file` 统计启用文件源的总行数
  - `wproj stat sink-file` 统计文件型 sink 输出行数
- `wproj validate …` 离线校验（保留）
  - `wproj validate sink-file` 按 expect 做比例/区间校验（支持 `--stats-file/--input-cnt`）
 - `wproj knowdb …` 知识库工具（V2）
   - `wproj knowdb init [-w <WR>] [--full]` 生成目录式 KnowDB 骨架（最小化；`--full` 生成包含 base_dir/[default]/[csv]/expected_rows 等完整模板）
   - `wproj knowdb check [-w <WR>]` 校验 KnowDB 目录结构与必要文件
   - `wproj knowdb clean [-w <WR>]` 清理 KnowDB 目录与权威库缓存（models/knowledge 与 .run/authority.sqlite）

> 说明：迁移后，联通性测试（`sink ping`）、Kafka 工具（`kafka`）与模型工具（`model`）均在 `wproj` 提供；`wpsink` 仅保留过渡期兼容。

## 全局参数与约定
- `--work-root, -w`：解析相对路径的根（默认 `.`）
- 检查类命令（conf/sources/sinks/project）：使用 comfy-table 表格输出；长错误信息自动换行并限制总宽度，便于终端阅读。
- `-q, --quiet`：隐藏启动 Banner（Banner 输出到 stderr，不影响 stdout）
- 退出码：`0=成功；2=参数错误；3=执行错误；4=校验失败`

### 统一 JSON 约定
- rule/parse：`{"ok": bool, "action": "verify|analyse|parse.run"}`；失败时包含 `error`
- stat：
  - `src-file`：`{"ok": true, "summary": {"total_enabled_lines": N}, "items": [...]}`
  - `sink-file`：`{"ok": true, "summary": {"total_lines": N}, "items": [...]}`
- validate：`{"ok": bool, "items": [{"severity","group","sink","msg"}]}`（`ok=false` 表示校验失败，进程退出非 0）

### CI 中将 warnings 视为错误
- 提供脚本：`tools/ci-cargo.sh`（构建与测试均以 `-D warnings` 运行）
- 建议在 CI 中使用：
  - `tools/ci-cargo.sh`，或等价的 `RUSTFLAGS="-D warnings" cargo build/test`

## JSON 示例

- rule-verify（成功）
```
{
  "ok": true,
  "action": "verify"
}
```

- rule analyse（失败示例）
```
{
  "ok": false,
  "action": "analyse",
  "error": "cannot load rule: ..."
}
```

- stat src-file（成功）
```
{
  "ok": true,
  "summary": { "total_enabled_lines": 12345 },
  "items": [
    { "key": "file_1", "path": "/abs/a.log", "enabled": true,  "lines": 12000, "error": null },
    { "key": "file_2", "path": "/abs/b.log", "enabled": false, "lines": null,  "error": null }
  ]
}
```

- stat sink-file（成功）
```
{
  "ok": true,
  "summary": { "total_lines": 9876 },
  "items": [
    { "group": "http", "sink": "ok_sink",     "path": "./out/http/ok.dat",     "framework": false, "lines": 9600 },
    { "group": "http", "sink": "residue_sink", "path": "./out/http/residue.dat", "framework": false, "lines": 276  }
  ]
}
```

- validate sink-file（失败）
```
{
  "ok": false,
  "items": [
    { "severity": "ERROR", "group": "http", "sink": "residue_sink", "msg": "actual ratio 0.04 > max 0.02" }
  ]
}
```


## 子命令详解

### A) Check 能力（表格输出）

通用规则
- 搜索范围：只在 `-w <PATH>` 下递归搜索项目根；识别 `<proj>/conf/wparse.toml` 与 `<proj>/wparse.toml` 两种布局。
- 退出码：任一项目检查失败 → 进程退出非 0；表格结尾会打印 `summary` 汇总行。
- 列宽：使用 comfy-table 自动换行并限制总宽（避免错误信息拉宽表格）。

1. 配置检查（conf）
- 命令：`wproj conf check -w <ROOT>`
- 输出列：`project | conf | error`
- 行为：加载主配置（conf/wparse.toml）；失败时在 `error` 列显示原因短语。

2. Sources 检查（sources）
- 命令：`wproj sources check -w <ROOT>`
- 输出列：`project | sources | error/note`
- 行为：解析 `models/sources/wpsrc.toml`，合并 connectors/source.d 并执行白名单与插件校验；缺少 wpsrc.toml 记为 `SKIP`（不计入失败）。

3. Sinks 检查（sinks）
- 命令：`wproj sinks check -w <ROOT>`
- 输出列：`project | sinks | error`
- 行为：加载 connectors/sink.d + defaults.toml，解析 business.d/infra.d 路由并校验；失败时在 `error` 列展示具体上下文（group/sink/connector/file）。

4. 工程汇总（project）
- 命令：`wproj project check -w <ROOT> --what conf,sources,sinks,wpl,oml,all [--console] [--fail-fast]`
- 输出列：`project | conf | sources | sinks | wpl | oml`
- 行为：按 `--what` 聚合执行检查；`--fail-fast` 命中第一处失败立即停止；`--console` 控制日志落点。

5. Sources 列表/路由（展示）
- 列表：`wproj sources list -w <WR>` → 表格列：`id | kind | refs | allow | detail`
- 路由：`wproj sources route -w <WR>` → 表格列：`key | kind | on | detail`

6. Sinks 列表/路由（展示）
- 列表：`wproj sinks list -w <WR>` → 两张表：
  - 汇总：`id | kind | refs`
  - 明细：`connector | route_file | group`（连接器被哪些路由/组引用）
- 路由：`wproj sinks route -w <WR>` → 表格列：`scope(biz/infra) | full_name | connector | target | fmt | detail`

示例
```
# 汇总总览
wproj project check -w usecase/core

# 仅 conf
wproj conf check -w usecase/core

# 仅 sources/sinks
wproj sources check -w usecase/core
wproj sinks check -w usecase/core

# 列表/路由展示
wproj sources list -w usecase/core
wproj sources route -w usecase/core
wproj sinks list -w usecase/core
wproj sinks route -w usecase/core
```

7. Sources 初始化（生成源配置）
- 命令：
  - 简洁：`wproj sources init`（无参）→ 一次性生成 file 与 syslog 两类默认项
  - `wproj sources init --kind file --key <KEY> --base <DIR> --file <NAME> [--encode text] [--connect <ID>] [--overwrite]`
  - `wproj sources init --kind syslog --key <KEY> [--protocol udp|tcp] [--addr <ADDR>] [--port <PORT>] [--connect <ID>] [--overwrite]`
- 行为：将 [[sources]] 以结构体序列化写入 `models/sources/wpsrc.toml`（使用 WarpSources/V2SourceItem）；同名 `key` 则替换，未指定 `--overwrite` 时为在现有文档中追加/更新。
- 同步生成/补全 `connectors/source.d` 模板（如不存在）：
  - file：`00-file-default.toml`（id=`file_src`）、`90-file-gen.toml`（id=`file_gen`）
  - syslog：`10-syslog-udp.toml`（id=`syslog_udp_src`）、`11-syslog-tcp.toml`（id=`syslog_tcp_src`）
  - kafka：`30-kafka.toml`（id=`kafka_src`）
- 示例：
  - `wproj sources init`（生成 2 项：`file_1`、`syslog_1`）
  - `wproj sources init --kind file --key demo_file --base ./data/in_dat --file sample.dat`
  - `wproj sources init --kind syslog --key demo_syslog --protocol tcp --addr 127.0.0.1 --port 1515`

8. Sinks 初始化（生成路由配置）
- 命令：
  - 简洁：`wproj sinks init`（无参）→ 一次性生成 business/demo、infra 骨架与 defaults.toml；可直接使用（不生成 connectors）
  - 高级（可选）：
    - `wproj sinks init --scope business --kind file --group <GROUP> --name <SINK> [--base <DIR>] [--file <NAME>] [--connect <ID>] [--overwrite]`
    - `wproj sinks init --scope business --kind syslog --group <GROUP> --name <SINK> [--protocol udp|tcp] [--addr <ADDR>] [--port <PORT>] [--connect <ID>] [--overwrite]`
    - `wproj sinks init --scope infra    --kind file   --group <GROUP> --name <SINK> [--base <DIR>] [--file <NAME>] [--connect <ID>] [--overwrite]`
    - `wproj sinks init --scope infra    --kind syslog --group <GROUP> --name <SINK> [--protocol udp|tcp] [--addr <ADDR>] [--port <PORT>] [--connect <ID>] [--overwrite]`
- 简洁行为：
  - models/sinks/business.d/demo.toml：预置一个 `file_json_sink` 的 `json` sink（`demo.json`）
  - models/sinks/infra.d：补齐 `default/miss/residue/intercept/error/monitor` 六个兜底组
  - models/sinks/defaults.toml：写入最小 defaults（tags/expect）
- 高级行为：将路由以结构体序列化写入 `models/sinks/<scope>.d/<group>.toml`（使用 RouteFile/RouteGroup），同名 `name` 则替换；`--overwrite` 覆盖整文件。
- 示例：
  - 零参数：`wproj sinks init`
  - 追加业务项：`wproj sinks init --scope business --kind file --group demo --name out_kv --base ./data/out_dat --file demo.kv`



### 1) 规则与解析
- rule-verify（旧 verify 等价）
  - 常用：`wproj rule-verify -w <root> --in-path ./sample.log --rule <expr>`
- rule analyse（旧 ana 等价）
  - 常用：`wproj rule analyse -w <root> --in-path ./sample.log --line-max 3`
- parse-run（当前复用 analyse 流程）
  - 常用：`wproj parse-run -w <root> --in-path ./sample.log`

### 2) 源/汇离线统计与校验（保留）
- 源统计：`wproj stat src-file -w <root> [--json]`
  - JSON 形态：`{ "total_enabled_lines": N, "items": [{ "key", "path", "enabled", "lines?", "error?" }] }`
- sink 统计：`wproj stat sink-file -w <root> [--group ...] [--sink ...] [--path-like ...] [--json]`
  - JSON 形态：`{ "total": N, "items": [{ "group", "sink", "path", "framework", "lines" }] }`
- sink 校验：`wproj validate sink-file -w <root> [--stats-file ./out/stats.json] [--input-cnt N] [--json]`
  - `wproj data validate -w <root> [--input-cnt N]` 为简化入口，仅保留工作目录与分母覆盖两个参数；若需 JSON 输出或筛选组/路径，请直接使用 `wproj validate sink-file ...`。
  - JSON 形态：`{ "pass": bool, "issues": [{ "severity", "group", "sink", "msg" }] }`
  - 分母口径与 expect 详见 `../03-sinks/defaults_expect.md`

提示：以上命令均可加 `-q/--quiet` 隐藏 Banner，例如 `wproj -q stat src-file -w <root> --json`。

### 3) 工程/环境
- 体检：`wproj project doctor`
  - 打印 OS/Arch/版本等；未来可拓展更多检查项

## Sink/Kafka/Model 工具（并入 wproj）
- 联通性测试：`wproj sink ping file|mysql|clickhouse|es|syslog|kafka`、`wproj sink ping-config`
- Kafka 工具：`wproj kafka {produce,consume}`
- 模型脚手架：`wproj model {example,benchmark,new,all}`

## 迁移映射（旧 → 新）
- 规则：
  - `wproj verify` → `wproj rule-verify`（旧命令仍可用）
  - `wproj ana` → `wproj rule analyse`（旧命令仍可用）
- 解析运行：
  - （新增）`wproj parse-run`（当前等价 analyse 流程）
- 源/汇统计与校验（保留）：
  - `wproj stat src-file`（不变）
  - `wproj stat sink-file`（不变）
  - `wproj validate sink-file`（不变）
— 迁移完成（旧 → 新）：
  - `wpsink ping …` → `wproj sink ping …`
  - `wpsink ping-from-config` → `wproj sink ping-config`
  - `wpsink kafka …` → `wproj kafka …`
  - `wpcfg ldm …` → `wproj model …`

## 示例
- 统计 sink 输出并校验（JSON）：
  - `wproj stat sink-file -w usecase/core/getting_started --json`
  - `wproj validate sink-file -w usecase/core/getting_started --json`
- 规则校验：
  - `wproj rule-verify -w . --in-path ./sample.log --rule "..." --print_stat`
- 联通性测试：
  - `wproj sink ping-config -w usecase/core/getting_started --json`

## 版本与兼容
- 兼容期：旧命令（`verify/ana/os info` 等）仍可用；帮助中优先展示新结构。
- 建议：脚本逐步迁移至 `wproj rule/parse/project/sink/kafka/model`；`wpsink` 仅在过渡期保留。

## 后续规划
- 统一 rule/parse 的 JSON 输出与退出码（与 stat/validate 对齐）。
- 根据使用反馈，在 `wproj` 中为旧命令追加 deprecate 提示（不改变行为）。
- `project doctor` 扩展更多检查项（feature/插件/依赖诊断）。

注：本文为使用者文档；开发细节与代码路径请参考 `docs/dev/sinks_code_flow.md`。
- 连接器模板（Connectors）
  - 命令：`wproj connectors init`
  - 行为：在 `connectors/` 下生成基础模板（如缺失）：
    - sources：`source.d/00-file-default.toml`（id=`file_src`）、`10-syslog-udp.toml`、`11-syslog-tcp.toml`
    - sinks：`sink.d/01-file-prototext.toml`、`02-file-json.toml`、`03-file-kv.toml`、`04-file-raw.toml`、`10-syslog-udp.toml`、`11-syslog-tcp.toml`
  - Lint：`wproj connectors lint` → 检查项：
    - 文件名是否符合 `NN-<kind>-<variant>.toml`
    - id 是否仅含 `[a-z0-9_]`
    - syslog 源 id 是否为 `syslog_*_src`；syslog 汇 id 是否为 `syslog_*_sink`
    - file 源/汇 id 是否以 `file_` 开头
    - 文件名 kind 提示与 `type` 字段是否一致（不一致为 WARN）
