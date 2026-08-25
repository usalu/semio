export const BROWSER_HOST_MAX_EVENT_BODY_BYTES = 1024;
export const BROWSER_HOST_EVENT_ENVELOPE_BYTES = 27;
export const BROWSER_HOST_MAX_ENCODED_EVENT_BYTES = BROWSER_HOST_MAX_EVENT_BODY_BYTES + BROWSER_HOST_EVENT_ENVELOPE_BYTES;
export const BROWSER_HOST_PAGE_ENVELOPE_BYTES = 18;
export const BROWSER_HOST_MAX_PAGE_BODY_BYTES = BROWSER_HOST_MAX_ENCODED_EVENT_BYTES - BROWSER_HOST_PAGE_ENVELOPE_BYTES;
export const BROWSER_HOST_INITIAL_POLL_BYTES = BROWSER_HOST_MAX_EVENT_BODY_BYTES;

/** 🌐️ Sole browser-object and Wasm linear-memory owner for the framework UI host. */
export function createBrowserHostImports(options = {}) {
  const state = {
    memory: undefined,
    listeners: new Map(),
    freeSlots: [],
    generations: new Map(),
    critical: [],
    latest: new Map(),
    retained: undefined,
    cancelled: new Set(),
    awaitingAcknowledgement: new Map(),
    nextSlot: 1,
    nextEvent: 1n,
    closed: false,
  };
  const encoder = new TextEncoder();
  const decoder = new TextDecoder("utf-8", { fatal: true });
  const maximumCritical = options.maximumCritical ?? 32;
  const maximumLatest = options.maximumLatest ?? 32;
  const maximumListeners = Math.min(options.maximumListeners ?? 64, 0xffffffff);
  const maximumAcknowledgements = options.maximumAcknowledgements ?? 64;

  const bindMemory = (memory) => {
    state.memory = memory;
  };

  const enqueue = (bytes, latestKey, metadata = {}) => {
    if (state.closed) return;
    if (bytes.length > BROWSER_HOST_MAX_ENCODED_EVENT_BYTES) {
      state.closed = true;
      closeAll();
      state.critical.length = 0;
      state.latest.clear();
      state.retained = undefined;
      return;
    }
    const item = { bytes, ...metadata };
    if (latestKey !== undefined) {
      if (!state.latest.has(latestKey) && state.latest.size >= maximumLatest) {
        state.closed = true;
        closeAll();
        state.latest.clear();
        return;
      }
      state.latest.set(latestKey, item);
      return;
    }
    if (state.critical.length >= maximumCritical) {
      state.closed = true;
      closeAll();
      state.latest.clear();
      return;
    }
    state.critical.push(item);
  };

  const reply = (request, status, body = new Uint8Array()) => {
    const writer = new Writer();
    writer.u8(1); writer.u8(2); writer.u64(request.requestId); writer.u32(request.generation);
    writer.u16(status); writer.u8(0); writer.bytes(body);
    enqueue(writer.finish(), undefined, { requestKey: `${request.requestId}:${request.generation}` });
  };

  const event = (record, code, body, latestKey) => {
    if (body.length > BROWSER_HOST_MAX_EVENT_BODY_BYTES) {
      state.closed = true;
      closeAll();
      state.latest.clear();
      return;
    }
    const writer = new Writer();
    const requestId = state.nextEvent++;
    writer.u8(1); writer.u8(3); writer.u64(requestId); writer.u32(record.generation);
    writer.u32(record.sequence++); writer.u16(code); writer.u16(0); writer.u8(0); writer.bytes(body);
    const scopedKey = latestKey === undefined ? undefined : `${record.slot}:${record.listenerGeneration}:${latestKey}`;
    if (scopedKey !== undefined) record.latestKeys.add(scopedKey);
    enqueue(writer.finish(), scopedKey, { acknowledgementKey: `${requestId}:${record.generation}`, listenerKey: `${record.slot}:${record.listenerGeneration}` });
  };

  const prefix = (record) => {
    const writer = new Writer();
    writer.u8(1); writer.u32(record.canvasId); writer.u32(record.slot); writer.u32(record.listenerGeneration);
    return writer;
  };

  const fail = (request, unavailable) => reply(request, 4, Uint8Array.of(unavailable));

  const attach = (request) => {
    const globalWindow = typeof window === "undefined" ? undefined : window;
    if (!globalWindow) return fail(request, 1);
    const globalDocument = globalWindow.document;
    if (!globalDocument) return fail(request, 2);
    const canvasId = readCanvas(request.body);
    const canvas = options.resolveCanvas?.(canvasId) ?? globalDocument.querySelector(`[data-semio-canvas-id="${canvasId}"]`);
    if (!canvas) return fail(request, 3);
    const configuredLabel = typeof options.accessibleLabel === "function" ? options.accessibleLabel(canvasId) : options.accessibleLabel;
    const accessibleLabel = configuredLabel ?? canvas.getAttribute?.("aria-label");
    if (typeof accessibleLabel !== "string" || accessibleLabel.length === 0) return fail(request, 6);
    if (state.listeners.size >= maximumListeners) return fail(request, 5);
    let slot;
    while (state.freeSlots.length > 0) {
      const candidate = state.freeSlots.pop();
      if ((state.generations.get(candidate) ?? 0) < 0xffffffff) {
        slot = candidate;
        break;
      }
    }
    if (slot === undefined) {
      if (state.nextSlot > maximumListeners) return fail(request, 5);
      slot = state.nextSlot++;
    }
    const listenerGeneration = (state.generations.get(slot) ?? 0) + 1;
    state.generations.set(slot, listenerGeneration);
    const record = { canvasId, canvas, globalWindow, slot, listenerGeneration, generation: request.generation, sequence: 0, removers: [], latestKeys: new Set(), clipboardKeys: new Set(), observer: undefined, frame: undefined, framePending: false, closed: false };
    state.listeners.set(slot, record);
    canvas.tabIndex = canvas.tabIndex < 0 ? 0 : canvas.tabIndex;
    canvas.setAttribute("role", options.role ?? "application");
    canvas.setAttribute("aria-label", accessibleLabel);
    listen(record, globalWindow, globalDocument);
    const body = new Writer(); body.u8(1); body.u32(slot); body.u32(listenerGeneration);
    reply(request, 0, body.finish());
    emitMetrics(record, "metrics");
    emitVisibility(record, globalDocument);
  };

  const listen = (record, globalWindow, globalDocument) => {
    const add = (target, kind, callback, settings) => {
      const guarded = (value) => {
        const current = state.listeners.get(record.slot);
        if (current !== record || current.closed || current.listenerGeneration !== record.listenerGeneration) return;
        try { callback(value); } catch { closeRecord(record); }
      };
      target.addEventListener(kind, guarded, settings);
      record.removers.push(() => target.removeEventListener(kind, guarded, settings));
    };
    add(record.canvas, "pointermove", (value) => emitPointer(record, 1804, value, `pointer:${value.pointerId}`));
    add(record.canvas, "pointerdown", (value) => { record.canvas.focus(); emitPointer(record, 1805, value); });
    add(record.canvas, "pointerup", (value) => emitPointer(record, 1806, value));
    add(record.canvas, "wheel", (value) => { value.preventDefault(); emitWheel(record, value); }, { passive: false });
    add(record.canvas, "keydown", (value) => emitKey(record, 1808, value));
    add(record.canvas, "keyup", (value) => emitKey(record, 1809, value));
    add(record.canvas, "beforeinput", (value) => { if (typeof value.data === "string") emitText(record, value.data); });
    add(globalDocument, "visibilitychange", () => emitVisibility(record, globalDocument));
    const ObserverType = globalWindow.ResizeObserver ?? (typeof ResizeObserver === "undefined" ? undefined : ResizeObserver);
    if (ObserverType) {
      record.observer = new ObserverType(() => {
        if (state.listeners.get(record.slot) === record && !record.closed) emitMetrics(record, "metrics");
      });
      record.observer.observe(record.canvas);
    } else {
      add(globalWindow, "resize", () => emitMetrics(record, "metrics"));
    }
  };

  const emitMetrics = (record, key) => {
    const ratio = Number.isFinite(record.globalWindow.devicePixelRatio) ? record.globalWindow.devicePixelRatio : 1;
    const width = Math.max(0, Math.round(record.canvas.clientWidth * ratio));
    const height = Math.max(0, Math.round(record.canvas.clientHeight * ratio));
    record.canvas.width = width; record.canvas.height = height;
    const body = prefix(record); body.u32(width); body.u32(height); body.f32(ratio);
    event(record, 1801, body.finish(), key);
  };

  const emitVisibility = (record, globalDocument) => {
    const body = prefix(record); body.u8(globalDocument.hidden ? 0 : 1);
    event(record, 1802, body.finish(), "visibility");
  };

  const emitPointer = (record, code, value, latestKey) => {
    const body = prefix(record);
    const kind = value.pointerType === "touch" ? 1 : value.pointerType === "pen" ? 2 : 0;
    body.i32(value.pointerId); body.u8(kind); body.f32(value.pressure || Number.NaN);
    body.f32(value.tiltX || Number.NaN); body.f32(value.tiltY || Number.NaN);
    body.f32(value.offsetX); body.f32(value.offsetY); body.i16(value.button ?? 0);
    event(record, code, body.finish(), latestKey);
  };

  const emitWheel = (record, value) => {
    const scale = value.deltaMode === 1 ? 40 : value.deltaMode === 2 ? record.canvas.clientHeight : 1;
    const body = prefix(record); body.f32(value.offsetX); body.f32(value.offsetY);
    body.f32(value.deltaX * scale); body.f32(value.deltaY * scale);
    event(record, 1807, body.finish());
  };

  const modifierBits = (value) => (value.shiftKey ? 1 : 0) | (value.ctrlKey ? 2 : 0) | (value.altKey ? 4 : 0) | (value.metaKey ? 8 : 0);
  const emitKey = (record, code, value) => {
    const text = encoder.encode(value.key);
    const body = prefix(record); body.u8(modifierBits(value)); body.u16(text.length); body.raw(text);
    event(record, code, body.finish());
  };

  const emitText = (record, value) => {
    const text = encoder.encode(value);
    const body = prefix(record); body.u16(text.length); body.raw(text);
    event(record, 1810, body.finish());
  };

  const scheduleFrame = (request) => {
    const record = findCanvas(readCanvas(request.body));
    if (!record) return fail(request, 5);
    if (typeof requestAnimationFrame !== "function") return fail(request, 6);
    reply(request, 0);
    if (record.framePending) return;
    record.framePending = true;
    record.frame = requestAnimationFrame((timestamp) => {
      if (record.closed || state.listeners.get(record.slot) !== record) return;
      record.framePending = false;
      const body = prefix(record); body.f64(timestamp);
      event(record, 1803, body.finish());
    });
  };

  const clipboard = (request, write) => {
    const record = findCanvas(readCanvas(request.body));
    if (!record) return fail(request, 5);
    const api = typeof navigator === "undefined" ? undefined : navigator.clipboard;
    if (!api) return fail(request, 4);
    if (record.clipboardKeys.size >= 1) return fail(request, 6);
    const key = `${request.requestId}:${request.generation}`;
    record.clipboardKeys.add(key);
    let action;
    try {
      action = write ? api.writeText(decoder.decode(request.body.subarray(5))) : api.readText();
    } catch {
      record.clipboardKeys.delete(key);
      return fail(request, 4);
    }
    Promise.resolve(action).then((value) => {
      record.clipboardKeys.delete(key);
      const cancelled = state.cancelled.delete(key);
      if (cancelled || record.closed || state.listeners.get(record.slot) !== record) return;
      reply(request, 0, write ? new Uint8Array() : encoder.encode(value));
    }).catch(() => {
      record.clipboardKeys.delete(key);
      const cancelled = state.cancelled.delete(key);
      if (!cancelled && !record.closed && state.listeners.get(record.slot) === record) fail(request, 4);
    });
  };

  const closeRecord = (record) => {
    if (record.closed) return;
    record.closed = true;
    if (record.frame !== undefined && typeof cancelAnimationFrame === "function") cancelAnimationFrame(record.frame);
    record.observer?.disconnect();
    for (const remove of record.removers.splice(0)) remove();
    const listenerKey = `${record.slot}:${record.listenerGeneration}`;
    state.critical = state.critical.filter((item) => item.listenerKey !== listenerKey);
    if (state.retained?.item.listenerKey === listenerKey) state.retained = undefined;
    for (const key of record.latestKeys) {
      state.latest.delete(key);
      if (state.retained?.source === "latest" && state.retained.key === key) state.retained = undefined;
    }
    record.latestKeys.clear();
    for (const key of record.clipboardKeys) state.cancelled.delete(key);
    record.clipboardKeys.clear();
    for (const [key, owner] of state.awaitingAcknowledgement) {
      if (owner === listenerKey) state.awaitingAcknowledgement.delete(key);
    }
    state.listeners.delete(record.slot);
    if (record.listenerGeneration < 0xffffffff) state.freeSlots.push(record.slot);
  };
  const closeAll = () => { for (const record of [...state.listeners.values()]) closeRecord(record); };
  const findCanvas = (canvasId) => [...state.listeners.values()].find((record) => record.canvasId === canvasId && !record.closed);

  const cancelQueuedRequest = (key) => {
    let removed = false;
    state.critical = state.critical.filter((item) => {
      const keep = item.requestKey !== key;
      removed ||= !keep;
      return keep;
    });
    for (const [latestKey, item] of state.latest) {
      if (item.requestKey === key) {
        state.latest.delete(latestKey);
        removed = true;
      }
    }
    if (state.retained?.item.requestKey === key) {
      state.retained = undefined;
      removed = true;
    }
    return removed;
  };

  const receive = (bytes) => {
    let message;
    try { message = decodeMessage(bytes); } catch { return 0; }
    if (message.tag === 1) {
      if (message.operation === 1793) attach(message);
      else if (message.operation === 1794) scheduleFrame(message);
      else if (message.operation === 1795) {
        const record = findCanvas(readCanvas(message.body));
        if (!record) fail(message, 5); else { record.canvas.style.cursor = ["default", "pointer", "text", "grab", "grabbing"][message.body[5]] ?? "default"; reply(message, 0); }
      } else if (message.operation === 1796) clipboard(message, false);
      else if (message.operation === 1797) clipboard(message, true);
      else if (message.operation === 1798) { const record = findCanvas(readCanvas(message.body)); if (record) closeRecord(record); reply(message, 0); }
      else reply(message, 3);
    } else if (message.tag === 2) {
      return state.awaitingAcknowledgement.delete(`${message.requestId}:${message.generation}`) ? 1 : 0;
    } else if (message.tag === 5 && message.control === 1) {
      const key = `${message.requestId}:${message.generation}`;
      const active = [...state.listeners.values()].some((record) => record.clipboardKeys.has(key));
      cancelQueuedRequest(key);
      if (active) state.cancelled.add(key);
    } else if (message.tag === 5 && message.control === 2) {
      const record = state.listeners.get(message.slot);
      if (record?.listenerGeneration === message.listenerGeneration) closeRecord(record);
    }
    return 1;
  };

  const imports = {
    send(pointer, length) {
      if (!state.memory || state.closed) return 0;
      return receive(new Uint8Array(state.memory.buffer, pointer, length).slice());
    },
    poll(pointer, capacity) {
      if (!state.memory) return -1;
      let retained = state.retained;
      if (!retained) {
        const critical = state.critical[0];
        const latest = critical ? undefined : state.latest.entries().next().value;
        if (!critical && !latest) return state.closed ? -1 : 0;
        retained = critical ? { source: "critical", item: critical } : { source: "latest", key: latest[0], item: latest[1] };
      }
      const { bytes } = retained.item;
      if (bytes.length > capacity) {
        state.retained = retained;
        return bytes.length;
      }
      if (retained.item.acknowledgementKey !== undefined && !state.awaitingAcknowledgement.has(retained.item.acknowledgementKey) && state.awaitingAcknowledgement.size >= maximumAcknowledgements) {
        state.closed = true;
        closeAll();
        state.critical.length = 0;
        state.latest.clear();
        state.retained = undefined;
        return -1;
      }
      new Uint8Array(state.memory.buffer, pointer, bytes.length).set(bytes);
      if (retained.source === "critical" && state.critical[0] === retained.item) state.critical.shift();
      if (retained.source === "latest" && state.latest.get(retained.key) === retained.item) state.latest.delete(retained.key);
      if (retained.item.acknowledgementKey !== undefined) state.awaitingAcknowledgement.set(retained.item.acknowledgementKey, retained.item.listenerKey);
      state.retained = undefined;
      return bytes.length;
    },
  };

  return { bindMemory, imports: { semio_browser_host: imports }, close: () => { state.closed = true; closeAll(); state.critical.length = 0; state.latest.clear(); state.retained = undefined; state.cancelled.clear(); state.awaitingAcknowledgement.clear(); } };
}

