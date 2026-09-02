/** 🦠️ delete-primitive: cohesive atomic mesh mutation. */
import type { GltfJson, GltfSnapshot, GltfPrimitive, GltfMorphTarget } from '../../📸️snapshot/🟦️.ts';
import { run, reject, positionIn, itemIndex, permutation, moveItem, type GltfLeafResult, type GltfMutationRejection } from './🟦️';
export const GltfDeletePrimitiveDescriptor = { id: 's.stdio.gltf.mutation.delete-primitive.v1', version: 1, kind: 'delete', touchedPaths: ["document/meshes/*/primitives"], referencePolicy: 'removes one primitive; mesh weights stay coherent because target counts are unchanged' } as const;
export interface GltfDeletePrimitivePayload { mesh: number; primitive: number }
export type GltfDeletePrimitiveResult = GltfLeafResult;
export const validateGltfDeletePrimitive = (payload: GltfDeletePrimitivePayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const mesh = itemIndex(payload.mesh, base.document.meshes.length, 'document/meshes'); if (mesh) return mesh; const primitive = itemIndex(payload.primitive, base.document.meshes[payload.mesh]!.primitives.length, `document/meshes/${payload.mesh}/primitives`); if (primitive) return primitive; return undefined; };
export const applyGltfDeletePrimitive = (base: GltfSnapshot, payload: GltfDeletePrimitivePayload): GltfDeletePrimitiveResult => run(base, payload, validateGltfDeletePrimitive, (next, payload) => { next.document.meshes[payload.mesh]!.primitives.splice(payload.primitive, 1); }, GltfDeletePrimitiveDescriptor.touchedPaths);
