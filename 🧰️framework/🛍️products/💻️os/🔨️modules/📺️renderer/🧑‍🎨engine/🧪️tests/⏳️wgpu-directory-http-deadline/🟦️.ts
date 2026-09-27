import { afterEach, describe, expect, it, vi } from "vitest";
import law from "../../🧫️fixtures/⏳️wgpu-directory-http-deadline/🔣️.json";
import { WGPU_DIRECTORY_HTTP_TIMEOUT_MS, createWgpuPageHostIo } from "../../🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts";

afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllGlobals();
});

describe("wgpu directory HTTP deadline", () => {
  it("aborts an unresolved page fetch before it can retain the renderer interaction owner", async () => {
    vi.useFakeTimers();
    let signal: AbortSignal | undefined;
    vi.stubGlobal(
      "fetch",
      vi.fn((_url: string, init?: RequestInit) => {
        signal = init?.signal ?? undefined;
        return new Promise<Response>((_resolve, reject) => signal?.addEventListener("abort", () => reject(signal?.reason ?? new Error("aborted")), { once: true }));
      }),
    );
    const request = { ...law.request, timeoutMs: law.timeoutMs };
    const pending = createWgpuPageHostIo()(JSON.stringify(request), null);
    expect(WGPU_DIRECTORY_HTTP_TIMEOUT_MS).toBe(law.timeoutMs);
    expect(signal?.aborted).toBe(false);
    await vi.advanceTimersByTimeAsync(law.timeoutMs);
    const answer = JSON.parse(await pending) as { error?: string };
    expect(signal?.aborted).toBe(law.expected.aborted);
    expect(answer.error?.toLowerCase()).toContain(law.expected.errorIncludes);
  });
});
