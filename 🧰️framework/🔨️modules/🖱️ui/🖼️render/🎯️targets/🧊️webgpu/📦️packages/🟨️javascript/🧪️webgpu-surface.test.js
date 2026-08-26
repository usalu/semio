import { GPU_MAX_FRAME_BYTES, GPU_MAX_IN_FLIGHT_CONTROLS, GPU_MAX_IN_FLIGHT_FRAMES, GPU_MAX_IN_FLIGHT_PAGES, GPU_MAX_SURFACE_SESSIONS, createBrowserWebGpuImports, createWebGpuSurfacePort } from "./🟨️webgpu-surface.js";

class Canvas {
  constructor() { this.attributes = new Map(); this.width = 0; this.height = 0; this.listeners = new Map(); this.style = {}; this.tabIndex = 0; }
  setAttribute(key, value) { this.attributes.set(key, value); }
  getAttribute(key) { return this.attributes.get(key); }
  removeAttribute(key) { this.attributes.delete(key); }
  addEventListener(kind, callback) { this.listeners.set(kind, callback); }
  removeEventListener(kind) { this.listeners.delete(kind); }
  focus() {}
}

const equal = (actual, expected, label) => { if (actual !== expected) throw new Error(`${label}: ${actual} !== ${expected}`); };
const memory = new WebAssembly.Memory({ initial: 2 });
const canvas = new Canvas();
const host = createBrowserWebGpuImports({ resolveCanvas: (id) => id === 1 ? canvas : undefined, accessibleLabel: "Diagram", adapterSupported: true });
host.bindMemory(memory);
equal(Boolean(host.imports.semio_browser_host), true, "composes-a2-port");
equal(Boolean(host.imports.semio_webgpu_surface), true, "composes-a3-port");

const port = host.surfacePort;
const create = port.create({ surfaceId: 1, canvasId: 1, width: 640, height: 480 });
equal(create.accepted, true, "create-admitted");
const createRequest = poll(port, memory);
equal(createRequest[4], 1, "create-request-generation");
sendOutcome(port, memory, page(1, 1, 0, outcome(1, create.requestId, 1, 1, (writer) => writer.u32(640).u32(480).f32(1))));
equal(canvas.getAttribute("data-raw-handle"), "1", "owned-raw-handle-registration");

const resize = port.resize({ surfaceId: 1, generation: 1, width: 800, height: 600, scaleFactor: 2 });
poll(port, memory);
sendOutcome(port, memory, page(1, 1, 1, outcome(2, resize.requestId, 1, 1, (writer) => writer.u32(800).u32(600).f32(2))));
equal(canvas.width, 800, "resize-width"); equal(canvas.height, 600, "resize-height");

const frame = port.frame({ surfaceId: 1, generation: 1, frameId: 7n, bytes: Uint8Array.of(1, 2, 3) });
equal(frame.accepted, true, "frame-admitted"); poll(port, memory);
sendOutcome(port, memory, page(1, 1, 2, outcome(3, frame.requestId, 1, 1, (writer) => writer.u64(7n).u32(3))));
equal(port.state.frames, 1, "frame-retained-until-ack");
equal(port.acknowledge({ slot: 1, generation: 1 }, 2).accepted, true, "ack-admitted"); poll(port, memory);
equal(port.state.frames, 0, "frame-release-on-ack");

const oversized = new Uint8Array(GPU_MAX_FRAME_BYTES + 1);
const oversizedResult = port.frame({ surfaceId: 1, generation: 1, frameId: 8n, bytes: oversized });
equal(oversizedResult.accepted, false, "max-plus-one-frame-rejected"); equal(oversizedResult.bytes, oversized, "exact-frame-handback");

const frameOwners = [];
for (let index = 0; index < GPU_MAX_IN_FLIGHT_FRAMES; index += 1) frameOwners.push(port.frame({ surfaceId: 1, generation: 1, frameId: BigInt(10 + index), bytes: new Uint8Array() }));
equal(port.frame({ surfaceId: 1, generation: 1, frameId: 99n, bytes: new Uint8Array() }).accepted, false, "max-plus-one-in-flight-frame");
while (port.state.queue.length) poll(port, memory);

const missing = port.create({ surfaceId: 2, canvasId: 2, width: 10, height: 10 });
equal(missing.accepted, true, "missing-canvas-envelope");
const missingBytes = poll(port, memory); equal(missingBytes[20 + 1], 1, "missing-canvas-status");

