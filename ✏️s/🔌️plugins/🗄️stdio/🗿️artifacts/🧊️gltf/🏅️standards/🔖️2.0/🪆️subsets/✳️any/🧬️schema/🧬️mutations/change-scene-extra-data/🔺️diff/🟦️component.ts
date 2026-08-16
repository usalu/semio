/** 🔺️ change-scene-extra-data direct field-local sparse diff. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfChangeSceneExtraData, type GltfChangeSceneExtraDataPayload } from '../../change-scene-extra-data/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfChangeSceneExtraDataDiff { operation: GltfChangeSceneExtraDataPayload; after: GltfJson | undefined; touchedPaths: readonly string[] }
export type GltfChangeSceneExtraDataDiffResult={accepted:true;diff:GltfChangeSceneExtraDataDiff}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfChangeSceneExtraDataDiff=(base:GltfSnapshot,operation:GltfChangeSceneExtraDataPayload):GltfChangeSceneExtraDataDiffResult=>{const rejection=validateGltfChangeSceneExtraData(operation,base);if(rejection)return{accepted:false,rejection};const after=operation.data.state==='present'?operation.data.value:undefined;return{accepted:true,diff:{operation,after,touchedPaths:GltfChangeSceneExtraDataDescriptor.touchedPaths}};};
export const applyGltfChangeSceneExtraDataDiff=(base:GltfSnapshot,diff:GltfChangeSceneExtraDataDiff):GltfSnapshot=>{const next=structuredClone(base);next.document.scenes[diff.operation.scene]!.extras=diff.after;return next;};
export const encodeGltfChangeSceneExtraDataDiff=(diff:GltfChangeSceneExtraDataDiff):string=>JSON.stringify(diff);
export const GltfChangeSceneExtraDataDescriptor={id:'s.stdio.gltf.mutation.change-scene-extra-data.v1',version:1,touchedPaths:["document/scenes/*/extras"]}as const;
