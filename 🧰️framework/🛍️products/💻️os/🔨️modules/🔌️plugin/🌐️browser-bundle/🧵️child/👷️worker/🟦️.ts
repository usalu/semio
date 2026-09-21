import { BROWSER_ACTOR_CHILD_LIMITS, BROWSER_ACTOR_CHILD_LOAD_STAGES, BROWSER_ACTOR_CHILD_SCHEMA, childBindingMatches, childRecord, childRejectionReason, childSha256, measureChildValue, type BrowserActorChildBinding, type BrowserActorChildLoadStageV1, type BrowserActorChildRejectionV1 } from "../🧬️schema/🟦️.ts";

type Actor = { invoke(path: string[], args: unknown[]): Promise<unknown>; close(): Promise<void> };
const child = globalThis as unknown as DedicatedWorkerGlobalScope;
let port: MessagePort | null = null, binding: BrowserActorChildBinding | null = null;
let actor: Actor | null = null, phase: "boot" | "ready" | "loading" | "active" | "closed" = "boot";
let sequence = 0, busy = false, actorId = "", moduleUrl: string | null = null;
const control = new AbortController();
const send = (value: Record<string, unknown>, transfers: Transferable[] = []) => port?.postMessage({ ...binding, ...value }, transfers);
const retire = () => {
  if (phase === "closed") return;
  phase = "closed"; control.abort();
  if (moduleUrl) URL.revokeObjectURL(moduleUrl); moduleUrl = null;
  const ownedActor = actor; actor = null;
  void ownedActor?.close().catch(() => {});
  if (port) { port.onmessage = null; port.onmessageerror = null; port.close(); port = null; }
  child.close();
};
const fault = (reason?: BrowserActorChildRejectionV1) => { try { send(reason ? { kind: "fault", reason } : { kind: "fault" }); } finally { retire(); } };
const denied = () => { throw new Error("browser actor child: host effect denied"); };
let wasiOutputBytes = 0, wasiOutputWrites = 0;
const wasiPort = Object.freeze({
  nowNs(): bigint {
    if (phase !== "loading" && phase !== "active") throw new Error("browser actor child: retired WASI clock");
    return BigInt(Math.floor(performance.now() * 1_000_000));
  },
  wallNs(): bigint {
    if (phase !== "loading" && phase !== "active") throw new Error("browser actor child: retired WASI clock");
    return BigInt(Date.now()) * 1_000_000n;
  },
  write(channel: "stdout" | "stderr", bytes: Uint8Array): void {
    if ((phase !== "loading" && phase !== "active") || (channel !== "stdout" && channel !== "stderr") || !(bytes instanceof Uint8Array) || bytes.byteLength > BROWSER_ACTOR_CHILD_LIMITS.wasiOutputBytes - wasiOutputBytes || wasiOutputWrites >= BROWSER_ACTOR_CHILD_LIMITS.wasiOutputWrites) throw new Error("browser actor child: WASI output capacity");
    measureChildValue(bytes, BROWSER_ACTOR_CHILD_LIMITS.messageBytes);
    wasiOutputBytes += bytes.byteLength; wasiOutputWrites++;
    if (bytes.byteLength) (channel === "stdout" ? console.log : console.warn)("[browser actor " + channel + "] " + new TextDecoder().decode(bytes));
  },
  exit(): void { fault(); },
});
const hostPort = Object.freeze({ dispatch: denied, cancelEffect: () => "closed" as const, log: denied, traceSpan: denied, nowMs: () => BigInt(Date.now()), wasi: wasiPort });

/** 🪜️ Announces one load stage to the owner. The load phase used to be the only silent stretch of
 * the child's life: the bytes arrived and nothing was heard again until the module either activated
 * or the owner's flat budget expired, so a stall inside it had no name. Every frame re-arms the
 * owner's per-stage budget, so a stage that stops advancing faults under its own name.
 *
 * A load decodes and compiles megabytes in small steps, so the same stage would otherwise speak
 * thousands of times; a repeat is admitted only after a whole progress unit of work or at the end of
 * its stage, and a stage change always speaks. */
let announcedStage: BrowserActorChildLoadStageV1 | null = null, announcedBytes = 0;
const LOAD_PROGRESS_UNIT_BYTES = 4 * 1024 * 1024;
const announce = (stage: BrowserActorChildLoadStageV1, completedBytes: number, totalBytes: number): void => {
  if (phase !== "loading" || !BROWSER_ACTOR_CHILD_LOAD_STAGES.includes(stage) || !Number.isSafeInteger(completedBytes) || !Number.isSafeInteger(totalBytes) || completedBytes < 0 || completedBytes > totalBytes) return;
  if (stage === announcedStage && completedBytes < totalBytes && completedBytes - announcedBytes < LOAD_PROGRESS_UNIT_BYTES) return;
  announcedStage = stage; announcedBytes = completedBytes;
  send({ kind: "progress", stage, completedBytes, totalBytes });
};

/** 🛰️ Relays the bundle's own activation progress. The generated actor already reports decoding,
 * compiling and instantiating each embedded core module and nothing above it ever listened, so the
 * one part of a load that takes real time reported nothing. Anything the bundle names outside the
 * admitted stage vocabulary is dropped rather than widened. */
