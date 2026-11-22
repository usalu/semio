/** 📡️ `FlowSession`: the client of the flow solver for one figure. It opens a session for an asset, a pose or clip and an environment, asks for the field of the time its clock shows, and tells its owner what is going on: developing (with a fraction and a cancel), ready, cancelled or failed.
 *
 * The solver runs in one shared worker of the page (the asset is posted once and released with its last session); where workers are missing the same solver runs in the page in cooperative slices. Requests are back-pressured: while a frame is being solved only the latest later time is kept.
 *
 * @see ../🧵️server/🟦️.ts — the protocol and the solver
 * @see ../🟦️.ts — the time rules
 */
import type { Asset, Environment, EvaluationQuery, FlowField } from "@teaching/staffage";
import { FLOW_DEVELOP_S, FLOW_REFRESH_HZ, flowPeriodS, flowTimeS } from "../🟦️.ts";
import { FlowServer, type FlowMessage, type FlowReply } from "../🧵️server/🟦️.ts";

/** 🧾️ Where a flow session is. */
export type FlowStatus = { readonly phase: "idle" | "developing" | "ready" | "cancelled" | "error"; readonly fraction: number; readonly message: string };

type Port = { readonly post: (message: FlowMessage) => void; readonly listeners: Map<number, (reply: FlowReply) => void>; readonly assets: WeakMap<Asset, { readonly id: number; count: number }> };

let port: Port | null = null;
let sequence = 0;

function portOf(): Port {
  if (port !== null) return port;
  const listeners = new Map<number, (reply: FlowReply) => void>();
  const receive = (reply: FlowReply): void => listeners.get(reply.session)?.(reply);
  let post: (message: FlowMessage) => void;
  try {
    if (typeof Worker !== "function") throw new Error("no worker");
    const worker = new Worker(new URL("../🧵️server/worker.ts", import.meta.url), { type: "module" });
    worker.onmessage = ({ data }: MessageEvent<FlowReply>) => receive(data);
    post = (message) => worker.postMessage(message);
  } catch {
    const server = new FlowServer((reply) => queueMicrotask(() => receive(reply)));
    post = (message) => server.post(message);
  }
  return (port = { post, listeners, assets: new WeakMap() });
}

/** 📡️ One figure's connection to the flow solver. */
export class FlowSession {
  #status: FlowStatus = { phase: "idle", fraction: 0, message: "" };
  #flow: FlowField | null = null;
  #id = 0;
  #asset: Asset | null = null;
  #query: EvaluationQuery | null = null;
  #periodS = 0;
  #lastS: number | null = null;
  #sentS: number | null = null;
  #waiting = false;
  #queued: number | null = null;
  #elapsedS = 0;
  #clockS: number | null = null;
  #durationS: number | null = null;

  constructor(private readonly changed: () => void) {}

  /** 🧾️ The state of the session. */
  get status(): FlowStatus {
    return this.#status;
  }

  /** 🌊️ The latest field, `null` until the first one arrived. */
  get flow(): FlowField | null {
    return this.#flow;
  }

  /** ⏱️ Seconds the clock has run forward since the session opened: the time the tracers move by, continuous over a wrap of the clock. */
  get elapsedS(): number {
    return this.#elapsedS;
  }

