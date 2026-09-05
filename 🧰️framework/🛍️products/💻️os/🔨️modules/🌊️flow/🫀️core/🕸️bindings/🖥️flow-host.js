//#region 🧬️Contract

export const FLOW_MAX_REQUEST_BYTES = 1_048_576;
export const FLOW_MAX_PAGE_BYTES = 65_536;
export const FLOW_MAX_TRANSFER_BYTES = 16_777_216;
export const FLOW_MAX_ENCODED_MESSAGE_BYTES = FLOW_MAX_REQUEST_BYTES + 32;
export const FLOW_MAX_IN_FLIGHT = 256;

export const FlowOperation = Object.freeze({
  open:2500,vcsCheckpoint:2501,vcsFault:2502,vcsRetryCheckpoint:2503,catalogueJson:2504,setCatalogueJson:2505,setNeuronKindInfosJson:2506,addInputPort:2507,removeInputPort:2508,addOutputPort:2509,removeOutputPort:2510,connectPorts:2511,compiledWireLiteral:2512,applyEvalOutputsJson:2513,setComputingProgress:2514,setNodeStatuses:2515,clearComputingWidgetIds:2516,previewText:2517,selectedWidgetIds:2518,selectedEdgeIds:2519,selectionDomainsJson:2520,hoveredWidgetId:2521,hoveredChannelJson:2522,selectedChannelsJson:2523,previewOffWidgetIds:2524,setSelection:2525,setHover:2526,setHoverChannel:2527,setSelectedChannels:2528,setPreviewOff:2529,togglePreview:2530,collapseSelection:2531,explodeCluster:2532,takePendingExportClick:2533,exportPayloadJson:2534,takePendingClusterExplode:2535,setSliderValue:2536,sliderOverlayStateJson:2537,setNoteText:2538,beginNoteEdit:2539,noteInsertText:2540,noteBackspace:2541,noteDeleteForward:2542,noteMoveCaret:2543,noteCommitEdit:2544,setNoteCaretVisible:2545,setImageSrc:2546,schemasJson:2547,setVariableName:2548,setVariableSchema:2549,addWidget:2550,setGhostWidget:2551,clearGhostWidget:2552,removeWidget:2553,moveWidget:2554,insertBetween:2555,makeSpace:2556,setNeuronParams:2557,connect:2558,disconnect:2559,undo:2560,redo:2561,canUndo:2562,canRedo:2563,worldFromScreen:2564,setCamera:2565,cameraJson:2566,wheelScreen:2567,setWheelZoomActive:2568,lodScaleJson:2569,setAutomaticLod:2570,setProximityDistance:2571,setForcedDrawLodLabel:2572,drawLodLabel:2573,labelOverlayPaintStateJson:2574,attachSurface:2575,surfaceStatus:2576,gpuReady:2577,setSize:2578,setCanvasThemeJson:2579,reorganize:2580,renderFrame:2581,pointerDownScreen:2582,pointerMoveScreen:2583,pickTargetsAtScreenJson:2584,entityScreenJson:2585,widgetDragActive:2586,pointerUpScreen:2587,setSelectionOptions:2588,selectionPreviewPointsJson:2589,selectionPreviewCrossing:2590,selectionPreviewMethod:2591,selectionUnionBoundsScreenJson:2592,alignSelection:2593,preselectWidgetIdsJson:2594,cancelAreaSelect:2595,deleteSelection:2596,hasSelection:2597,selectAll:2598,tessellate:2599,renderDrawingScene:2600,exportDrawingSvg:2601,exportDrawingPdf:2602,exportDrawingDwg:2603,importDrawingDwg:2604,traceDrawingBitmap:2605,booleanDrawingSegments:2606,dispose:2607,dwgEncodeMeshJson:2608,documentJson:2609,synchronizeDocumentJson:2610,
});

