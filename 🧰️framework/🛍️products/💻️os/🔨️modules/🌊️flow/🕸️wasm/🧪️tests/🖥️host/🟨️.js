import { FLOW_MAX_REQUEST_BYTES, FlowOperation, attachFlowSurface, createFlowFeatures, createFlowHost, decodeFlowMessage } from "../../📦️packages/🟨️javascript/🖥️flow-host.js";
import { createFlowBrowserRuntime } from "../../📦️packages/🟨️javascript/🌐️flow-browser.js";
import * as flowBrowser from "../../📦️packages/🟨️javascript/🌐️flow-browser.js";
import { readFile } from "node:fs/promises";
import { flowWasmContract } from "../../📦️packages/🟨️javascript/📜️script.ts";
import { deepStrictEqual } from "node:assert";
import { MockFlowBridge } from "../🎭️mock-flow-bridge/🟦️.ts";
import { testFlowOpenOwnership } from "../🔓️open-ownership/🟦️.ts";

const equal = (actual, expected, law) => { if (actual !== expected) throw new Error(`${law}: ${actual} !== ${expected}`); };
const startup = JSON.parse(await readFile(new URL("../../🧪️fixtures/🚀️browser-startup/🔣️.json", import.meta.url), "utf8"));
const sessionClose = JSON.parse(await readFile(new URL("../../🧪️fixtures/🧹️session-close/🔣️.json", import.meta.url), "utf8"));
equal(flowWasmContract("FlowRetainedSessionCloseV1")(sessionClose), true, "session-close-schema");
equal(flowWasmContract("FlowBrowserStartupV1")(startup), true, "startup-schema");
for (const law of startup.cases) equal(law.source === "exports" || law.initializer === "custom" || law.imports === "empty", law.accepted, "startup-independent-admission-oracle");
const memory = new WebAssembly.Memory({ initial: 400 });
const runtimeLifetime = JSON.parse(await readFile(new URL("../../🧪️fixtures/🧑‍🤝‍🧑️browser-runtime/🔣️.json", import.meta.url), "utf8"));
equal(flowWasmContract("FlowBrowserRuntimeLifetimeV1")(runtimeLifetime), true, "runtime-lifetime-schema");
equal(typeof flowBrowser.createFlowBrowserRuntime, "function", "explicit-browser-runtime-owner");
const sharedBridge = new MockFlowBridge(memory);
const runtime = await flowBrowser.createFlowBrowserRuntime({ source: sharedBridge.exports });
equal(sharedBridge.operations.length, runtimeLifetime.initialSessions, "runtime-opens-no-orphan-session");
const sessionA = runtime.openSession();
const sessionB = runtime.openSession();
await Promise.all([sessionA.selectedWidgetIds().result, sessionB.selectedWidgetIds().result]);
deepStrictEqual(sharedBridge.openRequestIds, runtimeLifetime.openRequestIds);
const closeA = sessionA.close();
equal(sessionA.close(), closeA, "session-close-retains-exact-promise");
await closeA;
await sessionB.selectedWidgetIds().result;
equal(sharedBridge.globalCloseCalls, runtimeLifetime.afterCloseA.globalCloseCalls, "session-close-preserves-sibling-runtime");
const beforeDuplicate = sharedBridge.operations.length;
let duplicateRuntime = false;
try { await flowBrowser.createFlowBrowserRuntime({ source: sharedBridge.exports }); } catch { duplicateRuntime = true; }
equal(duplicateRuntime, true, "duplicate-export-owner-rejected");
let copiedExports = false;
try { await flowBrowser.createFlowBrowserRuntime({ source: { ...sharedBridge.exports } }); } catch { copiedExports = true; }
equal(copiedExports, true, "copied-export-container-preserves-runtime-identity");
equal(sharedBridge.operations.length, beforeDuplicate, "duplicate-export-owner-before-frame");
const runtimeClose = runtime.close();
equal(runtime.close(), runtimeClose, "runtime-close-retains-exact-promise");
await runtimeClose;
equal(sharedBridge.globalCloseCalls, runtimeLifetime.runtimeClose.globalCloseCalls, "one-global-runtime-close");
equal(runtime.terminalIsEmpty(), runtimeLifetime.runtimeClose.terminal, "runtime-terminal-proof");
let lateAdmission = false;
try { runtime.openSession(); } catch { lateAdmission = true; }
equal(lateAdmission, true, "runtime-closing-refuses-new-session");
console.log("[DEBUG] Flow browser runtime isolated two sessions, acknowledged exact session retirement, refused duplicate exports, and closed its bridge exactly once");
const lateBridge = new MockFlowBridge(memory);
const lateRuntime = await createFlowBrowserRuntime({ source: lateBridge.exports });
const lateSession = lateRuntime.openSession();
const refusedTask = lateSession.selectedWidgetIds();
const lateClose = lateSession.close();
let cancelledBeforeOpen = false;
try { await refusedTask.result; } catch { cancelledBeforeOpen = true; }
equal(cancelledBeforeOpen, true, "closing-before-open-reply-refuses-queued-feature");
const lateSibling = lateRuntime.openSession();
await Promise.all([lateClose, lateSibling.selectedWidgetIds().result]);
deepStrictEqual(lateBridge.closedSessionSlots, [1]);
equal(lateBridge.globalCloseCalls, 0, "late-open-close-preserves-sibling");
await lateRuntime.close();
deepStrictEqual(lateBridge.closedSessionSlots, [1, 2]);
const receiptBridge = new MockFlowBridge(memory, { rejectSessionReceiptAcks: runtimeLifetime.backpressure.receiptAckRejections, rejectSessionCloseControls: runtimeLifetime.backpressure.closeControlRejections });
const receiptRuntime = await createFlowBrowserRuntime({ source: receiptBridge.exports });
const receiptSession = receiptRuntime.openSession();
await receiptSession.selectedWidgetIds().result;
await receiptSession.close();
equal(receiptBridge.sessionReceiptAckAttempts, runtimeLifetime.backpressure.receiptAckRejections + 1, "exact-session-receipt-ack-retried");
equal(receiptBridge.sessionCloseControlAttempts, runtimeLifetime.backpressure.closeControlRejections + 1, "exact-session-close-control-retried");
deepStrictEqual(receiptBridge.closedSessionSlots, [1]);
await receiptRuntime.close();
console.log("[DEBUG] Flow late open retired only its cancelled owner; session retirement waited for the third exact receipt ACK without resending close");

