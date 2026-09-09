import { parseBrowserBrokerPortRequestV1, parseBrowserBrokerPortResponseV1, type BrowserBrokerPortRequestV1 } from "../../../../🟦️.ts";

export const BROWSER_BROKER_CLIENT_TIMEOUT_MS = 2_000;
export const BROWSER_BROKER_CLIENT_INITIALIZATION_TIMEOUT_MS = 90_000;
export const BROWSER_BROKER_CLIENT_MAX_PENDING = 64;

type PendingBrowserBrokerRequestV1 = {
  readonly resolve: (value: { status: number; body: string }) => void;
  readonly reject: (error: Error) => void;
  timer: ReturnType<typeof setTimeout> | null;
  readonly signal?: AbortSignal;
  readonly abort: () => void;
};

/** 🤝️ Admits bounded session reads only after the private worker port acknowledges its proof. */
export class BrowserBrokerPortClientV1 {
  private phase: "initializing" | "ready" | "closed" = "initializing";
  private readonly pending = new Map<string, PendingBrowserBrokerRequestV1>();
  private initializationTimer: ReturnType<typeof setTimeout> | null = null;

  constructor(private readonly port: MessagePort, proof?: string) {
    port.onmessage = (event: MessageEvent<unknown>) => {
      const message = parseBrowserBrokerPortResponseV1(event.data);
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
    const initialize = parseBrowserBrokerPortRequestV1({ kind: "initialize", proof });
    if (initialize === undefined) {
      this.close();
      return;
    }
    this.initializationTimer = setTimeout(() => this.close(), BROWSER_BROKER_CLIENT_INITIALIZATION_TIMEOUT_MS);
    this.post(initialize);
  }

  me(signal?: AbortSignal): Promise<{ status: number; body: string }> {
    if (this.phase === "closed" || signal?.aborted) return Promise.reject(new Error("browser broker unavailable"));
    if (this.pending.size >= BROWSER_BROKER_CLIENT_MAX_PENDING) return Promise.reject(new Error("browser broker capacity exceeded"));
    const requestId = crypto.randomUUID();
    return new Promise((resolve, reject) => {
      const abort = (): void => {
        const pending = this.take(requestId);
        if (!pending) return;
        if (this.phase === "ready") this.post({ kind: "cancel", requestId });
        pending.reject(new Error("browser broker cancelled"));
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
    try { this.port.postMessage({ kind: "close" } satisfies BrowserBrokerPortRequestV1); } catch {}
    for (const requestId of this.pending.keys()) {
      const pending = this.take(requestId);
      if (ready) {
        try { this.port.postMessage({ kind: "cancel", requestId } satisfies BrowserBrokerPortRequestV1); } catch {}
      }
      pending?.reject(new Error("browser broker closed"));
    }
    this.port.close();
  }

  private take(requestId: string): PendingBrowserBrokerRequestV1 | undefined {
    const pending = this.pending.get(requestId);
    if (!pending) return undefined;
    this.pending.delete(requestId);
    if (pending.timer !== null) clearTimeout(pending.timer);
    pending.signal?.removeEventListener("abort", pending.abort);
    return pending;
  }

  private post(message: BrowserBrokerPortRequestV1): void {
    if (this.phase === "closed") return;
    try { this.port.postMessage(message); } catch { this.close(); }
  }

  private dispatch(requestId: string): void {
    const pending = this.pending.get(requestId);
    if (this.phase !== "ready" || pending === undefined || pending.timer !== null) return;
    pending.timer = setTimeout(pending.abort, BROWSER_BROKER_CLIENT_TIMEOUT_MS);
    this.post({ kind: "request", requestId, operation: "me" });
  }
}
