// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/UtilityTree/component.tsx
/** @emoji 🌳️ `UtilityTree` — the ribbon/tree renderer for an app's `utilities` taxonomy
 * (`groupUtilityNodesByCategory` groups+sorts `UtilityNode`s by `UtilityCategory`, `UtilityTree`
 * renders the grouped result as a `Ribbon`). Used by the framework chrome's utility panel and by
 * `ShellSync`'s sync-scoped variant.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useEffect, useMemo, useState, type ReactElement, type ReactNode } from "react";
import {
  ButtonGroup,
  ButtonGroupItem,
  Icon,
  type IconName,
  Ribbon,
  type RibbonDirection,
  RibbonDivider,
  RibbonGroup,
  RibbonItem,
  type RibbonRow,
  RibbonZone,
  ToggleGroup,
} from "@semio-tech/ui-react";
import { type ActionDescriptor, SET_ACTIVE_UTILITY_ACTION_ID, type UtilityCategory, type UtilityLeaf, type UtilityNode } from "@semio-tech/framework-core";
import { SelectionUtilityOptions } from "../ShellHelpers/🟦️component.tsx";
// #endregion 🔌️Adapters

//#region 🔖️utility-tree

type UtilityTreeProps = {
  readonly utilities: readonly UtilityNode[];
  readonly onAction: (action: ActionDescriptor) => void;
  readonly id?: string;
  /** @emoji 🎀️ `up` stacks a new ribbon line above the base row per pressed collection (window utility bar); `inline` keeps the horizontal drill-down (footer). */
  readonly direction?: RibbonDirection;
  /** @emoji 🎓️ A utility id the introduction walkthrough is anchored on — when it names a leaf nested inside
   * a collapsed group picker, the picker auto-drills into that group so the leaf actually mounts (see
   * {@link findUtilityGroupPath}). `null`/not-found leaves `activePath` alone. */
  readonly revealUtilityId?: string | null;
  /** @emoji 🎯️ Utility-scoped measure chrome for the active utility — rendered as an extra ribbon row under the utilities. */
  readonly utilityOptions?: ReactNode;
};

function resolveLeafAction(node: UtilityLeaf | Extract<UtilityNode, { readonly kind: "button" | "toggle" }>): ActionDescriptor | null {
  if ("onPress" in node && node.onPress) return node.onPress;
  if ("onChange" in node && node.onChange) return node.onChange;
  if (node.kind === "button" || node.kind === "toggle") {
    if (!node.action || !node.controllerId) return null;
    return { controllerId: node.controllerId, action: node.action, args: node.args as Record<string, unknown> | undefined };
  }
  return null;
}

/** @emoji 🔢️ Sorts utility nodes by `order`. */
export function sortUtilityNodes(nodes: readonly UtilityNode[]): UtilityNode[] {
  return [...nodes].sort((left, right) => (left.order ?? 0) - (right.order ?? 0));
}

//#region 🗂️UtilityCategoryGrouping

const UTILITY_CATEGORY_ORDER: readonly UtilityCategory[] = ["selection", "utilities", "history", "sync"];

/** @emoji 🪟️ Categories that are scoped to whatever window/pane the user is interacting with — selecting or editing content varies per window, so these live in each window's own bottom-left panel. */
export const UTILITY_CATEGORIES: readonly UtilityCategory[] = ["selection", "utilities"];

export const UTILITY_CATEGORY_ICON_ID: Readonly<Record<UtilityCategory, IconName>> = {
  selection: "mouse-pointer",
  utilities: "wrench",
  history: "undo",
  sync: "cloud",
};

function utilityNodeCategory(node: UtilityNode): UtilityCategory {
  if (node.kind === "separator") return "utilities";
  if (node.category) return node.category;
  return "utilities";
}

/** @emoji 🗂️ Buckets top-level utility nodes into the given categories (default: all) so activating a category expands the panel with another line, matching {@link buildUtilityRibbonSegments}'s one-active-group-per-level picker. A category with a single already-meaningful collection is used as-is instead of being re-wrapped in a synthetic one, avoiding a redundant picker level with a duplicate-looking label (e.g. a lone "Selection" collection nested under a "Selection" category chip). Separators default to `utilities` (mirrors Rust `UtilityNode::category()`), so dividers between same-category runs survive; dividers that only separated different categories become redundant once those categories are separate picker lines. */
export function groupUtilityNodesByCategory(nodes: readonly UtilityNode[], categories: readonly UtilityCategory[] = UTILITY_CATEGORY_ORDER): UtilityNode[] {
  const buckets = new Map<UtilityCategory, UtilityNode[]>();
  for (const node of nodes) {
    const category = utilityNodeCategory(node);
    if (!categories.includes(category)) continue;
    const bucket = buckets.get(category) ?? [];
    bucket.push(node);
    buckets.set(category, bucket);
  }
  return categories
    .filter((category) => hasInteractiveUtilityNodes(buckets.get(category)))
    .map((category, order) => {
      const bucket = buckets.get(category)!;
      if (bucket.length === 1 && bucket[0].kind === "collection") return { ...bucket[0], order };
      return { id: category, kind: "collection" as const, iconId: UTILITY_CATEGORY_ICON_ID[category], text: category, order, category, children: bucket };
    });
}

