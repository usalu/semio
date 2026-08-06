// #region 🧲️Header
/// <reference types="vitest/importMeta" />
/** @emoji 🧭️ `@semio-tech/framework-core` — shared canvas pick helpers, layout factories, and inspector utilities for UI renderers. */
// #endregion 🧲️Header

import { PLAYGROUND_BUILD_TARGETS, type PlaygroundBuildTarget } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🟦️playgrounds.ts";
import { PLUGIN_BUILD_TARGETS, PLUGIN_HOST_CONFIGS, pluginModuleUrl } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🟦️plugins.ts";
import type { IconName } from "@semio-tech/assets";
export type { IconName };
import { SHELL_LOCALES, isShellLocale, SHELL_TERMINOLOGIES, isShellTerminology, type ShellLocale, type ShellTerminology, type LocalizedLabel } from "./🤖️generated/🟦️ui-axes.ts";
export { SHELL_LOCALES, isShellLocale, SHELL_TERMINOLOGIES, isShellTerminology };
export type { ShellLocale, ShellTerminology, LocalizedLabel };

// #region 🧬️GeneratedMirror
/** 🧬️ Types generated from `framework/core/rs/lib.rs` via ts-rs (`bun nx run @semio-tech/framework-core:generate`); re-exported below alongside their hand-written neighbors so this stays the one import surface. */
import type {
  ActionDescriptor as GeneratedActionDescriptor,
  ActionKind as GeneratedActionKind,
  ActionDefinition as GeneratedActionDefinition,
  ActionArgDef as GeneratedActionArgDef,
  ActionArgControl as GeneratedActionArgControl,
  ActionArgOption as GeneratedActionArgOption,
  UtilityDefinition as GeneratedUtilityDefinition,
  UtilityRef as GeneratedUtilityRef,
  ToolDefinition as GeneratedToolDefinition,
  ToolRef as GeneratedToolRef,
  CommandScope as GeneratedCommandScope,
  CommandDefinition as GeneratedCommandDefinition,
  CommandRef as GeneratedCommandRef,
  WindowMeasure as GeneratedWindowMeasure,
  WindowEngagementOption as GeneratedWindowEngagementOption,
  WindowEngagementInput as GeneratedWindowEngagementInput,
  WindowEngagementStatus as GeneratedWindowEngagementStatus,
  WindowEngagementPossible as GeneratedWindowEngagementPossible,
  WindowEngagementRingOption as GeneratedWindowEngagementRingOption,
  WindowEngagementToggleGroupOption as GeneratedWindowEngagementToggleGroupOption,
  WindowEngagementSelectItem as GeneratedWindowEngagementSelectItem,
  WindowEngagementControl as GeneratedWindowEngagementControl,
  WindowEngagement as GeneratedWindowEngagement,
  WindowEngagementSlot as GeneratedWindowEngagementSlot,
  WindowOptions as GeneratedWindowOptions,
  ActionRef as GeneratedActionRef,
  PanelGroup as GeneratedPanelGroup,
  PanelTabKind as GeneratedPanelTabKind,
  PanelTabDefinition as GeneratedPanelTabDefinition,
  ModeDefinition as GeneratedModeDefinition,
  WindowKindDefinition as GeneratedWindowKindDefinition,
  AppDefinition as GeneratedAppDefinition,
  IntroductionDefinition as GeneratedIntroductionDefinition,
  IntroductionStepDefinition as GeneratedIntroductionStepDefinition,
  IntroductionPlacement as GeneratedIntroductionPlacement,
  IntroductionInteraction as GeneratedIntroductionInteraction,
  IntroductionInteractionKind as GeneratedIntroductionInteractionKind,
  IntroductionLogo as GeneratedIntroductionLogo,
  IntroductionPoint as GeneratedIntroductionPoint,
  IntroductionGesture as GeneratedIntroductionGesture,
  IntroductionKeyModifier as GeneratedIntroductionKeyModifier,
  IntroductionPointerButton as GeneratedIntroductionPointerButton,
  IntroductionCursor as GeneratedIntroductionCursor,
  IntroductionDemonstration as GeneratedIntroductionDemonstration,
  DialogDefinition as GeneratedDialogDefinition,
} from "./🤖️generated/🟦️manifest.ts";
// #endregion 🧬️GeneratedMirror

export const CANVAS_HOVER_SOURCE_CANVAS = "canvas";
export const CANVAS_HOVER_SOURCE_PICK_MENU = "pick-menu";
export const CANVAS_HOVER_SOURCE_CATALOG = "catalog";
export const CANVAS_HOVER_SOURCE_DOCUMENT = "document";

export const FRAMEWORK_PANEL_TAB_DOCUMENT_ID = "framework.panel.document";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_ID = "framework.panel.catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_ID = "framework.panel.inspection";
export const FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL = "Document";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL = "Catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_LABEL = "Inspection";
export const FRAMEWORK_PANEL_TAB_DOCUMENT_ICON_ID = "framework.panel.document";
export const FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID = "framework.panel.catalogue";
export const FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID = "framework.panel.inspection";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_ID = "framework.panel.parameters";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL = "Parameters";
export const FRAMEWORK_PANEL_TAB_PARAMETERS_ICON_ID = "framework.panel.parameters";
/** 🕰️ Mirrors Rust `FRAMEWORK_PANEL_TAB_HISTORY_ID` — auto-injected into every app's `panelTabs`. */
export const FRAMEWORK_PANEL_TAB_HISTORY_ID = "framework.panel.history";
export const FRAMEWORK_PANEL_TAB_HISTORY_LABEL = "History";
export const FRAMEWORK_PANEL_TAB_HISTORY_ICON_ID = "framework.panel.history";

export const UI_INSPECTOR_MIXED_PLACEHOLDER = "Mixed";

//#region 🆔️ElementId
/** 🆔️ Element id of the app shell's navbar/footer — singular, shell-owned chrome. */
export const UI_NAVBAR_ELEMENT_ID = "ui.navbar";
export const UI_FOOTER_ELEMENT_ID = "ui.footer";

/** 🆔️ Normalizes arbitrary input into a single camelCase element-id segment — byte-for-byte mirror of
 * `element_id_segment` in `framework/core/rs/lib.rs` (core/js stays DOM-free, so the DOM-facing
 * `elementIdSelector`/alias helpers live in `framework/ui/js/react` instead). */
function elementIdSegment(raw: string): string {
  let segment = "";
  let capitalizeNext = false;
  for (const ch of raw) {
    if (ch === "-" || ch === "_" || ch === " " || ch === ".") {
      capitalizeNext = true;
      continue;
    }
    if (!/[a-zA-Z0-9]/.test(ch)) continue;
    if (segment.length === 0) {
      segment += ch.toLowerCase();
    } else if (capitalizeNext) {
      segment += ch.toUpperCase();
      capitalizeNext = false;
    } else {
      segment += ch;
    }
  }
  return segment;
}

/** 🆔️ Element id of a window kind's body — `framework.window.{camelCased kind id}`. */
export function windowElementId(kindId: string): string {
  return `framework.window.${elementIdSegment(kindId)}`;
}

/** 🆔️ Element id of a panel tab's uncollapsed panel body; `tabId` is already dotted, appended verbatim. */
export function panelTabElementId(tabId: string): string {
  return `framework.panelTab.${tabId}`;
}

/** 🆔️ Alias id of the first draggable tree row inside a panel tab, stamped via `data-element-alias`. */
export function panelTabFirstDraggableElementId(tabId: string): string {
  return `framework.panelTab.${tabId}.firstDraggable`;
}
//#endregion 🆔️ElementId

export type CanvasPickTarget = {
  readonly domain: string;
  readonly id: string;
  readonly generality: number;
  readonly label: string;
  readonly kind?: string;
};

export type CanvasPickRequest = {
  readonly targets: readonly CanvasPickTarget[];
  readonly client: { readonly x: number; readonly y: number };
  readonly modifiers?: Readonly<Record<string, boolean>>;
};

export type CanvasHoverFocus = {
  readonly sourceId: string;
  readonly target: CanvasPickTarget | null;
};

/** 🧬️ Generated from Rust `ActionDescriptor` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type ActionDescriptor = GeneratedActionDescriptor;

export type WindowLayoutWindowNode = {
  readonly kind: "window";
  readonly windowKindId: string;
  readonly title?: string;
  readonly instanceId?: string;
  readonly templateId?: string;
};

export type WindowLayoutStackNode = {
  readonly kind: "stack";
  readonly size?: number;
  readonly children: readonly WindowLayoutWindowNode[];
};

export type WindowLayoutAxisNode = {
  readonly kind: "row" | "column";
  readonly size?: number;
  readonly children: readonly (WindowLayoutAxisNode | WindowLayoutStackNode)[];
};

export type WindowLayout = {
  readonly root: WindowLayoutAxisNode | WindowLayoutStackNode;
};

export type NamedLayout = {
  readonly id: string;
  readonly label: string;
  readonly iconId?: IconName;
  readonly layout: WindowLayout;
  readonly origin: "builtin" | "user";
  readonly groupPath?: readonly string[];
};

export type UtilityCategory = "selection" | "utilities" | "history" | "sync";

export type UtilityLeaf =
  | { readonly id: string; readonly kind: "separator"; readonly order?: number; readonly disabled?: boolean }
  | {
      readonly id: string;
      readonly kind: "button";
      readonly iconId: IconName;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly disabled?: boolean;
      readonly category?: UtilityCategory;
      readonly controllerId?: string;
      readonly action?: string;
      readonly args?: unknown;
    }
  | {
      readonly id: string;
      readonly kind: "toggle";
      readonly iconId: IconName;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly pressed?: boolean;
      readonly disabled?: boolean;
      readonly category?: UtilityCategory;
      readonly controllerId?: string;
      readonly action?: string;
      readonly args?: unknown;
    };

export type UtilityNode =
  | UtilityLeaf
  | {
      readonly id: string;
      readonly kind: "collection";
      readonly iconId: IconName;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly disabled?: boolean;
      readonly category?: UtilityCategory;
      readonly children: readonly UtilityNode[];
    }
  | {
      readonly id: string;
      readonly kind: "button";
      readonly iconId: IconName;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly disabled?: boolean;
      readonly category?: UtilityCategory;
      readonly onPress: ActionDescriptor;
    }
  | {
      readonly id: string;
      readonly kind: "toggle";
      readonly iconId: IconName;
      readonly label?: string;
      readonly text?: string;
      readonly title?: string;
      readonly order?: number;
      readonly pressed?: boolean;
      readonly disabled?: boolean;
      readonly category?: UtilityCategory;
      readonly onChange: ActionDescriptor;
    };

export type UiSectionNode = {
  readonly type: "section";
  readonly id: string;
  readonly label?: string;
  readonly defaultOpen?: boolean;
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly menu?: UiMenuRef;
  readonly children: readonly UiNode[];
};

/** @emoji 🌳️ One hover-revealed row action on a {@link UiTreeItemNode}; renderer-side addition on top of the base wasm tree-item shape. */
export type UiTreeActionPlacement = "row" | "menu";

export type UiTreeItemAction = {
  readonly iconId: IconName;
  readonly label?: string;
  readonly action: ActionDescriptor;
  readonly placement?: UiTreeActionPlacement;
};

export type UiTreeItemNode = {
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly icon?: string;
  readonly iconId?: IconName;
  readonly selected?: boolean;
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly defaultOpen?: boolean;
  readonly action?: ActionDescriptor;
  readonly hoverAction?: ActionDescriptor;
  readonly unhoverAction?: ActionDescriptor;
  readonly actions?: readonly UiTreeItemAction[];
  readonly draggable?: boolean;
  readonly dragData?: Readonly<Record<string, string>>;
  readonly items?: readonly UiTreeItemNode[];
  readonly control?: UiControlNode;
  readonly isHidden?: boolean;
  /** 🖱️ Row-level context-menu address — most rows share one `menu.id` across a tree with the row
   * id carried in `args` (e.g. `{ id: row.id }`), rather than minting a unique menu id per row. */
  readonly menu?: UiMenuRef;
};

export type UiTreeSectionNode = {
  readonly id: string;
  readonly label?: string;
  readonly defaultOpen?: boolean;
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly items: readonly UiTreeItemNode[];
};

export type UiTreeNode = {
  readonly type: "tree";
  readonly sections: readonly UiTreeSectionNode[];
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly selectedIds?: readonly string[];
  readonly highlightedIds?: readonly string[];
  readonly selectionChange?: ActionDescriptor;
  readonly dropAction?: ActionDescriptor;
  readonly menu?: UiMenuRef;
};

export type UiControlNode = UiInputNode | UiSelectNode | UiToggleNode | UiButtonNode | UiKeyValueNode | UiSliderNode | UiNumberStepperNode | UiRingNode | UiIconSelectNode;

export type UiInputNode = {
  readonly type: "input";
  readonly id: string;
  readonly inputKind: string;
  readonly value: string;
  readonly placeholder?: string;
  readonly commit?: string;
  readonly min?: number;
  readonly max?: number;
  readonly step?: number;
  readonly accept?: string;
  readonly onChange: ActionDescriptor;
  readonly menu?: UiMenuRef;
};

export type UiSelectItem = {
  readonly value: string;
  readonly label: string;
};

export type UiSelectNode = {
  readonly type: "select";
  readonly id: string;
  readonly value: string;
  readonly items: readonly UiSelectItem[];
  readonly placeholder?: string;
  readonly onChange: ActionDescriptor;
  readonly menu?: UiMenuRef;
};

export type UiToggleNode = {
  readonly type: "toggle";
  readonly id: string;
  readonly iconId: IconName;
  readonly pressed: boolean;
  readonly text?: string;
  readonly onChange: ActionDescriptor;
  readonly menu?: UiMenuRef;
};

/** @emoji 🌿️ A nestable labeled container of {@link UiNode} children — the declarative-tree mechanism
 * for subtrees like `Origin > X/Y/Z`: {@link uiDeclarativeChildToTreeItem} expands a `Group` into a
 * {@link UiTreeItemNode} whose `items` are its recursively-converted children, so depth composes to
 * any level (`Plane > Origin > X/Y/Z`). Unlike {@link UiSectionNode} (top-level tree sections only,
 * see `assertNoNestedTreeSections`), a `Group` may itself appear as another `Group`'s or
 * {@link UiFieldNode}'s child. */
export type UiGroupNode = {
  readonly type: "group";
  readonly id: string;
  readonly label: string;
  readonly defaultOpen?: boolean;
  readonly menu?: UiMenuRef;
  readonly children: readonly UiNode[];
};

export type UiKeyValueEntry = {
  readonly label: string;
  readonly value: string;
};

export type UiKeyValueNode = {
  readonly type: "keyValue";
  readonly entries: readonly UiKeyValueEntry[];
  readonly menu?: UiMenuRef;
};

export type UiSliderNode = {
  readonly type: "slider";
  readonly id: string;
  readonly value: number;
  readonly min: number;
  readonly max: number;
  readonly step: number;
  readonly unit?: string;
  readonly onChange: ActionDescriptor;
  readonly menu?: UiMenuRef;
};

export type UiNumberStepperNode = {
  readonly type: "numberStepper";
  readonly id: string;
  readonly value: number;
  readonly step: number;
  readonly uniform: boolean;
  readonly onAbsolute: ActionDescriptor;
  readonly onDelta: ActionDescriptor;
  readonly menu?: UiMenuRef;
};

export type UiRingNode = {
  readonly type: "ring";
  readonly id: string;
  readonly orbId: string;
  readonly t: number;
  readonly disabled?: boolean;
  readonly onChange: ActionDescriptor;
  readonly menu?: UiMenuRef;
};

export type UiIconSelectNode = {
  readonly type: "iconSelect";
  readonly id: string;
  readonly value: string;
  readonly uniform: boolean;
  readonly classifierKind: string;
  readonly onChange: ActionDescriptor;
  readonly menu?: UiMenuRef;
};

export type UiFieldNode = {
  readonly type: "field";
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly required?: boolean;
  readonly error?: string;
  readonly child: UiNode;
  readonly menu?: UiMenuRef;
};

/** 🎨️ Renderer-side visual variant/size/density hints on a {@link UiButtonNode} — no wasm/plugin equivalent, purely a display hint. */
export type StyleSpec = {
  readonly variant?: string;
  readonly size?: string;
  readonly density?: string;
};

export type UiButtonNode = {
  readonly type: "button";
  readonly id?: string;
  readonly iconId: IconName;
  readonly label: string;
  readonly action: ActionDescriptor;
  readonly style?: StyleSpec;
  readonly disabled?: boolean;
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly menu?: UiMenuRef;
};

export type UiTextNode = {
  readonly type: "text";
  readonly value: string;
  readonly emphasize?: boolean;
  readonly dataAttributes?: Readonly<Record<string, string>>;
  readonly menu?: UiMenuRef;
};

export type UiStackNode = {
  readonly type: "stack";
  readonly direction: string;
  readonly gap?: string;
  readonly padding?: string;
  readonly id?: string;
  readonly selected?: boolean;
  readonly loading?: boolean;
  readonly waiting?: boolean;
  readonly activate?: ActionDescriptor;
  readonly dropAction?: ActionDescriptor;
  readonly dropOverlay?: UiDropOverlaySpec;
  readonly menu?: UiMenuRef;
  readonly children: readonly UiNode[];
};

/** 📥️ Hover-state copy for a {@link UiStackNode}'s `dropOverlay` — shown while a drag is over the stack, ahead of `dropAction` firing on release. */
export type UiDropOverlaySpec = {
  readonly title: string;
  readonly hint: string;
  readonly accept?: string;
};

export type UiSeparatorNode = { readonly type: "separator"; readonly menu?: UiMenuRef };

export type UiImageNode = {
  readonly type: "image";
  readonly id: string;
  readonly src: string;
  readonly alt?: string;
  readonly menu?: UiMenuRef;
};

export type UiNode =
  | UiStackNode
  | UiTextNode
  | UiButtonNode
  | UiSeparatorNode
  | UiSectionNode
  | UiInputNode
  | UiSelectNode
  | UiToggleNode
  | UiKeyValueNode
  | UiSliderNode
  | UiNumberStepperNode
  | UiRingNode
  | UiIconSelectNode
  | UiFieldNode
  | UiGroupNode
  | UiTreeNode
  | UiImageNode
  | UiComponentSceneNode
  | UiExternalSlotNode;

export type UiInspectorFieldGroup = {
  readonly id: string;
  readonly label: string;
  readonly defaultOpen?: boolean;
  readonly fields: readonly UiNode[];
};

//#region ComponentSceneProtocol
/** 🖼️ A 2D canvas surface scene payload — mirrors the wasm `componentScene` node's `canvas2d` field. */
export type Canvas2dScene = {
  readonly cameraX: number;
  readonly cameraY: number;
  readonly zoom: number;
  readonly layersJson: string;
};

/** 🖱️ A render-time address for an on-demand context menu — bytes only, never items. At right-click
 * time the host resolves the nearest `menu` up the tree (or a scene's implicit surface-kind
 * convention id) and asks the owning plugin's `contextMenu` export to compute rows fresh; nothing
 * here is cached across clicks. */
export type UiMenuRef = {
  readonly id: string;
  readonly args?: Record<string, unknown>;
};

/** 🌐️ A 3D world surface scene payload — mirrors the wasm `componentScene` node's `world3d` field. */
/** 🖱️ One row of a resolved context menu — TS twin of the Rust `ContextMenuItemSpec`
 * (`🧰️framework/🔨️modules/🖱️ui/🧊️wgpu/📦️packages/🦀️rust/📦️lib.rs`). Plugins build these with
 * `MenuBuilder`; the host maps them through `ContextMenuController` (React) / `render_context_menu`
 * (wgpu) unchanged. */
export type ContextMenuItemSpec = {
  readonly id: string;
  readonly label?: string;
  readonly icon?: string;
  readonly color?: string;
  readonly shortcut?: string;
  readonly disabled?: boolean;
  readonly separator?: boolean;
  readonly checked?: boolean;
  readonly destructive?: boolean;
  readonly action?: string;
  readonly args?: Record<string, unknown>;
  readonly hoverAction?: string;
  readonly hoverArgs?: Record<string, unknown>;
  readonly children?: readonly ContextMenuItemSpec[];
};

//#region 🗂️ContextMenuOrganizer
/** 🗂️ Canonical ribbon-parent taxonomy — TS twin of the Rust `RIBBON_PARENT_CATEGORIES` const
 * (`🧰️framework/🔨️modules/🖱️ui/🧊️wgpu/📦️packages/🦀️rust/📦️lib.rs`) and of ui-react's closed
 * `UiRibbonParentCategory` union (`🧰️framework/🔨️modules/🖱️ui/⚛️react/📦️packages/🟦️typescript/📦️index.tsx`
 * ~3419). Id spelling and order are load-bearing: `organizeContextMenu` sorts `menu.group.<category>`
 * rows by this order (unknown categories sort after, in emit order). */
