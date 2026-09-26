//#region 🔀️StreamMux
/**
 * 🔀️ One channel per page and origin for every long-lived stream (contract `semio.io.stream-mux/v1`, `./🧬️schema/🔣️.json`).
 *
 * A browser allows six HTTP/1.1 connections per origin. Every `EventSource`, long poll or blocking job request holds one of
 * them for its whole life, so a shell with two watch streams and a folder watch per open document starved its own module
 * fetches and commands (measured: 4–5 of 6 held permanently, ticket 26/09/23 F2). This module carries all of them as numbered
 * streams on ONE WebSocket (WebSockets are pooled apart from those six): the frame codec, the server side (routes, per-stream
 * credit, coalescing queues, resumable event rings, jobs with progress and cancellation) and the page side (one reconnecting
 * channel, subscriptions with credit, a `MessagePort` bridge so workers share the page's channel).
 * Ein Kanal pro Seite und Ursprung trägt alle langlebigen Ströme; keiner belegt eine der sechs HTTP/1.1-Verbindungen.
 */
import contract from "./🧬️schema/🔣️.json" with { type: "json" };

/** @emoji 📏️ The bounds the frame contract declares (read from the schema's `Bounds` consts — no second source). */
export type StreamMuxBoundsV1 = {
  readonly subprotocol: string;
  readonly maxFrameBytes: number;
  readonly maxStreamsPerChannel: number;
  readonly maxCredit: number;
  readonly defaultCredit: number;
  readonly maxQueuedFrames: number;
  readonly maxRetainedEvents: number;
  readonly resumeGraceMs: number;
  readonly beatMs: number;
  readonly deadAfterMs: number;
  readonly reconnectMinMs: number;
  readonly reconnectMaxMs: number;
  readonly socketHighWaterBytes: number;
  readonly maxKeyLength: number;
  readonly maxDetailLength: number;
  readonly lingerMs: number;
};

/** @emoji 📐️ {@link StreamMuxBoundsV1} as declared by `semio.io.stream-mux/v1`. */
export const STREAM_MUX_BOUNDS_V1: StreamMuxBoundsV1 = Object.freeze(
  Object.fromEntries(Object.entries(contract.$defs.Bounds.properties).map(([name, spec]) => [name, (spec as { readonly const: string | number }).const])) as StreamMuxBoundsV1,
);

/** @emoji 🧾️ Any JSON value a route carries. */
export type StreamMuxJsonV1 = null | boolean | number | string | readonly StreamMuxJsonV1[] | { readonly [key: string]: StreamMuxJsonV1 };

/** @emoji ⏮️ Where a reader left off: the server epoch and the last sequence number it received. */
export type StreamMuxResumeV1 = { readonly epoch: string; readonly seq: number };

/** @emoji 📤️ Frames a reader sends. */
export type StreamMuxClientFrameV1 =
  | { readonly kind: "open"; readonly stream: number; readonly route: string; readonly key: string; readonly resume: StreamMuxResumeV1 | null; readonly credit: number }
  | { readonly kind: "grant"; readonly stream: number; readonly credit: number }
  | { readonly kind: "cancel"; readonly stream: number };

/** @emoji 🏁️ Why a stream ended. */
export type StreamMuxEndReasonV1 = "done" | "cancelled" | "refused" | "failed";

/** @emoji 📥️ Frames the server sends. */
export type StreamMuxServerFrameV1 =
  | { readonly kind: "hello"; readonly version: 1; readonly epoch: string; readonly beatMs: number }
  | { readonly kind: "opened"; readonly stream: number; readonly mode: "fresh" | "resumed"; readonly epoch: string; readonly seq: number }
  | { readonly kind: "data"; readonly stream: number; readonly seq: number; readonly data: StreamMuxJsonV1 }
  | { readonly kind: "progress"; readonly stream: number; readonly done: number; readonly total: number | null; readonly note: string }
  | { readonly kind: "end"; readonly stream: number; readonly reason: StreamMuxEndReasonV1; readonly detail: string }
  | { readonly kind: "beat"; readonly at: number };

/** @emoji 🚫️ Why a frame was refused (the schema's `Refusal` enum). */
export type StreamMuxRefusalV1 = "too-large" | "malformed-json" | "not-an-object" | "unknown-kind" | "unknown-field" | "missing-field" | "invalid-field";

/** @emoji ⚖️ A decoded frame or the refusal that stopped it. */
export type StreamMuxDecodedV1<F> = { readonly ok: true; readonly frame: F } | { readonly ok: false; readonly refusal: StreamMuxRefusalV1; readonly field: string | null };
//#endregion 🔀️StreamMux

//#region 🧬️Codec
type FieldCheck = (value: unknown) => boolean;

const MAX_SEQ = Number.MAX_SAFE_INTEGER;
const ROUTE_PATTERN = new RegExp(contract.$defs.Route.pattern, "u");
const EPOCH_PATTERN = new RegExp(contract.$defs.Epoch.pattern, "u");
const codePoints = (value: string): number => {
  let count = 0;
  for (const _ of value) count += 1;
  return count;
};
const isInteger = (value: unknown, min: number, max: number): boolean => typeof value === "number" && Number.isInteger(value) && value >= min && value <= max;
const streamId: FieldCheck = (value) => isInteger(value, 1, 2147483647);
const seq: FieldCheck = (value) => isInteger(value, 0, MAX_SEQ);
const epoch: FieldCheck = (value) => typeof value === "string" && EPOCH_PATTERN.test(value);
const text = (max: number): FieldCheck => (value) => typeof value === "string" && (value.length <= max || codePoints(value) <= max);
const isRecord = (value: unknown): value is Record<string, unknown> => typeof value === "object" && value !== null && !Array.isArray(value);
const resume: FieldCheck = (value) => value === null || (isRecord(value) && Object.keys(value).length === 2 && epoch(value.epoch) && seq(value.seq));

