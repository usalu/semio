/** 🦠️ change-primitive-extra-data: cohesive atomic mesh mutation. */
import type { GltfJson, GltfSnapshot, GltfPrimitive, GltfMorphTarget } from '../../📸️snapshot/🟦️component.ts';
import { run, reject, positionIn, itemIndex, permutation, moveItem, type GltfLeafResult, type GltfMutationRejection } from '../../🔨️modules/🧬️mutation-support/📚️top-level/🟦️component.ts';
export const GltfChangePrimitiveExtraDataDescriptor = { id: 's.stdio.gltf.mutation.change-primitive-extra-data.v1', version: 1, kind: 'change', touchedPaths: ["document/meshes/*/primitives/*/extras"], referencePolicy: 'none' } as const;
export type GltfDataPresence = { state: 'absent' } | { state: 'present'; value: GltfJson };
export interface GltfChangePrimitiveExtraDataPayload { mesh: number; primitive: number; data: GltfDataPresence }
export type GltfChangePrimitiveExtraDataResult = GltfLeafResult;
export const validateGltfChangePrimitiveExtraData = (payload: GltfChangePrimitiveExtraDataPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const mesh = itemIndex(payload.mesh, base.document.meshes.length, 'document/meshes'); if (mesh) return mesh; const primitive = itemIndex(payload.primitive, base.document.meshes[payload.mesh]!.primitives.length, `document/meshes/${payload.mesh}/primitives`); if (primitive) return primitive; return undefined; };
export const applyGltfChangePrimitiveExtraData = (base: GltfSnapshot, payload: GltfChangePrimitiveExtraDataPayload): GltfChangePrimitiveExtraDataResult => run(base, payload, validateGltfChangePrimitiveExtraData, (next, payload) => { next.document.meshes[payload.mesh]!.primitives[payload.primitive]!.extras = payload.data.state === 'present' ? payload.data.value : undefined; }, GltfChangePrimitiveExtraDataDescriptor.touchedPaths);
