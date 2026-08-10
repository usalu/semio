/** @emoji 📦️ `@semio-tech/framework` — package glue (reexports + inline vitest). */
export * from "../../🔨️modules/🎯️action-bus/🟦️component.ts";
export * from "../../🔨️modules/🧬️schema/🟦️component.ts";
export * from "../../🔨️modules/🖥️platform/🟦️component.ts";
export * from "../../🔨️modules/🔺️mesh/🟦️component.ts";
export * from "../../🔨️modules/🛂️manifest/🟦️component.ts";
export * from "../../🔨️modules/🎠️kernel/🟦️component.ts";

import {
  organizeContextMenu,
  type ContextMenuItemSpec,
} from "../../🔨️modules/🔺️mesh/🟦️component.ts";
import {
  createMemoryStoragePort,
  DockLayoutStore,
  DockUiStateStore,
  WindowPaneStateStore,
  type DockSkeleton,
  type DockUiState,
  type WindowPaneUiState,
} from "../../🔨️modules/🖥️platform/🟦️component.ts";
import {
  createDevPluginSource,
  createExtensionSource,
  multiplexPluginSources,
  pluginWorkerUrl,
  resolvePlaygroundBoot,
  resolvePluginHostConfig,
  resolvePluginRegistryId,
  acquirePluginModule,
  evictPluginModule,
  createLeasePool,
} from "../../🔨️modules/🎠️kernel/🟦️component.ts";

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
      const osState: DockUiState = { ...emptyUiState(), pathMemory: { "framework.category.workbench": "framework.panel.artifact" } };
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

    it("multiplexPluginSources() merges list() and resolves moduleUrl from the matching child", async () => {
      const dev = createDevPluginSource(registry);
      const extensions = createExtensionSource();
      const multiplexed = multiplexPluginSources(dev, extensions);
      expect(multiplexed.id).toBe("dev+extensions");
      const listed = await multiplexed.list();
      expect(listed.map((entry) => entry.pluginId).sort()).toEqual([...registry.map((entry) => entry.pluginId), ...EXTENSION_TARGETS.map((entry) => entry.pluginId)].sort());
      expect(multiplexed.moduleUrl("note")).toBe("/plugin-modules/note/note_plugin.js");
      expect(() => multiplexed.moduleUrl("missing")).toThrow(/missing/);
    });
  });

  describe("ephemeralBox", () => {
    it("stores a function-typed init as the current value (not as a lazy factory)", () => {
      const identity = (id: string) => id;
      const box = ephemeralBox<(id: string) => string>(`test.ephemeralBox.fn.${Math.random()}`, identity);
      expect(typeof box.current).toBe("function");
      expect(box.current("ui.nav.back")).toBe("ui.nav.back");
    });

    it("stores a no-op function init without invoking it", () => {
      let calls = 0;
      const noop = () => {
        calls += 1;
      };
      const box = ephemeralBox<() => void>(`test.ephemeralBox.noop.${Math.random()}`, noop);
      expect(typeof box.current).toBe("function");
      expect(calls).toBe(0);
      box.current();
      expect(calls).toBe(1);
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
