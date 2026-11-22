/** 🧵️ The flow solver behind a message protocol: it keeps the assets and sessions a client opened, develops the transport of a session to the time a frame asks for in cooperative slices, reports progress, stops when cancelled and answers with the flow field. The same class runs in the flow worker and, where workers are missing, in the page.
 *
 * A session answers one frame at a time; a frame that arrives while another is solved replaces any frame still waiting, so the solver never works on a time the reader has already left. Going forward is incremental (the solver keeps its parcels); a time before the last one is solved afresh from one parcel life earlier.
 *
 * @see ../📡️session/🟦️.ts — the client
 * @see ../../../../../🔨️modules/💨️transport/🟦️.ts — `developTransport`
 */
import { developTransport } from "@teaching/staffage";
import type { Asset, Environment, EvaluationQuery, FlowField } from "@teaching/staffage";

/** 📬️ What a client sends. */
export type FlowMessage =
  | { readonly kind: "asset"; readonly id: number; readonly asset: Asset }
  | { readonly kind: "release"; readonly id: number }
  | { readonly kind: "open"; readonly session: number; readonly asset: number; readonly query: EvaluationQuery; readonly environment: Environment; readonly maxAgeS: number; readonly periodS: number }
  | { readonly kind: "frame"; readonly session: number; readonly timeS: number }
  | { readonly kind: "cancel"; readonly session: number }
  | { readonly kind: "close"; readonly session: number };

/** 📨️ What the solver answers. */
export type FlowReply =
  | { readonly session: number; readonly kind: "progress"; readonly done: number; readonly total: number }
  | { readonly session: number; readonly kind: "flow"; readonly timeS: number; readonly flow: FlowField }
  | { readonly session: number; readonly kind: "cancelled" }
  | { readonly session: number; readonly kind: "error"; readonly message: string };

type Session = { readonly asset: Asset; readonly query: EvaluationQuery; readonly environment: Environment; readonly maxAgeS: number; readonly periodS: number; lastS: number | null; wanted: number | null; busy: boolean; abort: AbortController; closed: boolean };

const macrotask = (): Promise<void> => new Promise((resolve) => setTimeout(resolve, 0));

/** 🧮️ The solver side of the protocol; `reply` carries its answers. */
export class FlowServer {
  readonly #assets = new Map<number, Asset>();
  readonly #sessions = new Map<number, Session>();

  constructor(private readonly reply: (message: FlowReply) => void) {}

  /** 📥️ Handles one message of a client. */
  post(message: FlowMessage): void {
    if (message.kind === "asset") return void this.#assets.set(message.id, message.asset);
    if (message.kind === "release") return void this.#assets.delete(message.id);
    if (message.kind === "open") {
      const asset = this.#assets.get(message.asset);
      if (asset === undefined) return this.reply({ session: message.session, kind: "error", message: "The flow solver does not hold the requested asset" });
      return void this.#sessions.set(message.session, { asset, query: message.query, environment: message.environment, maxAgeS: message.maxAgeS, periodS: message.periodS, lastS: null, wanted: null, busy: false, abort: new AbortController(), closed: false });
    }
    const session = this.#sessions.get(message.session);
    if (session === undefined) return;
    if (message.kind === "frame") {
      session.wanted = message.timeS;
      if (!session.busy) void this.#run(message.session, session);
    } else {
      session.wanted = null;
      session.abort.abort(new DOMException("flow solve cancelled", "AbortError"));
      session.abort = new AbortController();
      if (message.kind === "close") {
        session.closed = true;
        this.#sessions.delete(message.session);
      } else if (!session.busy) this.reply({ session: message.session, kind: "cancelled" });
    }
  }

  async #run(id: number, session: Session): Promise<void> {
    session.busy = true;
    while (session.wanted !== null && !session.closed) {
      const timeS = session.wanted;
      session.wanted = null;
      const signal = session.abort.signal;
      const resume = session.lastS !== null && timeS >= session.lastS && timeS - session.lastS <= session.maxAgeS;
      try {
        const field = await developTransport(session.asset, { ...session.query, timeS } as EvaluationQuery, session.environment, { maxAgeS: session.maxAgeS, ...(session.periodS > 0 ? { periodS: session.periodS } : {}), ...(resume ? { fromS: session.lastS! } : {}), flow: true, pathlines: false, streamlines: false, signal, progress: (done, total) => total > 4 && this.reply({ session: id, kind: "progress", done, total }) });
        session.lastS = timeS;
        if (!session.closed && field.flow !== null) this.reply({ session: id, kind: "flow", timeS, flow: field.flow });
      } catch (error) {
        if (session.closed) break;
        if (signal.aborted) this.reply({ session: id, kind: "cancelled" });
        else this.reply({ session: id, kind: "error", message: error instanceof Error ? error.message : String(error) });
      }
      await macrotask();
    }
    session.busy = false;
  }
}