export const FlowOperationFields = Object.freeze({
  vcsCheckpoint:"sessionGeneration:u,baseRevision:q,parentRevision:q",vcsFault:"sessionGeneration:u,baseRevision:q,parentRevision:q",vcsRetryCheckpoint:"sessionGeneration:u,baseRevision:q,parentRevision:q",setCatalogueJson:"json:s",setNeuronKindInfosJson:"json:s",addInputPort:"widgetId:s,index:q",removeInputPort:"widgetId:s,portId:s",addOutputPort:"widgetId:s,index:q",removeOutputPort:"widgetId:s,portId:s",connectPorts:"fromId:s,fromPort:s,toId:s,toPort:s",applyEvalOutputsJson:"json:s",setComputingProgress:"json:s",setNodeStatuses:"json:s",setSelection:"json:s",setHover:"widgetId:o",setHoverChannel:"widgetId:o,port:o",setSelectedChannels:"json:s",setPreviewOff:"json:s",togglePreview:"widgetId:s",collapseSelection:"json:s",explodeCluster:"clusterId:s",exportPayloadJson:"widgetId:s",setSliderValue:"widgetId:s,value:d",setNoteText:"widgetId:s,text:s",beginNoteEdit:"widgetId:s,worldX:d,worldY:d",noteInsertText:"chunk:s",noteMoveCaret:"direction:s,extend:b",setNoteCaretVisible:"visible:b",setImageSrc:"widgetId:s,src:s",setVariableName:"widgetId:s,name:s",setVariableSchema:"widgetId:s,schema:s",addWidget:"descriptorJson:s,worldX:d,worldY:d",setGhostWidget:"descriptorJson:s,worldX:d,worldY:d",removeWidget:"widgetId:s",moveWidget:"widgetId:s,x:d,y:d",insertBetween:"anchorId:s,anchorOutPort:s,midId:s,midInPort:s,midOutPort:s",makeSpace:"anchorId:s,dx:d,dy:d",setNeuronParams:"widgetId:s,paramsJson:s",connect:"fromId:s,toId:s",disconnect:"synapseId:s",worldFromScreen:"sx:d,sy:d",setCamera:"x:d,y:d,zoom:d",wheelScreen:"sx:d,sy:d,deltaX:d,deltaY:d,zoomGesture:b",setWheelZoomActive:"active:b",setAutomaticLod:"enabled:b",setProximityDistance:"world:d",setForcedDrawLodLabel:"label:s",attachSurface:"surface:u,surfaceGeneration:u,width:u,height:u,dpr:d",surfaceStatus:"surface:u,surfaceGeneration:u,status:s",setSize:"width:u,height:u,dpr:d",setCanvasThemeJson:"json:s",reorganize:"json:s",pointerDownScreen:"sx:d,sy:d,button:c,shift:b,ctrlOrMeta:b,alt:b,pan:b",pointerMoveScreen:"sx:d,sy:d,shift:b,ctrlOrMeta:b,alt:b",pickTargetsAtScreenJson:"sx:d,sy:d",entityScreenJson:"domain:s,id:s",pointerUpScreen:"sx:d,sy:d,shift:b,ctrlOrMeta:b,alt:b",setSelectionOptions:"method:s,mode:s",alignSelection:"mode:s",tessellate:"handle:s,tolerance:d",renderDrawingScene:"handle:s",exportDrawingSvg:"handle:s",exportDrawingPdf:"handle:s",exportDrawingDwg:"handle:s",importDrawingDwg:"dataBase64:s",traceDrawingBitmap:"width:u,height:u,mask:x,threshold:d,simplifyEpsilon:d",booleanDrawingSegments:"aJson:s,bJson:s,operation:s",dispose:"handle:s",dwgEncodeMeshJson:"meshJson:s",synchronizeDocumentJson:"json:s",
});

//#endregion 🧬️Contract

//#region 🌉️ReactiveHost

