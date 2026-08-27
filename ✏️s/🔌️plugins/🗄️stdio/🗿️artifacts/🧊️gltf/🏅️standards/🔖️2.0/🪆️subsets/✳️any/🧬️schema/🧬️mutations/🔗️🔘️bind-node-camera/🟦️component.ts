/** 🦠️ bind-node-camera is an atomic, typed glTF 2.0 command. */
import type { GltfJson, GltfSnapshot, GltfPrimitive, GltfMorphTarget, GltfAccessor, GltfSparseAccessor, GltfSparseIndices, GltfSparseValues } from '../../📸️snapshot/🟦️component.ts';
import { run, reject, positionIn, itemIndex, moveItem, type GltfLeafResult, type GltfMutationRejection } from '../../🔨️modules/🧬️mutation-support/📚️top-level/🟦️component.ts';
export const GltfBindNodeCameraDescriptor = { id: 's.stdio.gltf.mutation.bind-node-camera.v1', version: 1, kind: 'bind', touchedPaths: ["document/nodes/*/camera"], referencePolicy: 'validates the typed camera reference' } as const;
export interface GltfBindNodeCameraPayload { node: number; camera: number }
export type GltfBindNodeCameraResult = GltfLeafResult;
export const validateGltfBindNodeCamera = (payload: GltfBindNodeCameraPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const node = itemIndex(payload.node, base.document.nodes.length, 'document/nodes'); if (node) return node; const target = itemIndex(payload.camera, base.document.cameras.length, 'document/cameras'); if (target) return target; return undefined; };
export const applyGltfBindNodeCamera = (base: GltfSnapshot, payload: GltfBindNodeCameraPayload): GltfBindNodeCameraResult => run(base, payload, validateGltfBindNodeCamera, (next, payload) => { next.document.nodes[payload.node]!.camera = payload.camera; }, GltfBindNodeCameraDescriptor.touchedPaths);
