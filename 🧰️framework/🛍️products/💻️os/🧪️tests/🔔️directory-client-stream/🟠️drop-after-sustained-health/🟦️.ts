/** 🟠️ The drop after sustained health tests of the DirectoryClient.stream suite. */
import { expect, it, vi } from "vitest";
import { HUB_HEALTHY_RESET_MS, HUB_RECONNECT_MAX_MS, HUB_RECONNECT_MIN_MS } from "@semio-tech/framework/directory-client";
import { FakeDirectoryWebSocket, testDirectoryClient } from "../🟢️shared/🟦️.ts";

/** 🟡 Registers the drop after sustained health tests of the DirectoryClient.stream suite. */
export function registerDropAfterSustainedHealthTests(): void {
  it("(a) a drop after sustained health resets the backoff — reconnects near HUB_RECONNECT_MIN_MS, not at an escalated delay", async () => {
    vi.useFakeTimers();
    const randomSpy = vi.spyOn(Math, "random");
    try {
      FakeDirectoryWebSocket.instances = [];
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
      randomSpy.mockReturnValue(0);
      const client = testDirectoryClient();
      const handle = client.stream(0, () => {});
      await Promise.resolve();
      const first = FakeDirectoryWebSocket.instances[0]!;
      first.triggerOpen();

      await vi.advanceTimersByTimeAsync(HUB_HEALTHY_RESET_MS);
      first.triggerClose();
      await Promise.resolve();

      await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MIN_MS - 1);
      expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
      await vi.advanceTimersByTimeAsync(1);
      expect(FakeDirectoryWebSocket.instances).toHaveLength(2);

      handle.close();
    } finally {
      randomSpy.mockRestore();
      vi.useRealTimers();
    }
  });
  it("(b) rapid accept-then-drop cycling never crosses the health threshold — backoff keeps escalating, never resets", async () => {
    vi.useFakeTimers();
    const randomSpy = vi.spyOn(Math, "random");
    try {
      FakeDirectoryWebSocket.instances = [];
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
      randomSpy.mockReturnValue(1);
      const client = testDirectoryClient();
      const handle = client.stream(0, () => {});
      await Promise.resolve();

      let instanceCount = 1;
      for (let attempt = 1; attempt <= 3; attempt++) {
        const socket = FakeDirectoryWebSocket.instances[instanceCount - 1]!;
        socket.triggerOpen();
        socket.triggerClose();
        await Promise.resolve();
        const cap = Math.min(HUB_RECONNECT_MAX_MS, HUB_RECONNECT_MIN_MS * 2 ** attempt);
        await vi.advanceTimersByTimeAsync(cap - 1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(instanceCount);
        await vi.advanceTimersByTimeAsync(1);
        instanceCount += 1;
        expect(FakeDirectoryWebSocket.instances).toHaveLength(instanceCount);
      }

      handle.close();
    } finally {
      randomSpy.mockRestore();
      vi.useRealTimers();
    }
  });
  it("(c) close() during a healthy-but-not-yet-reset connection cancels promptly, clears the health timer, and never reconnects", async () => {
    vi.useFakeTimers();
    try {
      FakeDirectoryWebSocket.instances = [];
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
      const client = testDirectoryClient();
      const handle = client.stream(0, () => {});
      await Promise.resolve();
      const first = FakeDirectoryWebSocket.instances[0]!;
      first.triggerOpen();
      await vi.advanceTimersByTimeAsync(HUB_HEALTHY_RESET_MS / 2);
      handle.close();
      first.triggerClose();
      await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MAX_MS * 2);
      expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
      expect(vi.getTimerCount()).toBe(0);
    } finally {
      vi.useRealTimers();
    }
  });
}
