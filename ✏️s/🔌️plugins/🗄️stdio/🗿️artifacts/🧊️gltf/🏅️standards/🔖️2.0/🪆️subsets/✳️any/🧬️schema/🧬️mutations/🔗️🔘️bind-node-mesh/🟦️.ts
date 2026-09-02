/** 🦠️ bind-node-mesh is an atomic, typed glTF 2.0 command. */
import type { GltfJson, GltfSnapshot, GltfPrimitive, GltfMorphTarget, GltfAccessor, GltfSparseAccessor, GltfSparseIndices, GltfSparseValues } from '../../📸️snapshot/🟦️.ts';
import { run, reject, positionIn, itemIndex, moveItem, type GltfLeafResult, type GltfMutationRejection } from '../../🔨️modules/🧬️mutation-support/📚️top-level/🟦️.ts';
export const GltfBindNodeMeshDescriptor = { id: 's.stdio.gltf.mutation.bind-node-mesh.v1', version: 1, kind: 'bind', touchedPaths: ["document/nodes/*/mesh"], referencePolicy: 'validates the typed mesh reference' } as const;
export interface GltfBindNodeMeshPayload { node: number; mesh: number }
export type GltfBindNodeMeshResult = GltfLeafResult;
export const validateGltfBindNodeMesh = (payload: GltfBindNodeMeshPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const node = itemIndex(payload.node, base.document.nodes.length, 'document/nodes'); if (node) return node; const target = itemIndex(payload.mesh, base.document.meshes.length, 'document/meshes'); if (target) return target; return undefined; };
export const applyGltfBindNodeMesh = (base: GltfSnapshot, payload: GltfBindNodeMeshPayload): GltfBindNodeMeshResult => run(base, payload, validateGltfBindNodeMesh, (next, payload) => { next.document.nodes[payload.node]!.mesh = payload.mesh; }, GltfBindNodeMeshDescriptor.touchedPaths);
