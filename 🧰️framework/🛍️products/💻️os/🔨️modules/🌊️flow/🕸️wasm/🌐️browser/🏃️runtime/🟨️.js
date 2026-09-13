//! 🌐️ Production Flow browser entry over the owned serialized Wasm ABI.

import { FlowFeatureGroups, FlowOperation, FlowOperationFields, attachFlowSurface, createFlowFeatures, createFlowHost, isFlowSessionOpenRejected, renderFlowSurface } from "../../🖥️host/🏃️runtime/🟨️.js";

//#region 🌐️BrowserConsumer

/** 🌐️ Opens the flow runtime over one wasm instance.
 *
 * `bindings` is the generated `flow_core.js` module namespace. It is separate from `source` because
 * `source` carries only the RAW wasm exports, which cannot take an `HtmlCanvasElement` — the
 * wasm-bindgen wrappers can, and that is how a flow surface reaches its own WebGPU presenter
 * (`flowAttachSurfaceCanvas`). Without it every surface simply presents through the encoded draw
 * list instead, which is also what a host with no WebGPU gets. */
export async function createFlowBrowserRuntime({ source, imports = {}, instantiate, bindings, ...hostOptions } = {}) {
  if (source === undefined) throw new Error("Flow Wasm source is required");
  let exports = source?.exports ?? source;
  let surfaceBindings = bindings;
  if (typeof exports?.flow_bridge_allocate !== "function") {
    if (instantiate) {
      let bytes = source?.module_or_path ?? source;
      if (typeof bytes === "string" || bytes instanceof URL) bytes = await fetch(bytes);
      if (bytes instanceof Response) bytes = await bytes.arrayBuffer();
      const instantiated = await instantiate(bytes, imports);
      exports = (instantiated?.instance ?? instantiated)?.exports;
    } else {
      if (Reflect.ownKeys(imports).length !== 0) throw new Error("custom Flow imports require their exact embedding initializer");
      const core = await import("../../../🫀️core/🕸️bindings/flow_core.js");
      surfaceBindings ??= core;
      exports = await core.default({ module_or_path: source?.module_or_path ?? source });
    }
  }
  const memory = exports?.memory;
  if (!exports || !(memory instanceof WebAssembly.Memory)) throw new Error("Flow Wasm instance must export memory");
  const host = createFlowHost({ ...hostOptions, exports, memory });
  const sessions = new Set();
  let closePromise;
  let closing = false;
  return Object.freeze({
    openSession() {
      if (closing) throw new Error("Flow runtime is closed");
      const session = new FlowSession(sessionAuthority, createFlowFeatures(host), () => sessions.delete(session), surfaceBindings);
      sessions.add(session);
      return session;
    },
    close() {
      if (closePromise) return closePromise;
      closing = true;
      closePromise = Promise.allSettled([...sessions].map((session) => session.close())).then(async (results) => {
        await host.close();
        sessions.clear();
        const failure = results.find((result) => result.status === "rejected");
        if (failure) throw failure.reason;
      });
      return closePromise;
    },
    terminalIsEmpty: () => closing && sessions.size === 0 && host.terminalIsEmpty(),
  });
}

const invokeFeature = Symbol("FlowSession.invokeFeature");
const sessionAuthority = Symbol("FlowSession.owner");

export class FlowSession {
  #ready;
  #closed = false;
  #closePromise;
  #release;
  #released = false;
  #bindings;

  constructor(authority, ready, release, bindings) {
    if (authority !== sessionAuthority) throw new Error("Flow sessions require their runtime owner");
    this.#release = release;
    this.#bindings = bindings;
    this.#ready = Promise.resolve(ready).catch((error) => {
      if (isFlowSessionOpenRejected(error)) this.#releaseOnce();
      throw error;
    });
  }

  [invokeFeature](name, args) {
    return deferredFlowTask(this.#ready, (features) => {
      if (this.#closed) throw new Error("Flow session is closed");
      const group = Object.keys(FlowFeatureGroups).find((candidate) => FlowFeatureGroups[candidate].includes(name));
      return features[group][name](args);
    });
  }

  attachCanvas(canvas, width, height, dpr) {
    return deferredFlowTask(this.#ready, (features) => {
      if (this.#closed) throw new Error("Flow session is closed");
      return attachFlowSurface(features, canvas, { width, height, dpr, bindings: this.#bindings });
    });
  }

  renderCanvas(canvas) {
    return deferredFlowTask(this.#ready, (features) => {
      if (this.#closed) throw new Error("Flow session is closed");
      return renderFlowSurface(features, canvas);
    });
  }

  close() {
    if (this.#closePromise) return this.#closePromise;
    this.#closed = true;
    this.#closePromise = this.#ready.then((features) => features.lifetime.close()).then(() => this.#releaseOnce());
    return this.#closePromise;
  }

  #releaseOnce() {
    if (this.#released) return;
    this.#released = true;
    this.#release();
  }

  free() { return this.close(); }
  [Symbol.dispose]() { void this.close(); }
}

function deferredFlowTask(ready, start) {
  let active;
  let cancelled = false;
  let unsubscribe = () => {};
  const observers = new Set();
  const result = ready.then((features) => {
    if (cancelled) throw new Error("Flow feature cancelled before admission");
    active = start(features);
    unsubscribe = active.subscribe((event) => { for (const observer of observers) observer(event); });
    return active.result;
  }).finally(() => unsubscribe());
  return {
    result,
    cancel() {
      if (cancelled) return false;
      cancelled = true;
      return active ? active.cancel() : true;
    },
    subscribe(observer) { observers.add(observer); return () => observers.delete(observer); },
  };
}

for (const name of Object.keys(FlowOperation).slice(1)) {
  if (name === "attachSurface" || name === "renderFrame") continue;
  Object.defineProperty(FlowSession.prototype, name, {
    value(...values) {
      const keys = (FlowOperationFields[name] ?? "").split(",").filter(Boolean).map((field) => field.split(":")[0]);
      const args = values.length === 1 && values[0] && typeof values[0] === "object" && !ArrayBuffer.isView(values[0]) ? values[0] : Object.fromEntries(keys.map((key, index) => [key, values[index]]));
      return this[invokeFeature](name, args);
    },
    configurable: false,
    enumerable: false,
  });
}

//#endregion 🌐️BrowserConsumer
