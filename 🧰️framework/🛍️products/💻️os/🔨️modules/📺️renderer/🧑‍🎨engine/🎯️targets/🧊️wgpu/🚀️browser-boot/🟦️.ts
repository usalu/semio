import { admitPluginCatalogV1 } from "../../../../../🔌️plugin/📇️registry/🟦️.ts";
import { documentBootMetaReader, resolveWgpuBootDescriptor, stripBootBrokerProof, wgpuReadinessBeacon, WGPU_READINESS_BEACON_UNKNOWN_PLUGIN } from "../🧭️boot-descriptor/🟦️.ts";
import { mountWgpuBrowserHost } from "../🌐️browser-host/🟦️.ts";
import { Locale } from "../../../../../../../../🔨️modules/🖱️ui/🌐️locale/🟦️.ts";

export { WGPU_CANVAS_ID, WGPU_INTROSPECTION_GLOBAL, WGPU_HUB_PROJECTION_GLOBAL, WGPU_ACCESSIBILITY_MIRROR_ID } from "../🌐️browser-host/🟦️.ts";

await new Promise<void>(resolve => {
  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", () => resolve(), { once: true });
  else resolve();
});
const root = document.getElementById("root");
if (!root) throw new Error("missing-root: #root is unavailable");
try {
  const inventory = document.getElementById("semio-plugin-catalog");
  const text = inventory?.textContent;
  if (!text || text.length > 2097152) throw new Error("plugin-catalog-invalid: missing or oversized supplied inventory");
  const catalog = admitPluginCatalogV1(JSON.parse(text), { maxBytes: 2097152, maxRows: 128, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => false, progress: () => {} });
  const descriptor = resolveWgpuBootDescriptor({ search: window.location.search, hash: window.location.hash, meta: documentBootMetaReader(document), defaultVariant: "" });
  stripBootBrokerProof(window.location, window.history);
  const locale = Locale.fromLanguageTag(descriptor.locks.locale || navigator.language).id as "en" | "de";
  await mountWgpuBrowserHost(root, { catalog, descriptor, pluginRegistrySelection: descriptor.pluginVariant ? "variant" : "all", locale, mountId: "root", pageBindings: true });
} catch (error) {
  const detail = error instanceof Error ? error.message : String(error);
  wgpuReadinessBeacon(document.documentElement, WGPU_READINESS_BEACON_UNKNOWN_PLUGIN).error();
  if (!root.querySelector('[role="alert"]')) {
    const banner = document.createElement("div");
    banner.setAttribute("role", "alert");
    banner.textContent = detail;
    root.append(banner);
    console.error("wgpu renderer fault: worker-boot-failed: " + detail);
  }
  throw error;
}
