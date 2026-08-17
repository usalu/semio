// #region 📄️DocumentWindowKit
/// <reference types="vitest/importMeta" />
/** @emoji 📄️ `@semio-tech/plugin-window-kits` — TS twin of Rust `DocumentWindowKit` (`framework.window.document`). */
import type { UiNode, UiStackNode, UiTextNode } from "@semio-tech/framework";

/** 🆔️ Frozen kind id — twin of Rust `DocumentWindowKit::KIND_ID`. */
export const DOCUMENT_WINDOW_KIND_ID = "framework.window.document";

/** 📄️ One page of plain text — twin of Rust `DocumentPage`. */
export type DocumentPage = {
  readonly text: string;
};

/** 📄️ A paginated text document — twin of Rust `DocumentView`. */
export type DocumentView = {
  readonly pages: readonly DocumentPage[];
};

/** 📄️ Twin of Rust `DocumentWindowKit::render` — one text child per page inside an unlabeled vertical stack. */
export function renderDocument(view: DocumentView): UiNode {
  const children: UiTextNode[] = view.pages.map((page) => ({ type: "text", value: page.text }));
  const stack: UiStackNode = { type: "stack", direction: "vertical", gap: "standard", children };
  return stack;
}

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("renderDocument", () => {
    it("renders one child per page", () => {
      const node = renderDocument({ pages: [{ text: "p1" }, { text: "p2" }] });
      if (node.type !== "stack") throw new Error("expected stack");
      expect(node.children.length).toBe(2);
    });
  });
}
//#endregion 🧪️Tests
// #endregion 📄️DocumentWindowKit