await testFlowOpenOwnership(runtimeLifetime.openFailure);
const bridge = new MockFlowBridge(memory);
const host = createFlowHost({ exports: bridge.exports, memory });
const features = await createFlowFeatures(host);

const browserBridge = new MockFlowBridge(memory);
let browserInstantiationAttempted = false;
const browser = await createFlowBrowserRuntime({
  source: browserBridge.exports,
  instantiate: async () => {
    browserInstantiationAttempted = true;
    throw new Error("preinitialized Flow exports must not be instantiated again");
  },
});
equal(browserInstantiationAttempted, false, "preinitialized-browser-exports");
await browser.close();

let foreignRejected = false;
try { await createFlowBrowserRuntime({ source: new Uint8Array(), imports: { foreign: {} } }); } catch (error) { foreignRejected = error.message === "custom Flow imports require their exact embedding initializer"; }
equal(foreignRejected, true, "generated-loader-rejects-foreign-imports");
const customBridge = new MockFlowBridge(memory);
const foreignImports = { foreign: { identity: 7 } };
const custom = await createFlowBrowserRuntime({ source: new Uint8Array(), imports: foreignImports, instantiate: async (_bytes, imports) => {
  equal(imports, foreignImports, "custom-loader-exact-import-owner");
  return { instance: { exports: customBridge.exports } };
} });
await custom.close();

for (const provenTerminal of [sessionClose.browser.terminalOnClosingPoll, false]) {
  const boundary = new MockFlowBridge(memory);
  let closing = false;
  let terminal = false;
  const owner = await createFlowBrowserRuntime({ source: {
    ...boundary.exports,
    flow_bridge_begin_close() { closing = true; },
    flow_bridge_poll(...args) { if (!closing) return boundary.exports.flow_bridge_poll(...args); terminal = provenTerminal; return -1; },
    flow_bridge_terminal_is_empty() { return Number(terminal); },
  } });
  let accepted = false;
  try { await owner.close(); accepted = true; } catch (error) { equal(error.message, "Flow closed before terminal-empty", "unproven-close-error"); }
  equal(accepted, provenTerminal, "closing-poll-exact-terminal-witness");
  equal(owner.terminalIsEmpty(), provenTerminal, "closing-poll-retained-terminal-state");
}
console.log("[DEBUG] Flow close poll terminal witness: exact-terminal accepted, unproven-terminal rejected");

const yieldingBridge = new MockFlowBridge(memory);
let yieldingClose = false;
let closePolls = 0;
const yieldingOwner = await createFlowBrowserRuntime({ source: {
  ...yieldingBridge.exports,
  flow_bridge_begin_close() { yieldingClose = true; },
  flow_bridge_poll(...args) { if (!yieldingClose) return yieldingBridge.exports.flow_bridge_poll(...args); closePolls += 1; return closePolls < sessionClose.browser.pendingClosePolls ? 0 : -1; },
  flow_bridge_terminal_is_empty() { return Number(yieldingClose && closePolls >= sessionClose.browser.pendingClosePolls); },
} });
let closeCompleted = false;
let eventObserved = false;
const pendingClose = yieldingOwner.close().then(() => { closeCompleted = true; });
const externalEvent = new Promise((resolve) => setTimeout(() => { eventObserved = !closeCompleted; resolve(); }, 0));
await Promise.all([pendingClose, externalEvent]);
equal(eventObserved, sessionClose.browser.yieldsToEvents, "pending-close-yields-to-user-events");
equal(closePolls, sessionClose.browser.pendingClosePolls, "bounded-close-poll-count");
console.log("[DEBUG] Flow retained close yielded to an external event before its four bounded poll turns completed");

