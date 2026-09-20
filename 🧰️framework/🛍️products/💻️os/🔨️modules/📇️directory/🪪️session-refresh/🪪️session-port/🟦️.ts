import { parseHubSessionPortRequestV1, parseHubSessionPortResponseV1, type HubSessionPortRequestV1 } from "../../../../🟦️.ts";

export const HUB_SESSION_PORT_CLIENT_TIMEOUT_MS = 2_000;
export const HUB_SESSION_PORT_CLIENT_INITIALIZATION_TIMEOUT_MS = 90_000;
export const HUB_SESSION_PORT_CLIENT_MAX_PENDING = 64;

type PendingHubSessionRequestV1 = {
  readonly resolve: (value: { status: number; body: string }) => void;
  readonly reject: (error: Error) => void;
  timer: ReturnType<typeof setTimeout> | null;
  readonly signal?: AbortSignal;
  readonly abort: () => void;
};

/** 🪪️ Hands the credential-owning backbone worker the human's own hub session capability and reads
 * `GET /auth/sessions/me` back through it. The worker is the only holder of the lane the capability
 * authorizes, so the authority this returns is the hub's own statement about the session the WORKER
 * can act with — which is exactly the predicate the shell gates hub-authenticated work on. */
export class HubSessionPortClientV1 {
  private phase: "initializing" | "ready" | "closed" = "initializing";
  private readonly pending = new Map<string, PendingHubSessionRequestV1>();
  private initializationTimer: ReturnType<typeof setTimeout> | null = null;

  constructor(private readonly port: MessagePort, capability: string) {
    port.onmessage = (event: MessageEvent<unknown>) => {
      const message = parseHubSessionPortResponseV1(event.data);
      if (!message || this.phase === "closed") return;
      if (message.kind === "initialized") {
        if (this.phase !== "initializing" || !message.ok) return this.close();
        this.phase = "ready";
        if (this.initializationTimer !== null) clearTimeout(this.initializationTimer);
        this.initializationTimer = null;
        for (const requestId of this.pending.keys()) this.dispatch(requestId);
        return;
      }
      if (this.phase !== "ready") return this.close();
      const pending = this.take(message.requestId);
      pending?.resolve({ status: message.status, body: message.body });
    };
    port.onmessageerror = () => this.close();
    port.start();
    const initialize = parseHubSessionPortRequestV1({ kind: "initialize", capability });
    if (initialize === undefined) {
      this.close();
      return;
    }
    this.initializationTimer = setTimeout(() => this.close(), HUB_SESSION_PORT_CLIENT_INITIALIZATION_TIMEOUT_MS);
    this.post(initialize);
  }

  me(signal?: AbortSignal): Promise<{ status: number; body: string }> {
    if (this.phase === "closed" || signal?.aborted) return Promise.reject(new Error("hub session unavailable"));
    if (this.pending.size >= HUB_SESSION_PORT_CLIENT_MAX_PENDING) return Promise.reject(new Error("hub session capacity exceeded"));
    const requestId = crypto.randomUUID();
    return new Promise((resolve, reject) => {
      const abort = (): void => {
        const pending = this.take(requestId);
        if (!pending) return;
        if (this.phase === "ready") this.post({ kind: "cancel", requestId });
        pending.reject(new Error("hub session cancelled"));
      };
      this.pending.set(requestId, { resolve, reject, timer: null, signal, abort });
      signal?.addEventListener("abort", abort, { once: true });
      this.dispatch(requestId);
    });
  }

  close(): void {
    if (this.phase === "closed") return;
    const ready = this.phase === "ready";
    this.phase = "closed";
    if (this.initializationTimer !== null) clearTimeout(this.initializationTimer);
    this.initializationTimer = null;
    this.port.onmessage = null;
    this.port.onmessageerror = null;
    try { this.port.postMessage({ kind: "close" } satisfies HubSessionPortRequestV1); } catch {}
    for (const requestId of this.pending.keys()) {
      const pending = this.take(requestId);
      if (ready) {
        try { this.port.postMessage({ kind: "cancel", requestId } satisfies HubSessionPortRequestV1); } catch {}
      }
      pending?.reject(new Error("hub session closed"));
    }
    this.port.close();
  }

  private take(requestId: string): PendingHubSessionRequestV1 | undefined {
    const pending = this.pending.get(requestId);
    if (!pending) return undefined;
    this.pending.delete(requestId);
    if (pending.timer !== null) clearTimeout(pending.timer);
    pending.signal?.removeEventListener("abort", pending.abort);
    return pending;
  }

  private post(message: HubSessionPortRequestV1): void {
    if (this.phase === "closed") return;
    try { this.port.postMessage(message); } catch { this.close(); }
  }

  private dispatch(requestId: string): void {
    const pending = this.pending.get(requestId);
    if (this.phase !== "ready" || pending === undefined || pending.timer !== null) return;
    pending.timer = setTimeout(pending.abort, HUB_SESSION_PORT_CLIENT_TIMEOUT_MS);
    this.post({ kind: "request", requestId, operation: "me" });
  }
}
