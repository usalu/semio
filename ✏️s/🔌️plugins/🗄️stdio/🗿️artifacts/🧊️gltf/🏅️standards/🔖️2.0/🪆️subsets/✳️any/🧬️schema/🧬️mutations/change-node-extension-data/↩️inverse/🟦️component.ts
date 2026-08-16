/** ↩️ change-node-extension-data exact-base field-local inverse. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfChangeNodeExtensionData, type GltfChangeNodeExtensionDataPayload } from '../../change-node-extension-data/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfChangeNodeExtensionDataInverse { operation: GltfChangeNodeExtensionDataPayload; before: GltfJson | undefined; touchedPaths: readonly string[] }
export type GltfChangeNodeExtensionDataInverseResult={accepted:true;inverse:GltfChangeNodeExtensionDataInverse}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfChangeNodeExtensionDataInverse=(base:GltfSnapshot,operation:GltfChangeNodeExtensionDataPayload):GltfChangeNodeExtensionDataInverseResult=>{const rejection=validateGltfChangeNodeExtensionData(operation,base);if(rejection)return{accepted:false,rejection};const before=base.document.nodes[operation.node]!.extensions;return{accepted:true,inverse:{operation,before,touchedPaths:GltfChangeNodeExtensionDataInverseDescriptor.touchedPaths}};};
export const applyGltfChangeNodeExtensionDataInverse=(base:GltfSnapshot,inverse:GltfChangeNodeExtensionDataInverse):GltfSnapshot=>{const next=structuredClone(base);next.document.nodes[diff.operation.node]!.extensions=inverse.before;return next;};
export const encodeGltfChangeNodeExtensionDataInverse=(inverse:GltfChangeNodeExtensionDataInverse):string=>JSON.stringify(inverse);
export const GltfChangeNodeExtensionDataInverseDescriptor={id:'s.stdio.gltf.mutation.change-node-extension-data.v1',version:1,touchedPaths:["document/nodes/*/extensions"]}as const;