export function createFlowHost({ exports, memory, schedule = queueMicrotask, now = Date.now, maximumInFlight = FLOW_MAX_IN_FLIGHT } = {}) {
  if (!exports || !memory) throw new Error("Flow Wasm exports and memory are required");
  const state = { nextRequest: 1n, generation: 1, pending: new Map(), pages: new Map(), blocked: undefined, pumping: false, closing: false, closed: false, closePromise: undefined };

  const budget = () => { const time = BigInt(Math.trunc(now())); return [4_096, time, time + 8n]; };
  const transfer = (bytes) => {
    if (bytes.length === 0 || bytes.length > FLOW_MAX_ENCODED_MESSAGE_BYTES) throw new Error("Flow message exceeds its bound");
    const pointer = exports.flow_bridge_allocate(bytes.length);
    if (!pointer) throw new Error("Flow allocation failed");
    try {
      new Uint8Array(memory.buffer, pointer, bytes.length).set(bytes);
      const [credit, time, deadline] = budget();
      if (exports.flow_bridge_send(pointer, bytes.length, credit, time, deadline) !== 1) throw new Error("Flow message rejected");
    } finally { exports.flow_bridge_release(pointer, bytes.length); }
  };
  const pollExact = () => {
    let capacity = 1_024;
    let pointer = exports.flow_bridge_allocate(capacity);
    if (!pointer) throw new Error("Flow poll allocation failed");
    try {
      let [credit, time, deadline] = budget();
      let length = exports.flow_bridge_poll(pointer, capacity, credit, time, deadline);
      if (length <= 0) return { length };
      if (length > FLOW_MAX_ENCODED_MESSAGE_BYTES) throw new Error("Flow poll exceeds its bound");
      if (length > capacity) {
        exports.flow_bridge_release(pointer, capacity);
        capacity = length;
        pointer = exports.flow_bridge_allocate(capacity);
        if (!pointer) throw new Error("Flow exact poll allocation failed");
        [credit, time, deadline] = budget();
        length = exports.flow_bridge_poll(pointer, capacity, credit, time, deadline);
        if (length !== capacity) throw new Error("Flow retained poll changed length");
      }
      return { length, bytes: new Uint8Array(memory.buffer, pointer, length).slice() };
    } finally { exports.flow_bridge_release(pointer, capacity); }
  };
  const transferControl = (bytes, commit) => { try { transfer(bytes); } catch { return false; } commit(); return true; };
  const accept = (message) => {
    if (message.tag === 3) {
      const origin = message.requestId ^ (BigInt(message.sequence) << 32n);
      let operation;
      if (message.event === 2_650) operation = readHandle(new Reader(message.body));
      if (message.event === 2_655) {
        const reader = new Reader(message.body);
        const handle = readHandle(reader);
        const total = Number(reader.u64());
        reader.finish();
        if (total > FLOW_MAX_TRANSFER_BYTES) throw new Error("Flow output exceeds its bound");
        state.pages.set(handleKey(handle), { total, chunks: [], length: 0 });
      }
      return transferControl(encodeReply(message.requestId, message.generation, new Uint8Array()), () => {
        const pending = state.pending.get(origin);
        if (operation && pending) pending.operation = operation;
        if (pending) for (const observer of pending.observers) observer(message);
      });
    }
    if (message.tag === 4) {
      if (message.body.length > FLOW_MAX_PAGE_BYTES) throw new Error("Flow page exceeds its bound");
      const page = state.pages.get(handleKey(message.handle));
      if (!page || message.index !== page.chunks.length) throw new Error("Flow page is unadmitted or out of order");
      const next = page.length + message.body.length;
      if (next > page.total || next > FLOW_MAX_TRANSFER_BYTES) throw new Error("Flow page exceeds its admitted total");
      return transferControl(encodeAcknowledge(message.handle, message.index), () => { page.chunks.push(message.body); page.length = next; });
    }
    if (message.tag !== 2) throw new Error("Unexpected Flow message");
    const pending = state.pending.get(message.requestId);
    if (!pending) return true;
    state.pending.delete(message.requestId);
    if (message.status !== 0) { pending.reject(new Error(message.errorMessage || `Flow status ${message.status}`)); return true; }
    let body = message.body;
    if (pending.operation) {
      const page = state.pages.get(handleKey(pending.operation));
      if (page) {
        if (page.length !== page.total) throw new Error("Flow output ended before its admitted total");
        body = join(page.chunks, page.total);
        state.pages.delete(handleKey(pending.operation));
      }
    }
    pending.resolve(body);
    return true;
  };
  const pump = () => {
    if (state.pumping) return;
    state.pumping = true;
    const step = () => {
      try {
        for (let count = 0; count < 64; count += 1) {
          if (state.blocked) { if (!accept(state.blocked)) break; state.blocked = undefined; continue; }
          const result = pollExact();
          if (result.length < 0) throw new Error("Flow bridge closed");
          if (result.length === 0) break;
          const message = decodeFlowMessage(result.bytes);
          if (!accept(message)) { state.blocked = message; break; }
        }
      } catch (error) {
        for (const pending of state.pending.values()) pending.reject(error);
        state.pending.clear();
        state.closed = true;
      } finally {
        if (!state.closed && !state.closing && (state.pending.size || state.blocked)) schedule(step);
        else state.pumping = false;
      }
    };
    schedule(step);
  };
  const start = (operation, args = {}, session) => {
    if (state.closed || state.closing) return rejectedTask(new Error("Flow host is closed"));
    if (state.pending.size >= maximumInFlight) return rejectedTask(new Error("Flow in-flight limit"));
    const requestId = state.nextRequest;
    let frame;
    try {
      const payload = operation === FlowOperation.open ? new Uint8Array() : encodeOperationArguments(operation, args);
      const body = session ? concat(encodeHandle(session), payload) : payload;
      if (body.length > FLOW_MAX_REQUEST_BYTES) throw new Error("Flow request exceeds its bound");
      frame = encodeRequest(operation, requestId, state.generation, body);
      transfer(frame);
    } catch (error) { return rejectedTask(error); }
    state.nextRequest += 1n;
    let settled;
    const result = new Promise((resolve, reject) => { settled = { resolve, reject, operation: undefined, observers: new Set() }; });
    state.pending.set(requestId, settled);
    try { pump(); } catch (error) { state.pending.delete(requestId); settled.reject(error); }
    return { requestId, result, cancel: () => cancel(requestId), subscribe(observer) { settled.observers.add(observer); return () => settled.observers.delete(observer); } };
  };
  const cancel = (requestId) => {
    if (!state.pending.has(requestId)) return false;
    transfer(encodeCancel(requestId, state.generation));
    pump();
    return true;
  };
  const closeHandle = (handle) => { transfer(encodeClose(handle)); return true; };
  const close = () => {
    if (state.closePromise) return state.closePromise;
    if (state.closed) return Promise.resolve();
    exports.flow_bridge_begin_close();
    state.closing = true;
    state.closePromise = new Promise((resolve, reject) => {
      const drain = () => {
        try {
          for (let count = 0; count < 64; count += 1) {
            if (state.blocked) { if (!accept(state.blocked)) break; state.blocked = undefined; continue; }
            if (exports.flow_bridge_terminal_is_empty() === 1) { state.closed = true; state.closing = false; state.pages.clear(); resolve(); return; }
            const result = pollExact();
            if (result.length < 0) throw new Error("Flow closed before terminal-empty");
            if (result.length === 0) break;
            const message = decodeFlowMessage(result.bytes);
            if (!accept(message)) { state.blocked = message; break; }
          }
          schedule(drain);
        } catch (error) { state.closed = true; state.closing = false; reject(error); }
      };
      schedule(drain);
    });
    return state.closePromise;
  };
  return { state, start, cancel, closeHandle, close, terminalIsEmpty: () => exports.flow_bridge_terminal_is_empty() === 1 };
}

