// #region 🌳️TreeWindowKit
/// <reference types="vitest/importMeta" />
/** @emoji 🌳️ `@semio-tech/plugin-window-kits` — TS twin of Rust `TreeWindowKit` (`framework.window.tree`).
 * ⚠️ The Rust twin is deliberately still on the old `ui_wgpu::wgpu::UiNode` return type this wave (see
 * `🔌️plugin/🦀️.rs` `#region 🔖️WindowKits`'s own doc comment, ticket
 * SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY packet `sdk-helpers`) — this file gets ahead of it onto the
 * new semantic contract (`BuiltNode`) since nothing currently constrains the two to match
 * simultaneously and no production code calls `renderTree` yet; re-verify parity once Rust migrates.
 * Nesting under the new contract's `Component::TreeItem` is via ordinary `BuiltNode.children` (no more
 * inline `items: [...]` — see `TreeItemProps`'s own doc comment), so a nested `TreeNodeView` becomes a
 * `treeItem` `BuiltNode` whose own children are its child `treeItem`s. */
import type { BuiltNode, Component, LayoutSpec, StyleSpec, AccessibilitySpec } from "@semio-tech/framework";

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

const DEFAULT_LAYOUT: LayoutSpec = { kind: "leaf", width: "hug", height: "hug" };
const DEFAULT_STYLE: StyleSpec = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
const DEFAULT_ACCESSIBILITY: AccessibilitySpec = { label: null, description: null, live: "off", shortcut: null, hidden: false };

/** 🧱️ Stamps a {@link BuiltNode} from `component`/`children`, filling every other field with the shared defaults. */
function builtNode(key: string, component: Component, children: readonly BuiltNode[] = []): BuiltNode {
  return { key, component, layout: DEFAULT_LAYOUT, style: DEFAULT_STYLE, activity: "idle", disabled: false, accessibility: DEFAULT_ACCESSIBILITY, bindings: [], menu: null, children: [...children] };
}

function toItem(node: TreeNodeView): BuiltNode {
  const children = (node.children ?? []).map(toItem);
  return builtNode(node.id, { type: "treeItem", label: node.label, description: null, icon: null, defaultOpen: null, draggable: null, dragData: null, dimmed: null, rowActions: [] }, children);
}

/** 🌳️ Twin of Rust `TreeWindowKit::render` — recursively expands `view.roots` into one tree section. */
export function renderTree(view: TreeView): BuiltNode {
  const section = builtNode(`${TREE_WINDOW_KIND_ID}-root`, { type: "treeSection", label: null, defaultOpen: true }, view.roots.map(toItem));
  return builtNode(TREE_WINDOW_KIND_ID, { type: "tree", interactionDomain: null }, [section]);
}

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("renderTree", () => {
    it("expands nested children recursively", () => {
      const node = renderTree({ roots: [{ id: "root", label: "Root", children: [{ id: "child", label: "Child" }] }] });
      if (node.component.type !== "tree") throw new Error("expected tree");
      expect(node.children.length).toBe(1);
      const rootItem = node.children[0]!.children[0]!;
      expect(rootItem.key).toBe("root");
      expect(rootItem.children[0]?.key).toBe("child");
    });
  });
}
//#endregion 🧪️Tests
// #endregion 🌳️TreeWindowKit
