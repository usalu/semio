/** 📝 Canonical one-line transport for the visible glTF mutation aggregate. */
import type { GltfMutation } from '../../../🧬️schema/🧬️mutations/🟦️component.ts';

export interface GltfMutationCodecRejection { readonly code: string; readonly path: string; readonly detail: string }
export type GltfMutationTextApplication = { readonly text: string; readonly value: GltfMutation; readonly rejection?: never } | { readonly text: string; readonly value?: never; readonly rejection: GltfMutationCodecRejection };
export type GltfMutationsText = string;

const maxPayloadBytes = 64 * 1024;
const utf8 = new TextEncoder();
const decodeUtf8 = new TextDecoder('utf-8', { fatal: true });
const rejected = (text: string, detail: string): GltfMutationTextApplication => ({ text, rejection: { code: 'gltf.mutation.malformed-transport', path: 'text', detail } });
const encodeHex = (bytes: Uint8Array): string => [...bytes].map(byte => byte.toString(16).padStart(2, '0')).join('');
const decodeHex = (value: string): Uint8Array | undefined => value.length % 2 === 0 && value.length <= maxPayloadBytes * 2 && /^[0-9a-f]*$/.test(value)
  ? Uint8Array.from({ length: value.length / 2 }, (_, index) => Number.parseInt(value.slice(index * 2, index * 2 + 2), 16))
  : undefined;

export const encodeGltfMutationText = (value: GltfMutation): GltfMutationTextApplication => {
  const payload = utf8.encode(JSON.stringify(value));
  return payload.length <= maxPayloadBytes ? { text: `gltf-mutation payload=${encodeHex(payload)}`, value } : rejected('', 'aggregate exceeds the payload budget');
};

export const decodeGltfMutationText = (text: string): GltfMutationTextApplication => {
  const bytes = text.startsWith('gltf-mutation payload=') ? decodeHex(text.slice('gltf-mutation payload='.length)) : undefined;
  if (!bytes) return rejected(text, 'expected a canonical lowercase hexadecimal aggregate payload');
  try { return { text, value: JSON.parse(decodeUtf8.decode(bytes)) as GltfMutation }; } catch (cause) { return rejected(text, String(cause)); }
};
