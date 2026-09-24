import { afterEach, describe, expect, it } from "vitest";
import { WGPU_SOCKET_POLL_MAX_MESSAGES, WGPU_SOCKET_QUEUE_MAX_MESSAGES, WGPU_SOCKET_SEND_MAX_BYTES, createWgpuPageHostIo } from "../../🎯️targets/🧊️wgpu/🚪️host-io/🟦️.ts";

/** @emoji 🔌️ The PAGE half of packet W15e's DUPLEX socket door, tested where it runs. The Rust half's
 * own laws (wire codec, the bounded `SocketLane`) live beside it in this case's `🦀️.rs`; what only this suite can prove is that the
 * servicer really constructs a `WebSocket` with the EXACT ordered subprotocol list, really pages what
 * the peer sent instead of handing over the whole queue, and really REPORTS what its bounded queue
 * dropped instead of losing it silently.
 *
 * 🩸️ Why the door exists: the wgpu shell lives in the frame Worker and its two socket consumers (the
 * hub directory stream and the MCP agent bridge) reach their trait seams SYNCHRONOUSLY, which a wasm
 * isolate cannot serve from a promise. The page owns the socket; the shell pages it. */
type SocketEvents = { onopen?: () => void; onmessage?: (event: { data: unknown }) => void; onerror?: () => void; onclose?: (event: { code: number }) => void };

/** 🔌️ A `WebSocket` test double that records what it was constructed with and what was written to
 * it, and lets a case drive the peer's side by hand. */
class FakeSocket implements SocketEvents {
  static constructed: FakeSocket[] = [];
  static refuseConstruction = false;

  readonly sent: unknown[] = [];
  binaryType = "";
  closed = false;
  onopen?: () => void;
  onmessage?: (event: { data: unknown }) => void;
  onerror?: () => void;
  onclose?: (event: { code: number }) => void;

  constructor(
    readonly url: string,
    readonly protocols?: string[],
  ) {
    if (FakeSocket.refuseConstruction) throw new Error("refused by the engine");
    FakeSocket.constructed.push(this);
  }

  send(payload: unknown): void {
    this.sent.push(payload);
  }

  close(): void {
    this.closed = true;
  }
}

function installFakeSocket(): void {
  FakeSocket.constructed = [];
  FakeSocket.refuseConstruction = false;
  Object.defineProperty(globalThis, "WebSocket", { configurable: true, value: FakeSocket, writable: true });
}

type SocketAnswer = { state?: string; messages?: readonly ({ text: string } | { binary: string })[]; more?: boolean; dropped?: number; closeCode?: number; error?: string };

function door(): (request: unknown) => Promise<SocketAnswer> {
  installFakeSocket();
  const io = createWgpuPageHostIo();
  return async (request) => JSON.parse(await io(JSON.stringify(request), null)) as SocketAnswer;
}

afterEach(() => {
  Reflect.deleteProperty(globalThis, "WebSocket");
});

describe("wgpu socket door — dialling", () => {
  it("constructs the socket with the url and the EXACT ordered subprotocol list", async () => {
    const call = door();
    const answer = await call({ op: "socket", protocols: ["semio.mcp.bridge.v1", "session.v1.selector.proof"], socketId: 1, url: "ws://127.0.0.1:6300/bridge", verb: "open" });
    expect(answer.state).toBe("connecting");
    const socket = FakeSocket.constructed[0];
    expect(socket?.url).toBe("ws://127.0.0.1:6300/bridge");
    expect(socket?.protocols).toEqual(["semio.mcp.bridge.v1", "session.v1.selector.proof"]);
    expect(socket?.binaryType).toBe("arraybuffer");
  });

  it("answers a construction refusal as an error rather than a silent connecting socket", async () => {
    const call = door();
    FakeSocket.refuseConstruction = true;
    expect((await call({ op: "socket", socketId: 1, url: "ws://nowhere", verb: "open" })).error).toContain("refused by the engine");
  });

  it("refuses a second open on the same socket id", async () => {
    const call = door();
    await call({ op: "socket", socketId: 1, url: "ws://127.0.0.1:6300/bridge", verb: "open" });
    expect((await call({ op: "socket", socketId: 1, url: "ws://127.0.0.1:6300/bridge", verb: "open" })).error).toContain("already open");
  });

  it("refuses every verb on a socket it does not hold", async () => {
    const call = door();
    expect((await call({ op: "socket", socketId: 9, verb: "poll" })).error).toContain("unknown socket 9");
    expect((await call({ op: "socket", socketId: 9, text: "hi", verb: "send" })).error).toContain("unknown socket 9");
  });
});

