/** 🔺️ reorder-primitives: direct sparse mesh diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfReorderPrimitives, type GltfReorderPrimitivesPayload } from '../../reorder-primitives/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export type GltfReorderPrimitivesDiffResult = { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection };
export const deriveGltfReorderPrimitivesDiff = (base: GltfSnapshot, payload: GltfReorderPrimitivesPayload): GltfReorderPrimitivesDiffResult => { const applied = applyGltfReorderPrimitives(base, payload); return applied.accepted ? { accepted: true, diff: applied.diff, touchedPaths: applied.touchedPaths } : applied; };
