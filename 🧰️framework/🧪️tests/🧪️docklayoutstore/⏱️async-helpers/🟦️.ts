/** ⏱️ The async helpers of the framework in-source suites: jittered retry backoff, latest wins, fetch with timeout and wait for event. */
import type { EventSubscribe, FetchTimeoutResponse } from "@semio-tech/framework";

export async function registerAsyncHelperTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: typeof import("@semio-tech/framework")): Promise<void> {
  const { fetchWithTimeout, latestWins, retryWithJitteredBackoff, waitForEvent } = dependencies;
  const { describe, expect, it, vi } = vitest;

  //#region 🔖️RetryWithJitteredBackoffTests
  describe("retryWithJitteredBackoff", () => {
    it("keeps every backoff delay within [minMs, maxMs] and varies it across attempts", async () => {
      vi.useFakeTimers();
      try {
        const setTimeoutSpy = vi.spyOn(globalThis, "setTimeout");
        let calls = 0;
        const fn = vi.fn(async () => {
          calls += 1;
          if (calls < 6) throw new Error("retry me");
          return "ok";
        });
        const resultPromise = retryWithJitteredBackoff(fn, { minMs: 10, maxMs: 100 });
        for (let i = 0; i < 5; i++) {
          await vi.advanceTimersByTimeAsync(100);
        }
        await expect(resultPromise).resolves.toBe("ok");
        expect(calls).toBe(6);
        const delays = setTimeoutSpy.mock.calls.map(([, ms]) => Number(ms));
        expect(delays.length).toBe(5);
        for (const delay of delays) {
          expect(delay).toBeGreaterThanOrEqual(10);
          expect(delay).toBeLessThanOrEqual(100);
        }
        expect(new Set(delays).size).toBeGreaterThan(1);
      } finally {
        vi.useRealTimers();
      }
    });

    it("rejects promptly on abort mid-wait and makes no further attempt", async () => {
      vi.useFakeTimers();
      try {
        const controller = new AbortController();
        let calls = 0;
        const fn = vi.fn(async () => {
          calls += 1;
          throw new Error("always fails");
        });
        const promise = retryWithJitteredBackoff(fn, { minMs: 50, maxMs: 50, signal: controller.signal });
        await vi.advanceTimersByTimeAsync(0);
        expect(calls).toBe(1);
        controller.abort(new Error("stop"));
        await expect(promise).rejects.toThrow("stop");
        expect(calls).toBe(1);
      } finally {
        vi.useRealTimers();
      }
    });

    it("throws the last failure once maxAttempts attempts failed, waiting between them and not after the last", async () => {
      vi.useFakeTimers();
      try {
        const setTimeoutSpy = vi.spyOn(globalThis, "setTimeout");
        let calls = 0;
        const fn = vi.fn(async () => {
          calls += 1;
          throw new Error(`failure ${calls}`);
        });
        const promise = retryWithJitteredBackoff(fn, { minMs: 10, maxMs: 100, maxAttempts: 3 });
        const assertion = expect(promise).rejects.toThrow("failure 3");
        await vi.advanceTimersByTimeAsync(1000);
        await assertion;
        expect(calls).toBe(3);
        expect(setTimeoutSpy.mock.calls).toHaveLength(2);
      } finally {
        vi.useRealTimers();
      }
    });

    it("succeeds within maxAttempts and never counts the successful attempt as a failure", async () => {
      let calls = 0;
      const fn = vi.fn(async () => {
        calls += 1;
        if (calls < 3) throw new Error("retry me");
        return "ok";
      });
      await expect(retryWithJitteredBackoff(fn, { minMs: 1, maxMs: 2, maxAttempts: 3 })).resolves.toBe("ok");
      expect(calls).toBe(3);
    });

    it("throws immediately for an already-aborted signal without calling fn", async () => {
      const controller = new AbortController();
      controller.abort(new Error("pre-aborted"));
      const fn = vi.fn(async () => "unreachable");
      await expect(retryWithJitteredBackoff(fn, { minMs: 10, maxMs: 10, signal: controller.signal })).rejects.toThrow("pre-aborted");
      expect(fn).not.toHaveBeenCalled();
    });
  });

  //#endregion 🔖️RetryWithJitteredBackoffTests
  //#region 🔖️LatestWinsTests
  describe("latestWins", () => {
    it("collapses N concurrent calls during an in-flight run into exactly one trailing run, and every caller sees the latest result", async () => {
      let calls = 0;
      const resolvers: Array<(value: number) => void> = [];
      const run = vi.fn(() => {
        calls += 1;
        const callIndex = calls;
        return new Promise<number>((resolve) => resolvers.push(() => resolve(callIndex)));
      });
      const trigger = latestWins(run);

      const first = trigger();
      expect(calls).toBe(1);
      const concurrentA = trigger();
      const concurrentB = trigger();
      const concurrentC = trigger();
      expect(calls).toBe(1);

      resolvers[0]!(0);
      await new Promise((resolve) => setTimeout(resolve, 0));
      expect(calls).toBe(2);

      resolvers[1]!(0);
      await expect(first).resolves.toBe(1);
      await expect(concurrentA).resolves.toBe(2);
      await expect(concurrentB).resolves.toBe(2);
      await expect(concurrentC).resolves.toBe(2);
    });

    it("starts a fresh run when called again after everything has settled", async () => {
      let n = 0;
      const trigger = latestWins(async () => {
        n += 1;
        return n;
      });
      await expect(trigger()).resolves.toBe(1);
      await expect(trigger()).resolves.toBe(2);
    });
  });

  //#endregion 🔖️LatestWinsTests
  //#region 🔖️FetchWithTimeoutTests
  describe("fetchWithTimeout", () => {
    it("aborts the underlying fetch when the timeout elapses first", async () => {
      vi.useFakeTimers();
      try {
        const fetchMock = vi.fn(
          (_url: string, init?: RequestInit) =>
            new Promise((_resolve, reject) => {
              init?.signal?.addEventListener("abort", () => reject(new Error("aborted-by-fetch")));
            }),
        );
        vi.stubGlobal("fetch", fetchMock);
        const promise = fetchWithTimeout("https://example.test", undefined, { timeoutMs: 50 });
        const assertion = expect(promise).rejects.toThrow();
        await vi.advanceTimersByTimeAsync(50);
        await assertion;
      } finally {
        vi.unstubAllGlobals();
        vi.useRealTimers();
      }
    });

    it("aborts the underlying fetch when the external signal aborts", async () => {
      const controller = new AbortController();
      const fetchMock = vi.fn(
        (_url: string, init?: RequestInit) =>
          new Promise((_resolve, reject) => {
            init?.signal?.addEventListener("abort", () => reject(new Error("aborted-by-fetch")));
          }),
      );
      vi.stubGlobal("fetch", fetchMock);
      try {
        const promise = fetchWithTimeout("https://example.test", undefined, { timeoutMs: 10_000, signal: controller.signal });
        controller.abort(new Error("caller cancelled"));
        await expect(promise).rejects.toThrow();
      } finally {
        vi.unstubAllGlobals();
      }
    });

    it("returns the very response and clears the timer and the abort listener once its body has been read", async () => {
      vi.useFakeTimers();
      try {
        const clearTimeoutSpy = vi.spyOn(globalThis, "clearTimeout");
        const controller = new AbortController();
        const removeEventListenerSpy = vi.spyOn(controller.signal, "removeEventListener");
        const response: FetchTimeoutResponse = { ok: true, status: 200, statusText: "OK", headers: { get: () => null }, json: async () => ({}), text: async () => "body" };
        vi.stubGlobal(
          "fetch",
          vi.fn(async () => response),
        );
        const result = await fetchWithTimeout("https://example.test", undefined, { timeoutMs: 1000, signal: controller.signal });
        expect(result).toBe(response);
        expect(vi.getTimerCount()).toBe(1);
        await expect(result.text()).resolves.toBe("body");
        expect(vi.getTimerCount()).toBe(0);
        expect(clearTimeoutSpy).toHaveBeenCalled();
        expect(removeEventListenerSpy).toHaveBeenCalledWith("abort", expect.any(Function));
      } finally {
        vi.unstubAllGlobals();
        vi.useRealTimers();
      }
    });

    it("ends a body that stalls after the headers with the timeout error instead of hanging", async () => {
      vi.useFakeTimers();
      try {
        const response = { ok: true, status: 200, statusText: "OK", headers: { get: () => null }, json: () => new Promise<unknown>(() => undefined), text: () => new Promise<string>(() => undefined) };
        vi.stubGlobal(
          "fetch",
          vi.fn(async () => response),
        );
        const result = await fetchWithTimeout("https://example.test", undefined, { timeoutMs: 50 });
        const text = expect(result.text()).rejects.toThrow("timed out after 50ms");
        await vi.advanceTimersByTimeAsync(50);
        await text;
        await expect(result.json()).rejects.toThrow("timed out after 50ms");
      } finally {
        vi.unstubAllGlobals();
        vi.useRealTimers();
      }
    });

    it("ends a body that stalls after the headers when the external signal aborts, and aborts the request signal", async () => {
      const controller = new AbortController();
      let requestSignal: AbortSignal | undefined;
      const response = { ok: true, status: 200, statusText: "OK", headers: { get: () => null }, json: () => new Promise<unknown>(() => undefined), text: () => new Promise<string>(() => undefined) };
      vi.stubGlobal(
        "fetch",
        vi.fn(async (_url: string, init?: RequestInit) => {
          requestSignal = init?.signal ?? undefined;
          return response;
        }),
      );
      try {
        const result = await fetchWithTimeout("https://example.test", undefined, { timeoutMs: 10_000, signal: controller.signal });
        const text = expect(result.text()).rejects.toThrow("caller cancelled");
        controller.abort(new Error("caller cancelled"));
        await text;
        expect(requestSignal?.aborted).toBe(true);
      } finally {
        vi.unstubAllGlobals();
      }
    });

    it("keeps every other member of the response, binary readers included, and reads a real Response through the deadline", async () => {
      const real = new Response(new Uint8Array([1, 2, 3]), { status: 201, headers: { "x-kind": "bytes" } });
      vi.stubGlobal(
        "fetch",
        vi.fn(async () => real),
      );
      try {
        const result = await fetchWithTimeout("https://example.test", undefined, { timeoutMs: 1000 });
        expect(result.status).toBe(201);
        expect(result.headers.get("x-kind")).toBe("bytes");
        expect(new Uint8Array(await (result as unknown as Response).arrayBuffer())).toEqual(new Uint8Array([1, 2, 3]));
      } finally {
        vi.unstubAllGlobals();
      }
    });
  });

  //#endregion 🔖️FetchWithTimeoutTests
  //#region 🔖️WaitForEventTests
  describe("waitForEvent", () => {
    it("resolves with the first delivered value and unsubscribes", async () => {
      let handler: ((value: number) => void) | undefined;
      let unsubscribed = false;
      const subscribe: EventSubscribe<number> = (h) => {
        handler = h;
        return () => {
          unsubscribed = true;
        };
      };
      const promise = waitForEvent(subscribe);
      expect(unsubscribed).toBe(false);
      handler!(42);
      await expect(promise).resolves.toBe(42);
      expect(unsubscribed).toBe(true);
    });

    it("rejects on abort and unsubscribes without ever having received a value", async () => {
      let unsubscribed = false;
      const subscribe: EventSubscribe<number> = () => () => {
        unsubscribed = true;
      };
      const controller = new AbortController();
      const promise = waitForEvent(subscribe, { signal: controller.signal });
      controller.abort(new Error("cancelled"));
      await expect(promise).rejects.toThrow("cancelled");
      expect(unsubscribed).toBe(true);
    });

    it("rejects immediately for an already-aborted signal, never subscribing", async () => {
      const controller = new AbortController();
      controller.abort(new Error("pre-aborted"));
      let subscribed = false;
      const subscribe: EventSubscribe<number> = () => {
        subscribed = true;
        return () => {};
      };
      await expect(waitForEvent(subscribe, { signal: controller.signal })).rejects.toThrow("pre-aborted");
      expect(subscribed).toBe(false);
    });
  });
  //#endregion 🔖️WaitForEventTests
}
