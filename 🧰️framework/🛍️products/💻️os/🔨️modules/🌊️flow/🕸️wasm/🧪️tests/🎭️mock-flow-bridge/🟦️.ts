import assert from "node:assert/strict";
import flowProtocol from "../../🧪️fixtures/📡️abi.json";

export interface MockFlowBridgeOptions {
  readonly hold?: number;
  readonly heldOpenReplies?: number;
  readonly rejectControls?: number;
  readonly rejectSessionCloseControls?: number;
  readonly rejectSessionReceiptAcks?: number;
  readonly rejectOpenReplies?: number;
}

interface MockFlowExports {
  readonly memory: WebAssembly.Memory;
  flow_bridge_allocate(length: number): number;
  flow_bridge_release(pointer: number, length: number): void;
  flow_bridge_send(pointer: number, length: number, credit: number, nowMs: bigint, deadlineMs: bigint): number;
  flow_bridge_poll(pointer: number, capacity: number, credit: number, nowMs: bigint, deadlineMs: bigint): number;
  flow_bridge_begin_close(): void;
  flow_bridge_terminal_is_empty(): number;
}

interface MockSessionOwner {
  readonly request: bigint;
  readonly generation: number;
}

interface HeldOperation {
  readonly generation: number;
  readonly operationHandle: { readonly slot: number; readonly generation: number };
}

interface OpenReply {
  readonly request: bigint;
  readonly generation: number;
  readonly slot: number;
}

export class MockFlowBridge {
  readonly exports: MockFlowExports;
  readonly operations: number[] = [];
  readonly operationSessions: { readonly operation: number; readonly slot: number | null }[] = [];
  readonly openRequestIds: string[] = [];
  readonly closedSessionSlots: number[] = [];
  sessionReceiptAckAttempts = 0;
  sessionCloseControlAttempts = 0;
  globalCloseCalls = 0;
  private retained?: Uint8Array;
  private closing = false;
  private sequence = 1;
  private rejectedControls: number;
  private heldOpenReplies: number;
  private rejectedOpenReplies: number;
  private readonly held = new Map<bigint, HeldOperation>();
  private readonly sessions = new Map<number, MockSessionOwner>();
  private nextSession = 1;
  private readonly queue: Uint8Array[] = [];
  private readonly pendingOpenReplies: OpenReply[] = [];
  private readonly sessionReceipts = new Set<bigint>();

  constructor(
    private readonly targetMemory: WebAssembly.Memory,
    private readonly options: MockFlowBridgeOptions = {},
  ) {
    this.rejectedControls = options.rejectControls ?? 0;
    this.heldOpenReplies = options.heldOpenReplies ?? 0;
    this.rejectedOpenReplies = options.rejectOpenReplies ?? 0;
    this.exports = {
      memory: targetMemory,
      flow_bridge_allocate: () => 8,
      flow_bridge_release: () => {},
      flow_bridge_send: (pointer, length, _credit, nowMs, deadlineMs) => {
        assert.equal(typeof nowMs, "bigint", "send-now-u64");
        assert.equal(typeof deadlineMs, "bigint", "send-deadline-u64");
        const frame = new Uint8Array(targetMemory.buffer, pointer, length).slice();
        const tag = frame[1];
        if (tag === 5 && frame[2] === 2) {
          this.sessionCloseControlAttempts += 1;
          if (this.sessionCloseControlAttempts <= (options.rejectSessionCloseControls ?? 0)) return -1;
        }
        if (tag === 2 && this.sessionReceipts.has(u64(frame, 2))) {
          this.sessionReceiptAckAttempts += 1;
          if (this.sessionReceiptAckAttempts <= (options.rejectSessionReceiptAcks ?? 0)) return -1;
          this.sessionReceipts.delete(u64(frame, 2));
        }
        if (tag === 5 && this.rejectedControls > 0) {
          this.rejectedControls -= 1;
          return -1;
        }
        if (tag === 1) this.acceptRequest(frame);
        else if (tag === 5) this.acceptControl(frame);
        return 1;
      },
      flow_bridge_poll: (pointer, capacity, _credit, nowMs, deadlineMs) => {
        assert.equal(typeof nowMs, "bigint", "poll-now-u64");
        assert.equal(typeof deadlineMs, "bigint", "poll-deadline-u64");
        this.retained ??= this.queue.shift();
        if (!this.retained) return 0;
        if (this.retained.length > capacity) return this.retained.length;
        new Uint8Array(targetMemory.buffer, pointer, this.retained.length).set(this.retained);
        const length = this.retained.length;
        this.retained = undefined;
        return length;
      },
      flow_bridge_begin_close: () => {
        this.closing = true;
        this.globalCloseCalls += 1;
        for (const slot of [...this.sessions.keys()]) this.closeSession(slot);
      },
      flow_bridge_terminal_is_empty: () => Number(this.closing && !this.retained && this.queue.length === 0 && this.sessionReceipts.size === 0),
    };
  }

  releaseOpenReplies(count = this.pendingOpenReplies.length): void {
    for (const reply of this.pendingOpenReplies.splice(0, count)) this.queue.push(successReply(reply.request, reply.generation, handle(reply.slot, 1)));
  }

