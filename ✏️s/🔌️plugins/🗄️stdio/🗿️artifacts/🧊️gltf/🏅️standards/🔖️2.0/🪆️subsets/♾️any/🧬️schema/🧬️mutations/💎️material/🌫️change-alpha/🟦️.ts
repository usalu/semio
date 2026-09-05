/** 🦠️ change-material-alpha-mode executes one typed alpha-mode mutation. */
import type { GltfAlphaMode, GltfSnapshot } from '../../📸️snapshot/🟦️.ts';
import { index, type GltfMaterialAnimationFailure } from './🟦️';
export const GltfChangeMaterialAlphaModeDescriptor = { id: 's.stdio.gltf.mutation.change-material-alpha-mode.v1', version: 1, touchedPaths: ['document/materials/{material}/alphaMode'] } as const;
export const touchedPathsGltfChangeMaterialAlphaMode = (payload: GltfChangeMaterialAlphaModePayload): string[] => [`document/materials/${payload.material}/alphaMode`];
export interface GltfChangeMaterialAlphaModePayload { material: number; alphaMode: GltfAlphaMode }
export interface GltfChangeMaterialAlphaModeRejection { code: string; path: string; detail: string }
export type GltfChangeMaterialAlphaModeResult = { accepted: true; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfChangeMaterialAlphaModeRejection };
const failure = (value: GltfMaterialAnimationFailure): GltfChangeMaterialAlphaModeRejection => ({ ...value });
export const validateGltfChangeMaterialAlphaMode = (payload: GltfChangeMaterialAlphaModePayload, base: GltfSnapshot): GltfChangeMaterialAlphaModeRejection | undefined => {
  const target = index(base.document.materials.length, payload.material, 'document/materials'); if (target) return failure(target);
  if (base.document.materials[payload.material].alphaMode === payload.alphaMode) return { code: 'gltf.mutation.no-observable-change', path: `document/materials/${payload.material}/alphaMode`, detail: 'alphaMode already has that value' };
};
export const applyGltfChangeMaterialAlphaMode = (snapshot: GltfSnapshot, payload: GltfChangeMaterialAlphaModePayload): GltfChangeMaterialAlphaModeResult => {
  const rejection = validateGltfChangeMaterialAlphaMode(payload, snapshot); if (rejection) return { accepted: false, rejection };
  snapshot.document.materials[payload.material].alphaMode = payload.alphaMode;
  return { accepted: true, touchedPaths: touchedPathsGltfChangeMaterialAlphaMode(payload) };
};