function bundleProgress(frame: unknown): void {
  if (!frame || typeof frame !== "object") return;
  const { phase: stage, completed, total } = frame as { phase?: unknown; completed?: unknown; total?: unknown };
  if (typeof stage !== "string" || !BROWSER_ACTOR_CHILD_LOAD_STAGES.includes(stage as BrowserActorChildLoadStageV1)) return;
  const completedBytes = typeof completed === "number" ? completed : 0, totalBytes = typeof total === "number" ? total : 0;
  announce(stage as BrowserActorChildLoadStageV1, Math.min(completedBytes, totalBytes), totalBytes);
}

async function receive(value: unknown): Promise<void> {
  if (!binding || phase === "closed" || !childBindingMatches(value, binding)) return;
  const exact = (...keys: string[]) => childRecord(value, ["schema", "nonce", "generation", "kind", ...keys]);
  if (value.kind === "close" && exact()) { retire(); return; }
  if (value.kind === "load" && exact("bytes", "sha256") && phase === "ready") {
    phase = "loading";
    const bytes = value.bytes;
    try {
      if (!(bytes instanceof ArrayBuffer) || bytes.byteLength < 1 || bytes.byteLength > BROWSER_ACTOR_CHILD_LIMITS.actorBytes || (bytes as ArrayBuffer & { resizable?: boolean }).resizable || typeof value.sha256 !== "string" || !/^[0-9a-f]{64}$/.test(value.sha256)) throw new Error("load shape");
      const byteLength = bytes.byteLength;
      announce("received", byteLength, byteLength);
      const sha256 = await childSha256(bytes);
      if (sha256 !== value.sha256 || phase !== "loading") throw new Error("load digest");
      announce("verified", byteLength, byteLength);
      moduleUrl = URL.createObjectURL(new Blob([bytes], { type: "text/javascript" }));
      let module: Record<string, any>;
      announce("importing", 0, byteLength);
      try { module = await import(/* @vite-ignore */ moduleUrl); }
      finally { URL.revokeObjectURL(moduleUrl); moduleUrl = null; new Uint8Array(bytes).fill(0); }
      if (phase !== "loading" || typeof module.activate !== "function") throw new Error("load activation");
      announce("imported", byteLength, byteLength);
      announce("activating", 0, byteLength);
      actor = await module.activate({ actorId, activationGeneration: BigInt(binding.generation) }, hostPort, { signal: control.signal, onProgress: bundleProgress });
      if (!actor || typeof actor.invoke !== "function" || typeof actor.close !== "function" || phase !== "loading") throw new Error("activation shape");
      announce("active", byteLength, byteLength);
      phase = "active"; send({ kind: "loaded", byteLength, sha256 });
    } catch (error) { try { new Uint8Array(bytes).fill(0); } catch {} fault(childRejectionReason("load", ["activate"], error)); }
    return;
  }
  if (value.kind === "invoke" && exact("sequence", "path", "args") && phase === "active" && !busy && Number.isSafeInteger(value.sequence) && value.sequence === sequence + 1) {
    if (!Array.isArray(value.path) || value.path.length < 1 || value.path.length > 2 || value.path.some((name: unknown) => typeof name !== "string" || name.length < 1 || name.length > 256) || !Array.isArray(value.args) || value.args.length > 64) { fault(); return; }
    sequence = value.sequence; busy = true;
    try {
      measureChildValue([value.path, value.args], BROWSER_ACTOR_CHILD_LIMITS.messageBytes);
      const result = await actor!.invoke(value.path, value.args);
      if (phase !== "active") return;
      const measured = measureChildValue(result, BROWSER_ACTOR_CHILD_LIMITS.outputBytes);
      send({ kind: "result", sequence, value: result }, measured.transfers);
      send({ kind: "transferred", sequence, detached: measured.transfers.filter(buffer => buffer.byteLength === 0).length });
    } catch (error) { if (phase === "active") send({ kind: "rejected", sequence, reason: childRejectionReason("invoke", value.path, error) }); }
    finally { busy = false; }
    return;
  }
  fault();
}

child.onmessage = event => {
  const value = event.data;
  if (phase !== "boot" || !childRecord(value, ["schema", "nonce", "generation", "kind", "actorId", "port"]) || value.schema !== BROWSER_ACTOR_CHILD_SCHEMA || value.kind !== "init" || typeof value.nonce !== "string" || !/^[0-9a-f-]{36}$/.test(value.nonce) || typeof value.generation !== "string" || !/^[1-9][0-9]{0,19}$/.test(value.generation) || BigInt(value.generation) > 0xffffffffffffffffn || typeof value.actorId !== "string" || value.actorId.length < 1 || value.actorId.length > 512 || /[\x00-\x1f\x7f]/.test(value.actorId) || !(value.port instanceof MessagePort)) { fault(); return; }
  binding = { schema: BROWSER_ACTOR_CHILD_SCHEMA, nonce: value.nonce, generation: value.generation };
  actorId = value.actorId; port = value.port; phase = "ready";
  child.onmessage = null;
  port!.onmessage = event => { void receive(event.data).catch(error => fault(childRejectionReason(phase === "loading" ? "load" : "invoke", [], error))); };
  port!.onmessageerror = () => fault(); port!.start(); send({ kind: "ready" });
};
