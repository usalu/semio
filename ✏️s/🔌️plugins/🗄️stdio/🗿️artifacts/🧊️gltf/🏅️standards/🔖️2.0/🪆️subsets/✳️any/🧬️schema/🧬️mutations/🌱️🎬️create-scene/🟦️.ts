/** 🦠️ Creates one empty top-level glTF scene at an explicit u32 position. */
import type { GltfSnapshot } from '../../📸️snapshot/🟦️.ts';
import { clone, insertEmptyScene, insertionPosition, isU32, reject, type GltfCreateSceneRejection } from './🔒️private/🟦️.ts';

export const GltfCreateSceneDescriptor = { id: 's.stdio.gltf.mutation.create-scene.v1', version: 1, touchedPathPattern: 'document/scenes/{position}' } as const;

export interface GltfCreateScenePayload { readonly position: number }

export type GltfCreateScenePayloadDecode = { accepted: true; payload: GltfCreateScenePayload } | { accepted: false; rejection: GltfCreateSceneRejection };

export const decodeGltfCreateScenePayload = (encoded: string): GltfCreateScenePayloadDecode => {
  try {
    const value: unknown = JSON.parse(encoded);
    if (!value || typeof value !== 'object' || Array.isArray(value) || Object.keys(value).length !== 1 || !('position' in value) || typeof value.position !== 'number' || !isU32(value.position)) return { accepted: false, rejection: reject('gltf.mutation.malformed-payload', 'mutation/payload', 'payload must be exactly { position: u32 }') };
    return { accepted: true, payload: { position: value.position } };
  } catch (error) {
    return { accepted: false, rejection: reject('gltf.mutation.malformed-payload', 'mutation/payload', String(error)) };
  }
};

export const validateGltfCreateScene = (payload: GltfCreateScenePayload, base: GltfSnapshot): GltfCreateSceneRejection | undefined => insertionPosition(payload.position, base);

export const applyGltfCreateScene = (base: GltfSnapshot, payload: GltfCreateScenePayload): { accepted: true; snapshot: GltfSnapshot } | { accepted: false; rejection: GltfCreateSceneRejection } => {
  const rejection = validateGltfCreateScene(payload, base);
  if (rejection) return { accepted: false, rejection };
  const snapshot = clone(base);
  insertEmptyScene(snapshot, payload.position);
  return { accepted: true, snapshot };
};
