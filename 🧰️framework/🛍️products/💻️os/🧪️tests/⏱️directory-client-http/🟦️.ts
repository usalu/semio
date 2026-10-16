/** ⏱️ Directory client http: getJson and postJson timeout and abort.
 *
 * @see ../🧮️directory-client-helpers/🟦️.ts — the shared helpers
 * @see ../../🟦️.ts — the package entry the suite reads fixtures relative to
 */
import { describe, expect, it, vi } from "vitest";
import { DIRECTORY_HTTP_TIMEOUT_MS, DirectoryClient } from "@semio-tech/framework/directory-client";
import { source } from "../🧮️directory-client-helpers/🟦️.ts";

describe("DirectoryClient http (getJson/postJson timeout + abort)", () => {
  const originalFetch = globalThis.fetch;

  it("a hung server rejects at the timeout instead of hanging the caller forever", async () => {
    vi.useFakeTimers();
    try {
      globalThis.fetch = vi.fn((_url: string, init?: RequestInit) => {
        return new Promise((_resolve, reject) => {
          init?.signal?.addEventListener("abort", () => reject(init.signal!.reason ?? new Error("aborted")));
        });
      }) as unknown as typeof fetch;
      const client = new DirectoryClient("http://hub.test");
      const promise = client.me(); // 🪪️ the identity/boot-path call finding 1 is about.
      let settled = false;
      promise.then(
        () => (settled = true),
        () => (settled = true),
      );
      await vi.advanceTimersByTimeAsync(DIRECTORY_HTTP_TIMEOUT_MS + 1_000);
      expect(settled).toBe(true); // never hangs — this is what lets the boot path's own
      // catch-all ("hub unreachable, staying offline") actually run instead of awaiting forever.
      await expect(promise).rejects.toThrow();
    } finally {
      globalThis.fetch = originalFetch;
      vi.useRealTimers();
    }
  });

  it("an external abort cancels promptly, without ever waiting out the timeout", async () => {
    vi.useFakeTimers();
    try {
      globalThis.fetch = vi.fn((_url: string, init?: RequestInit) => {
        return new Promise((_resolve, reject) => {
          init?.signal?.addEventListener("abort", () => reject(init.signal!.reason ?? new Error("aborted")));
        });
      }) as unknown as typeof fetch;
      const client = new DirectoryClient("http://hub.test");
      const controller = new AbortController();
      const promise = client.spaces({ signal: controller.signal });
      controller.abort(new Error("caller cancelled"));
      // no `vi.advanceTimersByTime*` call at all: if this depended on the timeout timer firing,
      // fake timers would leave it pending forever and this await would hang the test.
      await expect(promise).rejects.toThrow("caller cancelled");
    } finally {
      globalThis.fetch = originalFetch;
      vi.useRealTimers();
    }
  });

  it("still resolves normally when the server answers promptly", async () => {
    globalThis.fetch = vi.fn(async () => ({ ok: true, status: 200, json: async () => [] })) as unknown as typeof fetch;
    try {
      const client = new DirectoryClient("http://hub.test");
      await expect(client.spaces()).resolves.toEqual([]);
    } finally {
      globalThis.fetch = originalFetch;
    }
  });

  it("pairs every shared workflow DSL fixture with a pack fixture", async () => {
    const { readdirSync, readFileSync } = await import("node:fs");
    const { fileURLToPath } = await import("node:url");
    const { dirname, join } = await import("node:path");
    const here = dirname(fileURLToPath(source.url));
    const fixturesDir = join(here, "🧪️tests/🧫️fixtures");
    const owners = readdirSync(fixturesDir, { withFileTypes: true }).filter((entry) => entry.isDirectory() && readdirSync(join(fixturesDir, entry.name)).some((name) => name === "🗣️.dsl" || name === "📦️.spk"));
    expect(owners.length).toBeGreaterThanOrEqual(5);
    for (const owner of owners) {
      expect(readFileSync(join(fixturesDir, owner.name, "🗣️.dsl"), "utf8").length).toBeGreaterThan(0);
      expect(readFileSync(join(fixturesDir, owner.name, "📦️.spk")).byteLength).toBeGreaterThan(0);
    }
  });
});