const CLIENT_SHAPES: Readonly<Record<string, readonly (readonly [string, FieldCheck])[]>> = {
  open: [["stream", streamId], ["route", (value) => typeof value === "string" && ROUTE_PATTERN.test(value)], ["key", text(STREAM_MUX_BOUNDS_V1.maxKeyLength)], ["resume", resume], ["credit", (value) => isInteger(value, 0, STREAM_MUX_BOUNDS_V1.maxCredit)]],
  grant: [["stream", streamId], ["credit", (value) => isInteger(value, 1, STREAM_MUX_BOUNDS_V1.maxCredit)]],
  cancel: [["stream", streamId]],
};

const SERVER_SHAPES: Readonly<Record<string, readonly (readonly [string, FieldCheck])[]>> = {
  hello: [["version", (value) => value === 1], ["epoch", epoch], ["beatMs", (value) => isInteger(value, 1, 60000)]],
  opened: [["stream", streamId], ["mode", (value) => value === "fresh" || value === "resumed"], ["epoch", epoch], ["seq", seq]],
  data: [["stream", streamId], ["seq", seq], ["data", () => true]],
  progress: [["stream", streamId], ["done", seq], ["total", (value) => value === null || seq(value)], ["note", text(STREAM_MUX_BOUNDS_V1.maxDetailLength)]],
  end: [["stream", streamId], ["reason", (value) => value === "done" || value === "cancelled" || value === "refused" || value === "failed"], ["detail", text(STREAM_MUX_BOUNDS_V1.maxDetailLength)]],
  beat: [["at", seq]],
};

const utf8Length = (value: string): number => {
  if (value.length * 3 <= STREAM_MUX_BOUNDS_V1.maxFrameBytes) return value.length;
  let bytes = 0;
  for (let index = 0; index < value.length; index += 1) {
    const unit = value.charCodeAt(index);
    if (unit < 0x80) bytes += 1;
    else if (unit < 0x800) bytes += 2;
    else if (unit >= 0xd800 && unit <= 0xdbff) {
      bytes += 4;
      index += 1;
    } else bytes += 3;
  }
  return bytes;
};

function decodeFrame<F>(textFrame: string, shapes: Readonly<Record<string, readonly (readonly [string, FieldCheck])[]>>): StreamMuxDecodedV1<F> {
  if (utf8Length(textFrame) > STREAM_MUX_BOUNDS_V1.maxFrameBytes) return { ok: false, refusal: "too-large", field: null };
  let value: unknown;
  try {
    value = JSON.parse(textFrame);
  } catch {
    return { ok: false, refusal: "malformed-json", field: null };
  }
  if (!isRecord(value)) return { ok: false, refusal: "not-an-object", field: null };
  const shape = typeof value.kind === "string" && Object.hasOwn(shapes, value.kind) ? shapes[value.kind] : undefined;
  if (shape === undefined) return { ok: false, refusal: "unknown-kind", field: "kind" };
  for (const name of Object.keys(value)) if (name !== "kind" && !shape.some(([field]) => field === name)) return { ok: false, refusal: "unknown-field", field: name };
  for (const [field] of shape) if (!Object.hasOwn(value, field)) return { ok: false, refusal: "missing-field", field };
  for (const [field, check] of shape) if (!check(value[field])) return { ok: false, refusal: "invalid-field", field };
  const frame: Record<string, unknown> = { kind: value.kind };
  for (const [field] of shape) frame[field] = field === "resume" && value[field] !== null ? { epoch: (value[field] as Record<string, unknown>).epoch, seq: (value[field] as Record<string, unknown>).seq } : value[field];
  return { ok: true, frame: frame as F };
}

/** @emoji 🛂️ Decodes and validates one reader frame (exact fields, schema ranges, byte bound). */
export function decodeStreamMuxClientFrameV1(textFrame: string): StreamMuxDecodedV1<StreamMuxClientFrameV1> {
  return decodeFrame<StreamMuxClientFrameV1>(textFrame, CLIENT_SHAPES);
}

/** @emoji 🛃️ Decodes and validates one server frame (exact fields, schema ranges, byte bound). */
export function decodeStreamMuxServerFrameV1(textFrame: string): StreamMuxDecodedV1<StreamMuxServerFrameV1> {
  return decodeFrame<StreamMuxServerFrameV1>(textFrame, SERVER_SHAPES);
}

/** @emoji 🖨️ Canonical text of a frame: `kind` first, then the fields in contract order, no whitespace. */
export function encodeStreamMuxFrameV1(frame: StreamMuxClientFrameV1 | StreamMuxServerFrameV1): string {
  const shape = (Object.hasOwn(CLIENT_SHAPES, frame.kind) ? CLIENT_SHAPES : SERVER_SHAPES)[frame.kind];
  const ordered: Record<string, unknown> = { kind: frame.kind };
  const source = frame as unknown as Record<string, unknown>;
  for (const [field] of shape) ordered[field] = field === "resume" && source[field] !== null ? { epoch: (source[field] as StreamMuxResumeV1).epoch, seq: (source[field] as StreamMuxResumeV1).seq } : source[field];
  return JSON.stringify(ordered);
}

/** @emoji ✂️ Clips a detail or note to the contract's code-point bound. */
export function clipStreamMuxDetailV1(value: string): string {
  if (value.length <= STREAM_MUX_BOUNDS_V1.maxDetailLength) return value;
  return [...value].slice(0, STREAM_MUX_BOUNDS_V1.maxDetailLength).join("");
}
//#endregion 🧬️Codec

