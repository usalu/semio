/** 🧭️ Interactivity audit: audited file constants, severity, patterns and the allowlist.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🏁️run/🟦️.ts
 */


//#region 🔖️InteractivityAudit
/**
 * ⏱️ "UI-reachable" Rust module roots for the forbidden-call audit (ticket
 * `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR`, Phase 0). Single source of truth — widen this list
 * as later phases push more of the repo onto the worker/job substrate (P3c flips
 * [[INTERACTIVITY_AUDIT_SEVERITY]] to `"deny"` once this scope covers everything that must never
 * block the UI thread). Repo-relative, trailing `/`.
 */
export const INTERACTIVITY_AUDIT_UI_ROOTS = ["🧰️framework/🔨️modules/🖱️ui/", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/", "🧰️framework/🛍️products/💻️os/🖥️host/", "✏️s/🔌️plugins/"] as const;

export const INTERACTIVITY_AUDIT_EXACT_BLOCKING_BRIDGE_FILES = ["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs"] as const;

export const INTERACTIVITY_AUDIT_MCP_HTTP_TRANSPORT_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🦀️.rs";
export const INTERACTIVITY_AUDIT_MCP_BRIDGE_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️.rs";
export const INTERACTIVITY_AUDIT_MCP_BRIDGE_SESSION_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧵️bridge/🪢️session/🦀️.rs";
export const INTERACTIVITY_AUDIT_MCP_ROOT_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs";
export const INTERACTIVITY_AUDIT_STORE_SYNC_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs";
export const INTERACTIVITY_AUDIT_DB_STORAGE_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs";
export const INTERACTIVITY_AUDIT_DB_SQLITE_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs";
export const INTERACTIVITY_AUDIT_DB_POSTGRES_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️.rs";
export const INTERACTIVITY_AUDIT_DB_NEO4J_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️.rs";
export const INTERACTIVITY_AUDIT_DB_ENGINE_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs";
export const INTERACTIVITY_AUDIT_ASYNC_FILE = "🧰️framework/🔨️modules/⏳️async/🦀️.rs";
export const INTERACTIVITY_P1X_CONTRACT_FILE = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-ONE-POOL-WORKER-RUNTIME/📓️p1x-db-engine-create-document-catalog-cas-caller-census-2026-08-23.md";
export const INTERACTIVITY_P1Y_CONTRACT_FILE = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-ONE-POOL-WORKER-RUNTIME/📓️p1y-db-compaction-retained-job-caller-census-2026-08-23.md";
export const INTERACTIVITY_P1Z_CONTRACT_FILE = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-ONE-POOL-WORKER-RUNTIME/📓️p1z-db-sync-hello-retained-job-caller-census-2026-08-23.md";
export const INTERACTIVITY_AUDIT_DB_SYNC_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs";
export const INTERACTIVITY_AUDIT_DB_ARTIFACT_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs";
export const INTERACTIVITY_AUDIT_DB_WAL_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs";
export const INTERACTIVITY_AUDIT_DB_STORAGE_FAMILY = ["🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧱️io-core/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/📇️io-registry/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🚦️io-driver/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🗃️memory/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/📁️fs/🦀️.rs"] as const;
export const INTERACTIVITY_AUDIT_DB_ENGINE_FAMILY = ["🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧱️base/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🔭️capability-open/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/📖️catalog-read/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🥾️catalog-bootstrap/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🏗️catalog-create/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🎫️handle/🦀️.rs"] as const;
export const INTERACTIVITY_AUDIT_DB_ARTIFACT_FAMILY = ["🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧬️contract/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/📜️history/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/⚙️engine/🦀️.rs"] as const;
export const INTERACTIVITY_AUDIT_DB_WAL_FAMILY = ["🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🧾️log/🦀️.rs"] as const;
export const INTERACTIVITY_AUDIT_SPR_FORMAT_FILE = "🧰️framework/🔨️modules/📡️replication/📐️format/🦀️.rs";
export const INTERACTIVITY_AUDIT_PACK_FORMAT_FILE = "🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs";
export const INTERACTIVITY_AUDIT_DB_SNAPSHOT_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📸️snapshot/🦀️.rs";
export const INTERACTIVITY_AUDIT_DB_INDEX_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/🦀️.rs";
export const INTERACTIVITY_AUDIT_DB_COMPACT_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs";
export const INTERACTIVITY_AUDIT_DB_CLUSTER_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🌐️cluster/🦀️.rs";
export const INTERACTIVITY_AUDIT_DB_STATE_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔘️state/🦀️.rs";
export const INTERACTIVITY_AUDIT_DB_QUERY_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔍️query/🦀️.rs";
export const INTERACTIVITY_AUDIT_DB_PROJECTION_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📽️projection/🦀️.rs";
export const INTERACTIVITY_AUDIT_DB_ROOT_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🦀️.rs";
export const INTERACTIVITY_AUDIT_DB_CLI_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🦀️.rs";
export const INTERACTIVITY_AUDIT_DB_TESTKIT_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️test/🦀️kit.rs";
export const INTERACTIVITY_AUDIT_HUB_BIN_FILE = "🌎️hub/🏗️bootstrap/🦀️.rs";
export const INTERACTIVITY_AUDIT_HUB_BIN_FAMILY = [INTERACTIVITY_AUDIT_HUB_BIN_FILE, "🌎️hub/🏗️bootstrap/⚙️runtime/🦀️.rs", "🌎️hub/🏗️bootstrap/📒️ledgers/🦀️.rs", "🌎️hub/🏗️bootstrap/🧱️models/🦀️.rs", "🌎️hub/🏗️bootstrap/🧠️state/🦀️.rs", "🌎️hub/🏗️bootstrap/🛰️session-routes/🦀️.rs", "🌎️hub/🏗️bootstrap/📨️document-routes/🦀️.rs", "🌎️hub/🏗️bootstrap/🛣️routes/🦀️.rs"];
export const INTERACTIVITY_AUDIT_PREPARED_RASTER_FILE = "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs";
export const INTERACTIVITY_AUDIT_UI_RECONCILE_FILE = "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs";
export const INTERACTIVITY_AUDIT_UI_RECONCILE_CURSOR_FILE = "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/⏭️cursor/🦀️.rs";
export const INTERACTIVITY_AUDIT_UI_RECONCILE_PRIMITIVES_FILE = "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧱️primitives/🦀️.rs";
export const INTERACTIVITY_AUDIT_UI_VALUE_FILE = "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs";
export const INTERACTIVITY_AUDIT_UI_LAYOUT_FILE = "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📐️layout/🦀️.rs";
export const INTERACTIVITY_AUDIT_UI_BUILDER_FILE = "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs";
export const INTERACTIVITY_AUDIT_UI_COMPONENT_FILE = "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs";
export const INTERACTIVITY_AUDIT_UI_ACCESSIBILITY_FILE = "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs";
export const INTERACTIVITY_AUDIT_UI_SURFACE_FILE = "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🗺️surface/🦀️.rs";
export const INTERACTIVITY_AUDIT_UI_DOCUMENT_FILE = "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs";
export const INTERACTIVITY_AUDIT_UI_LIMITS_FILE = "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🛡️limits/🦀️.rs";
export const INTERACTIVITY_AUDIT_UI_PRESENT_FILE = "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🎭️present/🦀️.rs";
export const INTERACTIVITY_AUDIT_REACTOR_PATCHES_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs";
export const INTERACTIVITY_AUDIT_REACTOR_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs";
// 🧺️ The turn's own file. The turn patch page is assembled HERE, not in `⚛️reactor/🦀️.rs`, so every
// gate string below that names it was reading a file it does not live in and measured nothing.
export const INTERACTIVITY_AUDIT_REACTOR_TURN_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs";
export const INTERACTIVITY_AUDIT_KERNEL_FILE = "🧰️framework/🔨️modules/🎠️kernel/🦀️.rs";
export const INTERACTIVITY_AUDIT_KERNEL_UI_FILE = "🧰️framework/🔨️modules/🎠️kernel/🖼️ui/🦀️.rs";
export const INTERACTIVITY_AUDIT_SHARD_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs";
export const INTERACTIVITY_AUDIT_RUN_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs";
export const INTERACTIVITY_AUDIT_OS_ACTIVATION_FILE = "🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation/🦀️.rs";
export const INTERACTIVITY_AUDIT_RENDERER_RUNTIME_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎠️runtime/🦀️.rs";
export const INTERACTIVITY_AUDIT_PLUGIN_CENTRAL_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs";
export const INTERACTIVITY_AUDIT_PREPARED_RASTER_DRAW_FILE = "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs";
export const INTERACTIVITY_AUDIT_PREPARED_RASTER_GPU_FILE = "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs";
export const INTERACTIVITY_AUDIT_CANVAS_RASTER_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs";
export const INTERACTIVITY_AUDIT_INTERPRETER_RASTER_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs";
export const INTERACTIVITY_AUDIT_RENDERER_GLUE_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs";
export const INTERACTIVITY_AUDIT_RENDERER_HOST_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs";
export const INTERACTIVITY_AUDIT_SURFACE_LANE_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📐️surface-lane/🦀️.rs";
export const INTERACTIVITY_AUDIT_WINIT_HOST_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs";
export const INTERACTIVITY_AUDIT_UI_ENGINE_FILE = "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs";
export const INTERACTIVITY_AUDIT_WINDOW_MEASURE_FILE = "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs";
export const INTERACTIVITY_AUDIT_SHELL_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs";
/** 🔎️ The shell find items the `contracts` crate owns: the audit reads them after the Shell text, because the Shell re-exports them from there. */
export const INTERACTIVITY_AUDIT_FIND_ITEMS_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🤝️contracts/📥️lifted/🔖️identity-and-types/🦀️.rs";
export const INTERACTIVITY_AUDIT_OS_SERVICES_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs";
export const INTERACTIVITY_AUDIT_ENGINE_CANVAS_FILE = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs";
export const INTERACTIVITY_AUDIT_WORLD3D_FILES = ["🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧩️components/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧲️cursors/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧱️scene/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🌉️bridge/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🦀️.rs"];

/**
 * ⏱️ The "single sanctioned runtime module" the thread/pool-construction rule (category
 * `thread-pool`) is scoped OUTSIDE of — this is an authored-production-roots check, unlike the
 * `blocking-bridge`/`sync-*` categories which are scoped to [[INTERACTIVITY_AUDIT_UI_ROOTS]] only.
 * The async primitives crate is the only sanctioned root because it owns the process-wide
 * `WorkerPool` worker constructors. Every service, product, plugin, and domain module remains in
 * scope so an implicit or explicit private scheduler cannot escape the one-pool invariant.
 */
export const INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS = ["🧰️framework/🔨️modules/⏳️async/"] as const;

/** 🏭️ Authored product roots whose Rust sources can execute in an interactive product process. */
export const INTERACTIVITY_AUDIT_PRODUCTION_ROOTS = ["✏️s", "🧰️framework", "🌎️hub"] as const;

/**
 * 🚦️ Severity for `verify interactivity`: `"warn"` reports and always exits 0 (Phase 0 — this
 * ticket); `"deny"` exits non-zero on any un-allowlisted finding or stale allowlist entry (flips at
 * Phase 3 packet P3c, "forbidden-call audit turns from warn to deny" per the master plan). ONE LINE
 * to flip.
 */
export const INTERACTIVITY_AUDIT_SEVERITY: "warn" | "deny" = "deny";

type InteractivityAuditCategory = "blocking-bridge" | "sync-fs" | "sync-net" | "sync-clipboard" | "sync-process" | "sync-db" | "thread-pool" | "tool-run";

export type InteractivityAuditPatternDef = { category: InteractivityAuditCategory; re: RegExp; label: string; scope: "ui" | "runtime-repo-wide" };

/** ⏱️ Every forbidden call pattern, its category, and which scope rule scans it (`"ui"` → [[INTERACTIVITY_AUDIT_UI_ROOTS]] only; `"runtime-repo-wide"` → [[INTERACTIVITY_AUDIT_PRODUCTION_ROOTS]] minus [[INTERACTIVITY_AUDIT_RUNTIME_SANCTIONED_ROOTS]]). */
export const INTERACTIVITY_AUDIT_PATTERNS: readonly InteractivityAuditPatternDef[] = [
  { category: "blocking-bridge", re: /\bblock_on\s*\(/, label: "block_on(", scope: "ui" },
  { category: "blocking-bridge", re: /\brun_blocking\s*\(/, label: "run_blocking(", scope: "ui" },
  { category: "sync-fs", re: /\bstd::fs::/, label: "std::fs::", scope: "ui" },
  { category: "sync-net", re: /\bstd::net::/, label: "std::net::", scope: "ui" },
  { category: "sync-net", re: /\breqwest::blocking\b/, label: "reqwest::blocking", scope: "ui" },
  { category: "sync-clipboard", re: /\barboard::/, label: "arboard::", scope: "ui" },
  { category: "sync-process", re: /\bstd::process::Command\b/, label: "std::process::Command", scope: "ui" },
  { category: "sync-db", re: /\brusqlite::/, label: "rusqlite::", scope: "ui" },
  { category: "sync-db", re: /\bsqlx::/, label: "sqlx::", scope: "ui" },
  { category: "thread-pool", re: /\b(?:std::)?thread::spawn\s*\(/, label: "thread::spawn(", scope: "runtime-repo-wide" },
  { category: "thread-pool", re: /\b(?:std::)?thread::Builder\b/, label: "thread::Builder", scope: "runtime-repo-wide" },
  { category: "thread-pool", re: /\b(?:std::)?thread::scope\s*\(/, label: "thread::scope(", scope: "runtime-repo-wide" },
  { category: "thread-pool", re: /\bWorkerPool::new\s*\(/, label: "WorkerPool::new(", scope: "runtime-repo-wide" },
  { category: "thread-pool", re: /\brayon::/, label: "rayon::", scope: "runtime-repo-wide" },
  { category: "thread-pool", re: /\btokio::runtime::Builder\b/, label: "tokio::runtime::Builder", scope: "runtime-repo-wide" },
];

/**
 * ⏱️ One allow-listed `block_on`/`run_blocking` call site — must carry a reason and the phase that
 * removes/reconciles it, so the list only ever shrinks. `inScope: false` marks a site pre-declared
 * for when [[INTERACTIVITY_AUDIT_UI_ROOTS]] widens (not yet reachable by today's scan — never
 * counted as stale). `expectedNeverToMatch: true` marks a site the scanner structurally cannot see
 * even when in scope (e.g. sits inside a `#[cfg(test)] mod` the test-mod exclusion already skips) —
 * kept for the record without being reported as stale.
 */
export type InteractivityAllowlistEntry = { file: string; lineHint: number; pattern: "block_on" | "run_blocking"; reason: string; phase: string; inScope: boolean; expectedNeverToMatch?: boolean };

/**
 * ⏱️ Known `block_on`/`run_blocking` call sites as of the Phase 0 audit (baseline commit
 * `95b8688ee2`). Verified against the repo, not copied blind from the master ticket. CI fails once `verify interactivity` flips
 * to `"deny"` if an entry here stops matching any real finding (stale) — see `runInteractivityAudit`.
 */
export const INTERACTIVITY_AUDIT_ALLOWLIST: readonly InteractivityAllowlistEntry[] = [
  {
    file: "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs",
    lineHint: 15,
    pattern: "block_on",
    reason: "Native process entry point — the single owned async driver for scale and headless smoke modes; never called by renderer/UI callbacks.",
    phase: "PERMANENT",
    inScope: true,
  },
  {
    file: "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🏗️bootstrap/🦀️.rs",
    lineHint: 255,
    pattern: "block_on",
    reason: "CLI root — approved process entry point (semio_framework_async::block_on(run_async(args))).",
    phase: "PERMANENT",
    inScope: false, // 🏃️run/ is not under any INTERACTIVITY_AUDIT_UI_ROOTS prefix today; pre-declared for when scope widens.
  },
  {
    file: "🧰️framework/🛍️products/💻️os/🖥️host/🔌️plugin-host/🦀️.rs",
    lineHint: 1331,
    pattern: "block_on",
    reason: "CORRECTED: verified test-only, not production. All block_on calls at this anchor (the `🧪️tests/🔬️host-unit` file `include!`d at the end of `pub mod host`) sit inside `#[cfg(test)] mod tests` — a local hygienic block_on helper the R4-clause-5 test-entry-point precedent already sanctions. The audit's test-mod exclusion filters these out on its own; this entry is kept for the record the master ticket asked for, not because the scanner would otherwise flag it.",
    phase: "PERMANENT (test-only)",
    inScope: true,
    expectedNeverToMatch: true,
  },
  {
    file: "🧰️framework/🔨️modules/⏳️async/✨️macros/🦀️.rs",
    lineHint: 58,
    pattern: "block_on",
    reason: "CORRECTED PATH: the master ticket's anchor (…/✨️macros/📦️packages/🦀️rust/🦀️component.rs) does not exist; the real file is …/✨️macros/🦀️.rs (no 📦️packages/🦀️rust segment — 🦀️.rs lives under that nested path instead). #[async_test] expansion generates a hygienically-named `__semio_async_test_block_on` helper via syn/quote! — the emitted identifier never appears as literal `block_on(` source text in THIS file or in any file that uses the attribute, so no static scan (this one included) can see it. Test-only, PERMANENT — kept for the record.",
    phase: "PERMANENT (test-only)",
    inScope: false, // ⏳️async/ is not a UI-reachable root; also structurally invisible to any text-based scanner.
  },
];

export type InteractivityFinding = { category: InteractivityAuditCategory; file: string; line: number; text: string };
//#endregion 🔖️InteractivityAudit
