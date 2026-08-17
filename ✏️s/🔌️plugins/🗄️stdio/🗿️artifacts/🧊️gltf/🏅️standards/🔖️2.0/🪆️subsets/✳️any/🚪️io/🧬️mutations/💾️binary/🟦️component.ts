/** 💾 Generic glTF mutation envelope binary transport. */
import { GLTF_MUTATION_MAX_COMMAND_ID_BYTES, GLTF_MUTATION_MAX_PAYLOAD_BYTES, validateGltfMutationEnvelope, type GltfMutation, type GltfMutationRejection } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';

export const GLTF_MUTATION_BINARY_FORMAT = 1 as const;
export const GLTF_MUTATION_BINARY_MARKER = 0x47 as const;
export type GltfMutationsBinary = Uint8Array;
export type GltfMutationBinaryApplication = { readonly bytes: Uint8Array; readonly value: GltfMutation; readonly rejection?: never } | { readonly bytes: Uint8Array; readonly value?: never; readonly rejection: GltfMutationRejection };

const utf8 = new TextEncoder();
const decodeUtf8 = new TextDecoder('utf-8', { fatal: true });
const rejected = (bytes: Uint8Array, code: string, path: string, detail: string): GltfMutationBinaryApplication => ({ bytes, rejection: { code, path, detail } });
const encodeVarint = (value: number): number[] => {
  const bytes: number[] = [];
  for (let pending = value; ; pending = Math.floor(pending / 128)) { bytes.push(pending % 128 | (pending >= 128 ? 128 : 0)); if (pending < 128) return bytes; }
};
const decodeVarint = (bytes: Uint8Array, offset: number): { readonly value: number; readonly offset: number } | GltfMutationRejection => {
  let value = 0;
  let multiplier = 1;
  for (let index = offset; index < bytes.length && index < offset + 5; index += 1) {
    const byte = bytes[index]!;
    value += (byte & 0x7f) * multiplier;
    if (byte < 128) return value <= 0xffff_ffff ? { value, offset: index + 1 } : { code: 'gltf.mutation.malformed-envelope', path: 'binary', detail: 'varint exceeds u32' };
    multiplier *= 128;
  }
  return { code: 'gltf.mutation.malformed-envelope', path: 'binary', detail: 'truncated or oversized varint' };
};

export const encodeGltfMutationBinary = (value: GltfMutation): GltfMutationBinaryApplication => {
  const rejection = validateGltfMutationEnvelope(value);
  if (rejection) return { bytes: new Uint8Array(), rejection };
  const commandId = utf8.encode(value.commandId);
  const payload = utf8.encode(value.payload);
  const phase = value.phase === 'mutation' ? 1 : 3;
  const bytes = new Uint8Array([GLTF_MUTATION_BINARY_FORMAT, GLTF_MUTATION_BINARY_MARKER, phase, ...encodeVarint(commandId.length), ...commandId, ...encodeVarint(value.version), ...encodeVarint(payload.length), ...payload]);
  return { bytes, value };
};

export const decodeGltfMutationBinary = (bytes: Uint8Array): GltfMutationBinaryApplication => {
  if (bytes.length < 3 || bytes[0] !== GLTF_MUTATION_BINARY_FORMAT || bytes[1] !== GLTF_MUTATION_BINARY_MARKER) return rejected(bytes, 'gltf.mutation.malformed-envelope', 'binary', 'format or marker is invalid');
  const phase = bytes[2] === 1 ? 'mutation' : bytes[2] === 3 ? 'inverse' : undefined;
  if (!phase) return rejected(bytes, 'gltf.mutation.malformed-envelope', 'phase', 'phase is not supported by a mutation envelope');
  const idLength = decodeVarint(bytes, 3);
  if ('code' in idLength || idLength.value === 0 || idLength.value > GLTF_MUTATION_MAX_COMMAND_ID_BYTES || idLength.offset + idLength.value > bytes.length) return rejected(bytes, 'gltf.mutation.malformed-envelope', 'commandId', 'command id length is invalid');
  const idStart = idLength.offset;
  const version = decodeVarint(bytes, idStart + idLength.value);
  if ('code' in version) return { bytes, rejection: version };
  const payloadLength = decodeVarint(bytes, version.offset);
  if ('code' in payloadLength || payloadLength.value > GLTF_MUTATION_MAX_PAYLOAD_BYTES || payloadLength.offset + payloadLength.value !== bytes.length) return rejected(bytes, 'gltf.mutation.malformed-envelope', 'payload', 'payload length is invalid or has trailing bytes');
  let commandId: string;
  let payload: string;
  try { commandId = decodeUtf8.decode(bytes.slice(idStart, idStart + idLength.value)); payload = decodeUtf8.decode(bytes.slice(payloadLength.offset)); } catch (cause) { return rejected(bytes, 'gltf.mutation.malformed-envelope', 'binary', String(cause)); }
  const value: GltfMutation = { commandId, version: version.value, phase, payload };
  const rejection = validateGltfMutationEnvelope(value);
  return rejection ? { bytes, rejection } : { bytes, value };
};
