/** 🦠️ change-mesh-extension-data: cohesive atomic mesh mutation. */
import type { GltfJson, GltfSnapshot, GltfPrimitive, GltfMorphTarget } from '../../📸️snapshot/🟦️.ts';
import { run, reject, positionIn, itemIndex, permutation, moveItem, type GltfLeafResult, type GltfMutationRejection } from '../../🔨️modules/🧬️mutation-support/📚️top-level/🟦️.ts';
export const GltfChangeMeshExtensionDataDescriptor = { id: 's.stdio.gltf.mutation.change-mesh-extension-data.v1', version: 1, kind: 'change', touchedPaths: ["document/meshes/*/extensions"], referencePolicy: 'none' } as const;
export type GltfDataPresence = { state: 'absent' } | { state: 'present'; value: GltfJson };
export interface GltfChangeMeshExtensionDataPayload { mesh: number; data: GltfDataPresence }
export type GltfChangeMeshExtensionDataResult = GltfLeafResult;
export const validateGltfChangeMeshExtensionData = (payload: GltfChangeMeshExtensionDataPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const mesh = itemIndex(payload.mesh, base.document.meshes.length, 'document/meshes'); if (mesh) return mesh; return undefined; };
export const applyGltfChangeMeshExtensionData = (base: GltfSnapshot, payload: GltfChangeMeshExtensionDataPayload): GltfChangeMeshExtensionDataResult => run(base, payload, validateGltfChangeMeshExtensionData, (next, payload) => { next.document.meshes[payload.mesh]!.extensions = payload.data.state === 'present' ? payload.data.value : undefined; }, GltfChangeMeshExtensionDataDescriptor.touchedPaths);