export async function createFlowFeatures(host) {
  const opened = host.start(FlowOperation.open);
  const openedReader = new Reader(await opened.result);
  const session = readHandle(openedReader);
  openedReader.finish();
  const feature = (name, args = {}) => mapTask(host.start(FlowOperation[name], args, session), name.startsWith("vcs") ? decodeFlowVcsPage : decodeJsonOutput);
  const groups = {};
  for (const [group, names] of Object.entries(FlowFeatureGroups)) {
    groups[group] = Object.fromEntries(names.map((name) => [name, (args) => feature(name, args)]));
  }
  groups.lifetime = { session, close: async () => { host.closeHandle(session); await host.close(); }, terminalIsEmpty: host.terminalIsEmpty };
  return groups;
}

let nextSurfaceId = 1;

export function attachFlowSurface(features, canvas, { width, height, dpr = 1, gpu = globalThis.navigator?.gpu } = {}) {
  if (!canvas) return rejectedTask(new Error("Flow canvas is required"));
  const surface = nextSurfaceId++;
  const surfaceGeneration = 1;
  const observers = new Set();
  let active;
  let cancelled = false;
  const notify = (event) => { for (const observer of observers) observer(event); };
  const attached = features.surface.attachSurface({ surface, surfaceGeneration, width, height, dpr });
  const unsubscribe = attached.subscribe(notify);
  const result = attached.result.then(async () => {
    if (cancelled) throw new Error("Flow surface attachment cancelled");
    try {
      const adapter = await gpu?.requestAdapter?.();
      if (!adapter) throw new Error("Flow GPU adapter unavailable");
      const device = await adapter.requestDevice();
      if (cancelled) throw new Error("Flow surface attachment cancelled");
      active = features.surface.surfaceStatus({ surface, surfaceGeneration, status: "created" });
      active.subscribe(notify);
      await active.result;
      device.lost?.then(() => {
        const lost = features.surface.surfaceStatus({ surface, surfaceGeneration, status: "device-lost" });
        const unsubscribeLost = lost.subscribe(notify);
        return lost.result.finally(unsubscribeLost);
      }).catch(() => {});
      return { surface, surfaceGeneration, canvas, device };
    } catch (error) {
      const status = cancelled ? "cancelled" : "rejected";
      active = features.surface.surfaceStatus({ surface, surfaceGeneration, status });
      active.subscribe(notify);
      await active.result.catch(() => {});
      throw error;
    } finally { unsubscribe(); }
  });
  return {
    requestId: attached.requestId,
    result,
    cancel() {
      if (cancelled) return false;
      cancelled = true;
      active?.cancel();
      attached.cancel();
      return true;
    },
    subscribe(observer) { observers.add(observer); return () => observers.delete(observer); },
  };
}

