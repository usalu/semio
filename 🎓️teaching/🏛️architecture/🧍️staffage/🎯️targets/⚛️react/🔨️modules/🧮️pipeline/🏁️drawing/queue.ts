import type { Asset, DepictionOptions, EvaluationQuery } from "@teaching/staffage";
import { drawFigure, type Drawing, type DrawingReply, type DrawingRequest } from "./🟦️.ts";

type AssetReference = { readonly id: number; count: number };
type Lease = { readonly asset: Asset; readonly reference: AssetReference; readonly options: DepictionOptions; readonly wrapLoops: boolean; readonly receive: (drawing: Drawing, query: EvaluationQuery) => void; readonly fail: (error: Error) => void; key: string };
type Work = { readonly query: EvaluationQuery; readonly lease: Lease; readonly id: number };
type DrawingPort = { postMessage(request: DrawingRequest): void; terminate(): void; onmessage: ((event: MessageEvent<DrawingReply>) => void) | null; onerror: ((event: ErrorEvent) => void) | null };

/** 🪢️ A mounted drawing target and its coalescing submission port. */
export type DrawingLease = { readonly request: (query: EvaluationQuery) => void; readonly cancelPending: () => void; readonly release: () => void };

/** 📭️ A single-worker mailbox. It keeps one latest pending request per target and lets each active reply paint before advancing the queue. */
export class DrawingQueue {
  #sequence = 0;
  #active: Work | null = null;
  #port: DrawingPort | null;
  readonly #leases = new Map<number, Lease>();
  readonly #assets = new WeakMap<Asset, AssetReference>();
  readonly #pending = new Map<number, Work>();

  constructor(port: DrawingPort, readonly empty: () => void) {
    this.#port = port;
    port.onmessage = ({ data }) => {
      const active = this.#active;
      if (active?.id !== data.lease) return;
      this.#active = null;
      if (this.#leases.has(data.lease)) {
        if ("drawing" in data) active.lease.receive(data.drawing, active.query);
        else active.lease.fail(new Error(data.error));
      }
      this.#flush();
    };
    port.onerror = () => {
      port.terminate();
      this.#port = null;
      if (this.#active && !this.#pending.has(this.#active.id)) this.#pending.set(this.#active.id, this.#active);
      this.#active = null;
      this.#flush();
    };
  }

  retain(asset: Asset, options: DepictionOptions, wrapLoops: boolean, receive: Lease["receive"], fail: Lease["fail"]): DrawingLease {
    let reference = this.#assets.get(asset);
    if (!reference) {
      this.#assets.set(asset, (reference = { id: ++this.#sequence, count: 0 }));
      this.#port?.postMessage({ kind: "asset", id: reference.id, asset });
    }
    reference.count++;
    const lease: Lease = { asset, reference, options, wrapLoops, receive, fail, key: "" };
    const id = ++this.#sequence;
    this.#leases.set(id, lease);
    return {
      request: query => {
        if (!this.#leases.has(id)) return;
        const key = JSON.stringify(query);
        if (lease.key === key) return;
        lease.key = key;
        this.#pending.set(id, { query, lease, id });
        this.#flush();
      },
      cancelPending: () => {
        if (this.#pending.delete(id)) lease.key = "";
      },
      release: () => {
        if (!this.#leases.delete(id)) return;
        this.#pending.delete(id);
        if (--reference.count === 0) {
          this.#assets.delete(asset);
          this.#port?.postMessage({ kind: "release", id: reference.id });
        }
        if (this.#leases.size === 0) {
          this.#port?.terminate();
          this.#port = null;
          this.#active = null;
          this.empty();
        }
      },
    };
  }

  #flush(): void {
    if (this.#active) return;
    for (const [id, work] of this.#pending) {
      this.#pending.delete(id);
      const { query, lease } = work;
      if (this.#port) {
        this.#active = work;
        this.#port.postMessage({ kind: "draw", id: lease.reference.id, lease: id, options: lease.options, query, wrapLoops: lease.wrapLoops });
        return;
      }
      try {
        lease.receive(drawFigure(lease.asset, lease.options, query, lease.wrapLoops), query);
      } catch (error) {
        lease.fail(error instanceof Error ? error : new Error(String(error)));
      }
    }
  }
}

let queue: DrawingQueue | null = null;

/** 🧵️ Retains a target on the browser's shared drawing worker. */
export function retainDrawing(asset: Asset, options: DepictionOptions, wrapLoops: boolean, receive: Lease["receive"], fail: Lease["fail"]): DrawingLease | null {
  if (typeof Worker !== "function") return null;
  if (queue === null) {
    try {
      const port = new Worker(new URL("./worker.ts", import.meta.url), { type: "module" });
      queue = new DrawingQueue(port, () => { queue = null; });
    } catch {
      return null;
    }
  }
  return queue.retain(asset, options, wrapLoops, receive, fail);
}
