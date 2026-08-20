// #region 📄️DocumentWindowKit
/// <reference types="vitest/importMeta" />
/** @emoji 📄️ `@semio-tech/plugin-window-kits` — TS twin of Rust `DocumentWindowKit` (`framework.window.document`).
 * ⚠️ The Rust twin is deliberately still on the old `ui_wgpu::wgpu::UiNode` return type this wave (see
 * `🔌️plugin/🦀️component.rs` `#region 🔖️WindowKits`'s own doc comment, ticket
 * SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY packet `sdk-helpers`) — this file gets ahead of it onto the
 * new semantic contract (`BuiltNode`) since nothing currently constrains the two to match
 * simultaneously and no production code calls `renderDocument` yet; re-verify parity once Rust migrates. */
import type { BuiltNode, Component, LayoutSpec, StyleSpec, AccessibilitySpec } from "@semio-tech/framework";

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

const DEFAULT_LEAF_LAYOUT: LayoutSpec = { kind: "leaf", width: "hug", height: "hug" };
const DEFAULT_STACK_LAYOUT: LayoutSpec = { kind: "stack", axis: "vertical", gap: "md", padding: { all: "none" }, align: "stretch", justify: "start", grow: false, wrap: false };
const DEFAULT_STYLE: StyleSpec = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
const DEFAULT_ACCESSIBILITY: AccessibilitySpec = { label: null, description: null, live: "off", shortcut: null, hidden: false };

/** 🧱️ Stamps a {@link BuiltNode} from `component`/`layout`/`children`, filling every other field with the shared defaults. */
function builtNode(key: string, component: Component, layout: LayoutSpec, children: readonly BuiltNode[] = []): BuiltNode {
  return { key, component, layout, style: DEFAULT_STYLE, activity: "idle", disabled: false, accessibility: DEFAULT_ACCESSIBILITY, bindings: [], menu: null, children: [...children] };
}

/** 📄️ Twin of Rust `DocumentWindowKit::render` — one text child per page inside an unlabeled vertical stack. */
export function renderDocument(view: DocumentView): BuiltNode {
  const children = view.pages.map((page, index) => builtNode(`page-${index}`, { type: "text", value: page.text, emphasize: null, dataAttributes: null }, DEFAULT_LEAF_LAYOUT));
  return builtNode(DOCUMENT_WINDOW_KIND_ID, { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null }, DEFAULT_STACK_LAYOUT, children);
}

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("renderDocument", () => {
    it("renders one child per page", () => {
      const node = renderDocument({ pages: [{ text: "p1" }, { text: "p2" }] });
      if (node.component.type !== "container") throw new Error("expected container");
      expect(node.children.length).toBe(2);
    });
  });
}
//#endregion 🧪️Tests
// #endregion 📄️DocumentWindowKit