const RIBBON_PARENT_CATEGORIES = [
  "history", "hand", "selection", "lasso", "filter", "open", "save", "transfer", "transform", "create", "view", "actions", "settings",
  "methods", "mode", "targets", "export", "tools", "utilities", "sync",
] as const;

const CONTEXT_MENU_ROW_BUDGET = 9;
const CONTEXT_MENU_PRIMARY_BUDGET = 5;

function contextMenuIsBareSeparator(item: ContextMenuItemSpec): boolean {
  return item.separator === true && item.label === undefined;
}

/** 🗂️ D1: a separator carrying a `label` is a non-interactive section header, not a divider. */
function contextMenuIsHeader(item: ContextMenuItemSpec): boolean {
  return item.separator === true && item.label !== undefined;
}

function contextMenuIsGroupRow(item: ContextMenuItemSpec): boolean {
  return item.id.startsWith("menu.group.");
}

function contextMenuGroupCategory(item: ContextMenuItemSpec): string {
  return item.id.startsWith("menu.group.") ? item.id.slice("menu.group.".length) : item.id;
}

function contextMenuTaxonomyRank(category: string): number {
  const index = (RIBBON_PARENT_CATEGORIES as readonly string[]).indexOf(category);
  return index === -1 ? RIBBON_PARENT_CATEGORIES.length : index;
}

function contextMenuSeparatorRow(seed: number): ContextMenuItemSpec {
  return { id: `separator-organized-${seed}`, separator: true };
}

/** 🗂️ Collapses a run of consecutive bare (unlabeled) separators down to one, then drops a bare
 * separator left at the very start or end (nothing to separate from/to). A labeled separator (header,
 * see `contextMenuIsHeader`) is never touched by this — it always survives in place, adjacent bare
 * separators collapse/drop around it independently. */
function contextMenuNormalizeSeparators(items: readonly ContextMenuItemSpec[]): ContextMenuItemSpec[] {
  const out: ContextMenuItemSpec[] = [];
  for (const item of items) {
    if (contextMenuIsBareSeparator(item) && out.length > 0 && contextMenuIsBareSeparator(out[out.length - 1]!)) {
      continue;
    }
    out.push(item);
  }
  if (out.length > 0 && contextMenuIsBareSeparator(out[0]!)) {
    out.shift();
  }
  while (out.length > 0 && contextMenuIsBareSeparator(out[out.length - 1]!)) {
    out.pop();
  }
  return out;
}

/** 🗂️ Merges rows sharing a `menu.group.<category>` id at the position of the first occurrence,
 * concatenating and deduping their `children` by id (first occurrence wins). */
function contextMenuMergeGroupRows(items: readonly ContextMenuItemSpec[]): ContextMenuItemSpec[] {
  const out: ContextMenuItemSpec[] = [];
  const groupIndex = new Map<string, number>();
  for (const item of items) {
    if (contextMenuIsGroupRow(item)) {
      const index = groupIndex.get(item.id);
      if (index !== undefined) {
        const children = out[index]!.children ? [...out[index]!.children!] : [];
        for (const child of item.children ?? []) {
          if (!children.some((existing) => existing.id === child.id)) {
            children.push(child);
          }
        }
        out[index] = { ...out[index]!, children };
      } else {
        groupIndex.set(item.id, out.length);
        out.push(item);
      }
    } else {
      out.push(item);
    }
  }
  return out;
}

/** 🗂️ ≤9-interactive-row emission (D2 rule): plain leaves/headers in emit order, then group rows in
 * taxonomy order (unknown categories after, emit order), then — only if any exist — a separator
 * followed by destructive leaves. */
function contextMenuEmitWithinBudget(items: readonly ContextMenuItemSpec[]): ContextMenuItemSpec[] {
  const leavesAndHeaders: ContextMenuItemSpec[] = [];
  const groupRows: ContextMenuItemSpec[] = [];
  const destructiveLeaves: ContextMenuItemSpec[] = [];
  for (const item of items) {
    if (contextMenuIsGroupRow(item)) {
      groupRows.push(item);
    } else if (item.destructive === true) {
      destructiveLeaves.push(item);
    } else {
      leavesAndHeaders.push(item);
    }
  }
  groupRows.sort((a, b) => contextMenuTaxonomyRank(contextMenuGroupCategory(a)) - contextMenuTaxonomyRank(contextMenuGroupCategory(b)));
  const out = [...leavesAndHeaders, ...groupRows];
  if (destructiveLeaves.length > 0) {
    out.push(contextMenuSeparatorRow(out.length));
    out.push(...destructiveLeaves);
  }
  return out;
}

/** 🗂️ >9-interactive-row emission (D2 rule): the first 5 plain leaves outside any header section stay
 * primaries; every header's trailing run of leaves folds into a group keyed by that header's own
 * (slugified) label; every other plain leaf buckets into `menu.group.<categoryOf(action) ?? "actions">`;
 * pre-existing group rows pass through unchanged; groups then sort in taxonomy order and, if the
 * primaries+groups row count is still over budget, the excess trailing groups fold into one
 * `menu.group.more`. Destructive leaves are carried separately and appended last, after a separator. */
function contextMenuEmitOverBudget(
  items: readonly ContextMenuItemSpec[],
  categoryOf: (id: string) => string | undefined,
): ContextMenuItemSpec[] {
  function bucketMut(buckets: ContextMenuItemSpec[], id: string): number {
    const index = buckets.findIndex((bucket) => bucket.id === id);
    if (index !== -1) return index;
    buckets.push({ id, label: undefined, children: [] });
    return buckets.length - 1;
  }

  const primaries: ContextMenuItemSpec[] = [];
  const existingGroups: ContextMenuItemSpec[] = [];
  const destructiveLeaves: ContextMenuItemSpec[] = [];
  const bucketedGroups: ContextMenuItemSpec[] = [];
  let currentHeaderKey: string | undefined;

  for (const item of items) {
    if (contextMenuIsHeader(item)) {
      currentHeaderKey = item.label;
      continue;
    }
    if (contextMenuIsGroupRow(item)) {
      existingGroups.push(item);
      currentHeaderKey = undefined;
      continue;
    }
    if (item.destructive === true) {
      destructiveLeaves.push(item);
      continue;
    }
    if (currentHeaderKey !== undefined) {
      const slug = currentHeaderKey.toLowerCase().split(/\s+/).join("-");
      const index = bucketMut(bucketedGroups, `menu.group.${slug}`);
      bucketedGroups[index] = { ...bucketedGroups[index]!, children: [...(bucketedGroups[index]!.children ?? []), item] };
      continue;
    }
    if (primaries.length < CONTEXT_MENU_PRIMARY_BUDGET) {
      primaries.push(item);
      continue;
    }
    const category = categoryOf(item.action ?? item.id) ?? "actions";
    const index = bucketMut(bucketedGroups, `menu.group.${category}`);
    bucketedGroups[index] = { ...bucketedGroups[index]!, children: [...(bucketedGroups[index]!.children ?? []), item] };
  }

  const groups = [...existingGroups, ...bucketedGroups];
  groups.sort((a, b) => contextMenuTaxonomyRank(contextMenuGroupCategory(a)) - contextMenuTaxonomyRank(contextMenuGroupCategory(b)));

  let out = [...primaries, ...groups];
  if (out.length > CONTEXT_MENU_ROW_BUDGET) {
    const foldFrom = CONTEXT_MENU_ROW_BUDGET - 1;
    const overflowingGroups = out.slice(foldFrom);
    out = out.slice(0, foldFrom);
    const foldedChildren: ContextMenuItemSpec[] = [];
    for (const group of overflowingGroups) {
      foldedChildren.push(...(group.children ?? []));
    }
    out.push({ id: "menu.group.more", label: undefined, children: foldedChildren });
  }
  if (destructiveLeaves.length > 0) {
    out.push(contextMenuSeparatorRow(out.length));
    out.push(...destructiveLeaves);
  }
  return out;
}

/** 🗂️ Pure organizer enforced at every context-menu funnel — recurses into `children`, normalizes
 * separators (labeled = kept header, bare leading/trailing/doubled = dropped), merges duplicate
 * `menu.group.<category>` rows (deduping their children by id), then applies the ≤9-row / >9-row
 * emission policy from D2 of the grouped-context-menu mechanism design
 * (`contextMenuEmitWithinBudget`/`contextMenuEmitOverBudget`). `categoryOf` resolves a leaf's
 * dispatched action id to a `RIBBON_PARENT_CATEGORIES` id (`undefined` buckets into `"actions"`) —
 * branch-for-branch twin of the Rust `organize_context_menu`
 * (`🧰️framework/🔨️modules/🖱️ui/🧊️wgpu/📦️packages/🦀️rust/📦️lib.rs`). */
export function organizeContextMenu(
  items: readonly ContextMenuItemSpec[],
  categoryOf: (id: string) => string | undefined,
): ContextMenuItemSpec[] {
  const mapped = items.map((item) => ({
    ...item,
    children: item.children ? organizeContextMenu(item.children, categoryOf) : item.children,
  }));
  const normalized = contextMenuMergeGroupRows(contextMenuNormalizeSeparators(mapped));
  const interactiveCount = normalized.filter((item) => item.separator !== true).length;
  return interactiveCount <= CONTEXT_MENU_ROW_BUDGET
    ? contextMenuEmitWithinBudget(normalized)
    : contextMenuEmitOverBudget(normalized, categoryOf);
}
//#endregion 🗂️ContextMenuOrganizer

export type World3dScene = {
  readonly cameraJson: string;
  readonly meshesJson: string;
  readonly instancesJson: string;
  readonly selectionJson: string;
  readonly vorticesJson?: string;
  readonly attractionsJson?: string;
  readonly targetVolumesJson?: string;
  readonly referencesJson?: string;
  readonly brushPreviewJson?: string;
  readonly interactionJson?: string;
  readonly engagementPreviewJson?: string;
  readonly lodJson?: string;
  readonly chunkingJson?: string;
  readonly environmentJson?: string;
  readonly frameJson?: string;
  readonly fitJson?: string;
  /** 🌐️⛰️ GIS 3D terrain style/source descriptor, consumed by `WorldTerrainLayer`. */
  readonly terrainJson?: string;
  /** ☁️ Point-cloud rendering layers (10^5-10^6 points) — an array of `{ id, positionsB64 (base64 le
   * f32 xyz), colorsB64? (base64 u8 rgb), size, sizeAttenuation }`, consumed by `WorldPointCloudLayer`. */
  readonly pointsJson?: string;
  /** ⏳️ Off-main-thread compute status (`{"computing": true, "label": "…"}`) shown as an overlay while
   * a `flowEvalTick` chain resolves the meshes this scene renders. */
  readonly statusJson?: string;
};

/** 🔌️ One port on a node-graph node: identity + display label (direction is implied by whether the
 * record lives in the owning node's `inputs` or `outputs` array). `code`/`abbreviation`/`fullName`/
 * `resourceKind` are set only for OS-workflow app-instance nodes (the wire key stays `resourceKind` —
 * the rename to `artifactKind` is W4/`OsWorkflowNodeGraphPayload` scope, not this ticket's). */
export type NodeGraphPortRecord = {
  readonly id: string;
  readonly label?: string;
  readonly code?: string;
  readonly abbreviation?: string;
  readonly fullName?: string;
  readonly resourceKind?: string;
};

/** 🕸️ One node-graph node: identity, label, layout rect, typed ports. `instanceId`/`pluginId`/`appId`/
 * `icon` are set only for OS-workflow app-instance nodes (the space canvas's node-graph). */
export type NodeGraphNodeRecord = {
  readonly id: string;
  readonly label?: string;
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly inputs: readonly NodeGraphPortRecord[];
  readonly outputs: readonly NodeGraphPortRecord[];
  readonly instanceId?: string;
  readonly pluginId?: string;
  readonly appId?: string;
  readonly icon?: string;
};

/** 🕸️ One node-graph edge between two node/port endpoints. */
export type NodeGraphEdgeRecord = {
  readonly id: string;
  readonly sourceNodeId: string;
  readonly sourcePortId: string;
  readonly targetNodeId: string;
  readonly targetPortId: string;
  readonly label?: string;
};

/** 📷️ Node-graph camera: pan position + zoom factor. */
export type NodeGraphViewport = {
  readonly x: number;
  readonly y: number;
  readonly zoom: number;
};

/** 🔎️ One spotlight/find result row for a node-graph surface. */
export type NodeGraphFindItem = {
  readonly id: string;
  readonly label: string;
  readonly category: string;
};

/** 🖱️ Hovered node id, if any. */
export type NodeGraphHover = {
  readonly nodeId?: string;
};

/** ➕️ Variadic input/output slot on an operator catalogue entry. */
export type NodeGraphOperatorVariadicRecord = {
  readonly slotKey: string;
  readonly min: number;
  readonly max?: number;
};

/** 🔌️ Declared operator channel (input or output); `cardinality` rides as its serialized symbol
 * string (`"!"`/`"?"`/`"*"`/`"+"`/digits). */
export type NodeGraphOperatorChannelRecord = {
  readonly code: string;
  readonly abbreviation: string;
  readonly name: string;
  readonly fullName: string;
  readonly operators?: readonly string[];
  readonly default?: unknown;
  readonly label?: string;
  readonly cardinality: string;
};

/** 🧠️ One operator catalogue entry offered to a flow-backed node-graph's spotlight/palette. */
export type NodeGraphOperatorRecord = {
  readonly id: string;
  readonly extension: string;
  readonly name: string;
  readonly abbreviation: string;
  readonly icon: string;
  readonly summary: string;
  readonly inputs: readonly NodeGraphOperatorChannelRecord[];
  readonly outputs: readonly NodeGraphOperatorChannelRecord[];
  readonly variadicInput?: NodeGraphOperatorVariadicRecord;
  readonly variadicOutput?: NodeGraphOperatorVariadicRecord;
  readonly group?: readonly string[];
};

/** 🕸️ A node-graph surface scene payload — mirrors the wasm `componentScene` node's `nodeGraph` field. */
export type NodeGraphScene = {
  readonly nodes: readonly NodeGraphNodeRecord[];
  readonly edges: readonly NodeGraphEdgeRecord[];
  readonly viewport?: NodeGraphViewport;
  readonly editable?: boolean;
  readonly operators?: readonly NodeGraphOperatorRecord[];
  readonly findItems?: readonly NodeGraphFindItem[];
  readonly selection?: readonly string[];
  readonly hover?: NodeGraphHover;
  readonly previewOffJson?: string;
  readonly lodJson?: string;
  readonly catalogueJson?: string;
  readonly controlsJson?: string;
  readonly clustersJson?: string;
  readonly computingJson?: string;
  readonly capabilitiesJson?: string;
  readonly fixtureJson?: string;
  readonly presencePeersJson?: string;
  /** 🧵️ Channel-structured eval outputs from an off-main-thread `flowEvalTick` chain, applied via
   * `FlowWasmSession.applyEvalOutputsJson` — lets the canvas session pick up results without ever
   * evaluating itself. */
  readonly evalJson?: string;
};

/** 👥️ A live-collaboration cursor/selection peer shown on a shared surface. */
export type PresencePeer = {
  readonly clientId: string;
  readonly name: string;
  readonly selectionCount: number;
};

/** 📝️ A text-editor surface scene payload — mirrors the wasm `componentScene` node's `textEditor` field. */
export type TextEditorScene = {
  readonly buffer: string;
  readonly language?: string;
  readonly selectionJson?: string;
  readonly tokensJson?: string;
  readonly diagnosticsJson?: string;
  readonly completionsJson?: string;
  readonly overlaysJson?: string;
  readonly occurrencesJson?: string;
  readonly placeholdersJson?: string;
  readonly extraCaretsJson?: string;
  readonly selectableSpansJson?: string;
  readonly settingsJson?: string;
  readonly cameraJson?: string;
  readonly hoverJson?: string;
  readonly newlineGatesJson?: string;
  readonly renameJson?: string;
};

export const nodeGraphActions = {
  select: "nodeGraphSelect",
  hover: "nodeGraphHover",
  edit: "nodeGraphEdit",
  viewport: "nodeGraphViewport",
  spotlightCommit: "spotlightCommit",
} as const;

export const textEditorActions = {
  edit: "textEdit",
  select: "textSelect",
  hover: "textHover",
  requestCompletions: "requestCompletions",
  commitRename: "commitRename",
  formatDocument: "formatDocument",
} as const;

/** 📋️ A table surface scene payload — mirrors the wasm `componentScene` node's `table` field. */
export type TableScene = {
  readonly columnsJson: string;
  readonly rowsJson: string;
  readonly selectionJson?: string;
  readonly rowDragMime?: string;
  readonly dropAction?: ActionDescriptor;
  readonly sortJson?: string;
};

/** 🖌️ A 2D paint surface scene payload — mirrors the wasm `componentScene` node's `paint2d` field. */
export type Paint2dScene = {
  readonly documentSyncJson: string;
  readonly assetsJson: string;
  readonly cameraJson: string;
  readonly selectionJson: string;
  readonly hoveredId?: string;
  readonly activeUtility: string;
  readonly brushSize: number;
  readonly brushOpacity: number;
  readonly viewMode: string;
  readonly compositeViewportJson?: string;
};

/** 🎨️ An icon-render preview surface scene payload — mirrors the wasm `componentScene` node's `iconRender` field. */
export type IconRenderScene = {
  readonly requestJson: string;
  readonly footer?: string;
  readonly frameJson?: string;
};

/** 🗂️ A virtual-file-system browser surface scene payload — mirrors the wasm `componentScene` node's `virtualFileSystem` field. */
export type VirtualFileSystemScene = {
  readonly schemaJson: string;
  readonly rowsJson: string;
  readonly selectedRowIdsJson?: string;
  readonly hoveredRowId?: string;
  readonly emptyMessage?: string;
  readonly dragDropEnabled?: boolean;
};

/** 🗺️ A tiled map surface scene payload — mirrors the wasm `componentScene` node's `tiledMap` field. */
export type TiledMapScene = {
  readonly mapFixtureJson: string;
  readonly cameraJson: string;
  readonly renderMode: string;
  readonly vectorStyle: string;
  readonly lodMode: string;
  readonly tileUrlTemplate: string;
  readonly vectorTileUrlTemplate: string;
  readonly layerVisibilityJson: string;
  readonly layerStrokeScaleJson: string;
  readonly selectionJson: string;
  readonly hoverJson: string;
  readonly selectionMethod: string;
  readonly selectionMode: string;
};

/** 🧩️ A 2D board surface scene payload — mirrors the wasm `componentScene` node's `board2d` field. */
export type Board2dScene = {
  readonly fixtureJson: string;
  readonly cameraJson: string;
  readonly glyphCatalogsJson: string;
  readonly selectionJson: string;
  readonly interactive: boolean;
  readonly hoveredId?: string;
  readonly activeUtility?: string;
  readonly selectionMethod: string;
  readonly gridSnapEnabled: boolean;
  readonly gridFactor: number;
  readonly suggestionOffset: number;
  readonly brushWeightsJson: string;
  readonly placementCompatibilityJson: string;
  readonly lodMode: string;
};

/** 🖊️ An ink-canvas surface scene payload — mirrors the wasm `componentScene` node's `inkCanvas` field. `documentJson` is opaque to the framework: the owning program defines its shape, conventionally an array of items (e.g. stroke | shape | text | image) each carrying its own transform; `selectionJson` is a `string[]` of selected item ids. */
export type InkCanvasScene = {
  readonly documentJson: string;
  readonly selectionJson: string;
  readonly hoveredId?: string;
  readonly activeUtility: string;
  readonly viewMode: string;
  readonly interactive: boolean;
};

/** 🖊️ Renderer-to-plugin action names for ink-canvas surfaces (modeled after {@link nodeGraphActions}/{@link textEditorActions}). */
export const inkCanvasActions = {
  applyEvents: "inkApplyEvents",
  setSelection: "setSelection",
  setCamera: "setCamera",
  setHover: "setHover",
} as const;

/** 🗄️ A checkpoint ancestor-graph history view. `columnsJson` is a `HistoryColumn[]` array, newest checkpoint first. */
export type GraphTimelineScene = {
  readonly columnsJson: string;
};

/** 🧩️ A palette entry for a block kind insertable into a {@link BlockListScene}, contributed either by the host app's own built-ins or by a `playbookBlockKind` module contribution. */
export type BlockPaletteEntry = {
  readonly blockKind: string;
  readonly label: string;
  readonly iconId: IconName;
};

