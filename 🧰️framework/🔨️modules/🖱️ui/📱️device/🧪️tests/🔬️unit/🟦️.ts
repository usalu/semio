// #region 🧲️Header
/** @emoji 🧪️ Breakpoint-policy laws, plus the byte-parity gate that reads the wgpu dock's OWN Rust
 * constants off disk — a policy "shared" only by a comment is a policy that drifts. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { describe, expect, it } from "vitest";
import {
  UI_AVAILABLE_HEIGHT,
  UI_MOBILE_MAX_WIDTH_PX,
  UI_MOBILE_MEDIA_QUERY,
  UI_TABLET_MAX_WIDTH_PX,
  UI_TABLET_MEDIA_QUERY,
  availableViewportHeightPx,
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
const CSS_PATH = '🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css';
const HOST_PATH = '🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts';

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

//#region 📐 Available viewport
describe("📐 available viewport height", () => {
  it("uses the visible viewport when the screen is taller than what the browser toolbar leaves", () => {
    expect(availableViewportHeightPx({ innerHeight: 800, visualHeight: 640 })).toBe(640);
    expect(availableViewportHeightPx({ innerHeight: 800, visualHeight: 640.4 })).toBe(640);
  });

  it("falls back to the screen when the visible viewport is missing", () => {
    expect(availableViewportHeightPx({ innerHeight: 800, visualHeight: null })).toBe(800);
    expect(availableViewportHeightPx({ innerHeight: 800, visualHeight: 0 })).toBe(800);
    expect(availableViewportHeightPx({ innerHeight: 800 })).toBe(800);
    expect(availableViewportHeightPx({ innerHeight: 700, visualHeight: Number.NaN })).toBe(700);
  });

  it("returns zero when neither reading is a usable height", () => {
    expect(availableViewportHeightPx({ innerHeight: 0, visualHeight: 0 })).toBe(0);
    expect(availableViewportHeightPx({ innerHeight: Number.POSITIVE_INFINITY })).toBe(0);
  });

  it("names the CSS length shells use in place of 100vh", () => {
    expect(UI_AVAILABLE_HEIGHT).toBe("var(--ui-available-height, 100dvh)");
  });

  it("the stylesheet and the host boot script both size shells to that available height", () => {
    const root = repoRoot();
    const css = readFileSync(resolve(root, CSS_PATH), "utf8");
    expect(css).toContain("--ui-available-height: 100dvh");
    expect(css).toContain(".h-screen {\n  height: var(--ui-available-height, 100dvh);");
    const host = readFileSync(resolve(root, HOST_PATH), "utf8");
    const script = /export const PLAYGROUND_PLAY_BOOT_VIEWPORT_SCRIPT = `([\s\S]*?)`;/.exec(host);
    expect(script, "host boot viewport script").toBeTruthy();
    const style = documentFromScript(script![1]!, { innerHeight: 800, visualHeight: 640 });
    expect(style).toBe("640px");
    expect(style).toBe(`${availableViewportHeightPx({ innerHeight: 800, visualHeight: 640 })}px`);
  });
});

/** 📐 Runs the host boot script against a fake window and returns the height it published. */
function documentFromScript(script: string, reading: { readonly innerHeight: number; readonly visualHeight: number }): string {
  const props: Record<string, string> = {};
  const window = {
    innerHeight: reading.innerHeight,
    visualViewport: { height: reading.visualHeight, addEventListener() {} },
    addEventListener() {},
  };
  const document = { documentElement: { style: { getPropertyValue(name: string) { return props[name] ?? ""; }, setProperty(name: string, value: string) { props[name] = value; } } } };
  new Function("window", "document", script)(window, document);
  return props["--ui-available-height"] ?? "";
}
// #endregion 📐 Available viewport
