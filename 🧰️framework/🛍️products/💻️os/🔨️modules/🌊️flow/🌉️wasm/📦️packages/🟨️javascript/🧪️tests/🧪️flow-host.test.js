import { FLOW_MAX_REQUEST_BYTES, FlowOperation, attachFlowSurface, createFlowFeatures, createFlowHost, decodeFlowMessage } from "../🟨️flow-host.js";
import { createFlowBrowserFeatures } from "../🟨️flow-browser.js";
import { readFile } from "node:fs/promises";
import Ajv from "ajv";

const equal = (actual, expected, law) => { if (actual !== expected) throw new Error(`${law}: ${actual} !== ${expected}`); };
const startup = JSON.parse(await readFile(new URL("../../../🧪️fixtures/⏱️browser-startup.json", import.meta.url), "utf8"));
const startupSchema = JSON.parse(await readFile(new URL("../../../🧪️fixtures/⏱️browser-startup.schema.json", import.meta.url), "utf8"));
equal(new Ajv({ strict: true }).compile(startupSchema)(startup), true, "startup-schema");
for (const law of startup.cases) equal(law.source === "exports" || law.initializer === "custom" || law.imports === "empty", law.accepted, "startup-independent-admission-oracle");
const encoder = new TextEncoder();
const memory = new WebAssembly.Memory({ initial: 400 });
const bridge = new MockFlowBridge(memory);
const host = createFlowHost({ exports: bridge.exports, memory });
const features = await createFlowFeatures(host);

const browserBridge = new MockFlowBridge(memory);
let browserInstantiationAttempted = false;
const browser = await createFlowBrowserFeatures({
  source: browserBridge.exports,
  instantiate: async () => {
    browserInstantiationAttempted = true;
    throw new Error("preinitialized Flow exports must not be instantiated again");
  },
});
equal(browserInstantiationAttempted, false, "preinitialized-browser-exports");
await browser.features.lifetime.close();

let foreignRejected = false;
try { await createFlowBrowserFeatures({ source: new Uint8Array(), imports: { foreign: {} } }); } catch (error) { foreignRejected = error.message === "custom Flow imports require their exact embedding initializer"; }
equal(foreignRejected, true, "generated-loader-rejects-foreign-imports");
const customBridge = new MockFlowBridge(memory);
const foreignImports = { foreign: { identity: 7 } };
const custom = await createFlowBrowserFeatures({ source: new Uint8Array(), imports: foreignImports, instantiate: async (_bytes, imports) => {
  equal(imports, foreignImports, "custom-loader-exact-import-owner");
  return { instance: { exports: customBridge.exports } };
} });
await custom.features.lifetime.close();

const integrated = await createFlowBrowserFeatures({ source: await readFile(new URL("../../../../🫀️core/pkg/flow_core_bg.wasm", import.meta.url)) });
const integratedCatalogue = await integrated.features.document.catalogueJson({}).result;
equal(integratedCatalogue !== undefined, true, "compiled-flow-bridge");
const integratedBurst = await Promise.all(Array.from({ length: 32 }, () => integrated.features.document.catalogueJson({}).result));
equal(integratedBurst.every((catalogue) => catalogue !== undefined), true, "compiled-flow-bridge-burst");
await integrated.features.lifetime.close();
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
while (!releaseAdapter) await Promise.resolve();
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
equal(host.terminalIsEmpty(), true, "terminal-empty");
console.log(JSON.stringify({ reactive: "progress-cancel", surface: "generation-status", controls: "nine-rejected-then-valid", bytes: "max-plus-one", terminal: "empty" }));