const callbackPort = createWebGpuSurfacePort({ resolveCanvas: () => canvas, adapterSupported: true, onOutcome() { throw new Error("interrupt"); } });
callbackPort.bindMemory(memory); const interrupted = callbackPort.create({ surfaceId: 3, canvasId: 3 }); poll(callbackPort, memory);
equal(sendOutcome(callbackPort, memory, page(3, 1, 0, outcome(1, interrupted.requestId, 3, 1, (writer) => writer.u32(1).u32(1).f32(1)))), 11, "interrupted-callback");
equal(callbackPort.state.pages.length, 0, "interruption-retains-no-partial-page");
const clock = [0, 8];
const deadlinePort = createWebGpuSurfacePort({ resolveCanvas: () => canvas, adapterSupported: true, now: () => clock.shift() }); deadlinePort.bindMemory(memory);
const deadline = deadlinePort.create({ surfaceId: 4, canvasId: 4 }); poll(deadlinePort, memory);
equal(sendOutcome(deadlinePort, memory, page(4, 1, 0, outcome(1, deadline.requestId, 4, 1, (writer) => writer.u32(1).u32(1).f32(1)))), 17, "eight-millisecond-watchdog");
equal(deadlinePort.state.sessions.size, 0, "deadline-before-host-state-mutation");

const unsupportedPort = createWebGpuSurfacePort({ resolveCanvas: () => canvas, adapterSupported: false }); unsupportedPort.bindMemory(memory);
unsupportedPort.create({ surfaceId: 5, canvasId: 5 }); const unsupportedRequest = poll(unsupportedPort, memory);
equal(unsupportedRequest[21], 3, "unsupported-adapter-status");
const badPort = createWebGpuSurfacePort({ resolveCanvas: () => ({}) , adapterSupported: true }); badPort.bindMemory(memory);
badPort.create({ surfaceId: 6, canvasId: 6 }); const badRequest = poll(badPort, memory);
equal(badRequest[21], 2, "bad-canvas-status");

const pagePort = createWebGpuSurfacePort({ resolveCanvas: () => canvas, adapterSupported: true }); pagePort.bindMemory(memory);
for (let index = 0; index < GPU_MAX_IN_FLIGHT_PAGES; index += 1) sendOutcome(pagePort, memory, page(1, 1, index, outcome(8, BigInt(index + 1), 1, 1, (writer) => writer.u16(9))));
equal(sendOutcome(pagePort, memory, page(1, 1, GPU_MAX_IN_FLIGHT_PAGES, outcome(8, 99n, 1, 1, (writer) => writer.u16(9)))), 19, "max-plus-one-page");
equal(pagePort.resize({ surfaceId: 1, generation: 1, width: 1, height: 1 }).accepted, false, "page-cap-preflights-before-request-owner");

const pendingPagePort = createWebGpuSurfacePort({ resolveCanvas: () => canvas, adapterSupported: true }); pendingPagePort.bindMemory(memory);
for (let index = 0; index < GPU_MAX_IN_FLIGHT_PAGES; index += 1) equal(pendingPagePort.resize({ surfaceId: 1, generation: 1, width: index, height: 1 }).accepted, true, `pending-page-${index}`);
equal(pendingPagePort.resize({ surfaceId: 1, generation: 1, width: 9, height: 1 }).accepted, false, "pending-page-max-plus-one");

const sessionPort = createWebGpuSurfacePort({ resolveCanvas: () => new Canvas(), adapterSupported: true }); sessionPort.bindMemory(memory);
for (let id = 1; id <= GPU_MAX_SURFACE_SESSIONS; id += 1) sessionPort.state.sessions.set(id, { canvas: new Canvas() });
equal(sessionPort.create({ surfaceId: 9, canvasId: 9 }).accepted, false, "max-plus-one-session");

const controlBoundaryHost = createBrowserWebGpuImports({ maximumQueue: GPU_MAX_IN_FLIGHT_CONTROLS + 1 }); controlBoundaryHost.bindMemory(memory);
for (let index = 0; index < GPU_MAX_IN_FLIGHT_CONTROLS; index += 1) equal(controlBoundaryHost.surfacePort.cancel(BigInt(index + 1), 1).accepted, true, `control-boundary-${index}`);
equal(controlBoundaryHost.surfacePort.cancel(9n, 1).accepted, false, "control-max-plus-one");
equal(controlBoundaryHost.surfacePort.state.controls, GPU_MAX_IN_FLIGHT_CONTROLS, "control-boundary-retained");
while (controlBoundaryHost.surfacePort.state.queue.length) poll(controlBoundaryHost.surfacePort, memory);
equal(controlBoundaryHost.surfacePort.state.controls, 0, "control-boundary-released");
equal(controlBoundaryHost.surfacePort.cancel(10n, 1).accepted, true, "control-valid-after-max-plus-one"); poll(controlBoundaryHost.surfacePort, memory);
equal(controlBoundaryHost.surfacePort.state.controls, 0, "control-valid-after-max-plus-one-released");
controlBoundaryHost.close();

