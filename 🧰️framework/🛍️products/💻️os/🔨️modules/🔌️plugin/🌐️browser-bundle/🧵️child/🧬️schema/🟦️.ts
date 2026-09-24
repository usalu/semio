export const BROWSER_ACTOR_CHILD_LIMITS = Object.freeze({ actors: 2, actorBytes: 67108864, messageBytes: 262144, outputBytes: 1048576, bootMs: 30000, loadMs: 15000, loadBytesPerMs: 1024, invokeMs: 5000, wasiOutputBytes: 65536, wasiOutputWrites: 128 });

/** 🪜️ Every stage one child load passes through, in order, as the child itself names them. The
 * bundle's own `onProgress` hook already emits `decode`, `compile` and `instantiate` per core
 * module; the four around them are the child's: taking the bytes, verifying their digest, importing
 * the module and activating it. A load that stops moving now stops inside a named stage. */
export const BROWSER_ACTOR_CHILD_LOAD_STAGES = Object.freeze(["received", "verified", "importing", "imported", "decode", "compile", "instantiate", "activating", "active"] as const);
export type BrowserActorChildLoadStageV1 = (typeof BROWSER_ACTOR_CHILD_LOAD_STAGES)[number];
export type BrowserActorChildLoadProgressV1 = Readonly<{ stage: BrowserActorChildLoadStageV1; completedBytes: number; totalBytes: number }>;

/** 🏗️ The stages whose cost is proportional to the bundle: importing a module, decoding its embedded
 * cores, compiling them and instantiating the component. Every other stage is a handoff. */
const BROWSER_ACTOR_CHILD_LOAD_WORK_STAGES: readonly BrowserActorChildLoadStageV1[] = Object.freeze(["importing", "decode", "compile", "instantiate"]);

/** ⏱️ Prices ONE stage of a load against the bytes the caller admitted. A flat whole-load budget
 * cannot say which stage stopped, so the child re-arms this budget on every stage frame it sends and
 * a stage that does not advance within it faults under its own name. `loadMs` is the fixed handoff
 * overhead and `loadBytesPerMs` the admitted throughput floor the slowest supported engine must
 * beat on the stages whose work is proportional to the bundle. */
export function childLoadStageDeadlineMs(stage: BrowserActorChildLoadStageV1, bundleByteLength: number): number {
  if (!BROWSER_ACTOR_CHILD_LOAD_STAGES.includes(stage)) throw new Error("browser actor child: load stage bound");
  if (!Number.isSafeInteger(bundleByteLength) || bundleByteLength < 1 || bundleByteLength > BROWSER_ACTOR_CHILD_LIMITS.actorBytes) throw new Error("browser actor child: load budget bound");
  return BROWSER_ACTOR_CHILD_LIMITS.loadMs + (BROWSER_ACTOR_CHILD_LOAD_WORK_STAGES.includes(stage) ? Math.ceil(bundleByteLength / BROWSER_ACTOR_CHILD_LIMITS.loadBytesPerMs) : 0);
}

/** ⏱️ Prices the WHOLE load as exactly the sum of the stage budgets it contains, so the load owner's
 * backstop can never be shorter than the stages it waits on. */
export function childLoadDeadlineMs(bundleByteLength: number): number {
  return BROWSER_ACTOR_CHILD_LOAD_STAGES.reduce((total, stage) => total + childLoadStageDeadlineMs(stage, bundleByteLength), 0);
}

/** 🧾️ Admits one exact load-progress record off the wire; every other shape is a protocol violation. */
export function isChildLoadProgress(value: unknown): value is BrowserActorChildLoadProgressV1 {
  return (
    childRecord(value, ["stage", "completedBytes", "totalBytes"]) &&
    BROWSER_ACTOR_CHILD_LOAD_STAGES.includes(value.stage) &&
    Number.isSafeInteger(value.completedBytes) &&
    Number.isSafeInteger(value.totalBytes) &&
    value.completedBytes >= 0 &&
    value.totalBytes >= 0 &&
    value.completedBytes <= value.totalBytes
  );
}

