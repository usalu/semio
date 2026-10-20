import { type InteractionDefinition, validateState, type DomainTopology, type DomainSelection, nextSelection } from "@semio-tech/framework-interaction";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { type TreeDataItem, TreeItemCollapsibleState, type TreeSelectionMode } from "../📋️data/🟦️.ts";

interface TreeSelectionComputationArgs {
  selectionMode: TreeSelectionMode;
  selectedIds: readonly string[];
  orderedIds: readonly string[];
  targetId: string;
  anchorId?: string;
  additiveKey: boolean;
  rangeKey: boolean;
}

interface TreeSelectionComputationResult {
  selectedIds: string[];
  anchorId?: string;
}

// #region 🕹️InteractionDelegation
/** 🕹️ Tree has no real granularity/domain concept of its own — every row is addressed under this one
 * fixed granularity id when adapting to `🕹️interaction`'s domain-keyed machine below. */
const TREE_INTERACTION_GRANULARITY = "item";
const TREE_INTERACTION_DOMAIN = "tree";
const TREE_INTERACTION_LABEL = { native: { en: "Tree", de: "Baum" }, reuse: { en: "Tree", de: "Baum" } } as const;
const TREE_INTERACTION_ITEM_LABEL = { native: { en: "Item", de: "Element" }, reuse: { en: "Item", de: "Element" } } as const;

/** 🕹️ Minimal `InteractionDefinition` wrapping `selectionMode` — only `granularities[0].id` and
 * `selection.modes[0]` are read by `nextSelection`/`validateState`; `hover`/`hierarchy`/`iconId` are
 * unused filler required by the type. */
const treeInteractionDefinition = (selectionMode: TreeSelectionMode): InteractionDefinition => ({
  id: TREE_INTERACTION_DOMAIN,
  label: TREE_INTERACTION_LABEL,
  granularities: [{ id: TREE_INTERACTION_GRANULARITY, label: TREE_INTERACTION_ITEM_LABEL, iconId: "circle" }],
  hierarchy: { kind: "flat" },
  hover: { enabled: true, transitive: false, channels: ["pointer"], broadcast: false },
  selection: { modes: [selectionMode], methods: ["pick"], merges: ["replace", "additive", "subtractive", "invertive", "range"], transitive: false, broadcast: false },
});
// #endregion 🕹️InteractionDelegation

/** 🕹️ Delegates to `🕹️interaction`'s `validateState` (wave 0) for the FIRST-id clamp Tree has always
 * applied to externally-supplied/persisted ids (`defaultSelectedIds`, controlled `selectedIds`/
 * `highlightedIds`) — distinct from {@link getTreeNextSelectionState}'s LAST-target clamp for a live
 * pick. Falsy-id filtering + dedup stay local sanitization ahead of the shared machine, which has no
 * notion of either. */
export const normalizeTreeSelectedIds = (selectedIds: readonly string[], selectionMode: TreeSelectionMode): string[] => {
  const sanitizedIds = Array.from(new Set(selectedIds.filter(Boolean)));
  const validated = validateState(
    [treeInteractionDefinition(selectionMode)],
    { domains: {} },
    {
      selection: { [TREE_INTERACTION_DOMAIN]: { granularity: TREE_INTERACTION_GRANULARITY, ids: sanitizedIds } },
      hover: {},
      activeMode: {},
      activeGranularity: {},
    },
  );
  return [...(validated.selection[TREE_INTERACTION_DOMAIN]?.ids ?? [])];
};

export const getTreeItemDefaultOpen = (item: TreeDataItem): boolean => item.defaultOpen ?? item.collapsibleState === TreeItemCollapsibleState.Expanded;

/** 🕹️ Delegates to `🕹️interaction`'s `nextSelection` (wave 0) for a live pick's LAST-target clamp — see
 * {@link normalizeTreeSelectedIds}'s doc for the two clamps this wave deliberately keeps distinct.
 * `additiveKey`/`rangeKey` fold to one `MergeMode` the same way `interactionMergeFromModifiers` folds raw
 * pointer modifiers (shift wins over ctrl/meta) — this function's own boolean-pair signature is kept
 * unchanged since `⚙️VirtualFileSystem`/tests already call it this shape. */
