// #region 🎬️MediaWindowKit
/// <reference types="vitest/importMeta" />
/** @emoji 🎬️ `@semio-tech/plugin-window-kits` — TS twin of Rust `MediaWindowKit` (`framework.window.media`).
 * ⚠️ The Rust twin is deliberately still on the old `ui_wgpu::wgpu::UiNode` return type this wave (see
 * `🔌️plugin/🦀️component.rs` `#region 🔖️WindowKits`'s own doc comment, ticket
 * SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY packet `sdk-helpers`) — this file gets ahead of it onto the
 * new semantic contract (`BuiltNode`) since nothing currently constrains the two to match
 * simultaneously and no production code calls `renderMedia` yet; re-verify parity once Rust migrates. */
import type { BuiltNode, Component, LayoutSpec, StyleSpec, AccessibilitySpec } from "@semio-tech/framework";

/** 🆔️ Frozen kind id — twin of Rust `MediaWindowKit::KIND_ID`. */
export const MEDIA_WINDOW_KIND_ID = "framework.window.media";

/** 🎬️ Twin of Rust `MediaKind`. */
export type MediaKind = "audio" | "video";

/** 🎬️ Audio/video transport state — duration/position in milliseconds, no playback engine — twin of Rust `MediaView`. */
export type MediaView = {
  readonly durationMs: number;
  readonly positionMs: number;
  readonly kind: MediaKind;
};

const DEFAULT_LAYOUT: LayoutSpec = { kind: "leaf", width: "hug", height: "hug" };
const DEFAULT_STYLE: StyleSpec = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
const DEFAULT_ACCESSIBILITY: AccessibilitySpec = { label: null, description: null, live: "off", shortcut: null, hidden: false };

/** 🧱️ Stamps a leaf {@link BuiltNode} from `component`, filling every other field with the shared defaults. */
function leafNode(key: string, component: Component): BuiltNode {
  return { key, component, layout: DEFAULT_LAYOUT, style: DEFAULT_STYLE, activity: "idle", disabled: false, accessibility: DEFAULT_ACCESSIBILITY, bindings: [], menu: null, children: [] };
}

/** 🎬️ Twin of Rust `MediaWindowKit::render` — duration/position/kind as a read-only key-value list. */
export function renderMedia(view: MediaView): BuiltNode {
  return leafNode(MEDIA_WINDOW_KIND_ID, {
    type: "keyValueList",
    entries: [
      { label: "Duration", value: String(view.durationMs) },
      { label: "Position", value: String(view.positionMs) },
      { label: "Kind", value: view.kind },
    ],
  });
}

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("renderMedia", () => {
    it("renders duration, position, and kind as key-value entries", () => {
      const node = renderMedia({ durationMs: 60_000, positionMs: 1_500, kind: "video" });
      if (node.component.type !== "keyValueList") throw new Error("expected keyValueList");
      expect(node.component.entries.map((entry) => entry.value)).toEqual(["60000", "1500", "video"]);
    });
  });
}
//#endregion 🧪️Tests
// #endregion 🎬️MediaWindowKit
