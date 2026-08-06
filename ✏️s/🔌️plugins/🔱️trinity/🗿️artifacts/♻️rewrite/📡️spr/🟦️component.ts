/** rewrite spr — thin WASM encode/decode facade. */
export function encode(value: unknown): Uint8Array {
  throw new Error("wire to plugin WASM");
}
export function decode(bytes: Uint8Array): unknown {
  throw new Error("wire to plugin WASM");
}