//#region 🖥️Server
/** @emoji 🔌️ The server's view of one accepted WebSocket (whatever runtime socket backs it). */
export interface StreamMuxSocketV1 {
  send(frame: string): void;
  bufferedAmount(): number;
  close(code: number, reason: string): void;
}

/** @emoji ⏲️ Timers the server and channel run on (injectable for deterministic laws). */
export interface StreamMuxTimersV1 {
  now(): number;
  setTimeout(run: () => void, ms: number): unknown;
  clearTimeout(handle: unknown): void;
}

/** @emoji 🕰️ The platform's own timers. */
export const PLATFORM_STREAM_MUX_TIMERS_V1: StreamMuxTimersV1 = {
  now: () => Date.now(),
  setTimeout: (run, ms) => globalThis.setTimeout(run, ms),
  clearTimeout: (handle) => globalThis.clearTimeout(handle as ReturnType<typeof globalThis.setTimeout>),
};

/** @emoji 🧵️ What a job route's work sees: its cancellation and its progress lane. */
export interface StreamMuxJobV1 {
  readonly signal: AbortSignal;
  progress(done: number, total: number | null, note: string): void;
}

/** @emoji 🛣️ One named route. A watch route has a `snapshot` (sent on every fresh open), an optional `coalesce` key and an
 * optional `source` (the live producer of one key: started with its route instance, stopped when the instance retires); a job
 * route has `run` (started by its first reader of a key, cancelled when its last reader cancels or its grace expires). */
export interface StreamMuxRouteV1 {
  readonly admit?: (key: string) => boolean;
  readonly snapshot?: (key: string) => StreamMuxJsonV1 | Promise<StreamMuxJsonV1>;
  readonly coalesce?: (data: StreamMuxJsonV1) => string;
  readonly source?: (key: string, emit: (data: StreamMuxJsonV1) => void) => () => void;
  readonly run?: (key: string, job: StreamMuxJobV1) => Promise<void>;
}

type RetainedEvent = { readonly seq: number; readonly data: StreamMuxJsonV1 };
type JobState = { readonly controller: AbortController; progress: { readonly done: number; readonly total: number | null; readonly note: string } | null; terminal: { readonly reason: StreamMuxEndReasonV1; readonly detail: string } | null };
type RouteInstance = {
  readonly name: string;
  readonly key: string;
  readonly route: StreamMuxRouteV1;
  floor: number;
  readonly ring: RetainedEvent[];
  readonly readers: Set<ServerStream>;
  grace: unknown;
  job: JobState | null;
  stop: (() => void) | null;
};
type QueuedData = { readonly seq: number; readonly data: StreamMuxJsonV1; readonly coalesce: string | null };
type ServerStream = {
  readonly id: number;
  readonly connection: ServerConnection;
  readonly instance: RouteInstance;
  credit: number;
  queue: QueuedData[];
  progress: { readonly done: number; readonly total: number | null; readonly note: string } | null;
  resync: boolean;
  snapshotting: boolean;
  ended: boolean;
};
type ServerConnection = { readonly socket: StreamMuxSocketV1; readonly streams: Map<number, ServerStream>; retry: unknown; closed: boolean };

/** @emoji 🔗️ One accepted channel: feed it every received text frame, and tell it when the socket closed. */
export interface StreamMuxConnectionV1 {
  receive(frame: string): void;
  closed(): void;
}

/** @emoji 🖥️ The server half: named routes, route instances with resumable event rings, and every accepted channel. */
export class StreamMuxServerV1 {
  readonly epoch: string;
  private readonly timers: StreamMuxTimersV1;
  private readonly routes = new Map<string, StreamMuxRouteV1>();
  private readonly instances = new Map<string, RouteInstance>();
  private readonly connections = new Set<ServerConnection>();
  private head = 0;
  private beat: unknown = null;

  constructor(options: { readonly epoch?: string; readonly timers?: StreamMuxTimersV1 } = {}) {
    this.timers = options.timers ?? PLATFORM_STREAM_MUX_TIMERS_V1;
    this.epoch = options.epoch ?? randomEpoch();
  }

  /** @emoji 🪧️ Registers a route; answers its unregistration (every open stream of it ends `refused`). */
  route(name: string, route: StreamMuxRouteV1): () => void {
    if (!ROUTE_PATTERN.test(name)) throw new Error(`stream-mux: invalid route name ${name}`);
    if (this.routes.has(name)) throw new Error(`stream-mux: route ${name} is already registered`);
    this.routes.set(name, route);
    return () => {
      if (this.routes.get(name) !== route) return;
      this.routes.delete(name);
      for (const instance of [...this.instances.values()]) if (instance.name === name) this.retire(instance, "refused", "route-removed");
    };
  }

  /** @emoji 📣️ Publishes one event to every reader of `route`/`key` and retains it for resuming readers. Every publish takes the
   * next sequence number of the epoch, read or not, so a reader that resumes across an unobserved event starts fresh. */
  publish(name: string, key: string, data: StreamMuxJsonV1): void {
    this.head += 1;
    const instance = this.instances.get(instanceKey(name, key));
    if (instance === undefined) return;
    this.record(instance, { seq: this.head, data });
    for (const reader of instance.readers) this.enqueue(reader, this.head, data);
  }

  /** @emoji 🔢️ How many channels and streams are live (diagnostics and laws). */
  census(): { readonly channels: number; readonly streams: number; readonly instances: number } {
    let streams = 0;
    for (const connection of this.connections) streams += connection.streams.size;
    return { channels: this.connections.size, streams, instances: this.instances.size };
  }

