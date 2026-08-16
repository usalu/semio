/** 🔺️ unbind-node-skin direct field-local sparse diff. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfUnbindNodeSkin, type GltfUnbindNodeSkinPayload } from '../../unbind-node-skin/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfUnbindNodeSkinDiff { operation: GltfUnbindNodeSkinPayload; after: number | undefined; touchedPaths: readonly string[] }
export type GltfUnbindNodeSkinDiffResult={accepted:true;diff:GltfUnbindNodeSkinDiff}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfUnbindNodeSkinDiff=(base:GltfSnapshot,operation:GltfUnbindNodeSkinPayload):GltfUnbindNodeSkinDiffResult=>{const rejection=validateGltfUnbindNodeSkin(operation,base);if(rejection)return{accepted:false,rejection};const after=undefined;return{accepted:true,diff:{operation,after,touchedPaths:GltfUnbindNodeSkinDescriptor.touchedPaths}};};
export const applyGltfUnbindNodeSkinDiff=(base:GltfSnapshot,diff:GltfUnbindNodeSkinDiff):GltfSnapshot=>{const next=structuredClone(base);next.document.nodes[diff.operation.node]!.skin=diff.after;return next;};
export const encodeGltfUnbindNodeSkinDiff=(diff:GltfUnbindNodeSkinDiff):string=>JSON.stringify(diff);
export const GltfUnbindNodeSkinDescriptor={id:'s.stdio.gltf.mutation.unbind-node-skin.v1',version:1,touchedPaths:["document/nodes/*/skin"]}as const;
