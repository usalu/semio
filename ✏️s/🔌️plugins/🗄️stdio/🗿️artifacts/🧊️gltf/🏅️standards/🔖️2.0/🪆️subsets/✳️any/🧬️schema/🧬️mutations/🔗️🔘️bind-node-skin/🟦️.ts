/** 🦠️ bind-node-skin is an atomic, typed glTF 2.0 command. */
import type { GltfJson, GltfSnapshot, GltfPrimitive, GltfMorphTarget, GltfAccessor, GltfSparseAccessor, GltfSparseIndices, GltfSparseValues } from '../../📸️snapshot/🟦️.ts';
import { run, reject, positionIn, itemIndex, moveItem, type GltfLeafResult, type GltfMutationRejection } from './🟦️';
export const GltfBindNodeSkinDescriptor = { id: 's.stdio.gltf.mutation.bind-node-skin.v1', version: 1, kind: 'bind', touchedPaths: ["document/nodes/*/skin"], referencePolicy: 'validates the typed skin reference' } as const;
export interface GltfBindNodeSkinPayload { node: number; skin: number }
export type GltfBindNodeSkinResult = GltfLeafResult;
export const validateGltfBindNodeSkin = (payload: GltfBindNodeSkinPayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const node = itemIndex(payload.node, base.document.nodes.length, 'document/nodes'); if (node) return node; const target = itemIndex(payload.skin, base.document.skins.length, 'document/skins'); if (target) return target; return undefined; };
export const applyGltfBindNodeSkin = (base: GltfSnapshot, payload: GltfBindNodeSkinPayload): GltfBindNodeSkinResult => run(base, payload, validateGltfBindNodeSkin, (next, payload) => { next.document.nodes[payload.node]!.skin = payload.skin; }, GltfBindNodeSkinDescriptor.touchedPaths);
