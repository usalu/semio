/** 🔺️ unbind-primitive-material: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfUnbindPrimitiveMaterial, type GltfUnbindPrimitiveMaterialPayload } from '../../unbind-primitive-material/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfUnbindPrimitiveMaterialDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfUnbindPrimitiveMaterialDiff = (base: GltfSnapshot, payload: GltfUnbindPrimitiveMaterialPayload): GltfUnbindPrimitiveMaterialDiffResult => { const applied = applyGltfUnbindPrimitiveMaterial(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
