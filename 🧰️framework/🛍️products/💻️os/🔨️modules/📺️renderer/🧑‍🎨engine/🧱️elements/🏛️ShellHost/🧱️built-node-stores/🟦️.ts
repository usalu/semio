/** 🧱️ Built-node reconciler helpers, the pending window ui node, the built-node store cache and the spawned-refresh budgets and slot types.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHost}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import {
  type AppDefinition,
  type LocalizedLabel,
  panelTabKindId,
  pendingPanelUiNode,
  pendingWindowUiNode,
  type BuiltNode,
  type Severity
} from "@semio-tech/framework";
// #endregion 🔌️Adapters
import { panelTabChildren, type PanelTabNode } from "@semio-tech/ui-react";
import { builtNodeToSnapshot, UiDocumentStore } from "../../📃️UiDocumentStore/🟦️.tsx";
import type { ViewModel } from "../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { flattenPanelTabLeaves, shellLabel } from "../../🛠️ShellHelpers/🟦️.tsx";
import { isPluginInstanceRetiredV1 } from "../../🔌️PluginRuntime/🟦️.tsx";

//#region 🔖️BuiltNodeReconciler
/** 🩹️ `WindowKindDefinition.label` has no owned schema mirror yet (`unknown` — see that generated type's own
 * doc comment, and `resolveManifestLabel`'s "some call sites may still see the pre-migration shape
 * until that typegen lands" note) — asserts each window kind's real, documented label shape for the
 * `*LayoutSeed`/`retitleWindowLayoutNode` helpers that still declare their `windowKinds` param
 * strictly as `{ id, label: LocalizedLabel | string }[]`. */
export function withLocalizedWindowKindLabels(windowKinds: readonly { readonly id: string; readonly label: unknown }[]): readonly { readonly id: string; readonly label: LocalizedLabel | string }[] {
  return windowKinds as readonly { readonly id: string; readonly label: LocalizedLabel | string }[];
}

/** 🌳️ Every node of a `PanelTabNode` tree, branches included — the walk an id lookup needs: a plugin
 * that owns `framework.panel.artifact` as a BRANCH (remodel's Reconstruction/Run children) was invisible
 * to the leaves-only flatten this replaces, and the shell then mounted its own Artifact tab beside the
 * plugin's under the same React key. `flattenPanelTabLeaves`'s generic constraint rejects `PanelTabNode`
 * as a TS "weak type" (its `PanelTabLeaf` variant carries no `children` key at all, see that helper's own
 * doc comment), so this walks the union via `panelTabChildren`'s own kind-aware accessor. */
export function flattenPanelTabNodes(tabs: readonly PanelTabNode[]): readonly PanelTabNode[] {
  return tabs.flatMap((tab) => {
    const children = panelTabChildren(tab);
    return children && children.length > 0 ? [tab, ...flattenPanelTabNodes(children)] : [tab];
  });
}

export function panelDefinitionPath(tabs: AppDefinition["panelTabs"], targetId: string): readonly string[] | null {
  for (const tab of tabs) {
    const id = panelTabKindId(tab.kind);
    if (id === targetId && tab.children.length === 0) return [id];
    const childPath = panelDefinitionPath(tab.children, targetId);
    if (childPath) return [id, ...childPath];
  }
  return null;
}

export function requiredHostPanelLeafId(app: AppDefinition | undefined): string {
  const leaf = app ? flattenPanelTabLeaves(app.panelTabs)[0] : undefined;
  if (!leaf) throw new Error("configured host app has no panel leaf");
  return panelTabKindId(leaf.kind);
}
//#endregion 🔖️BuiltNodeReconciler

//#region FrameworkOsShell
//#region 🔖️BuiltNodeStoreCache
/** 🦴 The one placeholder body every not-yet-refreshed window shares. Reference-stable on purpose:
 * `pendingWindowUiNode()` allocates a fresh node per call, so calling it inside the shell's JSX made
 * every render look like an authored-node change to {@link createBuiltNodeStoreCacheV1} and reloaded
 * that window's store on each pass. */
export const PENDING_WINDOW_UI_NODE: BuiltNode = pendingWindowUiNode();

/** 🧬️ One `📃️UiDocumentStore` per window/panel key, reloaded from its authored `BuiltNode` only when
 * that node's own reference changed. `storeFor` is called from JSX during the shell's render, so it
 * MUST NOT mutate a store that already has mounted `UiNodeView` subscribers: `loadSnapshot` notifies
 * them, and a subscriber update raised while another component renders is exactly React's
 * "Cannot update a component (`UiNodeView`) while rendering a different component
 * (`FrameworkOsShellInner`)" warning (`📓️runtime-verification-2026-09-09.md` boot #3). A first load
 * happens inline — a store nobody has subscribed to yet cannot notify anyone — and every later reload
 * is queued for {@link BuiltNodeStoreCacheV1.flushPendingReloads}, which the shell calls from a layout
 * effect (post-commit, pre-paint, so nothing renders stale).
 *
 * 🔢️ Every reload carries the next revision of that store. A built node has no retained revision of its own,
 * and loading each one at revision 0 left `useUiDocumentRevision` constant for the store's whole life: a
 * window body that re-rendered in place never re-ran an effect keyed on its revision, so Home's windowed
 * table measured its 1-row window once after a reload and served "Rows 1–1 of 6" forever (ticket 26/09/23
 * U5 §12-1). */
