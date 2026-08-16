/** 🔺️ delete-primitive: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfDeletePrimitive, type GltfDeletePrimitivePayload } from '../../delete-primitive/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfDeletePrimitiveDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfDeletePrimitiveDiff = (base: GltfSnapshot, payload: GltfDeletePrimitivePayload): GltfDeletePrimitiveDiffResult => { const applied = applyGltfDeletePrimitive(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
