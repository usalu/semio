/** 🦀️ Ambient declarations for the untyped `@webassemblyjs/leb128` CommonJS package this router imports for WAL varint oracles. */

declare module "@webassemblyjs/leb128" {
  export const MAX_NUMBER_OF_BYTE_U32: number;
  export const MAX_NUMBER_OF_BYTE_U64: number;
  export function decodeInt64(encodedBuffer: Uint8Array, index: number): { value: number; nextIndex: number };
  export function decodeUInt64(encodedBuffer: Uint8Array, index: number): { value: number; nextIndex: number };
  export function decodeInt32(encodedBuffer: Uint8Array, index: number): { value: number; nextIndex: number };
  export function decodeUInt32(encodedBuffer: Uint8Array, index: number): { value: number; nextIndex: number };
  export function encodeU32(value: number): number[];
  export function encodeI32(value: number): number[];
  export function encodeI64(value: number): number[];
}

declare module "@webassemblyjs/leb128/lib/leb.js" {
  /** 🔢️ The single CommonJS default export of `lib/leb.js`, re-exported by the package index. */
  const leb: {
    decodeInt32(encodedBuffer: Uint8Array, index: number): { value: number; nextIndex: number };
    decodeInt64(encodedBuffer: Uint8Array, index: number): { value: number; nextIndex: number };
    decodeIntBuffer(encodedBuffer: Uint8Array, index: number): { value: Uint8Array; nextIndex: number; lossy: boolean };
    decodeUInt32(encodedBuffer: Uint8Array, index: number): { value: number; nextIndex: number };
    decodeUInt64(encodedBuffer: Uint8Array, index: number): { value: number; nextIndex: number };
    decodeUIntBuffer(encodedBuffer: Uint8Array, index: number): { value: Uint8Array; nextIndex: number; lossy: boolean };
    encodeInt32(value: number): number[];
    encodeInt64(value: number): number[];
    encodeIntBuffer(buffer: Uint8Array): number[];
    encodeUInt32(value: number): number[];
    encodeUInt64(value: number): number[];
    encodeUIntBuffer(buffer: Uint8Array): number[];
  };
  export default leb;
}
