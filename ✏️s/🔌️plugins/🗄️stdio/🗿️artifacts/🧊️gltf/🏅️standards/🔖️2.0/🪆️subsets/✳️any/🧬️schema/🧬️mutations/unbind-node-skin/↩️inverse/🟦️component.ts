/** ↩️ unbind-node-skin exact-base field-local inverse. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfUnbindNodeSkin, type GltfUnbindNodeSkinPayload } from '../../unbind-node-skin/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfUnbindNodeSkinInverse { operation: GltfUnbindNodeSkinPayload; before: number | undefined; touchedPaths: readonly string[] }
export type GltfUnbindNodeSkinInverseResult={accepted:true;inverse:GltfUnbindNodeSkinInverse}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfUnbindNodeSkinInverse=(base:GltfSnapshot,operation:GltfUnbindNodeSkinPayload):GltfUnbindNodeSkinInverseResult=>{const rejection=validateGltfUnbindNodeSkin(operation,base);if(rejection)return{accepted:false,rejection};const before=base.document.nodes[operation.node]!.skin;return{accepted:true,inverse:{operation,before,touchedPaths:GltfUnbindNodeSkinInverseDescriptor.touchedPaths}};};
export const applyGltfUnbindNodeSkinInverse=(base:GltfSnapshot,inverse:GltfUnbindNodeSkinInverse):GltfSnapshot=>{const next=structuredClone(base);next.document.nodes[diff.operation.node]!.skin=inverse.before;return next;};
export const encodeGltfUnbindNodeSkinInverse=(inverse:GltfUnbindNodeSkinInverse):string=>JSON.stringify(inverse);
export const GltfUnbindNodeSkinInverseDescriptor={id:'s.stdio.gltf.mutation.unbind-node-skin.v1',version:1,touchedPaths:["document/nodes/*/skin"]}as const;
