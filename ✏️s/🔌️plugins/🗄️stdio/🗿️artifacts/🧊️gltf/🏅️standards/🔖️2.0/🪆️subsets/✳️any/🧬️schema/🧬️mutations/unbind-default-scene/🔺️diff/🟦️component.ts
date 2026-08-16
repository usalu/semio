/** 🔺️ unbind-default-scene direct sparse diff. */
import type { GltfDiff } from '../../../🔺️diff/🟦️component.ts';
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfUnbindDefaultScene, type GltfUnbindDefaultScenePayload } from '../../unbind-default-scene/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const deriveGltfUnbindDefaultSceneDiff = (base: GltfSnapshot, payload: GltfUnbindDefaultScenePayload): { accepted: true; diff: GltfDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfMutationRejection } => { const applied = applyGltfUnbindDefaultScene(base, payload); return applied.accepted ? { accepted: true, diff: { scene: null }, touchedPaths: GltfUnbindDefaultSceneDescriptor.touchedPaths } : applied; };
export const GltfUnbindDefaultSceneDescriptor = { id: 's.stdio.gltf.mutation.unbind-default-scene.v1', touchedPaths: ["document/scene"] } as const;
