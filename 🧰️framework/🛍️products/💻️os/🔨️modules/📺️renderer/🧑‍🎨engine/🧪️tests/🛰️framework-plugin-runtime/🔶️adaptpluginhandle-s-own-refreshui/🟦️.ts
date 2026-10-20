/** 🔶️ The adaptpluginhandle s own refreshui tests of the framework plugin runtime suite. */
import { stubFetch } from "../../../../../../🧪️tests/🌐️fetch-stub/🟦️.ts";
import { expect, it } from "vitest";
import { adaptPluginHandle, fetchDescriptorManifest, resolveDescriptorBeforeRuntime } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { exchangeStyleChannel } from "../🟣️shared/🟦️.ts";

/** 🔷 Registers the adaptpluginhandle s own refreshui tests of the framework plugin runtime suite. */
export function registerAdaptpluginhandleSOwnRefreshuiTests(): void {
  it("adaptPluginHandle's own refreshUi is an honest empty result — window-body refresh now lives in loadPluginModule's ActivationRegistry/ShardClient turn loop, which a bare no-command handle has no access to", async () => {
    const { encodePackValue } = await import("@semio-tech/framework-os");
    const fakeHandle = {
      manifest: { pluginId: "mock-refresh", label: "Mock Refresh", version: "0", apps: [], programs: [], examples: [] } as unknown as import("@semio-tech/framework").PluginManifest,
      createApp: async () => 7,
      destroyApp: async () => {},
      takeSegmentedDownloadChunk: async () => undefined,
      ...exchangeStyleChannel(() => {
        throw new Error("adaptPluginHandle.refreshUi must not call enqueue() — there is no AppCommand for it anymore");
      }),
      dispose: () => {},
    };
    const handle = await adaptPluginHandle("mock-refresh", { handle: fakeHandle, release: () => {} } as unknown as Parameters<typeof adaptPluginHandle>[1]);
    const instanceId = await handle.createApp("main");
    await expect(handle.refreshUi(instanceId, { viewState: {}, windows: [{ key: "overview", bodyKey: "overview" }] })).resolves.toEqual({});
  });
  it("fetchDescriptorManifest refuses a missing descriptor and surfaces a published one", async () => {
    const originalFetch = globalThis.fetch;
    try {
      globalThis.fetch = stubFetch(async () => new Response(null, { status: 404 }));
      await expect(fetchDescriptorManifest("mock", "/🔌️plugin-modules/mock/index.js")).rejects.toThrow("plugin.descriptor-unavailable");

      globalThis.fetch = stubFetch(async () => new Response(JSON.stringify({ manifest: { pluginId: "mock", label: "Mock", version: "1.0.0", apps: [{ id: "main" }] } }), { headers: { "content-type": "application/json" } }));
      const real = await fetchDescriptorManifest("mock", "/🔌️plugin-modules/mock/index.js");
      expect(real).toEqual({ pluginId: "mock", label: "Mock", version: "1.0.0", apps: [{ id: "main" }] });
    } finally {
      globalThis.fetch = originalFetch;
    }
  });
  it("resolves the descriptor before initializing the shard runtime", async () => {
    const order: string[] = [];
    const resolved = await resolveDescriptorBeforeRuntime(
      async () => {
        order.push("descriptor-start");
        await Promise.resolve();
        order.push("descriptor-ready");
        return "manifest";
      },
      () => {
        order.push("runtime");
        return "shards";
      },
    );
    expect(order).toEqual(["descriptor-start", "descriptor-ready", "runtime"]);
    expect(resolved).toEqual({ manifest: "manifest", runtime: "shards" });
  });
}