/** @emoji 🦶️ Deduplicates utility nodes by id across every window's utility set (mode-wide utilities are attached identically to each window kind when a plugin doesn't differentiate per window), for a single shared footer entry per utility. */
export function dedupeUtilityNodesById(nodeLists: readonly (readonly UtilityNode[])[]): UtilityNode[] {
  const seen = new Map<string, UtilityNode>();
  for (const nodes of nodeLists) {
    for (const node of nodes) {
      if (!seen.has(node.id)) seen.set(node.id, node);
    }
  }
  return [...seen.values()];
}

//#endregion 🗂️UtilityCategoryGrouping

function isInteractiveUtilityNode(node: UtilityNode): boolean {
  if (node.kind === "separator") return false;
  if (node.kind === "collection") return hasInteractiveUtilityNodes(node.children);
  return true;
}

function hasInteractiveUtilityNodes(nodes?: readonly UtilityNode[]): boolean {
  return Boolean(nodes?.some((node) => isInteractiveUtilityNode(node)));
}

function hasInteractiveUtilityLeaves(items: readonly UtilityLeaf[]): boolean {
  return items.some((node) => node.kind !== "separator");
}

type UtilityCollectionNode = Extract<UtilityNode, { readonly kind: "collection" }>;

export type UtilityRibbonSegment = { readonly kind: "picker"; readonly collections: readonly UtilityCollectionNode[]; readonly depth: number } | { readonly kind: "utilities"; readonly items: readonly UtilityLeaf[]; readonly depth: number };

/** @emoji 🎀️ Builds drill-down ribbon segments from a utility tree and active collection path; `depth` marks how many collections were drilled into to reach a segment. Collections never auto-activate: a level only recurses when `path[depth]` names one of its enabled collections, so at most one group per level is active and an unresolved level simply shows its picker. */
export function buildUtilityRibbonSegments(nodes: readonly UtilityNode[], path: readonly string[], depth = 0): UtilityRibbonSegment[] {
  const sorted = sortUtilityNodes(nodes);
  const collections = sorted.filter((node): node is UtilityCollectionNode => node.kind === "collection" && !node.disabled);
  const looseLeaves = sorted.filter((node): node is UtilityLeaf => node.kind !== "collection");
  const segments: UtilityRibbonSegment[] = [];

  if (collections.length > 0) segments.push({ kind: "picker", collections, depth });
  if (hasInteractiveUtilityLeaves(looseLeaves)) segments.push({ kind: "utilities", items: looseLeaves, depth });
  if (collections.length === 0) return segments;

  const activeId = path[depth];
  const active = activeId ? collections.find((node) => node.id === activeId) : undefined;
  if (!active) return segments;
  return [...segments, ...buildUtilityRibbonSegments(active.children, path, depth + 1)];
}

/** @emoji 🎀️ Validates an active-group path against the current utility tree: keeps each entry only while it still names an enabled collection at that level, truncating at the first miss rather than substituting a default. */
export function reconcileUtilityPath(nodes: readonly UtilityNode[], path: readonly string[]): readonly string[] {
  let current = nodes;
  const reconciled: string[] = [];

  for (const collectionId of path) {
    const collections = sortUtilityNodes(current).filter((node): node is UtilityCollectionNode => node.kind === "collection" && !node.disabled);
    const active = collections.find((node) => node.id === collectionId);
    if (!active) break;
    reconciled.push(collectionId);
    current = active.children;
  }

  return reconciled;
}

/** @emoji 🎛️ Id of the pressed utility leaf in a derived utility tree, if any. */
export function findPressedUtilityLeafId(nodes: readonly UtilityNode[]): string | undefined {
  for (const node of nodes) {
    if (node.kind === "collection") {
      const nested = findPressedUtilityLeafId(node.children);
      if (nested) return nested;
    } else if (node.kind === "toggle" && node.pressed) {
      return node.id;
    }
  }
  return undefined;
}

