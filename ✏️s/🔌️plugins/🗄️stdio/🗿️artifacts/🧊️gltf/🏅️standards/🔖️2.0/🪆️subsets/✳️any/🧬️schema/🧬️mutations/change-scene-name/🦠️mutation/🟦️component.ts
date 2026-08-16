/** 🦠️ change-scene-name is an atomic, typed glTF 2.0 command. */
import type { GltfJson, GltfSnapshot, GltfPrimitive, GltfMorphTarget, GltfAccessor, GltfSparseAccessor, GltfSparseIndices, GltfSparseValues } from '../../../📸️snapshot/🟦️component.ts';
import { run, reject, positionIn, itemIndex, moveItem, type GltfLeafResult, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const GltfChangeSceneNameDescriptor = { id: 's.stdio.gltf.mutation.change-scene-name.v1', version: 1, kind: 'change', touchedPaths: ["document/scenes/*/name"], referencePolicy: 'none' } as const;
export interface GltfChangeSceneNamePayload { scene: number; value: string | null }
export type GltfChangeSceneNameResult = GltfLeafResult;
export const validateGltfChangeSceneName = (payload: GltfChangeSceneNamePayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const scene = itemIndex(payload.scene, base.document.scenes.length, 'document/scenes'); if (scene) return scene; return undefined; };
export const applyGltfChangeSceneName = (base: GltfSnapshot, payload: GltfChangeSceneNamePayload): GltfChangeSceneNameResult => run(base, payload, validateGltfChangeSceneName, (next, payload) => { next.document.scenes[payload.scene]!.name = payload.value ?? undefined; }, GltfChangeSceneNameDescriptor.touchedPaths);
