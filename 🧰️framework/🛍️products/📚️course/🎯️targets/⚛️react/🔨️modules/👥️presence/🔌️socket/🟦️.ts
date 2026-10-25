/** 🔁️ The presence socket of the quiz client: the frame rates and timings, the browser socket behind a port, and the room that joins, rejoins and leaves with jittered backoff. */

import { retryWithJitteredBackoff } from "@semio-tech/framework";
import { PRESENCE_PROTOCOL, decodePresenceFrame, encodePresenceFrame, type PresenceEntry as SocketEntry } from "@semio-tech/framework-server";
import { RETRY_TIMING, type RetryTiming } from "../../🛂️proctor/🟦️.ts";

//#region 🔌️Socket
/** 🎞️ The most state frames a room sends per second (design §15: pointer frames ≤ 15 Hz). */
export const PRESENCE_FRAME_HZ = 15;

/** ⏱️ The least time between two state frames of one room. */
export const PRESENCE_FRAME_INTERVAL_MS = Math.ceil(1000 / PRESENCE_FRAME_HZ);

/** 💭️ The least time between two thinking frames (design §17: drafts ≤ 2 Hz, coalesced). */
export const THINKING_FRAME_INTERVAL_MS = 500;

/** ⌚️ How often the watched rooms report their changes (design §17: about 4 Hz). */
export const WATCH_INTERVAL_MS = 250;

/** 🔢️ The most rooms one socket watches (the framework's bound). */
export const MAX_WATCHED_ROOMS = 16;

/** ⚓️ How long a joined session must last to count as stable; one that ends sooner is a flap. */
export const PRESENCE_STABLE_MS = 10_000;

/** ⏳️ The pause before a room joins again after `flaps` short-lived sessions in a row (`random` in [0, 1]): a first loss
 * is a blip and is rejoined within the shortest backoff; from the second flap on the pause is at least that backoff
 * plus a random share of an exponentially growing ceiling, capped at the longest — so a proctor that admits and drops
 * sockets (a connection limit) is asked ever more rarely. */
export function rejoinDelay(flaps: number, timing: RetryTiming, random: number): number {
  if (flaps <= 1) return random * timing.minMs;
  return timing.minMs + random * (Math.min(timing.maxMs, timing.minMs * 2 ** (flaps - 1)) - timing.minMs);
}

/** 🕸️ What a presence room needs of a WebSocket; the browser's `WebSocket` is one. */
export interface PresenceSocket {
  readonly readyState: number;
  onopen: ((event: Event) => void) | null;
  onmessage: ((event: MessageEvent) => void) | null;
  onclose: ((event: CloseEvent) => void) | null;
  onerror: ((event: Event) => void) | null;
  send(data: string): void;
  close(code?: number, reason?: string): void;
}

/** 🔌️ Opens a presence socket at `url` speaking `protocol`. */
export type PresenceConnect = (url: string, protocol: string) => PresenceSocket;

/** 🌐️ The browser's WebSocket. */
export const browserPresenceConnect: PresenceConnect = (url, protocol) => new WebSocket(url, protocol);

const CONNECTING = 0;
const OPEN = 1;

/** 👋️ Closes a socket that is left: at once when it is open, else as soon as it opens — a socket closed while it still
 * connects is reported by the browser as a failed connection, though nothing failed. */
function leave(socket: PresenceSocket | undefined): void {
  if (socket === undefined) return;
  if (socket.readyState === CONNECTING) socket.onopen = () => socket.close(1000, "left");
  else socket.close(1000, "left");
}

/** 🚦️ Where a room is: joining, joined, waiting to join again after a loss, or left. */
export type RoomStatus = "connecting" | "open" | "waiting" | "stopped";

/** 🧍️ One session of a room with an admitted state. */
export interface RoomMember<S> {
  readonly session: string;
  readonly colour: number;
  readonly surface: string;
  readonly state: S;
}

/** 📸️ What a room knows: its status, the own session and palette slot once joined, the members, the members of every
 * watched room by scope, and the last refusal. */
export interface RoomSnapshot<S, W = never> {
  readonly status: RoomStatus;
  readonly self: string | undefined;
  readonly colour: number | undefined;
  readonly members: readonly RoomMember<S>[];
  readonly watched: ReadonlyMap<string, readonly RoomMember<W>[]>;
  readonly refused: string | undefined;
}

/** ⚙️ One room: its socket URL, the socket factory, the checks that admit a member's state (of the joined room, and of
 * a watched room by scope), the frame interval, the backoff bounds and the random source of the jitter. */
