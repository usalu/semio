/** 🔺️ bind-node-skin direct field-local sparse diff. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfBindNodeSkin, type GltfBindNodeSkinPayload } from '../../bind-node-skin/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfBindNodeSkinDiff { operation: GltfBindNodeSkinPayload; after: number | undefined; touchedPaths: readonly string[] }
export type GltfBindNodeSkinDiffResult={accepted:true;diff:GltfBindNodeSkinDiff}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfBindNodeSkinDiff=(base:GltfSnapshot,operation:GltfBindNodeSkinPayload):GltfBindNodeSkinDiffResult=>{const rejection=validateGltfBindNodeSkin(operation,base);if(rejection)return{accepted:false,rejection};const after=operation.skin;return{accepted:true,diff:{operation,after,touchedPaths:GltfBindNodeSkinDescriptor.touchedPaths}};};
export const applyGltfBindNodeSkinDiff=(base:GltfSnapshot,diff:GltfBindNodeSkinDiff):GltfSnapshot=>{const next=structuredClone(base);next.document.nodes[diff.operation.node]!.skin=diff.after;return next;};
export const encodeGltfBindNodeSkinDiff=(diff:GltfBindNodeSkinDiff):string=>JSON.stringify(diff);
export const GltfBindNodeSkinDescriptor={id:'s.stdio.gltf.mutation.bind-node-skin.v1',version:1,touchedPaths:["document/nodes/*/skin"]}as const;
