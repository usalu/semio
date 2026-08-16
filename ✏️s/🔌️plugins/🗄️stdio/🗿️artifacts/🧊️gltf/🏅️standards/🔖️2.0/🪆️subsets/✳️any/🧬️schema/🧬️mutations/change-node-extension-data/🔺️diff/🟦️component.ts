/** 🔺️ change-node-extension-data direct field-local sparse diff. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfChangeNodeExtensionData, type GltfChangeNodeExtensionDataPayload } from '../../change-node-extension-data/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfChangeNodeExtensionDataDiff { operation: GltfChangeNodeExtensionDataPayload; after: GltfJson | undefined; touchedPaths: readonly string[] }
export type GltfChangeNodeExtensionDataDiffResult={accepted:true;diff:GltfChangeNodeExtensionDataDiff}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfChangeNodeExtensionDataDiff=(base:GltfSnapshot,operation:GltfChangeNodeExtensionDataPayload):GltfChangeNodeExtensionDataDiffResult=>{const rejection=validateGltfChangeNodeExtensionData(operation,base);if(rejection)return{accepted:false,rejection};const after=operation.data.state==='present'?operation.data.value:undefined;return{accepted:true,diff:{operation,after,touchedPaths:GltfChangeNodeExtensionDataDescriptor.touchedPaths}};};
export const applyGltfChangeNodeExtensionDataDiff=(base:GltfSnapshot,diff:GltfChangeNodeExtensionDataDiff):GltfSnapshot=>{const next=structuredClone(base);next.document.nodes[diff.operation.node]!.extensions=diff.after;return next;};
export const encodeGltfChangeNodeExtensionDataDiff=(diff:GltfChangeNodeExtensionDataDiff):string=>JSON.stringify(diff);
export const GltfChangeNodeExtensionDataDescriptor={id:'s.stdio.gltf.mutation.change-node-extension-data.v1',version:1,touchedPaths:["document/nodes/*/extensions"]}as const;
