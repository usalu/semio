import { BROWSER_HOST_INITIAL_POLL_BYTES, BROWSER_HOST_MAX_ENCODED_EVENT_BYTES, BROWSER_HOST_MAX_EVENT_BODY_BYTES, createBrowserHostImports } from "./🟨️browser-host.js";

const equal = (actual, expected, law) => {
  if (actual !== expected) throw new Error(`${law}: ${actual} !== ${expected}`);
};

class Target {
  constructor() { this.listeners = new Map(); }
  addEventListener(kind, listener) { this.listeners.set(kind, listener); }
  removeEventListener(kind) { this.listeners.delete(kind); }
}

class Canvas extends Target {
  constructor() {
    super(); this.clientWidth = 100; this.clientHeight = 50; this.tabIndex = -1; this.style = {}; this.attributes = new Map();
  }
  setAttribute(name, value) { this.attributes.set(name, value); }
  focus() {}
}

class Observer {
  constructor(callback) { this.callback = callback; }
  observe() {}
  disconnect() { this.disconnected = true; }
}

const canvas = new Canvas();
const documentTarget = new Target();
documentTarget.hidden = false;
documentTarget.querySelector = () => canvas;
const windowTarget = new Target();
windowTarget.document = documentTarget;
windowTarget.devicePixelRatio = 2;
globalThis.window = windowTarget;
globalThis.document = documentTarget;
globalThis.navigator = { clipboard: { readText: async () => "paste", writeText: async () => undefined } };
globalThis.ResizeObserver = Observer;
let frameCallback;
globalThis.requestAnimationFrame = (callback) => { frameCallback = callback; return 1; };
globalThis.cancelAnimationFrame = () => undefined;

const memory = new WebAssembly.Memory({ initial: 2 });
const host = createBrowserHostImports({ resolveCanvas: () => canvas, accessibleLabel: "Diagram" });
host.bindMemory(memory);
const bridge = host.imports.semio_browser_host;

const send = (bytes, expected = 1) => {
  new Uint8Array(memory.buffer, 0, bytes.length).set(bytes);
  equal(bridge.send(0, bytes.length), expected, "send-admitted");
};
const poll = () => {
  const length = bridge.poll(4096, 1024);
  return length > 0 ? new Uint8Array(memory.buffer, 4096, length).slice() : undefined;
};

send(request(1793, 1n, 1, canvasBody(1)));
equal(poll()[1], 2, "attach-reply");
equal(canvas.attributes.get("role"), "application", "accessible-role");
equal(canvas.attributes.get("aria-label"), "Diagram", "accessible-label");
poll(); poll();

canvas.listeners.get("pointermove")({ pointerId: 7, pointerType: "mouse", pressure: 0, tiltX: 0, tiltY: 0, offsetX: 10, offsetY: 20, button: 0 });
canvas.listeners.get("pointermove")({ pointerId: 7, pointerType: "mouse", pressure: 0, tiltX: 0, tiltY: 0, offsetX: 11, offsetY: 21, button: 0 });
const pointer = poll();
equal(new DataView(pointer.buffer, pointer.byteOffset + pointer.length - 10, 4).getFloat32(0, true), 11, "pointer-latest-wins");
const pointerView = new DataView(pointer.buffer, pointer.byteOffset, pointer.byteLength);
const pointerAck = replyAck(pointerView.getBigUint64(2, true), pointerView.getUint32(10, true));
send(pointerAck);
send(pointerAck, 0);

send(request(1794, 2n, 1, canvasBody(1)));
equal(poll()[1], 2, "frame-ack");
frameCallback(16);
const frame = poll();
equal(new DataView(frame.buffer, frame.byteOffset + 18, 2).getUint16(0, true), 1803, "frame-event");

globalThis.navigator.clipboard.readText = async () => { throw new Error("denied"); };
send(request(1796, 3n, 1, canvasBody(1)));
await Promise.resolve(); await Promise.resolve();
const rejected = poll();
equal(new DataView(rejected.buffer, rejected.byteOffset + 14, 2).getUint16(0, true), 4, "clipboard-rejected");

canvas.listeners.get("pointermove")({ get pointerId() { host.close(); return 7; }, pointerType: "mouse", pressure: 0, tiltX: 0, tiltY: 0, offsetX: 1, offsetY: 2, button: 0 });
equal(canvas.listeners.size, 0, "close-during-callback-release");

