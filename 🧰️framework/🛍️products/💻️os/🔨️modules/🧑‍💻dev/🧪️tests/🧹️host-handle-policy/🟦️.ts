/** 🧩️ Semantic host handle policy owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";

import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";

import {
  BundleScript,
  ScriptRouter,
  buildBudgetMs,
  daemonBudgetOpts,
  describeDevPortOccupant,
  devServerUrl,
  getWorkspaceRoot,
  getRepoMetaDir,
  isDevPortInUse,
  loadFrameworkOsPlaygroundCatalog,
  wgpuDevPlayUrl,
  runBundleScriptMain,
  runCmd,
  runCmdStatus,
  runBunxStatus,
  runNodeBinStatus,
  runProbe,
  runVitest,
  spawnDaemon,
  type SpawnDaemonHandle,
  runViteBunxDev,
  frameworkOsPlaygroundDefaultPort,
  frameworkOsLockedPrefsEnv,
  resolveTestLevel,
  atTestLevel,
  cargoProfileDir,
  selectComponentWasmProfile,
  semioBuildMode,
  semioShipEnv,
} from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();

import { walkRustSources } from "../🧹️capability-policy/🟦️.ts";



//#endregion 🔖️PluginIndexExportPathLint

//#region 🔖️HostHandleReachLint
/** 🕳️ `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`'s detector for a violation
 * `PluginCapabilityLintScript`'s ambient-mutability rule structurally cannot see: that rule bans
 * item-scope `thread_local!`/`static mut`/`Mutex`/`RwLock`/`RefCell`/`Cell`/`Atomic*` but deliberately
 * exempts bare `OnceLock`/`OnceCell`/`LazyLock` as write-once-by-type — every artifact's
 * `io_registry` uses `static ENTRIES: OnceLock<Vec<ComposerEntry>>`, and flagging those would drown
 * the signal. But `OnceLock<Vec<ComposerEntry>>` (a plugin caching its own immutable data) and
 * `OnceLock<BrepEngineHost>` (a plugin holding a handle to HOST-owned engine state for the process
 * lifetime) are identical in mutability and entirely different in violation. It is not ambient
 * *mutability*, it is ambient **reach** — `OnceLock` only makes the handle unforgeable after init; it
 * does nothing about a plugin having one at all. So this is a distinct check, not a widened
 * mutability rule (which would only manufacture false positives against the sanctioned registry
 * tables).
 *
 * Confirmed live: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️.rs:91-94`
 * (`static HOST: OnceLock<BrepEngineHost>`, constructed via `get_or_init`) and
 * `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️.rs:403,415`
 * (`host: BrepEngineHost` struct field, `BrepEngineHost::new(64 * 1024 * 1024)`).
 *
 * **Deliberately report-only, never wired into `verify`/`plugin lint`**, same posture as
 * `PluginIndexExportPathLintScript` above: several sessions are actively running against the gate
 * and this rule will fire on plugins they own. Fixing what it finds is cross-session work (`process`
 * belongs to another session, `cad` to this ticket, and the host-handle model reaches `💻️os/🖥️host`) and is
 * deliberately not attempted here. */
const HOST_ENGINE_HANDLE_TYPES: Readonly<Record<string, string>> = {
  // 🧠️ Host-owned brep engine wrapper — wraps `Mutex<EngineCache>` (byte-budgeted, host-managed compute cache) plus `Mutex<Brep>` kernel
  // session (🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🖥️host/🦀️.rs). It is the current handle-type surface — a plugin holding one
  // reaches directly for host-managed compute-cache/dispatch state instead of going through the WIT
  // `engine-derive`/`engine-read` guest<->host boundary.
  BrepEngineHost: "host-owned brep engine wrapper (byte-budgeted engine-result cache + kernel session) — a process-lifetime handle to host-managed compute state, not a plugin's own data",
  // 🧠️ The host-owned LRU byte-budgeted engine-result cache `BrepEngineHost` wraps
  // (🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️.rs, doc comment: "Host-owned LRU engine
  // result cache with a byte budget"). Holding one directly bypasses `BrepEngineHost`'s own wrapper but
  // reaches for the identical host-managed caching/dispatch authority.
  EngineCache: "host-owned LRU byte-budgeted engine-result cache underlying BrepEngineHost — same ambient reach as holding that wrapper directly, just unwrapped",
};

