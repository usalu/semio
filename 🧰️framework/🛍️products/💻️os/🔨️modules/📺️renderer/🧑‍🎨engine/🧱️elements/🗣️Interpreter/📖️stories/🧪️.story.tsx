// #region 🧲️Header
// 💻️ 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/📖️stories/🧪️.story.tsx
// Specs: Exercise the retained-document interpreter (`InterpretedUiNode`, `🗣️Interpreter/🟦️.tsx`) directly
// against hand-written `BuiltNode` trees — no wasm, no plugin runtime, just the declarative half of the
// renderer.
// Summary: Three fixtures — a button-in-a-stack, a two-section tree (nested row plus an inline `toggle`),
// and a `section`/`field` "properties panel" (input/select/slider/keyValueList). Each is minted into a
// `UiSnapshot` by the framework's own `builtNodeToSnapshot`, loaded into a real `UiDocumentStore`, and
// rendered by the same interpreter the shell runs. A debug readout records the last dispatched
// `ActionDescriptor`, so button clicks, toggle flips, select/slider changes and tree selection all
// visibly round-trip.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react-vite";
import { useCallback, useMemo, useState, type ReactElement } from "react";

import { InterpretedUiNode, UiDocumentStore, builtNodeToSnapshot } from "@semio-tech/framework-renderer-react";
import type { ActionBinding, ActionDescriptor, BuiltNode, Component, LayoutSpec, UiTrigger } from "@semio-tech/framework";

const STORY_UI_INTERPRETER_CONTROLLER_ID = "ui-interpreter-story";

//#region Authoring
/** 🧱️ Everything a `BuiltNode` carries beyond its component and children is boilerplate in a story:
 * these are the contract's own neutral defaults, spelled once. */
const LEAF: LayoutSpec = { kind: "leaf", width: "hug", height: "hug" };
const COLUMN: LayoutSpec = { kind: "stack", axis: "vertical", gap: "sm", padding: { all: "none" }, align: "stretch", justify: "start", grow: false, wrap: false };
const STYLE = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" } as const;
const ACCESSIBILITY = { label: null, description: null, live: "off", shortcut: null, hidden: false } as const;

/** 🎬️ One action binding addressed at this story's own controller, in the retained wire shape
 * (`ActionId { scope, name, version }` plus the trigger it fires on). */
function bind(trigger: UiTrigger, name: string): ActionBinding {
  return { trigger, action: { scope: STORY_UI_INTERPRETER_CONTROLLER_ID, name, version: 1 }, args: null, capability: null };
}

function node(spec: { key: string; component: Component; layout?: LayoutSpec; bindings?: readonly ActionBinding[]; disabled?: boolean; children?: readonly BuiltNode[] }): BuiltNode {
  return {
    key: spec.key,
    component: spec.component,
    layout: spec.layout ?? LEAF,
    style: STYLE,
    activity: "idle",
    disabled: spec.disabled ?? false,
    accessibility: ACCESSIBILITY,
    bindings: [...(spec.bindings ?? [])],
    menu: null,
    children: [...(spec.children ?? [])],
  };
}
//#endregion Authoring

//#region Fixtures
/** 🔘️ A vertical container of text plus two `button` components — one live, one loading and disabled. */
const BUTTON_TREE: BuiltNode = node({
  key: "story-buttons",
  component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
  layout: COLUMN,
  children: [
    node({ key: "story-intro", component: { type: "text", value: "Click the button below:", emphasize: null, dataAttributes: null } }),
    node({ key: "story-add-item", component: { type: "button", icon: "plus", label: "Add Item" }, bindings: [bind("activate", "addItem")] }),
    node({ key: "story-rule", component: { type: "separator" } }),
    node({ key: "story-loading-caption", component: { type: "text", value: "A disabled, loading variant:", emphasize: true, dataAttributes: null } }),
    node({ key: "story-add-item-loading", component: { type: "button", icon: "loader", label: "Working…" }, disabled: true, bindings: [bind("activate", "noOperation")] }),
  ],
});

