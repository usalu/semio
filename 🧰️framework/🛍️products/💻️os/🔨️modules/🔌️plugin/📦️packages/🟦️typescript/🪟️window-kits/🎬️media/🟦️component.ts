// #region 🎬️MediaWindowKit
/// <reference types="vitest/importMeta" />
/** @emoji 🎬️ `@semio-tech/plugin-window-kits` — TS twin of Rust `MediaWindowKit` (`framework.window.media`). */
import type { UiKeyValueNode, UiNode } from "@semio-tech/framework";

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

/** 🎬️ Twin of Rust `MediaWindowKit::render` — duration/position/kind as a read-only key-value list. */
export function renderMedia(view: MediaView): UiNode {
  const node: UiKeyValueNode = {
    type: "keyValue",
    entries: [
      { label: "Duration", value: String(view.durationMs) },
      { label: "Position", value: String(view.positionMs) },
      { label: "Kind", value: view.kind },
    ],
  };
  return node;
}

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("renderMedia", () => {
    it("renders duration, position, and kind as key-value entries", () => {
      const node = renderMedia({ durationMs: 60_000, positionMs: 1_500, kind: "video" });
      if (node.type !== "keyValue") throw new Error("expected keyValue");
      expect(node.entries.map((entry) => entry.value)).toEqual(["60000", "1500", "video"]);
    });
  });
}
//#endregion 🧪️Tests
// #endregion 🎬️MediaWindowKit