export interface PresenceRoomOptions<S, W = never> {
  readonly url: string;
  readonly connect: PresenceConnect;
  readonly parse: (state: unknown) => S | undefined;
  readonly parseWatched?: (scope: string, state: unknown) => W | undefined;
  readonly intervalMs?: number;
  readonly timing?: RetryTiming;
  readonly random?: () => number;
}

function pause(ms: number, signal: AbortSignal): Promise<void> {
  return new Promise((resolve) => {
    const timer = setTimeout(done, ms);
    signal.addEventListener("abort", done, { once: true });
    function done(): void {
      clearTimeout(timer);
      signal.removeEventListener("abort", done);
      resolve();
    }
  });
}

/** 🚪️ One presence room over one socket: joins on {@link start}, keeps the admitted members of the latest `welcome` and
 * every `batch`, sends the latest {@link publish}ed state at most once per interval, watches the rooms of
 * {@link watch} (their `watched` frames: a snapshot replaces a room, else changes apply), sends state and watch again
 * after every rejoin, rejoins a lost socket with jittered backoff and leaves on {@link stop}. */
export class PresenceRoom<S, W = never> {
  private readonly listeners = new Set<() => void>();
  private snapshot: RoomSnapshot<S, W> = { status: "stopped", self: undefined, colour: undefined, members: [], watched: new Map(), refused: undefined };
  private members = new Map<string, RoomMember<S>>();
  private watchedMembers = new Map<string, Map<string, RoomMember<W>>>();
  private socket: PresenceSocket | undefined;
  private self: string | undefined;
  private colour: number | undefined;
  private refused: string | undefined;
  private welcomed = false;
  private desired: string | undefined;
  private sent: string | undefined;
  private sentAt = Number.NEGATIVE_INFINITY;
  private watching: { readonly scopes: readonly string[]; readonly frame: string } | undefined;
  private watchSent: string | undefined;
  private flushTimer: ReturnType<typeof setTimeout> | undefined;
  private lifetime: AbortController | undefined;

  constructor(private readonly options: PresenceRoomOptions<S, W>) {}

  readonly subscribe = (listener: () => void): (() => void) => {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  };

  readonly getSnapshot = (): RoomSnapshot<S, W> => this.snapshot;

  /** ▶️ Joins the room, and keeps joining until {@link stop}. */
  start(): void {
    if (this.lifetime !== undefined) return;
    const lifetime = new AbortController();
    this.lifetime = lifetime;
    this.emit("connecting");
    void this.run(lifetime.signal).catch(() => undefined);
  }

  /** 🛑️ Leaves the room: closes the socket, drops the members and forgets what was sent. */
  stop(): void {
    this.lifetime?.abort();
    this.lifetime = undefined;
    clearTimeout(this.flushTimer);
    this.flushTimer = undefined;
    leave(this.detach());
    this.members = new Map();
    this.watchedMembers = new Map();
    this.self = undefined;
    this.colour = undefined;
    this.emit("stopped");
  }

  /** 📤️ Shares `state`, replacing any state not sent yet. */
  publish(state: S): void {
    this.desired = JSON.stringify(encodePresenceFrame({ type: "state", state }));
    this.schedule();
  }

  /** 👀️ Watches the rooms of `scopes` read-only (at most {@link MAX_WATCHED_ROOMS}), their changes every `intervalMs`;
   * rooms no longer watched are forgotten. */
  watch(scopes: readonly string[], intervalMs: number = WATCH_INTERVAL_MS): void {
    const kept = [...new Set(scopes)].slice(0, MAX_WATCHED_ROOMS);
    if (kept.length === 0 && this.watching === undefined) return;
    const frame = JSON.stringify(encodePresenceFrame({ type: "watch", scopes: kept, intervalMs }));
    if (frame === this.watching?.frame) return;
    this.watching = { scopes: kept, frame };
    const before = this.watchedMembers.size;
    for (const scope of [...this.watchedMembers.keys()]) if (!kept.includes(scope)) this.watchedMembers.delete(scope);
    this.sendWatch();
    if (this.watchedMembers.size !== before) this.emit(this.snapshot.status);
  }

  private async run(signal: AbortSignal): Promise<void> {
    const timing = this.options.timing ?? RETRY_TIMING;
    let flaps = 0;
    while (!signal.aborted) {
      const joined = await retryWithJitteredBackoff(() => this.join(signal), { ...timing, signal });
      const since = Date.now();
      await joined.ended;
      if (signal.aborted) return;
      flaps = Date.now() - since < PRESENCE_STABLE_MS ? flaps + 1 : 0;
      await pause(rejoinDelay(flaps, timing, (this.options.random ?? Math.random)()), signal);
    }
  }

