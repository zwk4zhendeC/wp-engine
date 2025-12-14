CLI Layers Overview

This workspace splits CLI-related code into three crates with clear roles and dependencies:

- wp-cli-utils (a.k.a. wpcnt_lib)
  - Purpose: shared utilities for CLI features (pretty printing, counting lines, simple validators, helpers, types).
  - Dependencies: standard libs + wp-config types (read-only). It must not depend on wp-engine.
  - Consumers: used by core and shim apps (wparse/wproj/wpgen/wprescue).

- wp-cli-core
  - Purpose: configuration-bound logic for sources/sinks/knowdb/obs (resolve connectors, merge params, route building, stats, validation).
  - Dependencies: wp-config + wp-cli-utils; keep it free of wp-engine to avoid dependency cycles.
  - API: returns structured rows/reports; no UI/printing side effects.

- wp-cli
  - Purpose: thin packaging on top of wp-engine context (ConfManager, EngineConfig). It marshals work_root, resolves sink_root, and delegates to wp-cli-core.
  - Policy: avoid duplicating types; re-export core types and forward results.
  - Consumers: shim binaries in apps-shim/* call into wp-cli.

Shim Apps (apps-shim/*)
- wparse-shim, wpgen-shim, wproj-shim, wprescue-shim are thin binaries that wire argument parsing to wp-cli (and via that, to core/engine).
- They intentionally keep minimal logic to speed up iteration and reduce binary coupling.

Design Guidelines
- No duplication: prefer `pub use` to re-export types from core, and forward functions (`Ok(core(... )?)`).
- Keep printing/formatting in utils; keep policy and resolution in core; keep engine/context wiring in wp-cli.
- Avoid cycles: wp-engine must not depend on wp-cli; wp-cli-core must not depend on wp-engine.

Testing Guidance
- Core (wp-cli-core): unit tests create a temporary work tree (connectors + models) and assert route building, stats, and validation.
- CLI (wp-cli): integration tests (in `crates/wp-cli/tests/`) initialize a test work tree via `ConfManager::new_for_tests` and exercise packaging calls:
  - `obs::stat::stat_file_combined`
  - `obs::validate::validate_sink_file`

Common Pitfalls
- File sink params: engine-side validation requires either `path`, or `file`/`base`. When using only `file`, the default base is `./data/out_dat`.
- Connectors allowlist: overrides must be listed in `allow_override` for the connector (`path`, `file`, `base`, `fmt`, ...).

Verification
- Lint/format: `cargo fmt --all && cargo clippy --workspace --all-targets --all-features`
- Tests: `cargo test --workspace --all-features` (use `-- --nocapture` for verbose logs)