/** ⏱️ Prices the WHOLE document opening the shell waits on, so the outer deadline can never be
 * shorter than the inner ones it contains. Opening an actor-backed document is: the socket
 * handshake, then the host fetching and verifying the actor's bytes, then the child's own load of
 * those same bytes, then its boot and first invocation. `childLoadDeadlineMs` prices only the last
 * of those, so a flat outer budget silently caps a large component mid-load and leaves no refusal
 * behind — the host's race rejects while the child is still inside its own admitted budget. The
 * host's fetch-and-verify pass is charged at the same admitted throughput floor as the child's
 * load, against the largest actor the child will accept. */
export function documentOpeningDeadlineMs(): number {
  return childLoadDeadlineMs(BROWSER_ACTOR_CHILD_LIMITS.actorBytes) + Math.ceil(BROWSER_ACTOR_CHILD_LIMITS.actorBytes / BROWSER_ACTOR_CHILD_LIMITS.loadBytesPerMs) + BROWSER_ACTOR_CHILD_LIMITS.bootMs + BROWSER_ACTOR_CHILD_LIMITS.invokeMs;
}
/** 📥️ `reactor::command-ingress-status.kind` (`🔌️plugin/🧬️schema/📜️.wit`) indexed by its `u8`: a guest turn
 * reports its command ingress as that flat record, never as a tagged variant, on every browser lane. */
export const COMMAND_INGRESS_KINDS = Object.freeze(["idle", "page-accepted", "backpressure", "command-pending", "command-complete", "fault"] as const);
export type CommandIngressKindV1 = (typeof COMMAND_INGRESS_KINDS)[number];
export const BROWSER_ACTOR_CHILD_SCHEMA = "semio.os.browser-actor-child/v1";
export const BROWSER_ACTOR_CHILD_REJECTION_LIMITS = Object.freeze({ pathBytes: 128, classBytes: 64, messageBytes: 512, frameBytes: 256, frames: 4 });
export const BROWSER_ACTOR_CHILD_REJECTION_PHASES = Object.freeze(["load", "invoke"] as const);
export type BrowserActorChildRejectionPhase = (typeof BROWSER_ACTOR_CHILD_REJECTION_PHASES)[number];
export type BrowserActorChildRejectionV1 = Readonly<{ phase: BrowserActorChildRejectionPhase; path: string; errorClass: string; message: string; frame: string }>;
export type BrowserActorChildValue = null | boolean | number | string | bigint | ArrayBuffer | Uint8Array | BrowserActorChildValue[] | { [key: string]: BrowserActorChildValue };
export type BrowserActorChildBinding = Readonly<{ schema: typeof BROWSER_ACTOR_CHILD_SCHEMA; nonce: string; generation: string }>;

/** 🪡️ Accepts exact own-data records without evaluating getters. */
export function childRecord(value: unknown, keys: readonly string[]): value is Record<string, any> {
  if (!value || typeof value !== "object" || ![Object.prototype, null].includes(Object.getPrototypeOf(value))) return false;
  const descriptors = Object.getOwnPropertyDescriptors(value);
  return Reflect.ownKeys(value).length === keys.length && keys.every(key => Object.hasOwn(descriptors, key) && Object.hasOwn(descriptors[key]!, "value"));
}

/** 🔢️ The typed arrays jco lifts WIT numeric lists into — `list<u8>` → `Uint8Array`, `list<node-id>` (`list<u64>`, a
 * `set-children` patch op) → `BigUint64Array`, and so on (`_liftFlatList`'s `new typedArray(values)`, always a fresh
 * exclusive buffer). Each crosses as ONE transferable buffer; a `DataView` or `Uint8ClampedArray` is no WIT lift. */
const WIT_NUMERIC_LIST_TYPES = [Uint8Array, Int8Array, Uint16Array, Int16Array, Uint32Array, Int32Array, BigUint64Array, BigInt64Array, Float32Array, Float64Array] as const;

/** 🔢️ Whether `value` is one of {@link WIT_NUMERIC_LIST_TYPES}. */
export function isWitNumericList(value: unknown): value is InstanceType<(typeof WIT_NUMERIC_LIST_TYPES)[number]> {
  return WIT_NUMERIC_LIST_TYPES.some((type) => value instanceof type);
}

/** 🧮️ Charges framing and ordinary transferable owners before crossing the child boundary.
 *
 * `undefined` is a value here, not a refusal: an absent `option<T>` lifts to `undefined` across the
 * component ABI, so a guest turn result carrying any unset optional is an ordinary structured-clone
 * payload. Each refusal states one cause — an unsupported type and a repeated object reference are
 * different faults and a single conflated reason cannot be acted on. */
