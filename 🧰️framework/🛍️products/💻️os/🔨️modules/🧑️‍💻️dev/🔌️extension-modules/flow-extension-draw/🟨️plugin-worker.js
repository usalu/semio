/** @generated semio plugin web worker */
let pluginApi = null;

async function loadPlugin(moduleUrl) {
  if (pluginApi) return pluginApi;
  const module = await import(moduleUrl);
  if (module.createPluginApi) {
    pluginApi = await module.createPluginApi();
    return pluginApi;
  }
  throw new Error("plugin module missing createPluginApi export");
}

function reply(requestId, type, payload) {
  self.postMessage({ requestId, type, ...payload });
}

function replyError(requestId, message) {
  self.postMessage({ requestId, type: "error", message });
}

self.addEventListener("message", async (event) => {
  const msg = event.data ?? {};
  const { type, requestId } = msg;
  // 🔗️ Backbone relay passthrough (main thread ⇄️ host-shim): inbound messages from the sync actor
  // (`🟦️backbone-🟦️worker.ts`) land in the shared queue the host-shim's `backbonePoll` drains; the shim's
  // `backboneSend` posts `backboneOutbound` straight up to the main thread, so there is nothing to do
  // for it here. These carry no requestId, so they must be handled before the request/response guard.
  if (type === "backboneInbound") {
    const queues = (globalThis.__semioBackboneInbound ??= new Map());
    const queue = queues.get(msg.uri) ?? [];
    for (const message of msg.messages ?? []) queue.push(message);
    queues.set(msg.uri, queue);
    return;
  }
  if (!requestId || !type) return;
  try {
    if (type === "init") {
      // 🪶️ GUESTSLIM: bytes forwarded from the main thread's `acquirePluginModule` fetch (a worker
      // never owns fetch itself); `readAsset` in `🟨️host-shim.js` reads from this global.
      if (msg.guestSlimAssets) {
        globalThis.__semioGuestSlimAssets = new Map(msg.guestSlimAssets.map(([handle, buffer]) => [handle, new Uint8Array(buffer)]));
      }
      await loadPlugin(msg.moduleUrl);
      reply(requestId, "init", { ok: true });
      return;
    }
    const api = pluginApi;
    if (!api) throw new Error("worker not initialized");
    switch (type) {
      case "manifest":
        reply(requestId, "manifest", { value: await api.manifest() });
        break;
      case "createApp":
        reply(requestId, "createApp", { instanceId: await api.createApp(msg.appId) });
        break;
      case "destroy":
        await api.destroyApp?.(msg.instanceId);
        reply(requestId, "destroy", { ok: true });
        break;
      case "exchange":
        reply(requestId, "exchange", { value: await api.exchange(msg.instanceId, msg.frames) });
        break;
      default:
        throw new Error(`unknown worker message type: ${type}`);
    }
  } catch (error) {
    const payload = error && typeof error === "object" && "payload" in error ? error.payload : undefined;
    const detail = payload !== undefined ? ` payload=${(() => { try { return JSON.stringify(payload); } catch { return String(payload); } })()}` : "";
    replyError(requestId, (error instanceof Error ? error.message : String(error)) + detail);
  }
});
