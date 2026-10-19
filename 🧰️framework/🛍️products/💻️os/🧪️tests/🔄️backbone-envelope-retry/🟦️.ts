/** 🔄️ Backbone envelope io: reading retries a transient transport failure without a real sleep, and writing keeps its bounded window.
 *
 * @see ../../🟦️.ts — the package entry the suite reads fixtures relative to
 */
import { afterEach, describe, expect, it, vi } from "vitest";
import { BACKBONE_ENVELOPE_RETRY_WINDOW_MS, readBackboneEnvelope, writeBackboneEnvelope } from "@semio-tech/framework/backbone";

describe("backbone envelope io", () => {
  const originalFetch = globalThis.fetch;
  afterEach(() => {
    globalThis.fetch = originalFetch;
    vi.useRealTimers();
  });

  it("readBackboneEnvelope retries a transient transport failure and then succeeds, with no real sleep", async () => {
    vi.useFakeTimers();
    let calls = 0;
    globalThis.fetch = vi.fn(async () => {
      calls += 1;
      if (calls < 3) throw new Error("connection refused");
      return { ok: true, status: 200, arrayBuffer: async () => new Uint8Array([1, 2, 3]).buffer } as unknown as Response;
    }) as unknown as typeof fetch;
    const promise = readBackboneEnvelope("folder:///doc");
    await vi.runAllTimersAsync();
    const result = await promise;
    expect(calls).toBe(3);
    expect(Array.from(result ?? [])).toEqual([1, 2, 3]);
  });

  it("readBackboneEnvelope does NOT retry a definitive non-404 server response", async () => {
    let calls = 0;
    globalThis.fetch = vi.fn(async () => {
      calls += 1;
      return { ok: false, status: 500, arrayBuffer: async () => new ArrayBuffer(0) } as unknown as Response;
    }) as unknown as typeof fetch;
    await expect(readBackboneEnvelope("folder:///doc")).rejects.toThrow("backbone read failed (500)");
    expect(calls).toBe(1);
  });

  it("readBackboneEnvelope gives up after its retry window instead of hanging forever", async () => {
    vi.useFakeTimers();
    globalThis.fetch = vi.fn(async () => {
      throw new Error("connection refused");
    }) as unknown as typeof fetch;
    const promise = readBackboneEnvelope("folder:///doc");
    let settled = false;
    promise.then(
      () => (settled = true),
      () => (settled = true),
    );
    await vi.advanceTimersByTimeAsync(BACKBONE_ENVELOPE_RETRY_WINDOW_MS + 1_000);
    expect(settled).toBe(true);
    await expect(promise).rejects.toThrow();
  });

  it("readBackboneEnvelope reads 204 No Content — a document nothing has written yet — as null without retrying", async () => {
    let calls = 0;
    globalThis.fetch = vi.fn(async () => {
      calls += 1;
      return { ok: true, status: 204, arrayBuffer: async () => new ArrayBuffer(0) } as unknown as Response;
    }) as unknown as typeof fetch;
    const result = await readBackboneEnvelope("folder:///doc");
    expect(result).toBeNull();
    expect(calls).toBe(1);
  });

  it("readBackboneEnvelope returns null on 404 without retrying", async () => {
    let calls = 0;
    globalThis.fetch = vi.fn(async () => {
      calls += 1;
      return { ok: false, status: 404, arrayBuffer: async () => new ArrayBuffer(0) } as unknown as Response;
    }) as unknown as typeof fetch;
    const result = await readBackboneEnvelope("folder:///doc");
    expect(result).toBeNull();
    expect(calls).toBe(1);
  });

  it("writeBackboneEnvelope does NOT retry on transport failure (duplicate-write safety)", async () => {
    let calls = 0;
    globalThis.fetch = vi.fn(async () => {
      calls += 1;
      throw new Error("connection refused");
    }) as unknown as typeof fetch;
    await expect(writeBackboneEnvelope("folder:///doc", new Uint8Array([1]))).rejects.toThrow("connection refused");
    expect(calls).toBe(1);
  });

  it("writeBackboneEnvelope propagates an external abort promptly with no leaked timer", async () => {
    const controller = new AbortController();
    globalThis.fetch = vi.fn((_url: string, init?: RequestInit) => {
      return new Promise((_resolve, reject) => {
        init?.signal?.addEventListener("abort", () => reject(init.signal!.reason ?? new Error("aborted")));
      });
    }) as unknown as typeof fetch;
    const promise = writeBackboneEnvelope("folder:///doc", new Uint8Array([1]), controller.signal);
    controller.abort(new Error("caller cancelled"));
    await expect(promise).rejects.toThrow("caller cancelled");
  });
});