export function measureChildValue(value: unknown, limit: number): { bytes: number; transfers: ArrayBuffer[] } {
  let bytes = 0, nodes = 0;
  const objects = new Set<object>(), transfers: ArrayBuffer[] = [];
  const charge = (length: number) => { bytes += length; if (bytes > limit) throw new Error("browser actor child: message bound"); };
  const visit = (item: unknown, depth: number): void => {
    if (++nodes > 4096 || depth > 32) throw new Error("browser actor child: value depth/nodes");
    charge(8);
    if (item === null || item === undefined || typeof item === "boolean") return;
    if (typeof item === "number") { if (!Number.isFinite(item)) throw new Error("browser actor child: non-finite value"); return; }
    if (typeof item === "bigint") { if (item < -0x8000000000000000n || item > 0xffffffffffffffffn) throw new Error("browser actor child: bigint bound"); return; }
    if (typeof item === "string") { if (item.length > limit - bytes) throw new Error("browser actor child: string bound"); charge(new TextEncoder().encode(item).length); return; }
    if (typeof item !== "object") throw new Error("browser actor child: unsupported value type " + typeof item);
    if (objects.has(item)) throw new Error("browser actor child: value alias");
    objects.add(item);
    if (item instanceof ArrayBuffer || isWitNumericList(item)) {
      const buffer = item instanceof ArrayBuffer ? item : item.buffer;
      if (!(buffer instanceof ArrayBuffer) || (buffer as ArrayBuffer & { resizable?: boolean }).resizable || (!(item instanceof ArrayBuffer) && (item.byteOffset !== 0 || item.byteLength !== buffer.byteLength)) || transfers.includes(buffer)) throw new Error("browser actor child: exclusive fixed buffer required");
      new Uint8Array(buffer);
      charge(buffer.byteLength); transfers.push(buffer); return;
    }
    const descriptors = Object.getOwnPropertyDescriptors(item);
    const keys = Reflect.ownKeys(item);
    if (Array.isArray(item)) {
      if (keys.length !== item.length + 1 || item.length > 4096) throw new Error("browser actor child: dense array required");
      for (let index = 0; index < item.length; index++) {
        const descriptor = descriptors[String(index)];
        if (!descriptor || !Object.hasOwn(descriptor, "value")) throw new Error("browser actor child: own array data required");
        visit(descriptor.value, depth + 1);
      }
      return;
    }
    if (![Object.prototype, null].includes(Object.getPrototypeOf(item)) || keys.length > 4096) throw new Error("browser actor child: record required");
    for (const key of keys) {
      if (typeof key !== "string" || !Object.hasOwn(descriptors[key]!, "value")) throw new Error("browser actor child: own record data required");
      visit(key, depth + 1); visit(descriptors[key]!.value, depth + 1);
    }
  };
  visit(value, 0);
  return { bytes, transfers };
}

/** 🔗️ Fences one private port generation; a stale port carries no reusable authority. */
export function childBindingMatches(value: unknown, binding: BrowserActorChildBinding): value is Record<string, any> {
  if (!value || typeof value !== "object") return false;
  const fields = Object.getOwnPropertyDescriptors(value);
  return fields.schema?.value === binding.schema && fields.nonce?.value === binding.nonce && fields.generation?.value === binding.generation;
}

/** ✂️ Truncates one diagnostic string to an exact UTF-8 byte bound without ever growing it. */
export function boundChildText(value: string, limit: number): string {
  if (new TextEncoder().encode(value).byteLength <= limit) return value;
  let end = value.length;
  while (end > 0 && new TextEncoder().encode(value.slice(0, end)).byteLength > limit - 3) end -= 1;
  return value.slice(0, end) + "...";
}

/** 🩻️ Names one guest failure: the phase it happened in, the export path it was reached through, and
 * the guest error's own class and message, each bounded. It carries no bytes, no module URL and no
 * host object — only what the guest itself said about why it refused. */
