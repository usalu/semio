import { DEFAULT_PLAYGROUND_VARIANT } from "../../../../../🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";
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
  const descriptor = resolveWgpuBootDescriptor({ search: window.location.search, hash: window.location.hash, meta: documentBootMetaReader(document), defaultVariant: DEFAULT_PLAYGROUND_VARIANT });
  stripBootBrokerProof(window.location, window.history);
  const locale = Locale.fromLanguageTag(descriptor.locks.locale || navigator.language).id as "en" | "de";
  await mountWgpuBrowserHost(root, { descriptor, locale, mountId: "root", pageBindings: true });
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