  /** @emoji 🤝️ Accepts one channel: sends `hello` and answers the receive/close entry points. */
  connect(socket: StreamMuxSocketV1): StreamMuxConnectionV1 {
    const connection: ServerConnection = { socket, streams: new Map(), retry: null, closed: false };
    this.connections.add(connection);
    this.send(connection, { kind: "hello", version: 1, epoch: this.epoch, beatMs: STREAM_MUX_BOUNDS_V1.beatMs });
    this.armBeat();
    return {
      receive: (frame) => this.receive(connection, frame),
      closed: () => this.drop(connection),
    };
  }

  /** @emoji 🧹️ Ends every channel and job (server shutdown). */
  close(): void {
    for (const connection of [...this.connections]) {
      connection.socket.close(1001, "server-closing");
      this.drop(connection);
    }
    for (const instance of [...this.instances.values()]) this.retire(instance, "cancelled", "server-closing");
    if (this.beat !== null) this.timers.clearTimeout(this.beat);
    this.beat = null;
  }

  private armBeat(): void {
    if (this.beat !== null || this.connections.size === 0) return;
    this.beat = this.timers.setTimeout(() => {
      this.beat = null;
      for (const connection of this.connections) this.send(connection, { kind: "beat", at: this.timers.now() });
      this.armBeat();
    }, STREAM_MUX_BOUNDS_V1.beatMs);
  }

  private send(connection: ServerConnection, frame: StreamMuxServerFrameV1): void {
    if (!connection.closed) connection.socket.send(encodeStreamMuxFrameV1(frame));
  }

  private receive(connection: ServerConnection, textFrame: string): void {
    if (connection.closed) return;
    const decoded = decodeStreamMuxClientFrameV1(textFrame);
    if (!decoded.ok) {
      connection.socket.close(1008, `stream-mux: ${decoded.refusal}${decoded.field === null ? "" : ` ${decoded.field}`}`);
      this.drop(connection);
      return;
    }
    const frame = decoded.frame;
    if (frame.kind === "open") this.open(connection, frame);
    else if (frame.kind === "grant") {
      const stream = connection.streams.get(frame.stream);
      if (stream === undefined) return;
      stream.credit = Math.min(STREAM_MUX_BOUNDS_V1.maxCredit, stream.credit + frame.credit);
      this.pump(stream);
    } else {
      const stream = connection.streams.get(frame.stream);
      if (stream === undefined) return;
      this.finish(stream, "cancelled", "");
      this.leave(stream.instance, true);
    }
  }

  private open(connection: ServerConnection, frame: Extract<StreamMuxClientFrameV1, { kind: "open" }>): void {
    if (connection.streams.has(frame.stream)) {
      connection.socket.close(1008, "stream-mux: duplicate-stream");
      this.drop(connection);
      return;
    }
    const refuse = (detail: string): void => this.send(connection, { kind: "end", stream: frame.stream, reason: "refused", detail });
    if (connection.streams.size >= STREAM_MUX_BOUNDS_V1.maxStreamsPerChannel) return refuse("stream-limit");
    const route = this.routes.get(frame.route);
    if (route === undefined) return refuse("unknown-route");
    if (route.admit !== undefined && !route.admit(frame.key)) return refuse("key-refused");
    const key = instanceKey(frame.route, frame.key);
    let instance = this.instances.get(key);
    if (instance !== undefined && instance.job !== null && instance.job.terminal !== null && instance.job.terminal.reason !== "done") {
      this.retire(instance, instance.job.terminal.reason, instance.job.terminal.detail);
      instance = undefined;
    }
    if (instance === undefined) {
      const created: RouteInstance = { name: frame.route, key: frame.key, route, floor: this.head, ring: [], readers: new Set(), grace: null, job: null, stop: null };
      instance = created;
      this.instances.set(key, created);
      if (route.source !== undefined) created.stop = route.source(frame.key, (data) => {
        if (this.instances.get(key) === created) this.publish(frame.route, frame.key, data);
      });
    }
    if (instance.grace !== null) {
      this.timers.clearTimeout(instance.grace);
      instance.grace = null;
    }
    const stream: ServerStream = { id: frame.stream, connection, instance, credit: frame.credit, queue: [], progress: null, resync: false, snapshotting: false, ended: false };
    connection.streams.set(frame.stream, stream);
    instance.readers.add(stream);
    const resumable = frame.resume !== null && frame.resume.epoch === this.epoch && frame.resume.seq >= instance.floor && frame.resume.seq <= this.head;
    if (resumable) {
      const after = frame.resume!.seq;
      this.send(connection, { kind: "opened", stream: stream.id, mode: "resumed", epoch: this.epoch, seq: after });
      for (const event of instance.ring) if (event.seq > after) stream.queue.push({ seq: event.seq, data: event.data, coalesce: null });
    } else stream.resync = true;
    if (instance.job !== null && instance.job.progress !== null) stream.progress = instance.job.progress;
    this.pump(stream);
    if (instance.job !== null && instance.job.terminal !== null) this.finish(stream, instance.job.terminal.reason, instance.job.terminal.detail);
    else if (route.run !== undefined && instance.job === null) this.start(instance, route.run);
  }

  private start(instance: RouteInstance, run: (key: string, job: StreamMuxJobV1) => Promise<void>): void {
    const job: JobState = { controller: new AbortController(), progress: null, terminal: null };
    instance.job = job;
    const settle = (reason: StreamMuxEndReasonV1, detail: string): void => {
      if (job.terminal !== null) return;
      job.terminal = { reason, detail: clipStreamMuxDetailV1(detail) };
      for (const reader of [...instance.readers]) this.finish(reader, reason, job.terminal.detail);
      if (instance.readers.size === 0 && instance.grace === null && this.instances.get(instanceKey(instance.name, instance.key)) === instance) this.linger(instance);
    };
    const context: StreamMuxJobV1 = {
      signal: job.controller.signal,
      progress: (done, total, note) => {
        if (job.terminal !== null) return;
        job.progress = { done, total, note: clipStreamMuxDetailV1(note) };
        for (const reader of instance.readers) {
          reader.progress = job.progress;
          this.pump(reader);
        }
      },
    };
    void Promise.resolve()
      .then(() => run(instance.key, context))
      .then(
        () => settle(job.controller.signal.aborted ? "cancelled" : "done", ""),
        (error: unknown) => settle(job.controller.signal.aborted ? "cancelled" : "failed", error instanceof Error ? error.message : String(error)),
      );
  }

