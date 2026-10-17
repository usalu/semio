/** 🧭️ The playground resolution, effective action arguments, context menu organisation, plugin sources and lease pool eviction of the framework in-source suites. */
import { loadKernelSourceWatchFixture, loadKernelSchema } from "@semio-tech/framework-kernel/testing/fixtures";
import type { ActionArgDef, ArgSchema, ContextMenuItemSpec, PluginCatalog, PluginRegistryEntry, PluginSourceEvent } from "@semio-tech/framework";

export async function registerPlaygroundActionAndPluginSourceTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: typeof import("@semio-tech/framework")): Promise<void> {
  const { createDevPluginSource, createExtensionSource, createLeasePool, effectiveActionArgs, PluginModuleUnavailableError, extensionSourceEventToPluginSourceEvent, missingRequiredArgs, multiplexPluginSources, organizeContextMenu, resolvePlaygroundBoot, resolvePluginHostConfig, resolvePluginRegistryId } = dependencies;
  const { describe, expect, it, vi } = vitest;

  // 🧪️ Framework-level tests must not know about real product plugin ids (`"puzzle3d"`, `"s"`, …) — a
  // small synthetic catalog stands in for the OS product's generated `PLUGIN_CATALOG` here.
  const SYNTHETIC_PLUGIN_CATALOG: PluginCatalog = {
    plugins: [
      { pluginId: "alpha", wasmOut: "alpha.wasm", role: "plugin", contributes: [], consumes: [] },
      { pluginId: "beta", wasmOut: "beta.wasm", role: "plugin", contributes: [], consumes: [] },
    ],
    extensions: [{ pluginId: "beta-extension-gamma", wasmOut: "beta_gamma.wasm", role: "extension", contributes: ["beta.module"], consumes: [] }],
    hosts: [{ pluginId: "alpha", landingAppId: "home", hostAppId: "studio" }],
    playgrounds: [
      { variant: "alpha", pluginId: "alpha", aliases: [] },
      { variant: "beta-play", pluginId: "beta", app: "beta-play-app", aliases: ["b", "beta play"] },
    ],
    moduleUrl: (pluginId) => `/🔌️plugin-modules/${pluginId}/🌉️bridge.js`,
    extensionModuleUrl: (pluginId) => `/🧩️extension-modules/${pluginId}/🌉️bridge.js`,
  };

  describe("PlaygroundResolution", () => {
    it("resolves host config from the injected catalog", () => {
      expect(resolvePluginHostConfig(SYNTHETIC_PLUGIN_CATALOG, "alpha")).toEqual({ pluginId: "alpha", landingAppId: "home", hostAppId: "studio" });
      expect(resolvePluginHostConfig(SYNTHETIC_PLUGIN_CATALOG, "beta-play")).toBeUndefined();
    });

    it("resolves playground aliases to registry plugin ids", () => {
      expect(resolvePluginRegistryId(SYNTHETIC_PLUGIN_CATALOG, "b")).toBe("beta");
      expect(resolvePluginRegistryId(SYNTHETIC_PLUGIN_CATALOG, "beta play")).toBe("beta");
    });

    it("rebuilds program rows when the generated session variant is stale", () => {
      const boot = resolvePlaygroundBoot(SYNTHETIC_PLUGIN_CATALOG, "beta-play", {
        variant: "alpha",
        defaultAppId: "alpha-app",
        plugins: [{ pluginId: "alpha", moduleUrl: "/🔌️plugin-modules/alpha/alpha_plugin.js" }],
      });
      expect(boot.variant).toBe("beta-play");
      expect(boot.defaultAppId).toBe("beta-play-app");
      expect(boot.plugins).toEqual([{ pluginId: "beta", moduleUrl: "/🔌️plugin-modules/beta/🌉️bridge.js", contributes: [], consumes: [] }]);
    });
  });

  describe("effectiveActionArgs", () => {
    const TEXT_SCHEMA: ArgSchema = { kind: "string", options: [] };
    const textArg = (id: string, extra: Partial<ActionArgDef> = {}): ActionArgDef => ({
      id,
      label: id,
      schema: TEXT_SCHEMA,
      required: false,
      ...extra,
    });

    it("keeps a seeded arg that is not a declared form field, alongside the form's own staged fields (26/08/16 HUB-SPACES shareSpace regression: spaceId must reach the dispatched descriptor)", () => {
      const defs = [textArg("email")];
      const effective = effectiveActionArgs(defs, { email: "user2@semio.dev" }, { spaceId: "sp-1" });
      expect(effective).toEqual({ spaceId: "sp-1", email: "user2@semio.dev" });
    });

    it("a seed value for a declared field pre-fills it until the form stages its own value (renameSpace's current-name prefill)", () => {
      const defs = [textArg("name")];
      expect(effectiveActionArgs(defs, {}, { spaceId: "sp-1", name: "Old Name" })).toEqual({ spaceId: "sp-1", name: "Old Name" });
      expect(effectiveActionArgs(defs, { name: "New Name" }, { spaceId: "sp-1", name: "Old Name" })).toEqual({ spaceId: "sp-1", name: "New Name" });
    });

    it("a zero-declared-field confirm dialog passes seed+staged through wholesale (deleteSpace's confirm/cancel shape)", () => {
      expect(effectiveActionArgs([], {}, { spaceId: "sp-1", confirmed: true })).toEqual({ spaceId: "sp-1", confirmed: true });
    });

    it("missingRequiredArgs is unaffected by extra seed keys", () => {
      const defs = [textArg("email", { required: true })];
      const effective = effectiveActionArgs(defs, {}, { spaceId: "sp-1" });
      expect(missingRequiredArgs(defs, effective)).toEqual(["email"]);
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

  describe("PluginSource", () => {
    const registry: readonly PluginRegistryEntry[] = [
      { pluginId: "note", moduleUrl: "/🔌️plugin-modules/note/note_plugin.js" },
      { pluginId: "s", moduleUrl: "/🔌️plugin-modules/s/s_plugin.js" },
    ];
    const silentWatch = () => () => {};

    it("opens and closes exactly its owner's watch streams without a product route dependency", async () => {
      const fixture = await loadKernelSourceWatchFixture();
      const schema = await loadKernelSchema();
      const { default: Ajv } = await import("ajv");
      
      const opened: string[] = [], closed: string[] = [];
      const watchFor = (label: string) => () => {
        opened.push(label);
        return () => void closed.push(label);
      };
      for (const stream of fixture.streams) {
        const source = stream.source === "dev" ? createDevPluginSource(registry, watchFor(stream.watch)) : createExtensionSource(SYNTHETIC_PLUGIN_CATALOG, watchFor(stream.watch));
        const unsubscribe = source.subscribe(() => {});
        expect(opened.at(-1)).toBe(stream.watch);
        unsubscribe();
        expect(closed.at(-1)).toBe(stream.watch);
      }
      expect(opened).toEqual(fixture.streams.map((stream) => stream.watch));
      expect(closed).toEqual(opened);
    });

    it("list() returns the registry it was created with", async () => {
      const source = createDevPluginSource(registry, silentWatch);
      expect(source.id).toBe("dev");
      await expect(source.list()).resolves.toEqual(registry);
    });

    /** 🔎️ Answers the HEAD probe of a locally served module's descriptor: `served` paths answer 200. */
    const servedModules = (served: readonly string[]) => {
      const probes: string[] = [];
      vi.stubGlobal("fetch", async (url: string, init?: RequestInit) => {
        probes.push(`${init?.method ?? "GET"} ${decodeURIComponent(url)}`);
        return new Response(null, { status: served.includes(decodeURIComponent(url)) ? 200 : 404 });
      });
      return probes;
    };
    const acquisition = () => ({ signal: new AbortController().signal });

    it("acquireModule() cache-busts a cold page load even before the build snapshot arrives", async () => {
      const probes = servedModules(["/🔌️plugin-modules/note/🔣️.json", "/🔌️plugin-modules/s/🔣️.json"]);
      try {
        const source = createDevPluginSource(registry, silentWatch);
        const firstAcquired = await source.acquireModule("note", undefined, acquisition());
        const first = new URL(firstAcquired.moduleUrl, "http://semio.test");
        const second = new URL((await source.acquireModule("s", undefined, acquisition())).moduleUrl, "http://semio.test");
        expect(decodeURIComponent(first.pathname)).toBe("/🔌️plugin-modules/note/note_plugin.js");
        expect(first.searchParams.get("v")).toMatch(/^\d+$/);
        expect(second.searchParams.get("v")).toBe(first.searchParams.get("v"));
        expect(firstAcquired.rebuiltAt).toBe(Number(first.searchParams.get("v")));
        expect(probes).toEqual(["HEAD /🔌️plugin-modules/note/🔣️.json", "HEAD /🔌️plugin-modules/s/🔣️.json"]);
      } finally {
        vi.unstubAllGlobals();
      }
    });

    it("acquireModule() cache-busts with a rebuiltAt query param", async () => {
      servedModules(["/🔌️plugin-modules/note/🔣️.json"]);
      try {
        const source = createDevPluginSource(registry, silentWatch);
        await expect(source.acquireModule("note", 1785789943669, acquisition())).resolves.toEqual({ moduleUrl: "/🔌️plugin-modules/note/note_plugin.js?v=1785789943669", rebuiltAt: 1785789943669 });
      } finally {
        vi.unstubAllGlobals();
      }
    });

    it("acquireModule() answers unavailable for an unknown pluginId and for a module this device does not serve", async () => {
      servedModules([]);
      try {
        const source = createDevPluginSource(registry, silentWatch);
        await expect(source.acquireModule("missing", undefined, acquisition())).rejects.toBeInstanceOf(PluginModuleUnavailableError);
        await expect(source.acquireModule("note", undefined, acquisition())).rejects.toThrow(/HTTP 404/);
      } finally {
        vi.unstubAllGlobals();
      }
    });

    it("subscribe() through a watch that never delivers yields nothing and unsubscribes cleanly", () => {
      const source = createDevPluginSource(registry, silentWatch);
      const events: PluginSourceEvent[] = [];
      const unsubscribe = source.subscribe((event) => events.push(event));
      expect(() => unsubscribe()).not.toThrow();
      expect(events).toEqual([]);
    });

    it("normalizes extension snapshots and installs into plugin availability events", () => {
      expect(extensionSourceEventToPluginSourceEvent({ kind: "snapshot", extensions: [{ extensionId: "gamma-extension", installedAt: 1785789943669 }] })).toEqual({
        kind: "snapshot",
        plugins: [{ pluginId: "gamma-extension", rebuiltAt: 1785789943669 }],
      });
      expect(extensionSourceEventToPluginSourceEvent({ kind: "installed", extensionId: "gamma-extension", installedAt: 1785789943670 })).toEqual({
        kind: "built",
        pluginId: "gamma-extension",
        rebuiltAt: 1785789943670,
      });
      expect(extensionSourceEventToPluginSourceEvent({ kind: "uninstalled", extensionId: "gamma-extension" })).toBeUndefined();
    });

    it("multiplexPluginSources() merges list() and acquires from the first child that serves the module", async () => {
      const catalog: PluginCatalog = {
        plugins: [],
        extensions: [{ pluginId: "gamma-extension", wasmOut: "gamma.wasm", role: "extension", contributes: [], consumes: [] }],
        hosts: [],
        playgrounds: [],
        moduleUrl: (pluginId) => `/🔌️plugin-modules/${pluginId}/🌉️bridge.js`,
        extensionModuleUrl: (pluginId) => `/🧩️extension-modules/${pluginId}/🌉️bridge.js`,
      };
      const dev = createDevPluginSource(registry, silentWatch);
      const extensions = createExtensionSource(catalog, silentWatch);
      const multiplexed = multiplexPluginSources(dev, extensions);
      expect(multiplexed.id).toBe("dev+extensions");
      const listed = await multiplexed.list();
      expect(listed.map((entry) => entry.pluginId).sort()).toEqual([...registry.map((entry) => entry.pluginId), ...catalog.extensions.map((entry) => entry.pluginId)].sort());
      servedModules(["/🔌️plugin-modules/note/🔣️.json"]);
      try {
        expect(decodeURIComponent(new URL((await multiplexed.acquireModule("note", undefined, acquisition())).moduleUrl, "http://semio.test").pathname)).toBe("/🔌️plugin-modules/note/note_plugin.js");
        await expect(multiplexed.acquireModule("gamma-extension", 7, acquisition())).resolves.toEqual({ moduleUrl: "/🧩️extension-modules/gamma-extension/🌉️bridge.js?v=7", rebuiltAt: 7 });
        await expect(multiplexed.acquireModule("missing", undefined, acquisition())).rejects.toBeInstanceOf(PluginModuleUnavailableError);
        const remote = { id: "remote", list: async () => [], subscribe: () => () => {}, acquireModule: async (pluginId: string) => ({ moduleUrl: `/remote/${pluginId}/🌉️bridge.js`, rebuiltAt: 9 }) };
        await expect(multiplexPluginSources(dev, remote).acquireModule("s", undefined, acquisition())).resolves.toEqual({ moduleUrl: "/remote/s/🌉️bridge.js", rebuiltAt: 9 });
        const failing = { ...remote, acquireModule: async () => Promise.reject(new Error("transport down")) };
        await expect(multiplexPluginSources(failing, dev).acquireModule("note", undefined, acquisition())).rejects.toThrow("transport down");
      } finally {
        vi.unstubAllGlobals();
      }
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
