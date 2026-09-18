import { describe, expect, it } from "vitest";
import { WGPU_READINESS_BEACON_UNKNOWN_PLUGIN, wgpuReadinessBeacon } from "../../🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts";

/** @emoji 🚦️ LAW: the wgpu page publishes React's own readiness beacon — `data-semio-os-ready` /
 * `data-semio-os-error` on the document element, each carrying the plugin id and each clearing the
 * other — so one probe, one e2e runner and one CI gate work unchanged on both renderers.
 *
 * 🩸️ Packet W6a item 5: the wgpu page set neither attribute, so a 45 s CLEAN boot still read
 * `data-semio-os-ready = null` (`📓️audit-visual-parity-puzzle3d.md` §7 item 8) and every reader had
 * to fall back to a fixed sleep on this target alone.
 *
 * 🪟️ The beacon is exercised through the same exported function `🚀️browser-boot/🟦️.ts` calls at its
 * `onReady` / `onFault` / `pagehide` milestones. That module cannot be imported by a suite — it mounts
 * the shell on import, at top level — which is exactly why the beacon lives in the pure boot-descriptor
 * module beside the page realm's other reads. */
function beaconRoot(): HTMLElement {
  const dataset: Record<string, string> = {};
  return { dataset } as unknown as HTMLElement;
}

describe("🔖️ the wgpu readiness beacon", () => {
  it("publishes React's own two attributes, each clearing the other", () => {
    const root = beaconRoot();
    const beacon = wgpuReadinessBeacon(root, "puzzle3d");

    expect(root.dataset.semioOsReady).toBeUndefined();
    expect(root.dataset.semioOsError).toBeUndefined();

    beacon.ready();
    expect(root.dataset.semioOsReady).toBe("puzzle3d");
    expect(root.dataset.semioOsError).toBeUndefined();

    beacon.error();
    expect(root.dataset.semioOsError).toBe("puzzle3d");
    expect(root.dataset.semioOsReady).toBeUndefined();

    beacon.ready();
    expect(root.dataset.semioOsReady).toBe("puzzle3d");
    expect(root.dataset.semioOsError).toBeUndefined();

    beacon.clear();
    expect(root.dataset.semioOsReady).toBeUndefined();
    expect(root.dataset.semioOsError).toBeUndefined();
  });

  it("carries the plugin id, and React's own `unknown` when no descriptor resolved", () => {
    const named = beaconRoot();
    wgpuReadinessBeacon(named, "generation3d").ready();
    expect(named.dataset.semioOsReady).toBe("generation3d");

    const anonymous = beaconRoot();
    wgpuReadinessBeacon(anonymous, WGPU_READINESS_BEACON_UNKNOWN_PLUGIN).error();
    expect(anonymous.dataset.semioOsError).toBe("unknown");
    expect(WGPU_READINESS_BEACON_UNKNOWN_PLUGIN).toBe("unknown");
  });
});
