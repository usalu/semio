export const BROWSER_ACTOR_CHILD_LIMITS = Object.freeze({ actors: 2, actorBytes: 67108864, messageBytes: 262144, outputBytes: 1048576, bootMs: 10000, loadMs: 5000, invokeMs: 2000, wasiOutputBytes: 65536, wasiOutputWrites: 128 });
export const BROWSER_ACTOR_CHILD_SCHEMA = "semio.os.browser-actor-child/v1";
export type BrowserActorChildValue = null | boolean | number | string | bigint | ArrayBuffer | Uint8Array | BrowserActorChildValue[] | { [key: string]: BrowserActorChildValue };
export type BrowserActorChildBinding = Readonly<{ schema: typeof BROWSER_ACTOR_CHILD_SCHEMA; nonce: string; generation: string }>;

/** 🪡️ Accepts exact own-data records without evaluating getters. */
export function childRecord(value: unknown, keys: readonly string[]): value is Record<string, any> {
  if (!value || typeof value !== "object" || ![Object.prototype, null].includes(Object.getPrototypeOf(value))) return false;
  const descriptors = Object.getOwnPropertyDescriptors(value);
  return Reflect.ownKeys(value).length === keys.length && keys.every(key => Object.hasOwn(descriptors, key) && Object.hasOwn(descriptors[key]!, "value"));
}

/** 🧮️ Charges framing and ordinary transferable owners before crossing the child boundary. */
export function measureChildValue(value: unknown, limit: number): { bytes: number; transfers: ArrayBuffer[] } {
  let bytes = 0, nodes = 0;
  const objects = new Set<object>(), transfers: ArrayBuffer[] = [];
  const charge = (length: number) => { bytes += length; if (bytes > limit) throw new Error("browser actor child: message bound"); };
  const visit = (item: unknown, depth: number): void => {
    if (++nodes > 4096 || depth > 32) throw new Error("browser actor child: value depth/nodes");
    charge(8);
    if (item === null || typeof item === "boolean") return;
    if (typeof item === "number") { if (!Number.isFinite(item)) throw new Error("browser actor child: non-finite value"); return; }
    if (typeof item === "bigint") { if (item < -0x8000000000000000n || item > 0xffffffffffffffffn) throw new Error("browser actor child: bigint bound"); return; }
    if (typeof item === "string") { if (item.length > limit - bytes) throw new Error("browser actor child: string bound"); charge(new TextEncoder().encode(item).length); return; }
    if (!item || typeof item !== "object" || objects.has(item)) throw new Error("browser actor child: value type/alias");
    objects.add(item);
    if (item instanceof ArrayBuffer || item instanceof Uint8Array) {
      const buffer = item instanceof ArrayBuffer ? item : item.buffer;
      if (!(buffer instanceof ArrayBuffer) || (buffer as ArrayBuffer & { resizable?: boolean }).resizable || (item instanceof Uint8Array && (item.byteOffset !== 0 || item.byteLength !== buffer.byteLength)) || transfers.includes(buffer)) throw new Error("browser actor child: exclusive fixed buffer required");
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

/** 🔐️ Computes the byte identity using the browser's native cryptographic primitive. */
export async function childSha256(bytes: ArrayBuffer): Promise<string> {
  return Array.from(new Uint8Array(await crypto.subtle.digest("SHA-256", bytes)), value => value.toString(16).padStart(2, "0")).join("");
}