const noQueueHost = createBrowserWebGpuImports({ maximumQueue: 0 }); noQueueHost.bindMemory(memory);
for (let index = 0; index <= GPU_MAX_IN_FLIGHT_CONTROLS; index += 1) equal(noQueueHost.surfacePort.cancel(BigInt(index + 1), 1).accepted, false, `zero-queue-rejected-cancel-${index}`);
equal(noQueueHost.surfacePort.state.controls, 0, "zero-queue-rejections-retain-no-control");
equal(noQueueHost.surfacePort.state.queue.length, 0, "zero-queue-rejections-retain-no-message");
noQueueHost.close();

for (const kind of ["cancel", "acknowledge", "close"]) rejectedControlRecoveryLaw(kind);

const closedHost = createBrowserWebGpuImports({ maximumQueue: 1 }); closedHost.bindMemory(memory); closedHost.close();
for (let index = 0; index <= GPU_MAX_IN_FLIGHT_CONTROLS; index += 1) {
  equal(closedHost.surfacePort.cancel(BigInt(index + 1), 1).accepted, false, `closed-rejected-cancel-${index}`);
  equal(closedHost.surfacePort.acknowledge({ slot: 1, generation: 1 }, index).accepted, false, `closed-rejected-acknowledge-${index}`);
  equal(closedHost.surfacePort.closePage({ slot: 1, generation: 1 }).accepted, false, `closed-rejected-close-${index}`);
}
equal(closedHost.surfacePort.state.controls, 0, "closed-rejections-retain-no-control");
equal(closedHost.surfacePort.state.queue.length, 0, "closed-rejections-retain-no-message");

const lifecycleCanvas = new Canvas();
const lifecycle = createWebGpuSurfacePort({ resolveCanvas: () => lifecycleCanvas, adapterSupported: true }); lifecycle.bindMemory(memory);
const lifecycleCreate = lifecycle.create({ surfaceId: 7, canvasId: 7, generation: 1, width: 320, height: 200 }); poll(lifecycle, memory);
sendOutcome(lifecycle, memory, page(7, 1, 0, outcome(1, lifecycleCreate.requestId, 7, 1, (writer) => writer.u32(320).u32(200).f32(1))));
const loss = lifecycle.lose({ surfaceId: 7, generation: 1, reason: 2 }); poll(lifecycle, memory);
sendOutcome(lifecycle, memory, page(7, 1, 1, outcome(4, loss.requestId, 7, 1, (writer) => writer.u8(2))));
equal(lifecycle.takeOutcome().outcome.tag, 1, "last-valid-surface-retained-during-loss");
const recovery = lifecycle.recover({ surfaceId: 7, generation: 1 }); poll(lifecycle, memory);
sendOutcome(lifecycle, memory, page(7, 2, 2, outcome(5, recovery.requestId, 7, 2, (writer) => writer.u32(1).u32(320).u32(200).f32(1))));
equal(lifecycle.state.sessions.get(7).generation, 2, "deterministic-recovery-generation");
const stale = lifecycle.frame({ surfaceId: 7, generation: 1, frameId: 9n, bytes: new Uint8Array() }); poll(lifecycle, memory);
sendOutcome(lifecycle, memory, page(7, 1, 3, outcome(8, stale.requestId, 7, 1, (writer) => writer.u16(6))));
equal(lifecycle.state.frames, 0, "stale-frame-owner-released");
const dropped = lifecycle.drop({ surfaceId: 7, generation: 2 }); poll(lifecycle, memory);
sendOutcome(lifecycle, memory, page(7, 2, 4, outcome(6, dropped.requestId, 7, 2)));
equal(lifecycleCanvas.getAttribute("data-raw-handle"), undefined, "drop-removes-raw-handle");

