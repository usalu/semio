/** @generated semio plugin jco component bridge */
import { plugin } from "./semio_s_plugin_flow_extension_text_component.js";

const apps = new Set();
let tail = Promise.resolve();
let pluginApiPromise = null;

function runSerialized(fn) {
  const job = tail.then(async () => {
    for (let attempt = 0; attempt < 8; attempt += 1) {
      try {
        return await fn();
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        const payload = error && typeof error === "object" && "payload" in error ? error.payload : undefined;
        const detail = payload !== undefined ? `${message} payload=${(() => { try { return JSON.stringify(payload); } catch { return String(payload); } })()}` : message;
        const busy = detail.includes("plugin instance busy") || detail.includes("plugin busy");
        const trapped = detail.includes("unreachable") || /trap|panicked/i.test(detail);
        if (busy || trapped) {
          try { plugin.clearInstanceGuard?.(); } catch { /* guard heal is best-effort */ }
        }
        if (!busy) throw error;
        await new Promise((resolve) => setTimeout(resolve, attempt + 1));
      }
    }
    return fn();
  }, async () => fn());
  tail = job.then(
    () => undefined,
    () => undefined,
  );
  return job;
}

async function createPluginApiInner() {
  const core = {
    async manifest() {
      return await plugin.manifest();
    },
    async createApp(appId) {
      // 🐚️ A random instance id (not `appId` itself) so two shells sharing this worker's plugin module
      // (see acquirePluginModule in framework/core) can each instantiate the same app without colliding
      // on the guest's instance-id-keyed `INSTANCES` table.
      const instanceId = await plugin.instantiateApp(appId, crypto.randomUUID());
      apps.add(instanceId);
      return instanceId;
    },
    async destroyApp(instanceId) {
      apps.delete(instanceId);
    },
    async exchange(instanceId, frames) {
      if (!apps.has(instanceId)) throw new Error(`unknown instance: ${instanceId}`);
      return await plugin.exchange(instanceId, frames);
    },
  };
  return {
    manifest: () => runSerialized(() => core.manifest()),
    createApp: (appId) => runSerialized(() => core.createApp(appId)),
    destroyApp: (instanceId) => runSerialized(() => core.destroyApp(instanceId)),
    exchange: (instanceId, frames) => runSerialized(() => core.exchange(instanceId, frames)),
  };
}

export async function createPluginApi() {
  if (!pluginApiPromise) pluginApiPromise = createPluginApiInner();
  return pluginApiPromise;
}