// 🕵️ Deliberately excludes plain opaque handle VALUES the framework explicitly documents as safe for a
// plugin to hold — `EngineHandle`/`EngineKey`/`GeometryHandle` are unforgeable content-addressed tokens
// ("plugins may store and read, never mint" per `EngineHandle`'s own doc comment), not connections to
// host-side state; and per-document domain models plugins legitimately own outright (`FlowHost`,
// `DagHost`, `GraphHost`, `MapHost`, `RasterHost`, `EditorHost`, `BoardHost` — all rebuilt fresh
// per-call/per-document from a fixture/snapshot, own no cache/arena/budget/connection to shared
// process-lifetime state) are excluded on the same "plain data, not a handle" ground. Also deliberately
// excludes the OS-crate host types (`ArtifactHost`, `SpaceHost`, `PluginHost`, `BackboneWorkerHost`,
// `WasmtimeNodeHost`) — those live inside `semio-framework-os`, already a blanket-forbidden dependency
// under `PluginCapabilityLintScript`'s `depRules`, so a plugin reaching them is already caught (coarser,
// but caught) there; adding them here would be redundant noise, not a new gap. And excludes bare
// `NativeHost`/`WasmHost`/`TestHost` — confirmed by inspection to be generic actor-model types plugins
// (`🖍️draw`, `🧩️puzzle`) *define locally themselves* inside their own `🔄️fsm`/`🌉️wasm` modules (a
// same-named but unrelated generic `Machine`-parameterized abstraction, not an OS host reference) — a
// bare name match on those would false-positive on legitimate plugin-owned code.
const HOST_ENGINE_HANDLE_TYPE_NAMES = Object.keys(HOST_ENGINE_HANDLE_TYPES);

const HOST_ENGINE_HANDLE_TYPE_ALTERNATION = HOST_ENGINE_HANDLE_TYPE_NAMES.join("|");

/** 🎯️ Rule 1 — a `static` of the handle type, any wrapper (`OnceLock`, `LazyLock`, `Mutex`, `RwLock`, or
 * none) per the ticket's explicit scope: unlike `PluginCapabilityLintScript`'s ambient-mutability rule,
 * `OnceLock`/`LazyLock` are NOT exempt here — the wrapper is irrelevant to ambient reach. */
const HOST_HANDLE_STATIC_PATTERN = new RegExp(`\\bstatic\\s+\\w+\\s*:\\s*(?:(?:OnceLock|LazyLock|Mutex|RwLock)\\s*<\\s*)?(${HOST_ENGINE_HANDLE_TYPE_ALTERNATION})\\s*>?`, "g");

/** 🎯️ Rule 2 — a struct field whose declared type names the handle type. The `(?!::)` guard excludes a
 * struct-LITERAL initializer line (`host: BrepEngineHost::new(...)`, rule 3's territory) from also
 * double-counting as a field-declaration hit — those two rules report the same physical violation from
 * two different source lines (the `pub struct` field decl vs. the `impl ... new()` initializer) in the
 * real `process3d` finding, and each should be counted once, not twice. */
const HOST_HANDLE_FIELD_PATTERN = new RegExp(`^\\s*(?:pub(?:\\([^)]*\\))?\\s+)?[a-z_][a-zA-Z0-9_]*\\s*:\\s*(?:(?:Option|Box|Arc|Mutex|RwLock|OnceLock|LazyLock)\\s*<\\s*)*(${HOST_ENGINE_HANDLE_TYPE_ALTERNATION})(?!::)\\b`, "gm");

