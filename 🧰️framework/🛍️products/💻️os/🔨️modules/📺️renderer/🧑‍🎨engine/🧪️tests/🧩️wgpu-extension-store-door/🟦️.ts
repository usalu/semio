import { afterEach, describe, expect, it, vi } from "vitest";
import fixture from "../../../../🔌️plugin/🏪️store/📥️installation/🧫️fixtures/🔣️.json";
import { createWgpuPageHostIo } from "../../🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts";

const answer = async (request: unknown): Promise<Record<string, unknown>> => JSON.parse(await createWgpuPageHostIo()(JSON.stringify(request), null)) as Record<string, unknown>;

afterEach(() => vi.unstubAllGlobals());

describe("the wgpu extension store page door", () => {
  it("lists complete canonical store records and keeps their target identity", async () => {
    const fetch = vi.fn(async () => new Response(JSON.stringify([fixture]), { status: 200, headers: { "content-type": "application/json" } }));
    vi.stubGlobal("fetch", fetch);
    expect(await answer({ op: "extension-store-list" })).toEqual({ extensions: [fixture] });
    expect(fetch).toHaveBeenCalledWith("/🧩️extension-modules/install", expect.objectContaining({ credentials: "same-origin" }));
  });

  it("installs by URL and returns the store-authored record instead of caller metadata", async () => {
    const fetch = vi.fn(async () => new Response(JSON.stringify(fixture), { status: 200, headers: { "content-type": "application/json" } }));
    vi.stubGlobal("fetch", fetch);
    expect(await answer({ op: "extension-store-install-url", url: "https://example.test/fixture.sxt" })).toEqual({ extension: fixture });
    expect(fetch).toHaveBeenCalledWith("/🧩️extension-modules/install", expect.objectContaining({ method: "POST", body: JSON.stringify({ url: "https://example.test/fixture.sxt" }) }));
  });

  it("asks the page for a URL when the retained renderer supplies no text field and keeps cancellation local", async () => {
    const fetch = vi.fn(async () => new Response(JSON.stringify(fixture), { status: 200, headers: { "content-type": "application/json" } }));
    vi.stubGlobal("fetch", fetch);
    vi.stubGlobal("window", { prompt: vi.fn(() => "  https://example.test/prompted.sxt  ") });
    expect(await answer({ op: "extension-store-install-url", prompt: "Extension URL" })).toEqual({ extension: fixture });
    expect(fetch).toHaveBeenCalledWith("/🧩️extension-modules/install", expect.objectContaining({ body: JSON.stringify({ url: "https://example.test/prompted.sxt" }) }));
    vi.stubGlobal("window", { prompt: vi.fn(() => null) });
    expect(await answer({ op: "extension-store-install-url", prompt: "Extension URL" })).toEqual({ cancelled: true });
    expect(fetch).toHaveBeenCalledTimes(1);
  });

  it("uninstalls by public identity and reports a cancelled file pick without moving package bytes through Rust", async () => {
    const fetch = vi.fn(async () => new Response(JSON.stringify({ extensionId: fixture.extensionId }), { status: 200, headers: { "content-type": "application/json" } }));
    vi.stubGlobal("fetch", fetch);
    expect(await answer({ op: "extension-store-uninstall", extensionId: fixture.extensionId })).toEqual({ extensionId: fixture.extensionId });
    expect(fetch.mock.calls[0]?.[0]).toBe(`/🧩️extension-modules/install?extensionId=${encodeURIComponent(fixture.extensionId)}`);
    expect(await answer({ op: "extension-store-install-file" })).toEqual({ cancelled: true });
    expect(fetch).toHaveBeenCalledTimes(1);
  });

  it("refuses incomplete records at the page boundary", async () => {
    vi.stubGlobal("fetch", vi.fn(async () => new Response(JSON.stringify([{ extensionId: fixture.extensionId }]), { status: 200, headers: { "content-type": "application/json" } })));
    expect((await answer({ op: "extension-store-list" })).error).toContain("directoryName");
  });
});
