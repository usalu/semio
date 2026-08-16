/** 🔺️ bind-morph-target-attribute: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfBindMorphTargetAttribute, type GltfBindMorphTargetAttributePayload } from '../../bind-morph-target-attribute/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfBindMorphTargetAttributeDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfBindMorphTargetAttributeDiff = (base: GltfSnapshot, payload: GltfBindMorphTargetAttributePayload): GltfBindMorphTargetAttributeDiffResult => { const applied = applyGltfBindMorphTargetAttribute(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
