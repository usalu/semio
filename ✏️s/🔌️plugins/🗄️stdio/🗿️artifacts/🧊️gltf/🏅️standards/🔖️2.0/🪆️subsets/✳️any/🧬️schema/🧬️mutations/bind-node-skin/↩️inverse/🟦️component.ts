/** ↩️ bind-node-skin exact-base field-local inverse. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfBindNodeSkin, type GltfBindNodeSkinPayload } from '../../bind-node-skin/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfBindNodeSkinInverse { operation: GltfBindNodeSkinPayload; before: number | undefined; touchedPaths: readonly string[] }
export type GltfBindNodeSkinInverseResult={accepted:true;inverse:GltfBindNodeSkinInverse}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfBindNodeSkinInverse=(base:GltfSnapshot,operation:GltfBindNodeSkinPayload):GltfBindNodeSkinInverseResult=>{const rejection=validateGltfBindNodeSkin(operation,base);if(rejection)return{accepted:false,rejection};const before=base.document.nodes[operation.node]!.skin;return{accepted:true,inverse:{operation,before,touchedPaths:GltfBindNodeSkinInverseDescriptor.touchedPaths}};};
export const applyGltfBindNodeSkinInverse=(base:GltfSnapshot,inverse:GltfBindNodeSkinInverse):GltfSnapshot=>{const next=structuredClone(base);next.document.nodes[diff.operation.node]!.skin=inverse.before;return next;};
export const encodeGltfBindNodeSkinInverse=(inverse:GltfBindNodeSkinInverse):string=>JSON.stringify(inverse);
export const GltfBindNodeSkinInverseDescriptor={id:'s.stdio.gltf.mutation.bind-node-skin.v1',version:1,touchedPaths:["document/nodes/*/skin"]}as const;
