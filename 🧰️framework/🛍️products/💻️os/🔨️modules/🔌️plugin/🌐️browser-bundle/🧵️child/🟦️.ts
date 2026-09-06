import { BROWSER_ACTOR_CHILD_LIMITS, BROWSER_ACTOR_CHILD_SCHEMA, childBindingMatches, childRecord, childSha256, measureChildValue, type BrowserActorChildBinding, type BrowserActorChildValue } from "./🧬️schema/🟦️.ts";
export { BROWSER_ACTOR_CHILD_LIMITS } from "./🧬️schema/🟦️.ts";
export type { BrowserActorChildValue } from "./🧬️schema/🟦️.ts";

type ChildAdmission = Readonly<{ actorId: string; activationGeneration: bigint; bundleSha256: string; bundleByteLength: number }>;
type ChildPhase = "booting" | "ready" | "loading" | "active" | "closed";
type Pending = { sequence: number; resolve(value: BrowserActorChildValue): void; reject(error: Error): void; result?: { value: BrowserActorChildValue; transfers: number } };
let reservedActors = 0, reservedBytes = 0;

/** 📊️ Reports intentional buffer reservations, not engine heap usage or a security quota. */
export function browserActorChildCapacity(): Readonly<{ actors: number; bytes: number }> {
  return Object.freeze({ actors: reservedActors, bytes: reservedBytes });
}

/** 🧷️ Reserves a dedicated fixed child before any caller fetch; this is not catalog or execution authorization. */
export async function reserveBrowserActorChild(admission: ChildAdmission, signal?: AbortSignal): Promise<BrowserActorChild> {
  if (!childRecord(admission, ["actorId", "activationGeneration", "bundleSha256", "bundleByteLength"]) || typeof admission.actorId !== "string" || admission.actorId.length < 1 || admission.actorId.length > 512 || /[\x00-\x1f\x7f]/.test(admission.actorId) || typeof admission.activationGeneration !== "bigint" || admission.activationGeneration < 1n || admission.activationGeneration > 0xffffffffffffffffn || typeof admission.bundleSha256 !== "string" || !/^[0-9a-f]{64}$/.test(admission.bundleSha256) || !Number.isSafeInteger(admission.bundleByteLength) || admission.bundleByteLength < 1 || admission.bundleByteLength > BROWSER_ACTOR_CHILD_LIMITS.actorBytes || signal?.aborted) throw new Error("browser actor child: admission denied");
  if (reservedActors >= BROWSER_ACTOR_CHILD_LIMITS.actors) throw new Error("browser actor child: capacity");
  const bytes = admission.bundleByteLength + 2 * BROWSER_ACTOR_CHILD_LIMITS.messageBytes + BROWSER_ACTOR_CHILD_LIMITS.outputBytes;
  reservedActors++; reservedBytes += bytes;
  let released = false;
  const release = () => { if (!released) { released = true; reservedActors--; reservedBytes -= bytes; } };
  let owner: BrowserActorChild | undefined;
  try {
    owner = new BrowserActorChild(Object.freeze({ ...admission }), release, signal);
    await owner.ready;
    return owner;
  } catch (error) { owner?.close(); release(); throw error; }
}

/** 🛟️ Owns one deadline-bounded child and port. No Hub origin, receipt, broker or host callbacks cross it. */
class BrowserActorChild {
  readonly ready: Promise<void>;
  private phase: ChildPhase = "booting";
  private worker: Worker | null = null;
  private port: MessagePort | null = null;
  private bootPort: MessagePort | null = null;
  private timer: ReturnType<typeof setTimeout> | undefined;
  private pending: Pending | undefined;
  private operation: { resolve(): void; reject(error: Error): void } | undefined;
  private sequence = 0;
  private sourceDetached = false;
  private source: ArrayBuffer | null = null;
  private resultTransfersDetached = 0;
  private binding: BrowserActorChildBinding;
  private readonly onAbort = () => this.close("cancelled");

  constructor(private readonly admission: ChildAdmission, private readonly release: () => void, private readonly signal?: AbortSignal) {
    this.binding = Object.freeze({ schema: BROWSER_ACTOR_CHILD_SCHEMA, nonce: crypto.randomUUID(), generation: admission.activationGeneration.toString() });
    this.ready = new Promise((resolve, reject) => { this.operation = { resolve, reject }; });
    void this.ready.catch(() => {});
    try {
      const channel = new MessageChannel();
      this.port = channel.port1; this.bootPort = channel.port2;
      this.port.onmessage = event => this.receive(event.data);
      this.port.onmessageerror = () => this.close("message decode");
      this.port.start();
      this.worker = new Worker(new URL("./🧵️worker.ts", import.meta.url), { type: "module" });
      this.worker.onerror = event => { event.preventDefault(); this.close("worker error"); };
      this.worker.onmessageerror = () => this.close("worker message decode");
      signal?.addEventListener("abort", this.onAbort, { once: true });
      this.deadline(BROWSER_ACTOR_CHILD_LIMITS.bootMs);
      this.worker.postMessage({ ...this.binding, kind: "init", actorId: admission.actorId, port: channel.port2 }, [channel.port2]);
      this.bootPort = null;
      if (signal?.aborted) this.close("cancelled");
    } catch (error) { this.close("worker construction"); throw error; }
  }

  progress() {
    return Object.freeze({ phase: this.phase, activeInvocations: this.pending ? 1 : 0, sourceDetached: this.sourceDetached, resultTransfersDetached: this.resultTransfersDetached });
  }