const cappedCanvas = new Canvas();
const cappedMemory = new WebAssembly.Memory({ initial: 2 });
const cappedHost = createBrowserHostImports({ resolveCanvas: () => cappedCanvas, accessibleLabel: "Diagram", maximumCritical: 1 });
cappedHost.bindMemory(cappedMemory);
const cappedBridge = cappedHost.imports.semio_browser_host;
const attachBytes = request(1793, 9n, 1, canvasBody(9));
new Uint8Array(cappedMemory.buffer, 0, attachBytes.length).set(attachBytes);
equal(cappedBridge.send(0, attachBytes.length), 1, "critical-max-attach");
cappedCanvas.listeners.get("pointerdown")({ pointerId: 1, pointerType: "mouse", pressure: 0, tiltX: 0, tiltY: 0, offsetX: 0, offsetY: 0, button: 0 });
equal(cappedCanvas.listeners.size, 0, "critical-max-plus-one-closes");
const retainedLength = cappedBridge.poll(4096, 1);
equal(retainedLength > 1, true, "critical-capacity-reported");
equal(cappedBridge.poll(4096, 1024), retainedLength, "critical-max-retained");
equal(cappedBridge.poll(4096, 1024), -1, "critical-terminal-empty");

const latestCanvas = new Canvas();
const latestMemory = new WebAssembly.Memory({ initial: 2 });
const latestHost = createBrowserHostImports({ resolveCanvas: () => latestCanvas, accessibleLabel: "Diagram", maximumLatest: 1 });
latestHost.bindMemory(latestMemory);
const latestBridge = latestHost.imports.semio_browser_host;
const latestAttach = request(1793, 10n, 1, canvasBody(10));
new Uint8Array(latestMemory.buffer, 0, latestAttach.length).set(latestAttach);
equal(latestBridge.send(0, latestAttach.length), 1, "latest-max-attach");
equal(latestCanvas.listeners.size, 0, "latest-max-plus-one-closes");
equal(latestBridge.poll(4096, 1024) > 0, true, "latest-max-retained-reply");
equal(latestBridge.poll(4096, 1024), -1, "latest-terminal-empty");

const exactCanvas = new Canvas();
const exactMemory = new WebAssembly.Memory({ initial: 2 });
const exactHost = createBrowserHostImports({ resolveCanvas: () => exactCanvas, accessibleLabel: "Diagram" });
exactHost.bindMemory(exactMemory);
const exactBridge = exactHost.imports.semio_browser_host;
sendTo(exactBridge, exactMemory, request(1793, 11n, 1, canvasBody(11)));
drain(exactBridge);
exactCanvas.listeners.get("beforeinput")({ data: "x".repeat(1009) });
const exactTarget = new Uint8Array(exactMemory.buffer, 8192, BROWSER_HOST_MAX_ENCODED_EVENT_BYTES);
exactTarget.fill(0xa5);
equal(exactBridge.poll(8192, BROWSER_HOST_INITIAL_POLL_BYTES), BROWSER_HOST_MAX_ENCODED_EVENT_BYTES, "encoded-max-required");
equal(exactTarget.every((value) => value === 0xa5), true, "undersized-poll-no-copy");
equal(exactBridge.poll(8192, BROWSER_HOST_MAX_ENCODED_EVENT_BYTES), BROWSER_HOST_MAX_ENCODED_EVENT_BYTES, "encoded-max-exact-retry");
const exactView = new DataView(exactMemory.buffer, 8192, BROWSER_HOST_MAX_ENCODED_EVENT_BYTES);
equal(exactView.getUint32(23, true), BROWSER_HOST_MAX_EVENT_BODY_BYTES, "event-body-exact-max");
sendTo(exactBridge, exactMemory, replyAck(exactView.getBigUint64(2, true), exactView.getUint32(10, true)));

const cancelledCanvas = new Canvas();
const cancelledMemory = new WebAssembly.Memory({ initial: 2 });
const cancelledHost = createBrowserHostImports({ resolveCanvas: () => cancelledCanvas, accessibleLabel: "Diagram" });
cancelledHost.bindMemory(cancelledMemory);
const cancelledBridge = cancelledHost.imports.semio_browser_host;
sendTo(cancelledBridge, cancelledMemory, request(1793, 12n, 1, canvasBody(12)));
drain(cancelledBridge);
globalThis.navigator.clipboard.readText = async () => "y".repeat(1030);
sendTo(cancelledBridge, cancelledMemory, request(1796, 13n, 1, canvasBody(12)));
await Promise.resolve(); await Promise.resolve();
equal(cancelledBridge.poll(8192, BROWSER_HOST_INITIAL_POLL_BYTES), BROWSER_HOST_MAX_ENCODED_EVENT_BYTES, "cancel-retained-required");
sendTo(cancelledBridge, cancelledMemory, controlCancel(13n, 1));
equal(cancelledBridge.poll(8192, BROWSER_HOST_MAX_ENCODED_EVENT_BYTES), 0, "cancel-between-polls");