export function renderFlowSurface(features, canvas, render = renderFlowCanvas) {
  const task = features.surface.renderFrame({});
  return mapTask(task, (state) => { render(canvas, state); return state; });
}

export const FlowFeatureGroups = Object.freeze({
  document:Object.freeze(Object.keys(FlowOperation).slice(1,25)),interaction:Object.freeze(Object.keys(FlowOperation).slice(25,50)),editing:Object.freeze(Object.keys(FlowOperation).slice(50,75)),surface:Object.freeze(Object.keys(FlowOperation).slice(75,99)),drawing:Object.freeze(Object.keys(FlowOperation).slice(99)),
});

//#endregion 🌉️ReactiveHost

//#region 🧱️Codec

const encoder = new TextEncoder();
const decoder = new TextDecoder("utf-8", { fatal: true });

function encodeRequest(operation, requestId, generation, body) { return fields((w) => { w.u8(1); w.u8(1); w.u16(operation); w.u64(requestId); w.u32(generation); w.bytes(body); }); }
function encodeReply(requestId, generation, body) { return fields((w) => { w.u8(1); w.u8(2); w.u64(requestId); w.u32(generation); w.u16(0); w.u8(0); w.bytes(body); }); }
function encodeCancel(requestId, generation) { return fields((w) => { w.u8(1); w.u8(5); w.u8(1); w.u64(requestId); w.u32(generation); }); }
function encodeClose(handle) { return fields((w) => { w.u8(1); w.u8(5); w.u8(2); w.handle(handle); }); }
function encodeAcknowledge(handle, index) { return fields((w) => { w.u8(1); w.u8(5); w.u8(3); w.handle(handle); w.u32(index); }); }