function MockFlowBridge(targetMemory, options = {}) {
  let retained;
  let closing = false;
  let sequence = 1;
  let rejectedControls = options.rejectControls ?? 0;
  const held = new Map();
  const queue = [];
  this.operations = [];
  this.exports = {
    memory: targetMemory,
    flow_bridge_allocate: () => 8,
    flow_bridge_release: () => {},
    flow_bridge_send: (pointer, length, _credit, nowMs, deadlineMs) => {
      equal(typeof nowMs, "bigint", "send-now-u64");
      equal(typeof deadlineMs, "bigint", "send-deadline-u64");
      const frame = new Uint8Array(targetMemory.buffer, pointer, length).slice();
      const tag = frame[1];
      if (tag === 5 && rejectedControls > 0) { rejectedControls -= 1; return -1; }
      if (tag === 1) acceptRequest(frame);
      else if (tag === 5) acceptControl(frame);
      return 1;
    },
    flow_bridge_poll: (pointer, capacity, _credit, nowMs, deadlineMs) => {
      equal(typeof nowMs, "bigint", "poll-now-u64");
      equal(typeof deadlineMs, "bigint", "poll-deadline-u64");
      retained ??= queue.shift();
      if (!retained) return 0;
      if (retained.length > capacity) return retained.length;
      new Uint8Array(targetMemory.buffer, pointer, retained.length).set(retained);
      const length = retained.length;
      retained = undefined;
      return length;
    },
    flow_bridge_begin_close: () => { closing = true; },
    flow_bridge_terminal_is_empty: () => Number(closing && !retained && queue.length === 0),
  };

  const acceptRequest = (frame) => {
    const operation = u16(frame, 2), request = u64(frame, 4), generation = u32(frame, 12);
    this.operations.push(operation);
    if (operation === FlowOperation.open) { queue.push(reply(request, generation, handle(1, 1))); return; }
    const operationHandle = { slot: 2, generation: Number(request) };
    queue.push(event(request, generation, sequence++, 2_650, handle(operationHandle.slot, operationHandle.generation)));
    queue.push(event(request, generation, sequence++, 2_651, new Uint8Array()));
    queue.push(event(request, generation, sequence++, 2_652, new Uint8Array()));
    queue.push(event(request, generation, sequence++, 2_653, new Uint8Array()));
    if (options.hold === operation) { held.set(request, { generation, operationHandle }); return; }
    finish(request, generation, operation, operationHandle);
  };
  const acceptControl = (frame) => {
    if (frame[2] !== 1) return;
    const request = u64(frame, 3), generation = u32(frame, 11);
    const pending = held.get(request);
    if (!pending) return;
    held.delete(request);
    queue.push(event(request, generation, sequence++, 2_656, new Uint8Array()));
    queue.push(failedReply(request, generation, "cancelled"));
  };
  const finish = (request, generation, operation, operationHandle) => {
    const body = text("{}");
    queue.push(event(request, generation, sequence++, 2_656, handle(operationHandle.slot, operationHandle.generation)));
    queue.push(reply(request, generation, body));
  };
}

function text(value) { return encoder.encode(value); }
function handle(slot, generation) { return bytes((w) => { w.u32(slot); w.u32(generation); }); }
function reply(request, generation, body) { return bytes((w) => { w.u8(1); w.u8(2); w.u64(request); w.u32(generation); w.u16(0); w.u8(0); w.body(body); }); }
function failedReply(request, generation, message) {
  const encoded = text(message);
  return bytes((w) => { w.u8(1); w.u8(2); w.u64(request); w.u32(generation); w.u16(1); w.u8(1); w.u16(16); w.u16(encoded.length); w.raw(encoded); w.body(new Uint8Array()); });
}
function event(origin, generation, sequence, code, body) { const acknowledgement = origin ^ (BigInt(sequence) << 32n); return bytes((w) => { w.u8(1); w.u8(3); w.u64(acknowledgement); w.u32(generation); w.u32(sequence); w.u16(code); w.u16(0); w.u8(0); w.body(body); }); }
function bytes(build) {
  const values = [];
  const writer = { u8: (v) => values.push(v), u16: (v) => number(2, (d) => d.setUint16(0, v, true)), u32: (v) => number(4, (d) => d.setUint32(0, v, true)), u64: (v) => number(8, (d) => d.setBigUint64(0, BigInt(v), true)), raw: (v) => values.push(...v), body: (v) => { writer.u32(v.length); values.push(...v); } };
  const number = (length, write) => { const value = new Uint8Array(length); write(new DataView(value.buffer)); values.push(...value); };
  build(writer);
  return Uint8Array.from(values);
}
function u16(value, offset) { return new DataView(value.buffer, value.byteOffset, value.byteLength).getUint16(offset, true); }
function u32(value, offset) { return new DataView(value.buffer, value.byteOffset, value.byteLength).getUint32(offset, true); }
function u64(value, offset) { return new DataView(value.buffer, value.byteOffset, value.byteLength).getBigUint64(offset, true); }
