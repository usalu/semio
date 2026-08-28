//#region 🔒️IntrinsicOwnership
type ViewKind = "Uint8Array" | "BigUint64Array";
function getter(prototype: object, key: PropertyKey): (owner: unknown) => unknown {
  const read = Object.getOwnPropertyDescriptor(prototype, key)?.get;
  if (!read) throw new Error("Required native buffer intrinsic is unavailable");
  return owner => Reflect.apply(read, owner, []);
}
const viewPrototype: object = Object.getPrototypeOf(Uint8Array.prototype);
const viewKind = getter(viewPrototype, Symbol.toStringTag);
const viewBuffer = getter(viewPrototype, "buffer");
const viewOffset = getter(viewPrototype, "byteOffset");
const viewBytes = getter(viewPrototype, "byteLength");
const bufferBytes = getter(ArrayBuffer.prototype, "byteLength");
function ordinaryBuffer(value: unknown): value is ArrayBuffer {
  try { return typeof bufferBytes(value) === "number"; } catch { return false; }
}
/** 🔒️ Intrinsic branding binds transfer to the actual whole view, including across realms and shadowed metadata. */
export function takeOwnedNativeBuffer(input: unknown, kind: ViewKind, maximumBytes: number): ArrayBuffer {
  if (!Number.isSafeInteger(maximumBytes) || maximumBytes < 0 || viewKind(input) !== kind) throw new Error("Invalid native buffer admission");
  const buffer = viewBuffer(input); const bytes = viewBytes(input);
  if (!ordinaryBuffer(buffer) || viewOffset(input) !== 0 || typeof bytes !== "number" || bytes !== bufferBytes(buffer) || bytes > maximumBytes) throw new Error("Native ownership requires an entire non-shared admitted buffer");
  try {
    new Uint8Array(buffer);
    const moved: unknown = structuredClone(buffer, { transfer: [buffer] });
    if (!ordinaryBuffer(moved) || bufferBytes(buffer) !== 0 || bufferBytes(moved) !== bytes) throw new Error("Native buffer transfer did not preserve exact ownership");
    return moved;
  } catch { throw new Error("Native buffer ownership transfer failed"); }
}
//#endregion 🔒️IntrinsicOwnership
