/** 🔺️ create-primitive: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfCreatePrimitive, type GltfCreatePrimitivePayload } from '../../create-primitive/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfCreatePrimitiveDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfCreatePrimitiveDiff = (base: GltfSnapshot, payload: GltfCreatePrimitivePayload): GltfCreatePrimitiveDiffResult => { const applied = applyGltfCreatePrimitive(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
