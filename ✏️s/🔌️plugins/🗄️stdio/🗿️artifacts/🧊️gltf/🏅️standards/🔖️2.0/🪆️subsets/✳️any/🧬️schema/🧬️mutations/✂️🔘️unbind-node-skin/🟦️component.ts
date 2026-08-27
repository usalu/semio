/** 🦠️ unbind-node-skin is an atomic, typed glTF 2.0 command. */
import type { GltfJson, GltfSnapshot, GltfPrimitive, GltfMorphTarget, GltfAccessor, GltfSparseAccessor, GltfSparseIndices, GltfSparseValues } from '../../📸️snapshot/🟦️component.ts';
import { run, reject, positionIn, itemIndex, moveItem, type GltfLeafResult, type GltfMutationRejection } from '../../🔨️modules/🧬️mutation-support/📚️top-level/🟦️component.ts';
export const GltfUnbindNodeSkinDescriptor = { id: 's.stdio.gltf.mutation.unbind-node-skin.v1', version: 1, kind: 'unbind', touchedPaths: ["document/nodes/*/skin"], referencePolicy: 'clears only the optional node skin reference' } as const;
export interface GltfUnbindNodeSkinPayload { node: number }
export type GltfUnbindNodeSkinResult = GltfLeafResult;
export const validateGltfUnbindNodeSkin = (payload: GltfUnbindNodeSkinPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const node = itemIndex(payload.node, base.document.nodes.length, 'document/nodes'); if (node) return node; if (base.document.nodes[payload.node]!.skin === undefined) return reject('gltf.mutation.relation-absent', `document/nodes/${payload.node}/skin`, 'node has no skin binding'); return undefined; };
export const applyGltfUnbindNodeSkin = (base: GltfSnapshot, payload: GltfUnbindNodeSkinPayload): GltfUnbindNodeSkinResult => run(base, payload, validateGltfUnbindNodeSkin, (next, payload) => { next.document.nodes[payload.node]!.skin = undefined; }, GltfUnbindNodeSkinDescriptor.touchedPaths);
