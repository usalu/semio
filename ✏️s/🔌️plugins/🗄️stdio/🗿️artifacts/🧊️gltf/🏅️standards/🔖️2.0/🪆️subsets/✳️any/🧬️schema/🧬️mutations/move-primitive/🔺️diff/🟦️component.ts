/** 🔺️ move-primitive: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfMovePrimitive, type GltfMovePrimitivePayload } from '../../move-primitive/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfMovePrimitiveDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfMovePrimitiveDiff = (base: GltfSnapshot, payload: GltfMovePrimitivePayload): GltfMovePrimitiveDiffResult => { const applied = applyGltfMovePrimitive(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
