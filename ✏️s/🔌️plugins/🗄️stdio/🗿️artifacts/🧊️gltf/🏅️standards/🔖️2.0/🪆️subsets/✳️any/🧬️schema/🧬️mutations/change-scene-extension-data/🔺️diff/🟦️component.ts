/** 🔺️ change-scene-extension-data direct field-local sparse diff. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfChangeSceneExtensionData, type GltfChangeSceneExtensionDataPayload } from '../../change-scene-extension-data/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfChangeSceneExtensionDataDiff { operation: GltfChangeSceneExtensionDataPayload; after: GltfJson | undefined; touchedPaths: readonly string[] }
export type GltfChangeSceneExtensionDataDiffResult={accepted:true;diff:GltfChangeSceneExtensionDataDiff}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfChangeSceneExtensionDataDiff=(base:GltfSnapshot,operation:GltfChangeSceneExtensionDataPayload):GltfChangeSceneExtensionDataDiffResult=>{const rejection=validateGltfChangeSceneExtensionData(operation,base);if(rejection)return{accepted:false,rejection};const after=operation.data.state==='present'?operation.data.value:undefined;return{accepted:true,diff:{operation,after,touchedPaths:GltfChangeSceneExtensionDataDescriptor.touchedPaths}};};
export const applyGltfChangeSceneExtensionDataDiff=(base:GltfSnapshot,diff:GltfChangeSceneExtensionDataDiff):GltfSnapshot=>{const next=structuredClone(base);next.document.scenes[diff.operation.scene]!.extensions=diff.after;return next;};
export const encodeGltfChangeSceneExtensionDataDiff=(diff:GltfChangeSceneExtensionDataDiff):string=>JSON.stringify(diff);
export const GltfChangeSceneExtensionDataDescriptor={id:'s.stdio.gltf.mutation.change-scene-extension-data.v1',version:1,touchedPaths:["document/scenes/*/extensions"]}as const;
