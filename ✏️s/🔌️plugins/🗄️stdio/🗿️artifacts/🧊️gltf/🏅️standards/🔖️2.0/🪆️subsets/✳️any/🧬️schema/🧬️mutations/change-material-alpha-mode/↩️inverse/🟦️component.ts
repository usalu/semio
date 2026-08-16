/** ↩️ change-material-alpha-mode reconstructs the prior alpha-mode value. */
import type { GltfAlphaMode, GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfChangeMaterialAlphaMode, type GltfChangeMaterialAlphaModePayload, type GltfChangeMaterialAlphaModeRejection } from '../../change-material-alpha-mode/🦠️mutation/🟦️component.ts';
export interface GltfChangeMaterialAlphaModeInverse { material: number; expectedAlphaMode: GltfAlphaMode; alphaMode: GltfAlphaMode; touchedPaths: readonly string[] }
export const touchedPathsGltfChangeMaterialAlphaModeInverse = (inverse: GltfChangeMaterialAlphaModeInverse): string[] => [`document/materials/${inverse.material}/alphaMode`];
export const reconstructGltfChangeMaterialAlphaModeInverse = (base: GltfSnapshot, payload: GltfChangeMaterialAlphaModePayload): { accepted: true; inverse: GltfChangeMaterialAlphaModeInverse; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfChangeMaterialAlphaModeRejection } => {
  const rejection = validateGltfChangeMaterialAlphaMode(payload, base);
  if (rejection) return { accepted: false, rejection };
  const touchedPaths = [`document/materials/${payload.material}/alphaMode`];
  return { accepted: true, inverse: { material: payload.material, expectedAlphaMode: payload.alphaMode, alphaMode: base.document.materials[payload.material].alphaMode, touchedPaths }, touchedPaths };
};
export const applyGltfChangeMaterialAlphaModeInverse = (snapshot: GltfSnapshot, inverse: GltfChangeMaterialAlphaModeInverse): GltfChangeMaterialAlphaModeRejection | undefined => {
  const expectedPaths = touchedPathsGltfChangeMaterialAlphaModeInverse(inverse);
  if (JSON.stringify(inverse.touchedPaths) !== JSON.stringify(expectedPaths)) return { code: 'gltf.mutation.invalid-touched-paths', path: 'inverse/touchedPaths', detail: 'touched paths must equal the concrete material alpha-mode path' };
  const material = snapshot.document.materials[inverse.material]; if (!material) return { code: 'gltf.mutation.index-out-of-range', path: 'document/materials', detail: 'the addressed index must exist' };
  if (material.alphaMode !== inverse.expectedAlphaMode) return { code: 'gltf.mutation.stale-inverse', path: `document/materials/${inverse.material}/alphaMode`, detail: 'current alpha mode does not equal the planned forward result' };
  material.alphaMode = inverse.alphaMode;
};
