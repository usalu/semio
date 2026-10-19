/** 🗄️ Renderer module cache: the server asserted artifact identity and the fail-soft cache. */
import { describe, expect, it } from "vitest";
import { evictCachedRendererModule, readCachedRendererModule, rendererArtifactTag, writeCachedRendererModule } from "@semio-tech/framework-renderer-wgpu/wasm-module-cache";
import { stubFetch } from "../../../../../../🧪️tests/🌐️fetch-stub/🟦️.ts";

describe("wgpu renderer module cache", () => {
  it("reads the artifact's server-asserted identity and disables itself when the server offers none", async () => {
    const original = globalThis.fetch;
    try {
      globalThis.fetch = stubFetch(async () => new Response(null, { status: 200, headers: { etag: 'W/"7ab1"' } }));
      expect(await rendererArtifactTag("http://host/renderer_bg.wasm")).toBe('etag:W/"7ab1"');
      globalThis.fetch = stubFetch(async () => new Response(null, { status: 200, headers: { "last-modified": "Thu, 10 Sep 2026 09:29:09 GMT", "content-length": "76048601" } }));
      expect(await rendererArtifactTag("http://host/renderer_bg.wasm")).toBe("mtime:Thu, 10 Sep 2026 09:29:09 GMT:76048601");
      globalThis.fetch = stubFetch(async () => new Response(null, { status: 200 }));
      expect(await rendererArtifactTag("http://host/renderer_bg.wasm")).toBe("");
      globalThis.fetch = stubFetch(async () => new Response(null, { status: 404 }));
      expect(await rendererArtifactTag("http://host/renderer_bg.wasm")).toBe("");
      globalThis.fetch = stubFetch(async () => { throw new Error("offline"); });
      expect(await rendererArtifactTag("http://host/renderer_bg.wasm")).toBe("");
    } finally {
      globalThis.fetch = original;
    }
  });

  it("fails soft to no cache wherever IndexedDB or a taggable artifact is unavailable", async () => {
    expect((globalThis as { indexedDB?: unknown }).indexedDB).toBeUndefined();
    expect(await readCachedRendererModule("http://host/renderer_bg.wasm", "")).toBeUndefined();
    expect(await readCachedRendererModule("http://host/renderer_bg.wasm", "etag:1")).toBeUndefined();
    expect(await writeCachedRendererModule("http://host/renderer_bg.wasm", "", {} as WebAssembly.Module, 1, 0)).toBe(false);
    expect(await writeCachedRendererModule("http://host/renderer_bg.wasm", "etag:1", {} as WebAssembly.Module, 1, 0)).toBe(false);
    await expect(evictCachedRendererModule("http://host/renderer_bg.wasm")).resolves.toBeUndefined();
  });
});