function readCanvas(body) {
  if (body.length < 5 || body[0] !== 1) throw new Error("malformed canvas envelope");
  return new DataView(body.buffer, body.byteOffset + 1, 4).getUint32(0, true);
}

function decodeMessage(bytes) {
  const reader = new Reader(bytes);
  if (reader.u8() !== 1) throw new Error("version");
  const tag = reader.u8();
  if (tag === 1) return { tag, operation: reader.u16(), requestId: reader.u64(), generation: reader.u32(), body: reader.bytes() };
  if (tag === 2) return { tag, requestId: reader.u64(), generation: reader.u32() };
  if (tag === 5) {
    const control = reader.u8();
    if (control === 1) return { tag, control, requestId: reader.u64(), generation: reader.u32() };
    if (control === 2) return { tag, control, slot: reader.u32(), listenerGeneration: reader.u32() };
    if (control === 3) return { tag, control, slot: reader.u32(), listenerGeneration: reader.u32(), index: reader.u32() };
  }
  throw new Error("tag");
}

class Writer {
  constructor() { this.values = []; }
  u8(value) { this.values.push(value & 255); }
  i16(value) { this.raw(numberBytes(2, (view) => view.setInt16(0, value, true))); }
  u16(value) { this.raw(numberBytes(2, (view) => view.setUint16(0, value, true))); }
  i32(value) { this.raw(numberBytes(4, (view) => view.setInt32(0, value, true))); }
  u32(value) { this.raw(numberBytes(4, (view) => view.setUint32(0, value, true))); }
  u64(value) { this.raw(numberBytes(8, (view) => view.setBigUint64(0, BigInt(value), true))); }
  f32(value) { this.raw(numberBytes(4, (view) => view.setFloat32(0, value, true))); }
  f64(value) { this.raw(numberBytes(8, (view) => view.setFloat64(0, value, true))); }
  bytes(value) { this.u32(value.length); this.raw(value); }
  raw(value) { this.values.push(...value); }
  finish() { return Uint8Array.from(this.values); }
}

class Reader {
  constructor(bytes) { this.bytesValue = bytes; this.cursor = 0; }
  take(length) { const value = this.bytesValue.subarray(this.cursor, this.cursor + length); if (value.length !== length) throw new Error("length"); this.cursor += length; return value; }
  view(length) { const value = this.take(length); return new DataView(value.buffer, value.byteOffset, value.byteLength); }
  u8() { return this.take(1)[0]; }
  u16() { return this.view(2).getUint16(0, true); }
  u32() { return this.view(4).getUint32(0, true); }
  u64() { return this.view(8).getBigUint64(0, true); }
  bytes() { return this.take(this.u32()); }
}

function numberBytes(length, write) {
  const bytes = new Uint8Array(length);
  write(new DataView(bytes.buffer));
  return bytes;
}