  private record(instance: RouteInstance, event: RetainedEvent): void {
    instance.ring.push(event);
    if (instance.ring.length > STREAM_MUX_BOUNDS_V1.maxRetainedEvents) instance.floor = instance.ring.shift()!.seq;
  }

  private enqueue(stream: ServerStream, eventSeq: number, data: StreamMuxJsonV1): void {
    if (stream.ended || stream.resync) return;
    const coalesce = stream.instance.route.coalesce === undefined ? null : stream.instance.route.coalesce(data);
    if (coalesce !== null) stream.queue = stream.queue.filter((queued) => queued.coalesce !== coalesce);
    stream.queue.push({ seq: eventSeq, data, coalesce });
    if (stream.queue.length > STREAM_MUX_BOUNDS_V1.maxQueuedFrames) {
      stream.queue = [];
      stream.resync = true;
    }
    this.pump(stream);
  }

  private pump(stream: ServerStream): void {
    const connection = stream.connection;
    if (stream.ended || connection.closed) return;
    if (connection.socket.bufferedAmount() > STREAM_MUX_BOUNDS_V1.socketHighWaterBytes) {
      if (connection.retry === null)
        connection.retry = this.timers.setTimeout(() => {
          connection.retry = null;
          for (const waiting of connection.streams.values()) this.pump(waiting);
        }, 20);
      return;
    }
    if (stream.resync && stream.credit > 0 && !stream.snapshotting) {
      stream.resync = false;
      stream.queue = [];
      this.send(connection, { kind: "opened", stream: stream.id, mode: "fresh", epoch: this.epoch, seq: this.head });
      const snapshot = stream.instance.route.snapshot;
      if (snapshot !== undefined) {
        const at = this.head;
        stream.snapshotting = true;
        void Promise.resolve()
          .then(() => snapshot(stream.instance.key))
          .then(
            (data) => {
              stream.snapshotting = false;
              if (!stream.ended && !stream.resync) stream.queue.unshift({ seq: at, data, coalesce: null });
              this.pump(stream);
            },
            (error: unknown) => {
              stream.snapshotting = false;
              this.finish(stream, "failed", error instanceof Error ? error.message : String(error));
              this.leave(stream.instance, true);
            },
          );
      }
    }
    if (stream.resync) return;
    if (stream.progress !== null) {
      const progress = stream.progress;
      stream.progress = null;
      this.send(connection, { kind: "progress", stream: stream.id, ...progress });
    }
    while (stream.credit > 0 && !stream.snapshotting) {
      const next = stream.queue.shift();
      if (next === undefined) return;
      stream.credit -= 1;
      this.send(connection, { kind: "data", stream: stream.id, seq: next.seq, data: next.data });
    }
  }

  private finish(stream: ServerStream, reason: StreamMuxEndReasonV1, detail: string): void {
    if (stream.ended) return;
    stream.ended = true;
    stream.connection.streams.delete(stream.id);
    stream.instance.readers.delete(stream);
    this.send(stream.connection, { kind: "end", stream: stream.id, reason, detail: clipStreamMuxDetailV1(detail) });
  }

  private leave(instance: RouteInstance, deliberate: boolean): void {
    if (instance.readers.size > 0) return;
    if (deliberate && instance.job !== null && instance.job.terminal === null) instance.job.controller.abort(new Error("stream-mux: cancelled by its last reader"));
    if (instance.grace === null && this.instances.get(instanceKey(instance.name, instance.key)) === instance) this.linger(instance);
  }

  private linger(instance: RouteInstance): void {
    instance.grace = this.timers.setTimeout(() => {
      instance.grace = null;
      if (instance.readers.size === 0) this.retire(instance, "cancelled", "resume-grace-expired");
    }, STREAM_MUX_BOUNDS_V1.resumeGraceMs);
  }

  private retire(instance: RouteInstance, reason: StreamMuxEndReasonV1, detail: string): void {
    if (instance.grace !== null) this.timers.clearTimeout(instance.grace);
    instance.grace = null;
    if (instance.job !== null && instance.job.terminal === null) instance.job.controller.abort(new Error(`stream-mux: ${detail}`));
    for (const reader of [...instance.readers]) this.finish(reader, reason, detail);
    if (this.instances.get(instanceKey(instance.name, instance.key)) === instance) this.instances.delete(instanceKey(instance.name, instance.key));
    const stop = instance.stop;
    instance.stop = null;
    stop?.();
  }

  private drop(connection: ServerConnection): void {
    if (connection.closed) return;
    connection.closed = true;
    this.connections.delete(connection);
    if (connection.retry !== null) this.timers.clearTimeout(connection.retry);
    const instances = new Set<RouteInstance>();
    for (const stream of connection.streams.values()) {
      stream.ended = true;
      stream.instance.readers.delete(stream);
      instances.add(stream.instance);
    }
    connection.streams.clear();
    for (const instance of instances) this.leave(instance, false);
  }
}

const instanceKey = (route: string, key: string): string => `${route}\n${key}`;

function randomEpoch(): string {
  const bytes = new Uint8Array(8);
  globalThis.crypto.getRandomValues(bytes);
  return [...bytes].map((byte) => byte.toString(16).padStart(2, "0")).join("");
}
//#endregion 🖥️Server

