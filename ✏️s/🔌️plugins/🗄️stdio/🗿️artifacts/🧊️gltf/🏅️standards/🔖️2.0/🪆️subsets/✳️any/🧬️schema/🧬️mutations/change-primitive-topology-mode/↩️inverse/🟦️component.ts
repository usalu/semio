/** ↩️ change-primitive-topology-mode: exact-base undo diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangePrimitiveTopologyMode, type GltfChangePrimitiveTopologyModePayload } from '../../change-primitive-topology-mode/🦠️mutation/🟦️component.ts';
import { topLevelDiff, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfChangePrimitiveTopologyModeInverseResult = { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfChangePrimitiveTopologyModeInverse = (base: GltfSnapshot, payload: GltfChangePrimitiveTopologyModePayload): GltfChangePrimitiveTopologyModeInverseResult => { const applied = applyGltfChangePrimitiveTopologyMode(base, payload); return applied.accepted ? { accepted: true, inverse: topLevelDiff(applied.snapshot, base), touchedPaths: applied.touchedPaths } : applied; };
