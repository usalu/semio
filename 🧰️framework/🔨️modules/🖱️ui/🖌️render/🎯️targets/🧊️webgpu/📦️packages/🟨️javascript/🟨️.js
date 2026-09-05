import { createBrowserHostImports } from "../../../../../🖥️host/📦️packages/🟨️javascript/🟨️.js";

export const GPU_MAX_SURFACE_SESSIONS = 8;
export const GPU_MAX_IN_FLIGHT_FRAMES = 4;
export const GPU_MAX_IN_FLIGHT_PAGES = 8;
export const GPU_MAX_IN_FLIGHT_CONTROLS = 8;
export const GPU_MAX_FRAME_BYTES = 4096;
export const GPU_MAX_CALLBACK_MILLISECONDS = 8;

//#region 🔌️SurfacePort

/** 🧊️ A1 byte/page port owning canvas lookup and raw-handle registration beside A2. */
export function createWebGpuSurfacePort(options = {}) {
  const state = { memory: undefined, queue: [], pages: [], pending: new Map(), sessions: new Map(), retained: undefined, closed: false, nextRequest: 1n, frames: 0, controls: 0 };
  const maximumQueue = options.maximumQueue ?? 32;

  const bindMemory = (memory) => { state.memory = memory; };
  const memoryBytes = () => {
    if (!state.memory) throw new Error("surface memory is not bound");
    return new Uint8Array(state.memory.buffer);
  };
  const enqueue = (bytes, delivered) => {
    if (state.closed) return { accepted: false, code: 20 };
    if (state.queue.length === maximumQueue) return { accepted: false, code: 19 };
    state.queue.push({ bytes, delivered });
    return { accepted: true };
  };
  const request = (operation, generation, body, effect) => {
    if (state.pages.length + state.pending.size === GPU_MAX_IN_FLIGHT_PAGES) return { accepted: false, code: 19 };
    const requestId = state.nextRequest++;
    const result = enqueue(encodeRequest(operation, requestId, generation, body));
    if (result.accepted) state.pending.set(requestId, effect);
    return { ...result, requestId };
  };
  const canvasStatus = (canvasId) => {
    const canvas = options.resolveCanvas?.(canvasId);
    if (!canvas) return { status: 1, canvas };
    if (typeof canvas.setAttribute !== "function" || typeof canvas.removeAttribute !== "function") return { status: 2, canvas };
    const supported = options.adapterSupported ?? Boolean(globalThis.navigator?.gpu);
    return { status: supported ? 0 : 3, canvas };
  };
  const create = ({ surfaceId, canvasId = surfaceId, generation = 1, width = 0, height = 0, scaleFactor = 1 }) => {
    if (state.sessions.size + [...state.pending.values()].filter((value) => value.kind === "create").length === GPU_MAX_SURFACE_SESSIONS) return { accepted: false, code: 19 };
    const { status, canvas } = canvasStatus(canvasId);
    return request(1825, generation, body((writer) => writer.u8(status).u32(surfaceId).u32(canvasId).u32(generation).u32(width).u32(height).f32(scaleFactor)), { kind: "create", surfaceId, canvasId, generation, canvas, width, height, scaleFactor });
  };
  const resize = ({ surfaceId, generation, width, height, scaleFactor = 1 }) => request(1826, generation, body((writer) => writer.u32(surfaceId).u32(generation).u32(width).u32(height).f32(scaleFactor)), { kind: "resize", surfaceId, generation, width, height, scaleFactor });
  const frame = ({ surfaceId, generation, frameId, bytes = new Uint8Array() }) => {
    if (!(bytes instanceof Uint8Array)) return { accepted: false, code: 2, bytes };
    if (bytes.length > GPU_MAX_FRAME_BYTES) return { accepted: false, code: 5, bytes };
    if (state.frames === GPU_MAX_IN_FLIGHT_FRAMES) return { accepted: false, code: 19, bytes };
    state.frames += 1;
    const result = request(1827, generation, body((writer) => writer.u32(surfaceId).u32(generation).u64(BigInt(frameId)).bytes(bytes)), { kind: "frame", surfaceId, generation, bytes });
    if (!result.accepted) state.frames -= 1;
    return { ...result, bytes: result.accepted ? undefined : bytes };
  };
  const lose = ({ surfaceId, generation, reason }) => request(1828, generation, body((writer) => writer.u32(surfaceId).u32(generation).u8(reason)), { kind: "loss", surfaceId, generation, reason });
  const recover = ({ surfaceId, generation }) => request(1829, generation, body((writer) => writer.u32(surfaceId).u32(generation)), { kind: "recover", surfaceId, generation });
  const drop = ({ surfaceId, generation }) => request(1830, generation, body((writer) => writer.u32(surfaceId).u32(generation)), { kind: "drop", surfaceId, generation });
  const control = (bytes, delivered) => {
    if (state.controls === GPU_MAX_IN_FLIGHT_CONTROLS) return { accepted: false, code: 19 };
    const result = enqueue(bytes, () => { state.controls -= 1; delivered?.(); });
    if (result.accepted) state.controls += 1;
    return result;
  };
  const releasePage = (predicate) => {
    const position = state.pages.findIndex(predicate);
    if (position < 0) return;
    const [page] = state.pages.splice(position, 1);
    if (page.effect?.kind === "frame" && !page.effect.frameReleased) state.frames = Math.max(0, state.frames - 1);
  };
  const cancel = (requestId, generation) => control(encodeCancel(requestId, generation), () => {
    releasePage((page) => page.outcome.requestId === BigInt(requestId) && page.outcome.generation === generation);
  });
  const acknowledge = (handle, index) => control(encodeAcknowledge(handle, index), () => {
    releasePage((page) => page.handle.slot === handle.slot && page.handle.generation === handle.generation && page.index === index);
  });
  const closePage = (handle) => control(encodeClose(handle), () => {
    releasePage((page) => page.handle.slot === handle.slot && page.handle.generation === handle.generation);
  });
  const poll = (pointer, capacity) => {
    if (state.closed && state.queue.length === 0) return 0xffffffff;
    const item = state.retained ?? state.queue[0];
    if (!item) return 0;
    if (capacity < item.bytes.length) { state.retained = item; return item.bytes.length; }
    memoryBytes().set(item.bytes, pointer);
    state.queue.shift(); state.retained = undefined; item.delivered?.();
    return item.bytes.length;
  };
  const send = (pointer, length) => {
    if (state.closed) return 20;
    if (length > 4160) return 5;
    let page;
    try { page = decodePage(memoryBytes().slice(pointer, pointer + length)); } catch { return 1; }
    if (state.pages.length === GPU_MAX_IN_FLIGHT_PAGES) return 19;
    const outcome = decodeOutcome(page.bytes);
    const effect = state.pending.get(outcome.requestId);
    const now = options.now ?? (() => globalThis.performance?.now?.() ?? Date.now());
    const started = now();
    try { options.onOutcome?.(outcome); } catch { return 11; }
    if (now() - started >= GPU_MAX_CALLBACK_MILLISECONDS) return 17;
    applyOutcome(state, outcome, effect);
    state.pending.delete(outcome.requestId);
    state.pages.push({ ...page, outcome, effect });
    return 0;
  };
  const takeOutcome = () => state.pages[0] ? { ...state.pages[0] } : undefined;
  const close = () => {
    for (const record of state.sessions.values()) record.canvas?.removeAttribute?.("data-raw-handle");
    state.closed = true; state.queue.length = 0; state.pages.length = 0; state.pending.clear(); state.sessions.clear(); state.retained = undefined; state.frames = 0; state.controls = 0;
  };

  return { bindMemory, imports: { semio_webgpu_surface: { send, poll } }, create, resize, frame, lose, recover, drop, cancel, acknowledge, closePage, takeOutcome, close, state };
}