/** 🧩️ A strict, ordered list of steps/blocks for the Blockly-like list editor. `stepsJson` is a `PlaybookStep[]` array, `paletteJson` is a `BlockPaletteEntry[]` array of the block kinds available to insert. */
export type BlockListScene = {
  readonly stepsJson: string;
  readonly paletteJson: string;
  readonly selectedId?: string;
  readonly draggingId?: string;
};

/** 🆚️ A before/after text diff surface scene payload — mirrors the wasm `componentScene` node's `diffView` field. */
export type DiffViewScene = {
  readonly before: string;
  readonly after: string;
  readonly language?: string;
  readonly mode?: "unified" | "split";
};

/** 📰️ One entry of an {@link EventFeedScene}'s `entriesJson` array. */
export type EventFeedEntry = {
  readonly id: string;
  readonly timestampMs: number;
  readonly iconId: IconName;
  readonly title: string;
  readonly detail?: string;
  readonly tone?: string;
};

/** 📰️ A scrolling event/log feed surface scene payload — mirrors the wasm `componentScene` node's `eventFeed` field. `entriesJson` is an {@link EventFeedEntry}`[]` array. */
export type EventFeedScene = {
  readonly entriesJson: string;
  readonly follow?: boolean;
  readonly activateAction?: string;
};

/** 🔌️ A plugin-contributed external body rendered inline — mirrors the wasm `externalSlot` node. */
export type UiExternalSlotNode = {
  readonly type: "externalSlot";
  readonly pluginId: string;
  readonly appId: string;
  readonly bodyKey: string;
  readonly paramsJson: string;
  readonly menu?: UiMenuRef;
};

/** 🧭️ The dispatch key on {@link UiComponentSceneNode} — matches the lazy-loaded host component per `framework/os/renderer/js/react/index.tsx`. */
export type ComponentKind = "canvas-2d" | "world-3d" | "node-graph" | "text-editor" | "table" | "paint-2d" | "tiled-map" | "board-2d" | "icon-render" | "ink-canvas" | "graph-timeline" | "block-list" | "diff-view" | "event-feed";

/** 🖥️ A native (non-declarative) rendering surface — mirrors the wasm `componentScene` node; the active `componentKind` selects which optional scene field is populated. */
export type UiComponentSceneNode = {
  readonly type: "componentScene";
  readonly surfaceId: string;
  readonly controllerId: string;
  readonly componentKind: string;
  readonly paneId?: string;
  readonly bindingId?: string;
  /** 🖱️ Optional override of the implicit per-`componentKind` convention id (`"world3d"`,
   * `"nodeGraph"`, `"tiledMap"`, ...) the host uses when resolving which surface answers a
   * right-click — set only when a plugin needs a menu id other than the surface-kind default. */
  readonly menu?: UiMenuRef;
  readonly canvas2d?: Canvas2dScene;
  readonly world3d?: World3dScene;
  readonly nodeGraph?: NodeGraphScene;
  readonly textEditor?: TextEditorScene;
  readonly table?: TableScene;
  readonly paint2d?: Paint2dScene;
  readonly virtualFileSystem?: VirtualFileSystemScene;
  readonly tiledMap?: TiledMapScene;
  readonly board2d?: Board2dScene;
  readonly iconRender?: IconRenderScene;
  readonly inkCanvas?: InkCanvasScene;
  readonly graphTimeline?: GraphTimelineScene;
  readonly blockList?: BlockListScene;
  readonly diffView?: DiffViewScene;
  readonly eventFeed?: EventFeedScene;
};