export type BuiltNodeStoreCacheV1 = {
  readonly storeFor: (key: string, node: BuiltNode) => UiDocumentStore;
  /** 🔎️ The store already minted for `key`, without offering a node — what a render reads for a body it does not own
   * (a spawned window's body is loaded outside render, {@link BuiltNodeStoreCacheV1.flushPendingReloads}). */
  readonly storeOf: (key: string) => UiDocumentStore | null;
  /** 🧾️ The authored node `store` currently holds — set before a reload notifies, so a subscriber reads the new one. */
  readonly nodeOf: (store: UiDocumentStore) => BuiltNode | null;
  readonly flushPendingReloads: () => void;
  readonly forceReload: (key: string, node: BuiltNode) => void;
  readonly pendingReloadKeys: () => readonly string[];
};

export function createBuiltNodeStoreCacheV1(): BuiltNodeStoreCacheV1 {
  const stores = new Map<string, { node: BuiltNode; revision: number; readonly store: UiDocumentStore }>();
  const byStore = new WeakMap<UiDocumentStore, { node: BuiltNode }>();
  const pending = new Map<string, BuiltNode>();
  return {
    storeFor: (key, node) => {
      const existing = stores.get(key);
      if (!existing) {
        const store = new UiDocumentStore(key);
        store.loadSnapshot(builtNodeToSnapshot(key, node));
        const entry = { node, revision: 0, store };
        stores.set(key, entry);
        byStore.set(store, entry);
        return store;
      }
      if (existing.node === node) pending.delete(key);
      else pending.set(key, node);
      return existing.store;
    },
    storeOf: (key) => stores.get(key)?.store ?? null,
    nodeOf: (store) => byStore.get(store)?.node ?? null,
    flushPendingReloads: () => {
      for (const [key, node] of pending) {
        const entry = stores.get(key);
        if (!entry) continue;
        entry.revision += 1;
        entry.node = node;
        entry.store.loadSnapshot(builtNodeToSnapshot(key, node, entry.revision));
      }
      pending.clear();
    },
    forceReload: (key, node) => {
      const existing = stores.get(key);
      if (!existing) return;
      pending.set(key, node);
    },
    pendingReloadKeys: () => [...pending.keys()],
  };
}

/** 📤️ Loads bodies into their stores OUTSIDE render and reloads them at once. For a body no render owns — a spawned
 * program's window, `spawned:<windowId>` — the store is its only holder: the mounted `UiNodeView`s update in place and
 * only a window gaining or losing its body, or a changed activity, reaches the shell reducer. */
export function publishBuiltNodesV1(cache: BuiltNodeStoreCacheV1, entries: readonly (readonly [string, BuiltNode])[]): void {
  for (const [key, node] of entries) cache.storeFor(key, node);
  cache.flushPendingReloads();
}

/** 🗂️ Publishes panel bodies into their stores (`panel:<record key>`) outside render and answers each key's store: a body
 * the refresh did not re-serialize keeps its store's content, a panel with no store yet starts on the pending body. */
export function publishPanelBodiesV1(cache: BuiltNodeStoreCacheV1, bodies: readonly (readonly [string, BuiltNode | undefined])[]): (readonly [string, UiDocumentStore])[] {
  publishBuiltNodesV1(
    cache,
    bodies.flatMap(([key, node]) => (node !== undefined ? [[`panel:${key}`, node] as const] : cache.storeOf(`panel:${key}`) === null ? [[`panel:${key}`, pendingPanelUiNode()] as const] : [])),
  );
  return bodies.map(([key]) => [key, cache.storeOf(`panel:${key}`)!] as const);
}
//#endregion 🔖️BuiltNodeStoreCache

/** ⌨️ How many animation frames an opened program's active window has to mount before it is no longer offered focus. */
export const SPAWNED_FOCUS_FRAME_BUDGET = 240;

/** 🪦️ The catch every refresh and effect lane shares: a failure that only says the instance it addressed was retired
 * under it ({@link isPluginInstanceRetiredV1} — a session switch, a hot swap, a revoked activation) is cancelled with
 * that instance, and every other failure stays loud. The lanes used to log both, so a load that crossed a session switch
 * printed `typed-operation progress refresh failed … actor-activation.revoked` (ticket 26/09/23 U5, C10 relay). */
/** 🌐️ A fresh id for a preference-lane change or its directory command request: 32 lowercase hex characters, the
 * directory's request-id grammar. */
export function mintPreferenceLaneIdV1(): string {
  return crypto.randomUUID().replaceAll("-", "");
}

export function logUnlessRetiredV1(what: string): (error: unknown) => void {
  return (error) => {
    if (!isPluginInstanceRetiredV1(error)) console.error(what, error);
  };
}

/** 📎️ A sync attach that failed is told to the person and logged with its cause; the attach of a program retired under it
 * is neither. Nothing is saved to a target that did not attach, so the failure is never left to the console alone. */
export function tellSyncAttachFailureV1(notice: (message: string, kind?: Severity, code?: string) => void): (error: unknown) => void {
  return (error) => {
    if (isPluginInstanceRetiredV1(error)) return;
    console.error("[os-shell] sync attach failed", error);
    notice(shellLabel("ui.sync.attachFailed"), "error", "sync.attach.failed");
  };
}

/** 🔁️ One spawned program's refresh in flight: at most one pass runs per program, and a refresh asked for while it
 * runs only records the newest view state and that another pass is owed, which runs the moment this one lands — every
 * caller awaits the same `done`, and a newer refresh never discards an older one's result. The owed pass is always the
 * NEWEST `refreshSpawnedUiPass` (read through a ref), so a pass owed across a language switch renders in the new
 * language instead of the one the first caller's closure held (ticket 26/09/23 U5 §12-2, §12-3). */
export type SpawnedRefreshSlotV1 = { viewState: ViewModel; again: boolean; done: Promise<void> };
//#endregion FrameworkOsShell
