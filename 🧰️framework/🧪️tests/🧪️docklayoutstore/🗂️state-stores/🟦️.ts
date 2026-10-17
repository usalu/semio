/** 🗂️ The dock layout, dock UI state and window pane state stores and the OS shell configuration of the framework in-source suites. */
import type { DockSkeleton, DockUiState, WindowPaneUiState } from "@semio-tech/framework";

export async function registerStateStoreTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: typeof import("@semio-tech/framework")): Promise<void> {
  const { DockLayoutStore, DockUiStateStore, OsShellConfig, WindowPaneStateStore, createMemoryStoragePort } = dependencies;
  const { describe, expect, it } = vitest;

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
      expect(new OsShellConfig(storage).getSnapshot().dockLayouts.apps["my-app"]).toEqual(emptySkeleton());
      store.save(null);
      expect(new OsShellConfig(storage).getSnapshot().dockLayouts.apps["my-app"]).toBeUndefined();
      expect(store.getSnapshot()).toBeNull();
    });

    it("reset() clears both layers", () => {
      const storage = createMemoryStoragePort();
      const store = new DockLayoutStore(storage, "my-app");
      store.saveOs(emptySkeleton());
      store.save(emptySkeleton());
      store.reset();
      expect(new OsShellConfig(storage).getSnapshot().dockLayouts).toEqual({ apps: {} });
      expect(store.getSnapshot()).toBeNull();
    });

    it("returns null on corrupt JSON rather than throwing", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.config", "{not json");
      const store = new DockLayoutStore(storage);
      expect(() => store.getSnapshot()).not.toThrow();
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a stale version-1 (corners) blob instead of migrating it", () => {
      const storage = createMemoryStoragePort();
      new OsShellConfig(storage).update((current) => ({ ...current, dockLayouts: { os: { version: 1, corners: { "top-left": [{ id: "a" }] } } as unknown as DockSkeleton, apps: {} } }));
      const store = new DockLayoutStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a stale version-2 (six-anchor) blob instead of migrating it to eight anchors", () => {
      const storage = createMemoryStoragePort();
      new OsShellConfig(storage).update((current) => ({ ...current, dockLayouts: { os: { version: 2, anchors: {} } as unknown as DockSkeleton, apps: {} } }));
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
      expect(new OsShellConfig(storage).getSnapshot().dockUi.apps["my-app"]).toEqual(emptyUiState());
      store.save(null);
      expect(new OsShellConfig(storage).getSnapshot().dockUi.apps["my-app"]).toBeUndefined();
      expect(store.getSnapshot()).toBeNull();
    });

    it("reset() clears both layers", () => {
      const storage = createMemoryStoragePort();
      const store = new DockUiStateStore(storage, "my-app");
      store.saveOs(emptyUiState());
      store.save(emptyUiState());
      store.reset();
      expect(new OsShellConfig(storage).getSnapshot().dockUi).toEqual({ apps: {} });
      expect(store.getSnapshot()).toBeNull();
    });

    it("returns null on corrupt JSON rather than throwing", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.config", "{not json");
      const store = new DockUiStateStore(storage);
      expect(() => store.getSnapshot()).not.toThrow();
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a stale version-1 (corners) blob instead of migrating it", () => {
      const storage = createMemoryStoragePort();
      new OsShellConfig(storage).update((current) => ({ ...current, dockUi: { os: { version: 1, corners: {} } as unknown as DockUiState, apps: {} } }));
      const store = new DockUiStateStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a stale version-2 (six-anchor) blob instead of migrating it to eight anchors", () => {
      const storage = createMemoryStoragePort();
      new OsShellConfig(storage).update((current) => ({ ...current, dockUi: { os: { version: 2, anchors: {} } as unknown as DockUiState, apps: {} } }));
      const store = new DockUiStateStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });

    it('keeps dock layout and dock ui as distinct projections for an app literally named "ui"', () => {
      const storage = createMemoryStoragePort();
      new DockLayoutStore(storage, "ui").save({
        version: 3,
        anchors: { "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
      });
      new DockUiStateStore(storage).saveOs(emptyUiState());
      const config = new OsShellConfig(storage).getSnapshot();
      expect(config.dockLayouts.apps.ui).toBeDefined();
      expect(config.dockUi.os).toEqual(emptyUiState());
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
      expect(new OsShellConfig(storage).getSnapshot().windowPanes.apps["my-app"]).toEqual(emptyPaneState());
      store.save(null);
      expect(new OsShellConfig(storage).getSnapshot().windowPanes.apps["my-app"]).toBeUndefined();
      expect(store.getSnapshot()).toBeNull();
    });

    it("reset() clears both layers", () => {
      const storage = createMemoryStoragePort();
      const store = new WindowPaneStateStore(storage, "my-app");
      store.saveOs(emptyPaneState());
      store.save(emptyPaneState());
      store.reset();
      expect(new OsShellConfig(storage).getSnapshot().windowPanes).toEqual({ apps: {} });
      expect(store.getSnapshot()).toBeNull();
    });

    it("returns null on corrupt JSON rather than throwing", () => {
      const storage = createMemoryStoragePort();
      storage.set("semio.os.config", "{not json");
      const store = new WindowPaneStateStore(storage);
      expect(() => store.getSnapshot()).not.toThrow();
      expect(store.getSnapshot()).toBeNull();
    });

    it("discards a foreign-version blob instead of migrating it", () => {
      const storage = createMemoryStoragePort();
      new OsShellConfig(storage).update((current) => ({ ...current, windowPanes: { os: { version: 2, windows: {} } as unknown as WindowPaneUiState, apps: {} } }));
      const store = new WindowPaneStateStore(storage);
      expect(store.getSnapshot()).toBeNull();
    });
  });

  describe("OsShellConfig", () => {
    it("consolidates all three persisted device-local shell projections into one config document", () => {
      const values = new Map<string, string>();
      const storage = {
        get: (key: string) => values.get(key) ?? null,
        set: (key: string, value: string) => void values.set(key, value),
        remove: (key: string) => void values.delete(key),
      };
      const skeleton: DockSkeleton = {
        version: 3,
        anchors: { "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
      };
      new DockLayoutStore(storage, "draw").save(skeleton);
      new DockUiStateStore(storage, "draw").save({ version: 3, anchors: { "left-middle": { visible: true } } });
      new WindowPaneStateStore(storage, "draw").save({ version: 1, windows: {} });
      new OsShellConfig(storage).setPreference("ui.chrome.locale", "de");

      expect([...values.keys()]).toEqual(["semio.os.config"]);
      const snapshot = new OsShellConfig(storage).getSnapshot();
      expect(snapshot.dockLayouts.apps.draw).toEqual(skeleton);
      expect(snapshot.dockUi.apps.draw?.anchors["left-middle"]?.visible).toBe(true);
      expect(snapshot.windowPanes.apps.draw).toEqual({ version: 1, windows: {} });
      expect(snapshot.preferences["ui.chrome.locale"]).toBe("de");
    });
  });
}