export function childRejectionReason(phase: BrowserActorChildRejectionPhase, path: readonly unknown[], error: unknown): BrowserActorChildRejectionV1 {
  const text = (read: () => unknown): string | null => {
    try {
      const value = read();
      return typeof value === "string" ? value : null;
    } catch {
      return null;
    }
  };
  const route = path.filter((name): name is string => typeof name === "string").join("/");
  const errorClass = text(() => (error as Error)?.name) || text(() => (error as { constructor?: { name?: unknown } })?.constructor?.name) || typeof error;
  const message = text(() => (error as Error)?.message) ?? text(() => String(error)) ?? "";
  return Object.freeze({
    phase,
    path: boundChildText(route, BROWSER_ACTOR_CHILD_REJECTION_LIMITS.pathBytes),
    errorClass: boundChildText(errorClass, BROWSER_ACTOR_CHILD_REJECTION_LIMITS.classBytes),
    message: boundChildText(message, BROWSER_ACTOR_CHILD_REJECTION_LIMITS.messageBytes),
    frame: boundChildText(childRejectionFrame(text(() => (error as Error)?.stack)), BROWSER_ACTOR_CHILD_REJECTION_LIMITS.frameBytes),
  });
}

/** 📍️ Names the innermost guest call sites as `<function>@<line>:<column>` and nothing else. A module
 * URL, blob id or origin never leaves the child: the lines alone locate the fault inside the one
 * bundle whose digest the owner already verified, and one frame is rarely enough when the innermost
 * one is a generated helper shared by every lowering. */
export function childRejectionFrame(stack: string | null): string {
  const frames: string[] = [];
  for (const line of (stack ?? "").split("\n").slice(1)) {
    const at = /^\s*at\s+(?:(?:async\s+)?([^\s(]+)\s+)?\(?.*?:(\d+):(\d+)\)?\s*$/u.exec(line);
    if (at) frames.push((at[1] ?? "<anonymous>").split("/").pop()! + "@" + at[2] + ":" + at[3]);
    if (frames.length >= BROWSER_ACTOR_CHILD_REJECTION_LIMITS.frames) break;
  }
  return frames.join("<");
}

/** 🧾️ Admits one exact rejection record off the wire; every other shape is a protocol violation. */
export function isChildRejectionReason(value: unknown): value is BrowserActorChildRejectionV1 {
  if (!childRecord(value, ["phase", "path", "errorClass", "message", "frame"]) || !BROWSER_ACTOR_CHILD_REJECTION_PHASES.includes(value.phase)) return false;
  const encoder = new TextEncoder();
  return (
    typeof value.path === "string" &&
    typeof value.errorClass === "string" &&
    typeof value.message === "string" &&
    typeof value.frame === "string" &&
    encoder.encode(value.path).byteLength <= BROWSER_ACTOR_CHILD_REJECTION_LIMITS.pathBytes &&
    encoder.encode(value.errorClass).byteLength <= BROWSER_ACTOR_CHILD_REJECTION_LIMITS.classBytes &&
    encoder.encode(value.message).byteLength <= BROWSER_ACTOR_CHILD_REJECTION_LIMITS.messageBytes &&
    encoder.encode(value.frame).byteLength <= BROWSER_ACTOR_CHILD_REJECTION_LIMITS.frameBytes
  );
}

/** 🗣️ Renders one rejection record as the single line every owner above this boundary reports. */
export function childRejectionText(reason: BrowserActorChildRejectionV1): string {
  return reason.phase + " " + (reason.path || "-") + ": " + reason.errorClass + ": " + reason.message + (reason.frame ? " at " + reason.frame : "");
}

/** 🔐️ Computes the byte identity using the browser's native cryptographic primitive. */
export async function childSha256(bytes: ArrayBuffer): Promise<string> {
  return Array.from(new Uint8Array(await crypto.subtle.digest("SHA-256", bytes)), value => value.toString(16).padStart(2, "0")).join("");
}

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️browser-actor-child-rejection-carries-a-bounded-typed-reason/🟦️.ts");
  await registerTests1(import.meta.vitest, { BROWSER_ACTOR_CHILD_REJECTION_LIMITS, BROWSER_ACTOR_CHILD_REJECTION_PHASES, boundChildText, childRejectionFrame, childRejectionReason, childRejectionText, isChildRejectionReason }, { directory: (await import("node:url")).fileURLToPath(new URL(".", import.meta.url)), url: import.meta.url });
  const { registerTests2 } = await import("./🧪️tests/🧪️browser-actor-child-admits-wit-numeric-lists/🟦️.ts");
  await registerTests2(import.meta.vitest, { measureChildValue }, { directory: (await import("node:url")).fileURLToPath(new URL(".", import.meta.url)), url: import.meta.url });
}