describe("wgpu socket door — writing", () => {
  it("refuses a send before the socket opened, and writes it afterwards", async () => {
    const call = door();
    await call({ op: "socket", socketId: 1, url: "ws://hub", verb: "open" });
    expect((await call({ op: "socket", socketId: 1, text: "early", verb: "send" })).error).toContain("connecting");
    FakeSocket.constructed[0]?.onopen?.();
    expect((await call({ op: "socket", socketId: 1, text: "later", verb: "send" })).state).toBe("open");
    expect(FakeSocket.constructed[0]?.sent).toEqual(["later"]);
  });

  it("writes a base64 frame as real bytes", async () => {
    const call = door();
    await call({ op: "socket", socketId: 1, url: "ws://hub", verb: "open" });
    FakeSocket.constructed[0]?.onopen?.();
    await call({ binary: btoa(String.fromCharCode(0, 1, 2, 250)), op: "socket", socketId: 1, verb: "send" });
    const written = FakeSocket.constructed[0]?.sent[0] as Uint8Array;
    expect(Array.from(written)).toEqual([0, 1, 2, 250]);
  });

  it("refuses a frame past the send ceiling instead of handing it to the socket", async () => {
    const call = door();
    await call({ op: "socket", socketId: 1, url: "ws://hub", verb: "open" });
    FakeSocket.constructed[0]?.onopen?.();
    expect((await call({ op: "socket", socketId: 1, text: "x".repeat(WGPU_SOCKET_SEND_MAX_BYTES + 1), verb: "send" })).error).toContain("exceeds");
    expect(FakeSocket.constructed[0]?.sent).toEqual([]);
  });

  it("refuses a send that carries neither text nor binary", async () => {
    const call = door();
    await call({ op: "socket", socketId: 1, url: "ws://hub", verb: "open" });
    FakeSocket.constructed[0]?.onopen?.();
    expect((await call({ op: "socket", socketId: 1, verb: "send" })).error).toContain("neither text nor binary");
  });
});

describe("wgpu socket door — paging and back-pressure", () => {
  it("hands back one PAGE of what the peer sent and says more is waiting", async () => {
    const call = door();
    await call({ op: "socket", socketId: 1, url: "ws://hub", verb: "open" });
    const socket = FakeSocket.constructed[0];
    socket?.onopen?.();
    for (let index = 0; index < WGPU_SOCKET_POLL_MAX_MESSAGES + 5; index += 1) socket?.onmessage?.({ data: `frame-${index}` });
    const first = await call({ op: "socket", socketId: 1, verb: "poll" });
    expect(first.messages).toHaveLength(WGPU_SOCKET_POLL_MAX_MESSAGES);
    expect(first.more).toBe(true);
    const second = await call({ op: "socket", socketId: 1, verb: "poll" });
    expect(second.messages).toHaveLength(5);
    expect(second.more).toBe(false);
  });

  it("clamps a caller that asks for more than the page ceiling", async () => {
    const call = door();
    await call({ op: "socket", socketId: 1, url: "ws://hub", verb: "open" });
    const socket = FakeSocket.constructed[0];
    socket?.onopen?.();
    for (let index = 0; index < WGPU_SOCKET_POLL_MAX_MESSAGES + 10; index += 1) socket?.onmessage?.({ data: `${index}` });
    const answer = await call({ maxMessages: 10_000, op: "socket", socketId: 1, verb: "poll" });
    expect(answer.messages).toHaveLength(WGPU_SOCKET_POLL_MAX_MESSAGES);
  });

  it("drops the OLDEST frames past the queue ceiling and REPORTS the loss exactly once", async () => {
    const call = door();
    await call({ op: "socket", socketId: 1, url: "ws://hub", verb: "open" });
    const socket = FakeSocket.constructed[0];
    socket?.onopen?.();
    for (let index = 0; index < WGPU_SOCKET_QUEUE_MAX_MESSAGES + 3; index += 1) socket?.onmessage?.({ data: `frame-${index}` });
    const first = await call({ op: "socket", socketId: 1, verb: "poll" });
    expect(first.dropped).toBe(3);
    expect(first.messages?.[0]).toEqual({ text: "frame-3" });
    const second = await call({ op: "socket", socketId: 1, verb: "poll" });
    expect(second.dropped ?? 0).toBe(0);
  });

  it("carries a binary frame back as standard base64", async () => {
    const call = door();
    await call({ op: "socket", socketId: 1, url: "ws://hub", verb: "open" });
    const socket = FakeSocket.constructed[0];
    socket?.onopen?.();
    socket?.onmessage?.({ data: new Uint8Array([7, 8, 9]).buffer });
    const answer = await call({ op: "socket", socketId: 1, verb: "poll" });
    expect(answer.messages).toEqual([{ binary: btoa(String.fromCharCode(7, 8, 9)) }]);
  });
});

describe("wgpu socket door — closing", () => {
  it("reports the peer's close code and keeps the frames that really arrived first", async () => {
    const call = door();
    await call({ op: "socket", socketId: 1, url: "ws://hub", verb: "open" });
    const socket = FakeSocket.constructed[0];
    socket?.onopen?.();
    socket?.onmessage?.({ data: "last word" });
    socket?.onclose?.({ code: 4401 });
    const answer = await call({ op: "socket", socketId: 1, verb: "poll" });
    expect(answer.state).toBe("closed");
    expect(answer.closeCode).toBe(4401);
    expect(answer.messages).toEqual([{ text: "last word" }]);
    // 🧯️ Forgotten only once drained — a second poll no longer knows the socket.
    expect((await call({ op: "socket", socketId: 1, verb: "poll" })).error).toContain("unknown socket 1");
  });

  it("closes the real socket and forgets it", async () => {
    const call = door();
    await call({ op: "socket", socketId: 1, url: "ws://hub", verb: "open" });
    expect((await call({ op: "socket", socketId: 1, verb: "close" })).state).toBe("closed");
    expect(FakeSocket.constructed[0]?.closed).toBe(true);
    expect((await call({ op: "socket", socketId: 1, verb: "close" })).error).toContain("unknown socket 1");
  });

  it("keeps two doors' sockets disjoint", async () => {
    const first = door();
    await first({ op: "socket", socketId: 1, url: "ws://hub", verb: "open" });
    const second = door();
    expect((await second({ op: "socket", socketId: 1, verb: "poll" })).error).toContain("unknown socket 1");
  });
});