//#region 📡️Reader
/** @emoji 🎧️ What a reader of one stream handles. `opened("fresh")` voids every earlier state of the stream (a snapshot
 * follows when the route has one); `data` may return a promise — its credit is returned only when it settles (backpressure). */
export interface StreamMuxHandlersV1 {
  readonly opened?: (mode: "fresh" | "resumed") => void;
  readonly data?: (data: StreamMuxJsonV1) => void | Promise<void>;
  readonly progress?: (done: number, total: number | null, note: string) => void;
  readonly end?: (reason: StreamMuxEndReasonV1, detail: string) => void;
}

/** @emoji 🎟️ One open stream; `close()` cancels it (idempotent). */
export interface StreamMuxSubscriptionV1 {
  close(): void;
}

/** @emoji 📡️ Anything that opens streams: the page's channel, or a worker's port onto it. */
export interface StreamMuxEndpointV1 {
  open(route: string, key: string, handlers: StreamMuxHandlersV1, options?: { readonly credit?: number; readonly signal?: AbortSignal }): StreamMuxSubscriptionV1;
}

/** @emoji 🔇️ An endpoint that never opens anything (no WebSocket in this runtime). */
export const SILENT_STREAM_MUX_ENDPOINT_V1: StreamMuxEndpointV1 = { open: () => ({ close: () => undefined }) };

/** @emoji 👂️ One route of `endpoint` as a plain subscription: every data value of the stream goes to `listener`; answers the
 * close. The shape a source owner hands a kernel plugin source (the kernel's `PluginSourceWatch`). */
export function streamMuxWatchV1(endpoint: StreamMuxEndpointV1, route: string, key = ""): (listener: (data: StreamMuxJsonV1) => void) => () => void {
  return (listener) => endpoint.open(route, key, { data: (data) => listener(data) }).close;
}

/** @emoji 🪣️ Delivers server frames of one stream to its handlers and returns credit as handlers finish. */
class ReaderCore {
  readonly handlers: StreamMuxHandlersV1;
  readonly window: number;
  private readonly grant: (credit: number) => void;
  private consumed = 0;
  done = false;

  constructor(handlers: StreamMuxHandlersV1, window: number, grant: (credit: number) => void) {
    this.handlers = handlers;
    this.window = window;
    this.grant = grant;
  }

  deliver(frame: StreamMuxServerFrameV1): void {
    if (this.done) return;
    if (frame.kind === "opened") this.handlers.opened?.(frame.mode);
    else if (frame.kind === "progress") this.handlers.progress?.(frame.done, frame.total, frame.note);
    else if (frame.kind === "end") {
      this.done = true;
      this.handlers.end?.(frame.reason, frame.detail);
    } else if (frame.kind === "data") {
      let settled: void | Promise<void> = undefined;
      try {
        settled = this.handlers.data?.(frame.data);
      } catch (error) {
        console.error("[stream-mux] a data handler threw", error);
      }
      if (settled instanceof Promise) settled.then(() => this.consume(), (error: unknown) => {
        console.error("[stream-mux] a data handler rejected", error);
        this.consume();
      });
      else this.consume();
    }
  }

  private consume(): void {
    if (this.done) return;
    this.consumed += 1;
    if (this.consumed * 2 < this.window) return;
    const credit = this.consumed;
    this.consumed = 0;
    this.grant(credit);
  }
}

const creditOf = (credit: number | undefined): number => Math.max(1, Math.min(STREAM_MUX_BOUNDS_V1.maxCredit, Math.trunc(credit ?? STREAM_MUX_BOUNDS_V1.defaultCredit)));
//#endregion 📡️Reader

//#region 🛰️Channel
/** @emoji ⛓️ One live link of a channel (a WebSocket, or a test double). */
export interface StreamMuxLinkV1 {
  send(frame: string): void;
  close(): void;
}

/** @emoji 🛰️ How a channel opens links. */
export interface StreamMuxTransportV1 {
  connect(events: { readonly open: () => void; readonly message: (frame: string) => void; readonly closed: () => void }): StreamMuxLinkV1;
}

/** @emoji 🌐️ The WebSocket transport (subprotocol `semio.stream-mux.v1`); a link whose socket negotiates anything else closes. */
export function webSocketStreamMuxTransportV1(url: string): StreamMuxTransportV1 {
  return {
    connect(events) {
      const socket = new WebSocket(url, [STREAM_MUX_BOUNDS_V1.subprotocol]);
      let closed = false;
      const close = (): void => {
        if (closed) return;
        closed = true;
        socket.onopen = null;
        socket.onmessage = null;
        socket.onclose = null;
        socket.onerror = null;
        if (socket.readyState === WebSocket.CONNECTING || socket.readyState === WebSocket.OPEN) socket.close(1000, "stream-mux: closed");
        events.closed();
      };
      socket.onopen = () => {
        if (socket.protocol !== STREAM_MUX_BOUNDS_V1.subprotocol) return close();
        events.open();
      };
      socket.onmessage = (event: MessageEvent) => {
        if (typeof event.data === "string") events.message(event.data);
      };
      socket.onclose = () => close();
      socket.onerror = () => close();
      return { send: (frame) => socket.send(frame), close };
    },
  };
}

/** @emoji 🧺️ Receives the frames of one channel-side stream (a page subscription or a bridged port stream). */
type ChannelSink = { deliver(frame: StreamMuxServerFrameV1): void };
type ChannelStream = { readonly id: number; readonly route: string; readonly key: string; readonly sink: ChannelSink; available: number; epoch: string | null; seq: number };

