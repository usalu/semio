/** 🦠️ change-mesh-extra-data: cohesive atomic mesh mutation. */
import type { GltfJson, GltfSnapshot, GltfPrimitive, GltfMorphTarget } from '../../📸️snapshot/🟦️.ts';
import { run, reject, positionIn, itemIndex, permutation, moveItem, type GltfLeafResult, type GltfMutationRejection } from '../../🔨️modules/🧬️mutation-support/📚️top-level/🟦️.ts';
export const GltfChangeMeshExtraDataDescriptor = { id: 's.stdio.gltf.mutation.change-mesh-extra-data.v1', version: 1, kind: 'change', touchedPaths: ["document/meshes/*/extras"], referencePolicy: 'none' } as const;
export type GltfDataPresence = { state: 'absent' } | { state: 'present'; value: GltfJson };
export interface GltfChangeMeshExtraDataPayload { mesh: number; data: GltfDataPresence }
export type GltfChangeMeshExtraDataResult = GltfLeafResult;
export const validateGltfChangeMeshExtraData = (payload: GltfChangeMeshExtraDataPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const mesh = itemIndex(payload.mesh, base.document.meshes.length, 'document/meshes'); if (mesh) return mesh; return undefined; };
export const applyGltfChangeMeshExtraData = (base: GltfSnapshot, payload: GltfChangeMeshExtraDataPayload): GltfChangeMeshExtraDataResult => run(base, payload, validateGltfChangeMeshExtraData, (next, payload) => { next.document.meshes[payload.mesh]!.extras = payload.data.state === 'present' ? payload.data.value : undefined; }, GltfChangeMeshExtraDataDescriptor.touchedPaths);
