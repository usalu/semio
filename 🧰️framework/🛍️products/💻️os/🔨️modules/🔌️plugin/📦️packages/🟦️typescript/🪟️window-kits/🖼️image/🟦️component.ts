// #region 🖼️ImageWindowKit
/// <reference types="vitest/importMeta" />
/** @emoji 🖼️ `@semio-tech/plugin-window-kits` — TS twin of Rust `ImageWindowKit` (`framework.window.image`). */
import type { UiImageNode, UiNode } from "@semio-tech/framework";

/** 🆔️ Frozen kind id — twin of Rust `ImageWindowKit::KIND_ID`. */
export const IMAGE_WINDOW_KIND_ID = "framework.window.image";

/** 🖼️ Raw pixel payload as a base64 blob — twin of Rust `ImageView`. */
export type ImageView = {
  readonly width: number;
  readonly height: number;
  readonly mime: string;
  readonly base64: string;
};

/** 🖼️ Twin of Rust `ImageWindowKit::render` — encodes `view` into a base64 data URI image node. */
export function renderImage(view: ImageView): UiNode {
  const node: UiImageNode = { type: "image", id: IMAGE_WINDOW_KIND_ID, src: `data:${view.mime};base64,${view.base64}`, alt: `${view.width}x${view.height}` };
  return node;
}

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("renderImage", () => {
    it("builds a base64 data URI from mime + base64", () => {
      const node = renderImage({ width: 4, height: 2, mime: "image/png", base64: "QUJD" });
      if (node.type !== "image") throw new Error("expected image");
      expect(node.src).toBe("data:image/png;base64,QUJD");
    });
  });
}
//#endregion 🧪️Tests
// #endregion 🖼️ImageWindowKit
