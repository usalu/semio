/** 🔺️ change-scene-name direct field-local sparse diff. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfChangeSceneName, type GltfChangeSceneNamePayload } from '../../change-scene-name/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfChangeSceneNameDiff { operation: GltfChangeSceneNamePayload; after: string | undefined; touchedPaths: readonly string[] }
export type GltfChangeSceneNameDiffResult={accepted:true;diff:GltfChangeSceneNameDiff}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfChangeSceneNameDiff=(base:GltfSnapshot,operation:GltfChangeSceneNamePayload):GltfChangeSceneNameDiffResult=>{const rejection=validateGltfChangeSceneName(operation,base);if(rejection)return{accepted:false,rejection};const after=operation.value??undefined;return{accepted:true,diff:{operation,after,touchedPaths:GltfChangeSceneNameDescriptor.touchedPaths}};};
export const applyGltfChangeSceneNameDiff=(base:GltfSnapshot,diff:GltfChangeSceneNameDiff):GltfSnapshot=>{const next=structuredClone(base);next.document.scenes[diff.operation.scene]!.name=diff.after;return next;};
export const encodeGltfChangeSceneNameDiff=(diff:GltfChangeSceneNameDiff):string=>JSON.stringify(diff);
export const GltfChangeSceneNameDescriptor={id:'s.stdio.gltf.mutation.change-scene-name.v1',version:1,touchedPaths:["document/scenes/*/name"]}as const;