/** 🌐️ Composes the accepted A2 browser host and A3 surface port over one Wasm memory. */
export function createBrowserWebGpuImports(options = {}) {
  const browserHost = options.browserHost ?? createBrowserHostImports(options);
  const surfacePort = createWebGpuSurfacePort(options);
  return {
    browserHost,
    surfacePort,
    imports: { ...browserHost.imports, ...surfacePort.imports },
    bindMemory(memory) { browserHost.bindMemory(memory); surfacePort.bindMemory(memory); },
    close() { surfacePort.close(); browserHost.close(); },
  };
}

//#endregion 🔌️SurfacePort

//#region 🧱️Ledger

function body(write) { const writer = new Writer(); writer.u8(1); write(writer); return writer.finish(); }
function encodeRequest(operation, requestId, generation, value) { const writer = new Writer(); return writer.u8(1).u8(1).u16(operation).u64(requestId).u32(generation).bytes(value).finish(); }
function encodeCancel(requestId, generation) { return new Writer().u8(1).u8(5).u8(1).u64(BigInt(requestId)).u32(generation).finish(); }
function encodeClose(handle) { return new Writer().u8(1).u8(5).u8(2).u32(handle.slot).u32(handle.generation).finish(); }
function encodeAcknowledge(handle, index) { return new Writer().u8(1).u8(5).u8(3).u32(handle.slot).u32(handle.generation).u32(index).finish(); }

