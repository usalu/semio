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
      const promise = client.me();
      let settled = false;
      promise.then(
        () => (settled = true),
        () => (settled = true),
      );
      await vi.advanceTimersByTimeAsync(DIRECTORY_HTTP_TIMEOUT_MS + 1_000);
      expect(settled).toBe(true);
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

  it("keeps every shared workflow fixture a complete graph case with its dirty set and expected deliveries", async () => {
    const { readdirSync, readFileSync, existsSync } = await import("node:fs");
    const { fileURLToPath } = await import("node:url");
    const { dirname, join } = await import("node:path");
    const here = dirname(fileURLToPath(source.url));
    const fixturesDir = join(here, "🧪️tests/🧫️fixtures");
    const cases = readdirSync(fixturesDir, { withFileTypes: true })
      .filter((entry) => entry.isDirectory() && existsSync(join(fixturesDir, entry.name, "🔣️.json")))
      .map((entry) => ({ owner: entry.name, body: JSON.parse(readFileSync(join(fixturesDir, entry.name, "🔣️.json"), "utf8")) as { name?: string; graph?: { nodes?: unknown[] }; dirtyNodeIds?: unknown[]; expectedDeliveries?: unknown[] } }))
      .filter(({ body }) => body.graph !== undefined);
    expect(cases.length).toBeGreaterThanOrEqual(5);
    for (const { owner, body } of cases) {
      expect(owner.endsWith(body.name ?? "\0")).toBe(true);
      expect(body.graph?.nodes?.length).toBeGreaterThan(0);
      expect(Array.isArray(body.dirtyNodeIds)).toBe(true);
      expect(Array.isArray(body.expectedDeliveries)).toBe(true);
    }
  });
});
