// #region 🌳️TreeWindowKit
/// <reference types="vitest/importMeta" />
/** @emoji 🌳️ `@semio-tech/plugin-window-kits` — TS twin of Rust `TreeWindowKit` (`framework.window.tree`). */
import type { UiNode, UiTreeItemNode, UiTreeNode } from "@semio-tech/framework";

/** 🆔️ Frozen kind id — twin of Rust `TreeWindowKit::KIND_ID`. */
export const TREE_WINDOW_KIND_ID = "framework.window.tree";

/** 🌳️ One recursive labeled node — twin of Rust `TreeNodeView`. */
export type TreeNodeView = {
  readonly id: string;
  readonly label: string;
  readonly children?: readonly TreeNodeView[];
};

/** 🌳️ `roots` seeds a single unlabeled tree section — twin of Rust `TreeView`. */
export type TreeView = {
  readonly roots: readonly TreeNodeView[];
};

function toItem(node: TreeNodeView): UiTreeItemNode {
  return { id: node.id, label: node.label, items: node.children && node.children.length > 0 ? node.children.map(toItem) : undefined };
}

/** 🌳️ Twin of Rust `TreeWindowKit::render` — recursively expands `view.roots` into one tree section. */
export function renderTree(view: TreeView): UiNode {
  const tree: UiTreeNode = { type: "tree", sections: [{ id: `${TREE_WINDOW_KIND_ID}-root`, defaultOpen: true, items: view.roots.map(toItem) }] };
  return tree;
}

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("renderTree", () => {
    it("expands nested children recursively", () => {
      const node = renderTree({ roots: [{ id: "root", label: "Root", children: [{ id: "child", label: "Child" }] }] });
      if (node.type !== "tree") throw new Error("expected tree");
      expect(node.sections.length).toBe(1);
      const rootItem = node.sections[0]!.items[0]!;
      expect(rootItem.id).toBe("root");
      expect(rootItem.items?.[0]?.id).toBe("child");
    });
  });
}
//#endregion 🧪️Tests
// #endregion 🌳️TreeWindowKit