  /** 🚪️ Opens (or reopens) the session for an asset at a pose or clip under an environment; the first {@link frame} starts the solve. */
  open(asset: Asset, query: EvaluationQuery, environment: Environment): void {
    this.close();
    const link = portOf();
    let reference = link.assets.get(asset);
    if (reference === undefined) {
      link.assets.set(asset, (reference = { id: ++sequence, count: 0 }));
      link.post({ kind: "asset", id: reference.id, asset });
    }
    reference.count++;
    this.#id = ++sequence;
    this.#asset = asset;
    this.#query = query;
    this.#periodS = flowPeriodS(asset, query);
    link.listeners.set(this.#id, (reply) => this.#receive(reply));
    link.post({ kind: "open", session: this.#id, asset: reference.id, query, environment, maxAgeS: FLOW_DEVELOP_S, periodS: this.#periodS });
    this.#set({ phase: "idle", fraction: 0, message: "" });
  }

  /** 🕰️ Shows the clock time `clockS` (of a clock that wraps at `durationS` when it loops): advances the tracer time and asks for the field of that instant unless the same instant is already shown or asked for. */
  frame(clockS: number, durationS: number | null): void {
    if (this.#asset === null || this.#query === null || this.#status.phase === "cancelled" || this.#status.phase === "error") return;
    if (this.#clockS !== null) {
      let delta = clockS - this.#clockS;
      const wrap = durationS ?? this.#durationS;
      if (wrap !== null && wrap > 0 && delta < -wrap / 2) delta += wrap;
      if (delta > 0) this.#elapsedS += delta;
    }
    this.#clockS = clockS;
    this.#durationS = durationS;
    const quantised = Math.round(clockS * FLOW_REFRESH_HZ) / FLOW_REFRESH_HZ;
    const elapsed = Math.round(this.#elapsedS * FLOW_REFRESH_HZ) / FLOW_REFRESH_HZ;
    const timeS = flowTimeS(this.#query, this.#periodS, quantised, elapsed, this.#lastS);
    if (timeS === this.#sentS) return;
    if (this.#waiting) return void (this.#queued = timeS);
    this.#send(timeS);
  }

  /** ✋️ Stops the running solve; the session stays cancelled until {@link retry}. */
  cancel(): void {
    if (this.#id === 0) return;
    this.#queued = null;
    this.#waiting = false;
    portOf().post({ kind: "cancel", session: this.#id });
    this.#set({ phase: "cancelled", fraction: 0, message: "" });
  }

  /** 🔁️ Solves again after a cancellation or a failure. */
  retry(): void {
    if (this.#status.phase !== "cancelled" && this.#status.phase !== "error") return;
    this.#sentS = null;
    this.#set({ phase: "idle", fraction: 0, message: "" });
    if (this.#clockS !== null) this.frame(this.#clockS, this.#durationS);
  }

  /** 🧹️ Closes the session and forgets its field. */
  close(): void {
    if (this.#id === 0) return;
    const link = portOf();
    link.post({ kind: "close", session: this.#id });
    link.listeners.delete(this.#id);
    const reference = this.#asset === null ? undefined : link.assets.get(this.#asset);
    if (reference !== undefined && --reference.count === 0) {
      link.assets.delete(this.#asset!);
      link.post({ kind: "release", id: reference.id });
    }
    this.#id = 0;
    this.#asset = null;
    this.#flow = null;
    this.#lastS = this.#sentS = this.#queued = this.#clockS = null;
    this.#waiting = false;
    this.#elapsedS = 0;
  }

  #send(timeS: number): void {
    this.#sentS = timeS;
    this.#waiting = true;
    if (this.#flow === null) this.#set({ phase: "developing", fraction: 0, message: "" });
    portOf().post({ kind: "frame", session: this.#id, timeS });
  }

  #receive(reply: FlowReply): void {
    if (reply.kind === "progress") {
      if (this.#status.phase !== "cancelled") this.#set({ phase: "developing", fraction: reply.done / reply.total, message: "" });
      return;
    }
    this.#waiting = false;
    if (reply.kind === "cancelled") return;
    if (reply.kind === "error") return this.#set({ phase: "error", fraction: 0, message: reply.message });
    if (this.#status.phase === "cancelled") return;
    this.#flow = reply.flow;
    this.#lastS = reply.timeS;
    this.#set({ phase: "ready", fraction: 1, message: "" });
    const queued = this.#queued;
    this.#queued = null;
    if (queued !== null && queued !== reply.timeS) this.#send(queued);
  }

  #set(status: FlowStatus): void {
    this.#status = status;
    this.changed();
  }
}
