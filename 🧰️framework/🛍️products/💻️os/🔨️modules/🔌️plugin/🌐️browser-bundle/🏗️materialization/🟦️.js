#!/usr/bin/env bun
"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.SHARD_TURN_DRIVE_STEP_CEILING = exports.SHARD_TURN_GUEST_COST_MS = exports.SHARD_WORKER_DIAGNOSTICS_PARAM = exports.SHARD_RUNTIME_DIAGNOSTICS_KEY = exports.SEGMENTED_DOWNLOAD_CHUNK_BYTES = exports.SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS = exports.GUESTSLIM_FONT_RELATIVE = exports.SHARD_WORKER_FILE = exports.PLUGIN_HOST_SHIM_FILE = exports.rewritePreview2ShimImportSource = exports.PREVIEW2_VENDOR_RELATIVE = void 0;
exports.ensurePreview2ShimVendorAt = ensurePreview2ShimVendorAt;
exports.patchPreview2ShimCliLineBuffer = patchPreview2ShimCliLineBuffer;
exports.shardWorkerSource = shardWorkerSource;
exports.pluginComponentBridgeSource = pluginComponentBridgeSource;
exports.rewritePreview2ShimImports = rewritePreview2ShimImports;
exports.rewriteJcoAsyncResultLifting = rewriteJcoAsyncResultLifting;
exports.rewriteJcoComponentAssetUrls = rewriteJcoComponentAssetUrls;
exports.transpilePluginComponent = transpilePluginComponent;
exports.optimizePluginCoreModulesAsync = optimizePluginCoreModulesAsync;
exports.transpilePluginComponentAsync = transpilePluginComponentAsync;
exports.hostShimSource = hostShimSource;
/** @emoji 🌐 Shared jco transpile + plugin web glue (dev runner + extension store).
 *
 * MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (H2): `pluginWorkerSource`/`PLUGIN_WORKER_FILE`
 * (one Worker per plugin) are replaced by `shardWorkerSource`/`SHARD_WORKER_FILE` — ONE
 * package-agnostic `🟨️shard-worker.js`, served from `/🔌️plugin-modules/🧵️shard/`, multiplexed by
 * `actorId` across a bounded shard pool (see `🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`'s
 * `ShardClient`, the client-side transport this worker pairs with). V8 reserves a 4 GiB guard region
 * per wasm module per worker — one-worker-per-plugin capped the browser at ~20 plugins; this is the
 * change that lifts that ceiling. */
var node_child_process_1 = require("node:child_process");
var node_fs_1 = require("node:fs");
var node_path_1 = require("node:path");
var typescript_1 = require("typescript");
var ____ts_1 = require("../../../../../../\uD83D\uDD28\uFE0Fmodules/\uD83C\uDFAD\uFE0Factor/\uD83D\uDEAA\uFE0Flifetime/\uD83D\uDFE6\uFE0F.ts");
var ____ts_2 = require("../../../../../../\uD83D\uDD28\uFE0Fmodules/\uD83C\uDFAD\uFE0Factor/\uD83D\uDEAA\uFE0Flifetime/\uD83E\uDE79\uFE0Fpatch/\uD83D\uDFE6\uFE0F.ts");
var ___script_ts_1 = require("../../../../../\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCDA\uFE0Flibrary/\u26A1\uFE0Fcaching/\uD83D\uDE80\uFE0Fbootstrap/\uD83D\uDEE0\uFE0Ftools/\uD83D\uDD78\uFE0Fwasm/\uD83D\uDCDC\uFE0Fscript.ts");
var ____ts_3 = require("../../../../../\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCDA\uFE0Flibrary/\uD83C\uDFC3\uFE0Fprocess/\uD83D\uDFE6\uFE0F.ts");
var ____ts_4 = require("../\uD83D\uDD78\uFE0Fimports/\uD83D\uDFE6\uFE0F.ts");
var ____ts_5 = require("../\uD83D\uDD78\uFE0Fimports/\uD83D\uDFE6\uFE0F.ts");
Object.defineProperty(exports, "PREVIEW2_VENDOR_RELATIVE", { enumerable: true, get: function () { return ____ts_5.PREVIEW2_VENDOR_RELATIVE; } });
Object.defineProperty(exports, "rewritePreview2ShimImportSource", { enumerable: true, get: function () { return ____ts_5.rewritePreview2ShimImportSource; } });
exports.PLUGIN_HOST_SHIM_FILE = "🟨️.js";
exports.SHARD_WORKER_FILE = "🟨️shard-worker.js";
exports.GUESTSLIM_FONT_RELATIVE = "🪞️vendor/🔤️guestslim-typst-fonts.bin";
/** 🫀️ The generated worker's progress-heartbeat cadence, interpolated into
 * {@link shardWorkerSource}. Its OWNER is the schema-owned liveness policy
 * (`https://json.schemas.assets.semio-tech.com/framework/actor/shard-client/schema.json#/$defs/ShardClient` —
 * `🎭️actor/📮️shard-client/🧬️schema/🔣️.json` +
 * `🧫️fixtures/🔣️.json`, mirrored on the host side by `SHARD_LIVENESS_POLICY`); this declaration is
 * held equal to `policy.progressIntervalMs` by that module's own in-source suite, which reads this
 * literal straight out of this file, so editing one alone fails closed.
 *
 * Declared rather than read from the fixture at generation time on purpose: this module sits in
 * `⚙️vite.config.ts`'s import graph and is bundled by `Bun.build` for the bench harness, where
 * neither a relative `import.meta.url` file read nor an import of `📮️shard-client/🟦️.ts` (which
 * would drag the whole kernel/ui/resident graph into config loading) survives. */
exports.SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS = 1000;
/** 📤️ The generated worker's per-chunk byte cap for the segmented-download lane, interpolated into
 * {@link shardWorkerSource}. Its OWNER is the schema-owned chunk contract
 * (`https://json.schemas.assets.semio-tech.com/framework/actor/shard-client/segmented-download/schema.json` —
 * `🎭️actor/📮️shard-client/📤️segmented-download/🧬️schema/🔣️.json` + `🧫️fixtures/🔣️.json`, mirrored on the
 * host side by `SEGMENTED_DOWNLOAD_CONTRACT` and on the guest side by `🔌️plugin/🦀️.rs`'s
 * `ARTIFACT_OUTPUT_CHUNK_BYTES`); this declaration is held equal to `contract.chunkBytes` by the
 * shard-client suite, which reads this literal straight out of this file, so editing one alone fails
 * closed. Declared rather than imported for the same reason
 * {@link SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS} is. */
exports.SEGMENTED_DOWNLOAD_CHUNK_BYTES = 4096;
/** 🩺️ Mirrors `🎭️actor/🧵️shard-runtime/🟦️.ts`'s `SHARD_RUNTIME_DIAGNOSTICS_KEY`/
 * `SHARD_WORKER_DIAGNOSTICS_PARAM` and, through them, the guest's own `RUNTIME_DIAGNOSTICS_ENV`
 * (`🧰️framework/🔨️modules/⏱️trace/🦀️.rs`). Declared rather than imported for the same reason
 * {@link SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS} is: this module is bundled into `⚙️vite.config.ts`'s
 * import graph, and reaching `🧵️shard-runtime` from here would drag `ShardClient` and the whole
 * kernel/ui/resident graph into config loading. Held equal by the engine-contract suite. */
exports.SHARD_RUNTIME_DIAGNOSTICS_KEY = "SEMIO_RUNTIME_DIAGNOSTICS";
exports.SHARD_WORKER_DIAGNOSTICS_PARAM = "diagnostics";
/** 🚚️ What ONE whole reactor turn costs on the renderer this drive was measured against, in
 * milliseconds — the `worker.guest` mean of `🐍️react-hop-cost-probe.mjs` over a 10-step run of the
 * procedural 3d React door (12.9 ms over 1 154 crossings and 13.2 ms over 1 214, 2026-09-15). It is a
 * MEASUREMENT carried as a declaration, because the drive's bounds are DERIVED from it rather than
 * chosen: `SHARD_TURN_DRIVE_STEPS = floor(grant wall / this)`.
 *
 * 🐛️ It replaces a wall budget of `SHARD_TURN_SILENT_HOLD_MS = 8`, which was the reactor's own
 * executor slice (`⚛️reactor/🔄️turn/🦀️.rs`'s `run_until_deadline(64, 256 KiB, now + 8 ms)`) borrowed
 * as the worker's drive budget. A whole turn costs more than the reactor's slice of it, so an 8 ms
 * wall admitted `floor(8 / 13) = 0` further turns and the hold degenerated into "one extra poll" —
 * named but not taken by `📓️reactor-reconcile-spin-2026-09-14.md` §7 item 2. The reactor's slice is
 * still the reactor's: `REACTOR_TURN_EXECUTOR_HOLD_MS` is unchanged and unrelated to this number.
 *
 * Held equal to `⚛️reactor/🧫️fixtures/🚚️more-work-drive.json`'s `measuredGuestTurnCostMs` — and
 * through it to the reactor's own `MEASURED_GUEST_TURN_COST_MS` — by the more-work-drive suite, which
 * reads this literal straight out of this file. Declared rather than imported for the same reason
 * {@link SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS} is. */
exports.SHARD_TURN_GUEST_COST_MS = 13;
/** 🚚️ Hard cap on the drive's step ceiling however large a lane's granted wall gets, so a guest whose
 * turns cost microseconds cannot park the worker's whole message queue behind one drive. It bounds
 * the DERIVATION, it is not the derivation. */
