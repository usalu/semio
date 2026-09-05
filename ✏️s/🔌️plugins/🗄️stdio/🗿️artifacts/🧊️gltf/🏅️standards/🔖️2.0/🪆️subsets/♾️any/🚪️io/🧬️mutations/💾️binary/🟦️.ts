/** 💾 Binary transport for the visible glTF mutation aggregate. */
import type { GltfMutation } from '../../../🧬️schema/🧬️mutations/🟦️.ts';

export const GLTF_MUTATION_BINARY_FORMAT = 1 as const;
export const GLTF_MUTATION_BINARY_MARKER = 0x47 as const;
export interface GltfMutationCodecRejection { readonly code: string; readonly path: string; readonly detail: string }
export type GltfMutationsBinary = Uint8Array;
export type GltfMutationBinaryApplication = { readonly bytes: Uint8Array; readonly value: GltfMutation; readonly rejection?: never } | { readonly bytes: Uint8Array; readonly value?: never; readonly rejection: GltfMutationCodecRejection };

const maxPayloadBytes = 64 * 1024;
const utf8 = new TextEncoder();
const decodeUtf8 = new TextDecoder('utf-8', { fatal: true });
const rejected = (bytes: Uint8Array, detail: string): GltfMutationBinaryApplication => ({ bytes, rejection: { code: 'gltf.mutation.malformed-transport', path: 'binary', detail } });
const encodeVarint = (value: number): number[] => {
  const bytes: number[] = [];
  for (let pending = value; ; pending = Math.floor(pending / 128)) { bytes.push(pending % 128 | (pending >= 128 ? 128 : 0)); if (pending < 128) return bytes; }
};
const decodeVarint = (bytes: Uint8Array, offset: number): { readonly value: number; readonly offset: number } | undefined => {
  let value = 0;
  let multiplier = 1;
  for (let index = offset; index < bytes.length && index < offset + 10; index += 1) {
    const byte = bytes[index]!;
    value += (byte & 0x7f) * multiplier;
    if (byte < 128) return Number.isSafeInteger(value) ? { value, offset: index + 1 } : undefined;
    multiplier *= 128;
  }
  return undefined;
};

export const encodeGltfMutationBinary = (value: GltfMutation): GltfMutationBinaryApplication => {
  const payload = utf8.encode(JSON.stringify(value));
  if (payload.length > maxPayloadBytes) return rejected(new Uint8Array(), 'aggregate exceeds the payload budget');
  return { bytes: new Uint8Array([GLTF_MUTATION_BINARY_FORMAT, GLTF_MUTATION_BINARY_MARKER, ...encodeVarint(payload.length), ...payload]), value };
};

export const decodeGltfMutationBinary = (bytes: Uint8Array): GltfMutationBinaryApplication => {
  if (bytes.length < 3 || bytes[0] !== GLTF_MUTATION_BINARY_FORMAT || bytes[1] !== GLTF_MUTATION_BINARY_MARKER) return rejected(bytes, 'format or aggregate marker is invalid');
  const length = decodeVarint(bytes, 2);
  if (!length || length.value > maxPayloadBytes || length.offset + length.value !== bytes.length) return rejected(bytes, 'payload length is invalid or has trailing bytes');
  try { return { bytes, value: JSON.parse(decodeUtf8.decode(bytes.slice(length.offset))) as GltfMutation }; } catch (cause) { return rejected(bytes, String(cause)); }
};
