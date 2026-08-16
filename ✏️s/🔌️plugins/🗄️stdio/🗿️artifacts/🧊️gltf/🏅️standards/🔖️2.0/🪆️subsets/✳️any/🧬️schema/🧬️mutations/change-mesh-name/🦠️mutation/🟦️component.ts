/** 🦠️ change-mesh-name: cohesive atomic mesh mutation. */
import type { GltfJson, GltfSnapshot, GltfPrimitive, GltfMorphTarget } from '../../../📸️snapshot/🟦️component.ts';
import { run, reject, positionIn, itemIndex, permutation, moveItem, type GltfLeafResult, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const GltfChangeMeshNameDescriptor = { id: 's.stdio.gltf.mutation.change-mesh-name.v1', version: 1, kind: 'change', touchedPaths: ["document/meshes/*/name"], referencePolicy: 'none' } as const;
export interface GltfChangeMeshNamePayload { mesh: number; value: string | null }
export type GltfChangeMeshNameResult = GltfLeafResult;
export const validateGltfChangeMeshName = (payload: GltfChangeMeshNamePayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const mesh = itemIndex(payload.mesh, base.document.meshes.length, 'document/meshes'); if (mesh) return mesh; return undefined; };
export const applyGltfChangeMeshName = (base: GltfSnapshot, payload: GltfChangeMeshNamePayload): GltfChangeMeshNameResult => run(base, payload, validateGltfChangeMeshName, (next, payload) => { next.document.meshes[payload.mesh]!.name = payload.value ?? undefined; }, GltfChangeMeshNameDescriptor.touchedPaths);
