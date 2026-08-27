/** 🦠️ unbind-node-camera is an atomic, typed glTF 2.0 command. */
import type { GltfJson, GltfSnapshot, GltfPrimitive, GltfMorphTarget, GltfAccessor, GltfSparseAccessor, GltfSparseIndices, GltfSparseValues } from '../../📸️snapshot/🟦️component.ts';
import { run, reject, positionIn, itemIndex, moveItem, type GltfLeafResult, type GltfMutationRejection } from '../../🔨️modules/🧬️mutation-support/📚️top-level/🟦️component.ts';
export const GltfUnbindNodeCameraDescriptor = { id: 's.stdio.gltf.mutation.unbind-node-camera.v1', version: 1, kind: 'unbind', touchedPaths: ["document/nodes/*/camera"], referencePolicy: 'clears only the optional node camera reference' } as const;
export interface GltfUnbindNodeCameraPayload { node: number }
export type GltfUnbindNodeCameraResult = GltfLeafResult;
export const validateGltfUnbindNodeCamera = (payload: GltfUnbindNodeCameraPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const node = itemIndex(payload.node, base.document.nodes.length, 'document/nodes'); if (node) return node; if (base.document.nodes[payload.node]!.camera === undefined) return reject('gltf.mutation.relation-absent', `document/nodes/${payload.node}/camera`, 'node has no camera binding'); return undefined; };
export const applyGltfUnbindNodeCamera = (base: GltfSnapshot, payload: GltfUnbindNodeCameraPayload): GltfUnbindNodeCameraResult => run(base, payload, validateGltfUnbindNodeCamera, (next, payload) => { next.document.nodes[payload.node]!.camera = undefined; }, GltfUnbindNodeCameraDescriptor.touchedPaths);
