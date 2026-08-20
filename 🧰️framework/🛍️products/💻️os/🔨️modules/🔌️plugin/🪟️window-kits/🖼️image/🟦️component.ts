// #region 🖼️ImageWindowKit
/// <reference types="vitest/importMeta" />
/** @emoji 🖼️ `@semio-tech/plugin-window-kits` — TS twin of Rust `ImageWindowKit` (`framework.window.image`).
 * ⚠️ The Rust twin (`🔌️plugin/🦀️component.rs` `#region 🔖️WindowKits`) is deliberately still on the old
 * `ui_wgpu::wgpu::UiNode` return type this wave (see its own doc comment, ticket
 * SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY packet `sdk-helpers`) — this file gets ahead of it onto
 * the new semantic contract (`BuiltNode`) since nothing currently constrains the two to match
 * simultaneously and no production code calls `renderImage` yet; re-verify parity once Rust migrates. */
import type { BuiltNode, Component, LayoutSpec, StyleSpec, AccessibilitySpec } from "@semio-tech/framework";

/** 🆔️ Frozen kind id — twin of Rust `ImageWindowKit::KIND_ID`. */
export const IMAGE_WINDOW_KIND_ID = "framework.window.image";

/** 🖼️ Raw pixel payload as a base64 blob — twin of Rust `ImageView`. */
export type ImageView = {
  readonly width: number;
  readonly height: number;
  readonly mime: string;
  readonly base64: string;
};

const DEFAULT_LAYOUT: LayoutSpec = { kind: "leaf", width: "hug", height: "hug" };
const DEFAULT_STYLE: StyleSpec = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
const DEFAULT_ACCESSIBILITY: AccessibilitySpec = { label: null, description: null, live: "off", shortcut: null, hidden: false };

/** 🧱️ Stamps a leaf {@link BuiltNode} from `component`, filling every other field with the shared defaults. */
function leafNode(key: string, component: Component): BuiltNode {
  return { key, component, layout: DEFAULT_LAYOUT, style: DEFAULT_STYLE, activity: "idle", disabled: false, accessibility: DEFAULT_ACCESSIBILITY, bindings: [], menu: null, children: [] };
}

/** 🖼️ Twin of Rust `ImageWindowKit::render` — encodes `view` into a base64 data URI image node. */
export function renderImage(view: ImageView): BuiltNode {
  return leafNode(IMAGE_WINDOW_KIND_ID, { type: "image", src: `data:${view.mime};base64,${view.base64}`, alt: `${view.width}x${view.height}` });
}

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("renderImage", () => {
    it("builds a base64 data URI from mime + base64", () => {
      const node = renderImage({ width: 4, height: 2, mime: "image/png", base64: "QUJD" });
      if (node.component.type !== "image") throw new Error("expected image");
      expect(node.component.src).toBe("data:image/png;base64,QUJD");
    });
  });
}
//#endregion 🧪️Tests
// #endregion 🖼️ImageWindowKit
