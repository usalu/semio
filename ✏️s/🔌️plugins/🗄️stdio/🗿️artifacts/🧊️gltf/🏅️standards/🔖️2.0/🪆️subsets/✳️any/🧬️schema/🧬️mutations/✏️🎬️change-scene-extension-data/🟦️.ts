/** 🦠️ change-scene-extension-data is an atomic, typed glTF 2.0 command. */
import type { GltfJson, GltfSnapshot, GltfPrimitive, GltfMorphTarget, GltfAccessor, GltfSparseAccessor, GltfSparseIndices, GltfSparseValues } from '../../📸️snapshot/🟦️.ts';
import { run, reject, positionIn, itemIndex, moveItem, type GltfLeafResult, type GltfMutationRejection } from './🟦️';
export const GltfChangeSceneExtensionDataDescriptor = { id: 's.stdio.gltf.mutation.change-scene-extension-data.v1', version: 1, kind: 'change', touchedPaths: ["document/scenes/*/extensions"], referencePolicy: 'none' } as const;
export type GltfDataPresence = { state: 'absent' } | { state: 'present'; value: GltfJson };
export interface GltfChangeSceneExtensionDataPayload { scene: number; data: GltfDataPresence }
export type GltfChangeSceneExtensionDataResult = GltfLeafResult;
export const validateGltfChangeSceneExtensionData = (payload: GltfChangeSceneExtensionDataPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const scene = itemIndex(payload.scene, base.document.scenes.length, 'document/scenes'); if (scene) return scene; return undefined; };
export const applyGltfChangeSceneExtensionData = (base: GltfSnapshot, payload: GltfChangeSceneExtensionDataPayload): GltfChangeSceneExtensionDataResult => run(base, payload, validateGltfChangeSceneExtensionData, (next, payload) => { next.document.scenes[payload.scene]!.extensions = payload.data.state === 'present' ? payload.data.value : undefined; }, GltfChangeSceneExtensionDataDescriptor.touchedPaths);
