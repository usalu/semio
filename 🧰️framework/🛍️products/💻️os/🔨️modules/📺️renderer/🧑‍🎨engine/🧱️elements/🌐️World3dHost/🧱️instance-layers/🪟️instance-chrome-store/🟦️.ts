/** 🪟️ World3d instance layers, chrome store: the instance chrome snapshot, the local hover paint id and the external store that publishes per-instance chrome to the r3f subtree. */
import { reactHostPort } from "@semio-tech/ui-react";

//#region 🔖️World3dHost
//#region WorldInstanceChromeStore
type WorldInstanceChromeSnapshot = {
  readonly selectedIds: ReadonlySet<string>;
  readonly hoveredId: string | null;
  readonly hoveredIds: ReadonlySet<string> | null;
  readonly hoverSourceWindowId?: string | null;
  readonly hoveredKindId: string | null;
  readonly previewInstanceIds: ReadonlySet<string> | null;
};

/** 🎯️ What THIS pane's own raycast has under the pointer: an instance id, `null` for "inside the
 * pane, over nothing", `undefined` for "the pointer is not in this pane" (no local claim). */
export type WorldLocalHoverV1 = string | null | undefined;

/** 🎯️ The instance the hover chrome paints. A local raycast leads until an authored hover changes
 * owner or target from another surface.
 * A delayed echo from this same window leaves its latest local raycast in place. */
export function worldHoverPaintIdV1(local: WorldLocalHoverV1, guest: string | null): string | null {
  return local === undefined ? guest : local;
}

interface WorldInstanceChromeStore {
  subscribe: (listener: () => void) => () => void;
  getSnapshot: () => WorldInstanceChromeSnapshot;
  setSnapshot: (next: WorldInstanceChromeSnapshot) => void;
  /** 🎯️ The pane's raycast result, painted at once and reconciled against the guest echo by
   * {@link worldHoverPaintIdV1}. */
  setLocalHover: (local: WorldLocalHoverV1) => void;
  localHover: () => WorldLocalHoverV1;
  isSelected: (instanceId: string) => boolean;
  isHovered: (instanceId: string) => boolean;
  isHighlighted: (objectKind: string | undefined) => boolean;
  previewSelected: (instanceId: string) => boolean | undefined;
}

function sameIds(a: ReadonlySet<string> | null, b: ReadonlySet<string> | null): boolean {
  if (a === b) return true;
  if (!a || !b || a.size !== b.size) return false;
  for (const id of a) {
    if (!b.has(id)) return false;
  }
  return true;
}

function sameAuthoredHover(a: WorldInstanceChromeSnapshot, b: WorldInstanceChromeSnapshot): boolean {
  return a.hoverSourceWindowId === b.hoverSourceWindowId && sameIds(a.hoveredIds, b.hoveredIds)
    && (a.hoveredIds !== null || a.hoveredId === b.hoveredId);
}

export function createWorldInstanceChromeStore(windowInstanceId?: string | null): WorldInstanceChromeStore {
  let snapshot: WorldInstanceChromeSnapshot = {
    selectedIds: new Set(),
    hoveredId: null,
    hoveredIds: null,
    hoveredKindId: null,
    previewInstanceIds: null,
  };
  let localHover: WorldLocalHoverV1 = undefined;
  const listeners = new Set<() => void>();
  const notify = (): void => {
    for (const listener of listeners) listener();
  };
  return {
    subscribe(listener) {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
    getSnapshot() {
      return snapshot;
    },
    setSnapshot(next) {
      const sameHover = sameAuthoredHover(snapshot, next);
      if (sameHover && snapshot.hoveredId === next.hoveredId && snapshot.hoveredKindId === next.hoveredKindId
        && sameIds(snapshot.selectedIds, next.selectedIds) && sameIds(snapshot.previewInstanceIds, next.previewInstanceIds)) return;
      const source = next.hoverSourceWindowId;
      if (!sameHover && (source === null || (typeof source === "string" && source !== windowInstanceId))) {
        localHover = undefined;
      }
      snapshot = next;
      notify();
    },
    setLocalHover(local) {
      if (local === localHover) return;
      const paintedBefore = worldHoverPaintIdV1(localHover, snapshot.hoveredId);
      const projectionChanged = snapshot.hoveredIds !== null && (localHover === undefined) !== (local === undefined);
      localHover = local;
      if (!projectionChanged && worldHoverPaintIdV1(localHover, snapshot.hoveredId) === paintedBefore) return;
      notify();
    },
    localHover() {
      return localHover;
    },
    isSelected(instanceId) {
      return snapshot.selectedIds.has(instanceId);
    },
    isHovered(instanceId) {
      if (localHover !== undefined) return localHover === instanceId;
      return snapshot.hoveredIds?.has(instanceId) ?? (snapshot.hoveredId === instanceId);
    },
    isHighlighted(objectKind) {
      return objectKind != null && snapshot.hoveredKindId != null && objectKind === snapshot.hoveredKindId;
    },
    previewSelected(instanceId) {
      if (!snapshot.previewInstanceIds) return undefined;
      return snapshot.previewInstanceIds.has(instanceId);
    },
  };
}

export const WorldInstanceChromeContext = reactHostPort.createContext<WorldInstanceChromeStore | null>(null);

function useWorldInstanceChromeStore(): WorldInstanceChromeStore {
  const store = reactHostPort.useContext(WorldInstanceChromeContext);
  if (!store) throw new Error("World instance chrome hooks must render inside WorldInstancesLayer");
  return store;
}

export function useWorldInstanceChrome(
  instanceId: string,
  objectKind: string | undefined,
): {
  readonly selected: boolean;
  readonly hovered: boolean;
  readonly highlighted: boolean;
  readonly previewSelected: boolean | undefined;
} {
  const store = useWorldInstanceChromeStore();
  const selected = reactHostPort.useSyncExternalStore(store.subscribe, () => store.isSelected(instanceId), () => false);
  const hovered = reactHostPort.useSyncExternalStore(store.subscribe, () => store.isHovered(instanceId), () => false);
  const highlighted = reactHostPort.useSyncExternalStore(store.subscribe, () => store.isHighlighted(objectKind), () => false);
  const previewSelected = reactHostPort.useSyncExternalStore(store.subscribe, () => store.previewSelected(instanceId), () => undefined);
  return { selected, hovered, highlighted, previewSelected };
}
//#endregion WorldInstanceChromeStore
//#endregion 🔖️World3dHost