/** @emoji 🧰️ First `setActiveUtility` descriptor in a utility tree — used to deactivate when a collection that owns the pressed leaf is collapsed. */
function findSetActiveUtilityDescriptor(nodes: readonly UtilityNode[]): ActionDescriptor | undefined {
  for (const node of nodes) {
    if (node.kind === "collection") {
      const nested = findSetActiveUtilityDescriptor(node.children);
      if (nested) return nested;
    } else if (node.kind === "toggle" && "onChange" in node && node.onChange.action === SET_ACTIVE_UTILITY_ACTION_ID) {
      return node.onChange;
    }
  }
  return undefined;
}

/** @emoji 🎓️ Finds the group-id path (in {@link reconcileUtilityPath} shape) leading down to a utility leaf,
 * so a folded picker can drill straight to it. Returns `[]` when the id is a top-level (ungrouped) node,
 * `null` when the tree has no node with that id at all. */
export function findUtilityGroupPath(nodes: readonly UtilityNode[], targetId: string, prefix: readonly string[] = []): readonly string[] | null {
  for (const node of nodes) {
    if (node.id === targetId) return prefix;
    if (node.kind === "collection") {
      const nested = findUtilityGroupPath(node.children, targetId, [...prefix, node.id]);
      if (nested) return nested;
    }
  }
  return null;
}

function UtilityRibbonItems({ items, onAction }: { readonly items: readonly UtilityLeaf[]; readonly onAction: (action: ActionDescriptor) => void }): ReactElement {
  const sorted = useMemo(() => sortUtilityNodes(items) as UtilityLeaf[], [items]);
  const nodes = useMemo(() => {
    const rendered: ReactElement[] = [];
    let buttonRun: UtilityLeaf[] = [];
    let toggleRun: UtilityLeaf[] = [];

    const flushButtons = () => {
      if (buttonRun.length === 0) return;
      const run = buttonRun;
      buttonRun = [];
      rendered.push(
        <RibbonItem key={`buttons-${run.map((entry) => entry.id).join("-")}`}>
          <ButtonGroup>
            {run.map((entry) => {
              const action = resolveLeafAction(entry);
              if (!action) return null;
              return (
                <ButtonGroupItem
                  key={entry.id}
                  id={entry.id}
                  aria-label={entry.title ?? entry.label ?? entry.id}
                  title={entry.title ?? entry.label}
                  disabled={entry.disabled}
                  onClick={() => onAction(action)}
                  icon={<Icon icon={entry.iconId as IconName} size="small" />}
                  text={entry.text ?? entry.label}
                />
              );
            })}
          </ButtonGroup>
        </RibbonItem>,
      );
    };

    const flushToggles = () => {
      if (toggleRun.length === 0) return;
      const run = toggleRun;
      toggleRun = [];
      rendered.push(
        <RibbonItem key={`toggles-${run.map((entry) => entry.id).join("-")}`}>
          <ToggleGroup
            kind="multiple"
            value={run.filter((entry) => entry.pressed).map((entry) => entry.id)}
            onValueChange={(values) => {
              for (const entry of run) {
                const action = resolveLeafAction(entry);
                if (!action) continue;
                const pressed = values.includes(entry.id);
                if ((entry.pressed ?? false) !== pressed) onAction(action);
              }
            }}
            items={run.map((entry) => ({
              value: entry.id,
              id: entry.id,
              icon: <Icon icon={entry.iconId as IconName} size="small" />,
              text: entry.text ?? entry.label,
            }))}
          />
        </RibbonItem>,
      );
    };

    const flushRuns = () => {
      flushButtons();
      flushToggles();
    };

    for (const item of sorted) {
      if (item.kind === "separator") {
        flushRuns();
        rendered.push(<RibbonDivider key={item.id} />);
        continue;
      }
      if (item.kind === "toggle") {
        flushButtons();
        toggleRun.push(item);
        continue;
      }
      flushToggles();
      buttonRun.push(item);
    }
    flushRuns();
    return rendered;
  }, [onAction, sorted]);

  return <RibbonGroup>{nodes}</RibbonGroup>;
}

function utilityRibbonSegmentKey(segment: UtilityRibbonSegment, index: number): string {
  return segment.kind === "picker" ? `picker-${segment.depth}-${segment.collections.map((entry) => entry.id).join("-")}` : `utilities-${index}-${segment.items.map((entry) => entry.id).join("-")}`;
}