export function decodeFlowMessage(bytes) {
  const reader = new Reader(bytes);
  if (reader.u8() !== 1) throw new Error("Flow ABI version");
  const tag = reader.u8();
  if (tag === 2) {
    const requestId = reader.u64(), generation = reader.u32(), status = reader.u16(), marker = reader.u8();
    let errorMessage = "";
    if (marker === 1) { reader.u16(); errorMessage = decoder.decode(reader.take(reader.u16())); } else if (marker !== 0) throw new Error("Flow reply error marker");
    const body = reader.bytes(); reader.finish(); return { tag, requestId, generation, status, errorMessage, body };
  }
  if (tag === 3) {
    const requestId = reader.u64(), generation = reader.u32(), sequence = reader.u32(), event = reader.u16(), status = reader.u16(), marker = reader.u8();
    if (marker === 1) { reader.u16(); reader.take(reader.u16()); } else if (marker !== 0) throw new Error("Flow event error marker");
    const body = reader.bytes(); reader.finish(); return { tag, requestId, generation, sequence, event, status, body };
  }
  if (tag === 4) { const handle = readHandle(reader), index = reader.u32(), body = reader.bytes(); reader.finish(); return { tag, handle, index, body }; }
  throw new Error("Unexpected Flow ABI tag");
}

class Writer {
  constructor() { this.values = []; }
  u8(value) { this.values.push(unsigned(value, 0xff, "u8")); }
  u16(value) { this.number(2, (v) => v.setUint16(0, value, true)); }
  u32(value) { this.number(4, (v) => v.setUint32(0, unsigned(value, 0xffff_ffff, "u32"), true)); }
  u64(value) { this.number(8, (v) => v.setBigUint64(0, unsigned64(value), true)); }
  f64(value) { if (!Number.isFinite(value)) throw new Error("Flow number must be finite"); this.number(8, (v) => v.setFloat64(0, value, true)); }
  bool(value) { this.u8(value ? 1 : 0); }
  bytes(value) { this.u32(value.length); this.values.push(...value); }
  text(value) { this.bytes(encoder.encode(value)); }
  optionalText(value) { if (value === undefined || value === null) this.u8(0); else { this.u8(1); this.text(required(value, "optional", "string")); } }
  handle(value) { this.u32(value.slot); this.u32(value.generation); }
  number(length, write) { const bytes = new Uint8Array(length); write(new DataView(bytes.buffer)); this.values.push(...bytes); }
  finish() { return Uint8Array.from(this.values); }
}

class Reader {
  constructor(value) { this.value = value; this.cursor = 0; }
  take(length) { const value = this.value.subarray(this.cursor, this.cursor + length); if (value.length !== length) throw new Error("Flow malformed length"); this.cursor += length; return value; }
  view(length) { const value = this.take(length); return new DataView(value.buffer, value.byteOffset, value.byteLength); }
  u8() { return this.take(1)[0]; }
  u16() { return this.view(2).getUint16(0, true); }
  u32() { return this.view(4).getUint32(0, true); }
  u64() { return this.view(8).getBigUint64(0, true); }
  bytes() { return this.take(this.u32()); }
  finish() { if (this.cursor !== this.value.length) throw new Error("Flow trailing bytes"); }
}

