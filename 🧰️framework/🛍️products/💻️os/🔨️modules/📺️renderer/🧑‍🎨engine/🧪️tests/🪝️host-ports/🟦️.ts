// @vitest-environment jsdom
/** 🧪️ Host ports: 6 tests covering shell host ports of the product composition; binds the catalog admission to the plugin registry's own admission; answers document write access from the declared directory access policy; and more. */
/** 🪝️ The product-side composition of the shell host's ports: each port answers from the product module it binds, so the renderer can stay free of them. */
import { afterEach, describe, expect, it, vi } from "vitest";
import { setTheme } from "@semio-tech/framework/config/mutations";
import { createFrameworkOsHostPortsV1 } from "@semio-tech/framework-os-dev/host-ports";
import { MODULE_EXTENSION_ROUTE } from "@semio-tech/plugin-registry/deployment";

afterEach(() => vi.unstubAllGlobals());

describe("shell host ports of the product composition", () => {
  it("binds the catalog admission to the plugin registry's own admission", () => {
    const { catalogAdmission } = createFrameworkOsHostPortsV1();
    const control = { maxBytes: 2097152, maxRows: 128, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => false, progress: () => {} };
    const rows = catalogAdmission!.admit({ version: 1, targets: [], hosts: [], playgrounds: [] }, control);
    expect(rows).toEqual({ version: 1, targets: [], hosts: [], playgrounds: [] });
    expect(catalogAdmission!.catalogOf(rows).plugins).toEqual([]);
    expect(() => catalogAdmission!.admit({ version: 2 }, control)).toThrow("plugin-catalog-invalid");
  });

  it("answers document write access from the declared directory access policy", () => {
    const { documentAccess } = createFrameworkOsHostPortsV1();
    expect(documentAccess!.permitsDocumentWrite("author")).toBe(true);
    expect(documentAccess!.permitsDocumentWrite("spectator")).toBe(false);
  });

  it("classifies preference mutations from the mutation schema", () => {
    const { preferenceVocabulary } = createFrameworkOsHostPortsV1();
    expect(preferenceVocabulary!.knows("setTheme")).toBe(true);
    expect(preferenceVocabulary!.knows("setNothing")).toBe(false);
    expect(preferenceVocabulary!.dataClassOf(setTheme("night"))).toBe("persistedShared");
    expect(typeof preferenceVocabulary!.keyOf(setTheme("night"))).toBe("string");
  });

  it("serves the extension store on the deployment route and a hub source only where the browser can host the module store", () => {
    const { pluginDistribution } = createFrameworkOsHostPortsV1();
    expect(pluginDistribution!.extensionRoute).toBe(MODULE_EXTENSION_ROUTE);
    const onNotice = vi.fn();
    expect(pluginDistribution!.hubPluginSource({ onNotice })).toBeNull();
    vi.stubGlobal("caches", { open: async () => ({}) });
    vi.stubGlobal("navigator", Object.assign(Object.create(globalThis.navigator), { serviceWorker: { register: async () => ({}), ready: new Promise(() => {}), controller: null, addEventListener: () => {} }, locks: { request: async () => undefined }, storage: undefined }));
    const source = pluginDistribution!.hubPluginSource({ onNotice });
    expect(source).not.toBeNull();
    expect(typeof source!.ready).toBe("function");
    expect(typeof source!.installProgram).toBe("function");
  });

  it("names the four wasm engines the session loaders ask for", () => {
    const { wasmBindings } = createFrameworkOsHostPortsV1();
    expect(Object.keys(wasmBindings!).sort()).toEqual(["editor", "flow", "nodeGraph", "surface"]);
  });
});