function decodePage(bytes) {
  const reader = new Reader(bytes);
  if (reader.u8() !== 1 || reader.u8() !== 4) throw new Error("not an A1 page");
  const page = { handle: { slot: reader.u32(), generation: reader.u32() }, index: reader.u32(), bytes: reader.bytes(8192) };
  reader.finish(); return page;
}

function decodeOutcome(bytes) {
  const reader = new Reader(bytes);
  if (reader.u8() !== 1) throw new Error("bad outcome version");
  const tag = reader.u8();
  const outcome = { tag, requestId: reader.u64(), surfaceId: reader.u32(), generation: reader.u32() };
  if (tag === 1 || tag === 2) Object.assign(outcome, { width: reader.u32(), height: reader.u32(), scaleFactor: reader.f32() });
  else if (tag === 3) Object.assign(outcome, { frameId: reader.u64(), payloadBytes: reader.u32() });
  else if (tag === 4) outcome.reason = reader.u8();
  else if (tag === 5) Object.assign(outcome, { previousGeneration: reader.u32(), width: reader.u32(), height: reader.u32(), scaleFactor: reader.f32() });
  else if (tag === 8) outcome.code = reader.u16();
  reader.finish(); return outcome;
}

function applyOutcome(state, outcome, effect) {
  if (!effect) return;
  if (outcome.tag === 1 && effect.kind === "create") {
    effect.canvas?.setAttribute?.("data-raw-handle", String(effect.surfaceId));
    state.sessions.set(effect.surfaceId, { ...effect });
  } else if (outcome.tag === 2 && effect.kind === "resize") {
    const record = state.sessions.get(effect.surfaceId);
    if (record) { record.canvas.width = effect.width; record.canvas.height = effect.height; Object.assign(record, effect); }
  } else if (outcome.tag === 5 && effect.kind === "recover") {
    const record = state.sessions.get(effect.surfaceId); if (record) record.generation = outcome.generation;
  } else if (outcome.tag === 6 && effect.kind === "drop") {
    const record = state.sessions.get(effect.surfaceId); record?.canvas?.removeAttribute?.("data-raw-handle"); state.sessions.delete(effect.surfaceId);
  } else if ((outcome.tag === 7 || outcome.tag === 8) && effect.kind === "frame") {
    state.frames = Math.max(0, state.frames - 1); effect.frameReleased = true;
  }
}

class Writer {
  constructor() { this.value = []; }
  u8(value) { this.value.push(value & 255); return this; }
  u16(value) { const bytes = new Uint8Array(2); new DataView(bytes.buffer).setUint16(0, value, true); return this.raw(bytes); }
  u32(value) { const bytes = new Uint8Array(4); new DataView(bytes.buffer).setUint32(0, value, true); return this.raw(bytes); }
  u64(value) { const bytes = new Uint8Array(8); new DataView(bytes.buffer).setBigUint64(0, BigInt(value), true); return this.raw(bytes); }
  f32(value) { const bytes = new Uint8Array(4); new DataView(bytes.buffer).setFloat32(0, value, true); return this.raw(bytes); }
  bytes(value) { this.u32(value.length); return this.raw(value); }
  raw(value) { this.value.push(...value); return this; }
  finish() { return Uint8Array.from(this.value); }
}

class Reader {
  constructor(bytes) { this.bytesValue = bytes; this.cursor = 0; }
  take(length) { if (this.cursor + length > this.bytesValue.length) throw new Error("short ledger"); const value = this.bytesValue.slice(this.cursor, this.cursor + length); this.cursor += length; return value; }
  u8() { return this.take(1)[0]; }
  u16() { const value = this.take(2); return new DataView(value.buffer, value.byteOffset, 2).getUint16(0, true); }
  u32() { const value = this.take(4); return new DataView(value.buffer, value.byteOffset, 4).getUint32(0, true); }
  u64() { const value = this.take(8); return new DataView(value.buffer, value.byteOffset, 8).getBigUint64(0, true); }
  f32() { const value = this.take(4); return new DataView(value.buffer, value.byteOffset, 4).getFloat32(0, true); }
  bytes(maximum) { const length = this.u32(); if (length > maximum) throw new Error("oversized ledger"); return this.take(length); }
  finish() { if (this.cursor !== this.bytesValue.length) throw new Error("trailing ledger"); }
}

//#endregion 🧱️Ledger