function fields(build) { const writer = new Writer(); build(writer); return writer.finish(); }
function encodeOperationArguments(operation, args) {
  const name = Object.keys(FlowOperation).find((candidate) => FlowOperation[candidate] === operation);
  const descriptor = FlowOperationFields[name] ?? "";
  return fields((writer) => {
    for (const field of descriptor.split(",").filter(Boolean)) {
      const [key, type] = field.split(":");
      const value = args[key];
      if (type === "s") writer.text(required(value, key, "string"));
      else if (type === "o") writer.optionalText(value);
      else if (type === "d") writer.f64(required(value, key, "number"));
      else if (type === "q") writer.u64(required(value, key, "number"));
      else if (type === "u") writer.u32(required(value, key, "number"));
      else if (type === "c") writer.u8(required(value, key, "number"));
      else if (type === "b") writer.bool(required(value, key, "boolean"));
      else if (type === "x") writer.bytes(ArrayBuffer.isView(value) ? new Uint8Array(value.buffer, value.byteOffset, value.byteLength) : Uint8Array.from(required(value, key, "object")));
      else throw new Error(`Flow argument type ${type} is unknown`);
    }
  });
}
function required(value, key, type) { if (value === undefined || value === null || typeof value !== type) throw new Error(`Flow argument ${key} must be ${type}`); return value; }
function unsigned(value, maximum, type) { if (!Number.isSafeInteger(value) || value < 0 || value > maximum) throw new Error(`Flow ${type} is out of range`); return value; }
function unsigned64(value) { const encoded = typeof value === "bigint" ? value : BigInt(unsigned(value, Number.MAX_SAFE_INTEGER, "u64")); if (encoded < 0n || encoded > 0xffff_ffff_ffff_ffffn) throw new Error("Flow u64 is out of range"); return encoded; }
function encodeHandle(handle) { return fields((writer) => writer.handle(handle)); }
function readHandle(reader) { return { slot: reader.u32(), generation: reader.u32() }; }
function handleKey(handle) { return `${handle.slot}:${handle.generation}`; }
function concat(...values) { const output = new Uint8Array(values.reduce((sum, value) => sum + value.length, 0)); let cursor = 0; for (const value of values) { output.set(value, cursor); cursor += value.length; } return output; }
function join(values, length) { const output = new Uint8Array(length); let cursor = 0; for (const value of values) { output.set(value, cursor); cursor += value.length; } return output; }
function decodeJsonOutput(bytes) { if (bytes.length === 0) return undefined; const text = decoder.decode(bytes); try { return JSON.parse(text); } catch { return text; } }
export function decodeFlowVcsPage(bytes) {
  const reader = new Reader(bytes);
  const value = {
    session: { slot: reader.u32(), generation: reader.u32() },
    requestGeneration: reader.u32(),
    authority: { sessionGeneration: reader.u32(), baseRevision: reader.u64(), parentRevision: reader.u64() },
    retainedOperation: { operation: reader.u64(), slot: reader.u8(), generation: reader.u32() },
    sequence: reader.u64(),
    operation: reader.u64(),
    sessionGeneration: reader.u32(),
    revision: reader.u64(),
    parentRevision: reader.u64(),
    documentGeneration: reader.u64(),
    widgetCount: reader.u32(),
    synapseCount: reader.u32(),
    layoutCount: reader.u32(),
    semanticDigest: reader.u64(),
  };
  reader.finish();
  return value;
}
function mapTask(task, decode) { return { ...task, result: task.result.then(decode) }; }
function rejectedTask(error) { return { requestId: undefined, result: Promise.reject(error), cancel: () => false, subscribe: () => () => {} }; }

function renderFlowCanvas(canvas, state) {
  const context = canvas?.getContext?.("2d");
  if (!context) return;
  const dpr = state?.dpr ?? 1;
  const width = Math.max(1, state?.width ?? canvas.clientWidth ?? 1);
  const height = Math.max(1, state?.height ?? canvas.clientHeight ?? 1);
  canvas.width = Math.round(width * dpr);
  canvas.height = Math.round(height * dpr);
  context.setTransform(dpr, 0, 0, dpr, 0, 0);
  context.clearRect(0, 0, width, height);
  for (const widget of state?.fixture?.widgets ?? []) {
    const x = widget.x ?? widget.position?.x ?? 0;
    const y = widget.y ?? widget.position?.y ?? 0;
    context.fillRect(x - 60, y - 24, 120, 48);
  }
}

//#endregion 🧱️Codec
