import { BROWSER_ACTOR_CHILD_LIMITS, BROWSER_ACTOR_CHILD_SCHEMA, childBindingMatches, childRecord, childSha256, measureChildValue, type BrowserActorChildBinding } from "../🧬️schema/🟦️.ts";

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
const fault = () => { try { send({ kind: "fault" }); } finally { retire(); } };
const denied = () => { throw new Error("browser actor child: host effect denied"); };
let wasiOutputBytes = 0, wasiOutputWrites = 0;
const wasiPort = Object.freeze({
  nowNs(): bigint {
    if (phase !== "loading" && phase !== "active") throw new Error("browser actor child: retired WASI clock");
    return BigInt(Math.floor(performance.now() * 1_000_000));
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

async function receive(value: unknown): Promise<void> {
  if (!binding || phase === "closed" || !childBindingMatches(value, binding)) return;
  const exact = (...keys: string[]) => childRecord(value, ["schema", "nonce", "generation", "kind", ...keys]);
  if (value.kind === "close" && exact()) { retire(); return; }
  if (value.kind === "load" && exact("bytes", "sha256") && phase === "ready") {
    phase = "loading";
    const bytes = value.bytes;
    try {
      if (!(bytes instanceof ArrayBuffer) || bytes.byteLength < 1 || bytes.byteLength > BROWSER_ACTOR_CHILD_LIMITS.actorBytes || (bytes as ArrayBuffer & { resizable?: boolean }).resizable || typeof value.sha256 !== "string" || !/^[0-9a-f]{64}$/.test(value.sha256)) throw new Error("load shape");
      const byteLength = bytes.byteLength, sha256 = await childSha256(bytes);
      if (sha256 !== value.sha256 || phase !== "loading") throw new Error("load digest");
      moduleUrl = URL.createObjectURL(new Blob([bytes], { type: "text/javascript" }));
      let module: Record<string, any>;
      try { module = await import(/* @vite-ignore */ moduleUrl); }
      finally { URL.revokeObjectURL(moduleUrl); moduleUrl = null; new Uint8Array(bytes).fill(0); }
      if (phase !== "loading" || typeof module.activate !== "function") throw new Error("load activation");
      actor = await module.activate({ actorId, activationGeneration: BigInt(binding.generation) }, hostPort, { signal: control.signal });
      if (!actor || typeof actor.invoke !== "function" || typeof actor.close !== "function" || phase !== "loading") throw new Error("activation shape");
      phase = "active"; send({ kind: "loaded", byteLength, sha256 });
    } catch { try { new Uint8Array(bytes).fill(0); } catch {} fault(); }
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
    } catch { if (phase === "active") send({ kind: "rejected", sequence }); }
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
  port!.onmessage = event => { void receive(event.data).catch(fault); };
  port!.onmessageerror = fault; port!.start(); send({ kind: "ready" });
};
