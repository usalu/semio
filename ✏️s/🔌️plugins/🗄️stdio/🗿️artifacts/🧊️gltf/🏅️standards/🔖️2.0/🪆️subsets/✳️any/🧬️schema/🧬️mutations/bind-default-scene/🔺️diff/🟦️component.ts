/** 🔺️ bind-default-scene direct sparse diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfBindDefaultScene, type GltfBindDefaultScenePayload } from '../../bind-default-scene/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfBindDefaultSceneDiff = (base: GltfSnapshot, payload: GltfBindDefaultScenePayload): { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfBindDefaultScene(base, payload); return applied.accepted ? { accepted: true, diff: { scene: payload.scene }, touchedPaths: GltfBindDefaultSceneDescriptor.touchedPaths } : applied; };
export const GltfBindDefaultSceneDescriptor = { id: 's.stdio.gltf.mutation.bind-default-scene.v1', touchedPaths: ["document/scene"] } as const;
