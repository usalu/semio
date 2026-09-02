/** 🦠️ change-node-extension-data is an atomic, typed glTF 2.0 command. */
import type { GltfJson, GltfSnapshot, GltfPrimitive, GltfMorphTarget, GltfAccessor, GltfSparseAccessor, GltfSparseIndices, GltfSparseValues } from '../../📸️snapshot/🟦️.ts';
import { run, reject, positionIn, itemIndex, moveItem, type GltfLeafResult, type GltfMutationRejection } from '../../🔨️modules/🧬️mutation-support/📚️top-level/🟦️.ts';
export const GltfChangeNodeExtensionDataDescriptor = { id: 's.stdio.gltf.mutation.change-node-extension-data.v1', version: 1, kind: 'change', touchedPaths: ["document/nodes/*/extensions"], referencePolicy: 'none' } as const;
export type GltfDataPresence = { state: 'absent' } | { state: 'present'; value: GltfJson };
export interface GltfChangeNodeExtensionDataPayload { node: number; data: GltfDataPresence }
export type GltfChangeNodeExtensionDataResult = GltfLeafResult;
export const validateGltfChangeNodeExtensionData = (payload: GltfChangeNodeExtensionDataPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const node = itemIndex(payload.node, base.document.nodes.length, 'document/nodes'); if (node) return node; return undefined; };
export const applyGltfChangeNodeExtensionData = (base: GltfSnapshot, payload: GltfChangeNodeExtensionDataPayload): GltfChangeNodeExtensionDataResult => run(base, payload, validateGltfChangeNodeExtensionData, (next, payload) => { next.document.nodes[payload.node]!.extensions = payload.data.state === 'present' ? payload.data.value : undefined; }, GltfChangeNodeExtensionDataDescriptor.touchedPaths);
