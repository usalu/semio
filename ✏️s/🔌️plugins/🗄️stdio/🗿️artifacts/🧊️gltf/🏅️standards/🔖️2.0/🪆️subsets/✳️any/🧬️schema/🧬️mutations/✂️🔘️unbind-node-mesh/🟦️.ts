/** 🦠️ unbind-node-mesh is an atomic, typed glTF 2.0 command. */
import type { GltfJson, GltfSnapshot, GltfPrimitive, GltfMorphTarget, GltfAccessor, GltfSparseAccessor, GltfSparseIndices, GltfSparseValues } from '../../📸️snapshot/🟦️.ts';
import { run, reject, positionIn, itemIndex, moveItem, type GltfLeafResult, type GltfMutationRejection } from '../../🔨️modules/🧬️mutation-support/📚️top-level/🟦️.ts';
export const GltfUnbindNodeMeshDescriptor = { id: 's.stdio.gltf.mutation.unbind-node-mesh.v1', version: 1, kind: 'unbind', touchedPaths: ["document/nodes/*/mesh"], referencePolicy: 'clears only the optional node mesh reference' } as const;
export interface GltfUnbindNodeMeshPayload { node: number }
export type GltfUnbindNodeMeshResult = GltfLeafResult;
export const validateGltfUnbindNodeMesh = (payload: GltfUnbindNodeMeshPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const node = itemIndex(payload.node, base.document.nodes.length, 'document/nodes'); if (node) return node; if (base.document.nodes[payload.node]!.mesh === undefined) return reject('gltf.mutation.relation-absent', `document/nodes/${payload.node}/mesh`, 'node has no mesh binding'); return undefined; };
export const applyGltfUnbindNodeMesh = (base: GltfSnapshot, payload: GltfUnbindNodeMeshPayload): GltfUnbindNodeMeshResult => run(base, payload, validateGltfUnbindNodeMesh, (next, payload) => { next.document.nodes[payload.node]!.mesh = undefined; }, GltfUnbindNodeMeshDescriptor.touchedPaths);
