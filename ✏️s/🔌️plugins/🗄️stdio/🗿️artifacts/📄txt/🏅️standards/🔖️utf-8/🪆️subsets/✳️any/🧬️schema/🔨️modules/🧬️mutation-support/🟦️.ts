/** 🧰 TXT mutation transport primitives. */
export type TxtMutationDecodeErrorCode = 'record' | 'keys' | 'u32' | 'unicode' | 'protobuf-wire' | 'protobuf-utf8' | 'protobuf-unknown' | 'protobuf-duplicate' | 'protobuf-truncated' | 'protobuf-varint';

export class TxtMutationDecodeError extends Error {
  constructor(readonly code: TxtMutationDecodeErrorCode, readonly path: string) { super(`txt.mutation.${code}:${path}`); }
}

export type TxtWireRecord = Record<string, unknown>;
export const txtMaximumU32 = 0xffff_ffff;
export const failTxtMutationDecode = (code: TxtMutationDecodeErrorCode, path: string): never => { throw new TxtMutationDecodeError(code, path); };
export const txtOwn = (value: object, key: string): boolean => Object.prototype.hasOwnProperty.call(value, key);
export const txtWireRecord = (value: unknown, path: string): TxtWireRecord => {
  if (typeof value !== 'object' || value === null || Array.isArray(value) || (Object.getPrototypeOf(value) !== Object.prototype && Object.getPrototypeOf(value) !== null)) return failTxtMutationDecode('record', path);
  return value as TxtWireRecord;
};
export const txtExact = (value: unknown, path: string, allowed: readonly string[]): TxtWireRecord => {
  const record = txtWireRecord(value, path);
  if (Reflect.ownKeys(record).some((key) => typeof key !== 'string' || !allowed.includes(key))) return failTxtMutationDecode('keys', path);
  return record;
};
export const coerceTxtMutationUInt32Variable = (value: unknown): number => typeof value === 'number' && Number.isFinite(value) && Number.isInteger(value) && value >= 0 && value <= txtMaximumU32 ? value : failTxtMutationDecode('u32', 'graphql.uint32.variable');
export const coerceTxtMutationUInt32Literal = (kind: string, value: unknown): number => kind === 'IntValue' && typeof value === 'string' && /^(?:0|[1-9][0-9]*)$/.test(value) ? coerceTxtMutationUInt32Variable(Number(value)) : failTxtMutationDecode('u32', 'graphql.uint32.literal');
export const txtUnicode = (value: unknown, path: string): string => {
  if (typeof value !== 'string') return failTxtMutationDecode('unicode', path);
  for (let index = 0; index < value.length; index += 1) {
    const code = value.charCodeAt(index);
    if (code >= 0xd800 && code <= 0xdbff) {
      const following = value.charCodeAt(index + 1);
      if (!Number.isInteger(following) || following < 0xdc00 || following > 0xdfff) return failTxtMutationDecode('unicode', path);
      index += 1;
    } else if (code >= 0xdc00 && code <= 0xdfff) return failTxtMutationDecode('unicode', path);
  }
  return value;
};

export class TxtProtobufReader {
  #offset = 0;
  constructor(readonly bytes: Uint8Array) {}
  get remaining() { return this.bytes.length - this.#offset; }
  varint(path: string): bigint {
    const start = this.#offset;
    let value = 0n;
    for (let index = 0; index < 10; index += 1) {
      if (this.#offset >= this.bytes.length) return failTxtMutationDecode('protobuf-truncated', path);
      const byte = this.bytes[this.#offset++]!;
      if ((index === 9 && (byte & 0x80) !== 0) || (index === 9 && byte > 1)) return failTxtMutationDecode('protobuf-varint', path);
      value |= BigInt(byte & 0x7f) << BigInt(index * 7);
      if ((byte & 0x80) === 0) {
        let width = 1;
        for (let remainder = value; remainder >= 0x80n; remainder >>= 7n) width += 1;
        return width === this.#offset - start ? value : failTxtMutationDecode('protobuf-varint', path);
      }
    }
    return failTxtMutationDecode('protobuf-varint', path);
  }
  nested(path: string): Uint8Array {
    const length = this.varint(`${path}.length`);
    if (length > BigInt(Number.MAX_SAFE_INTEGER)) return failTxtMutationDecode('protobuf-wire', path);
    const end = this.#offset + Number(length);
    if (end > this.bytes.length) return failTxtMutationDecode('protobuf-truncated', path);
    const value = this.bytes.slice(this.#offset, end);
    this.#offset = end;
    return value;
  }
  finish(path: string): void { if (this.remaining !== 0) failTxtMutationDecode('protobuf-wire', path); }
}
export const txtProtobufKey = (reader: TxtProtobufReader, path: string): [number, number] => {
  const value = reader.varint(`${path}.tag`);
  const field = value >> 3n;
  if (field === 0n || field > BigInt(Number.MAX_SAFE_INTEGER)) return failTxtMutationDecode('protobuf-wire', path);
  return [Number(field), Number(value & 7n)];
};
export const txtProtobufString = (bytes: Uint8Array, path: string): string => {
  try { return txtUnicode(new TextDecoder('utf-8', { fatal: true, ignoreBOM: true }).decode(bytes), path); } catch { return failTxtMutationDecode('protobuf-utf8', path); }
};