  private acceptRequest(frame: Uint8Array): void {
    const operation = u16(frame, 2);
    const request = u64(frame, 4);
    const generation = u32(frame, 12);
    const slot = operation === flowProtocol.operations.open ? null : u32(frame, 20);
    this.operations.push(operation);
    this.operationSessions.push({ operation, slot });
    if (operation === flowProtocol.operations.open) {
      if (this.rejectedOpenReplies > 0) {
        this.rejectedOpenReplies -= 1;
        this.queue.push(failedReply(request, generation, "session open rejected before admission"));
        return;
      }
      const sessionSlot = this.nextSession++;
      this.sessions.set(sessionSlot, { request, generation });
      this.openRequestIds.push(String(request));
      const opened = { request, generation, slot: sessionSlot };
      if (this.heldOpenReplies > 0) {
        this.heldOpenReplies -= 1;
        this.pendingOpenReplies.push(opened);
      } else {
        this.queue.push(successReply(request, generation, handle(sessionSlot, 1)));
      }
      return;
    }
    const operationHandle = { slot: 2, generation: Number(request) };
    this.queue.push(event(request, generation, this.sequence++, 2_650, handle(operationHandle.slot, operationHandle.generation)));
    this.queue.push(event(request, generation, this.sequence++, 2_651, new Uint8Array()));
    this.queue.push(event(request, generation, this.sequence++, 2_652, new Uint8Array()));
    this.queue.push(event(request, generation, this.sequence++, 2_653, new Uint8Array()));
    if (this.options.hold === operation) {
      this.held.set(request, { generation, operationHandle });
      return;
    }
    this.finish(request, generation, operationHandle);
  }

  private acceptControl(frame: Uint8Array): void {
    if (frame[2] === 2) {
      this.closeSession(u32(frame, 3));
      return;
    }
    if (frame[2] !== 1) return;
    const request = u64(frame, 3);
    const generation = u32(frame, 11);
    const pending = this.held.get(request);
    if (!pending) return;
    this.held.delete(request);
    this.queue.push(event(request, generation, this.sequence++, 2_656, new Uint8Array()));
    this.queue.push(failedReply(request, generation, "cancelled"));
  }

  private closeSession(slot: number): void {
    const owner = this.sessions.get(slot);
    if (!owner) throw new Error("mock closed unknown session");
    this.sessions.delete(slot);
    this.closedSessionSlots.push(slot);
    const receipt = event(owner.request, owner.generation, this.sequence++, 2_657, handle(slot, 1));
    this.sessionReceipts.add(u64(receipt, 2));
    this.queue.push(receipt);
  }

  private finish(request: bigint, generation: number, operationHandle: { readonly slot: number; readonly generation: number }): void {
    this.queue.push(event(request, generation, this.sequence++, 2_656, handle(operationHandle.slot, operationHandle.generation)));
    this.queue.push(successReply(request, generation, text("{}")));
  }
}

const encoder = new TextEncoder();
const text = (value: string): Uint8Array => encoder.encode(value);
const handle = (slot: number, generation: number): Uint8Array =>
  bytes((writer) => {
    writer.u32(slot);
    writer.u32(generation);
  });
const successReply = (request: bigint, generation: number, body: Uint8Array): Uint8Array =>
  bytes((writer) => {
    writer.u8(1);
    writer.u8(2);
    writer.u64(request);
    writer.u32(generation);
    writer.u16(0);
    writer.u8(0);
    writer.body(body);
  });
const failedReply = (request: bigint, generation: number, message: string): Uint8Array => {
  const encoded = text(message);
  return bytes((writer) => {
    writer.u8(1);
    writer.u8(2);
    writer.u64(request);
    writer.u32(generation);
    writer.u16(1);
    writer.u8(1);
    writer.u16(16);
    writer.u16(encoded.length);
    writer.raw(encoded);
    writer.body(new Uint8Array());
  });
};
const event = (origin: bigint, generation: number, sequence: number, code: number, body: Uint8Array): Uint8Array => {
  const acknowledgement = origin ^ (BigInt(sequence) << 32n);
  return bytes((writer) => {
    writer.u8(1);
    writer.u8(3);
    writer.u64(acknowledgement);
    writer.u32(generation);
    writer.u32(sequence);
    writer.u16(code);
    writer.u16(0);
    writer.u8(0);
    writer.body(body);
  });
};

interface ByteWriter {
  u8(value: number): void;
  u16(value: number): void;
  u32(value: number): void;
  u64(value: bigint): void;
  raw(value: Uint8Array): void;
  body(value: Uint8Array): void;
}

function bytes(build: (writer: ByteWriter) => void): Uint8Array {
  const values: number[] = [];
  const number = (length: number, write: (view: DataView) => void): void => {
    const value = new Uint8Array(length);
    write(new DataView(value.buffer));
    values.push(...value);
  };
  const writer: ByteWriter = {
    u8: (value) => values.push(value),
    u16: (value) => number(2, (view) => view.setUint16(0, value, true)),
    u32: (value) => number(4, (view) => view.setUint32(0, value, true)),
    u64: (value) => number(8, (view) => view.setBigUint64(0, value, true)),
    raw: (value) => values.push(...value),
    body: (value) => {
      writer.u32(value.length);
      values.push(...value);
    },
  };
  build(writer);
  return Uint8Array.from(values);
}

const u16 = (value: Uint8Array, offset: number): number => new DataView(value.buffer, value.byteOffset, value.byteLength).getUint16(offset, true);
const u32 = (value: Uint8Array, offset: number): number => new DataView(value.buffer, value.byteOffset, value.byteLength).getUint32(offset, true);
const u64 = (value: Uint8Array, offset: number): bigint => new DataView(value.buffer, value.byteOffset, value.byteLength).getBigUint64(offset, true);