  private join(signal: AbortSignal): Promise<{ readonly ended: Promise<void> }> {
    return new Promise((resolve, reject) => {
      if (signal.aborted) {
        reject(signal.reason);
        return;
      }
      const socket = this.options.connect(this.options.url, PRESENCE_PROTOCOL);
      let end: () => void = () => undefined;
      const ended = new Promise<void>((done) => (end = done));
      signal.addEventListener("abort", () => (reject(signal.reason), end()), { once: true });
      this.socket = socket;
      socket.onmessage = (event) => {
        if (this.receive(socket, event.data) === "welcome") resolve({ ended });
      };
      socket.onclose = () => {
        if (this.socket === socket) this.lost();
        reject(new Error("presence socket closed"));
        end();
      };
      socket.onerror = () => undefined;
    });
  }

  private receive(socket: PresenceSocket, data: unknown): string | undefined {
    if (socket !== this.socket || typeof data !== "string") return undefined;
    let frame: ReturnType<typeof decodePresenceFrame>;
    try {
      frame = decodePresenceFrame(JSON.parse(data));
    } catch {
      return undefined;
    }
    switch (frame.type) {
      case "welcome":
        this.welcomed = true;
        this.self = frame.session;
        this.colour = frame.colour;
        this.refused = undefined;
        this.members = new Map();
        for (const entry of frame.roster) this.upsert(entry);
        this.emit("open");
        this.schedule();
        this.sendWatch();
        break;
      case "batch":
        for (const session of frame.left) this.members.delete(session);
        for (const entry of frame.entries) this.upsert(entry);
        this.emit("open");
        break;
      case "watched":
        this.receiveWatched(frame.scope, frame.entries, frame.left, frame.snapshot === true);
        break;
      case "refused":
        this.refused = frame.reason;
        this.emit("open");
        break;
      case "state":
      case "watch":
        break;
    }
    return frame.type;
  }

  private receiveWatched(scope: string, entries: readonly SocketEntry[], left: readonly string[], snapshot: boolean): void {
    const parse = this.options.parseWatched;
    if (parse === undefined || !this.watching?.scopes.includes(scope)) return;
    const room = snapshot ? new Map<string, RoomMember<W>>() : new Map(this.watchedMembers.get(scope) ?? []);
    for (const session of left) room.delete(session);
    for (const entry of entries) {
      const state = parse(scope, entry.state);
      if (state === undefined) room.delete(entry.session);
      else room.set(entry.session, { session: entry.session, colour: entry.colour, surface: entry.surface, state });
    }
    this.watchedMembers.set(scope, room);
    this.emit(this.snapshot.status);
  }

  private upsert(entry: SocketEntry): void {
    const state = this.options.parse(entry.state);
    if (state === undefined) this.members.delete(entry.session);
    else this.members.set(entry.session, { session: entry.session, colour: entry.colour, surface: entry.surface, state });
  }

  private lost(): void {
    this.detach();
    this.members = new Map();
    this.watchedMembers = new Map();
    this.emit("waiting");
  }

  private detach(): PresenceSocket | undefined {
    const socket = this.socket;
    this.socket = undefined;
    this.welcomed = false;
    this.sent = undefined;
    this.watchSent = undefined;
    clearTimeout(this.flushTimer);
    this.flushTimer = undefined;
    if (socket !== undefined) {
      socket.onopen = null;
      socket.onmessage = null;
      socket.onclose = null;
      socket.onerror = null;
    }
    return socket;
  }

  private sendWatch(): void {
    const socket = this.socket;
    const frame = this.watching?.frame;
    if (socket === undefined || socket.readyState !== OPEN || !this.welcomed || frame === undefined || frame === this.watchSent) return;
    socket.send(frame);
    this.watchSent = frame;
  }

  private schedule(): void {
    if (this.flushTimer !== undefined) return;
    const wait = this.sentAt + (this.options.intervalMs ?? PRESENCE_FRAME_INTERVAL_MS) - Date.now();
    if (wait <= 0) this.flush();
    else
      this.flushTimer = setTimeout(() => {
        this.flushTimer = undefined;
        this.flush();
      }, wait);
  }

  private flush(): void {
    const socket = this.socket;
    if (socket === undefined || socket.readyState !== OPEN || !this.welcomed || this.desired === undefined || this.desired === this.sent) return;
    socket.send(this.desired);
    this.sent = this.desired;
    this.sentAt = Date.now();
  }

  private emit(status: RoomStatus): void {
    const watched = new Map([...this.watchedMembers.entries()].map(([scope, room]) => [scope, [...room.values()]] as const));
    this.snapshot = { status, self: this.self, colour: this.colour, members: [...this.members.values()], watched, refused: this.refused };
    for (const listener of this.listeners) listener();
  }
}
//#endregion 🔌️Socket
