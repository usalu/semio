/** 🦠️ create-primitive: cohesive atomic mesh mutation. */
import type { GltfJson, GltfSnapshot, GltfPrimitive, GltfMorphTarget } from '../../📸️snapshot/🟦️.ts';
import { run, reject, positionIn, itemIndex, permutation, moveItem, type GltfLeafResult, type GltfMutationRejection } from '../../🔨️modules/🧬️mutation-support/📚️top-level/🟦️.ts';
export const GltfCreatePrimitiveDescriptor = { id: 's.stdio.gltf.mutation.create-primitive.v1', version: 1, kind: 'create', touchedPaths: ["document/meshes/*/primitives"], referencePolicy: 'creates an empty primitive only at a valid mesh-local position' } as const;
export interface GltfCreatePrimitivePayload { mesh: number; position: number }
export type GltfCreatePrimitiveResult = GltfLeafResult;
export const validateGltfCreatePrimitive = (payload: GltfCreatePrimitivePayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const mesh = itemIndex(payload.mesh, base.document.meshes.length, 'document/meshes'); if (mesh) return mesh; const position = positionIn(payload.position, base.document.meshes[payload.mesh]!.primitives.length, `document/meshes/${payload.mesh}/primitives`); if (position) return position; return undefined; };
export const applyGltfCreatePrimitive = (base: GltfSnapshot, payload: GltfCreatePrimitivePayload): GltfCreatePrimitiveResult => run(base, payload, validateGltfCreatePrimitive, (next, payload) => { next.document.meshes[payload.mesh]!.primitives.splice(payload.position, 0, { attributes: {}, targets: [] } as GltfPrimitive); }, GltfCreatePrimitiveDescriptor.touchedPaths);