/** 🌳️ Two tree sections — "Layers" (a leaf row and a nested row) and "Settings" (a `toggle` row). */
const TREE_TREE: BuiltNode = node({
  key: "story-tree",
  component: { type: "tree", presentation: "standard", interactionDomain: null },
  layout: COLUMN,
  children: [
    node({
      key: "sec-layers",
      component: { type: "treeSection", label: "Layers", defaultOpen: true, window: null },
      children: [
        node({
          key: "item-background",
          component: { type: "treeItem", label: "Background", description: null, icon: "square", defaultOpen: null, draggable: null, dragData: null, dimmed: null, window: null, granularity: null, rowActions: [] },
          bindings: [bind("activate", "selectItem")],
        }),
        node({
          key: "item-foreground",
          component: { type: "treeItem", label: "Foreground", description: null, icon: "square", defaultOpen: true, draggable: null, dragData: null, dimmed: null, window: null, granularity: null, rowActions: [] },
          children: [
            node({ key: "item-foreground-shadow", component: { type: "treeItem", label: "Shadow", description: null, icon: "circle", defaultOpen: null, draggable: null, dragData: null, dimmed: null, window: null, granularity: null, rowActions: [] } }),
          ],
        }),
      ],
    }),
    node({
      key: "sec-settings",
      component: { type: "treeSection", label: "Settings", defaultOpen: true, window: null },
      children: [
        node({
          key: "item-visible",
          component: { type: "treeItem", label: "Visible", description: null, icon: null, defaultOpen: null, draggable: null, dragData: null, dimmed: null, window: null, granularity: null, rowActions: [] },
          children: [node({ key: "toggle-visible", component: { type: "toggle", appearance: "checkbox", on: true, icon: "eye", text: "Visible" }, bindings: [bind("change", "toggleVisible")] })],
        }),
      ],
    }),
  ],
});

/** 🎛️ A `section` container wrapping two `field` containers (input/select), a bare slider and a
 * key-value summary — the "properties panel" shape. */
const PANEL_TREE: BuiltNode = node({
  key: "panel-properties",
  component: { type: "container", role: "section", label: "Properties", description: null, required: null, error: null, defaultOpen: true, dropOverlay: null },
  layout: COLUMN,
  children: [
    node({
      key: "field-name",
      component: { type: "container", role: "field", label: "Name", description: "Display label", required: null, error: null, defaultOpen: null, dropOverlay: null },
      layout: COLUMN,
      children: [node({ key: "input-name", component: { type: "input", kind: "text", value: "Board A", placeholder: null, commit: "blur", min: null, max: null, step: null, accept: null }, bindings: [bind("commit", "setName")] })],
    }),
    node({
      key: "field-kind",
      component: { type: "container", role: "field", label: "Kind", description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
      layout: COLUMN,
      children: [
        node({
          key: "select-kind",
          component: { type: "select", value: "seed", items: [{ value: "seed", label: "Seed" }, { value: "handle", label: "Handle" }], placeholder: null },
          bindings: [bind("change", "setKind")],
        }),
      ],
    }),
    node({ key: "slider-opacity", component: { type: "slider", value: 0.8, min: 0, max: 1, step: 0.05, unit: "α" }, bindings: [bind("change", "setOpacity")] }),
    node({ key: "summary", component: { type: "keyValueList", entries: [{ label: "Id", value: "node-42" }, { label: "Updated", value: "2026-07-19" }] } }),
  ],
});
//#endregion Fixtures

//#region StoryHost
function UiInterpreterStoryHost({ surface, node: root }: { readonly surface: string; readonly node: BuiltNode }): ReactElement {
  const [lastAction, setLastAction] = useState<ActionDescriptor | null>(null);

  const onAction = useCallback((action: ActionDescriptor): void => {
    setLastAction(action);
  }, []);

  /** 🗄️ A fresh per-fixture store: `builtNodeToSnapshot` mints the DFS-local ids the interpreter
   * subscribes by, and `loadSnapshot` is the store's whole-body hydration path. */
  const store = useMemo(() => {
    const created = new UiDocumentStore(surface);
    created.loadSnapshot(builtNodeToSnapshot(surface, root));
    return created;
  }, [surface, root]);

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ flex: "1 1 auto", minHeight: 0, overflow: "auto", padding: 16 }}>
        <InterpretedUiNode store={store} onAction={onAction} onIntent={() => undefined} />
      </div>
      <pre data-testid="ui-interpreter-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {JSON.stringify({ lastAction })}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🛠️framework🔌️hosts/UiInterpreter",
  component: UiInterpreterStoryHost,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof UiInterpreterStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🔘️ The `"button"` component, plus the `"separator"`/`"text"` kinds inside a stack container. */
export const Button: Story = {
  args: { surface: "story.ui-interpreter.button", node: BUTTON_TREE },
};

/** 🌳️ The `"tree"`/`"treeSection"`/`"treeItem"` components — a nested row and an inline `toggle`. */
export const Tree: Story = {
  args: { surface: "story.ui-interpreter.tree", node: TREE_TREE },
};

/** 🎛️ `section`/`field` containers wrapping `"input"`/`"select"`, plus a bare `"slider"` and `"keyValueList"`. */
export const Panel: Story = {
  args: { surface: "story.ui-interpreter.panel", node: PANEL_TREE },
};