/** 🧷️ Shared prop shape for every `framework/os/renderer/js/react/index.tsx` host component. */
export type ComponentSceneHostProps = {
  readonly node: UiComponentSceneNode;
  readonly onAction: (action: ActionDescriptor) => void;
  readonly requestContextMenu?: (request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
};
//#endregion ComponentSceneProtocol

export function canvasPickTargetKey(target: CanvasPickTarget): string {
  return `${target.domain}:${target.id}`;
}

/** @emoji 🪪️ Parses a pick target key into domain and id. */
export function parseCanvasPickTargetKey(key: string): { readonly domain: string; readonly id: string } | null {
  const colon = key.indexOf(":");
  if (colon < 0) return null;
  return { domain: key.slice(0, colon), id: key.slice(colon + 1) };
}

export function sortCanvasPickTargetsGeneralFirst(targets: readonly CanvasPickTarget[]): readonly CanvasPickTarget[] {
  return [...targets].sort((left, right) => left.generality - right.generality || left.label.localeCompare(right.label));
}

export function pickMostSpecificCanvasTarget(targets: readonly CanvasPickTarget[]): CanvasPickTarget | null {
  if (targets.length === 0) return null;
  return [...targets].sort((left, right) => right.generality - left.generality)[0] ?? null;
}

export function canvasHoverFocusFromTarget(sourceId: string, target: CanvasPickTarget | null): CanvasHoverFocus {
  return { sourceId, target };
}

export function createWindowLayout(windowKindId: string, title?: string, options?: { readonly instanceId?: string; readonly templateId?: string }): WindowLayoutWindowNode {
  return {
    kind: "window",
    windowKindId,
    ...(title ? { title } : {}),
    ...(options?.instanceId ? { instanceId: options.instanceId } : {}),
    ...(options?.templateId ? { templateId: options.templateId } : {}),
  };
}

export function createStackLayout(windowKindIds: readonly string[], titles?: readonly string[]): WindowLayout {
  return {
    root: {
      kind: "stack",
      children: windowKindIds.map((windowKindId, index) => createWindowLayout(windowKindId, titles?.[index])),
    },
  };
}

export function createDefaultLayout(windowIds: readonly string[], direction: "row" | "column" = "row", sizes?: readonly number[], titles?: readonly string[]): WindowLayout {
  return {
    root: {
      kind: direction,
      children: windowIds.map((id, index) => ({
        kind: "stack" as const,
        ...(sizes?.[index] !== undefined ? { size: sizes[index] } : {}),
        children: [createWindowLayout(id, titles?.[index] ?? id)],
      })),
    },
  };
}

export function createTabStackLayout(windowIds: readonly string[], titles?: readonly string[]): WindowLayout {
  return createStackLayout(windowIds, titles);
}

export function createNamedLayout(id: string, label: string, layout: WindowLayout, origin: NamedLayout["origin"] = "builtin", iconId?: IconName, groupPath?: readonly string[]): NamedLayout {
  return {
    id,
    label,
    layout,
    origin,
    ...(iconId ? { iconId } : {}),
    ...(groupPath?.length ? { groupPath } : {}),
  };
}

export function mergeById<T extends { id: string }>(base: readonly T[] | undefined, extension: readonly T[] | undefined): T[] | undefined {
  if (!base?.length && !extension?.length) return undefined;
  const merged = new Map<string, T>();
  base?.forEach((entry) => merged.set(entry.id, entry));
  extension?.forEach((entry) => merged.set(entry.id, entry));
  return [...merged.values()];
}

export function mergeNamedLayouts(base: readonly NamedLayout[] | undefined, extension: readonly NamedLayout[] | undefined): NamedLayout[] {
  return mergeById(base, extension) ?? [];
}

export type PlatformSubscriber = () => void;

export abstract class Store<TSnapshot> {
  private readonly listeners = new Set<PlatformSubscriber>();
  private disposed = false;

  abstract getSnapshot(): TSnapshot;

  subscribe(listener: PlatformSubscriber): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  protected notify(): void {
    if (this.disposed) return;
    for (const listener of this.listeners) listener();
  }

  dispose(): void {
    this.disposed = true;
    this.listeners.clear();
  }
}

export interface StoragePort {
  get(key: string): string | null;
  set(key: string, value: string): void;
  remove(key: string): void;
}

function namedLayoutStorageKey(appId: string): string {
  return `compose.display.layouts.${appId}`;
}

export class NamedLayoutStore extends Store<readonly NamedLayout[]> {
  private layouts: NamedLayout[] = [];

  constructor(
    private readonly appId: string,
    private readonly storage: StoragePort,
  ) {
    super();
    this.layouts = this.readPersisted();
  }

  getSnapshot(): readonly NamedLayout[] {
    return this.layouts;
  }

  save(layout: NamedLayout): void {
    const next = mergeNamedLayouts(
      this.layouts.filter((entry) => entry.id !== layout.id),
      [layout],
    );
    this.layouts = next;
    this.persist();
    this.notify();
  }

  remove(layoutId: string): void {
    const next = this.layouts.filter((entry) => entry.id !== layoutId);
    if (next.length === this.layouts.length) return;
    this.layouts = next;
    this.persist();
    this.notify();
  }

  private readPersisted(): NamedLayout[] {
    const raw = this.storage.get(namedLayoutStorageKey(this.appId));
    if (!raw) return [];
    try {
      const parsed = JSON.parse(raw) as unknown;
      if (!Array.isArray(parsed)) return [];
      return parsed.filter(
        (entry): entry is NamedLayout =>
          Boolean(entry) && typeof entry === "object" && typeof (entry as NamedLayout).id === "string" && typeof (entry as NamedLayout).label === "string" && (entry as NamedLayout).origin === "user" && Boolean((entry as NamedLayout).layout),
      );
    } catch {
      return [];
    }
  }

  private persist(): void {
    this.storage.set(namedLayoutStorageKey(this.appId), JSON.stringify(this.layouts));
  }
}

/** 🧭️ The eight anchor ids, mirroring `Anchor` in `framework/ui/js/react/index.tsx` (kept inline/private here to stay dependency-free of that package) — shared by every persisted anchor-keyed shape below so they can't drift apart from one another. */
type PersistedAnchor = "top-left" | "top-middle" | "top-right" | "right-middle" | "bottom-right" | "bottom-middle" | "bottom-left" | "left-middle";

//#region DockLayoutStore
/** 🐳️ One tab (leaf or branch) in a persisted dock panel-arrangement tree; leaves carry `trees`, branches carry `children`. */
export interface DockTabSkeleton {
  id: string;
  children?: readonly DockTabSkeleton[];
  trees?: readonly string[];
}

/** 🐳️ The full persisted dock arrangement, one tab tree per anchor — anchor ids mirror `Anchor` in `framework/ui/js/react/index.tsx` (kept inline here to stay dependency-free of that package). */
export interface DockSkeleton {
  version: 3;
  anchors: Record<PersistedAnchor, readonly DockTabSkeleton[]>;
}

function dockOsStorageKey(): string {
  return "semio.os.dock";
}

function dockAppStorageKey(appId: string): string {
  return `semio.os.dock.${appId}`;
}

/** 🧪️ Defensive read: corrupt or foreign JSON at `key` resolves to `null` rather than throwing. */
function readDockSkeleton(storage: StoragePort, key: string): DockSkeleton | null {
  const raw = storage.get(key);
  if (!raw) return null;
  try {
    const parsed = JSON.parse(raw) as unknown;
    if (!parsed || typeof parsed !== "object" || (parsed as DockSkeleton).version !== 3 || !(parsed as DockSkeleton).anchors || typeof (parsed as DockSkeleton).anchors !== "object") return null;
    return parsed as DockSkeleton;
  } catch {
    return null;
  }
}

/** 🐳️ Persists the dock panel arrangement across an "os" layer (global default across all apps) and an optional per-app layer that wins when present — `save(null)`/`saveOs(null)` remove rather than persist a JSON `"null"`. */
export class DockLayoutStore extends Store<DockSkeleton | null> {
  constructor(
    private readonly storage: StoragePort,
    private readonly appId?: string,
  ) {
    super();
  }

  getSnapshot(): DockSkeleton | null {
    if (this.appId) {
      const app = readDockSkeleton(this.storage, dockAppStorageKey(this.appId));
      if (app) return app;
    }
    return readDockSkeleton(this.storage, dockOsStorageKey());
  }

  save(skeleton: DockSkeleton | null): void {
    this.writeOrRemove(this.appId ? dockAppStorageKey(this.appId) : dockOsStorageKey(), skeleton);
    this.notify();
  }

  saveOs(skeleton: DockSkeleton | null): void {
    this.writeOrRemove(dockOsStorageKey(), skeleton);
    this.notify();
  }

  reset(): void {
    this.storage.remove(dockOsStorageKey());
    if (this.appId) this.storage.remove(dockAppStorageKey(this.appId));
    this.notify();
  }

  private writeOrRemove(key: string, skeleton: DockSkeleton | null): void {
    if (skeleton === null) this.storage.remove(key);
    else this.storage.set(key, JSON.stringify(skeleton));
  }
}
//#endregion DockLayoutStore

//#region DockUiStateStore
/** 🌱️ Persisted per-anchor panel chrome — only the fields that differ from the shell's computed defaults are ever stored. */
export interface DockUiPanelState {
  visible?: boolean;
  size?: number;
  path?: readonly string[];
}

/** 🌱️ The full persisted dock UI state: per-anchor visibility/size/active-path, per-branch drill-down memory, and tree section/group expansion. Anchor ids mirror `Anchor` (kept inline here to stay dependency-free of the `ui` package, same convention as {@link DockSkeleton}). */
export interface DockUiState {
  version: 3;
  anchors: Partial<Record<PersistedAnchor, DockUiPanelState>>;
  pathMemory?: Readonly<Record<string, string>>;
  treeOpen?: Readonly<Record<string, boolean>>;
}

function dockUiOsStorageKey(): string {
  return "semio.os.dockUi";
}

function dockUiAppStorageKey(appId: string): string {
  return `semio.os.dockUi.${appId}`;
}

/** 🧪️ Defensive read: corrupt or foreign JSON at `key` resolves to `null` rather than throwing. */
function readDockUiState(storage: StoragePort, key: string): DockUiState | null {
  const raw = storage.get(key);
  if (!raw) return null;
  try {
    const parsed = JSON.parse(raw) as unknown;
    if (!parsed || typeof parsed !== "object" || (parsed as DockUiState).version !== 3 || !(parsed as DockUiState).anchors || typeof (parsed as DockUiState).anchors !== "object") return null;
    return parsed as DockUiState;
  } catch {
    return null;
  }
}

/** 🌱️ Persists panel visibility/size/path, drill-down memory, and tree expansion across an "os" layer (global default) and an optional per-app layer that wins when present — `save(null)`/`saveOs(null)` remove rather than persist a JSON `"null"`. */
export class DockUiStateStore extends Store<DockUiState | null> {
  constructor(
    private readonly storage: StoragePort,
    private readonly appId?: string,
  ) {
    super();
  }

  getSnapshot(): DockUiState | null {
    if (this.appId) {
      const app = readDockUiState(this.storage, dockUiAppStorageKey(this.appId));
      if (app) return app;
    }
    return readDockUiState(this.storage, dockUiOsStorageKey());
  }

  save(state: DockUiState | null): void {
    this.writeOrRemove(this.appId ? dockUiAppStorageKey(this.appId) : dockUiOsStorageKey(), state);
    this.notify();
  }

  saveOs(state: DockUiState | null): void {
    this.writeOrRemove(dockUiOsStorageKey(), state);
    this.notify();
  }

  reset(): void {
    this.storage.remove(dockUiOsStorageKey());
    if (this.appId) this.storage.remove(dockUiAppStorageKey(this.appId));
    this.notify();
  }

  private writeOrRemove(key: string, state: DockUiState | null): void {
    if (state === null) this.storage.remove(key);
    else this.storage.set(key, JSON.stringify(state));
  }
}
//#endregion DockUiStateStore

//#region WindowPaneStateStore
/** 🪟️ Persisted state for one window-level pane (a {@link DockUiPanelState} sibling, but keyed per window INSTANCE id rather than globally) — only the fields that differ from the shell's computed defaults are ever stored. */
export interface WindowPaneState {
  anchor?: PersistedAnchor;
  folded?: boolean;
  size?: number;
}

/** 🪟️ The full persisted window-pane arrangement: per-window-instance, per-pane anchor/fold/size — the pane-level analog of {@link DockUiState}, since panes float inside a window rather than docking to the shell's own edges. */
export interface WindowPaneUiState {
  version: 1;
  windows: Record<string, Record<string, WindowPaneState>>;
}

function windowPaneUiOsStorageKey(): string {
  return "semio.os.paneUi";
}

function windowPaneUiAppStorageKey(appId: string): string {
  return `semio.os.paneUi.${appId}`;
}

/** 🧪️ Defensive read: corrupt or foreign JSON at `key` resolves to `null` rather than throwing. */
function readWindowPaneUiState(storage: StoragePort, key: string): WindowPaneUiState | null {
  const raw = storage.get(key);
  if (!raw) return null;
  try {
    const parsed = JSON.parse(raw) as unknown;
    if (!parsed || typeof parsed !== "object" || (parsed as WindowPaneUiState).version !== 1 || !(parsed as WindowPaneUiState).windows || typeof (parsed as WindowPaneUiState).windows !== "object") return null;
    return parsed as WindowPaneUiState;
  } catch {
    return null;
  }
}

/** 🪟️ Persists window-pane anchor/fold/size across an "os" layer (global default across all apps) and an optional per-app layer that wins when present — `save(null)`/`saveOs(null)` remove rather than persist a JSON `"null"`. */
export class WindowPaneStateStore extends Store<WindowPaneUiState | null> {
  constructor(
    private readonly storage: StoragePort,
    private readonly appId?: string,
  ) {
    super();
  }

  getSnapshot(): WindowPaneUiState | null {
    if (this.appId) {
      const app = readWindowPaneUiState(this.storage, windowPaneUiAppStorageKey(this.appId));
      if (app) return app;
    }
    return readWindowPaneUiState(this.storage, windowPaneUiOsStorageKey());
  }

  save(state: WindowPaneUiState | null): void {
    this.writeOrRemove(this.appId ? windowPaneUiAppStorageKey(this.appId) : windowPaneUiOsStorageKey(), state);
    this.notify();
  }

  saveOs(state: WindowPaneUiState | null): void {
    this.writeOrRemove(windowPaneUiOsStorageKey(), state);
    this.notify();
  }

  reset(): void {
    this.storage.remove(windowPaneUiOsStorageKey());
    if (this.appId) this.storage.remove(windowPaneUiAppStorageKey(this.appId));
    this.notify();
  }

  private writeOrRemove(key: string, state: WindowPaneUiState | null): void {
    if (state === null) this.storage.remove(key);
    else this.storage.set(key, JSON.stringify(state));
  }
}
//#endregion WindowPaneStateStore

export function createBrowserStoragePort(): StoragePort {
  return {
    get: (key) => {
      try {
        return typeof localStorage !== "undefined" ? localStorage.getItem(key) : null;
      } catch {
        return null;
      }
    },
    set: (key, value) => {
      try {
        if (typeof localStorage !== "undefined") localStorage.setItem(key, value);
      } catch {
        /* ignore */
      }
    },
    remove: (key) => {
      try {
        if (typeof localStorage !== "undefined") localStorage.removeItem(key);
      } catch {
        /* ignore */
      }
    },
  };
}

/** 🧠️ In-memory {@link StoragePort} — used by ephemeral branded shells so nothing survives a window refresh. */
export function createMemoryStoragePort(): StoragePort {
  const map = new Map<string, string>();
  return {
    get: (key) => map.get(key) ?? null,
    set: (key, value) => {
      map.set(key, value);
    },
    remove: (key) => {
      map.delete(key);
    },
  };
}

/** 🐚️ Namespaces a {@link StoragePort} under `semio.shell.<namespace>.` so several {@link FrameworkOsShell}
 * instances sharing one browser storage origin (e.g. several demonstrator panes) don't read/write each
 * other's `semio.os.dock`/`ui.chrome.*` keys. Not needed for a single page-owning shell — that shell's
 * default (unprefixed) storage is the intended shared surface. */
export function createScopedStoragePort(base: StoragePort, namespace: string): StoragePort {
  const prefix = `semio.shell.${namespace}.`;
  return {
    get: (key) => base.get(`${prefix}${key}`),
    set: (key, value) => base.set(`${prefix}${key}`, value),
    remove: (key) => base.remove(`${prefix}${key}`),
  };
}

export function uiInspectorAllEqual<T>(values: readonly T[]): boolean {
  if (values.length <= 1) return true;
  const first = values[0];
  for (let index = 1; index < values.length; index += 1) {
    if (values[index] !== first) return false;
  }
  return true;
}

export function uiInspectorMixedText(values: readonly string[]): { readonly value: string; readonly placeholder?: string } {
  const uniform = uiInspectorAllEqual(values);
  return { value: uniform ? (values[0] ?? "") : "", placeholder: uniform ? undefined : UI_INSPECTOR_MIXED_PLACEHOLDER };
}

export function uiInspectorMixedNumber(values: readonly number[]): { readonly value: number; readonly uniform: boolean } {
  const uniform = uiInspectorAllEqual(values);
  return { value: uniform ? (values[0] ?? 0) : Number.NaN, uniform };
}

export function uiInspectorMixedSelect(values: readonly string[]): { readonly value: string; readonly placeholder?: string } {
  return uiInspectorMixedText(values);
}

export function uiInspectorMixedToggle(values: readonly boolean[]): { readonly pressed: boolean; readonly uniform: boolean } {
  const uniform = uiInspectorAllEqual(values);
  return { pressed: uniform ? (values[0] ?? false) : false, uniform };
}

export function uiInspectorMixedSlider(values: readonly number[]): { readonly value: number; readonly uniform: boolean } {
  return uiInspectorMixedNumber(values);
}

/** @emoji 🔢️ Builds an editable number-stepper field row, computing the mixed/uniform display from
 * `values` via {@link uiInspectorMixedNumber}. `action` is merged into both `onAbsolute` (typed
 * entry, dispatched with `{value}`) and `onDelta` (nudge buttons, `{delta}`) — the patch handler
 * branches on whichever key the dispatched action actually carries. */
export function uiInspectorStepperField(id: string, label: string, values: readonly number[], step: number, action: ActionDescriptor): UiFieldNode {
  const mixed = uiInspectorMixedNumber(values);
  return {
    type: "field",
    id,
    label,
    child: { type: "numberStepper", id, value: mixed.value, step, uniform: mixed.uniform, onAbsolute: action, onDelta: action },
  };
}

/** @emoji 🔘️ Builds an editable boolean toggle field row, computing the mixed/uniform display from
 * `values` via {@link uiInspectorMixedToggle}. */
export function uiInspectorToggleField(id: string, label: string, iconId: IconName, values: readonly boolean[], action: ActionDescriptor): UiFieldNode {
  const mixed = uiInspectorMixedToggle(values);
  return {
    type: "field",
    id,
    label,
    child: { type: "toggle", id, iconId, pressed: mixed.pressed, onChange: action },
  };
}

/** @emoji 📐️ Builds a nested `Origin`-style group: a parent tree item labeled `label` containing
 * three {@link uiInspectorStepperField} children (`X`/`Y`/`Z`), each computing its own per-axis
 * mixed state independently — a multi-selection that agrees on X but not Y shows only Y as "Mixed".
 * `axisAction(axis)` builds the per-axis {@link ActionDescriptor}; callers typically merge
 * `{field: "<id>.x"}` etc. into its `args` so the patch handler can dot-path into the right
 * component with `value` (absolute) or `delta` (relative, offset-preserving across multi-select). */
export function uiInspectorVec3Group(
  id: string,
  label: string,
  values: readonly (readonly [number, number, number])[],
  step: number,
  axisAction: (axis: "x" | "y" | "z") => ActionDescriptor,
): UiGroupNode {
  const xs = values.map((v) => v[0]);
  const ys = values.map((v) => v[1]);
  const zs = values.map((v) => v[2]);
  return {
    type: "group",
    id,
    label,
    defaultOpen: true,
    children: [
      uiInspectorStepperField(`${id}.x`, "X", xs, step, axisAction("x")),
      uiInspectorStepperField(`${id}.y`, "Y", ys, step, axisAction("y")),
      uiInspectorStepperField(`${id}.z`, "Z", zs, step, axisAction("z")),
    ],
  };
}

export function uiInspectorGroupsToTree(groups: readonly UiInspectorFieldGroup[]): UiTreeNode {
  return uiDeclarativeSectionsToTree(
    groups
      .filter((group) => group.fields.length > 0)
      .map((group) => ({
        type: "section" as const,
        id: group.id,
        label: group.label,
        defaultOpen: group.defaultOpen ?? true,
        children: group.fields,
      })),
  );
}

const UI_CONTROL_NODE_TYPES = new Set(["input", "select", "toggle", "button", "keyValue", "slider", "numberStepper", "ring", "iconSelect"]);

function isUiControlNode(node: UiNode): node is UiControlNode {
  return UI_CONTROL_NODE_TYPES.has(node.type);
}

export function uiDeclarativeSectionsToTree(sections: readonly UiSectionNode[]): UiTreeNode {
  const treeSections: UiTreeSectionNode[] = sections.map((section) => ({
    id: section.id,
    label: section.label,
    defaultOpen: section.defaultOpen ?? true,
    items: section.children.map((child, index) => uiDeclarativeChildToTreeItem(child, `${section.id}.${index}`)),
  }));
  return {
    type: "tree",
    sections: treeSections.length ? treeSections : [{ id: "empty", items: [{ id: "empty", label: "—" }] }],
  };
}

function uiDeclarativeChildToTreeItem(node: UiNode, fallbackId: string): UiTreeItemNode {
  if (node.type === "text") return { id: `${fallbackId}.text`, label: node.value };
  if (node.type === "field") {
    if (node.child.type === "text") return { id: node.id, label: node.label, description: node.child.value };
    return { id: node.id, label: node.label, control: isUiControlNode(node.child) ? node.child : undefined };
  }
  if (node.type === "button") return { id: node.id ?? fallbackId, label: node.label, control: node };
  if (node.type === "group") {
    return {
      id: node.id,
      label: node.label,
      defaultOpen: node.defaultOpen,
      items: node.children.map((child, index) => uiDeclarativeChildToTreeItem(child, `${node.id}.${index}`)),
    };
  }
  if (node.type === "input" || node.type === "select" || node.type === "toggle" || node.type === "keyValue" || node.type === "slider" || node.type === "numberStepper" || node.type === "ring" || node.type === "iconSelect") {
    return { id: "id" in node ? String(node.id) : fallbackId, label: "", control: node };
  }
  if (node.type === "separator") return { id: `${fallbackId}.sep`, label: "—" };
  return { id: fallbackId, label: node.type };
}

//#region PluginRuntime
/** 🧬️ Generated from Rust `ActionKind`/`ActionDefinition` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type ActionKind = GeneratedActionKind;
export type ActionDefinition = GeneratedActionDefinition;
export type ActionArgDef = GeneratedActionArgDef;
export type ActionArgControl = GeneratedActionArgControl;
export type ActionArgOption = GeneratedActionArgOption;
export type UtilityDefinition = GeneratedUtilityDefinition;
export type UtilityRef = GeneratedUtilityRef;

/** 🛠️ Generated from Rust `ToolDefinition`/`ToolRef` (`framework/core/rs/lib.rs`) — a mode-level,
 * activatable capability (e.g. puzzle3d fill), distinct from a per-window `UtilityDefinition`. See
 * `js/generated/manifest.ts`. */
export type ToolDefinition = GeneratedToolDefinition;
export type ToolRef = GeneratedToolRef;

/** 🎛️ Generated from Rust `CommandScope`/`CommandDefinition`/`CommandRef` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type CommandScope = GeneratedCommandScope;
export type CommandDefinition = GeneratedCommandDefinition;
export type CommandRef = GeneratedCommandRef;

/** 🧰️ The framework-owned action id apps dispatch to activate a utility — mirrors `SET_ACTIVE_UTILITY_ACTION_ID`. */
export const SET_ACTIVE_UTILITY_ACTION_ID = "setActiveUtility";

/** 🛠️ The framework-owned action id apps dispatch to activate a mode-level tool — mirrors Rust `SET_ACTIVE_TOOL_ACTION_ID`. */
export const SET_ACTIVE_TOOL_ACTION_ID = "setActiveTool";

/** 🎓️ The framework-owned action id apps dispatch (or the shell auto-injects into the command palette)
 * to (re)start an app's introduction — mirrors Rust `START_INTRODUCTION_ACTION_ID`. */
export const START_INTRODUCTION_ACTION_ID = "startIntroduction";

/** 🎓️ Generated from Rust `Introduction*` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type IntroductionDefinition = GeneratedIntroductionDefinition;
export type IntroductionStepDefinition = GeneratedIntroductionStepDefinition;
export type IntroductionPlacement = GeneratedIntroductionPlacement;
export type IntroductionInteraction = GeneratedIntroductionInteraction;
export type IntroductionInteractionKind = GeneratedIntroductionInteractionKind;
export type IntroductionLogo = GeneratedIntroductionLogo;
export type IntroductionPoint = GeneratedIntroductionPoint;
export type IntroductionGesture = GeneratedIntroductionGesture;
export type IntroductionKeyModifier = GeneratedIntroductionKeyModifier;
export type IntroductionPointerButton = GeneratedIntroductionPointerButton;
export type IntroductionCursor = GeneratedIntroductionCursor;
export type IntroductionDemonstration = GeneratedIntroductionDemonstration;

/** 🗨️ Generated from Rust `DialogDefinition` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type DialogDefinition = GeneratedDialogDefinition;

//#region 🎬️Tutorial
/** 🎬️ The framework-owned action id apps dispatch (or the shell auto-injects into the command palette,
 * with a `tutorialId` Select arg) to (re)start a tutorial — mirrors Rust `START_TUTORIAL_ACTION_ID`.
 * Distinct from the docs-tooltip `tutorial` link field on `UiLabelLeaf` (`framework/ui/js/react`), a URL into the
 * manual — this is the interactive recorded-walkthrough mechanism. */
export const START_TUTORIAL_ACTION_ID = "startTutorial";

/** ⏺️ The framework-owned action id that opens the tutorial recorder chrome — injected into EVERY app
 * unconditionally (recording needs no app-side declaration). Mirrors Rust `RECORD_TUTORIAL_ACTION_ID`. */
export const RECORD_TUTORIAL_ACTION_ID = "recordTutorial";

/** ⏱️ Real-time (rate-independent) duration of the camera glide the player performs when the user
 * presses Play after deviating from an active tutorial's recorded state. Mirrors Rust `TUTORIAL_CONVERGE_MS`. */
export const TUTORIAL_CONVERGE_MS = 600;

// 🚧️ TODO(core-rs): these seven `Tutorial*` types mirror `framework/core/rs/lib.rs`'s `//#region 🔖️Tutorial`
// field-for-field (see that region's doc comments for the authoritative semantics) and are meant to be
// ts-rs GENERATED like their `Introduction*` neighbors above. Regeneration is blocked right now by an
// unrelated, pre-existing `typegen`-feature compile break in a concurrent session's work (`IconName` is
// missing its `TS` derive in `framework/ui/wgpu/rs/lib.rs`, breaking `cargo test --features typegen` workspace-wide).
// Once that lands, run `bun nx run @semio-tech/framework-core:generate`, delete this hand-written block,
// and re-add `Tutorial* as GeneratedTutorial*` imports above — names/shapes here were written to match the
// eventual generated output exactly, so every other file importing from this module is unaffected.
export type TutorialChapter = { readonly id: string; readonly at: number; readonly title: LocalizedLabel | string; readonly body?: LocalizedLabel | string };

export type TutorialAssetSrc =
  | { readonly kind: "url"; readonly url: string }
  | { readonly kind: "blob"; readonly hash: string; readonly size: number; readonly mediaType: string }
  | { readonly kind: "dataUrl"; readonly data: string };

export type TutorialCaption = { readonly at: number; readonly durationMs: number; readonly text: string };

export type TutorialNarrationCue = {
  readonly id: string;
  readonly at: number;
  readonly durationMs: number;
  readonly text: string;
  readonly audio?: TutorialAssetSrc;
  readonly voice?: string;
  readonly rate: number;
  readonly captions: readonly TutorialCaption[];
};

export type TutorialOverlayRect = { readonly x: number; readonly y: number; readonly width: number; readonly height: number };

export type TutorialVideoCue = {
  readonly at: number;
  readonly durationMs: number;
  readonly src: TutorialAssetSrc;
  readonly rect: TutorialOverlayRect;
  readonly muted: boolean;
  readonly sourceOffsetMs: number;
};

export type TutorialEventKind =
  | { readonly kind: "action"; readonly action: string; readonly args?: unknown }
  | { readonly kind: "command"; readonly command: string; readonly args?: unknown }
  | { readonly kind: "key"; readonly keys: string };

export type TutorialEvent = { readonly at: number; readonly kind: TutorialEventKind };

/** 🧮️ Renderer-neutral restore point for chrome/UI state — see the Rust doc comment on
 * `TutorialUiSnapshot` for why this is deliberately NOT a serialization of `ShellState`. */
export type TutorialUiSnapshot = {
  readonly activeModeId?: string;
  readonly focusedWindowId?: string;
  readonly activeUtilityByWindowId: Readonly<Record<string, string>>;
  readonly activeToolId?: string;
  readonly layout?: WindowLayout;
  readonly activePanelTabByGroup: Readonly<Record<string, string>>;
  readonly panelJson?: string;
  readonly selectionJson?: string;
  readonly openDialogId?: string;
  readonly expandedTreeIds: readonly string[];
  readonly commandPanelOpen: boolean;
};

export type TutorialUiChange =
  | { readonly kind: "activeMode"; readonly id: string }
  | { readonly kind: "focusedWindow"; readonly id?: string }
  | { readonly kind: "activeUtility"; readonly windowId: string; readonly utilityId?: string }
  | { readonly kind: "activeTool"; readonly id?: string }
  | { readonly kind: "layout"; readonly layout: WindowLayout }
  | { readonly kind: "panelTab"; readonly group: string; readonly tabId?: string }
  | { readonly kind: "panelState"; readonly panelJson: string }
  | { readonly kind: "selection"; readonly selectionJson: string }
  | { readonly kind: "dialog"; readonly id?: string; readonly args?: unknown }
  | { readonly kind: "treeExpansion"; readonly id: string; readonly expanded: boolean }
  | { readonly kind: "commandPanel"; readonly open: boolean };

export type TutorialUiSample =
  | { readonly kind: "snapshot"; readonly state: TutorialUiSnapshot }
  | { readonly kind: "delta"; readonly changes: readonly TutorialUiChange[] };

export type TutorialUiKeyframe = { readonly at: number; readonly sample: TutorialUiSample };

/** 🖋️ Mirrors `store::DocumentCommand` with `Operation = unknown` (opaque per-app operation JSON) — the
 * SOLE source of document mutation during playback; `TutorialEvent`s are annotational only. */
export type TutorialDocumentEventKind =
  | { readonly kind: "edit"; readonly forwards: readonly unknown[]; readonly backwards: readonly unknown[]; readonly description?: string; readonly coalesceKey?: string }
  | { readonly kind: "undo" }
  | { readonly kind: "redo" }
  | { readonly kind: "checkpoint"; readonly message?: string }
  | { readonly kind: "checkoutCheckpoint"; readonly checkpointId: string }
  | { readonly kind: "switchAlternative"; readonly alternativeId: string }
  | { readonly kind: "load"; readonly documentDsl: string; readonly previousDsl: string };

export type TutorialDocumentEvent = { readonly at: number; readonly kind: TutorialDocumentEventKind };

export type TutorialCameraState =
  | { readonly kind: "orbit"; readonly position: readonly [number, number, number]; readonly target: readonly [number, number, number]; readonly up: readonly [number, number, number]; readonly fov?: number }
  | { readonly kind: "canvas"; readonly x: number; readonly y: number; readonly zoom: number };

export type TutorialEasing = "linear" | "easeInOut" | "hold";

export type TutorialCameraKeyframe = { readonly at: number; readonly windowId: string; readonly camera: TutorialCameraState; readonly easing: TutorialEasing };

/** 👻️ Reuses the introduction demonstration vocabulary verbatim — see `IntroductionGesture`/`IntroductionPoint`. */
export type TutorialGestureCue = { readonly at: number; readonly durationMs: number; readonly gesture: IntroductionGesture; readonly cursor?: IntroductionCursor };

export type TutorialTracks = {
  readonly narration: readonly TutorialNarrationCue[];
  readonly video: readonly TutorialVideoCue[];
  readonly events: readonly TutorialEvent[];
  readonly ui: readonly TutorialUiKeyframe[];
  readonly document: readonly TutorialDocumentEvent[];
  readonly camera: readonly TutorialCameraKeyframe[];
  readonly gestures: readonly TutorialGestureCue[];
};

export type TutorialBase = {
  readonly documentDsl?: string;
  readonly exampleId?: string;
  readonly ui: TutorialUiSnapshot;
  readonly cameras: readonly TutorialCameraKeyframe[];
};

/** 🎬️ A recorded, timed, replayable walkthrough — the timeline sibling of `IntroductionDefinition`. A
 * *recording* IS a `TutorialDefinition`; the recorder simply produces a densely-sampled one. */
export type TutorialDefinition = {
  readonly id: string;
  readonly title: LocalizedLabel | string;
  readonly description?: LocalizedLabel | string;
  readonly durationMs: number;
  readonly chapters: readonly TutorialChapter[];
  readonly base: TutorialBase;
  readonly tracks: TutorialTracks;
  readonly recordedAt?: string;
};
//#endregion 🎬️Tutorial

//#region 🏷️ShellBrand
// 🌐️ ShellLocale/ShellTerminology are generated from ui_wgpu's 🔣️ui-axes.json (the same source of
// truth Rust's Locale/Terminology enums derive from), imported/re-exported above — so a locale
// added there and here can never drift. The single source `UiLocale` (`framework/ui/js/react`),
// `ShellBrandLocks.locale`, and `resolveShellLocks` all derive from this.

/** 🔒️ Shell preferences a brand pins at boot: each set axis is fixed and its in-app switcher hidden (validated by the renderer's `resolveShellLocks`). */
export type ShellBrandLocks = {
  readonly exampleId?: string;
  readonly locale?: ShellLocale;
  readonly terminology?: ShellTerminology;
  readonly themeId?: string;
  readonly appearance?: string;
};

/** 🎛️ Shell preferences a brand seeds at boot without pinning them: the value applies on first launch but the in-app switcher stays visible. */
export type ShellBrandDefaults = {
  readonly exampleId?: string;
};

/** 🏷️ Boot-time branding for a standalone shell artifact — identity (window title, logo mark, favicon), locked and defaulted shell preferences, and an optional brand-owned {@link IntroductionDefinition} replacing the app's own (already localized, rendered verbatim). */
export type ShellBrand = {
  readonly id: string;
  readonly windowTitle: string;
  readonly logoSvg?: string;
  readonly faviconIcoPath?: string;
  readonly locks?: ShellBrandLocks;
  readonly defaults?: ShellBrandDefaults;
  readonly introduction?: IntroductionDefinition;
  /** 🎬️ Brand-owned tutorials shown ALONGSIDE the app's own declared ones (never replacing them, unlike `introduction`). */
  readonly tutorials?: readonly TutorialDefinition[];
  /** 🎓️ When true, auto-starts the brand introduction on every window load and never persists a device-local "seen" flag. */
  readonly replayIntroductionOnLoad?: boolean;
  /** 🧊️ When true, the shell never reads or writes device-local shell state (dock, panes, named layouts, chrome prefs, introduction seen) — every refresh boots from brand locks/defaults only. */
  readonly ephemeral?: boolean;
  /** 🗂️ Repo-root-relative directory of this brand's own static assets (logos, etc.) — the dev/build server mounts it as a static route at `/<assetsDir>` alongside the shared `framework/ui/asset` mount. */
  readonly assetsDir?: string;
  /** 📦️ Repo-root-relative directory this brand's build output lands in instead of the shared playground `dist/` — keeps a brand's specialization (including its build artifact) self-contained. */
  readonly distDir?: string;
  /** 🌐️ Custom domain this brand's static build deploys to (e.g. GitHub Pages) — written verbatim into a `🌐️CNAME` file at the build root. */
  readonly cnameHost?: string;
};
//#endregion 🏷️ShellBrand

/** @emoji 🕹️ Mirrors `semio_framework_core::history_action_definitions` — the six framework-owned
 * History actions every app receives, used by the shell to render the same set without a wasm round trip. */
export const HISTORY_ACTION_IDS = ["undo", "redo", "commitCheckpoint", "createAlternative", "switchAlternative", "checkoutCheckpoint"] as const;

export type PluginViewState = {
  readonly activeModeId?: string;
  readonly activeWindowKindId?: string;
  /** 🧰️ Per-call overlay: host-owned active utility for the window targeted by this render/action (`windowId`). */
  readonly activeUtilityId?: string;
  /** 🧰️ Host-owned active utility per window instance (never a document field, never a VCS operation). */
  readonly activeUtilityByWindowId?: Readonly<Record<string, string>>;
  /** 🛠️ Host-owned active tool of the active mode (never a document field, never a VCS operation) — mutually
   * exclusive with `activeUtilityId`: activating one clears the other. */
  readonly activeToolId?: string;
  readonly selectionJson?: string;
  readonly panelJson?: string;
  readonly contributionsJson?: string;
  readonly locale?: string;
  readonly terminology?: string;
  /** 🪟️ The window instance a render/action call targets — programs key per-window option state off this, never off `activeWindowKindId`. */
  readonly windowId?: string;
  /** 🪟️ The live set of open window instances (base + spawned/split), so `windowMeasures`/`windowEngagements` can return one entry per instance. */
  readonly windowInstances?: readonly { readonly id: string; readonly windowKindId: string }[];
};

export type PluginUiNode = Record<string, unknown> & { readonly type: string };

/** 🗣️ Locale/terminology-aware label patch for an app's window-kind/panel-tab/mode labels, resolved fresh per {@link PluginViewState} — merge over the static {@link PluginManifest} app labels by id. */
export type PluginAppLabelsOverlay = {
  readonly windowKindLabels: Readonly<Record<string, string>>;
  readonly panelTabLabels: Readonly<Record<string, string>>;
  readonly modeLabels: Readonly<Record<string, string>>;
  readonly actionLabels: Readonly<Record<string, string>>;
  readonly utilityLabels: Readonly<Record<string, string>>;
  readonly exampleLabels: Readonly<Record<string, string>>;
  readonly actionArgLabels: Readonly<Record<string, string>>;
  readonly dialogLabels: Readonly<Record<string, string>>;
  readonly introductionLabels: Readonly<Record<string, string>>;
  readonly groupLabels: Readonly<Record<string, string>>;
};

export const EMPTY_APP_LABELS_OVERLAY: PluginAppLabelsOverlay = {
  windowKindLabels: {},
  panelTabLabels: {},
  modeLabels: {},
  actionLabels: {},
  utilityLabels: {},
  exampleLabels: {},
  actionArgLabels: {},
  dialogLabels: {},
  introductionLabels: {},
  groupLabels: {},
};

/** 🗣️ Rust's `skip_serializing_if` omits empty maps entirely, so a parsed overlay may be missing keys — fill them back in. */
export function normalizeAppLabelsOverlay(raw: Partial<PluginAppLabelsOverlay> | null | undefined): PluginAppLabelsOverlay {
  return {
    windowKindLabels: raw?.windowKindLabels ?? {},
    panelTabLabels: raw?.panelTabLabels ?? {},
    modeLabels: raw?.modeLabels ?? {},
    actionLabels: raw?.actionLabels ?? {},
    utilityLabels: raw?.utilityLabels ?? {},
    exampleLabels: raw?.exampleLabels ?? {},
    actionArgLabels: raw?.actionArgLabels ?? {},
    dialogLabels: raw?.dialogLabels ?? {},
    introductionLabels: raw?.introductionLabels ?? {},
    groupLabels: raw?.groupLabels ?? {},
  };
}

export type PluginContribution =
  | {
      readonly kind: "playbookBlockKind";
      readonly appId: string;
      readonly blockKind: string;
      readonly label: string;
      readonly iconId: IconName;
      readonly defaultValueJson?: string;
      readonly paramsBodyKey: string;
      readonly previewBodyKey: string;
    }
  | {
      readonly kind: "sourcingModule";
      readonly appId: string;
      readonly moduleId: string;
      readonly label: string;
      readonly iconId: IconName;
      readonly typologyJson: string;
      readonly kindsJson: string;
    };

export type ProgramContributionEntry = {
  readonly pluginId: string;
  readonly contribution: PluginContribution;
};

export type PluginManifest = {
  readonly pluginId: string;
  readonly label: string;
  readonly version: string;
  readonly apps: readonly Record<string, unknown>[];
  readonly workflows: readonly {
    readonly workflowStepId: string;
    readonly appId: string;
    readonly label: string;
    readonly document?: readonly string[];
    readonly yields: string;
  }[];
  readonly examples: readonly { readonly id: string; readonly label: string; readonly documentJson: string; readonly appId: string }[];
  readonly contributions?: readonly PluginContribution[];
  /** 🎛️ Plugin-scope commands this plugin exposes — apply whenever any of its apps is focused. */
  readonly commands?: readonly CommandDefinition[];
};

//#region AppManifestProtocol
/** 🧬️ Generated from Rust `WindowMeasure`/`WindowEngagement*` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type WindowMeasure = GeneratedWindowMeasure;
export type WindowEngagementOption = GeneratedWindowEngagementOption;
export type WindowEngagementInput = GeneratedWindowEngagementInput;
export type WindowEngagementStatus = GeneratedWindowEngagementStatus;
export type WindowEngagementPossible = GeneratedWindowEngagementPossible;
export type WindowEngagementRingOption = GeneratedWindowEngagementRingOption;
export type WindowEngagementToggleGroupOption = GeneratedWindowEngagementToggleGroupOption;
export type WindowEngagementSelectItem = GeneratedWindowEngagementSelectItem;
export type WindowEngagementControl = GeneratedWindowEngagementControl;
export type WindowEngagement = GeneratedWindowEngagement;

/** 🌳️ Mirrors Rust `PanelTabKind` — closes the informal `FRAMEWORK_CATEGORY_*`/`*_TAB_ID` string-constant convention: every panel tab is either a framework-predefined kind (exhaustively switchable) or an app-declared custom tab (`{ kind: "app", id }`). */
export type PanelTabKind = GeneratedPanelTabKind;
/** 🔤️ Flat string key for a `PanelTabKind` — mirrors Rust `PanelTabKind::id_str()`. Use for React `key=` props and legacy string-id matching. */
export function panelTabKindId(kind: PanelTabKind): string {
  switch (kind.kind) {
    case "workbenchCategory":
      return "framework.category.workbench";
    case "displayCategory":
      return "framework.category.display";
    case "detailsCategory":
      return "framework.category.details";
    case "settingsCategory":
      return "framework.category.settings";
    case "displayWindows":
      return "framework.display.windows";
    case "displayLayout":
      return "framework.display.layout";
    case "settingsGeneral":
      return "framework.settings.general";
    case "settingsTheme":
      return "framework.settings.theme";
    case "app":
      return kind.id;
  }
}

/** 🌳️ Mirrors Rust `PanelTabDefinition` — a leaf carries `bodyKey`, a branch carries `children`; `group` is only meaningful on root entries. */
export type AppPanelTabDefinition = GeneratedPanelTabDefinition;

/** 📦️ Mirrors Rust `AppDefinition` — generated 1:1 from `framework/core/rs/lib.rs` via ts-rs, except
 * `defaultLayout`/`namedLayouts` which keep this file's narrower hand-refined `WindowLayout` (ts-rs
 * widens `WindowLayoutAxisNode.kind`/`WindowLayoutStackNode.kind` to plain `string` since the Rust
 * field is a runtime `String`, not an enum — the narrower `"row" | "column" | "stack" | "window"`
 * literal unions here are domain knowledge worth keeping for exhaustive switches). */
export type AppActionDefinition = Omit<GeneratedActionDefinition, "iconId"> & { readonly iconId?: IconName };
export type AppUtilityDefinition = Omit<GeneratedUtilityDefinition, "iconId"> & { readonly iconId: IconName };
export type AppToolDefinition = Omit<GeneratedToolDefinition, "iconId"> & { readonly iconId: IconName };
export type AppCommandDefinition = Omit<GeneratedCommandDefinition, "iconId"> & { readonly iconId?: IconName };
export type AppWindowKindDefinition = Omit<GeneratedWindowKindDefinition, "iconId"> & { readonly iconId: IconName };
export type AppDefinition = Omit<GeneratedAppDefinition, "defaultLayout" | "namedLayouts" | "iconId"> & {
  readonly defaultLayout?: WindowLayout;
  readonly namedLayouts: readonly NamedLayout[];
  readonly iconId?: IconName;
  /** 🎬️ TODO(core-rs): fold into `GeneratedAppDefinition.tutorials` once typegen is unblocked (see the
   * `//#region 🎬️Tutorial` TODO above) — same field name/shape. */
  readonly tutorials: readonly TutorialDefinition[];
};
export type AppModeDefinition = GeneratedModeDefinition;
export type AppWindowOptions = GeneratedWindowOptions;
export type AppWindowEngagementSlot = GeneratedWindowEngagementSlot;
export type AppActionRef = GeneratedActionRef;
export type AppPanelGroup = GeneratedPanelGroup;

export type ProgramHotSwapEvent = {
  readonly pluginId: string;
  readonly version: string;
  readonly addedApps: readonly string[];
  readonly removedApps: readonly string[];
};
//#endregion AppManifestProtocol

//#region UiRefresh
/** @emoji 🐢️ One requested window/panel section — `bodyKey` only applies to windows/panels; `hash` is the host's known fnv1a-64 hex of that section's last payload, or absent on first fetch. */
export type PluginUiRefreshSectionRequest = { readonly key: string; readonly bodyKey?: string; readonly hash?: string };

/** @emoji 🐢️ One batched, hash-conditional refresh request — one round trip for the window/panel/engagements/measures/labels sections. Utility bars are no longer a plugin section: the renderer derives them from the utility registry via {@link deriveUtilityNodes}. */
export type PluginUiRefreshRequest = {
  readonly viewState: PluginViewState;
  readonly windows?: readonly PluginUiRefreshSectionRequest[];
  readonly panels?: readonly PluginUiRefreshSectionRequest[];
  readonly engagements?: { readonly hash?: string };
  readonly measures?: { readonly hash?: string };
  /** 🛠️ Mode-level tool measures, keyed by tool id — see `DocumentApp::tool_measures`. */
  readonly tools?: { readonly hash?: string };
  readonly labels?: { readonly hash?: string };
};

/** @emoji 🐢️ `value` is present only when `hash` differs from what the request supplied — an unchanged section costs one hash compare instead of a full re-serialize. */
export type PluginUiRefreshSectionResponse = { readonly key: string; readonly hash: string; readonly value?: unknown };

export type PluginUiRefreshResponse = {
  readonly windows?: readonly PluginUiRefreshSectionResponse[];
  readonly panels?: readonly PluginUiRefreshSectionResponse[];
  readonly engagements?: PluginUiRefreshSectionResponse;
  readonly measures?: PluginUiRefreshSectionResponse;
  readonly tools?: PluginUiRefreshSectionResponse;
  readonly labels?: PluginUiRefreshSectionResponse;
  /** ⏱️ See `DocumentApp::pending_effects` — background work (e.g. a `flowEvalTick` chain) the host
   * should dispatch right after this refresh, fed through the same `applyHostEffects` pass as an
   * action's own `requestedEffects`. */
  readonly requestedEffects?: readonly HostEffect[];
};
//#endregion UiRefresh

//#region 🖱️ContextMenu
/** @emoji 🖱️ Scene-target info for an on-demand context-menu request — hit-test results from the
 * surface's own picking (hover/selection), not cached across clicks. */
export type ContextMenuHit = {
  readonly domain: string;
  readonly id: string;
  readonly label?: string;
};

export type ContextMenuSelectionGroup = {
  readonly domain: string;
  readonly ids: readonly string[];
};

export type ContextMenuTextContext = {
  readonly caret: number;
  readonly hasSelection: boolean;
  readonly word?: string;
  readonly canRename: boolean;
  readonly hasCompletions: boolean;
};

export type PluginContextMenuSurfaceTarget = {
  readonly surfaceId: string;
  readonly kind: string;
  readonly hits?: readonly ContextMenuHit[];
  readonly selection?: readonly ContextMenuSelectionGroup[];
  readonly text?: ContextMenuTextContext;
};

export type PluginContextMenuPoint = { readonly x: number; readonly y: number };

/** @emoji 🖱️ On-demand context-menu request — never cached, never batched into {@link PluginUiRefreshRequest}.
 * `menu` is the {@link UiMenuRef} the host resolved from `data-menu-id`/a scene surface convention id
 * (`"world3d"`, `"nodeGraph"`, `"window"`, `"panel:<tabId>"`, ...). */
export type PluginContextMenuRequest = {
  readonly menu: UiMenuRef;
  readonly surface?: PluginContextMenuSurfaceTarget;
  readonly windowInstanceId?: string;
  readonly point?: PluginContextMenuPoint;
};

export type PluginContextMenuResponse = {
  readonly items: readonly ContextMenuItemSpec[];
};
//#endregion 🖱️ContextMenu

/**
 * 📡️ Host-facing shape of one loaded plugin, mirroring the 5-function `semio:framework/plugin` WIT
 * ABI exactly (`world.wit`): `manifest`/`instantiate-app`(as `createApp`)/`exchange` are the whole
 * runtime surface now — every former per-verb call (`handleAction`, `render`, `refreshUi`,
 * `contextMenu`, ...) is a binary `protocol_channel::AppCommand` sent through {@link exchange}
 * instead (see `🔖️AppChannelClient` in the os-product package, which frames these bytes). `dispose`
 * remains host-side only (never part of the WIT ABI) for worker/resource teardown.
 */
export type PluginWasmHandle = {
  readonly manifest: () => Promise<Uint8Array>;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  readonly exchange: (instanceId: number, frames: Uint8Array[]) => Promise<Uint8Array[]>;
  readonly dispose: () => void;
};

export function buildContributionsJson(loaded: ReadonlyArray<{ readonly pluginId: string; readonly manifest: PluginManifest }>): string {
  const entries: ProgramContributionEntry[] = [];
  for (const entry of loaded) {
    for (const contribution of entry.manifest.contributions ?? []) {
      entries.push({ pluginId: entry.pluginId, contribution });
    }
  }
  return JSON.stringify(entries);
}

export function resolveLayoutForMode(
  app: { readonly defaultLayout?: WindowLayout; readonly namedLayouts?: readonly NamedLayout[]; readonly modes: readonly { readonly id: string; readonly layoutId?: string }[] },
  modeId: string,
): WindowLayout | undefined {
  const mode = app.modes.find((entry) => entry.id === modeId);
  if (mode?.layoutId) {
    const named = app.namedLayouts?.find((entry) => entry.id === mode.layoutId);
    if (named) return named.layout;
  }
  return app.defaultLayout;
}

//#region 🧰️ActionArgsAndUtilities
/** 🧰️ A resolved utility ready for the utility bar — the TS twin of Rust `DerivedUtilitySpec` in `ui_wgpu`. */
export type DerivedUtilitySpec = {
  readonly id: string;
  readonly label: string;
  readonly iconId: IconName;
  readonly group?: string;
  readonly groupLabel?: string;
  readonly category?: UtilityCategory;
};

/**
 * 🧰️ Hand-written twin of Rust `derive_utility_nodes` (`framework/ui/wgpu/rs/lib.rs`): builds the utility bar node tree
 * from resolved utilities + the host-owned active utility id. Each utility becomes a `toggle` whose `pressed`
 * reflects `activeUtilityId === id` and whose `onChange` dispatches `setActiveUtility { utilityId }`; utilities
 * sharing a `group` collapse into one `collection` placed where the group first appears. A group that ends
 * with exactly one child is hoisted to a top-level toggle (no nested Transform/Transform pair).
 */
export function deriveUtilityNodes(controllerId: string, utilities: readonly DerivedUtilitySpec[], activeUtilityId?: string): UtilityNode[] {
  const toggle = (utility: DerivedUtilitySpec): UtilityNode => ({
    id: utility.id,
    kind: "toggle",
    iconId: utility.iconId,
    label: utility.label,
    title: utility.label,
    pressed: activeUtilityId === utility.id,
    category: utility.category,
    onChange: { controllerId, action: SET_ACTIVE_UTILITY_ACTION_ID, args: { utilityId: utility.id } },
  });
  const nodes: UtilityNode[] = [];
  const groupIndex = new Map<string, number>();
  for (const utility of utilities) {
    const node = toggle(utility);
    if (utility.group === undefined) {
      nodes.push(node);
      continue;
    }
    const existing = groupIndex.get(utility.group);
    if (existing !== undefined) {
      const collection = nodes[existing] as Extract<UtilityNode, { kind: "collection" }>;
      (collection.children as UtilityNode[]).push(node);
    } else {
      groupIndex.set(utility.group, nodes.length);
      const groupLabel = utility.groupLabel ?? utility.group;
      nodes.push({ id: `group:${utility.group}`, kind: "collection", iconId: utility.iconId, label: groupLabel, title: groupLabel, category: utility.category, children: [node] });
    }
  }
  return nodes.map((node) => (node.kind === "collection" && node.children.length === 1 ? node.children[0]! : node));
}

/**
 * 🎯️ Hand-written twin of Rust `partition_window_measures` (`framework/ui/wgpu/rs/lib.rs`): splits a window's
 * top-level measures into `general` and `utilityOptions`. A top-level `group` tagged with `activeUtilityId`
 * contributes its **children** to `utilityOptions` only when it equals the window's active utility (the
 * tagged wrapper is routing-only and never rendered), and is dropped from both buckets otherwise. Untagged
 * groups and non-group top-level measures stay in `general`, unchanged.
 */
export function partitionWindowMeasures(measures: readonly WindowMeasure[], activeUtilityId?: string): { readonly general: WindowMeasure[]; readonly utilityOptions: WindowMeasure[] } {
  const general: WindowMeasure[] = [];
  const utilityOptions: WindowMeasure[] = [];
  for (const measure of measures) {
    if (measure.kind === "group" && measure.activeUtilityId !== undefined) {
      if (measure.activeUtilityId === activeUtilityId) utilityOptions.push(...measure.children);
      continue;
    }
    general.push(measure);
  }
  return { general, utilityOptions };
}

/**
 * 🧮️ Hand-written twin of Rust `effective_action_args`: for each declared arg, the staged value if
 * present, else its declared `default`, else omitted.
 */
export function effectiveActionArgs(defs: readonly ActionArgDef[], staged: Readonly<Record<string, unknown>>): Record<string, unknown> {
  const effective: Record<string, unknown> = {};
  for (const def of defs) {
    if (Object.prototype.hasOwnProperty.call(staged, def.id)) {
      effective[def.id] = staged[def.id];
    } else if (def.default !== undefined && def.default !== null) {
      effective[def.id] = def.default;
    }
  }
  return effective;
}

/**
 * ❗️ Hand-written twin of Rust `missing_required_args`: ids of required args still unset in `effective`
 * (absent, null, or an empty string).
 */
export function missingRequiredArgs(defs: readonly ActionArgDef[], effective: Readonly<Record<string, unknown>>): string[] {
  return defs
    .filter((def) => def.required)
    .filter((def) => {
      const value = effective[def.id];
      return value === undefined || value === null || value === "";
    })
    .map((def) => def.id);
}

/**
 * 📇️ Hand-written twin of Rust `resolve_window_actions`: explicit `windowKind.actions` refs resolve in
 * order, plus any panel-eligible app action referenced by no window kind (an orphan) appears on every
 * window — the scoping fallback that prevents blank panels mid-migration. History and `setActiveUtility`
 * are never panel-eligible orphans.
 */
export function resolveWindowActions(
  app: { readonly actions?: readonly ActionDefinition[]; readonly windowKinds: readonly { readonly actions?: readonly AppActionRef[] }[] },
  windowKind: { readonly actions?: readonly AppActionRef[] },
): ActionDefinition[] {
  const actions = app.actions ?? [];
  const referenced = new Set<string>();
  for (const kind of app.windowKinds) {
    for (const ref of kind.actions ?? []) referenced.add(ref);
  }
  const panelEligible = (action: ActionDefinition) => action.kind !== "history" && action.id !== SET_ACTIVE_UTILITY_ACTION_ID && action.id !== SET_ACTIVE_TOOL_ACTION_ID;
  const resolved: ActionDefinition[] = [];
  const seen = new Set<string>();
  for (const ref of windowKind.actions ?? []) {
    const action = actions.find((entry) => entry.id === ref);
    if (action && !seen.has(action.id)) {
      seen.add(action.id);
      resolved.push(action);
    }
  }
  for (const action of actions) {
    if (panelEligible(action) && !referenced.has(action.id) && !seen.has(action.id)) {
      seen.add(action.id);
      resolved.push(action);
    }
  }
  return resolved;
}

/**
 * 🛠️ Hand-written twin of Rust `resolve_mode_tools`: resolves the active mode's tools in declared
 * order. Unlike `resolveWindowActions`, unresolvable or unreferenced tools have no orphan fallback —
 * tools are opt-in per mode, never automatically shown everywhere.
 */
export function resolveModeTools(
  app: { readonly tools?: readonly ToolDefinition[]; readonly modes: readonly { readonly id: string; readonly tools?: readonly ToolRef[] }[] } | undefined,
  activeModeId: string | undefined,
): ToolDefinition[] {
  const tools = app?.tools ?? [];
  const mode = app?.modes.find((entry) => entry.id === activeModeId);
  if (!mode) return [];
  const resolved: ToolDefinition[] = [];
  const seen = new Set<string>();
  for (const ref of mode.tools ?? []) {
    const tool = tools.find((entry) => entry.id === ref);
    if (tool && !seen.has(tool.id)) {
      seen.add(tool.id);
      resolved.push(tool);
    }
  }
  return resolved;
}
//#endregion 🧰️ActionArgsAndUtilities

/**
 * 🧩️ Expands a plugin registry for a primary plugin: `primaryPluginId` is matched directly
 * against entry `pluginId` (no registry-id indirection), then every other entry whose
 * `contributes` intersects the primary entry's `consumes` is appended. Studio mode, or the
 * absence of a primary id, passes the full registry through unchanged.
 */
export function expandPluginRegistry(plugins: readonly PluginRegistryEntry[], primaryPluginId?: string, studioMode = false): readonly PluginRegistryEntry[] {
  if (studioMode || !primaryPluginId) return plugins;
  const primaryEntries = plugins.filter((entry) => entry.pluginId === primaryPluginId);
  const consumes = new Set(primaryEntries.flatMap((entry) => entry.consumes ?? []));
  const contributorEntries = plugins.filter((entry) => entry.pluginId !== primaryPluginId && (entry.contributes ?? []).some((tag) => consumes.has(tag)));
  return [...primaryEntries, ...contributorEntries];
}

export type ExternalSlotResolverContext = {
  readonly plugins: ReadonlyMap<string, PluginWasmHandle>;
  readonly contributorInstances: Map<string, number>;
  readonly viewState: PluginViewState;
};

export async function ensureContributorInstance(pluginId: string, appId: string, context: ExternalSlotResolverContext): Promise<number | null> {
  const existing = context.contributorInstances.get(pluginId);
  if (existing != null) return existing;
  const handle = context.plugins.get(pluginId);
  if (!handle) return null;
  const instanceId = await handle.createApp(appId);
  context.contributorInstances.set(pluginId, instanceId);
  return instanceId;
}

export async function resolveExternalSlots(node: PluginUiNode, context: ExternalSlotResolverContext): Promise<PluginUiNode> {
  if (node.type === "externalSlot") {
    const pluginId = String(node.pluginId ?? "");
    const appId = String(node.appId ?? pluginId);
    const handle = context.plugins.get(pluginId);
    if (!handle) {
      return { type: "text", value: `Extension unavailable: ${pluginId}` };
    }
    const instanceId = await ensureContributorInstance(pluginId, appId, context);
    if (instanceId == null) {
      return { type: "text", value: `Extension unavailable: ${pluginId}` };
    }
    // 🚧️ Rendering a contributor's UI body now goes through `AppChannelClient.refreshUi`
    // (`RefreshUi` → `UiSection` over `exchange`, os-product `🔖️AppChannelClient` region) instead
    // of the removed per-verb `render`/`renderWithDocument`. Wiring that dispatch loop into this
    // exact call site is the dedicated follow-up work package this ticket flags for the React
    // renderer's dispatch/refresh loops — until then an external slot degrades to unavailable
    // rather than silently guessing at `SectionProbe.kind`/body-key framing.
    return { type: "text", value: `Extension unavailable: ${pluginId}` };
  }
  if (node.type === "stack" && Array.isArray(node.children)) {
    const children = await Promise.all(node.children.map((child) => resolveExternalSlots(child as PluginUiNode, context)));
    return { ...node, children };
  }
  if (node.type === "section" && Array.isArray(node.children)) {
    const children = await Promise.all(node.children.map((child) => resolveExternalSlots(child as PluginUiNode, context)));
    return { ...node, children };
  }
  return node;
}

export type PluginRegistryEntry = {
  readonly pluginId: string;
  readonly moduleUrl: string;
  readonly contributes?: readonly string[];
  readonly consumes?: readonly string[];
};

//#region InvocationResponse
/** @emoji 🕰️ Hybrid logical clock stamp carried by every kernel operation. */
export type HybridLogicalTimestamp = { readonly wall: number; readonly counter: number };

/** @emoji 🩹️ A schema-tagged document mutation payload (forward diff or inverse diff). */
export type DocumentDiff = { readonly schemaId: string; readonly payload: unknown };

/** @emoji ↩️ Undo semantics for a single kernel operation. */
export type UndoPolicy = "exactBaseOnly" | "transformAgainstConcurrent" | "semanticUndo" | "compensatingAction";

/** @emoji ↩️ The true inverse of a kernel operation, recorded from the store's `Edit.backwards`. */
export type InverseOperation = {
  readonly targetOperation: string;
  readonly inverseDiff: DocumentDiff;
  readonly baseVersion: number;
  readonly dependencies?: readonly string[];
  readonly undoPolicy: UndoPolicy;
};

/** @emoji 🔁️ One typed document operation with its true inverse — the CQRS wire unit. */
export type KernelOperation = {
  readonly id: string;
  readonly document: number;
  readonly baseVersion: number;
  readonly invocationId: string;
  readonly diff: DocumentDiff;
  readonly inverse: InverseOperation;
  readonly dependencies?: readonly string[];
  readonly author: string;
  readonly timestamp: HybridLogicalTimestamp;
};

/** @emoji 🎁️ The undo group binding an invocation (action or command) to its operations + inverses. */
export type UndoGroup = {
  readonly invocationId: string;
  readonly operations: readonly string[];
  readonly inverseOperations: readonly InverseOperation[];
};

/** @emoji 📣️ An out-of-band app event surfaced to the shell (e.g. history changed). */
export type AppEvent = { readonly kind: string; readonly payload: unknown };

/** @emoji 🩺️ Canonical severity for faults and diagnostics. */
export type Severity = "fatal" | "error" | "warning" | "hint";

/** @emoji 🧭️ Layer that produced a fault. */
export type FaultOrigin = "edge" | "renderer" | "os" | "module" | "plugin" | "app" | "extension";

export type FaultScope = {
  readonly pluginId?: string;
  readonly appId?: string;
  readonly instanceId?: string;
  readonly module?: string;
  readonly bodyKey?: string;
};

export type FaultCause = { readonly message: string; readonly code?: string };

export type TextSpan = { readonly line: number; readonly column: number; readonly length: number };

/** @emoji 🧯️ Structured abort report shared across Rust, WIT, and TypeScript. */
export type Fault = {
  readonly origin: FaultOrigin;
  readonly code: string;
  readonly severity: Severity;
  readonly message: string;
  readonly scope: FaultScope;
  readonly span?: TextSpan;
  readonly causes?: readonly FaultCause[];
  readonly retryable: boolean;
};

/** @emoji 🩺️ A diagnostic emitted alongside an action result. */
export type Diagnostic = {
  readonly code: string;
  readonly severity: Severity;
  readonly message: string;
  readonly scope?: FaultScope;
  readonly span?: TextSpan;
};

/** @emoji 🧯️ Error subclass carrying a structured {@link Fault}. */
export class SemioFaultError extends Error {
  readonly fault: Fault;
  constructor(fault: Fault) {
    super(fault.message);
    this.name = "SemioFaultError";
    this.fault = fault;
  }
}

/**
 * @emoji 🐚️ A typed side effect the shell performs on the app's behalf. Mirrors the Rust
 * `HostEffect` enum (externally tagged: unit variants are the plain tag string, struct variants are
 * a single-key object keyed by the camelCase variant name).
 */
export type HostEffect =
  | "requestSync"
  | { readonly openWindow: { readonly kind: string; readonly params: unknown } }
  | { readonly closeWindow: { readonly window: number } }
  | { readonly notify: { readonly message: string } }
  | { readonly navigate: { readonly uri: string } }
  /** @emoji 📂️ Replaces the active app instance's document with a VCS envelope JSON — host-owned
   * counterpart of `loadAppDocument` for catalog/example studio opens. */
  | { readonly loadDocument: { readonly pack?: readonly number[]; readonly spr?: readonly number[]; readonly documentJson?: string } }
  | { readonly openExternalUrl: { readonly url: string } }
  | { readonly setPanel: { readonly panelJson: string } }
  | { readonly downloadMediaExport: { readonly filename: string; readonly mimeType: string; readonly data: string; readonly encoding?: string } }
  | { readonly iconRenderExport: { readonly items: readonly { readonly filename: string; readonly request: unknown }[] } }
  | { readonly requestFileOpen: { readonly accept: string; readonly readAs?: string; readonly importAction: string; readonly multiple?: boolean } }
  /** @emoji 🎞️ Asks the shell to decode a video (file picker, or `payload` bytes already in hand)
   * and re-dispatch `frameAction` once per sampled frame with `{payload: dataUrl(image/jpeg), name,
   * frameIndex, timestampMs, index, total, width, height, ...args}`, then `doneAction` once with
   * `{name, durationMs, frameCount, sampledCount, width, height, codec, ...args}`; if the host can't
   * decode it, `fallbackAction` fires once with `{payload: dataUrl(raw bytes), name, ...args}`. The
   * numeric hints (`sampleStride`/`maxFrames`/`maxLongEdgePx`/`fpsHint`) are 0 when the caller wants
   * the host default. */
  | {
      readonly requestMediaFrames: {
        readonly accept: string;
        readonly frameAction: string;
        readonly doneAction: string;
        readonly fallbackAction: string;
        readonly sampleStride?: number;
        readonly maxFrames?: number;
        readonly maxLongEdgePx?: number;
        readonly fpsHint?: number;
        readonly payload?: string;
        readonly args?: unknown;
      };
    }
  | { readonly spawnPluginInstance: { readonly pluginId: string; readonly appId: string; readonly osInstanceId?: string; readonly label?: string; readonly documentJson?: string } }
  | { readonly openPluginInstance: { readonly pluginId: string; readonly appId: string; readonly osInstanceId?: string } }
  | { readonly setActiveUtility: { readonly windowId: string; readonly utilityId: string } }
  /** 🛠️ Programmatically switches the host-owned active tool of the active mode — the effect form of
   * `setActiveTool`. Empty `toolId` deactivates the current tool. */
  | { readonly setActiveTool: { readonly toolId: string } }
  | { readonly openDialog: { readonly dialogId: string; readonly args?: Record<string, unknown> } }
  /** @emoji 🔁️ Re-dispatches `action` onto the same plugin instance after `delayMs` — lets a program
   * advance staged/progressive work over several ticks without blocking the host; the response's own
   * `requestedEffects` are fed back through `applyHostEffects` recursively. */
  | { readonly dispatchAction: { readonly action: string; readonly args?: unknown; readonly delayMs: number } }
  /** @emoji 🎯️ Patches world-3d selection chrome and document-tree `selectedIds` without a composite re-render. */
  | {
      readonly patchWorld3dChrome: {
        readonly selectionJson: string;
        readonly vorticesJson?: string;
        readonly documentSelectedIds: readonly string[];
        readonly documentHighlightedIds?: readonly string[];
      };
    }
  | {
      readonly requestPluginExchange: {
        readonly pluginId: string;
        readonly appId: string;
        readonly requestJson: string;
        readonly responseAction: string;
      };
    };

/**
 * @emoji 🐢️ Mirrors the Rust `UiDirtyScope` — which rendered UI sections an action actually
 * invalidates. Absent (`undefined`) on an `InvocationResponse` means the same as the Rust side's missing
 * field: treat as `{kind: "full"}` (see {@link resolveUiDirtyScope}) — every program that doesn't emit
 * this yet keeps today's whole-shell-refresh behavior.
 */
export type UiDirtyScope =
  | { readonly kind: "full" }
  | { readonly kind: "none" }
  | {
      readonly kind: "partial";
      readonly windowBodies?: readonly string[];
      readonly panelBodies?: readonly string[];
      readonly utilities?: boolean;
      readonly tools?: boolean;
      readonly engagements?: boolean;
      readonly measures?: boolean;
      readonly labels?: boolean;
    };

/** @emoji 🐢️ Normalizes a possibly-absent `UiDirtyScope` — missing (older program, or a response built without one) means `full`. */
export function resolveUiDirtyScope(scope: UiDirtyScope | undefined): UiDirtyScope {
  return scope ?? { kind: "full" };
}

/**
 * @emoji 📤️ Typed result of a plugin `handle-action`/`handle-command` call — mirrors the Rust
 * `InvocationResult`. Replaces the legacy `string[]` JSON-patch shape: operations are now typed
 * `KernelOperation`s with true inverses, and the shell applies `requestedEffects` through
 * `applyHostEffects` (WS-E).
 */
export type InvocationResponse = {
  readonly output: unknown;
  readonly operations: readonly KernelOperation[];
  readonly inverseGroup: UndoGroup;
  readonly diagnostics?: readonly Diagnostic[];
  readonly requestedEffects?: readonly HostEffect[];
  readonly events?: readonly AppEvent[];
  readonly uiScope?: UiDirtyScope;
};

// 🐢️ `uiScope` deliberately left unset here (not `{kind: "none"}`) — `resolveUiDirtyScope` treats a
// missing scope as `full`, the safe default for the rare failure paths that return this constant
// (unparseable response, stub module missing `handleAction`/`handleCommand`).
const EMPTY_INVOCATION_RESPONSE: InvocationResponse = {
  output: null,
  operations: [],
  inverseGroup: { invocationId: "", operations: [], inverseOperations: [] },
};

/** @emoji 📥️ Parses a raw program `handle-action`/`handle-command` response string into a typed {@link InvocationResponse}. */
export function parseInvocationResponse(raw: string): InvocationResponse {
  try {
    const parsed = JSON.parse(raw) as Partial<InvocationResponse> | null;
    if (parsed && typeof parsed === "object" && Array.isArray(parsed.operations)) {
      return parsed as InvocationResponse;
    }
  } catch {
    // fall through to the empty response
  }
  return EMPTY_INVOCATION_RESPONSE;
}
//#endregion InvocationResponse

//#region SerializedPluginWasm
/** @emoji 🧾️ Flattens jco/component errors — message is often `[object Object] (see error.payload)` while the real text lives on `payload.val`. */
export function pluginErrorText(error: unknown): string {
  if (error instanceof Error) {
    const withPayload = error as Error & { payload?: unknown };
    const payload = withPayload.payload;
    if (payload && typeof payload === "object") {
      const record = payload as { val?: unknown; tag?: unknown; message?: unknown };
      if (typeof record.val === "string" && record.val.length > 0) {
        return `${withPayload.message} payload=${JSON.stringify(payload)}`;
      }
      if (typeof record.message === "string" && record.message.length > 0) {
        return `${withPayload.message} payload=${JSON.stringify(payload)}`;
      }
    }
    return withPayload.message;
  }
  if (error && typeof error === "object" && "payload" in error) {
    try {
      return JSON.stringify(error);
    } catch {
      return String(error);
    }
  }
  return String(error);
}

/** @emoji 🔒️ True when a plugin call hit the single-flight instance lock (or a poisoned guard after a trap). */
export function isPluginInstanceBusyError(error: unknown): boolean {
  const message = pluginErrorText(error);
  return message.includes("plugin instance busy") || message.includes("plugin busy");
}

/** @emoji 🔒️ Serializes wasm program entry points — the host keeps instances in one RefCell. */
export function withSerializedPluginWasmHandle(handle: PluginWasmHandle): PluginWasmHandle {
  let tail: Promise<void> = Promise.resolve();
  const runSerialized = <T>(fn: () => Promise<T>): Promise<T> => {
    const job = tail.then(async () => {
      for (let attempt = 0; attempt < 8; attempt += 1) {
        try {
          return await fn();
        } catch (error) {
          if (!isPluginInstanceBusyError(error)) throw error;
          await new Promise((resolve) => setTimeout(resolve, attempt + 1));
        }
      }
      return fn();
    });
    tail = job.then(
      () => undefined,
      () => undefined,
    );
    return job;
  };
  return {
    manifest: () => runSerialized(() => handle.manifest()),
    createApp: (appId) => runSerialized(() => handle.createApp(appId)),
    destroyApp: (instanceId) => runSerialized(() => handle.destroyApp(instanceId)),
    exchange: (instanceId, frames) => runSerialized(() => handle.exchange(instanceId, frames)),
    dispose: handle.dispose,
  };
}
//#endregion SerializedPluginWasm

//#region PluginWorkerClient
/** @emoji 🧵️ Message types the generated `🟨️plugin-worker.js` dispatches (framework/os/dev/script.ts `pluginWorkerSource`). */
type PluginWorkerMessageType = "init" | "manifest" | "createApp" | "destroy" | "exchange" | "error";

/** @emoji ⏱️ Logs only, never kills the worker — a plugin action owns in-flight, possibly undo-relevant
 * state, so abandoning it mid-call (the wgpu renderer's timeout+restart policy) would corrupt it. */
const PLUGIN_WORKER_UNRESPONSIVE_MS = 10000;

/** @emoji 🔌️ Derives the generic worker bootstrap script's URL from a plugin module URL — same directory,
 * `🟨️plugin-worker.js` instead of the plugin's own bridge filename. The bootstrap script itself never
 * needs cache-busting (it's plugin-version-agnostic; the *actual* module URL, `?v=`-busted or not, only
 * ever travels as the `init` request's `moduleUrl` payload — see `start()` below), so any `?query` or
 * `#hash` on `moduleUrl` (from `PluginSource.moduleUrl`'s hot-reload cache-busting) is stripped first —
 * otherwise the trailing `.js` no longer sits at the string's end and the replace silently no-ops,
 * pointing the worker at the plugin's own module instead of its bootstrap script. */
/** @emoji 🪶️ GUESTSLIM: the typst default font set (see `infinite_canvas`'s `render` feature doc),
 * static-served alongside every plugin's own output at `_vendor/guestslim-typst-fonts.bin`
 * (`📇️registry/📜️script.ts`'s `ensureGuestSlimTypstFontsAsset`). Fetched once and reused across every
 * plugin worker this tab spins up — the file itself never changes at runtime (pinned crate version). */
const GUESTSLIM_TYPST_DEFAULT_FONTS_ASSET_HANDLE = 1;
let guestSlimTypstFontsPromise: Promise<ArrayBuffer> | null = null;

/** @emoji 🛡️ Best-effort: most plugins never call `read-asset` at all, and the guest-side Rust already
 * degrades gracefully (empty font list → typst compile yields no glyphs → `BoardResolvedIcon::None`)
 * when no reader is registered — so a fetch hiccup here must never block a plugin worker from booting. */
async function guestSlimAssetsForModule(moduleUrl: string): Promise<ReadonlyArray<readonly [number, ArrayBuffer]>> {
  guestSlimTypstFontsPromise ??= (async () => {
    const vendorUrl = moduleUrl.split(/[?#]/)[0]!.replace(/\/[^/]+\/[^/]+\.js$/, "/_vendor/guestslim-typst-fonts.bin");
    const response = await fetch(vendorUrl);
    if (!response.ok) throw new Error(`GuestSlim typst fonts asset fetch failed: ${response.status} ${vendorUrl}`);
    return response.arrayBuffer();
  })();
  try {
    const buffer = await guestSlimTypstFontsPromise;
    return [[GUESTSLIM_TYPST_DEFAULT_FONTS_ASSET_HANDLE, buffer]];
  } catch (error) {
    console.warn("[DEBUG] GuestSlim typst fonts asset unavailable; affected plugins fall back to blank typst/emoji/text icons", error);
    guestSlimTypstFontsPromise = null;
    return [];
  }
}

export function pluginWorkerUrl(moduleUrl: string): string {
  const bare = moduleUrl.split(/[?#]/)[0]!;
  return bare.replace(/\/[^/]+\.js$/, "/🟨️plugin-worker.js");
}

/**
 * @emoji 🧵️ Runs a component-model plugin's WASM inside a Web Worker so `handleAction` — including
 * long-running precompute — never blocks the UI thread. Mirrors `framework/os/renderer/wgpu/js/🟦️boot.ts`'s
 * `PluginWorkerClient`, minus its 5s timeout+restart.
 */
class PluginWorkerClient {
  private worker: Worker | null = null;
  private readonly pending = new Map<string, { resolve: (value: Record<string, unknown>) => void; reject: (error: Error) => void; watchdog: number }>();
  onBackboneOutbound?: (uri: string, message: Uint8Array) => void;

  constructor(
    private readonly pluginId: string,
    private readonly moduleUrl: string,
  ) {}

  private clearPending(error: Error): void {
    for (const [requestId, entry] of this.pending) {
      window.clearTimeout(entry.watchdog);
      entry.reject(error);
      this.pending.delete(requestId);
    }
  }

  private attachWorker(worker: Worker): void {
    worker.onmessage = (event: MessageEvent) => {
      const message = event.data as {
        requestId?: string;
        type?: PluginWorkerMessageType | "backboneOutbound";
        uri?: string;
        message?: string;
      };
      if (message.type === "backboneOutbound" && message.uri && message.message != null) {
        const bytes = message.message instanceof Uint8Array ? message.message : new Uint8Array(message.message as ArrayBuffer);
        this.onBackboneOutbound?.(message.uri, bytes);
        return;
      }
      const requestId = message.requestId;
      if (!requestId) return;
      const entry = this.pending.get(requestId);
      if (!entry) return;
      window.clearTimeout(entry.watchdog);
      this.pending.delete(requestId);
      if (message.type === "error") {
        entry.reject(new Error(message.message ?? `program worker ${this.pluginId} error`));
        return;
      }
      entry.resolve(message);
    };
    worker.onerror = (error) => {
      console.error(`[DEBUG] program worker ${this.pluginId} crashed`, error);
      this.worker = null;
      this.clearPending(new Error(`program worker ${this.pluginId} crashed`));
    };
  }

  async start(): Promise<void> {
    const worker = new Worker(pluginWorkerUrl(this.moduleUrl), { type: "module" });
    this.attachWorker(worker);
    this.worker = worker;
    // 🪶️ GUESTSLIM: structured-clone copy, not a transfer — `guestSlimAssetsForModule` caches and
    // reuses the same master `ArrayBuffer` across every plugin worker this tab starts; transferring
    // it would detach (neuter) it after the first worker, breaking every subsequent one.
    const guestSlimAssets = await guestSlimAssetsForModule(this.moduleUrl);
    await this.request("init", { moduleUrl: this.moduleUrl, guestSlimAssets });
  }

  private request(type: PluginWorkerMessageType, payload: Record<string, unknown>): Promise<Record<string, unknown>> {
    return new Promise((resolve, reject) => {
      if (!this.worker) {
        reject(new Error(`program worker ${this.pluginId} is not running`));
        return;
      }
      const requestId = crypto.randomUUID();
      const watchdog = window.setTimeout(() => {
        console.warn(`[DEBUG] program worker ${this.pluginId} unresponsive for ${PLUGIN_WORKER_UNRESPONSIVE_MS}ms: ${type}`);
      }, PLUGIN_WORKER_UNRESPONSIVE_MS);
      this.pending.set(requestId, { resolve, reject, watchdog });
      this.worker.postMessage({ type, requestId, ...payload });
    });
  }

  async manifest(): Promise<Uint8Array> {
    return ((await this.request("manifest", {})).value as Uint8Array | undefined) ?? new Uint8Array();
  }

  async createApp(appId: string): Promise<number> {
    return Number((await this.request("createApp", { appId })).instanceId);
  }

  async destroyApp(instanceId: number): Promise<void> {
    await this.request("destroy", { instanceId });
  }

  async exchange(instanceId: number, frames: Uint8Array[]): Promise<Uint8Array[]> {
    return ((await this.request("exchange", { instanceId, frames })).value as Uint8Array[] | undefined) ?? [];
  }

  dispose(): void {
    this.clearPending(new Error(`program worker ${this.pluginId} disposed`));
    this.worker?.terminate();
    this.worker = null;
  }

  postBackboneInbound(uri: string, messages: readonly Uint8Array[]): void {
    this.worker?.postMessage({ type: "backboneInbound", uri, messages });
  }
}

/**
 * @emoji 🧵️ Worker-backed `PluginWasmHandle` for component-model plugins (the ABI the generated
 * `🟨️plugin-worker.js` supports). Caller falls back to the direct main-thread import on failure (no
 * `🟨️plugin-worker.js` alongside this module, wasm-bindgen-only program, or `Worker` unavailable).
 *
 * Keyed by `moduleUrl` (not `pluginId`): a hot reload acquires a *second* worker at a fresh
 * cache-busted URL for the same `pluginId` while the old one still serves live instances, so a
 * `pluginId`-keyed map would have the new worker's `set()` silently clobber the old entry and then
 * the old worker's `dispose()` delete the new one out from under it. `activeWorkerByPluginId` tracks
 * which of a plugin's (possibly several, during a swap) worker clients is the one inbound backbone
 * traffic should reach.
 */
const pluginWorkerClients = new Map<string, PluginWorkerClient>();
const activeWorkerByPluginId = new Map<string, PluginWorkerClient>();

async function loadPluginModuleViaWorker(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  const client = new PluginWorkerClient(pluginId, moduleUrl);
  pluginWorkerClients.set(moduleUrl, client);
  client.onBackboneOutbound = (uri, message) => relayPluginBackboneOutbound(uri, message);
  await client.start();
  activeWorkerByPluginId.set(pluginId, client);
  console.log(`[DEBUG] plugin worker + ${pluginId} (${pluginWorkerClients.size} live)`);
  return withSerializedPluginWasmHandle({
    manifest: () => client.manifest(),
    createApp: (appId) => client.createApp(appId),
    destroyApp: (instanceId) => client.destroyApp(instanceId),
    exchange: (instanceId, frames) => client.exchange(instanceId, frames),
    dispose: () => {
      if (pluginWorkerClients.get(moduleUrl) === client) pluginWorkerClients.delete(moduleUrl);
      if (activeWorkerByPluginId.get(pluginId) === client) activeWorkerByPluginId.delete(pluginId);
      client.dispose();
      console.log(`[DEBUG] plugin worker - ${pluginId} (${pluginWorkerClients.size} live)`);
    },
  });
}
//#endregion PluginWorkerClient

export function relayPluginBackboneOutbound(uri: string, message: Uint8Array): void {
  pluginBackboneRoutes.get(pluginBackboneDocumentIdFromUri(uri))?.(uri, message);
}

/** @emoji 🌉️ A direct-import (main-thread, no-worker) plugin's generated `🟨️host-shim.js` runs in this
 * same realm but can't import from this module, so it reaches the outbound relay through this
 * well-known global instead — the same relay a worker-backed program reaches via `postMessage`. */
(globalThis as unknown as { __semioMainThreadPluginBackboneOutbound?: (uri: string, message: Uint8Array) => void }).__semioMainThreadPluginBackboneOutbound = relayPluginBackboneOutbound;

/** @emoji 🌉️ Inbound counterpart: pushes straight into the same global queue a direct-import plugin's
 * `🟨️host-shim.js` `backbonePoll` drains, keyed by `uri` (globally unique per document, so no pluginId
 * scoping is needed even though several plugins may share this realm). */
function pushMainThreadPluginBackboneInbound(uri: string, messages: readonly Uint8Array[]): void {
  const bridge = globalThis as unknown as { __semioBackboneInbound?: Map<string, Uint8Array[]> };
  const queue = bridge.__semioBackboneInbound ?? new Map<string, Uint8Array[]>();
  queue.set(uri, [...(queue.get(uri) ?? []), ...messages]);
  bridge.__semioBackboneInbound = queue;
}

export function postPluginBackboneInbound(pluginId: string, uri: string, messages: readonly Uint8Array[]): void {
  const client = activeWorkerByPluginId.get(pluginId);
  if (client) {
    client.postBackboneInbound(uri, messages);
    return;
  }
  pushMainThreadPluginBackboneInbound(uri, messages);
}

//#region 🐚️PluginBackboneRouting
/** @emoji 🐚️ Extracts the `<documentId>` a plugin's `actor://<documentId>` backbone uri names — the
 * `framework/sync` `ChannelBackbone::pair` convention (see the react renderer's `openDocument`). Falls
 * back to the whole uri for any other scheme so an unrecognized realm still gets a routing key instead
 * of being silently dropped. */
function pluginBackboneDocumentIdFromUri(uri: string): string {
  return uri.startsWith("actor://") ? uri.slice("actor://".length) : uri;
}

const pluginBackboneRoutes = new Map<string, (uri: string, message: Uint8Array) => void>();

/**
 * @emoji 🐚️ Routes a plugin's outbound backbone bytes for one document to whichever shell instance owns
 * it — replaces the old page-global relay slot (`setPluginBackboneOutboundRelay`), which a second
 * mounted shell silently overwrote: misrouting the first shell's document sync into the second shell's
 * backbone worker, then severing it entirely the moment that second shell unmounted (it cleared the
 * slot to `null`). Register at the same point a shell learns it owns `documentId` (the react renderer's
 * `openDocument`) and call the returned unregister function at the matching `closeDocument`/unmount.
 */
export function registerPluginBackboneRoute(documentId: string, relay: (uri: string, message: Uint8Array) => void): () => void {
  pluginBackboneRoutes.set(documentId, relay);
  return () => {
    if (pluginBackboneRoutes.get(documentId) === relay) pluginBackboneRoutes.delete(documentId);
  };
}
//#endregion 🐚️PluginBackboneRouting

//#region 🪶️LeasePool
/** @emoji 🪶️ One caller's reference to a {@link LeasePool}-managed resource. `release()` is idempotent —
 * a second call is a no-op — and drops this caller's refcount; the pool only disposes the underlying
 * resource once every issued lease on that key has released (and, unless `lingerMs` is 0, only after
 * the linger window below elapses with no re-acquire). */
export interface Lease<T> {
  readonly value: T;
  release(): void;
}

export interface LeasePoolStats {
  readonly key: string;
  readonly refs: number;
  readonly state: "loading" | "resident" | "lingering";
}

export interface LeasePool<T> {
  acquire(key: string): Promise<Lease<T>>;
  /** Forces disposal of `key` (or every entry when omitted) right now, bypassing any linger timer.
   * A no-op (logged, not thrown) for a key with active leases — evicting a resource a caller still
   * holds would leave that caller's `Lease.value` silently dead underneath it. */
  evictNow(key?: string): void;
  stats(): readonly LeasePoolStats[];
}

type LeasePoolEntry<T> = {
  readonly promise: Promise<T>;
  refs: number;
  lingerTimer: ReturnType<typeof setTimeout> | null;
  settled: T | undefined;
};

/**
 * @emoji 🪶️ Generic refcounted resource pool with linger-based eviction — the shared mechanism both
 * {@link acquirePluginModule} (plugin worker modules) and the renderer's engine-session cache build on
 * top of, instead of each hand-rolling its own refcounting. A resource loads once per `key` and is
 * shared by every caller; when the last lease on a key releases, the resource isn't disposed
 * immediately — it lingers for `lingerMs` (default 30s) so a caller that re-acquires the same key
 * shortly after (e.g. reopening a just-closed window) reuses the still-live resource instead of paying
 * full reload cost. `lingerMs: 0` disposes the instant refs hit zero, matching the pre-`LeasePool`
 * `acquirePluginModule` behavior exactly.
 */
export function createLeasePool<T>(load: (key: string) => Promise<T>, dispose: (value: T) => void, options?: { readonly lingerMs?: number; readonly label?: string }): LeasePool<T> {
  const lingerMs = options?.lingerMs ?? 30_000;
  const label = options?.label ?? "resource";
  const entries = new Map<string, LeasePoolEntry<T>>();

  function disposeEntry(key: string, entry: LeasePoolEntry<T>): void {
    if (entries.get(key) !== entry) return;
    entries.delete(key);
    if (entry.settled !== undefined) {
      console.log(`[DEBUG] ${label} evicted ${key}`);
      dispose(entry.settled);
    }
  }

  return {
    async acquire(key: string): Promise<Lease<T>> {
      let entry = entries.get(key);
      if (!entry) {
        const created: LeasePoolEntry<T> = { promise: load(key), refs: 0, lingerTimer: null, settled: undefined };
        created.promise.then(
          (value) => {
            created.settled = value;
          },
          () => {
            if (entries.get(key) === created) entries.delete(key);
          },
        );
        entries.set(key, created);
        entry = created;
      }
      const active = entry;
      if (active.lingerTimer !== null) {
        clearTimeout(active.lingerTimer);
        active.lingerTimer = null;
      }
      active.refs += 1;
      try {
        const value = await active.promise;
        let released = false;
        return {
          value,
          release: () => {
            if (released) return;
            released = true;
            active.refs -= 1;
            if (active.refs > 0) return;
            if (lingerMs <= 0) {
              disposeEntry(key, active);
              return;
            }
            active.lingerTimer = setTimeout(() => disposeEntry(key, active), lingerMs);
          },
        };
      } catch (error) {
        active.refs -= 1;
        throw error;
      }
    },
    evictNow(key?: string): void {
      for (const [entryKey, entry] of key ? ([[key, entries.get(key)]] as const) : entries) {
        if (!entry) continue;
        if (entry.refs > 0) {
          console.warn(`[DEBUG] ${label} evictNow(${entryKey}) skipped — ${entry.refs} active lease(s)`);
          continue;
        }
        if (entry.lingerTimer !== null) clearTimeout(entry.lingerTimer);
        disposeEntry(entryKey, entry);
      }
    },
    stats(): readonly LeasePoolStats[] {
      return Array.from(entries.entries()).map(([key, entry]) => ({
        key,
        refs: entry.refs,
        state: entry.settled === undefined ? "loading" : entry.lingerTimer !== null ? "lingering" : "resident",
      }));
    },
  };
}
//#endregion 🪶️LeasePool

//#region 🐚️PluginModuleLease
export interface PluginModuleLease {
  readonly handle: PluginWasmHandle;
  /** Releases this caller's reference to the shared module — idempotent, a second call is a no-op.
   * The underlying worker/module disposes once every lease on this `moduleUrl` has released and the
   * pool's linger window (see {@link createLeasePool}) elapses with no re-acquire. */
  release(): void;
}

// 🐚️ The pool's `load` callback only receives the key (`moduleUrl` — already globally unique per
// plugin, matching the pre-pool cache's key exactly), but `loadPluginModuleUncached` also wants a
// human-readable `pluginId` for its worker/log labels. `acquirePluginModule` records that association
// here just before acquiring; safe as a plain overwrite since a given `moduleUrl` only ever maps to
// one `pluginId` in practice.
const pluginModuleIdByUrl = new Map<string, string>();
const pluginModulePool = createLeasePool<PluginWasmHandle>((moduleUrl) => loadPluginModuleUncached(pluginModuleIdByUrl.get(moduleUrl) ?? moduleUrl, moduleUrl), (handle) => handle.dispose(), { label: "plugin module" });

/**
 * @emoji 🐚️ Refcounted replacement for the old `loadPluginModule` — several shells (or several plugin
 * instances within one shell) loading the SAME `moduleUrl` share one worker/module, but each caller
 * gets its own {@link PluginModuleLease} and must `release()` it on unmount/teardown. Built on
 * {@link createLeasePool}: the shared module lingers briefly after the last lease releases (a shell
 * closed and immediately reopened reuses it) rather than disposing that instant — under the pre-pool
 * cache, a loaded module was in practice *never* disposed at all (its promise was cached forever with
 * nothing to evict it; `dispose()` was only ever reachable on load *failure*), so this is strictly a
 * bugfix on top of a lifecycle improvement.
 */
export async function acquirePluginModule(pluginId: string, moduleUrl: string): Promise<PluginModuleLease> {
  pluginModuleIdByUrl.set(moduleUrl, pluginId);
  const lease = await pluginModulePool.acquire(moduleUrl);
  return { handle: lease.value, release: lease.release };
}

/** @emoji 🔁️ Forces immediate disposal of a stale `moduleUrl` after a hot reload has released its last
 * lease — a no-op with a `[DEBUG]` warning (see {@link createLeasePool.evictNow}) if a caller still
 * holds the old lease, so a reload sequence must release before evicting. Skipping this after a
 * cache-busted reload would leave the old worker lingering for the pool's full 30s window per swap. */
export function evictPluginModule(moduleUrl: string): void {
  pluginModulePool.evictNow(moduleUrl);
}

/** @emoji 🔭️ Debug-only runtime snapshot — live plugin worker ids and the plugin module pool's lease
 * states — for verifying eager-boot-vs-lazy-residency changes from devtools without instrumenting call
 * sites by hand. Intentionally global rather than exported: this is a console/devtools aid, not API. */
(globalThis as unknown as { __semioPluginRuntimeStats?: () => unknown }).__semioPluginRuntimeStats = () => ({
  workerModuleUrls: Array.from(pluginWorkerClients.keys()),
  workerCount: pluginWorkerClients.size,
  activePluginIds: Array.from(activeWorkerByPluginId.keys()),
  modulePool: pluginModulePool.stats(),
});
//#endregion 🐚️PluginModuleLease

/**
 * 🌉️ Direct main-thread import fallback for {@link loadPluginModuleViaWorker} (no `Worker` global —
 * vitest/node — or no `🟨️plugin-worker.js` alongside this module). Only the component-model
 * `createPluginApi` ABI is supported: the pre-ABI-flip flat `semio_plugin_*` wasm-bindgen export
 * surface (one JS function per verb: `semio_plugin_handle_action`, `semio_plugin_render`, ...)
 * predates the binary `exchange` ABI entirely and has no equivalent under it, so it is dropped
 * rather than adapted — this is a greenfield codebase with no legacy-ABI support obligation.
 */
async function loadPluginModuleUncached(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
  // 🧵️ Worker-backed by default so a plugin's `exchange` (e.g. puzzle-3d's collision precompute) can
  // never block the UI thread. Falls back to the direct main-thread import below when unavailable: no
  // `Worker` global (vitest/node) or no `🟨️plugin-worker.js` alongside this module.
  if (typeof Worker !== "undefined") {
    try {
      return await loadPluginModuleViaWorker(pluginId, moduleUrl);
    } catch (error) {
      console.warn(`[DEBUG] program ${pluginId} worker-backed load failed, falling back to main thread: ${error instanceof Error ? error.message : String(error)}`);
    }
  }
  const module = (await import(/* @vite-ignore */ moduleUrl)) as {
    default?: () => Promise<void> | void;
    createPluginApi?: () => Promise<{
      manifest: () => Promise<Uint8Array>;
      createApp: (appId: string) => Promise<number>;
      destroyApp?: (instanceId: number) => Promise<void>;
      exchange: (instanceId: number, frames: Uint8Array[]) => Promise<Uint8Array[]>;
    }>;
  };
  if (module.default) await module.default();
  if (!module.createPluginApi) {
    throw new Error(`[DEBUG] program ${pluginId} missing createPluginApi export`);
  }
  const api = await module.createPluginApi();
  return withSerializedPluginWasmHandle({
    manifest: () => api.manifest(),
    createApp: (appId) => api.createApp(appId),
    destroyApp: async (instanceId) => {
      await api.destroyApp?.(instanceId);
    },
    exchange: (instanceId, frames) => api.exchange(instanceId, frames),
    dispose() {},
  });
}

/** 🌉️ Adapts a {@link PluginWasmHandle} to a plain-object shape safe to close over across a
 * `postMessage`/global-bridge boundary (see the wgpu renderer's own program-worker embedding) — a
 * pass-through now that the whole ABI is already binary (`manifest`/`exchange` bytes cross
 * structured clone natively, same as `Uint8Array` payloads elsewhere on this bridge). */
export function pluginHandleForBridge(handle: PluginWasmHandle) {
  return {
    manifest: () => handle.manifest(),
    createApp: (appId: string) => handle.createApp(appId),
    destroyApp: (instanceId: number) => handle.destroyApp(instanceId),
    exchange: (instanceId: number, frames: Uint8Array[]) => handle.exchange(instanceId, frames),
  };
}
//#endregion PluginRuntime

//#region 🔌️PluginSource
/** @emoji 🔌️ Dev-server SSE endpoint a `PluginSource` availability stream connects to (see
 * {@link createDevPluginSource}) — mounted by the dev runner's `semioPluginHotSwapVitePlugin`
 * alongside the `/plugin-modules` static alias it watches. Shared here (rather than duplicated as a
 * literal in both the dev vite plugin and the shell) so the two ends can't drift apart. */
export const PLUGIN_SOURCE_WATCH_PATH = "/plugin-modules/watch";

/** @emoji 🔌️ One entry of an availability stream: either the full set of currently-built plugins sent
 * once on connect (a reconnecting/late-connecting browser must not miss builds that already finished),
 * or a single plugin's rebuild landing. `rebuiltAt` is the artifact's build timestamp and doubles as
 * the cache-busting query value {@link PluginSource.moduleUrl} mints. */
export type PluginSourceEvent = { readonly kind: "snapshot"; readonly plugins: readonly { readonly pluginId: string; readonly rebuiltAt: number }[] } | { readonly kind: "built"; readonly pluginId: string; readonly rebuiltAt: number };

/**
 * @emoji 🔌️ Where the shell's incremental plugin runtime (install/uninstall/reload — see the react
 * renderer's plugin panel) gets its catalog and availability notifications from. `createDevPluginSource`
 * is the only implementation today; a future `HubPluginSource` (fetching manifests and artifacts from
 * the plugin hub over HTTP/SSE instead of the local dev server) implements the same three methods and
 * needs no changes anywhere else — the shell only ever depends on this interface.
 */
export interface PluginSource {
  readonly id: string;
  /** Every plugin this source can currently install (built or not — the panel shows "available"
   * entries that haven't finished their first build yet). */
  list(): Promise<readonly PluginRegistryEntry[]>;
  /** Mints a concrete, cache-busted module URL for one install/reload of `pluginId`. Omitting
   * `rebuiltAt` (initial install, before any `built` event) falls back to the registry's own
   * `moduleUrl`, unbusted — correct for a first load, where there is nothing stale to bust. */
  moduleUrl(pluginId: string, rebuiltAt?: number): string;
  /** Subscribes to availability events; returns an unsubscribe function. Fires an immediate `snapshot`
   * on subscribe against sources that support it (the dev source's SSE endpoint always sends one). */
  subscribe(listener: (event: PluginSourceEvent) => void): () => void;
}

/** @emoji 🔌️ `PluginSource` backed by the dev server's static `/plugin-modules` output and its
 * {@link PLUGIN_SOURCE_WATCH_PATH} SSE stream. `EventSource` is unavailable under vitest/node, so
 * `subscribe` there is a harmless no-op (matches every other browser-only feature detection in this
 * module, e.g. {@link loadPluginModuleUncached}'s `Worker` check). */
export function createDevPluginSource(registry: readonly PluginRegistryEntry[]): PluginSource {
  const byId = new Map(registry.map((entry) => [entry.pluginId, entry] as const));
  return {
    id: "dev",
    async list() {
      return registry;
    },
    moduleUrl(pluginId, rebuiltAt) {
      const entry = byId.get(pluginId);
      if (!entry) throw new Error(`[DEBUG] plugin source "dev" has no registry entry for ${pluginId}`);
      return rebuiltAt === undefined ? entry.moduleUrl : `${entry.moduleUrl}?v=${rebuiltAt}`;
    },
    subscribe(listener) {
      if (typeof EventSource === "undefined") return () => {};
      const source = new EventSource(PLUGIN_SOURCE_WATCH_PATH);
      source.onmessage = (event) => {
        try {
          listener(JSON.parse(event.data) as PluginSourceEvent);
        } catch (error) {
          console.warn(`[DEBUG] plugin source "dev" malformed event: ${error instanceof Error ? error.message : String(error)}`);
        }
      };
      return () => source.close();
    },
  };
}
//#endregion 🔌️PluginSource

// #region 🎮️PlaygroundResolution
/** @emoji 🎮️ Finds the generated playground catalog row for a variant id or one of its aliases. */
function findPlaygroundVariant(playgroundPluginId: string): PlaygroundBuildTarget | undefined {
  return PLAYGROUND_BUILD_TARGETS.find((entry) => entry.variant === playgroundPluginId || entry.aliases.includes(playgroundPluginId));
}

/** @emoji 🎯️ Resolves a playground filter/alias (e.g. "3d", "sourcing") to its underlying wasm component registry id. */
export function resolvePluginRegistryId(playgroundPluginId: string): string {
  return findPlaygroundVariant(playgroundPluginId)?.pluginId ?? playgroundPluginId;
}

/** @emoji 🎯️ Resolves a playground filter/alias to the app id that should be instantiated by default within its plugin's manifest. */
export function resolvePlaygroundDefaultAppId(playgroundPluginId: string): string | undefined {
  return findPlaygroundVariant(playgroundPluginId)?.app;
}

export type PlaygroundBootSession = {
  readonly variant: string;
  readonly defaultAppId?: string;
  readonly plugins: readonly PluginRegistryEntry[];
};

export type PlaygroundBoot = {
  readonly variant: string;
  readonly defaultAppId?: string;
  readonly plugins: readonly PluginRegistryEntry[];
};

/** @emoji 🎮️ Resolves the wasm plugin list and default app for one playground variant; when the on-disk
 * `generated/🟦️session.ts` was overwritten by another concurrent dev variant, rebuilds from the generated
 * program catalog instead of trusting the stale program rows. */
export function resolvePlaygroundBoot(variant: string, session?: PlaygroundBootSession): PlaygroundBoot {
  const defaultAppId = resolvePlaygroundDefaultAppId(variant);
  if (session?.variant === variant) {
    return { variant, defaultAppId: session.defaultAppId ?? defaultAppId, plugins: session.plugins };
  }
  const registryPluginId = resolvePluginRegistryId(variant);
  const studioMode = resolvePluginHostConfig(variant) !== undefined;
  const catalogPlugins: PluginRegistryEntry[] = PLUGIN_BUILD_TARGETS.map((target) => ({
    pluginId: target.pluginId,
    moduleUrl: pluginModuleUrl(target.pluginId, target.wasmOut),
    contributes: target.contributes,
    consumes: target.consumes,
  }));
  return {
    variant,
    defaultAppId,
    plugins: expandPluginRegistry(catalogPlugins, studioMode ? undefined : registryPluginId, studioMode),
  };
}

//#region 🏠️🧳️PluginHostConfig
/** 🏠️🧳️ Declares, for a plugin whose manifest offers a host-style multi-app experience (one app is the
 * landing/default view, another hosts other apps as spawned sub-instances — e.g. "s"'s home/studio
 * pair), which app ids play which role. Callers resolve controller ids and default panel tabs from
 * the *loaded manifest*'s own `controllerId`/`panelTabs` on those apps rather than hardcoding separate
 * literals — this table only ever needs to carry app-id role assignments. A pluginFilter absent here
 * simply boots through the ordinary single-app path (`resolvePlaygroundDefaultAppId`). Mirrored by
 * `PLUGIN_HOST_CONFIGS`/`resolve_plugin_host_config` in `framework/os/renderer/wgpu/rs/lib.rs`'s
 * `program_bridge` module for the WGPU renderer. */
export type PluginHostConfig = {
  readonly pluginId: string;
  readonly landingAppId: string;
  readonly hostAppId: string;
};

/** 🎯️ Resolves a playground filter/alias to its plugin's host config, or `undefined` when that program doesn't offer a host-style multi-app experience. */
export function resolvePluginHostConfig(playgroundPluginId: string): PluginHostConfig | undefined {
  const registryId = resolvePluginRegistryId(playgroundPluginId);
  return PLUGIN_HOST_CONFIGS.find((entry) => entry.pluginId === registryId);
}
//#endregion 🏠️🧳️PluginHostConfig
// #endregion 🎮️PlaygroundResolution

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("DockLayoutStore", () => {
    const emptySkeleton = (): DockSkeleton => ({
      version: 3,
      anchors: { "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
    });

    it("returns null when nothing persisted", () => {
      const store = new DockLayoutStore(createMemoryStoragePort());
      expect(store.getSnapshot()).toBeNull();
    });

    it("app layer wins over os layer when both are set", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      const osSkeleton = emptySkeleton();
      const appSkeleton: DockSkeleton = { ...emptySkeleton(), anchors: { ...emptySkeleton().anchors, "top-left": [{ id: "a" }] } };
      store.saveOs(osSkeleton);
      store.save(appSkeleton);
      expect(store.getSnapshot()).toEqual(appSkeleton);
    });

    it("falls back to os layer when app layer absent", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      const osSkeleton = emptySkeleton();
      store.saveOs(osSkeleton);
      expect(store.getSnapshot()).toEqual(osSkeleton);
    });

    it("save(null) removes the app-layer key", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      store.save(emptySkeleton());
      expect(storage.get("semio.os.dock.my-app")).not.toBeNull();
      store.save(null);
      expect(storage.get("semio.os.dock.my-app")).toBeNull();
      expect(store.getSnapshot()).toBeNull();
    });

    it("reset() clears both layers", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      store.saveOs(emptySkeleton());
      store.save(emptySkeleton());
      store.reset();
      expect(storage.get("semio.os.dock")).toBeNull();
      expect(storage.get("semio.os.dock.my-app")).toBeNull();
      expect(store.getSnapshot()).toBeNull();
    });

    it("returns null on corrupt JSON rather than throwing", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.dock", "{not json");
      const store = new DockLayoutStore(storage);
      expect(() => store.getSnapshot()).not.toThrow();
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a stale version-1 (corners) blob instead of migrating it", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.dock", JSON.stringify({ version: 1, corners: { "top-left": [{ id: "a" }], "top-right": [], "bottom-left": [], "bottom-right": [] } }));
      const store = new DockLayoutStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a stale version-2 (six-anchor) blob instead of migrating it to eight anchors", () => {
      const storage = createMemoryStoragePort();
      storage.set(
        "semio.os.dock",
        JSON.stringify({ version: 2, anchors: { "top-left": [{ id: "a" }], "top-middle": [], "top-right": [], "bottom-left": [], "bottom-middle": [], "bottom-right": [] } }),
      );
      const store = new DockLayoutStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });
  });

  describe("DockUiStateStore", () => {
    const emptyUiState = (): DockUiState => ({ version: 3, anchors: {} });

    it("returns null when nothing persisted", () => {
      const store = new DockUiStateStore(createMemoryStoragePort());
      expect(store.getSnapshot()).toBeNull();
    });

    it("app layer wins over os layer when both are set", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      const osState = emptyUiState();
      const appState: DockUiState = { ...emptyUiState(), anchors: { "top-left": { visible: true, size: 320 } } };
      store.saveOs(osState);
      store.save(appState);
      expect(store.getSnapshot()).toEqual(appState);
    });

    it("falls back to os layer when app layer absent", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      const osState: DockUiState = { ...emptyUiState(), pathMemory: { "framework.category.workbench": "framework.panel.document" } };
      store.saveOs(osState);
      expect(store.getSnapshot()).toEqual(osState);
    });

    it("save(null) removes the app-layer key", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      store.save(emptyUiState());
      expect(storage.get("semio.os.dockUi.my-app")).not.toBeNull();
      store.save(null);
      expect(storage.get("semio.os.dockUi.my-app")).toBeNull();
      expect(store.getSnapshot()).toBeNull();
    });

    it("reset() clears both layers", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      store.saveOs(emptyUiState());
      store.save(emptyUiState());
      store.reset();
      expect(storage.get("semio.os.dockUi")).toBeNull();
      expect(storage.get("semio.os.dockUi.my-app")).toBeNull();
      expect(store.getSnapshot()).toBeNull();
    });

    it("returns null on corrupt JSON rather than throwing", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.dockUi", "{not json");
      const store = new DockUiStateStore(storage);
      expect(() => store.getSnapshot()).not.toThrow();
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a stale version-1 (corners) blob instead of migrating it", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.dockUi", JSON.stringify({ version: 1, corners: { "top-left": { visible: true, size: 320 } } }));
      const store = new DockUiStateStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a stale version-2 (six-anchor) blob instead of migrating it to eight anchors", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.dockUi", JSON.stringify({ version: 2, anchors: { "top-left": { visible: true, size: 320 } } }));
      const store = new DockUiStateStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });

    it('uses a distinct key from DockLayoutStore for an app literally named "ui"', () => {
      const storage = createMemoryStoragePort();
      new DockLayoutStore(storage, "ui").save({
        version: 3,
        anchors: { "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
      });
      new DockUiStateStore(storage).saveOs(emptyUiState());
      expect(storage.get("semio.os.dock.ui")).not.toBeNull();
      expect(storage.get("semio.os.dockUi")).not.toBeNull();
      expect(storage.get("semio.os.dock.ui")).not.toEqual(storage.get("semio.os.dockUi"));
    });
  });

  describe("WindowPaneStateStore", () => {
    const emptyPaneState = (): WindowPaneUiState => ({ version: 1, windows: {} });

    it("returns null when nothing persisted", () => {
      const store = new WindowPaneStateStore(createMemoryStoragePort());
      expect(store.getSnapshot()).toBeNull();
    });

    it("app layer wins over os layer when both are set", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      const osState = emptyPaneState();
      const appState: WindowPaneUiState = { version: 1, windows: { "puzzle3d.play": { utilities: { anchor: "bottom-left", folded: false, size: 280 } } } };
      store.saveOs(osState);
      store.save(appState);
      expect(store.getSnapshot()).toEqual(appState);
    });

    it("falls back to os layer when app layer absent", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      const osState: WindowPaneUiState = { version: 1, windows: { "puzzle3d.play": { measures: { anchor: "top-right", size: 320 } } } };
      store.saveOs(osState);
      expect(store.getSnapshot()).toEqual(osState);
    });

    it("save(null) removes the app-layer key", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      store.save(emptyPaneState());
      expect(storage.get("semio.os.paneUi.my-app")).not.toBeNull();
      store.save(null);
      expect(storage.get("semio.os.paneUi.my-app")).toBeNull();
      expect(store.getSnapshot()).toBeNull();
    });

    it("reset() clears both layers", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      store.saveOs(emptyPaneState());
      store.save(emptyPaneState());
      store.reset();
      expect(storage.get("semio.os.paneUi")).toBeNull();
      expect(storage.get("semio.os.paneUi.my-app")).toBeNull();
      expect(store.getSnapshot()).toBeNull();
    });

    it("returns null on corrupt JSON rather than throwing", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.paneUi", "{not json");
      const store = new WindowPaneStateStore(storage);
      expect(() => store.getSnapshot()).not.toThrow();
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a foreign-version blob instead of migrating it", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.paneUi", JSON.stringify({ version: 2, windows: {} }));
      const store = new WindowPaneStateStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });
  });

  describe("PlaygroundResolution", () => {
    it("resolves host config from generated program metadata", () => {
      expect(resolvePluginHostConfig("s")).toEqual({ pluginId: "s", landingAppId: "home", hostAppId: "studio" });
      expect(resolvePluginHostConfig("puzzle3d")).toBeUndefined();
    });

    it("resolves playground aliases to registry plugin ids", () => {
      expect(resolvePluginRegistryId("aggregator")).toBe("puzzle");
      expect(resolvePluginRegistryId("3d")).toBe("puzzle");
    });

    it("rebuilds program rows when the generated session variant is stale", () => {
      const boot = resolvePlaygroundBoot("aggregator", {
        variant: "sourcing",
        defaultAppId: "sourcing-curate",
        plugins: [{ pluginId: "sourcing", moduleUrl: "/plugin-modules/sourcing/sourcing_plugin.js" }],
      });
      expect(boot.variant).toBe("aggregator");
      expect(boot.defaultAppId).toBe("puzzle3d-play");
      expect(boot.plugins).toEqual([{ pluginId: "puzzle", moduleUrl: "/plugin-modules/puzzle/🟨️puzzle_plugin.js", contributes: [], consumes: [] }]);
    });
  });

  describe("organizeContextMenu", () => {
    const menuLeaf = (id: string): ContextMenuItemSpec => ({ id, label: id, action: id });
    const menuDestructive = (id: string): ContextMenuItemSpec => ({ ...menuLeaf(id), destructive: true });

    it("keeps a flat within-budget menu as-is, with groups sorted after leaves", () => {
      const items = [menuLeaf("a"), menuLeaf("b"), { id: "menu.group.view", children: [menuLeaf("c")] }];
      expect(organizeContextMenu(items, () => undefined)).toEqual(items);
    });

    it("shares the Rust fixture's grouped structure for a flat 12-item over-budget menu", () => {
      // 🗂️ Mirrors `organize_context_menu_buckets_overflow_leaves_by_category_of` (5 primaries + N
      // categorized overflow leaves) combined with `organize_context_menu_puts_destructive_leaves_last_after_a_separator`
      // (a trailing destructive leaf) — same shape the Rust test suite asserts for an equivalent input.
      const items: ContextMenuItemSpec[] = [
        menuLeaf("primary0"),
        menuLeaf("primary1"),
        menuLeaf("primary2"),
        menuLeaf("primary3"),
        menuLeaf("primary4"),
        menuLeaf("overflow0"),
        menuLeaf("overflow1"),
        menuLeaf("overflow2"),
        menuLeaf("overflow3"),
        menuLeaf("overflow4"),
        menuLeaf("overflow5"),
        menuDestructive("delete"),
      ];
      const categoryOf = (id: string): string | undefined => (id.startsWith("overflow") ? "view" : undefined);
      const organized = organizeContextMenu(items, categoryOf);

      expect(organized.map((item) => item.id)).toEqual([
        "primary0",
        "primary1",
        "primary2",
        "primary3",
        "primary4",
        "menu.group.view",
        "separator-organized-6",
        "delete",
      ]);
      expect(organized[5]!.children?.map((child) => child.id)).toEqual([
        "overflow0",
        "overflow1",
        "overflow2",
        "overflow3",
        "overflow4",
        "overflow5",
      ]);
      expect(organized[6]!.separator).toBe(true);
      expect(organized[6]!.label).toBeUndefined();
      expect(organized[7]!.destructive).toBe(true);
    });
  });

  describe("pluginWorkerUrl (hot-reload cache-busting regression)", () => {
    it("swaps the plugin's own bridge filename for the generic worker bootstrap script", () => {
      expect(pluginWorkerUrl("/plugin-modules/note/note_plugin.js")).toBe("/plugin-modules/note/🟨️plugin-worker.js");
    });

    it("strips a cache-busting ?v= query before swapping the filename — a bare .js-suffix regex silently no-ops on a query string", () => {
      expect(pluginWorkerUrl("/plugin-modules/note/note_plugin.js?v=1785506741609")).toBe("/plugin-modules/note/🟨️plugin-worker.js");
    });

    it("also strips a trailing hash fragment", () => {
      expect(pluginWorkerUrl("/plugin-modules/note/note_plugin.js#fragment")).toBe("/plugin-modules/note/🟨️plugin-worker.js");
    });
  });

  describe("PluginSource", () => {
    const registry: readonly PluginRegistryEntry[] = [
      { pluginId: "note", moduleUrl: "/plugin-modules/note/note_plugin.js" },
      { pluginId: "s", moduleUrl: "/plugin-modules/s/s_plugin.js" },
    ];

    it("list() returns the registry it was created with", async () => {
      const source = createDevPluginSource(registry);
      expect(source.id).toBe("dev");
      await expect(source.list()).resolves.toEqual(registry);
    });

    it("moduleUrl() passes through unbusted without rebuiltAt", () => {
      const source = createDevPluginSource(registry);
      expect(source.moduleUrl("note")).toBe("/plugin-modules/note/note_plugin.js");
    });

    it("moduleUrl() cache-busts with a rebuiltAt query param", () => {
      const source = createDevPluginSource(registry);
      expect(source.moduleUrl("note", 1785789943669)).toBe("/plugin-modules/note/note_plugin.js?v=1785789943669");
    });

    it("moduleUrl() throws for an unknown pluginId", () => {
      const source = createDevPluginSource(registry);
      expect(() => source.moduleUrl("missing")).toThrow(/missing/);
    });

    it("subscribe() is a harmless no-op without a global EventSource (node/vitest)", () => {
      const source = createDevPluginSource(registry);
      const events: PluginSourceEvent[] = [];
      const unsubscribe = source.subscribe((event) => events.push(event));
      expect(() => unsubscribe()).not.toThrow();
      expect(events).toEqual([]);
    });
  });

  describe("LeasePool evictNow (hot-swap reload eviction)", () => {
    it("disposes a fully-released key immediately", async () => {
      const disposed: string[] = [];
      const pool = createLeasePool<string>(
        (key) => Promise.resolve(`value:${key}`),
        (value) => disposed.push(value),
        { lingerMs: 30_000 },
      );
      const lease = await pool.acquire("url-v1");
      lease.release();
      expect(disposed).toEqual([]);
      pool.evictNow("url-v1");
      expect(disposed).toEqual(["value:url-v1"]);
    });

    it("skips (does not throw) a key with an active lease, matching a reload that hasn't released the old handle yet", async () => {
      const disposed: string[] = [];
      const pool = createLeasePool<string>(
        (key) => Promise.resolve(`value:${key}`),
        (value) => disposed.push(value),
      );
      const lease = await pool.acquire("url-v1");
      expect(() => pool.evictNow("url-v1")).not.toThrow();
      expect(disposed).toEqual([]);
      lease.release();
      pool.evictNow("url-v1");
      expect(disposed).toEqual(["value:url-v1"]);
    });

    it("treats two cache-busted URLs of the same pluginId as independent keys", async () => {
      const disposed: string[] = [];
      const pool = createLeasePool<string>(
        (key) => Promise.resolve(`value:${key}`),
        (value) => disposed.push(value),
      );
      const oldLease = await pool.acquire("note.js?v=1");
      const newLease = await pool.acquire("note.js?v=2");
      oldLease.release();
      pool.evictNow("note.js?v=1");
      expect(disposed).toEqual(["value:note.js?v=1"]);
      newLease.release();
      pool.evictNow("note.js?v=2");
      expect(disposed).toEqual(["value:note.js?v=1", "value:note.js?v=2"]);
    });
  });
}
//#endregion 🧪️Tests
