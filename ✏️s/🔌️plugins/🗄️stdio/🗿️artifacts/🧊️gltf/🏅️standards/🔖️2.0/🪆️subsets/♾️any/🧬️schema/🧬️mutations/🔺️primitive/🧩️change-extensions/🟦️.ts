/** 🦠️ change-primitive-extension-data: cohesive atomic mesh mutation. */
import type { GltfJson, GltfSnapshot, GltfPrimitive, GltfMorphTarget } from '../../📸️snapshot/🟦️.ts';
import { run, reject, positionIn, itemIndex, permutation, moveItem, type GltfLeafResult, type GltfMutationRejection } from './🟦️';
export const GltfChangePrimitiveExtensionDataDescriptor = { id: 's.stdio.gltf.mutation.change-primitive-extension-data.v1', version: 1, kind: 'change', touchedPaths: ["document/meshes/*/primitives/*/extensions"], referencePolicy: 'none' } as const;
export type GltfDataPresence = { state: 'absent' } | { state: 'present'; value: GltfJson };
export interface GltfChangePrimitiveExtensionDataPayload { mesh: number; primitive: number; data: GltfDataPresence }
export type GltfChangePrimitiveExtensionDataResult = GltfLeafResult;
export const validateGltfChangePrimitiveExtensionData = (payload: GltfChangePrimitiveExtensionDataPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const mesh = itemIndex(payload.mesh, base.document.meshes.length, 'document/meshes'); if (mesh) return mesh; const primitive = itemIndex(payload.primitive, base.document.meshes[payload.mesh]!.primitives.length, `document/meshes/${payload.mesh}/primitives`); if (primitive) return primitive; return undefined; };
export const applyGltfChangePrimitiveExtensionData = (base: GltfSnapshot, payload: GltfChangePrimitiveExtensionDataPayload): GltfChangePrimitiveExtensionDataResult => run(base, payload, validateGltfChangePrimitiveExtensionData, (next, payload) => { next.document.meshes[payload.mesh]!.primitives[payload.primitive]!.extensions = payload.data.state === 'present' ? payload.data.value : undefined; }, GltfChangePrimitiveExtensionDataDescriptor.touchedPaths);