exports.SHARD_TURN_DRIVE_STEP_CEILING = 512;
function ensurePreview2ShimVendorAt(preview2VendorDir, repoRoot) {
    var distDir = (0, node_path_1.join)(repoRoot, "node_modules/@bytecodealliance/preview2-shim/dist/browser");
    var libDir = (0, node_path_1.join)(repoRoot, "node_modules/@bytecodealliance/preview2-shim/lib/browser");
    var sourceDir = (0, node_fs_1.existsSync)(distDir) ? distDir : libDir;
    if (!(0, node_fs_1.existsSync)(sourceDir))
        throw new Error("missing @bytecodealliance/preview2-shim browser shims; run bun install");
    (0, node_fs_1.mkdirSync)(preview2VendorDir, { recursive: true });
    for (var _i = 0, _a = (0, node_fs_1.readdirSync)(sourceDir, { withFileTypes: true }); _i < _a.length; _i++) {
        var entry = _a[_i];
        if (!entry.isFile() || !entry.name.endsWith(".js"))
            continue;
        (0, node_fs_1.copyFileSync)((0, node_path_1.join)(sourceDir, entry.name), (0, node_path_1.join)(preview2VendorDir, entry.name));
    }
    patchPreview2ShimCliLineBuffer((0, node_path_1.join)(preview2VendorDir, "cli.js"));
}
/** 🗣️ Preview2's default `cli.js` console.errors every `write()` token. One logical guest line must be one host call, and `[DEBUG]` must not use `error`. */
function patchPreview2ShimCliLineBuffer(cliPath) {
    if (!(0, node_fs_1.existsSync)(cliPath))
        return;
    var source = (0, node_fs_1.readFileSync)(cliPath, "utf8");
    if (source.includes("semioGuestLogCarry"))
        return;
    var patched = source.replace(/const textDecoder = new TextDecoder\(\);[\s\S]*?const stdin = \{/, "const textDecoder = new TextDecoder();\nconst stdoutCarry = { bytes: new Uint8Array(0) };\nconst stderrCarry = { bytes: new Uint8Array(0) };\nfunction writeSemioGuestLogLine(channel, contents, carry) {\n    const merged = new Uint8Array(carry.bytes.length + contents.length);\n    merged.set(carry.bytes);\n    merged.set(contents, carry.bytes.length);\n    let start = 0;\n    for (let i = 0; i < merged.length; i++) {\n        if (merged[i] === 10) {\n            const text = textDecoder.decode(merged.subarray(start, i));\n            if (channel === \"stdout\") console.log(text);\n            else if (text.startsWith(\"[DEBUG]\")) console.debug(text);\n            else console.error(text);\n            start = i + 1;\n        }\n    }\n    carry.bytes = start === 0 ? merged : merged.subarray(start);\n}\nwriteSemioGuestLogLine.semioGuestLogCarry = true;\nconst stdoutStream = outputStreamCreate({\n    write(contents) {\n        writeSemioGuestLogLine(\"stdout\", contents, stdoutCarry);\n    },\n    blockingFlush() { },\n    [symbolDispose]() { },\n});\nconst stderrStream = outputStreamCreate({\n    write(contents) {\n        writeSemioGuestLogLine(\"stderr\", contents, stderrCarry);\n    },\n    blockingFlush() { },\n    [symbolDispose]() { },\n});\nexport const stdin = {");
    if (patched === source)
        throw new Error("preview2 cli.js line-buffer patch did not match: ".concat(cliPath));
    (0, node_fs_1.writeFileSync)(cliPath, patched);
}
/**
 * @emoji 🧵️ ONE package-agnostic worker bootstrap shared by every actor this tab's shard pool
 * activates — pairs with `ShardClient` (`🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`), which
 * owns exactly K of these workers (design-runtime.md §1 `ShardTable`: `min(hardwareConcurrency-1,
 * 4)` on web) instead of one per plugin. Keeps `Map<actorId, {api, instance}>` and dynamically
 * `import()`s each actor's own jco bridge module (`pluginComponentBridgeSource`'s output) on its
 * first `activate` — never at worker-bootstrap time, since one worker now hosts many unrelated
 * plugins' actors over its lifetime.
 *
 * Runs *one turn at a time per actor*: two `turn` requests for the SAME `actorId` overlapping is a
 * caller bug (the scheduler's job to prevent, not this worker's), enforced here defensively by
 * rejecting a second in-flight turn rather than corrupting interleaved guest state. Different actors
 * DO interleave — every request handler is `async`, so a long `await` inside one actor's turn lets
 * another actor's message be picked up and start in the meantime; nothing here blocks the worker's
 * event loop across actors.
 *
 * Heartbeats: a beat is taken at the START of every request (before running any guest code) and at
 * every STEP BOUNDARY — the instant a `turn`/`stepJob` hands control back. The step beat is what
 * proves a guest running a BUDGETED job is alive, because such a guest blocks the worker's event loop
 * (and the while-busy ticker with it) for the whole of each step and is otherwise indistinguishable
 * from a dead worker (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). See `ShardClient`'s watchdog, which
 * times a turn out at `2×wallMs` and, after three such windows, terminates and rebuilds this worker.
 *
 * Neither of those two beats costs a `postMessage` of its own any more: they RIDE the crossing that is
 * already happening, as the reply's `beat` field, and `ShardClient.handleMessage` folds them into the
 * same `recordHeartbeat` a dedicated `{kind:"heartbeat"}` message reaches. One message carries
 * liveness. A beat with NO crossing to ride still posts — `loadActor`'s three await boundaries and
 * the while-busy `progress` ticker — because there is nothing else to carry it. Every beat also
 * mirrors its `turnSeq` into the shared `Atomics.store` heartbeat slot when `attachHeartbeatSab`
 * provided one (COOP/COEP already served ⇒ `SharedArrayBuffer` available) — purely a faster read path;
 * the messages above are unconditional, so correctness never depends on the SAB path.
 *
 * 🚧 See `🧵️shard-client.ts`'s header doc for the one open gap this generated worker inherits: `turn`
 * events/results here are the interim JSON `ShardEventEnvelope[]`/plain-object shape, not the real
 * hand-rolled `Envelope`/`TurnResult` pack encoding (no TS mirror of that codec exists yet — tracked
 * against A1's `🤖️generated/🎭️actor/🟦️.ts`). The WIT-level `stage-command-page`/`stage-cold-pair-page`
 * then `poll(events, budget)` calls this worker makes
 * against the guest's own jco bindings is unaffected either way (jco marshals those to/from the wasm
 * component boundary itself); only the Kernel↔Shard wire between this worker and `ShardClient` is
 * interim JSON rather than pack bytes.
 *
 * 📨️ terra-web-shardframe: also handles the NEW `"frame"` message kind — `ShardClient.grant`/
 * `ShardClient.envelope`'s `ShardFrame::Grant`/`ShardFrame::Envelope` wire, additive alongside `"turn"`
 * above (unchanged). `interpretFrame`/`grantedBudgets`/`orderEnvelopesByLane` below are a hand-
 * transcribed mirror of `🧵️shard-client.ts`'s `interpretShardFrame`/`GrantedBudgetTracker`/
 * `orderEnvelopesByLane` — this string can't `import` that module, so the logic is duplicated by
 * necessity; that file's own in-source tests are what's actually exercised for this behavior, this
 * copy is kept byte-for-byte equivalent by hand. A `Grant` remembers its budget for `frame.actor`;
 * a later budget-less `Envelope` for that actor runs under it (falling back to
 * `MAINTENANCE_LANE_DEFAULT_BUDGET` — `semio_framework_actor::lane_defaults::budget_for(Lane::
 * Maintenance)` — for an actor never granted one) instead of any caller-cached constant. An unknown
 * frame `kind` this worker has never heard of is acknowledged as `{ ignored: true }` rather than
 * thrown, so a future Rust-side `ShardFrame` variant can reach a live worker before its TS mirror
 * lands without wedging it. An `Envelope` whose `payload.kind` is `"effect-complete"`/`"effect-error"`
 * is routed to `deliverEffectResult` instead of a normal turn (🧪️ terra-web-bridges, see that
 * function's own doc) — it settles a `🟨️.js` Promise, it is never itself a turn to run.
 *
 * 🧪️ terra-web-bridges (async-worlds): every WIT function the target world exports/imports is now
 * `async func`, and jco's JS glue for that ALWAYS calls `new WebAssembly.Suspending(...)`/
 * `WebAssembly.promising(...)` regardless of `--async-mode` (📓️terra-jco-spike-report.md's VERDICT:
 * GO-jspi — confirmed no flag produces JSPI-free output). Without JSPI the failure is hard and early:
 * a `TypeError` at MODULE TOP-LEVEL, before any call, with no graceful degradation. The guard right
 * below turns that opaque failure into an explicit, actionable one — a diagnostic, NOT a fallback;
 * there is no code path here that runs a plugin without JSPI.
 */
function shardWorkerSource() {
    return "/** @generated semio shard worker (H2 \u2014 bounded pool, actorId-multiplexed) */\n// \uD83E\uDE7A\uFE0F SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION (1-B): raise the captured-frame\n// cap BEFORE anything else runs so a deep guest recursion's real stack survives `error.stack`\n// instead of being truncated to V8's 10-frame default \u2014 this worker's stack is otherwise destroyed\n// before `ShardClient` ever sees it (the main thread only ever saw one frame: `at worker.onmessage`).\nError.stackTraceLimit = 200;\n\n// \uD83E\uDDEA\uFE0F terra-web-bridges: explicit JSPI capability gate \u2014 see this file's own header doc (\"what must\n// change\" #1 in \uD83D\uDCD3\uFE0Fterra-jco-spike-report.md). Every plugin component this worker will ever `import()`\n// is fully async-lifted, and jco's glue for that unconditionally needs `WebAssembly.Suspending`/\n// `WebAssembly.promising`; without them the FIRST `import()` throws `TypeError: WebAssembly.Suspending\n// is not a constructor` at module top-level, before any plugin call \u2014 an opaque failure the spike\n// reproduced verbatim under plain Node 24. Posting a `\"trap\"` BEFORE throwing gives `ShardClient`'s\n// `onActorTrap` its best chance at a readable message even where a cross-context Worker `onerror`\n// gets redacted to `\"undefined undefined undefined\"` (also reproduced by the spike) \u2014 `actorId: \"*\"`\n// is a worker-wide sentinel, not a real actor, since no actor has activated yet at this point.\nif (typeof WebAssembly === \"undefined\" || typeof WebAssembly.Suspending !== \"function\" || typeof WebAssembly.promising !== \"function\") {\n  const message = \"semio shard worker: this browser/engine lacks JavaScript Promise Integration (JSPI) \u2014 WebAssembly.Suspending/WebAssembly.promising are required to run semio's async-lifted plugin components and there is no fallback. Chrome/Edge/Chromium-based browsers ship JSPI on by default; Firefox needs the javascript.options.wasm_js_promise_integration flag in about:config; Node.js needs --experimental-wasm-jspi.\";\n  self.postMessage({ kind: \"trap\", actorId: \"*\", activationGeneration: null, message });\n  throw new Error(message);\n}\n\n// \uD83E\uDE7A\uFE0F Nothing inside a Worker is visible from the page unless the worker says it: an exception that\n// escapes a message handler, or a promise nobody awaits, reaches the parent as an `ErrorEvent` whose\n// `message` the platform routinely redacts to nothing (exactly what the 2026-09-05 boot logged, four\n// times, with no other evidence). These three variables are the breadcrumb trail \u2014 updated at every\n// boundary so a fault report names the phase, the actor and the module URL it died on \u2014 and the two\n// global listeners below are what turn an otherwise-anonymous `Event` into a readable payload on the\n// host side. Registered BEFORE the \"message\" listener so a permissive test stub that keeps only the\n// last handler still keeps the dispatcher.\nlet faultPhase = \"bootstrap\";\nlet faultActorId = null;\nlet faultModuleUrl = null;\n\nfunction describeFault(value) {\n  if (value instanceof Error) return { message: value.message, stack: value.stack };\n  if (value && typeof value === \"object\" && (typeof value.stack === \"string\" || typeof value.message === \"string\")) return { message: String(value), stack: typeof value.stack === \"string\" ? value.stack : undefined };\n  if (value && typeof value === \"object\") { try { return { message: JSON.stringify(value) }; } catch { return { message: String(value) }; } }\n  return { message: String(value) };\n}\n\nfunction reportWorkerFault(source, value, event) {\n  const reason = value !== undefined && value !== null ? value : event && typeof event.message === \"string\" && event.message.length > 0 ? event.message : value;\n  const described = describeFault(reason);\n  self.postMessage({\n    kind: \"worker-fault\",\n    source,\n    phase: faultPhase,\n    actorId: faultActorId,\n    moduleUrl: faultModuleUrl,\n    message: described.message,\n    stack: described.stack,\n    filename: event && typeof event.filename === \"string\" ? event.filename : undefined,\n    lineno: event && typeof event.lineno === \"number\" ? event.lineno : undefined,\n  });\n}\n\nself.addEventListener(\"error\", (event) => reportWorkerFault(\"error\", event && event.error, event));\nself.addEventListener(\"unhandledrejection\", (event) => reportWorkerFault(\"unhandledrejection\", event && event.reason, null));\n\nconst actors = new Map(); // actorId -> { api, moduleUrl }\nconst activatingActors = new Set();\nlet lastActivationGeneration = 0n;\nconst inFlightTurnActors = new Set();\nlet turnSeq = 0;\nlet heartbeatSabView = null;\nlet heartbeatShardIndex = -1;\n// \uD83D\uDCE4\uFE0F Per-chunk byte cap of the segmented-download lane, interpolated from the ONE schema-owned chunk\n// contract (`\uD83C\uDFAD\uFE0Factor/\uD83D\uDCEE\uFE0Fshard-client/\uD83D\uDCE4\uFE0Fsegmented-download/\uD83E\uDDEB\uFE0Ffixtures/\uD83D\uDD23\uFE0F.json`'s `contract`, mirrored by\n// `SEGMENTED_DOWNLOAD_CONTRACT`) that the guest producer slices by and the host drain admits against \u2014\n// never a literal of this worker's own.\nconst SEGMENTED_DOWNLOAD_CHUNK_BYTES = ".concat(exports.SEGMENTED_DOWNLOAD_CHUNK_BYTES, ";\n// \uD83E\uDEC0\uFE0F Progress-heartbeat cadence, interpolated from the ONE schema-owned liveness policy\n// (`\uD83C\uDFAD\uFE0Factor/\uD83D\uDCEE\uFE0Fshard-client/\uD83E\uDDEB\uFE0Ffixtures/\uD83D\uDD23\uFE0F.json`'s `policy`, mirrored by `SHARD_LIVENESS_POLICY`) the\n// host watchdog reads \u2014 never a literal of this worker's own.\nconst PROGRESS_HEARTBEAT_INTERVAL_MS = ").concat(exports.SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS, ";\n// \uD83D\uDE9A\uFE0F The worker-owned MoreWork drive, interpolated from `SHARD_TURN_GUEST_COST_MS`/\n// `SHARD_TURN_DRIVE_STEP_CEILING` \u2014 the MEASURED cost of one whole guest turn, which is what the\n// drive's step ceiling is derived FROM. Twin of `\uD83C\uDFAD\uFE0Factor/\uD83D\uDDBC\uFE0Fwire-turn/\uD83D\uDFE6\uFE0F.ts`'s\n// `driveShardTurnMoreWorkV1`/`shardTurnDriveStepsV1`, which own the law.\nconst GUEST_TURN_COST_MS = ").concat(exports.SHARD_TURN_GUEST_COST_MS, ";\nconst DRIVE_STEP_CEILING = ").concat(exports.SHARD_TURN_DRIVE_STEP_CEILING, ";\nlet progressHandle = null;\nlet inFlightRequests = 0;\n// \uD83D\uDCEC\uFE0F Every message the host posts bumps this, and a drive that started at one value crosses back the\n// moment it sees another \u2014 that is the whole \"a host-owned input interrupts the drive\" contract. It\n// counts messages rather than naming kinds on purpose: a kind this worker has never heard of is still\n// the host speaking, and the drive must never be the reason it waits.\nlet hostInputSeq = 0;\n\n// \uD83D\uDCE8\uFE0F terra-web-shardframe: ShardFrame::Grant/Envelope support \u2014 see this file's own header doc.\nconst MAINTENANCE_LANE_DEFAULT_BUDGET = { fuel: 80000000, wallMs: 200, memoryBytes: 256 * 1024 * 1024, uiNodes: 4000, mailboxLen: 1024, maxEffects: 512, maxPatchBytes: 2097152 };\nconst SHARD_FRAME_LANE_ORDER = [\"Interactive\", \"UserVisible\", \"Background\", \"Maintenance\"];\nconst grantedBudgets = new Map(); // actorId -> last ShardFrame::Grant budget, mirrors ShardLoop::granted_budgets\n\nfunction orderEnvelopesByLane(envelopes) {\n  return envelopes\n    .map((envelope, index) => ({ envelope, index }))\n    .sort((left, right) => {\n      const rank = SHARD_FRAME_LANE_ORDER.indexOf(left.envelope.lane) - SHARD_FRAME_LANE_ORDER.indexOf(right.envelope.lane);\n      return rank !== 0 ? rank : left.index - right.index;\n    })\n    .map((entry) => entry.envelope);\n}\n\n// \uD83E\uDDE0\uFE0F Mirrors \uD83E\uDDF5\uFE0Fshard-client.ts's interpretShardFrame \u2014 see that function's own doc for the semantics.\nfunction interpretFrame(frame, actorId) {\n  switch (frame.kind) {\n    case \"Register\":\n      return { action: \"register\" };\n    case \"Unregister\":\n      return { action: \"unregister\" };\n    case \"Grant\":\n      grantedBudgets.set(frame.actor, frame.budget);\n      return { action: \"runEnvelopes\", budget: frame.budget, envelopes: orderEnvelopesByLane(frame.envelopes) };\n    case \"Envelope\":\n      return { action: \"runEnvelopes\", budget: grantedBudgets.has(actorId) ? grantedBudgets.get(actorId) : MAINTENANCE_LANE_DEFAULT_BUDGET, envelopes: [frame.envelope] };\n    default:\n      return { action: \"unknown\" };\n  }\n}\n\n// \uD83E\uDEC0\uFE0F ONE beat door for every liveness signal this worker emits. It advances the sequence and mirrors\n// it into the shared `Atomics` slot, and it RETURNS the beat instead of posting it \u2014 because a beat\n// taken at a boundary that is about to cross anyway rides the crossing (`reply`/`replyError` carry\n// `beat`), and a message that carries liveness is one message, not two.\n//\n// \uD83D\uDC1B\uFE0F It used to post unconditionally, so every turn crossing cost THREE main-thread messages: a\n// start-of-request `heartbeat`, a `turn-step` `heartbeat`, and the `result` itself \u2014 on a main\n// thread the hop measurement shows is the binding constraint\n// (`\uD83D\uDCD3\uFE0Freactor-reconcile-spin-2026-09-14.md` \u00A77 item 3, which named this and left it). Neither posted\n// beat told the host anything the reply does not: `ShardClient.noteLiveness` treats EVERY inbound\n// message as proof of life, and the request's own start instant already counts as proven-alive in\n// `evaluateShardLiveness`. What is NOT foldable is a beat with no crossing to ride \u2014 `loadActor`'s\n// await boundaries and the while-busy ticker \u2014 so those still call `postBeat`.\nfunction beat(phase) {\n  turnSeq += 1;\n  if (heartbeatSabView) Atomics.store(heartbeatSabView, heartbeatShardIndex, turnSeq);\n  return { turnSeq, phase: phase === undefined ? null : phase };\n}\n\nfunction postBeat(phase) {\n  self.postMessage({ kind: \"heartbeat\", ...beat(phase) });\n}\n\n// \uD83E\uDEC0\uFE0F The whole busy-versus-dead discriminator: while ANY request is outstanding, an interval beats\n// every PROGRESS_HEARTBEAT_INTERVAL_MS. A worker legitimately parked on one multi-second `await`\n// (fetching, compiling and instantiating a multi-MB wasm component) still runs its event loop, so it\n// keeps beating and the host's watchdog keeps its miss count at zero; a worker wedged inside\n// synchronous guest code cannot run this callback at all, so it still dies after the same\n// `missedLimit` windows. Guarded on `setInterval` existing so a bare VM/test context is unaffected.\nfunction beginRequest() {\n  inFlightRequests += 1;\n  if (progressHandle !== null || typeof setInterval !== \"function\") return;\n  progressHandle = setInterval(() => postBeat(\"progress\"), PROGRESS_HEARTBEAT_INTERVAL_MS);\n}\n\nfunction endRequest() {\n  inFlightRequests -= 1;\n  if (inFlightRequests > 0 || progressHandle === null) return;\n  clearInterval(progressHandle);\n  progressHandle = null;\n}\n\n// \u23F1\uFE0F The ONE clock this worker shares with the page. A Worker's `performance.now()` counts from its\n// OWN `timeOrigin`, so only `timeOrigin + now()` \u2014 the same Unix-epoch millisecond on both sides at\n// sub-millisecond resolution \u2014 lets the CROSSINGS be measured (post \u2192 receive, reply \u2192 receive)\n// rather than inferred by subtracting the parts from the whole. Twin of\n// `\uD83D\uDD28\uFE0Fmodules/\u23F1\uFE0Ftrace/\uD83D\uDFE6\uFE0F.ts`'s `hopTraceEpochNowMs`, which is what reads these back on the page.\nconst hopEpochNow = () => (typeof performance === \"object\" && typeof performance.now === \"function\" ? (typeof performance.timeOrigin === \"number\" ? performance.timeOrigin : 0) + performance.now() : Date.now());\n\n// \uD83C\uDFF7\uFE0F The ONE spelling of a turn status. jco lifts `more-work` kebab-cased, the host's own fixtures\n// spell it `moreWork`, and `\uD83D\uDDBC\uFE0Fwire-turn/\uD83D\uDFE6\uFE0F.ts`'s `wireTurnStatusTag` reconciles both \u2014 this is its\n// verbatim twin, because a drive that misreads the status would hand back a turn nobody asked for.\nfunction shardTurnStatusTag(result) {\n  const tag = result && typeof result === \"object\" && result.status && typeof result.status === \"object\" ? result.status.tag : undefined;\n  return typeof tag === \"string\" ? tag.replace(/([a-z])([A-Z])/g, \"$1-$2\").toLowerCase() : \"\";\n}\n\n// \uD83E\uDD2B\uFE0F Whether ONE reactor turn result carried nothing the host could act on: no ui patch, no effect,\n// no presence, no wake, neither receipt, and both ingress lanes idle. Every field of `turn-result`\n// (`\uD83D\uDD0C\uFE0Fplugin/\uD83E\uDDEC\uFE0Fschema/\uD83D\uDCDC\uFE0F.wit`) is named here on purpose \u2014 a carrier this predicate forgot would be\n// DROPPED by the drive below, so the list is exhaustive by construction and asserted as such by\n// `\uD83C\uDFAD\uFE0Factor/\uD83D\uDDBC\uFE0Fwire-turn/\uD83D\uDFE6\uFE0F.ts`'s law. `fuelUsed` is the one field with no host reader.\nfunction shardTurnCarriesNothing(result) {\n  if (!result || typeof result !== \"object\") return false;\n  if (!Array.isArray(result.uiPatches) || result.uiPatches.length !== 0) return false;\n  if (!Array.isArray(result.effects) || result.effects.length !== 0) return false;\n  if (result.presence !== undefined && (!Array.isArray(result.presence) || result.presence.length !== 0)) return false;\n  if (result.nextWake !== null && result.nextWake !== undefined) return false;\n  if (result.lifecycleReceipt !== undefined && result.lifecycleReceipt !== null) return false;\n  if (result.uiPatchReceipt !== undefined && result.uiPatchReceipt !== null) return false;\n  if (!result.commandIngress || result.commandIngress.tag !== \"idle\") return false;\n  if (!result.coldPairIngress || result.coldPairIngress.tag !== \"idle\") return false;\n  return true;\n}\n\n// \uD83D\uDE9A\uFE0F The drive's two bounds, DERIVED from the wall the host granted for this crossing and what one\n// guest turn measures \u2014 never a constant of this worker's own. Verbatim twins of\n// `\uD83C\uDFAD\uFE0Factor/\uD83D\uDDBC\uFE0Fwire-turn/\uD83D\uDFE6\uFE0F.ts`'s `shardTurnDriveStepsV1`/`shardTurnDriveBudgetMsV1`.\nfunction shardTurnDriveSteps(grantWallMs, guestTurnCostMs) {\n  if (!Number.isFinite(grantWallMs) || !Number.isFinite(guestTurnCostMs) || guestTurnCostMs <= 0) return 1;\n  return Math.max(1, Math.min(DRIVE_STEP_CEILING, Math.floor(grantWallMs / guestTurnCostMs)));\n}\n\nfunction shardTurnDriveBudgetMs(grantWallMs, guestTurnCostMs) {\n  if (!Number.isFinite(grantWallMs) || grantWallMs <= 0) return Math.max(0, guestTurnCostMs);\n  return Math.max(grantWallMs, guestTurnCostMs);\n}\n\n// \uD83D\uDCEC\uFE0F ONE macrotask, so a message the host already posted is DELIVERED before the drive takes its next\n// step. Without it `hostInputSeq` could never move mid-drive: an `await` settles on the microtask\n// queue, which never drains the message queue, so the drive would be uninterruptible by construction.\n// A `MessageChannel` task and not `setTimeout(0)`, for the reason the host's own\n// `hostContinuations` gives: a timer chain is throttled to one tick per second in a hidden tab (once\n// per minute under intensive throttling) and a channel message is a macrotask visibility never\n// throttles. Waiters are a QUEUE because two actors may be driving this worker at the same time.\nconst driveYieldChannel = typeof MessageChannel === \"function\" ? new MessageChannel() : null;\nconst driveYieldWaiters = [];\nif (driveYieldChannel) driveYieldChannel.port1.onmessage = () => { const resolve = driveYieldWaiters.shift(); if (resolve) resolve(); };\n\nfunction driveYield() {\n  if (!driveYieldChannel) return Promise.resolve();\n  return new Promise((resolve) => { driveYieldWaiters.push(resolve); driveYieldChannel.port2.postMessage(0); });\n}\n\n// \uD83E\uDEC0\uFE0F EVERY reply carries a beat \u2014 an answered request is a proven-alive worker, and the caller's own\n// `ShardClient` folds it into exactly the state a dedicated `heartbeat` message would have reached.\n// A boundary that has a name of its own (`turn-step`) passes it; everything else beats unphased.\nfunction reply(requestId, value, timings, carriedBeat) {\n  const carried = carriedBeat === undefined ? beat() : carriedBeat;\n  if (!timings) { self.postMessage({ kind: \"result\", requestId, ok: true, value, beat: carried }); return; }\n  timings.repliedAtEpochMs = hopEpochNow();\n  self.postMessage({ kind: \"result\", requestId, ok: true, value, timings, beat: carried });\n  // \u23F1\uFE0F Written AFTER the post so the host can separate the two halves of the reply: the structured\n  // CLONE this call performs synchronously (`clonedAtEpochMs` \u2212 `repliedAtEpochMs`, mutated on the\n  // object the clone already took a copy of, so the host reads it from the NEXT reply's carry) from\n  // the main thread's own pickup latency. A worker cannot amend a message it already posted, so the\n  // clone cost of turn N is carried on turn N+1 \u2014 one turn of lag, exact either way.\n  lastReplyCloneMs = hopEpochNow() - timings.repliedAtEpochMs;\n}\nlet lastReplyCloneMs = 0;\n\n// \uD83E\uDE7A\uFE0F `frames` is the request's own bulk payload (the `turn` message's `events` array \u2014 the largest,\n// most recursion-prone field a request carries) \u2014 sized WITHOUT ever JSON.stringify-ing it first\n// unless it isn't already a binary buffer, so a huge/cyclic payload can't itself blow the stack while\n// we're trying to report a stack overflow. A `turn`'s `events` is an ARRAY of wire buffers, never a\n// lone buffer, so summing member byte lengths is the only path that reports wire bytes: the stringify\n// fallback renders each byte as `\"index\":value` and inflated a 6 MB payload to 63 MB, which is a\n// memory diagnosis this ticket had to walk back (26/09/09/PROCEDURAL-3D-END-TO-END).\nfunction replyError(requestId, error, frames, retryableLifecycle) {\n  const payload = error && typeof error === \"object\" && \"payload\" in error ? error.payload : undefined;\n  const detail = payload !== undefined ? ` payload=${(() => { try { return JSON.stringify(payload); } catch { return String(payload); } })()}` : \"\";\n  let stack;\n  try { stack = error && error.stack ? String(error.stack) : undefined; } catch { stack = undefined; }\n  let type;\n  try { type = (error && error.constructor && error.constructor.name) || typeof error; } catch { type = typeof error; }\n  let framesBytes;\n  try {\n    framesBytes = frames instanceof Uint8Array || frames instanceof ArrayBuffer ? frames.byteLength\n      : Array.isArray(frames) && frames.every((frame) => frame instanceof Uint8Array || frame instanceof ArrayBuffer) ? frames.reduce((total, frame) => total + frame.byteLength, 0)\n      : frames !== undefined ? JSON.stringify(frames).length : undefined;\n  } catch { framesBytes = undefined; }\n  // \uD83E\uDE7A\uFE0F A guest plugin rejects with a LIFTED FAULT RECORD, not an `Error` \u2014 a plain object whose\n  // `String()` is the useless `[object Object]` that used to be all the host, the console, and the\n  // on-screen error surface ever saw for the single most common failure there is. Serialize the\n  // record itself; `Error` still reports its own message, and an unserializable value still falls\n  // back to `String`.\n  // \uD83E\uDE7A\uFE0F `instanceof Error` is false for an Error thrown in ANOTHER realm (a `vm` context, or any\n  // cross-context host callback this worker invokes), and `JSON.stringify` of an Error is the useless\n  // `\"{}\"` \u2014 which is what the host used to be told for exactly the faults it most needs to read.\n  // Duck-typing on `stack`/`message` catches the cross-realm case without ever mis-serializing a\n  // genuine lifted fault record, which has neither.\n  let reason;\n  if (error instanceof Error) reason = error.message;\n  else if (error && typeof error === \"object\" && (typeof error.stack === \"string\" || typeof error.message === \"string\")) reason = String(error);\n  else if (error && typeof error === \"object\") { try { reason = JSON.stringify(error); } catch { reason = String(error); } }\n  else reason = String(error);\n  self.postMessage({ kind: \"result\", requestId, ok: false, error: reason + detail, stack, type, framesBytes, retryableLifecycle: retryableLifecycle === true, beat: beat(\"fault\") });\n}\n\n// \uD83E\uDE7A\uFE0F Hands every component this worker hosts the guest-side diagnostics switch through\n// `wasi:cli/environment` \u2014 the ONE schema-declared door a `wasm32-wasip2` component's own\n// `std::env::var` reads (`semio_framework_trace::RUNTIME_DIAGNOSTICS_ENV`). Before this, every\n// `[DEBUG]` line the guest's Rust hot path prints was unreachable from a browser session: the page's\n// `localStorage` switch only ever armed TypeScript-side traces, and `runtime_diagnostics_from_environment`\n// resolved against an environment nobody populated (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,\n// `\uD83D\uDCD3\uFE0Faudit-guest-tick-cost-2026-09-12.md` \u00A70/\u00A74 rank 1).\n//\n// \uD83D\uDEAA\uFE0F The page resolved the switch and stamped it on this worker's OWN url (`shardWorkerUrl`), since a\n// Worker realm owns no `localStorage`. The shim module is the SAME instance the transpiled component\n// imports \u2014 both resolve `\uD83E\uDE9E\uFE0Fvendor/\uD83E\uDD1D\uFE0Fbytecode-alliance/\uD83E\uDE9F\uFE0Fpreview2-shim/cli.js` against the shared\n// `\uD83D\uDD0C\uFE0Fplugin-modules/` root \u2014 so seeding it here seeds the guest. Disarmed is the default and costs\n// one url read; a shim that cannot be loaded degrades to silent traces, never to a failed boot.\nlet guestRuntimeDiagnostics = null;\nasync function armGuestRuntimeDiagnostics() {\n  if (guestRuntimeDiagnostics !== null) return guestRuntimeDiagnostics;\n  guestRuntimeDiagnostics = (async () => {\n    if (new URL(self.location.href).searchParams.get(\"").concat(exports.SHARD_WORKER_DIAGNOSTICS_PARAM, "\") !== \"1\") return false;\n    try {\n      const { _setEnv } = await import(/* @vite-ignore */ \"../\uD83E\uDE9E\uFE0Fvendor/\uD83E\uDD1D\uFE0Fbytecode-alliance/\uD83E\uDE9F\uFE0Fpreview2-shim/cli.js\");\n      _setEnv({ \"").concat(exports.SHARD_RUNTIME_DIAGNOSTICS_KEY, "\": \"1\" });\n      return true;\n    } catch {\n      return false;\n    }\n  })();\n  return guestRuntimeDiagnostics;\n}\n\nasync function loadActor(actorId, activationGeneration, moduleUrl) {\n  if (typeof activationGeneration !== \"bigint\" || activationGeneration <= lastActivationGeneration || activationGeneration > 0xffffffffffffffffn) throw new Error(\"actor-close.invalid-activation-generation\");\n  if (actors.has(actorId) || activatingActors.has(actorId)) throw new Error(\"actor-close.activation-already-owned\");\n  lastActivationGeneration = activationGeneration;\n  activatingActors.add(actorId);\n  try {\n    // \uD83E\uDEC0\uFE0F The three await boundaries of the longest turn this worker ever runs: the dynamic import\n    // (network fetch + WebAssembly.compile + instantiate, all opaque inside one await) and the guest's\n    // own `createActorApi`. Each boundary beats explicitly so a console trace names where a slow boot\n    // actually sat; the ticker above is what carries liveness THROUGH each of them.\n    faultModuleUrl = moduleUrl;\n    faultPhase = \"load-bridge\";\n    postBeat(\"module-fetch\");\n    await armGuestRuntimeDiagnostics();\n    const bridge = await import(/* @vite-ignore */ moduleUrl);\n    faultPhase = \"instantiate\";\n    postBeat(\"module-ready\");\n    const api = await bridge.createActorApi(actorId, activationGeneration);\n    faultPhase = \"actor-ready\";\n    postBeat(\"actor-ready\");\n    const entry = { api, moduleUrl, activationGeneration, pendingAssets: [] };\n    actors.set(actorId, entry);\n    return entry;\n  } finally {\n    activatingActors.delete(actorId);\n  }\n}\n\n// \uD83E\uDDEA\uFE0F terra-web-bridges: settles a `\uD83D\uDFE8\uFE0F.js` `effectRequest` Promise from an `effect-complete`/\n// `effect-error` envelope \u2014 see `hostShimSource`'s own doc for the wire shape this expects\n// (`envelope.payload.payload.requestId`, `.value` on complete / `.message` on error). A missing actor\n// (already disposed, or the envelope arrived before `activate`) is silently dropped rather than\n// thrown \u2014 this envelope is itself an ANSWER, so nobody is awaiting a reply to it. That is exactly\n// what makes it different from `cancelJob`, which IS a request and is answered in the switch below.\nfunction deliverEffectResult(actorId, activationGeneration, envelope) {\n  const actor = actors.get(actorId);\n  if (!actor || actor.activationGeneration !== activationGeneration || envelope.to !== actorId || envelope.from?.kind !== \"kernel\") return;\n  const { kind, payload } = envelope.payload;\n  if (kind === \"effect-complete\") actor.api.resolveEffect(payload.requestId, payload.value);\n  else if (kind === \"effect-error\") actor.api.rejectEffect(payload.requestId, payload.message);\n}\n\n// \uD83E\uDEB6\uFE0F GUESTSLIM (design-runtime.md \u00A73): world `actor` exports NO `activate` function \u2014 activation is\n// pure bookkeeping (load the module, cache the named asset packs the main thread fetched) until the\n// KERNEL's own first `turn` for this actor carries a real `instance-open` event (it alone knows\n// `app-id`/`config`/`quotas`). This worker's only job is splicing the cached asset bytes into that\n// event's `assets` field right before the first `poll` \u2014 they must be resident before the guest's\n// first `surface-visible`, not fetched lazily on read.\nfunction spliceInstanceOpenAssets(entry, events) {\n  if (entry.pendingAssets.length === 0) return events;\n  const pending = entry.pendingAssets;\n  entry.pendingAssets = [];\n  return events.map((event) => {\n    if (event.kind !== \"instance-open\") return event;\n    return { kind: event.kind, payload: { ...event.payload, assets: [...(event.payload.assets ?? []), ...pending] } };\n  });\n}\n\n// \u23F1\uFE0F The browser twin of the host's own `retryable_lifecycle_turn` (`\uD83D\uDD0C\uFE0Fplugin/\uD83D\uDDA5\uFE0Fhost/\uD83E\uDD80\uFE0F.rs`, driven\n// by `\uD83D\uDDA5\uFE0Fhost/\uD83D\uDD01\uFE0Flifecycle/\uD83E\uDDEB\uFE0Ffixtures/\uD83D\uDD23\uFE0F.json`): a RETRYABLE `plugin.reactor-turn-deadline` on a turn\n// that carries at most one lifecycle event is a YIELD, not a death \u2014 the guest retained its receipt,\n// so the same events replay on the next tick and the open continues. Native `ShardLoop::pump` has\n// always re-granted these; without this the browser reported the identical verdict as a worker fault\n// and `onActorTrap` killed the actor on its very first step (ticket 26/09/09, boot #6).\nconst LIFECYCLE_TURN_EVENT_KINDS = [\"instance-open\", \"instance-close\", \"instance-lifecycle-ack\"];\nconst REACTOR_TURN_DEADLINE_CODE = \"plugin.reactor-turn-deadline\";\n\nfunction guestFaultRecord(error) {\n  for (const value of [error, error?.payload, error?.payload?.val, error?.val]) {\n    if (!value) continue;\n    if (typeof value.code === \"string\") return value;\n    if (value instanceof Uint8Array) {\n      try { return JSON.parse(new TextDecoder().decode(value)); } catch { return null; }\n    }\n  }\n  return null;\n}\n\nfunction retryableLifecycleTurn(error, events, commandPage) {\n  const fault = guestFaultRecord(error);\n  if (!fault || fault.code !== REACTOR_TURN_DEADLINE_CODE || fault.retryable !== true) return false;\n  return (commandPage === undefined || commandPage === null) && Array.isArray(events) && events.length <= 1 && events.every((entry) => entry && LIFECYCLE_TURN_EVENT_KINDS.includes(entry.kind));\n}\n\nself.addEventListener(\"message\", async (event) => {\n  // \u23F1\uFE0F FIRST statement of the handler: everything after it is already this worker's own cost, and\n  // `postedAtEpochMs` \u2212 this instant is the post crossing nobody could see from outside (the CDP\n  // `Performance` domain answers nothing on a worker target \u2014\n  // `\uD83D\uDCD3\uFE0Freact-hop-latency-2026-09-14.md` \u00A71). Twin vocabulary: `semio.hop.worker.*`.\n  const receivedAtEpochMs = hopEpochNow();\n  // \uD83D\uDCEC\uFE0F The host said something. A drive in flight reads this counter every step and crosses back the\n  // moment it moves, so an ingress message, a cancel or a view-state change never waits behind a\n  // guest the worker is pumping. Bumped BEFORE any dispatch, for every kind without exception.\n  hostInputSeq += 1;\n  const msg = event.data ?? {};\n  const { kind } = msg;\n  if (kind === \"attachHeartbeatSab\") {\n    heartbeatShardIndex = msg.shardIndex;\n    heartbeatSabView = msg.sab ? new Int32Array(msg.sab) : null;\n    return;\n  }\n  if (kind === \"dispose\") {\n    const actor = actors.get(msg.actorId);\n    if (!actor || actor.activationGeneration !== msg.activationGeneration) return;\n    actors.delete(msg.actorId);\n    inFlightTurnActors.delete(msg.actorId);\n    grantedBudgets.delete(msg.actorId);\n    return;\n  }\n  // \uD83E\uDDEA\uFE0F terra-web-bridges: an effect-complete/effect-error `\"frame\"` is a REPLY to something THIS\n  // worker sent (`\uD83D\uDFE8\uFE0F.js`'s `effectRequest`), never a request expecting a `reply()` of its\n  // own \u2014 settled directly, before the generic requestId/actorId-gated dispatch below (which always\n  // posts a `\"result\"` back, wrong for a message that is itself already an answer).\n  if (kind === \"frame\" && msg.frame && msg.frame.kind === \"Envelope\" && msg.frame.envelope && msg.frame.envelope.payload && (msg.frame.envelope.payload.kind === \"effect-complete\" || msg.frame.envelope.payload.kind === \"effect-error\")) {\n    deliverEffectResult(msg.actorId, msg.activationGeneration, msg.frame.envelope);\n    return;\n  }\n  const { requestId, actorId } = msg;\n  if (!requestId || !actorId) return;\n  const timings = { postedAtEpochMs: typeof msg.postedAtEpochMs === \"number\" ? msg.postedAtEpochMs : null, receivedAtEpochMs, guestEnteredAtEpochMs: null, guestLeftAtEpochMs: null, repliedAtEpochMs: null, previousReplyCloneMs: lastReplyCloneMs, events: Array.isArray(msg.events) ? msg.events.length : 0, eventKinds: Array.isArray(msg.events) ? [...new Set(msg.events.map((entry) => (entry && typeof entry.kind === \"string\" ? entry.kind : \"?\")))].slice(0, 4).join(\"+\") : \"\", patches: 0, commandPageBytes: msg.commandPage && msg.commandPage.bytes ? msg.commandPage.bytes.byteLength ?? msg.commandPage.bytes.length ?? 0 : 0 };\n  beat();\n  beginRequest();\n  faultPhase = kind;\n  faultActorId = actorId;\n  try {\n    if (kind === \"activate\") {\n      const entry = await loadActor(actorId, msg.activationGeneration, msg.moduleUrl);\n      entry.pendingAssets = msg.assets ?? [];\n      reply(requestId, undefined);\n      return;\n    }\n    const actor = actors.get(actorId);\n    // \uD83D\uDED1\uFE0F A cancel for an actor that is already gone is a no-op the CALLER still has to be told about:\n    // `ShardClient.cancelJob` awaits a `\"result\"`, so answering nothing leaves the request outstanding\n    // in `pending` forever and the watchdog kills the shard over it (ticket\n    // 26/09/02/PUZZLE-3D-END-TO-END wave B42). Every other kind genuinely needs the actor.\n    if (!actor && kind === \"cancelJob\") {\n      reply(requestId, undefined);\n      return;\n    }\n    if (!actor) throw new Error(`shard worker: actor ${actorId} not activated`);\n    switch (kind) {\n      case \"turn\": {\n        if (actor.activationGeneration !== msg.activationGeneration) throw new Error(\"actor-lifecycle.activation-mismatch\");\n        if (inFlightTurnActors.has(actorId)) throw new Error(`shard worker: actor ${actorId} already has a turn in flight`);\n        inFlightTurnActors.add(actorId);\n        faultPhase = (actor.turns ?? 0) === 0 ? \"first-step\" : \"turn\";\n        actor.turns = (actor.turns ?? 0) + 1;\n        try {\n          const admitted = spliceInstanceOpenAssets(actor, msg.events);\n          timings.guestEnteredAtEpochMs = hopEpochNow();\n          let result = await actor.api.poll(admitted, msg.commandPage, undefined, msg.budget);\n          // \uD83D\uDE9A\uFE0F THE worker-owned MoreWork drive. A `more-work` answer that carried nothing is not a\n          // message for the host \u2014 it is the guest asking to be pumped, and in the browser the host\n          // round trip IS the pump. So the WORKER owns that pump: it runs the next turn itself and\n          // crosses only to deliver something the host can act on. Every discarded result satisfied\n          // `shardTurnCarriesNothing`, so the drive is lossless by construction.\n          //\n          // Five other stops, and each one is a promise to somebody: the guest went `idle`;\n          // `hostInputSeq` moved, so a host-owned input is pending behind the drive and must not wait\n          // for it; the actor was disposed or re-activated; the derived step ceiling is spent; the\n          // derived wall budget is spent. The ceiling and the budget come from\n          // `shardTurnDriveSteps`/`shardTurnDriveBudgetMs` \u2014 the wall the host GRANTED for this\n          // crossing divided by what one turn MEASURES \u2014 never from the reactor's own 8 ms executor\n          // slice, which is smaller than one whole turn and therefore admitted no further turn at all.\n          const grantWallMs = msg.budget && typeof msg.budget.wallMs === \"number\" ? msg.budget.wallMs : MAINTENANCE_LANE_DEFAULT_BUDGET.wallMs;\n          // \uD83D\uDE9A\uFE0F The WALL bounds this drive; `DRIVE_STEP_CEILING` is the hard backstop against an\n          // unbounded loop, and `shardTurnDriveSteps` is the grant's derived EXPECTATION, reported so\n          // the host can read what one crossing was meant to cover.\n          const driveSteps = DRIVE_STEP_CEILING;\n          const driveExpectedSteps = shardTurnDriveSteps(grantWallMs, GUEST_TURN_COST_MS);\n          const driveDeadline = hopEpochNow() + shardTurnDriveBudgetMs(grantWallMs, GUEST_TURN_COST_MS);\n          const inputMark = hostInputSeq;\n          let drivePolls = 0;\n          let driveStopped = shardTurnStatusTag(result) === \"more-work\" ? (shardTurnCarriesNothing(result) ? \"\" : \"carried\") : \"idle\";\n          while (driveStopped === \"\") {\n            if (hostInputSeq !== inputMark) { driveStopped = \"input\"; break; }\n            if (actors.get(actorId) !== actor || actor.activationGeneration !== msg.activationGeneration) { driveStopped = \"closed\"; break; }\n            if (hopEpochNow() >= driveDeadline) { driveStopped = \"budget\"; break; }\n            if (drivePolls + 1 >= driveSteps) { driveStopped = \"steps\"; break; }\n            // \uD83D\uDCEC\uFE0F One macrotask before the next turn, so a message the host already posted is\n            // DELIVERED and `hostInputSeq` can actually move. Without it the drive is\n            // uninterruptible: an `await` settles on the microtask queue, which never drains the\n            // message queue.\n            await driveYield();\n            if (hostInputSeq !== inputMark) { driveStopped = \"input\"; break; }\n            if (actors.get(actorId) !== actor || actor.activationGeneration !== msg.activationGeneration) { driveStopped = \"closed\"; break; }\n            result = await actor.api.poll([], undefined, undefined, msg.budget);\n            drivePolls += 1;\n            driveStopped = shardTurnStatusTag(result) === \"more-work\" ? (shardTurnCarriesNothing(result) ? \"\" : \"carried\") : \"idle\";\n          }\n          timings.guestLeftAtEpochMs = hopEpochNow();\n          timings.drivePolls = drivePolls;\n          timings.driveSteps = driveExpectedSteps;\n          timings.driveStopped = driveStopped;\n          timings.patches = result && Array.isArray(result.uiPatches) ? result.uiPatches.length : 0;\n          timings.status = result && result.status && typeof result.status.tag === \"string\" ? result.status.tag : String(result && result.status);\n          // \uD83E\uDEC0\uFE0F THE step boundary. A guest running a BUDGETED job (a resumable tessellation, a\n          // resumable boolean) crosses this point once per step and blocks the event loop in\n          // between, so the while-busy ticker cannot fire and the only thing that distinguishes it\n          // from a dead worker is a beat taken HERE, the moment the guest hands control back\n          // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,\n          // `\uD83D\uDCD3\uFE0Fextension-evaluate-budget-2026-09-12.md`). It RIDES the reply rather than costing its\n          // own `postMessage`, because the reply is already crossing at this exact instant and\n          // `ShardClient` reads liveness off every inbound message it receives.\n          reply(requestId, result, timings, beat(\"turn-step\"));\n        } finally {\n          inFlightTurnActors.delete(actorId);\n        }\n        break;\n      }\n      case \"startJob\":\n        await actor.api.startJob(msg.job, msg.jobKind, msg.input);\n        reply(requestId, undefined);\n        break;\n      case \"stepJob\": {\n        // \uD83E\uDEC0\uFE0F Same step boundary as `turn` above: a job step is exactly the unit a budgeted guest\n        // yields at, so it is exactly where liveness is provable.\n        const step = await actor.api.stepJob(msg.job, msg.budget);\n        reply(requestId, step, undefined, beat(\"turn-step\"));\n        break;\n      }\n      case \"cancelJob\":\n        await actor.api.cancelJob(msg.job);\n        reply(requestId, undefined);\n        break;\n      case \"takeSegmentedDownloadChunk\": {\n        if (!Number.isSafeInteger(msg.instanceId) || msg.instanceId < 0 || typeof msg.operationId !== \"bigint\" || msg.operationId <= 0n || msg.operationId > ((1n << 64n) - 1n)) throw new Error(\"segmented-download-authority-invalid\");\n        const chunk = await actor.api.takeSegmentedDownloadChunk(msg.instanceId, msg.operationId);\n        // \uD83D\uDCE4\uFE0F One bound per refusal, never one \"limit\" for four different violations: a chunk of the wrong\n        // TYPE is a wire-adaptation defect (the guest's `option<list<u8>>` reaching this worker as jco's\n        // tagged `{ tag, val }` object instead of a `Uint8Array` \u2014 what the 2026-09-12 battery read as\n        // an exceeded byte cap while nothing was over any cap), an empty or oversized one is a producer\n        // that sliced by the wrong constant.\n        if (chunk !== undefined && chunk !== null) {\n          if (Object.prototype.toString.call(chunk) !== \"[object Uint8Array]\") throw new Error(\"segmented-download-chunk-type\");\n          if (chunk.byteLength === 0) throw new Error(\"segmented-download-chunk-empty\");\n          if (chunk.byteLength > SEGMENTED_DOWNLOAD_CHUNK_BYTES) throw new Error(\"segmented-download-chunk-over-cap\");\n        }\n        reply(requestId, chunk ?? undefined);\n        break;\n      }\n      case \"checkpoint\":\n        reply(requestId, await actor.api.checkpoint());\n        break;\n      case \"restore\":\n        await actor.api.restore(msg.state);\n        reply(requestId, undefined);\n        break;\n      case \"frame\": {\n        if (actor.activationGeneration !== msg.activationGeneration) throw new Error(\"actor-lifecycle.activation-mismatch\");\n        const result = interpretFrame(msg.frame, actorId);\n        if (result.action === \"register\") {\n          reply(requestId, undefined);\n          break;\n        }\n        if (result.action === \"unregister\") {\n          grantedBudgets.delete(actorId);\n          reply(requestId, undefined);\n          break;\n        }\n        if (result.action === \"unknown\") {\n          reply(requestId, { ignored: true });\n          break;\n        }\n        if (inFlightTurnActors.has(actorId)) throw new Error(`shard worker: actor ${actorId} already has a turn in flight`);\n        inFlightTurnActors.add(actorId);\n        try {\n          const events = spliceInstanceOpenAssets(actor, result.envelopes.map((envelope) => envelope.payload));\n          timings.events = events.length;\n          timings.guestEnteredAtEpochMs = hopEpochNow();\n          const polled = await actor.api.poll(events, undefined, undefined, result.budget);\n          timings.guestLeftAtEpochMs = hopEpochNow();\n          reply(requestId, polled, timings);\n        } finally {\n          inFlightTurnActors.delete(actorId);\n        }\n        break;\n      }\n      default:\n        throw new Error(`unknown shard worker message kind: ${kind}`);\n    }\n  } catch (error) {\n    // \uD83E\uDE7A\uFE0F Reported on BOTH channels on purpose: `replyError` answers the one caller that is awaiting\n    // this request, `reportWorkerFault` names the phase/actor/module to the shell's console for the\n    // boot faults nobody is awaiting a reply for. A retryable lifecycle-turn deadline is neither \u2014\n    // it is a yield the client replays, so it never reaches `onActorTrap`.\n    const retryableLifecycle = kind === \"turn\" && retryableLifecycleTurn(error, msg.events, msg.commandPage);\n    if (retryableLifecycle) console.log(`shard worker: retryable lifecycle deadline on ${faultPhase} for actor ${actorId}; receipt retained, replaying next tick`);\n    else reportWorkerFault(\"handler\", error, null);\n    replyError(requestId, error, msg.events, retryableLifecycle);\n  } finally {\n    endRequest();\n    faultPhase = \"idle\";\n  }\n});\n");
}
/**
 * @emoji 🌉️ Normalizes ONE actor's jco-transpiled component (`world actor`: exports `reactor`/
 * `jobs`/`checkpoint`/`describe`, imports only `pure` — see `component.wit`) behind the flat
 * `createActorApi()` shape `🟨️shard-worker.js` calls: `poll`/`startJob`/`stepJob`/`cancelJob`/
 * `takeSegmentedDownloadChunk`/`checkpoint`/`restore`.
 *
 * DROPS the old `runSerialized` retry/reload loop entirely (design-runtime.md §3: "recovery is the
 * kernel's job now"). Under the old ABI a guest panic (`panic = "abort"`, no unwind) permanently
 * killed the wasm32-wasip2 instance, and — with no host-side supervisor — the ONLY recovery available
 * was this bridge silently re-importing the module and replaying. Now `ActivationRegistry`'s
 * `FailurePolicy` (design-runtime.md §1) owns that: a trap here just throws, `🟨️shard-worker.js`'s
 * `replyError` propagates it to `ShardClient` as a rejected turn, and the KERNEL decides
 * `Trapped{restarts}` → drop + re-instantiate (fresh `activate`) + `restore()` the last checkpoint —
 * the SAME re-instantiation this bridge used to do blindly, now a supervised decision instead of a
 * local guess with no visibility into checkpoint state.
 *
 * 🚧 UNVERIFIED against a real compiled artifact of the PRODUCTION `world actor` component (the
 * wasm32-wasip2 fleet does not currently compile — a large in-flight conversion tracked elsewhere on
 * this ticket): the exact jco-generated export shape for a world that exports several *interfaces*
 * (rather than bare functions) is assumed here to be one JS binding per interface, named for the
 * interface (\`reactor\`/\`jobs\`/\`checkpoint\`/\`describe\`), field names camelCased from the WIT's
 * kebab-case. **This IS confirmed for a single-export-interface world against a real transpiled
 * component** (📓️terra-jco-spike-report.md's jcoprobe fixture: \`export * as probe from
 * './interfaces/...'\`, camelCased function names) — the multi-interface-export case (4 interfaces,
 * matching \`world actor\`) is extrapolated from that single-interface evidence plus jco's documented
 * per-interface naming convention, not independently re-confirmed here. If jco nests these
 * differently, only the four destructured names below need to change — every other line here is
 * interface-shape-agnostic.
 *
 * 🧪️ terra-web-bridges: every destructured method now returns a Promise (every WIT function in the
 * target world is `async func`) — made EXPLICITLY `async` here rather than relying on bare pass-
 * through. The component and its host imports use matching actor/activation/version URLs; a static
 * relative shim import would otherwise be shared across same-package activations. Each returned API
 * retains that immutable shim module for replies, including while other actors are interleaved.
 */
function pluginComponentBridgeSource(componentBase, wasmFileName) {
    return "/** @generated semio actor jco component bridge */\n\nconst ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES = ".concat(____ts_1.ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES, ";\nconst encodeActorInstanceLifecycle = ").concat(____ts_1.encodeActorInstanceLifecycle.toString(), ";\nconst ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES = ").concat(____ts_2.ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES, ";\nconst encodeActorUiPatchReceipt = ").concat(____ts_2.encodeActorUiPatchReceipt.toString(), ";\nconst validateActorUiPatchPairing = ").concat(____ts_2.validateActorUiPatchPairing.toString(), ";\nconst commandIngressKinds = new Map([[0, \"idle\"], [1, \"page-accepted\"], [2, \"backpressure\"], [3, \"command-pending\"], [4, \"command-complete\"], [5, \"fault\"]]);\n\n/** \uD83C\uDF81\uFE0F jco lifts `option<t>` as a tagged `{ tag: \"none\" | \"some\" }` variant; every host-side reader\n * below wants the bare value (or nothing), so unwrap exactly that shape and pass anything else through. */\nfunction unwrapOption(value) {\n  if (value === null || value === undefined) return undefined;\n  if (typeof value === \"object\" && (value.tag === \"none\" || value.tag === \"some\")) return value.tag === \"none\" ? undefined : value.val;\n  return value;\n}\n\nfunction lifecycleBody(value) {\n  return { lifetime: value.lifetime, requestSequence: BigInt(value.requestSequence), ...(\"closeGeneration\" in value ? { closeGeneration: value.closeGeneration } : {}) };\n}\n\nfunction lifecycleEvent(kind, payload, activationGeneration) {\n  if (kind === \"patch-ack\" || kind === \"patch-rejected\") {\n    encodeActorUiPatchReceipt(payload.receipt);\n    if (payload.receipt.lifetime.activationGeneration !== activationGeneration || payload.surface?.instance !== payload.receipt.lifetime.instanceId) throw new Error(\"actor-ui-patch.activation-mismatch\");\n    return { tag: kind, val: payload };\n  }\n  if (kind === \"instance-open\") {\n    encodeActorInstanceLifecycle({ kind: \"open\", activationGeneration: payload.activationGeneration, instanceId: payload.instance, requestSequence: payload.requestSequence });\n    if (payload.activationGeneration !== activationGeneration) throw new Error(\"actor-lifecycle.activation-mismatch\");\n    return { tag: kind, val: { ...payload, requestSequence: BigInt(payload.requestSequence) } };\n  }\n  if (kind === \"instance-close\" || kind === \"instance-lifecycle-ack\") {\n    if (payload?.kind !== (kind === \"instance-close\" ? \"close\" : \"ack\")) throw new Error(\"actor-lifecycle.event-kind\");\n    encodeActorInstanceLifecycle(payload);\n    const receipt = kind === \"instance-close\" ? payload : payload.receipt;\n    if (receipt.lifetime.activationGeneration !== activationGeneration) throw new Error(\"actor-lifecycle.activation-mismatch\");\n    return { tag: kind, val: kind === \"instance-close\" ? lifecycleBody(receipt) : { tag: receipt.kind, val: lifecycleBody(receipt) } };\n  }\n  return kind === \"wake\" ? ({ tag: kind }) : ({ tag: kind, val: payload });\n}\n\n// \uD83E\uDDE9\uFE0F `jobs::job-step` reaches the worker as jco's raw `{tag, val}` variant; `ShardJobStep`\n// (`\uD83D\uDCEE\uFE0Fshard-client/\uD83D\uDFE6\uFE0F.ts`) is the `{status, ...}` shape every caller declares. Converting here \u2014 the\n// one place that owns the guest boundary \u2014 keeps that declaration honest instead of leaving the two\n// shapes silently different, which is what they were while nothing on this target ever stepped a job.\nfunction normalizeJobStep(step) {\n  if (step?.tag === \"running\") return { status: \"running\", progress: unwrapOption(step.val) };\n  if (step?.tag === \"done\") return { status: \"done\", value: step.val };\n  if (step?.tag === \"failed\") return { status: \"failed\", value: step.val };\n  throw new Error(`unknown job-step shape: ${JSON.stringify(step)}`);\n}\n\nfunction lifecycleReceipt(raw, activationGeneration) {\n  const value = unwrapOption(raw);\n  if (value === undefined || value === null) return undefined;\n  if (value.tag !== \"captured\" && value.tag !== \"accepted\" && value.tag !== \"retired\") throw new Error(\"actor-lifecycle.receipt-required\");\n  const body = value.val;\n  if (typeof body?.requestSequence !== \"bigint\" || body.requestSequence <= 0n || body.requestSequence > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error(\"actor-lifecycle.request-sequence\");\n  if (body.lifetime?.activationGeneration !== activationGeneration) throw new Error(\"actor-lifecycle.activation-mismatch\");\n  return encodeActorInstanceLifecycle({ kind: value.tag, lifetime: body.lifetime, requestSequence: Number(body.requestSequence), ...(value.tag === \"captured\" ? {} : { closeGeneration: body.closeGeneration }) });\n}\n\nfunction normalizeCommandIngress(status) {\n  const tag = commandIngressKinds.get(status.kind);\n  if (!tag) throw new Error(`unknown command ingress kind: ${status.kind}`);\n  if (tag === \"idle\") return { tag };\n  if (tag === \"fault\") return { tag, val: { cursor: status.cursor, fault: { tag: \"fault\", val: status.fault } } };\n  return { tag, val: status.cursor };\n}\n\nfunction uiPatchReceipt(result, activationGeneration) {\n  if (!Array.isArray(result.uiPatches)) throw new Error(\"actor-ui-patch.envelope\");\n  const receipt = unwrapOption(result.uiPatchReceipt);\n  validateActorUiPatchPairing(result.uiPatches.length, receipt);\n  if (receipt === undefined || receipt === null) return undefined;\n  if (receipt.lifetime.activationGeneration !== activationGeneration) throw new Error(\"actor-ui-patch.activation-mismatch\");\n  return encodeActorUiPatchReceipt(receipt);\n}\n\nexport async function createActorApi(actorId, activationGeneration) {\n  if (typeof activationGeneration !== \"bigint\" || activationGeneration <= 0n || activationGeneration > 0xffffffffffffffffn) throw new Error(\"actor-close.invalid-activation-generation\");\n  const componentUrl = new URL(\"./").concat(componentBase, ".js\", import.meta.url);\n  const rebuildVersion = new URL(import.meta.url).searchParams.get(\"v\");\n  componentUrl.searchParams.set(\"actor\", actorId);\n  componentUrl.searchParams.set(\"activation\", activationGeneration.toString());\n  if (rebuildVersion) componentUrl.searchParams.set(\"v\", rebuildVersion);\n  const hostUrl = new URL(\"./").concat(exports.PLUGIN_HOST_SHIM_FILE, "\", import.meta.url);\n  hostUrl.search = componentUrl.search;\n  const hostShim = await import(hostUrl.href);\n  const { reactor, jobs, checkpoint, describe } = await import(componentUrl.href);\n  return {\n    poll: async (events, commandPage, coldPairPage, budget) => {\n      if (commandPage) await reactor.stageCommandPage(commandPage.cursor, commandPage.bytes);\n      if (coldPairPage) await reactor.stageColdPairPage(coldPairPage);\n      const result = await reactor.poll(events.map(({ kind, payload }) => lifecycleEvent(kind, payload, activationGeneration)), { fuel: BigInt(budget.fuel), deadlineMs: budget.wallMs, maxEffects: budget.maxEffects, maxPatchBytes: budget.maxPatchBytes, maxFrames: 8 });\n      return { ...result, nextWake: unwrapOption(result.nextWake) ?? null, lifecycleReceipt: lifecycleReceipt(result.lifecycleReceipt, activationGeneration), uiPatchReceipt: uiPatchReceipt(result, activationGeneration), commandIngress: normalizeCommandIngress(result.commandIngress) };\n    },\n    startJob: async (job, kind, input) => jobs.startJob(job, kind, input),\n    stepJob: async (job, budget) => normalizeJobStep(await jobs.stepJob(job, budget)),\n    cancelJob: async (job) => jobs.cancelJob(job),\n    // \uD83D\uDCE4\uFE0F THE 2026-09-12 segmented-export fault: this export returns `option<list<u8>>`, and jco's flat\n    // lift of an option yields the TAGGED variant object (`_liftFlatOption` delegates to\n    // `_liftFlatVariant`, which builds `{ tag }`/`{ tag, val }`; only `_liftFlatEnum` reduces to\n    // `.tag`). Handed on raw it reached the shard worker as `[object Object]` \u2014 so chunk 0 faulted the\n    // worker and the terminal `{ tag: \"none\" }` was not `undefined` either. Every other guest option in\n    // this bridge already routes through `unwrapOption`; this one is no exception.\n    takeSegmentedDownloadChunk: async (instanceId, operationId) => unwrapOption(await jobs.takeSegmentedDownloadChunk(instanceId, operationId)),\n    checkpoint: async () => checkpoint.checkpoint(),\n    restore: async (state) => checkpoint.restore(state),\n    describe: async () => describe.describe(),\n    resolveEffect: (requestId, value) => hostShim.__resolveEffect(requestId, value),\n    rejectEffect: (requestId, message) => hostShim.__rejectEffect(requestId, message),\n  };\n}\n");
}
function rewritePreview2ShimImports(componentJsPath, preview2VendorDir) {
    var outDir = (0, node_path_1.dirname)(componentJsPath);
    var rel = (0, node_path_1.relative)(outDir, preview2VendorDir).replace(/\\/g, "/");
    var prefix = rel.endsWith("/") ? rel : "".concat(rel, "/");
    var content = (0, node_fs_1.readFileSync)(componentJsPath, "utf8");
    var rewritten = (0, ____ts_4.rewritePreview2ShimImportSource)(content, prefix);
    if (rewritten !== content)
        (0, node_fs_1.writeFileSync)(componentJsPath, rewritten);
}
var WASM_OPT_ARGS = [
    "-Oz",
    "--low-memory-unused",
    "--strip-debug",
    "--strip-producers",
    "--enable-bulk-memory",
    "--enable-bulk-memory-opt",
    "--enable-call-indirect-overlong",
    "--enable-extended-const",
    "--enable-multivalue",
    "--enable-mutable-globals",
    "--enable-nontrapping-float-to-int",
    "--enable-reference-types",
    "--enable-sign-ext",
];
/** 🪶️ Optimizes extracted component cores with the native tool prepared by Nx. */
function optimizePluginCoreModules(outDir, componentBase, ctx) {
    var _a, _b;
    if (!((_a = ctx.optimize) !== null && _a !== void 0 ? _a : ((0, ____ts_3.semioBuildMode)() === "ship" && process.env.SEMIO_WASM_OPT !== "0")))
        return;
    var selected = (_b = ctx.wasmOptBin) !== null && _b !== void 0 ? _b : process.env.SEMIO_WASM_OPT_BIN;
    var wasmOptBin = selected ? (0, node_path_1.resolve)(ctx.repoRoot, selected) : (0, ___script_ts_1.preparedBinaryen)(ctx.repoRoot);
    for (var _i = 0, _c = (0, node_fs_1.readdirSync)(outDir); _i < _c.length; _i++) {
        var file = _c[_i];
        if (!file.startsWith("".concat(componentBase, ".core")) || !file.endsWith(".wasm"))
            continue;
        var coreWasm = (0, node_path_1.join)(outDir, file);
        var optimized = "".concat(coreWasm, ".opt");
        if ((0, ____ts_3.runCmdStatus)(wasmOptBin, __spreadArray(__spreadArray([coreWasm], WASM_OPT_ARGS, true), ["-o", optimized], false), { cwd: ctx.repoRoot, budgetMs: (0, ____ts_3.buildBudgetMs)() }) !== 0) {
            throw new Error("wasm-opt failed for ".concat(coreWasm));
        }
        (0, node_fs_1.renameSync)(optimized, coreWasm);
    }
}
//#region 🧬️JcoAsyncResultLifting
var JCO_INDIRECT_RESULT_MEMORY_GUARD = "if (!ctx.memory) {\n      _debugLog('missing memory despite indirect param usage'";
var JCO_RESOLVED_RESULT_MEMORY_GUARD = "if (!memory) {\n      _debugLog('missing memory despite indirect param usage'";
var JCO_TASK_RETURN_DIRECT_VALUES = 16;
function jcoResultField(node, name) {
    if (node && typescript_1.default.isObjectLiteralExpression(node)) {
        for (var _i = 0, _a = node.properties; _i < _a.length; _i++) {
            var field = _a[_i];
            if (typescript_1.default.isPropertyAssignment(field) && ((typescript_1.default.isIdentifier(field.name) || typescript_1.default.isStringLiteral(field.name)) && field.name.text === name))
                return field.initializer;
        }
    }
    throw new Error("jco task-return metadata is missing ".concat(name));
}
function jcoResultEntries(node) {
    if (node && typescript_1.default.isArrayLiteralExpression(node))
        return node.elements;
    throw new Error("jco task-return metadata must be an array");
}
function jcoResultFlatCount(node) {
    if (node.kind === typescript_1.default.SyntaxKind.NullKeyword)
        return 0;
    if (typescript_1.default.isIdentifier(node)) {
        if (/^_liftFlatString/.test(node.text))
            return 2;
        if (/^_liftFlat(Bool|Char|[SU](8|16|32|64)|Float(32|64))$/.test(node.text))
            return 1;
    }
    if (typescript_1.default.isCallExpression(node) && typescript_1.default.isIdentifier(node.expression)) {
        var meta = node.arguments[0];
        switch (node.expression.text) {
            case "_liftFlatList": return 2;
            case "_liftFlatEnum":
            case "_liftFlatOwn":
            case "_liftFlatBorrow": return 1;
            case "_liftFlatFlags": return Math.ceil(Number(jcoResultField(meta, "size32").getText()) / 4);
            case "_liftFlatRecord": return jcoResultEntries(jcoResultField(meta, "fieldMetas")).reduce(function (sum, field) { return sum + jcoResultFlatCount(jcoResultEntries(field)[1]); }, 0);
            case "_liftFlatTuple": return jcoResultEntries(jcoResultField(meta, "elemLiftFns")).reduce(function (sum, field) { return sum + jcoResultFlatCount(jcoResultEntries(field)[0]); }, 0);
            case "_liftFlatVariant":
            case "_liftFlatOption":
            case "_liftFlatResult": return 1 + Math.max.apply(Math, __spreadArray([0], jcoResultEntries(jcoResultField(meta, "caseMetas")).map(function (field) { return jcoResultFlatCount(jcoResultEntries(field)[1]); }), false));
        }
    }
    throw new Error("unsupported jco task-return lift: ".concat(node.getText()));
}
/** 🧬️ Derives callback directness from each generated result's canonical flattened shape; memory presence alone does not distinguish direct pointer/length values from an indirect return record. */
function rewriteJcoAsyncResultLifting(source) {
    source = source.replace(JCO_INDIRECT_RESULT_MEMORY_GUARD, JCO_RESOLVED_RESULT_MEMORY_GUARD);
    var parsed = typescript_1.default.createSourceFile("component.js", source, typescript_1.default.ScriptTarget.Latest, true, typescript_1.default.ScriptKind.JS);
    var edits = [];
    var visit = function (node) {
        if (typescript_1.default.isCallExpression(node) && typescript_1.default.isPropertyAccessExpression(node.expression) && typescript_1.default.isIdentifier(node.expression.expression) && node.expression.expression.text === "taskReturn" && node.expression.name.text === "bind") {
            var context = node.arguments[node.arguments.length - 1];
            var count = jcoResultEntries(jcoResultField(context, "liftFns")).reduce(function (sum, field) { return sum + jcoResultFlatCount(field); }, 0);
            var direct = jcoResultField(context, "useDirectParams");
            edits.push({ start: direct.getStart(parsed), end: direct.end, value: String(count <= JCO_TASK_RETURN_DIRECT_VALUES) });
        }
        typescript_1.default.forEachChild(node, visit);
    };
    visit(parsed);
    for (var _i = 0, _a = edits.sort(function (left, right) { return right.start - left.start; }); _i < _a.length; _i++) {
        var edit = _a[_i];
        source = source.slice(0, edit.start) + edit.value + source.slice(edit.end);
    }
    return source;
}
/** @emoji 💾️ Applies {@link rewriteJcoAsyncResultLifting} to one freshly transpiled jco module. */
function rewriteJcoAsyncResultLiftingAt(modulePath) {
    var source = (0, node_fs_1.readFileSync)(modulePath, "utf8");
    var rewritten = rewriteJcoAsyncResultLifting(source);
    if (rewritten !== source)
        (0, node_fs_1.writeFileSync)(modulePath, rewritten);
}
//#endregion 🧬️JcoAsyncResultLifting
//#region 🧊️JcoComponentAssetVersioning
var JCO_HOST_SHIM_URL_HELPER = "function __semioActivationHostUrl() {\n  const url = new URL(\"./\uD83D\uDFE8\uFE0F.js\", import.meta.url);\n  const source = new URL(import.meta.url);\n  for (const key of [\"actor\", \"activation\", \"v\"]) {\n    const value = source.searchParams.get(key);\n    if (value !== null) url.searchParams.set(key, value);\n  }\n  return url;\n}";
function rewriteJcoHostShimImports(source) {
    var _a, _b;
    var parsed = typescript_1.default.createSourceFile("component.js", source, typescript_1.default.ScriptTarget.Latest, true, typescript_1.default.ScriptKind.JS);
    var edits = [];
    for (var _i = 0, _c = parsed.statements; _i < _c.length; _i++) {
        var statement = _c[_i];
        if (!typescript_1.default.isImportDeclaration(statement) || !typescript_1.default.isStringLiteral(statement.moduleSpecifier) || statement.moduleSpecifier.text !== "./".concat(exports.PLUGIN_HOST_SHIM_FILE))
            continue;
        var bindings = (_a = statement.importClause) === null || _a === void 0 ? void 0 : _a.namedBindings;
        if (((_b = statement.importClause) === null || _b === void 0 ? void 0 : _b.name) || !bindings || !typescript_1.default.isNamedImports(bindings))
            throw new Error("unsupported jco host shim import shape");
        var fields = bindings.elements.map(function (field) { return field.propertyName ? "".concat(field.propertyName.getText(parsed), ": ").concat(field.name.text) : field.name.text; }).join(", ");
        edits.push({ start: statement.getStart(parsed), end: statement.end, value: "const { ".concat(fields, " } = await import(__semioActivationHostUrl().href);") });
    }
    for (var _d = 0, _e = edits.reverse(); _d < _e.length; _d++) {
        var edit = _e[_d];
        source = source.slice(0, edit.start) + edit.value + source.slice(edit.end);
    }
    return edits.length === 0 ? source : "".concat(JCO_HOST_SHIM_URL_HELPER, "\n\n").concat(source);
}
var JCO_COMPONENT_ASSET_URL = /new URL\((['"])(\.\/[^'"]+\.core\d*\.wasm)\1,\s*import\.meta\.url\)/g;
var JCO_COMPONENT_ASSET_URL_HELPER = "function __semioVersionedComponentAssetUrl(path) {\n  const url = new URL(path, import.meta.url);\n  const rebuildVersion = new URL(import.meta.url).searchParams.get(\"v\");\n  if (rebuildVersion) url.searchParams.set(\"v\", rebuildVersion);\n  return url;\n}";
/** 🪪️ Preserves activation identity for host imports and rebuild identity for extracted core Wasm. */
function rewriteJcoComponentAssetUrls(source) {
    source = rewriteJcoHostShimImports(source);
    if (source.includes("function __semioVersionedComponentAssetUrl(path)"))
        return source;
    var rewritten = source.replace(JCO_COMPONENT_ASSET_URL, function (_match, quote, assetPath) { return "__semioVersionedComponentAssetUrl(".concat(quote).concat(assetPath).concat(quote, ")"); });
    return rewritten === source ? source : "".concat(JCO_COMPONENT_ASSET_URL_HELPER, "\n\n").concat(rewritten);
}
/** @emoji 💾️ Applies {@link rewriteJcoComponentAssetUrls} to one freshly transpiled jco module. */
function rewriteJcoComponentAssetUrlsAt(modulePath) {
    var source = (0, node_fs_1.readFileSync)(modulePath, "utf8");
    var rewritten = rewriteJcoComponentAssetUrls(source);
    if (rewritten !== source)
        (0, node_fs_1.writeFileSync)(modulePath, rewritten);
}
//#endregion 🧊️JcoComponentAssetVersioning
function transpilePluginComponent(artifact, outDir, componentBase, ctx) {
    // 🧪️ terra-web-bridges (📓️terra-jco-spike-report.md "what must change" #2): NO `--async-mode`
    // flag — confirmed byte-identical to jco's bare/"sync" default for a component whose every WIT
    // function is already `async func` (`--async-mode jspi` was diffed against the bare transpile of
    // the SAME wasm and produced 0 bytes of difference). `world actor`'s import surface is now `pure`
    // (component.wit's `interface pure { log; now-ms; trace-span; }`, still plain `func`) PLUS
    // `host-async` (`interface host-async`, ~:887 — 24 `async func` imports + `emit`/`emit-patch`) —
    // both map to the SAME `🟨️.js`, which now implements both interfaces' exports from one file.
    if ((0, ____ts_3.runNodeBinStatus)(["@bytecodealliance/jco", "transpile", artifact, "-o", outDir, "--name", componentBase, "--map", "semio:framework/pure=./🟨️.js", "--map", "semio:framework/host-async=./🟨️.js"], ctx.repoRoot) !== 0) {
        throw new Error("jco transpile failed for ".concat(artifact));
    }
    rewriteJcoAsyncResultLiftingAt((0, node_path_1.join)(outDir, "".concat(componentBase, ".js")));
    rewriteJcoComponentAssetUrlsAt((0, node_path_1.join)(outDir, "".concat(componentBase, ".js")));
    optimizePluginCoreModules(outDir, componentBase, ctx);
    rewritePreview2ShimImports((0, node_path_1.join)(outDir, "".concat(componentBase, ".js")), ctx.preview2VendorDir);
}
/** @emoji 🚀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T-P8): non-blocking subprocess spawn, used ONLY
 * by {@link transpilePluginComponentAsync} below. The shared repo-lib's `runNodeBinStatus`/
 * `runCmdStatus` (used by the SYNC {@link transpilePluginComponent} above, which stays exactly as-is
 * for its one other caller, the extension store's `webMaterialize`) both wrap Node's `spawnSync` —
 * correct and desired for a genuinely one-at-a-time step (e.g. `cargo build`), but fatal to any attempt
 * at running several plugins' jco transpile CONCURRENTLY: an async concurrency limiter wrapped around a
 * synchronous blocking call achieves zero real overlap, since nothing else in this process can run
 * while the thread is stuck inside `spawnSync`. `stdio` is piped rather than inherited for the same
 * reason: several of these may be in flight at once, and `"inherit"` would interleave unrelated
 * processes' output byte-by-byte on the parent's own stdout/stderr; buffered output is instead
 * surfaced (as one block) only on failure. Reuses the shared repo-lib's `resolveWorkspaceBin` for the
 * exact same monorepo-aware `.bin/` lookup `runNodeBinStatus` itself uses, rather than
 * reimplementing it. */
function spawnAsync(cmd, args, cwd, signal) {
    signal === null || signal === void 0 ? void 0 : signal.throwIfAborted();
    return new Promise(function (resolveSpawn, rejectSpawn) {
        var _a, _b;
        var child = (0, node_child_process_1.spawn)(cmd, args, { cwd: cwd, shell: false, detached: signal !== undefined && process.platform !== "win32", stdio: ["ignore", "pipe", "pipe"] });
        var progress = setInterval(function () { return console.log("Running ".concat(cmd.split(/[\\/]/).pop(), "\u2026")); }, 10000);
        var output = "", forced;
        var terminate = function (force) {
            if (force === void 0) { force = false; }
            if (!child.pid)
                return;
            if (process.platform === "win32") {
                (0, node_child_process_1.spawn)("taskkill", ["/pid", String(child.pid), "/T", "/F"], { stdio: "ignore", windowsHide: true });
                return;
            }
            try {
                process.kill(-child.pid, force ? "SIGKILL" : "SIGTERM");
            }
            catch (_a) { }
        };
        var cancel = function () { terminate(); forced !== null && forced !== void 0 ? forced : (forced = setTimeout(function () { return terminate(true); }, 2000)); };
        signal === null || signal === void 0 ? void 0 : signal.addEventListener("abort", cancel, { once: true });
        var capture = function (chunk) { output = (output + chunk.toString("utf8")).slice(-64 * 1024); };
        (_a = child.stdout) === null || _a === void 0 ? void 0 : _a.on("data", capture);
        (_b = child.stderr) === null || _b === void 0 ? void 0 : _b.on("data", capture);
        child.on("error", rejectSpawn);
        child.on("close", function (code) {
            clearInterval(progress);
            if (forced)
                clearTimeout(forced);
            signal === null || signal === void 0 ? void 0 : signal.removeEventListener("abort", cancel);
            if (signal === null || signal === void 0 ? void 0 : signal.aborted)
                rejectSpawn(signal.reason);
            else if (code === 0)
                resolveSpawn();
            else
                rejectSpawn(new Error("".concat(cmd, " ").concat(args.join(" "), " exited with status ").concat(code, "\n").concat(output)));
        });
    });
}
function spawnNodeBinAsync(args, cwd, signal) {
    var binName = args[0];
    var resolved = (0, ____ts_3.resolveWorkspaceBin)(binName, cwd);
    return spawnAsync("node", __spreadArray([resolved !== null && resolved !== void 0 ? resolved : binName], args.slice(1), true), cwd, signal);
}
/** @emoji 🪶️ Async twin of {@link optimizePluginCoreModules} — same ship-mode-only `wasm-opt` pass,
 * same `WASM_OPT_ARGS`, just spawned via {@link spawnAsync} instead of `runCmdStatus`'s `spawnSync` so
 * it can run concurrently with sibling plugins' own optimize pass under
 * `📜️script.ts`'s bounded-parallel materialize stage (T-P8). */
function optimizePluginCoreModulesAsync(outDir, componentBase, ctx) {
    return __awaiter(this, void 0, void 0, function () {
        var selected, wasmOptBin, _i, _a, file, coreWasm, optimized, cause_1;
        var _b, _c, _d;
        return __generator(this, function (_e) {
            switch (_e.label) {
                case 0:
                    if (!((_b = ctx.optimize) !== null && _b !== void 0 ? _b : ((0, ____ts_3.semioBuildMode)() === "ship" && process.env.SEMIO_WASM_OPT !== "0")))
                        return [2 /*return*/];
                    selected = (_c = ctx.wasmOptBin) !== null && _c !== void 0 ? _c : process.env.SEMIO_WASM_OPT_BIN;
                    wasmOptBin = selected ? (0, node_path_1.resolve)(ctx.repoRoot, selected) : (0, ___script_ts_1.preparedBinaryen)(ctx.repoRoot);
                    _i = 0, _a = (0, node_fs_1.readdirSync)(outDir);
                    _e.label = 1;
                case 1:
                    if (!(_i < _a.length)) return [3 /*break*/, 7];
                    file = _a[_i];
                    if (!file.startsWith("".concat(componentBase, ".core")) || !file.endsWith(".wasm"))
                        return [3 /*break*/, 6];
                    coreWasm = (0, node_path_1.join)(outDir, file);
                    optimized = "".concat(coreWasm, ".opt");
                    _e.label = 2;
                case 2:
                    _e.trys.push([2, 4, , 5]);
                    return [4 /*yield*/, spawnAsync(wasmOptBin, __spreadArray(__spreadArray([coreWasm], WASM_OPT_ARGS, true), ["-o", optimized], false), ctx.repoRoot, ctx.signal)];
                case 3:
                    _e.sent();
                    return [3 /*break*/, 5];
                case 4:
                    cause_1 = _e.sent();
                    (_d = ctx.signal) === null || _d === void 0 ? void 0 : _d.throwIfAborted();
                    throw new Error("wasm-opt failed for ".concat(coreWasm), { cause: cause_1 });
                case 5:
                    (0, node_fs_1.renameSync)(optimized, coreWasm);
                    _e.label = 6;
                case 6:
                    _i++;
                    return [3 /*break*/, 1];
                case 7: return [2 /*return*/];
            }
        });
    });
}
/** @emoji 🚀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T-P8): async twin of
 * {@link transpilePluginComponent} — identical jco invocation, ship-mode `wasm-opt` pass, and
 * preview2-shim-import rewrite, but spawned non-blockingly so `📜️script.ts`'s bounded-parallel
 * MATERIALIZE stage (`buildPluginCatalog`) can actually overlap several plugins' transpile/optimize
 * work in wall-clock time — the sync {@link transpilePluginComponent} above cannot provide that overlap
 * no matter how it is scheduled from the caller side (see {@link spawnAsync}'s doc). Kept as a SEPARATE
 * export rather than changing the sync function in place: the extension store's `webMaterialize`
 * (`🏪️store/📥️installation/🟦️.ts`, outside this packet's owned paths) calls the sync version without awaiting
 * it, relying on it blocking until done before it deletes the temp artifact directory in its own
 * `finally` — flipping that function to async out from under that caller would silently race the
 * artifact's cleanup against jco still reading it. */
function transpilePluginComponentAsync(artifact, outDir, componentBase, ctx) {
    return __awaiter(this, void 0, void 0, function () {
        var _a;
        var _b;
        return __generator(this, function (_c) {
            switch (_c.label) {
                case 0:
                    _c.trys.push([0, 2, , 3]);
                    // 🧪️ terra-web-bridges: same flags/map pair as the sync {@link transpilePluginComponent} above —
                    // see that function's own doc for why no `--async-mode` flag is needed and why `host-async` maps
                    // to the same shim file `pure` already does.
                    return [4 /*yield*/, spawnNodeBinAsync(["@bytecodealliance/jco", "transpile", artifact, "-o", outDir, "--name", componentBase, "--map", "semio:framework/pure=./🟨️.js", "--map", "semio:framework/host-async=./🟨️.js"], ctx.repoRoot, ctx.signal)];
                case 1:
                    // 🧪️ terra-web-bridges: same flags/map pair as the sync {@link transpilePluginComponent} above —
                    // see that function's own doc for why no `--async-mode` flag is needed and why `host-async` maps
                    // to the same shim file `pure` already does.
                    _c.sent();
                    return [3 /*break*/, 3];
                case 2:
                    _a = _c.sent();
                    (_b = ctx.signal) === null || _b === void 0 ? void 0 : _b.throwIfAborted();
                    throw new Error("jco transpile failed for ".concat(artifact));
                case 3:
                    rewriteJcoAsyncResultLiftingAt((0, node_path_1.join)(outDir, "".concat(componentBase, ".js")));
                    rewriteJcoComponentAssetUrlsAt((0, node_path_1.join)(outDir, "".concat(componentBase, ".js")));
                    return [4 /*yield*/, optimizePluginCoreModulesAsync(outDir, componentBase, ctx)];
                case 4:
                    _c.sent();
                    rewritePreview2ShimImports((0, node_path_1.join)(outDir, "".concat(componentBase, ".js")), ctx.preview2VendorDir);
                    return [2 /*return*/];
            }
        });
    });
}
/**
 * @emoji 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2/H2, design-abi.md §1) + 🧪️ terra-web-bridges
 * (async-worlds). `pure` (`interface pure { log; now-ms; trace-span; }`, component.wit ~:823) stays
 * plain synchronous `func` and is unchanged from H2 — the old `host` world's larger surface
 * (`read-document`/`write-document`/`open-window`/`invoke-action`/`read-asset`/`network-fetch`/
 * `write-blob`/`read-blob`, plus the ad hoc `backboneSend`/`backbonePoll`/`backboneStatus`
 * worker-postMessage relay) is still gone.
 *
 * NEW: `host-async` (component.wit ~:887 — 24 `async func` imports plus the two fire-and-forget
 * `emit`/`emit-patch` doors) is now ALSO implemented in this one file, mapped alongside `pure` by
 * `transpilePluginComponent`'s `--map` pair. Every async import posts an `effect-request` and returns
 * a Promise settled by a later `effect-complete`/`effect-error` — see `effectRequest`'s own doc below
 * for the exact `ShardFrame`/`ShardEnvelope` shape it rides (reused verbatim from
 * `🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`, never a second wire). Host state belongs to
 * an immutable actor/activation/version module URL shared by the component and its returned API.
 *
 * 🚧 UNPROVEN beyond the jcoprobe fixture (📓️terra-jco-spike-report.md): (a) whether jco expects a
 * `result<T, pack>`-returning host-async import to signal `Err` by throwing — jcoprobe's own
 * `probe-host` never used a `result<>` return, so `effectRequest` rejecting on `effect-error` follows
 * jco's documented host-import convention, not a spike-confirmed one; (b) generated JavaScript and
 * ShardClient ownership tests do not establish a fresh guest round trip through every host import.
 */
function hostShimSource() {
    return "/** @generated semio actor host shim \u2014 implements the pure AND host-async import interfaces\n * (component.wit ~:823 / ~:887). See plugin-web-materialize.ts's hostShimSource doc for the design\n * this file is generated from. */\n\n//#region \uD83E\uDDEC\uFE0Fpure\nexport function log(level, message) {\n  if (level === \"error\") console.error(`[actor] ${message}`);\n  else console.log(`[actor] ${message}`);\n}\n\nexport function nowMs() {\n  return BigInt(Date.now());\n}\n\nexport function traceSpan(name) {\n  if (typeof performance !== \"undefined\" && typeof performance.mark === \"function\") performance.mark(name);\n}\n//#endregion \uD83E\uDDEC\uFE0Fpure\n\n//#region \uD83C\uDF09\uFE0Fhost-async\nconst bindingUrl = new URL(import.meta.url);\nconst boundActorId = bindingUrl.searchParams.get(\"actor\");\nconst generationText = bindingUrl.searchParams.get(\"activation\");\nconst boundActivationGeneration = generationText && /^[1-9][0-9]*$/.test(generationText) && generationText.length <= 20 ? BigInt(generationText) : null;\nlet effectSeq = 0;\nconst pendingEffects = new Map();\n\nfunction assertHostActivation() {\n  if (!boundActorId || boundActivationGeneration === null || boundActivationGeneration > 0xffffffffffffffffn) throw new Error(\"actor-activation.host-unbound\");\n}\n\n// \uD83C\uDF09\uFE0F Settles the Promise `effectRequest` handed back for `requestId` \u2014 called by `\uD83D\uDFE8\uFE0Fshard-worker.js`\n// when an `effect-complete`/`effect-error` envelope for this actor arrives.\nexport function __resolveEffect(requestId, value) {\n  const entry = pendingEffects.get(requestId);\n  if (!entry) return;\n  pendingEffects.delete(requestId);\n  entry.resolve(value);\n}\n\nexport function __rejectEffect(requestId, message) {\n  const entry = pendingEffects.get(requestId);\n  if (!entry) return;\n  pendingEffects.delete(requestId);\n  entry.reject(new Error(message));\n}\n\n// \uD83C\uDF0A\uFE0F jco's proven shape for a guest-consumable `stream<u8>` is a plain async generator yielding ONE\n// byte at a time \u2014 confirmed against a real component (jcoprobe's `fetchBody`/`read-body`, S4 in\n// \uD83D\uDCD3\uFE0Fterra-jco-spike-report.md), NOT a `ReadableStream` directly. An `effect-complete` for\n// `http-fetch`/`blob-read` is expected to carry a `ReadableStream` (structured-clone-transferable\n// across `postMessage`); this adapts it into the proven per-byte generator shape. Also accepts an\n// already-async-iterable value so a kernel handing back a plain byte array still works.\nasync function* streamToByteGenerator(body) {\n  if (body == null) return;\n  if (typeof body[Symbol.asyncIterator] === \"function\") {\n    for await (const chunk of body) {\n      if (chunk instanceof Uint8Array) { for (const byte of chunk) yield byte; } else yield chunk;\n    }\n    return;\n  }\n  const reader = body.getReader();\n  try {\n    for (;;) {\n      const { done, value } = await reader.read();\n      if (done) return;\n      if (value instanceof Uint8Array) { for (const byte of value) yield byte; } else yield value;\n    }\n  } finally {\n    reader.releaseLock();\n  }\n}\n\n// \uD83D\uDEAA\uFE0F Every `host-async` ASYNC import funnels through here \u2014 posts one `ShardFrame::Envelope` up to\n// the kernel over the SAME shape `\uD83E\uDDF5\uFE0Fshard-client.ts` declares (`to`/`from`/`lane`/`seq`/\n// `deadlineMs`/`coalesce`/`cancelOf`/`payload`), with `payload: {kind: \"effect-request\", payload:\n// {effect, requestId, params}}` \u2014 the SAME `{kind, payload}` envelope-payload shape\n// `ShardEventEnvelope` already uses for turn events, reused rather than inventing a new one. Resolves\n// or rejects once `__resolveEffect`/`__rejectEffect` fires for the matching `requestId`.\nfunction effectRequest(effect, params) {\n  assertHostActivation();\n  const requestId = `${boundActorId}:${boundActivationGeneration}:${effect}:${++effectSeq}`;\n  return new Promise((resolve, reject) => {\n    pendingEffects.set(requestId, { resolve, reject });\n    self.postMessage({\n      kind: \"frame\",\n      actorId: boundActorId,\n      activationGeneration: boundActivationGeneration,\n      frame: {\n        kind: \"Envelope\",\n        envelope: {\n          to: \"kernel\",\n          from: { kind: \"actor\", id: boundActorId },\n          lane: \"Background\",\n          seq: effectSeq,\n          deadlineMs: null,\n          coalesce: null,\n          cancelOf: null,\n          payload: { kind: \"effect-request\", payload: { effect, requestId, params } },\n        },\n      },\n    });\n  });\n}\n\n// \uD83D\uDEAA\uFE0F `emit`/`emit-patch` are plain (non-async) WIT `func`s \u2014 the ONE fire-and-forget door for the\n// ~24 one-way `effect` variants (plus `respond`) and for UI patches. No `requestId`/Promise: posts\n// and returns immediately, same envelope shape as `effectRequest` above minus the correlation.\nfunction postFireAndForget(kind, payload) {\n  assertHostActivation();\n  self.postMessage({\n    kind: \"frame\",\n    actorId: boundActorId,\n    activationGeneration: boundActivationGeneration,\n    frame: {\n      kind: \"Envelope\",\n      envelope: { to: \"kernel\", from: { kind: \"actor\", id: boundActorId }, lane: \"Background\", seq: ++effectSeq, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind, payload } },\n    },\n  });\n}\n\nconst call = (effect) => (params) => effectRequest(effect, params);\nexport const storageRead = call(\"storage-read\");\nexport const storageWrite = call(\"storage-write\");\nexport const storageDelete = call(\"storage-delete\");\nexport const blobLoad = call(\"blob-load\");\nexport const blobWrite = call(\"blob-write\");\nexport const blobRead = (hash) => effectRequest(\"blob-read\", { hash }).then(streamToByteGenerator);\nexport const httpFetch = (params) => effectRequest(\"http-fetch\", params).then((response) => ({ ...response, body: streamToByteGenerator(response.body) }));\nexport const artifactRead = call(\"artifact-read\");\nexport const documentWrite = call(\"artifact-write\");\nexport const linkResolve = (link) => effectRequest(\"link-resolve\", { link });\nexport const registryQuery = call(\"registry-query\");\nexport const ioCompose = call(\"io-compose\");\nexport const ioRun = call(\"io-run\");\nexport const cacheDerive = call(\"cache-derive\");\nexport const cacheRead = call(\"cache-read\");\nexport const invokeExtension = call(\"invoke-extension\");\nexport const openWindow = call(\"open-window\");\nexport const openDialog = call(\"open-dialog\");\nexport const dispatchAction = call(\"dispatch-action\");\nexport const spawnPluginInstance = call(\"spawn-plugin-instance\");\nexport const requestFileOpen = call(\"request-file-open\");\nexport const requestMediaFrames = call(\"request-media-frames\");\nexport const requestCapability = call(\"request-capability\");\nexport const spawnJob = (job, kind, input, placement) => effectRequest(\"spawn-job\", { job, kind, input, placement });\n\nexport function emit(value) {\n  postFireAndForget(\"effect-emit\", value);\n}\n\nexport function emitPatch(patch) {\n  postFireAndForget(\"ui-patch-emit\", patch);\n}\n//#endregion \uD83C\uDF09\uFE0Fhost-async\n";
}
