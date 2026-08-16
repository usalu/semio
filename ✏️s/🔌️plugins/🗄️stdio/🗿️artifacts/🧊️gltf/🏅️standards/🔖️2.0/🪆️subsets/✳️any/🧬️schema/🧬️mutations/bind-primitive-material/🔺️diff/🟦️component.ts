/** 🔺️ bind-primitive-material: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfBindPrimitiveMaterial, type GltfBindPrimitiveMaterialPayload } from '../../bind-primitive-material/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfBindPrimitiveMaterialDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfBindPrimitiveMaterialDiff = (base: GltfSnapshot, payload: GltfBindPrimitiveMaterialPayload): GltfBindPrimitiveMaterialDiffResult => { const applied = applyGltfBindPrimitiveMaterial(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
