/** ↩️ unbind-primitive-material: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfUnbindPrimitiveMaterial, type GltfUnbindPrimitiveMaterialPayload } from '../../unbind-primitive-material/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfUnbindPrimitiveMaterialInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfUnbindPrimitiveMaterialInverse = (base: GltfSnapshot, payload: GltfUnbindPrimitiveMaterialPayload): GltfUnbindPrimitiveMaterialInverseResult => { const applied = applyGltfUnbindPrimitiveMaterial(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
