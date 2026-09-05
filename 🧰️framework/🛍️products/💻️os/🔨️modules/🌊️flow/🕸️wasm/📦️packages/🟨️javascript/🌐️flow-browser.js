//! 🌐️ Production Flow browser entry over the owned serialized Wasm ABI.

import { FlowFeatureGroups, FlowOperation, FlowOperationFields, attachFlowSurface, createFlowFeatures, createFlowHost, renderFlowSurface } from "./🖥️flow-host.js";

//#region 🌐️BrowserConsumer

let defaultHost;

export async function createFlowBrowserFeatures({ source, imports = {}, instantiate, ...hostOptions } = {}) {
  if (source === undefined) throw new Error("Flow Wasm source is required");
  let exports = source?.exports ?? source;
  if (typeof exports?.flow_bridge_allocate !== "function") {
    if (instantiate) {
      let bytes = source?.module_or_path ?? source;
      if (typeof bytes === "string" || bytes instanceof URL) bytes = await fetch(bytes);
      if (bytes instanceof Response) bytes = await bytes.arrayBuffer();
      const instantiated = await instantiate(bytes, imports);
      exports = (instantiated?.instance ?? instantiated)?.exports;
    } else {
      if (Reflect.ownKeys(imports).length !== 0) throw new Error("custom Flow imports require their exact embedding initializer");
      const { default: initialize } = await import("../../../🫀️core/🕸️bindings/flow_core.js");
      exports = await initialize({ module_or_path: source?.module_or_path ?? source });
    }
  }
  const memory = exports?.memory;
  if (!exports || !(memory instanceof WebAssembly.Memory)) throw new Error("Flow Wasm instance must export memory");
  const host = createFlowHost({ ...hostOptions, exports, memory });
  return { host, features: await createFlowFeatures(host), exports };
}

export default async function init(source) {
  const initialized = await createFlowBrowserFeatures({ source });
  defaultHost = initialized.host;
  return initialized.exports;
}

const invokeFeature = Symbol("FlowSession.invokeFeature");

export class FlowSession {
  constructor() {
    if (!defaultHost) throw new Error("Flow browser ABI must be initialized before opening a session");
    this.ready = createFlowFeatures(defaultHost);
  }

  [invokeFeature](name, args) {
    return deferredFlowTask(this.ready, (features) => {
      const group = Object.keys(FlowFeatureGroups).find((candidate) => FlowFeatureGroups[candidate].includes(name));
      return features[group][name](args);
    });
  }

  attachCanvas(canvas, width, height, dpr) {
    return deferredFlowTask(this.ready, (features) => attachFlowSurface(features, canvas, { width, height, dpr }));
  }

  renderCanvas(canvas) {
    return deferredFlowTask(this.ready, (features) => renderFlowSurface(features, canvas));
  }

  async close() {
    const features = await this.ready;
    await features.lifetime.close();
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
