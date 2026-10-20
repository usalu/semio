/** 🔸️ The keeps installed service status tests of the DirectoryClient.stream suite. */
import { expect, it, vi } from "vitest";
import { BACKBONE_WORKER_WIRE_MAGIC, decodeBackboneWorkerResponse, encodeBackboneWorkerResponse } from "@semio-tech/framework/backbone";
import { HUB_RECONNECT_MAX_MS, HUB_RECONNECT_MIN_MS } from "@semio-tech/framework/directory-client";
import { decodePackValue, encodePackValue } from "@semio-tech/framework/packs";
import { FakeDirectoryWebSocket, sampleDirectoryEvent, testDirectoryClient, testSocketGrantReceipt } from "../🟢️shared/🟦️.ts";
import type { BackboneWorkerResponse, DirectoryStreamMessage } from "../🟢️shared/🟦️.ts";

/** 🔹 Registers the keeps installed service status tests of the DirectoryClient.stream suite. */
export function registerKeepsInstalledServiceStatusTests(): void {
  it("keeps installed service status scope and opaque payload exact across the private worker wire", () => {
    const scope = { spaceId: "space-a", documentId: "same-document" };
    const response: Extract<BackboneWorkerResponse, {kind: "inference-port-status"}> = {
      kind: "inference-port-status", operationEpoch: 7, scope,
      status: { owner: "neutral", serviceId: "neutral.service", payload: { phase: "ready", progress: { completed: 4, total: 4 }, result: { token: "opaque-result" } } },
    };
    expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response))).toEqual(response);
    const decoded = decodePackValue(encodeBackboneWorkerResponse(response).subarray(1)) as Record<string, unknown>;
    const badScope = encodePackValue({ ...decoded, scope: { spaceId: "space-b", documentId: "same-document", requestedSurface: "forbidden" } });
    expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...badScope]))).toThrow("invalid inference scope");
    const badStatus = encodePackValue({ ...decoded, status: { ...response.status, credential: "forged" } });
    expect(() => decodeBackboneWorkerResponse(new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...badStatus]))).toThrow("installed-service.invalid");
  });
  it("binds one document scope and treats close 4401 as terminal without reacquiring", async () => {
    vi.useFakeTimers();
    try {
      FakeDirectoryWebSocket.instances = [];
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
      const revoked = vi.fn();
      const handle = testDirectoryClient().streamScoped({ spaceId: "space/a", documentId: "document b" }, 4, () => {}, revoked);
      await Promise.resolve();
      const socket = FakeDirectoryWebSocket.instances[0]!;
      expect(socket.url).toBe("ws://hub.test/directory/spaces/space%2Fa/documents/document%20b/socket/v1?since=4");
      socket.triggerClose(4401);
      await Promise.resolve();
      await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MAX_MS * 2);
      expect(revoked).toHaveBeenCalledTimes(1);
      expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
      handle.close();
    } finally {
      vi.useRealTimers();
    }
  });
  it("replays then goes live with no gap and no duplicate", async () => {
    FakeDirectoryWebSocket.instances = [];
    (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
    const received: DirectoryStreamMessage[] = [];
    const client = testDirectoryClient();
    const handle = client.stream(0, (message) => received.push(message));
    await Promise.resolve();
    const socket = FakeDirectoryWebSocket.instances[0]!;
    expect(socket.url).toBe("ws://hub.test/directory/socket/v1?since=0");
    expect(socket.protocols).toEqual(["semio.socket.v1", testSocketGrantReceipt.grant]);
    socket.triggerOpen();
    socket.triggerMessage({ kind: "event", event: sampleDirectoryEvent(1) });
    socket.triggerMessage({ kind: "event", event: sampleDirectoryEvent(2) });
    socket.triggerMessage({ kind: "heartbeat", headSeq: 2 });
    expect(received.map((message) => message.kind)).toEqual(["event", "event", "heartbeat"]);
    expect(received).toHaveLength(3);
    handle.close();
  });
  it("reconnects an acknowledged stream only from the last Home-committed frontier", async () => {
    vi.useFakeTimers();
    const randomSpy = vi.spyOn(Math, "random").mockReturnValue(0);
    try {
      FakeDirectoryWebSocket.instances = [];
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
      const handle = testDirectoryClient().streamAcknowledged(3, () => {});
      await Promise.resolve();
      const first = FakeDirectoryWebSocket.instances[0]!;
      first.triggerMessage({ kind: "event", event: sampleDirectoryEvent(99) });
      first.triggerMessage({ kind: "heartbeat", headSeq: 101 });
      first.triggerClose();
      await Promise.resolve();
      await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MIN_MS);
      const second = FakeDirectoryWebSocket.instances[1]!;
      expect(second.url).toBe("ws://hub.test/directory/socket/v1?since=3");
      handle.acknowledge(5);
      expect(() => handle.acknowledge(4)).toThrow("invalid acknowledged frontier");
      second.triggerClose();
      await Promise.resolve();
      await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MIN_MS);
      expect(FakeDirectoryWebSocket.instances[2]!.url).toBe("ws://hub.test/directory/socket/v1?since=5");
      handle.close();
    } finally {
      randomSpy.mockRestore();
      vi.useRealTimers();
    }
  });
  it("reconnects resuming from the last seen seq, with jittered backoff within bounds", async () => {
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
      first.triggerMessage({ kind: "event", event: sampleDirectoryEvent(7) });
      first.triggerClose();
      await Promise.resolve();

      await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MIN_MS - 1);
      expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
      await vi.advanceTimersByTimeAsync(1);
      expect(FakeDirectoryWebSocket.instances).toHaveLength(2);
      const second = FakeDirectoryWebSocket.instances[1]!;
      expect(second.url).toBe("ws://hub.test/directory/socket/v1?since=7");

      randomSpy.mockReturnValue(1);
      second.triggerClose();
      await Promise.resolve();

      const attempt2Cap = Math.min(HUB_RECONNECT_MAX_MS, HUB_RECONNECT_MIN_MS * 2 ** 2);
      await vi.advanceTimersByTimeAsync(attempt2Cap - 1);
      expect(FakeDirectoryWebSocket.instances).toHaveLength(2);
      await vi.advanceTimersByTimeAsync(1);
      expect(FakeDirectoryWebSocket.instances).toHaveLength(3);

      handle.close();
    } finally {
      randomSpy.mockRestore();
      vi.useRealTimers();
    }
  });
  it("never throws into the caller on a malformed frame", async () => {
    FakeDirectoryWebSocket.instances = [];
    (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
    const received: DirectoryStreamMessage[] = [];
    const client = testDirectoryClient();
    const handle = client.stream(0, (message) => received.push(message));
    await Promise.resolve();
    const socket = FakeDirectoryWebSocket.instances[0]!;
    expect(() => socket.onmessage?.({ data: "not json" })).not.toThrow();
    socket.triggerMessage({ kind: "heartbeat", headSeq: 0 });
    expect(received).toHaveLength(1);
    handle.close();
  });
  it("close() stops the reconnect loop — no further socket is ever opened", async () => {
    vi.useFakeTimers();
    try {
      FakeDirectoryWebSocket.instances = [];
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
      const client = testDirectoryClient();
      const handle = client.stream(0, () => {});
      await Promise.resolve();
      const first = FakeDirectoryWebSocket.instances[0]!;
      handle.close();
      first.triggerClose();
      vi.advanceTimersByTime(HUB_RECONNECT_MAX_MS * 2);
      expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
    } finally {
      vi.useRealTimers();
    }
  });
}