export const getTreeNextSelectionState = ({ selectionMode, selectedIds, orderedIds, targetId, anchorId, additiveKey, rangeKey }: TreeSelectionComputationArgs): TreeSelectionComputationResult => {
  const merge = rangeKey ? "range" : additiveKey ? "invertive" : "replace";
  const topology: DomainTopology = { ordered: orderedIds.map((id) => ({ id, granularity: TREE_INTERACTION_GRANULARITY })) };
  const current: DomainSelection = { granularity: TREE_INTERACTION_GRANULARITY, ids: [...selectedIds], anchorId };
  const result = nextSelection(treeInteractionDefinition(selectionMode).selection, current, topology, {
    targets: [{ granularity: TREE_INTERACTION_GRANULARITY, id: targetId }],
    merge,
    mode: selectionMode,
  });
  return { selectedIds: [...result.ids], anchorId: result.anchorId };
};

export const collectTreeItemMap = (items: TreeDataItem[], map: Record<string, TreeDataItem> = {}): Record<string, TreeDataItem> => {
  items.forEach((item) => {
    map[item.id] = item;
    if (item.items?.length) {
      collectTreeItemMap(item.items, map);
    }
  });
  return map;
};

/** ✅️ Per-tree selection store; rows subscribe via {@link useSyncExternalStore} without invalidating {@link TreeDataRenderingContext}. */
interface TreeSelectionStore {
  subscribe: (listener: () => void) => () => void;
  getSelectedIds: () => readonly string[];
  isSelected: (itemId: string) => boolean;
  setSelectedIds: (selectedIds: readonly string[]) => void;
}

export function createTreeSelectionStore(): TreeSelectionStore {
  let selectedIds: readonly string[] = [];
  let selectedIdSet = new Set<string>();
  const listeners = new Set<() => void>();
  const notify = (): void => {
    for (const listener of listeners) {
      listener();
    }
  };
  return {
    subscribe(listener) {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
    getSelectedIds() {
      return selectedIds;
    },
    isSelected(itemId) {
      return selectedIdSet.has(itemId);
    },
    setSelectedIds(nextIds) {
      if (nextIds.length === selectedIds.length && nextIds.every((id, index) => id === selectedIds[index])) {
        return;
      }
      selectedIds = nextIds;
      selectedIdSet = new Set(nextIds);
      notify();
    },
  };
}

export const TreeSelectionContext = reactHostPort.createContext<TreeSelectionStore | null>(null);

function useTreeSelectionStore(): TreeSelectionStore {
  const value = reactHostPort.useContext(TreeSelectionContext);
  if (!value) {
    throw new Error("Tree selection hooks must render inside Tree");
  }
  return value;
}

export function useTreeItemRowSelected(itemId: string, itemSelectedOverride: boolean | undefined): boolean {
  const selectionStore = useTreeSelectionStore();
  const subscribedSelected = reactHostPort.useSyncExternalStore(
    selectionStore.subscribe,
    () => selectionStore.isSelected(itemId),
    () => selectionStore.isSelected(itemId),
  );
  return itemSelectedOverride ?? subscribedSelected;
}

/** 🖱️ Per-tree highlight store; rows subscribe via {@link useSyncExternalStore} without invalidating {@link TreeDataRenderingContext}. */
interface TreeHighlightStore {
  subscribe: (listener: () => void) => () => void;
  getHighlightedIds: () => readonly string[];
  isHighlighted: (itemId: string) => boolean;
  setHighlightedIds: (highlightedIds: readonly string[]) => void;
}

export function createTreeHighlightStore(): TreeHighlightStore {
  let highlightedIds: readonly string[] = [];
  let highlightedIdSet = new Set<string>();
  const listeners = new Set<() => void>();
  const notify = (): void => {
    for (const listener of listeners) {
      listener();
    }
  };
  return {
    subscribe(listener) {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
    getHighlightedIds() {
      return highlightedIds;
    },
    isHighlighted(itemId) {
      return highlightedIdSet.has(itemId);
    },
    setHighlightedIds(nextIds) {
      if (nextIds.length === highlightedIds.length && nextIds.every((id, index) => id === highlightedIds[index])) {
        return;
      }
      highlightedIds = nextIds;
      highlightedIdSet = new Set(nextIds);
      notify();
    },
  };
}

export const TreeHighlightContext = reactHostPort.createContext<TreeHighlightStore | null>(null);

function useTreeHighlightStore(): TreeHighlightStore {
  const value = reactHostPort.useContext(TreeHighlightContext);
  if (!value) {
    throw new Error("Tree highlight hooks must render inside Tree");
  }
  return value;
}

export function useTreeItemRowHighlighted(itemId: string, itemHighlightedOverride: boolean | undefined): boolean {
  const highlightStore = useTreeHighlightStore();
  const subscribedHighlighted = reactHostPort.useSyncExternalStore(
    highlightStore.subscribe,
    () => highlightStore.isHighlighted(itemId),
    () => highlightStore.isHighlighted(itemId),
  );
  return itemHighlightedOverride ?? subscribedHighlighted;
}