/** @emoji 📶️ The page's ONE channel to its origin: every stream of the page (and of its workers, through {@link attachPort})
 * rides it; a lost link reconnects with jittered backoff and reopens every stream from where it left off. */
export class StreamMuxChannelV1 implements StreamMuxEndpointV1 {
  private readonly transport: StreamMuxTransportV1;
  private readonly timers: StreamMuxTimersV1;
  private readonly streams = new Map<number, ChannelStream>();
  private link: StreamMuxLinkV1 | null = null;
  private linkOpen = false;
  private nextId = 1;
  private attempt = 0;
  private reconnect: unknown = null;
  private watchdog: unknown = null;
  private linger: unknown = null;
  private links = 0;

  constructor(transport: StreamMuxTransportV1, timers: StreamMuxTimersV1 = PLATFORM_STREAM_MUX_TIMERS_V1) {
    this.transport = transport;
    this.timers = timers;
  }

  /** @emoji 📊️ Link state and live streams (diagnostics and laws); `links` counts every link this channel opened. */
  census(): { readonly open: boolean; readonly streams: number; readonly links: number } {
    return { open: this.linkOpen, streams: this.streams.size, links: this.links };
  }

  open(route: string, key: string, handlers: StreamMuxHandlersV1, options: { readonly credit?: number; readonly signal?: AbortSignal } = {}): StreamMuxSubscriptionV1 {
    const window = creditOf(options.credit);
    let stream: ChannelStream | null = null;
    const reader = new ReaderCore(handlers, window, (credit) => {
      if (stream !== null) this.grant(stream, credit);
    });
    stream = this.add(route, key, window, { deliver: (frame) => reader.deliver(frame) });
    const subscribed = stream;
    const close = (): void => {
      if (reader.done) return;
      reader.done = true;
      this.cancel(subscribed);
    };
    if (options.signal !== undefined) {
      if (options.signal.aborted) close();
      else options.signal.addEventListener("abort", close, { once: true });
    }
    return { close };
  }

  /** @emoji 🌉️ Bridges a worker's port onto this channel: the port speaks the same client frames with its own stream ids and
   * receives the server frames re-addressed to them. Answers the detach that cancels every stream the port opened. */
  attachPort(port: MessagePort): () => void {
    const local = new Map<number, ChannelStream>();
    const detach = (): void => {
      port.onmessage = null;
      for (const stream of local.values()) this.cancel(stream);
      local.clear();
    };
    port.onmessage = (event: MessageEvent) => {
      if (typeof event.data !== "string") return;
      const decoded = decodeStreamMuxClientFrameV1(event.data);
      if (!decoded.ok) return;
      const frame = decoded.frame;
      if (frame.kind === "open") {
        if (local.has(frame.stream)) return;
        const localId = frame.stream;
        local.set(
          localId,
          this.add(frame.route, frame.key, frame.credit, {
            deliver: (server) => {
              if (server.kind === "end") local.delete(localId);
              port.postMessage(encodeStreamMuxFrameV1({ ...server, stream: localId } as StreamMuxServerFrameV1));
            },
          }),
        );
      } else {
        const stream = local.get(frame.stream);
        if (stream === undefined) return;
        if (frame.kind === "grant") this.grant(stream, frame.credit);
        else {
          local.delete(frame.stream);
          this.cancel(stream);
        }
      }
    };
    port.start();
    return detach;
  }

  /** @emoji 🚪️ Cancels every stream and closes the link for good. */
  close(): void {
    for (const stream of [...this.streams.values()]) this.cancel(stream);
    this.shut();
  }

  private add(route: string, key: string, credit: number, sink: ChannelSink): ChannelStream {
    const stream: ChannelStream = { id: this.nextId++, route, key, sink, available: credit, epoch: null, seq: 0 };
    this.streams.set(stream.id, stream);
    if (this.linger !== null) {
      this.timers.clearTimeout(this.linger);
      this.linger = null;
    }
    if (this.linkOpen) this.sendOpen(stream);
    else this.ensureLink();
    return stream;
  }

  private sendOpen(stream: ChannelStream): void {
    this.link?.send(encodeStreamMuxFrameV1({ kind: "open", stream: stream.id, route: stream.route, key: stream.key, resume: stream.epoch === null ? null : { epoch: stream.epoch, seq: stream.seq }, credit: Math.min(STREAM_MUX_BOUNDS_V1.maxCredit, Math.max(0, stream.available)) }));
  }

  private grant(stream: ChannelStream, credit: number): void {
    if (this.streams.get(stream.id) !== stream) return;
    stream.available += credit;
    if (this.linkOpen) this.link?.send(encodeStreamMuxFrameV1({ kind: "grant", stream: stream.id, credit: Math.min(STREAM_MUX_BOUNDS_V1.maxCredit, credit) }));
  }

  private cancel(stream: ChannelStream): void {
    if (this.streams.get(stream.id) !== stream) return;
    this.streams.delete(stream.id);
    if (this.linkOpen) this.link?.send(encodeStreamMuxFrameV1({ kind: "cancel", stream: stream.id }));
    this.release();
  }

  private release(): void {
    if (this.streams.size > 0 || this.linger !== null || this.link === null) return;
    this.linger = this.timers.setTimeout(() => {
      this.linger = null;
      if (this.streams.size === 0) this.shut();
    }, STREAM_MUX_BOUNDS_V1.lingerMs);
  }

  private ensureLink(): void {
    if (this.link !== null || this.reconnect !== null) return;
    this.links += 1;
    let link: StreamMuxLinkV1 | null = null;
    const current = (): boolean => link !== null && this.link === link;
    link = this.transport.connect({
      open: () => {
        if (!current()) return;
        this.linkOpen = true;
        this.arm();
        for (const stream of this.streams.values()) this.sendOpen(stream);
      },
      message: (frame) => {
        if (current()) this.receive(frame);
      },
      closed: () => {
        if (!current()) return;
        this.lost();
      },
    });
    this.link = link;
  }

