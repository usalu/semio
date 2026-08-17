/** 📝 Canonical one-line generic glTF mutation envelope transport. */
import { GLTF_MUTATION_MAX_PAYLOAD_BYTES, validateGltfMutationEnvelope, type GltfMutation, type GltfMutationRejection } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';

export type GltfMutationTextApplication = { readonly text: string; readonly value: GltfMutation; readonly rejection?: never } | { readonly text: string; readonly value?: never; readonly rejection: GltfMutationRejection };
export type GltfMutationsText = string;

const utf8 = new TextEncoder();
const decodeUtf8 = new TextDecoder('utf-8', { fatal: true });
const rejected = (text: string, code: string, path: string, detail: string): GltfMutationTextApplication => ({ text, rejection: { code, path, detail } });
const encodeHex = (bytes: Uint8Array): string => [...bytes].map(byte => byte.toString(16).padStart(2, '0')).join('');
const decodeHex = (value: string, path: string): Uint8Array | GltfMutationRejection => {
  if (value.length % 2 !== 0 || value.length > GLTF_MUTATION_MAX_PAYLOAD_BYTES * 2 || !/^[0-9a-f]*$/.test(value)) return { code: 'gltf.mutation.malformed-envelope', path, detail: 'hex must be lowercase, even-length, and within the payload budget' };
  return Uint8Array.from({ length: value.length / 2 }, (_, index) => Number.parseInt(value.slice(index * 2, index * 2 + 2), 16));
};

export const encodeGltfMutationText = (value: GltfMutation): GltfMutationTextApplication => {
  const rejection = validateGltfMutationEnvelope(value);
  if (rejection) return { text: '', rejection };
  const text = `gltf-mutation commandId=${encodeHex(utf8.encode(value.commandId))} version=${value.version} phase=${value.phase} payload=${value.payload ? encodeHex(utf8.encode(value.payload)) : '-'}`;
  return { text, value };
};

export const decodeGltfMutationText = (text: string): GltfMutationTextApplication => {
  const fields = text.split(/\s+/);
  if (fields.length !== 5 || fields[0] !== 'gltf-mutation') return rejected(text, 'gltf.mutation.malformed-envelope', 'text', 'expected the canonical gltf-mutation envelope');
  const field = (index: number, name: string): string | undefined => fields[index]?.startsWith(name) ? fields[index]!.slice(name.length) : undefined;
  const commandIdHex = field(1, 'commandId=');
  const versionText = field(2, 'version=');
  const phase = field(3, 'phase=');
  const payloadHex = field(4, 'payload=');
  if (commandIdHex === undefined || versionText === undefined || payloadHex === undefined || phase !== 'mutation' && phase !== 'inverse') return rejected(text, 'gltf.mutation.malformed-envelope', 'text', 'envelope fields are malformed');
  const commandIdBytes = decodeHex(commandIdHex, 'commandId');
  const payloadBytes = payloadHex === '-' ? new Uint8Array() : decodeHex(payloadHex, 'payload');
  if ('code' in commandIdBytes) return { text, rejection: commandIdBytes };
  if ('code' in payloadBytes) return { text, rejection: payloadBytes };
  let commandId: string;
  let payload: string;
  try { commandId = decodeUtf8.decode(commandIdBytes); payload = decodeUtf8.decode(payloadBytes); } catch (cause) { return rejected(text, 'gltf.mutation.malformed-envelope', 'text', String(cause)); }
  const value: GltfMutation = { commandId, version: Number(versionText), phase, payload };
  const rejection = validateGltfMutationEnvelope(value);
  return rejection ? { text, rejection } : { text, value };
};
