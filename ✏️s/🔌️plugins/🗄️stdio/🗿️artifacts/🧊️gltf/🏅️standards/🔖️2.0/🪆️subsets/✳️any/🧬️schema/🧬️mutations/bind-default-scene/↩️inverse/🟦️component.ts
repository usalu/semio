/** ↩️ bind-default-scene direct inverse from the base snapshot. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfBindDefaultScene, type GltfBindDefaultScenePayload } from '../../bind-default-scene/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfBindDefaultSceneInverse = (base: GltfSnapshot, payload: GltfBindDefaultScenePayload): { accepted: true; inverse: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfBindDefaultScene(base, payload); return applied.accepted ? { accepted: true, inverse: { scene: base.document.scene ?? null }, touchedPaths: ["document/scene"] } : applied; };
