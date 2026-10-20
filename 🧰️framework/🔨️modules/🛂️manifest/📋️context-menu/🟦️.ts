import type { MenuRef } from "../🏷️ui-contract-and-canvas-types/🟦️.ts";

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
 * (`🧰️framework/🔨️modules/🖱️ui/📜️declaration/🦀️.rs`). Plugins build these with
 * `MenuBuilder`; the host maps them through `ContextMenuController` (React) / `render_context_menu`
 * (wgpu) unchanged. */
export type ContextMenuItemSpec = {
  readonly id: string;
  readonly label?: string;
  readonly icon?: string;
  readonly color?: string;
  readonly shortcut?: string;
  readonly disabled?: boolean;
  /** 💬️ Why a disabled row cannot run: the row stays focusable (`aria-disabled`) and announces it (`aria-describedby`). */
  readonly reason?: string;
  readonly separator?: boolean;
  readonly checked?: boolean;
  readonly destructive?: boolean;
  readonly action?: string;
  readonly args?: Record<string, unknown>;
  readonly hoverAction?: string;
  readonly hoverArgs?: Record<string, unknown>;
  readonly children?: readonly ContextMenuItemSpec[];
};

/** 🗂️ Canonical ribbon-parent taxonomy — TS twin of the Rust `RIBBON_PARENT_CATEGORIES` const
 * (`🧰️framework/🔨️modules/🖱️ui/📜️declaration/🦀️.rs`) and of ui-react's closed
 * `UiRibbonParentCategory` union (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx`).
 * Id spelling and order are load-bearing: `organizeContextMenu` sorts `menu.group.<category>`
 * rows by this order (unknown categories sort after, in emit order). */
const RIBBON_PARENT_CATEGORIES = [
  "history", "hand", "selection", "lasso", "filter", "open", "save", "transfer", "transform", "create", "view", "actions", "settings",
  "methods", "mode", "targets", "export", "tools", "utilities", "sync",
] as const;

/** 🗂️ Id prefix every category group row carries. A group row travels with `label: undefined` on
 * purpose — the shell that renders the menu owns the chrome vocabulary and resolves the label from
 * the category behind this prefix (React: `contextMenuGroupLabel`; wgpu: `ribbon_parent_label`). */
export const CONTEXT_MENU_GROUP_ID_PREFIX = "menu.group.";

/** 🗂️ The one group category `organizeContextMenu` synthesizes rather than reads off the taxonomy:
 * the overflow bucket surplus groups fold into once the row budget is exceeded. It is deliberately
 * NOT a `RIBBON_PARENT_CATEGORIES` id, so it resolves through its own chrome label instead. */
export const CONTEXT_MENU_OVERFLOW_CATEGORY = "more";

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
  return item.id.startsWith(CONTEXT_MENU_GROUP_ID_PREFIX);
}

function contextMenuGroupCategory(item: ContextMenuItemSpec): string {
  return item.id.startsWith(CONTEXT_MENU_GROUP_ID_PREFIX) ? item.id.slice(CONTEXT_MENU_GROUP_ID_PREFIX.length) : item.id;
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
      const index = bucketMut(bucketedGroups, `${CONTEXT_MENU_GROUP_ID_PREFIX}${slug}`);
      bucketedGroups[index] = { ...bucketedGroups[index]!, children: [...(bucketedGroups[index]!.children ?? []), item] };
      continue;
    }
    if (primaries.length < CONTEXT_MENU_PRIMARY_BUDGET) {
      primaries.push(item);
      continue;
    }
    const category = categoryOf(item.action ?? item.id) ?? "actions";
    const index = bucketMut(bucketedGroups, `${CONTEXT_MENU_GROUP_ID_PREFIX}${category}`);
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
    out.push({ id: `${CONTEXT_MENU_GROUP_ID_PREFIX}${CONTEXT_MENU_OVERFLOW_CATEGORY}`, label: undefined, children: foldedChildren });
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

/** 🖱️ Scene-target info for an on-demand context-menu request — hit-test results from the
 * surface's own picking (hover/selection), not cached across clicks. */
export type UiContextMenuHit = {
  readonly domain: string;
  readonly id: string;
  readonly label?: string;
};

/** 🗂️ The ids of one domain that are selected when a context menu is requested. */
export type UiContextMenuSelectionGroup = {
  readonly domain: string;
  readonly ids: readonly string[];
};

/** 📝️ Text-editor facts a context menu depends on. */
export type UiContextMenuTextContext = {
  readonly caret: number;
  readonly hasSelection: boolean;
  readonly word?: string;
  readonly canRename: boolean;
  readonly hasCompletions: boolean;
};

/** 🎯️ The surface a context-menu request was raised on. */
export type UiContextMenuSurfaceTarget = {
  readonly surfaceId: string;
  readonly kind: string;
  readonly hits?: readonly UiContextMenuHit[];
  readonly selection?: readonly UiContextMenuSelectionGroup[];
  readonly text?: UiContextMenuTextContext;
};

/** 📍️ Screen point a context-menu request was raised at. */
export type UiContextMenuPoint = { readonly x: number; readonly y: number };

/** 🖱️ On-demand context-menu request — never cached, never batched into a refresh request.
 * `menu` is the {@link MenuRef} the host resolved from `data-menu-id`/a scene surface convention id
 * (`"world3d"`, `"nodeGraph"`, `"window"`, `"panel:<tabId>"`, ...). */
export type UiContextMenuRequest = {
  readonly menu: MenuRef;
  readonly surface?: UiContextMenuSurfaceTarget;
  readonly windowInstanceId?: string;
  readonly point?: UiContextMenuPoint;
};