/** 🎯️ Rule 3 — direct construction of the handle type. */
const HOST_HANDLE_CONSTRUCT_PATTERN = new RegExp(`\\b(${HOST_ENGINE_HANDLE_TYPE_ALTERNATION})::new\\s*\\(`, "g");

/** 🔢️ 1-based line number of `index` within `source`, for actionable breach messages. */
function lineNumberAtIndex(source: string, index: number): number {
  let line = 1;
  for (let i = 0; i < index; i++) if (source.charCodeAt(i) === 10) line++;
  return line;
}

type HostHandleBreach = { readonly relPath: string; readonly line: number; readonly rule: "static" | "field" | "construct"; readonly handleType: string };

/** 🕵️ Scans one Rust source file's text for all three rule sites, tagging each with its 1-based line. */
function scanHostHandleReach(relPath: string, source: string): HostHandleBreach[] {
  const breaches: HostHandleBreach[] = [];
  for (const match of source.matchAll(HOST_HANDLE_STATIC_PATTERN)) {
    breaches.push({ relPath, line: lineNumberAtIndex(source, match.index!), rule: "static", handleType: match[1]! });
  }
  for (const match of source.matchAll(HOST_HANDLE_FIELD_PATTERN)) {
    breaches.push({ relPath, line: lineNumberAtIndex(source, match.index!), rule: "field", handleType: match[1]! });
  }
  for (const match of source.matchAll(HOST_HANDLE_CONSTRUCT_PATTERN)) {
    breaches.push({ relPath, line: lineNumberAtIndex(source, match.index!), rule: "construct", handleType: match[1]! });
  }
  return breaches;
}

class HostHandleReachLintScript extends BundleScript {
  async run(): Promise<void> {
    const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
    let totalBreaches = 0;
    let pluginsWithBreaches = 0;
    for (const entry of readdirSync(pluginsRoot, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
      if (!entry.isDirectory()) continue;
      const pluginId = entry.name;
      const pluginDir = join(pluginsRoot, pluginId);
      const rustSources: string[] = [];
      walkRustSources(pluginDir, rustSources);
      const pluginBreaches: HostHandleBreach[] = [];
      for (const sourcePath of rustSources) {
        const source = await Bun.file(sourcePath).text();
        pluginBreaches.push(...scanHostHandleReach(relative(repoRoot, sourcePath), source));
      }
      if (pluginBreaches.length === 0) continue;
      pluginsWithBreaches++;
      totalBreaches += pluginBreaches.length;
      for (const breach of pluginBreaches) {
        const why = HOST_ENGINE_HANDLE_TYPES[breach.handleType];
        const site = breach.rule === "static" ? "static" : breach.rule === "field" ? "struct field" : "direct construction";
        console.warn(
          `[host-handle-reach-lint] WARN ${pluginId}: ${breach.relPath}:${breach.line}: ${site} of handle type ${breach.handleType} — ${why} (ambient REACH into host-owned state, not ambient mutability — a wrapping OnceLock/LazyLock only makes the handle unforgeable after init, it does not gate having one at all)`,
        );
      }
    }
    console.log(
      `host handle reach lint: ${totalBreaches} breach site(s) across ${pluginsWithBreaches} plugin(s) — REPORT ONLY, does not gate (26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE); fixing is cross-session work (process3d is another session's, cad is this ticket's, the trait model reaches 💻️os/🖥️host) and is deliberately not attempted here`,
    );
  }
}

export { HOST_ENGINE_HANDLE_TYPES, HOST_ENGINE_HANDLE_TYPE_ALTERNATION, HOST_ENGINE_HANDLE_TYPE_NAMES, HOST_HANDLE_CONSTRUCT_PATTERN, HOST_HANDLE_FIELD_PATTERN, HOST_HANDLE_STATIC_PATTERN, HostHandleBreach, HostHandleReachLintScript, lineNumberAtIndex, scanHostHandleReach };