port.close(); equal(canvas.getAttribute("data-raw-handle"), undefined, "exact-close-removes-handle");
host.close();
console.log(JSON.stringify({ a2Composition: true, surfaceTrace: "create-resize-frame-drop", maxSessions: GPU_MAX_SURFACE_SESSIONS, maxFrames: GPU_MAX_IN_FLIGHT_FRAMES, maxPages: GPU_MAX_IN_FLIGHT_PAGES, maxControls: GPU_MAX_IN_FLIGHT_CONTROLS, rejectedControlRecovery: ["cancel", "acknowledge", "close"], callbackMilliseconds: 8 }));

function rejectedControlRecoveryLaw(kind) {
  const lawHost = createBrowserWebGpuImports({ maximumQueue: 1 }); lawHost.bindMemory(memory);
  const lawPort = lawHost.surfacePort;
  const slot = kind === "cancel" ? 20 : kind === "acknowledge" ? 21 : 22;
  const retainedFrame = lawPort.frame({ surfaceId: 1, generation: 1, frameId: BigInt(slot), bytes: Uint8Array.of(slot) });
  equal(retainedFrame.accepted, true, `${kind}-retained-frame-admitted`); poll(lawPort, memory);
  sendOutcome(lawPort, memory, page(slot, 1, 0, outcome(3, retainedFrame.requestId, 1, 1, (writer) => writer.u64(BigInt(slot)).u32(1))));
  equal(lawPort.state.frames, 1, `${kind}-frame-retained-before-control`);
  equal(lawPort.state.pages.length, 1, `${kind}-page-retained-before-control`);
  equal(lawPort.resize({ surfaceId: 1, generation: 1, width: 1, height: 1 }).accepted, true, `${kind}-queue-saturated`);
  const submit = () => kind === "cancel"
    ? lawPort.cancel(retainedFrame.requestId, 1)
    : kind === "acknowledge"
      ? lawPort.acknowledge({ slot, generation: 1 }, 0)
      : lawPort.closePage({ slot, generation: 1 });
  for (let index = 0; index <= GPU_MAX_IN_FLIGHT_CONTROLS; index += 1) equal(submit().accepted, false, `${kind}-queue-full-rejection-${index}`);
  equal(lawPort.state.controls, 0, `${kind}-queue-full-rejections-retain-no-control`);
  equal(lawPort.state.frames, 1, `${kind}-rejections-preserve-frame-owner`);
  equal(lawPort.state.pages.length, 1, `${kind}-rejections-preserve-page-owner`);
  poll(lawPort, memory);
  equal(submit().accepted, true, `${kind}-valid-after-rejections`);
  equal(lawPort.state.controls, 1, `${kind}-valid-control-retained`); poll(lawPort, memory);
  equal(lawPort.state.controls, 0, `${kind}-valid-control-released`);
  equal(lawPort.state.frames, 0, `${kind}-valid-control-releases-frame-owner`);
  equal(lawPort.state.pages.length, 0, `${kind}-valid-control-releases-page-owner`);
  lawHost.close();
}

function poll(portValue, memoryValue) {
  const bridge = portValue.imports.semio_webgpu_surface;
  let length = bridge.poll(0, 64); if (length > 64) length = bridge.poll(0, length);
  return new Uint8Array(memoryValue.buffer).slice(0, length);
}
function sendOutcome(portValue, memoryValue, bytes) { new Uint8Array(memoryValue.buffer).set(bytes, 0); return portValue.imports.semio_webgpu_surface.send(0, bytes.length); }
function outcome(tag, requestId, surfaceId, generation, detail) { const writer = new Writer().u8(1).u8(tag).u64(requestId).u32(surfaceId).u32(generation); detail?.(writer); return writer.finish(); }
function page(slot, generation, index, bytes) { return new Writer().u8(1).u8(4).u32(slot).u32(generation).u32(index).bytes(bytes).finish(); }
class Writer {
  constructor() { this.value = []; }
  u8(value) { this.value.push(value & 255); return this; }
  u16(value) { const bytes = new Uint8Array(2); new DataView(bytes.buffer).setUint16(0, value, true); return this.raw(bytes); }
  u32(value) { const bytes = new Uint8Array(4); new DataView(bytes.buffer).setUint32(0, value, true); return this.raw(bytes); }
  u64(value) { const bytes = new Uint8Array(8); new DataView(bytes.buffer).setBigUint64(0, BigInt(value), true); return this.raw(bytes); }
  f32(value) { const bytes = new Uint8Array(4); new DataView(bytes.buffer).setFloat32(0, value, true); return this.raw(bytes); }
  bytes(value) { return this.u32(value.length).raw(value); }
  raw(value) { this.value.push(...value); return this; }
  finish() { return Uint8Array.from(this.value); }
}
