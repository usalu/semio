/** 🛰️ TXT physical protobuf field reader and UTF-8 decoding. */
import {txtUnicode} from "../../../../🧬️schema/🔨️modules/🧬️mutation-support/🟦️.ts";


export type TxtProtobufDecodeErrorCode='keys'|'u32'|'protobuf-wire'|'protobuf-utf8'|'protobuf-unknown'|'protobuf-duplicate'|'protobuf-truncated'|'protobuf-varint';
/** 🚫️ Physical protobuf refusal with an exact field coordinate. */
export class TxtProtobufDecodeError extends Error {constructor(readonly code:TxtProtobufDecodeErrorCode,readonly path:string){super(`txt.mutation.${code}:${path}`)}}
export const failTxtProtobufDecode=(code:TxtProtobufDecodeErrorCode,path:string):never=>{throw new TxtProtobufDecodeError(code,path)};

export class TxtProtobufReader {
  #offset = 0;
  constructor(readonly bytes: Uint8Array) {}
  get remaining() { return this.bytes.length - this.#offset; }
  varint(path: string): bigint {
    const start = this.#offset;
    let value = 0n;
    for (let index = 0; index < 10; index += 1) {
      if (this.#offset >= this.bytes.length) return failTxtProtobufDecode('protobuf-truncated', path);
      const byte = this.bytes[this.#offset++]!;
      if ((index === 9 && (byte & 0x80) !== 0) || (index === 9 && byte > 1)) return failTxtProtobufDecode('protobuf-varint', path);
      value |= BigInt(byte & 0x7f) << BigInt(index * 7);
      if ((byte & 0x80) === 0) {
        let width = 1;
        for (let remainder = value; remainder >= 0x80n; remainder >>= 7n) width += 1;
        return width === this.#offset - start ? value : failTxtProtobufDecode('protobuf-varint', path);
      }
    }
    return failTxtProtobufDecode('protobuf-varint', path);
  }
  nested(path: string): Uint8Array {
    const length = this.varint(`${path}.length`);
    if (length > BigInt(Number.MAX_SAFE_INTEGER)) return failTxtProtobufDecode('protobuf-wire', path);
    const end = this.#offset + Number(length);
    if (end > this.bytes.length) return failTxtProtobufDecode('protobuf-truncated', path);
    const value = this.bytes.slice(this.#offset, end);
    this.#offset = end;
    return value;
  }
  finish(path: string): void { if (this.remaining !== 0) failTxtProtobufDecode('protobuf-wire', path); }
}

export const txtProtobufKey = (reader: TxtProtobufReader, path: string): [number, number] => {
  const value = reader.varint(`${path}.tag`);
  const field = value >> 3n;
  if (field === 0n || field > BigInt(Number.MAX_SAFE_INTEGER)) return failTxtProtobufDecode('protobuf-wire', path);
  return [Number(field), Number(value & 7n)];
};

export const txtProtobufString = (bytes: Uint8Array, path: string): string => {
  try { return txtUnicode(new TextDecoder('utf-8', { fatal: true, ignoreBOM: true }).decode(bytes), path); } catch { return failTxtProtobufDecode('protobuf-utf8', path); }
};
