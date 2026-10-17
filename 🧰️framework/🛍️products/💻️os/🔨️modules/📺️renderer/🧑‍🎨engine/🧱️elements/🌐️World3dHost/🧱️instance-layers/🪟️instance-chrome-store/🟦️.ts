/** 🪟️ World3d instance layers, chrome store: the instance chrome snapshot, the local hover paint id and the external store that publishes per-instance chrome to the r3f subtree. */
import { reactHostPort } from "@semio-tech/ui-react";

//#region 🔖️World3dHost
//#region WorldInstanceChromeStore
type WorldInstanceChromeSnapshot = {
  readonly selectedIds: ReadonlySet<string>;
  readonly hoveredId: string | null;
  readonly hoveredKindId: string | null;
  readonly previewInstanceIds: ReadonlySet<string> | null;
};

/** 🎯️ What THIS pane's own raycast has under the pointer: an instance id, `null` for "inside the
 * pane, over nothing", `undefined` for "the pointer is not in this pane" (no local claim). */
export type WorldLocalHoverV1 = string | null | undefined;

/** 🎯️ The instance the hover chrome paints. The pane's own raycast LEADS while the pointer is inside
 * it — that is the value the guest is being told about, one round trip (hundreds of milliseconds
 * through `interactionHover` → reserved job → publication → refresh → commit) before the guest's
 * `hoveredId` echoes it back — so the highlight follows the pointer at raycast speed and the echo
 * only ever confirms it. The guest's lane leads when the pointer is elsewhere: a hover authored by
 * another surface (an outliner row, a remote presence) has nothing local to yield to.
 *
 * 🧭️ Nothing about the guest changes: the same `interactionHover` still reaches it, the same lane
 * comes back, the same selection semantics apply to a click. Only what is PAINTED between the raycast
 * and the echo differs, and it differs toward what the guest will answer. */
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

function chromeSnapshotsEqual(a: WorldInstanceChromeSnapshot, b: WorldInstanceChromeSnapshot): boolean {
  if (a.hoveredId !== b.hoveredId || a.hoveredKindId !== b.hoveredKindId) return false;
  if (a.previewInstanceIds !== b.previewInstanceIds) {
    if (!a.previewInstanceIds || !b.previewInstanceIds || a.previewInstanceIds.size !== b.previewInstanceIds.size) return false;
    for (const id of a.previewInstanceIds) {
      if (!b.previewInstanceIds.has(id)) return false;
    }
  }
  if (a.selectedIds.size !== b.selectedIds.size) return false;
  for (const id of a.selectedIds) {
    if (!b.selectedIds.has(id)) return false;
  }
  return true;
}

export function createWorldInstanceChromeStore(): WorldInstanceChromeStore {
  let snapshot: WorldInstanceChromeSnapshot = {
    selectedIds: new Set(),
    hoveredId: null,
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
      if (chromeSnapshotsEqual(snapshot, next)) return;
      snapshot = next;
      notify();
    },
    setLocalHover(local) {
      if (local === localHover) return;
      const paintedBefore = worldHoverPaintIdV1(localHover, snapshot.hoveredId);
      localHover = local;
      if (worldHoverPaintIdV1(localHover, snapshot.hoveredId) === paintedBefore) return;
      notify();
    },
    localHover() {
      return localHover;
    },
    isSelected(instanceId) {
      return snapshot.selectedIds.has(instanceId);
    },
    isHovered(instanceId) {
      return worldHoverPaintIdV1(localHover, snapshot.hoveredId) === instanceId;
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
