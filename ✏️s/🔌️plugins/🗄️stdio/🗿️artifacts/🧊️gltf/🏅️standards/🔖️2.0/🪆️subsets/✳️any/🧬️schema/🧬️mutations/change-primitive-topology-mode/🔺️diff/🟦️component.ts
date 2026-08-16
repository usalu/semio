/** 🔺️ change-primitive-topology-mode: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangePrimitiveTopologyMode, type GltfChangePrimitiveTopologyModePayload } from '../../change-primitive-topology-mode/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfChangePrimitiveTopologyModeDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfChangePrimitiveTopologyModeDiff = (base: GltfSnapshot, payload: GltfChangePrimitiveTopologyModePayload): GltfChangePrimitiveTopologyModeDiffResult => { const applied = applyGltfChangePrimitiveTopologyMode(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