  private receive(textFrame: string): void {
    this.arm();
    const decoded = decodeStreamMuxServerFrameV1(textFrame);
    if (!decoded.ok) {
      console.error(`[stream-mux] refused a server frame: ${decoded.refusal}${decoded.field === null ? "" : ` ${decoded.field}`}`);
      this.link?.close();
      return;
    }
    const frame = decoded.frame;
    if (frame.kind === "hello") return;
    if (frame.kind === "beat") {
      this.attempt = 0;
      return;
    }
    const stream = this.streams.get(frame.stream);
    if (stream === undefined) return;
    if (frame.kind === "opened") {
      stream.epoch = frame.epoch;
      stream.seq = frame.seq;
    } else if (frame.kind === "data") {
      stream.seq = frame.seq;
      stream.available -= 1;
    } else if (frame.kind === "end") this.streams.delete(stream.id);
    stream.sink.deliver(frame);
    if (frame.kind === "end") this.release();
  }

  private arm(): void {
    if (this.watchdog !== null) this.timers.clearTimeout(this.watchdog);
    this.watchdog = this.timers.setTimeout(() => {
      this.watchdog = null;
      this.link?.close();
    }, STREAM_MUX_BOUNDS_V1.deadAfterMs);
  }

  private lost(): void {
    this.link = null;
    this.linkOpen = false;
    if (this.watchdog !== null) this.timers.clearTimeout(this.watchdog);
    this.watchdog = null;
    if (this.streams.size === 0) return;
    const ceiling = Math.min(STREAM_MUX_BOUNDS_V1.reconnectMaxMs, STREAM_MUX_BOUNDS_V1.reconnectMinMs * 2 ** Math.min(this.attempt, 16));
    this.attempt += 1;
    const delay = STREAM_MUX_BOUNDS_V1.reconnectMinMs + Math.random() * Math.max(0, ceiling - STREAM_MUX_BOUNDS_V1.reconnectMinMs);
    this.reconnect = this.timers.setTimeout(() => {
      this.reconnect = null;
      if (this.streams.size > 0) this.ensureLink();
    }, delay);
  }

  private shut(): void {
    if (this.reconnect !== null) this.timers.clearTimeout(this.reconnect);
    this.reconnect = null;
    if (this.watchdog !== null) this.timers.clearTimeout(this.watchdog);
    this.watchdog = null;
    const link = this.link;
    this.link = null;
    this.linkOpen = false;
    link?.close();
  }
}

/** @emoji 🧷️ A worker's endpoint onto the page channel through a transferred `MessagePort` (see {@link StreamMuxChannelV1.attachPort}). */
export class StreamMuxPortEndpointV1 implements StreamMuxEndpointV1 {
  private readonly port: MessagePort;
  private readonly readers = new Map<number, ReaderCore>();
  private nextId = 1;

  constructor(port: MessagePort) {
    this.port = port;
    port.onmessage = (event: MessageEvent) => {
      if (typeof event.data !== "string") return;
      const decoded = decodeStreamMuxServerFrameV1(event.data);
      if (!decoded.ok || decoded.frame.kind === "hello" || decoded.frame.kind === "beat") return;
      const reader = this.readers.get(decoded.frame.stream);
      if (reader === undefined) return;
      if (decoded.frame.kind === "end") this.readers.delete(decoded.frame.stream);
      reader.deliver(decoded.frame);
    };
    port.start();
  }

  open(route: string, key: string, handlers: StreamMuxHandlersV1, options: { readonly credit?: number; readonly signal?: AbortSignal } = {}): StreamMuxSubscriptionV1 {
    const id = this.nextId++;
    const window = creditOf(options.credit);
    const reader = new ReaderCore(handlers, window, (credit) => {
      if (this.readers.get(id) === reader) this.port.postMessage(encodeStreamMuxFrameV1({ kind: "grant", stream: id, credit }));
    });
    this.readers.set(id, reader);
    this.port.postMessage(encodeStreamMuxFrameV1({ kind: "open", stream: id, route, key, resume: null, credit: window }));
    const close = (): void => {
      if (this.readers.get(id) !== reader) return;
      reader.done = true;
      this.readers.delete(id);
      this.port.postMessage(encodeStreamMuxFrameV1({ kind: "cancel", stream: id }));
    };
    if (options.signal !== undefined) {
      if (options.signal.aborted) close();
      else options.signal.addEventListener("abort", close, { once: true });
    }
    return { close };
  }
}

const pageChannels = new Map<string, StreamMuxChannelV1>();

/** @emoji 📄️ The page's channel to its own origin at `path` (one per path per page); a runtime without `WebSocket` or a page
 * location gets {@link SILENT_STREAM_MUX_ENDPOINT_V1}. */
export function pageStreamMuxChannelV1(path: string): StreamMuxChannelV1 | typeof SILENT_STREAM_MUX_ENDPOINT_V1 {
  const location = (globalThis as { readonly location?: { readonly protocol: string; readonly host: string } }).location;
  if (typeof WebSocket === "undefined" || location === undefined || !/^https?:$/u.test(location.protocol)) return SILENT_STREAM_MUX_ENDPOINT_V1;
  const url = `${location.protocol === "https:" ? "wss:" : "ws:"}//${location.host}${path}`;
  let channel = pageChannels.get(url);
  if (channel === undefined) {
    channel = new StreamMuxChannelV1(webSocketStreamMuxTransportV1(url));
    pageChannels.set(url, channel);
  }
  return channel;
}
//#endregion 🛰️Channel