  /** 📥️ Consumes a shape-valid buffer synchronously, then owns verification, transfer and cancellation. */
  async load(bytes: ArrayBuffer): Promise<void> {
    if (this.phase !== "ready") throw new Error("browser actor child: not ready");
    if (!(bytes instanceof ArrayBuffer) || (bytes as ArrayBuffer & { resizable?: boolean }).resizable || bytes.byteLength !== this.admission.bundleByteLength) { this.close("load shape"); throw new Error("browser actor child: load shape"); }
    try { this.source = structuredClone(bytes, { transfer: [bytes] }); }
    catch (error) { this.close("source ownership"); throw error; }
    this.phase = "loading"; this.deadline(BROWSER_ACTOR_CHILD_LIMITS.loadMs);
    const source = this.source;
    const loaded = new Promise<void>((resolve, reject) => { this.operation = { resolve, reject }; });
    void (async () => {
      if (await childSha256(source) !== this.admission.bundleSha256) throw new Error("browser actor child: unverified bytes");
      if (this.phase !== "loading") return;
      this.post({ kind: "load", bytes: source, sha256: this.admission.bundleSha256 }, [source]);
      this.sourceDetached = source.byteLength === 0;
      if (!this.sourceDetached) throw new Error("browser actor child: transfer failed");
      this.source = null;
    })().catch(() => this.close("load failed"));
    return loaded;
  }

  async invoke(path: string[], args: BrowserActorChildValue[]): Promise<BrowserActorChildValue> {
    if (this.phase !== "active" || this.pending || this.sequence >= Number.MAX_SAFE_INTEGER) throw new Error("browser actor child: invocation unavailable");
    if (!Array.isArray(path) || path.length < 1 || path.length > 2 || path.some(name => typeof name !== "string" || name.length < 1 || name.length > 256) || !Array.isArray(args) || args.length > 64) throw new Error("browser actor child: invocation shape");
    const measured = measureChildValue([path, args], BROWSER_ACTOR_CHILD_LIMITS.messageBytes);
    const sequence = ++this.sequence;
    const result = new Promise<BrowserActorChildValue>((resolve, reject) => { this.pending = { sequence, resolve, reject }; });
    this.deadline(BROWSER_ACTOR_CHILD_LIMITS.invokeMs);
    try { this.post({ kind: "invoke", sequence, path, args }, measured.transfers); }
    catch { this.close("invocation transfer"); }
    return result;
  }

  close(reason = "closed"): void {
    if (this.phase === "closed") return;
    this.phase = "closed";
    clearTimeout(this.timer); this.timer = undefined;
    this.signal?.removeEventListener("abort", this.onAbort);
    try { this.post({ kind: "close" }); } catch {}
    if (this.port) { this.port.onmessage = null; this.port.onmessageerror = null; this.port.close(); this.port = null; }
    this.bootPort?.close(); this.bootPort = null;
    if (this.worker) { this.worker.onerror = null; this.worker.onmessageerror = null; this.worker.terminate(); this.worker = null; }
    const error = new Error("browser actor child: " + reason);
    const pending = this.pending, operation = this.operation;
    this.pending = undefined; this.operation = undefined;
    if (this.source) { new Uint8Array(this.source).fill(0); this.source = null; }
    if (pending?.result) for (const bytes of measureChildValue(pending.result.value, BROWSER_ACTOR_CHILD_LIMITS.outputBytes).transfers) new Uint8Array(bytes).fill(0);
    this.release(); pending?.reject(error); operation?.reject(error);
  }

  private deadline(ms: number): void {
    clearTimeout(this.timer); this.timer = setTimeout(() => this.close("deadline"), ms);
  }

  private post(value: Record<string, unknown>, transfer: Transferable[] = []): void {
    if (!this.port) throw new Error("browser actor child: no port");
    this.port.postMessage({ ...this.binding, ...value }, transfer);
  }

  private receive(value: unknown): void {
    if (this.phase === "closed" || !childBindingMatches(value, this.binding)) return;
    const exact = (...keys: string[]) => childRecord(value, ["schema", "nonce", "generation", "kind", ...keys]);
    if (value.kind === "ready" && exact() && this.phase === "booting") {
      this.phase = "ready"; clearTimeout(this.timer);
      const operation = this.operation; this.operation = undefined; operation?.resolve(); return;
    }
    if (value.kind === "loaded" && exact("byteLength", "sha256") && this.phase === "loading" && value.byteLength === this.admission.bundleByteLength && value.sha256 === this.admission.bundleSha256) {
      this.phase = "active"; clearTimeout(this.timer);
      const operation = this.operation; this.operation = undefined; operation?.resolve(); return;
    }
    const pending = this.pending;
    if (this.phase === "active" && pending && value.sequence === pending.sequence) {
      if (value.kind === "result" && exact("sequence", "value") && !pending.result) {
        try { const measured = measureChildValue(value.value, BROWSER_ACTOR_CHILD_LIMITS.outputBytes); pending.result = { value: value.value, transfers: measured.transfers.length }; return; }
        catch { this.close("result bound"); return; }
      }
      if (value.kind === "transferred" && exact("sequence", "detached") && pending.result && value.detached === pending.result.transfers) {
        clearTimeout(this.timer); this.pending = undefined; this.resultTransfersDetached += value.detached; pending.resolve(pending.result.value); return;
      }
      if (value.kind === "rejected" && exact("sequence") && !pending.result) {
        clearTimeout(this.timer); this.pending = undefined; pending.reject(new Error("browser actor child: invocation rejected")); return;
      }
    }
    this.close("protocol violation");
  }
}
