/** 🦠️ change-scene-extra-data is an atomic, typed glTF 2.0 command. */
import type { GltfJson, GltfSnapshot, GltfPrimitive, GltfMorphTarget, GltfAccessor, GltfSparseAccessor, GltfSparseIndices, GltfSparseValues } from '../../📸️snapshot/🟦️.ts';
import { run, reject, positionIn, itemIndex, moveItem, type GltfLeafResult, type GltfMutationRejection } from '../../🔨️modules/🧬️mutation-support/📚️top-level/🟦️.ts';
export const GltfChangeSceneExtraDataDescriptor = { id: 's.stdio.gltf.mutation.change-scene-extra-data.v1', version: 1, kind: 'change', touchedPaths: ["document/scenes/*/extras"], referencePolicy: 'none' } as const;
export type GltfDataPresence = { state: 'absent' } | { state: 'present'; value: GltfJson };
export interface GltfChangeSceneExtraDataPayload { scene: number; data: GltfDataPresence }
export type GltfChangeSceneExtraDataResult = GltfLeafResult;
export const validateGltfChangeSceneExtraData = (payload: GltfChangeSceneExtraDataPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const scene = itemIndex(payload.scene, base.document.scenes.length, 'document/scenes'); if (scene) return scene; return undefined; };
export const applyGltfChangeSceneExtraData = (base: GltfSnapshot, payload: GltfChangeSceneExtraDataPayload): GltfChangeSceneExtraDataResult => run(base, payload, validateGltfChangeSceneExtraData, (next, payload) => { next.document.scenes[payload.scene]!.extras = payload.data.state === 'present' ? payload.data.value : undefined; }, GltfChangeSceneExtraDataDescriptor.touchedPaths);
