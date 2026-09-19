// #region 🧲️Header
/** @emoji 🧪️ Breakpoint-policy laws, plus the byte-parity gate that reads the wgpu dock's OWN Rust
 * constants off disk — a policy "shared" only by a comment is a policy that drifts. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { describe, expect, it } from "vitest";
import {
  UI_MOBILE_MAX_WIDTH_PX,
  UI_MOBILE_MEDIA_QUERY,
  UI_TABLET_MAX_WIDTH_PX,
  UI_TABLET_MEDIA_QUERY,
  elementsSurfaceDeviceForMatches,
  elementsSurfaceDeviceForWidth,
  elementsSurfaceDeviceIsMobile,
  elementsSurfaceDeviceSupportsTabDrag,
} from "../../🟦️.ts";
// #endregion 🔌️Adapters

/** 🗂️ The repo root, walked up from the runner's cwd — `import.meta.url` is a vite `/@fs` URL under this
 * suite's transform, so it cannot be turned into a path here. */
function repoRoot(): string {
  let at = process.cwd();
  while (!existsSync(resolve(at, "nx.json"))) {
    const up = dirname(at);
    if (up === at) throw new Error("🗂️ no nx.json above the test runner's cwd");
    at = up;
  }
  return at;
}

const DOCK_WGPU_SOURCE = resolve(repoRoot(), "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs");

//#region 📱️Policy
describe("📱️ breakpoint policy", () => {
  it("names a phone, a tablet and a desktop at the exact inclusive boundaries", () => {
    expect(elementsSurfaceDeviceForWidth(320)).toBe("mobile");
    expect(elementsSurfaceDeviceForWidth(UI_MOBILE_MAX_WIDTH_PX)).toBe("mobile");
    expect(elementsSurfaceDeviceForWidth(UI_MOBILE_MAX_WIDTH_PX + 1)).toBe("tablet");
    expect(elementsSurfaceDeviceForWidth(UI_TABLET_MAX_WIDTH_PX)).toBe("tablet");
    expect(elementsSurfaceDeviceForWidth(UI_TABLET_MAX_WIDTH_PX + 1)).toBe("desktop");
    expect(elementsSurfaceDeviceForWidth(2560)).toBe("desktop");
  });

  it("falls back to the widest layout when the width itself is missing", () => {
    expect(elementsSurfaceDeviceForWidth(Number.NaN)).toBe("desktop");
    expect(elementsSurfaceDeviceForWidth(Number.POSITIVE_INFINITY)).toBe("desktop");
  });

  it("the tablet query is a BAND, so a phone width can never satisfy both queries", () => {
    expect(UI_MOBILE_MEDIA_QUERY).toBe("(max-width: 767px)");
    expect(UI_TABLET_MEDIA_QUERY).toBe("(min-width: 768px) and (max-width: 1023px)");
    expect(UI_TABLET_MEDIA_QUERY).toContain(`min-width: ${UI_MOBILE_MAX_WIDTH_PX + 1}px`);
  });

  it("resolves already-evaluated matches, with mobile winning an overlapping pair", () => {
    expect(elementsSurfaceDeviceForMatches({ mobile: true, tablet: false })).toBe("mobile");
    expect(elementsSurfaceDeviceForMatches({ mobile: false, tablet: true })).toBe("tablet");
    expect(elementsSurfaceDeviceForMatches({ mobile: false, tablet: false })).toBe("desktop");
    expect(elementsSurfaceDeviceForMatches({ mobile: true, tablet: true })).toBe("mobile");
  });

  it("only a phone collapses the dock anchors and loses tab drag", () => {
    expect(elementsSurfaceDeviceIsMobile("mobile")).toBe(true);
    expect(elementsSurfaceDeviceIsMobile("tablet")).toBe(false);
    expect(elementsSurfaceDeviceIsMobile("desktop")).toBe(false);
    expect(elementsSurfaceDeviceSupportsTabDrag("mobile")).toBe(false);
    expect(elementsSurfaceDeviceSupportsTabDrag("tablet")).toBe(true);
    expect(elementsSurfaceDeviceSupportsTabDrag("desktop")).toBe(true);
  });
});
//#endregion 📱️Policy

//#region 🧊️WgpuParity
describe("📱️ wgpu dock breakpoint parity", () => {
  /** 🧊️ The wgpu shell compares its own `screen_w` against these literals. If they ever stop matching
   * this module's, the two renderers paint different layouts at the same width — and nothing else in
   * the tree would catch it, because neither side imports the other. */
  it("the wgpu dock declares byte-identical thresholds", () => {
    const source = readFileSync(DOCK_WGPU_SOURCE, "utf8");
    const mobile = /pub const MODE_DOCK_MOBILE_MAX_WIDTH_PX: f32 = ([0-9.]+);/.exec(source);
    const tablet = /pub const MODE_DOCK_TABLET_MAX_WIDTH_PX: f32 = ([0-9.]+);/.exec(source);
    expect(mobile, "🧊️ the wgpu dock no longer declares MODE_DOCK_MOBILE_MAX_WIDTH_PX").toBeTruthy();
    expect(tablet, "🧊️ the wgpu dock no longer declares MODE_DOCK_TABLET_MAX_WIDTH_PX").toBeTruthy();
    expect(Number.parseFloat(mobile![1]!)).toBe(UI_MOBILE_MAX_WIDTH_PX);
    expect(Number.parseFloat(tablet![1]!)).toBe(UI_TABLET_MAX_WIDTH_PX);
  });

  it("the wgpu dock carries a device resolver with the same inclusive-maximum shape", () => {
    const source = readFileSync(DOCK_WGPU_SOURCE, "utf8");
    expect(source).toContain("pub fn mode_dock_device_for_width(width_px: f32) -> &'static str");
    expect(source).toContain("width_px <= MODE_DOCK_MOBILE_MAX_WIDTH_PX");
    expect(source).toContain("width_px <= MODE_DOCK_TABLET_MAX_WIDTH_PX");
  });
});
//#endregion 🧊️WgpuParity
