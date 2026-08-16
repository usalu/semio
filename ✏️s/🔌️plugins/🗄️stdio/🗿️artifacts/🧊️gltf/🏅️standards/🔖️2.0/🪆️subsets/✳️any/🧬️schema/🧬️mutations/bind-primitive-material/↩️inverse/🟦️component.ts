/** ↩️ bind-primitive-material: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfBindPrimitiveMaterial, type GltfBindPrimitiveMaterialPayload } from '../../bind-primitive-material/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfBindPrimitiveMaterialInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfBindPrimitiveMaterialInverse = (base: GltfSnapshot, payload: GltfBindPrimitiveMaterialPayload): GltfBindPrimitiveMaterialInverseResult => { const applied = applyGltfBindPrimitiveMaterial(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
