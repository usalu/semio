/** 🥾️ The shard worker bootstrap text: the clock, the JSPI gate, fault state and the fault reporter. */
/** 🩺 Returns the worker text from the clock and the stack limit through the JSPI capability gate to the fault reporter. */
export function shardWorkerBootstrapSource(): string {
  return `/** @generated semio shard worker (H2 — bounded pool, actorId-multiplexed) */
import { monotonicClock } from "../🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/clocks.js";
const operationClockUs = () => monotonicClock.now() / 1000n;
// 🩺️ SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION (1-B): raise the captured-frame
// cap BEFORE anything else runs so a deep guest recursion's real stack survives \`error.stack\`
// instead of being truncated to V8's 10-frame default — this worker's stack is otherwise destroyed
// before \`ShardClient\` ever sees it (the main thread only ever saw one frame: \`at worker.onmessage\`).
Error.stackTraceLimit = 200;

// 🧪️ terra-web-bridges: explicit JSPI capability gate — see this file's own header doc ("what must
// change" #1 in 📓️terra-jco-spike-report.md). Every plugin component this worker will ever \`import()\`
// is fully async-lifted, and jco's glue for that unconditionally needs \`WebAssembly.Suspending\`/
// \`WebAssembly.promising\`; without them the FIRST \`import()\` throws \`TypeError: WebAssembly.Suspending
// is not a constructor\` at module top-level, before any plugin call — an opaque failure the spike
// reproduced verbatim under plain Node 24. Posting a \`"trap"\` BEFORE throwing gives \`ShardClient\`'s
// \`onActorTrap\` its best chance at a readable message even where a cross-context Worker \`onerror\`
// gets redacted to \`"undefined undefined undefined"\` (also reproduced by the spike) — \`actorId: "*"\`
// is a worker-wide sentinel, not a real actor, since no actor has activated yet at this point.
if (typeof WebAssembly === "undefined" || typeof WebAssembly.Suspending !== "function" || typeof WebAssembly.promising !== "function") {
  const message = "semio shard worker: this browser/engine lacks JavaScript Promise Integration (JSPI) — WebAssembly.Suspending/WebAssembly.promising are required to run semio's async-lifted plugin components and there is no fallback. Chrome/Edge/Chromium-based browsers ship JSPI on by default; Firefox needs the javascript.options.wasm_js_promise_integration flag in about:config; Node.js needs --experimental-wasm-jspi.";
  self.postMessage({ kind: "trap", actorId: "*", activationGeneration: null, message });
  throw new Error(message);
}

// 🩺️ Nothing inside a Worker is visible from the page unless the worker says it: an exception that
// escapes a message handler, or a promise nobody awaits, reaches the parent as an \`ErrorEvent\` whose
// \`message\` the platform routinely redacts to nothing (exactly what the 2026-09-05 boot logged, four
// times, with no other evidence). These three variables are the breadcrumb trail — updated at every
// boundary so a fault report names the phase, the actor and the module URL it died on — and the two
// global listeners below are what turn an otherwise-anonymous \`Event\` into a readable payload on the
// host side. Registered BEFORE the "message" listener so a permissive test stub that keeps only the
// last handler still keeps the dispatcher.
let faultPhase = "bootstrap";
let faultActorId = null;
let faultModuleUrl = null;

function describeFault(value) {
  if (value instanceof Error) return { message: value.message, stack: value.stack };
  if (value && typeof value === "object" && (typeof value.stack === "string" || typeof value.message === "string")) return { message: String(value), stack: typeof value.stack === "string" ? value.stack : undefined };
  if (value && typeof value === "object") { try { return { message: JSON.stringify(value) }; } catch { return { message: String(value) }; } }
  return { message: String(value) };
}

function reportWorkerFault(source, value, event) {
  const reason = value !== undefined && value !== null ? value : event && typeof event.message === "string" && event.message.length > 0 ? event.message : value;
  const described = describeFault(reason);
  self.postMessage({
    kind: "worker-fault",
    source,
    phase: faultPhase,
    actorId: faultActorId,
    moduleUrl: faultModuleUrl,
    message: described.message,
    stack: described.stack,
    filename: event && typeof event.filename === "string" ? event.filename : undefined,
    lineno: event && typeof event.lineno === "number" ? event.lineno : undefined,
  });
}

self.addEventListener("error", (event) => reportWorkerFault("error", event && event.error, event));
self.addEventListener("unhandledrejection", (event) => reportWorkerFault("unhandledrejection", event && event.reason, null));

`;
}