const closedCanvas = new Canvas();
const closedMemory = new WebAssembly.Memory({ initial: 2 });
const closedHost = createBrowserHostImports({ resolveCanvas: () => closedCanvas, accessibleLabel: "Diagram" });
closedHost.bindMemory(closedMemory);
const closedBridge = closedHost.imports.semio_browser_host;
sendTo(closedBridge, closedMemory, request(1793, 14n, 1, canvasBody(14)));
drain(closedBridge);
closedCanvas.listeners.get("beforeinput")({ data: "z".repeat(1009) });
equal(closedBridge.poll(8192, BROWSER_HOST_INITIAL_POLL_BYTES), BROWSER_HOST_MAX_ENCODED_EVENT_BYTES, "close-retained-required");
closedHost.close();
equal(closedBridge.poll(8192, BROWSER_HOST_MAX_ENCODED_EVENT_BYTES), -1, "close-between-polls");

const oversizedCanvas = new Canvas();
const oversizedMemory = new WebAssembly.Memory({ initial: 2 });
const oversizedHost = createBrowserHostImports({ resolveCanvas: () => oversizedCanvas, accessibleLabel: "Diagram" });
oversizedHost.bindMemory(oversizedMemory);
const oversizedBridge = oversizedHost.imports.semio_browser_host;
sendTo(oversizedBridge, oversizedMemory, request(1793, 15n, 1, canvasBody(15)));
drain(oversizedBridge);
const oversizedTarget = new Uint8Array(oversizedMemory.buffer, 8192, BROWSER_HOST_MAX_ENCODED_EVENT_BYTES + 1);
oversizedTarget.fill(0x5a);
oversizedCanvas.listeners.get("beforeinput")({ data: "q".repeat(1010) });
equal(oversizedBridge.poll(8192, BROWSER_HOST_MAX_ENCODED_EVENT_BYTES + 1), -1, "event-body-max-plus-one-rejected");
equal(oversizedTarget.every((value) => value === 0x5a), true, "max-plus-one-rejected-before-copy");

function request(operation, requestId, generation, body) {
  const writer = new Writer(); writer.u8(1); writer.u8(1); writer.u16(operation); writer.u64(requestId); writer.u32(generation); writer.bytes(body); return writer.finish();
}

function canvasBody(canvasId) {
  const writer = new Writer(); writer.u8(1); writer.u32(canvasId); return writer.finish();
}

function controlClose(slot, generation) {
  const writer = new Writer(); writer.u8(1); writer.u8(5); writer.u8(2); writer.u32(slot); writer.u32(generation); return writer.finish();
}

function controlCancel(requestId, generation) {
  const writer = new Writer(); writer.u8(1); writer.u8(5); writer.u8(1); writer.u64(requestId); writer.u32(generation); return writer.finish();
}

function replyAck(requestId, generation) {
  const writer = new Writer(); writer.u8(1); writer.u8(2); writer.u64(requestId); writer.u32(generation); writer.u16(0); writer.u8(0); writer.bytes(new Uint8Array()); return writer.finish();
}

function sendTo(targetBridge, targetMemory, bytes, expected = 1) {
  new Uint8Array(targetMemory.buffer, 0, bytes.length).set(bytes);
  equal(targetBridge.send(0, bytes.length), expected, "target-send-admitted");
}

function drain(targetBridge) {
  while (targetBridge.poll(4096, BROWSER_HOST_MAX_ENCODED_EVENT_BYTES) > 0) {}
}

class Writer {
  constructor() { this.values = []; }
  u8(value) { this.values.push(value & 255); }
  u16(value) { this.number(2, (view) => view.setUint16(0, value, true)); }
  u32(value) { this.number(4, (view) => view.setUint32(0, value, true)); }
  u64(value) { this.number(8, (view) => view.setBigUint64(0, value, true)); }
  bytes(value) { this.u32(value.length); this.values.push(...value); }
  number(length, write) { const bytes = new Uint8Array(length); write(new DataView(bytes.buffer)); this.values.push(...bytes); }
  finish() { return Uint8Array.from(this.values); }
}

console.log(JSON.stringify({ attach: "ok", accessibility: "ok", pointerStorm: "latest", frame: "one", clipboardRejected: "owned", closeDuringCallback: "empty", criticalMaxPlusOne: "closed", latestMaxPlusOne: "closed", encodedMax: BROWSER_HOST_MAX_ENCODED_EVENT_BYTES, retainedRetry: "exact", cancelBetweenPolls: "empty", closeBetweenPolls: "closed", acknowledgement: "once", bodyMaxPlusOne: "rejected-before-copy" }));