export function UtilityTree({ utilities, onAction, id = "ui.utilities", direction = "inline", revealUtilityId = null, utilityOptions }: UtilityTreeProps): ReactElement | null {
  const [activePath, setActivePath] = useState<readonly string[]>([]);

  useEffect(() => {
    setActivePath((previousPath) => {
      const next = reconcileUtilityPath(utilities, previousPath);
      return previousPath.length === next.length && previousPath.every((entry, index) => entry === next[index]) ? previousPath : next;
    });
  }, [utilities]);

  useEffect(() => {
    const pressedId = revealUtilityId ?? findPressedUtilityLeafId(utilities);
    if (!pressedId) return;
    const path = findUtilityGroupPath(utilities, pressedId);
    if (!path) return;
    setActivePath((previousPath) => (previousPath.length === path.length && previousPath.every((entry, index) => entry === path[index]) ? previousPath : path));
  }, [revealUtilityId, utilities]);

  const segments = useMemo(() => buildUtilityRibbonSegments(utilities, activePath), [utilities, activePath]);

  if (!hasInteractiveUtilityNodes(utilities) && !utilityOptions) return null;

  const renderSegment = (segment: UtilityRibbonSegment): ReactNode =>
    segment.kind === "picker" ? (
      <RibbonItem>
        <ToggleGroup
          kind="single"
          value={activePath[segment.depth] ?? ""}
          onValueChange={(value) => {
            if (!value) {
              const pressedId = findPressedUtilityLeafId(utilities);
              const pressedPath = pressedId ? findUtilityGroupPath(utilities, pressedId) : null;
              if (pressedPath && pressedPath.length > segment.depth) {
                const template = findSetActiveUtilityDescriptor(utilities);
                if (template) onAction({ ...template, args: { ...(template.args as object | undefined), utilityId: "" } });
              }
              setActivePath(activePath.slice(0, segment.depth));
              return;
            }
            setActivePath(reconcileUtilityPath(utilities, [...activePath.slice(0, segment.depth), value]));
          }}
          items={segment.collections.map((entry) => ({
            value: entry.id,
            id: `${id}.group.${entry.id}`,
            icon: <Icon icon={entry.iconId as IconName} size="small" />,
            text: entry.text ?? entry.label,
          }))}
        />
      </RibbonItem>
    ) : (
      <UtilityRibbonItems items={segment.items} onAction={onAction} />
    );

  const windowId = id.startsWith("ui.utilities.") ? id.slice("ui.utilities.".length) : "";

  const findPressedSelectionUtility = (nodes: readonly UtilityNode[]): UtilityNode | undefined => {
    for (const node of nodes) {
      if (node.kind === "collection") {
        const found = findPressedSelectionUtility(node.children);
        if (found) return found;
      } else if (node.kind === "toggle" && node.pressed && node.id.startsWith("select")) {
        return node;
      }
    }
    return undefined;
  };

  const activeSelectionUtility = findPressedSelectionUtility(utilities);
  const hasActiveSelection = activeSelectionUtility != null;

  const rows: RibbonRow[] =
    direction === "inline"
      ? segments.map((segment, index) => ({ key: utilityRibbonSegmentKey(segment, index), content: renderSegment(segment) }))
      : Array.from(
          segments.reduce((byDepth, segment, index) => {
            const zones = byDepth.get(segment.depth) ?? [];
            zones.push(<RibbonZone key={utilityRibbonSegmentKey(segment, index)}>{renderSegment(segment)}</RibbonZone>);
            byDepth.set(segment.depth, zones);
            return byDepth;
          }, new Map<number, ReactElement[]>()),
        )
          .sort(([left], [right]) => left - right)
          .map(([depth, content]) => ({ key: `row-${depth}`, content }));

  if (utilityOptions && direction !== "inline") {
    rows.push({
      key: "row-utility-options",
      content: (
        <RibbonZone variableHeight className="items-start">
          <RibbonItem className="h-auto items-start">{utilityOptions}</RibbonItem>
        </RibbonZone>
      ),
    });
  } else if (hasActiveSelection && direction !== "inline") {
    rows.push({
      key: "row-selection-options",
      content: (
        <RibbonZone>
          <RibbonItem>
            <SelectionUtilityOptions activeUtilityId={activeSelectionUtility.id} windowId={windowId} onAction={onAction} />
          </RibbonItem>
        </RibbonZone>
      ),
    });
  }

  return <Ribbon id={id} direction={direction} rows={rows} />;
}
//#endregion 🔖️utility-tree