const integrated = await createFlowBrowserRuntime({ source: await readFile(new URL("../../../🫀️core/🕸️bindings/flow_core_bg.wasm", import.meta.url)) });
const integratedSession = integrated.openSession();
const integratedSibling = integrated.openSession();
const integratedCatalogue = await integratedSession.catalogueJson().result;
equal(integratedCatalogue !== undefined, true, "compiled-flow-bridge");
const integratedBurst = await Promise.all(Array.from({ length: sessionClose.browser.requestBurst }, () => integratedSession.catalogueJson().result));
equal(integratedBurst.every((catalogue) => catalogue !== undefined), true, "compiled-flow-bridge-burst");
equal(integrated.terminalIsEmpty(), sessionClose.browser.terminalBeforeClose, "compiled-session-retained-before-close");
await integratedSession.close();
await integratedSibling.catalogueJson().result;
await integrated.close();
equal(integrated.terminalIsEmpty(), sessionClose.browser.terminalAfterClose, "compiled-session-terminal-after-close");
console.log("[DEBUG] Flow compiled session close drained its real domain after %d completed requests and reached terminal-empty", sessionClose.browser.requestBurst + 1);
console.log("[DEBUG] Flow browser startup preserved four exact initializer/import ownership cases against the compiled module");

const fixtureTask = features.document.catalogueJson({});
const events = [];
fixtureTask.subscribe((event) => events.push(event.event));
equal(typeof await fixtureTask.result, "object", "real-domain-output");
for (const code of [2_650, 2_651, 2_652, 2_653, 2_656]) if (!events.includes(code)) throw new Error(`reactive event ${code} missing`);

const gpu = { requestAdapter: async () => ({ requestDevice: async () => ({ lost: new Promise(() => {}) }) }) };
const attached = attachFlowSurface(features, {}, { width: 800, height: 600, dpr: 2, gpu });
const attachedSurface = await attached.result;
equal(attachedSurface.surfaceGeneration, 1, "surface-generation");
equal(bridge.operations.includes(FlowOperation.surfaceStatus), true, "async-surface-status");
await features.surface.surfaceStatus({ surface: attachedSurface.surface, surfaceGeneration: attachedSurface.surfaceGeneration, status: "cancelled" }).result;

let releaseAdapter;
const interruptedAttach = attachFlowSurface(features, {}, { width: 1, height: 1, gpu: { requestAdapter: () => new Promise((resolve) => { releaseAdapter = resolve; }) } });
while (!releaseAdapter) await new Promise((resolve) => setTimeout(resolve, 0));
equal(interruptedAttach.cancel(), true, "cancel-gpu-create");
releaseAdapter({ requestDevice: async () => ({ lost: new Promise(() => {}) }) });
let attachCancelled = false;
try { await interruptedAttach.result; } catch { attachCancelled = true; }
equal(attachCancelled, true, "cancelled-gpu-terminal");

const before = host.state.nextRequest;
let oversized = false;
try { await host.start(FlowOperation.setCatalogueJson, { json: "x".repeat(FLOW_MAX_REQUEST_BYTES + 1) }, features.lifetime.session).result; } catch { oversized = true; }
equal(oversized, true, "request-max-plus-one");
equal(host.state.nextRequest, before, "preflight-before-request-credit");

let numericPlusOne = false;
try { await features.surface.attachSurface({ surface: 4_294_967_296, surfaceGeneration: 1, width: 1, height: 1, dpr: 1 }).result; } catch { numericPlusOne = true; }
equal(numericPlusOne, true, "numeric-max-plus-one");
equal(host.state.nextRequest, before, "numeric-preflight-before-request-credit");

let malformed = false;
try { decodeFlowMessage(Uint8Array.of(1, 4, 0)); } catch { malformed = true; }
equal(malformed, true, "malformed-page");

const hostileBridge = new MockFlowBridge(memory, { hold: FlowOperation.catalogueJson, rejectControls: 9 });
const hostileHost = createFlowHost({ exports: hostileBridge.exports, memory });
const hostileFeatures = await createFlowFeatures(hostileHost);
const held = hostileFeatures.document.catalogueJson({});
for (let attempt = 0; attempt < 9; attempt += 1) {
  let rejected = false;
  try { held.cancel(); } catch { rejected = true; }
  equal(rejected, true, "rejected-control");
}
equal(held.cancel(), true, "valid-control-after-rejections");
let cancelled = false;
try { await held.result; } catch (error) { cancelled = error.message === "cancelled"; }
equal(cancelled, true, "cancel-terminal");
await hostileHost.close();

await features.lifetime.close();
await host.close();
equal(host.terminalIsEmpty(), true, "terminal-empty");
console.log(JSON.stringify({ reactive: "progress-cancel", surface: "generation-status", controls: "nine-rejected-then-valid", bytes: "max-plus-one", terminal: "empty" }));
