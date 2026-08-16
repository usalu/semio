/** 🔺️ change-material-alpha-mode owns a sparse alpha-mode diff. */
import type { GltfAlphaMode, GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfChangeMaterialAlphaMode, type GltfChangeMaterialAlphaModePayload, type GltfChangeMaterialAlphaModeRejection } from '../../change-material-alpha-mode/🦠️mutation/🟦️component.ts';
export interface GltfChangeMaterialAlphaModeDiff { material: number; expectedAlphaMode: GltfAlphaMode; alphaMode: GltfAlphaMode; touchedPaths: readonly string[] }
export const touchedPathsGltfChangeMaterialAlphaModeDiff = (diff: GltfChangeMaterialAlphaModeDiff): string[] => [`document/materials/${diff.material}/alphaMode`];
export const deriveGltfChangeMaterialAlphaModeDiff = (base: GltfSnapshot, payload: GltfChangeMaterialAlphaModePayload): { accepted: true; diff: GltfChangeMaterialAlphaModeDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfChangeMaterialAlphaModeRejection } => {
  const rejection = validateGltfChangeMaterialAlphaMode(payload, base);
  if (rejection) return { accepted: false, rejection };
  const touchedPaths = [`document/materials/${payload.material}/alphaMode`];
  return { accepted: true, diff: { material: payload.material, expectedAlphaMode: base.document.materials[payload.material].alphaMode, alphaMode: payload.alphaMode, touchedPaths }, touchedPaths };
};
export const applyGltfChangeMaterialAlphaModeDiff = (snapshot: GltfSnapshot, diff: GltfChangeMaterialAlphaModeDiff): GltfChangeMaterialAlphaModeRejection | undefined => {
  const expectedPaths = touchedPathsGltfChangeMaterialAlphaModeDiff(diff);
  if (JSON.stringify(diff.touchedPaths) !== JSON.stringify(expectedPaths)) return { code: 'gltf.mutation.invalid-touched-paths', path: 'diff/touchedPaths', detail: 'touched paths must equal the concrete material alpha-mode path' };
  const material = snapshot.document.materials[diff.material]; if (!material) return { code: 'gltf.mutation.index-out-of-range', path: 'document/materials', detail: 'the addressed index must exist' };
  if (material.alphaMode !== diff.expectedAlphaMode) return { code: 'gltf.mutation.stale-diff', path: `document/materials/${diff.material}/alphaMode`, detail: 'current alpha mode does not equal the planned pre-state' };
  material.alphaMode = diff.alphaMode;
};
