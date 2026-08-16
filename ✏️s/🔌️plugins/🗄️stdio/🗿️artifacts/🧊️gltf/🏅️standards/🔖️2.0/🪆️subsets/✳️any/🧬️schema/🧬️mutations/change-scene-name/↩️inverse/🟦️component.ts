/** ↩️ change-scene-name exact-base field-local inverse. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfChangeSceneName, type GltfChangeSceneNamePayload } from '../../change-scene-name/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfChangeSceneNameInverse { operation: GltfChangeSceneNamePayload; before: string | undefined; touchedPaths: readonly string[] }
export type GltfChangeSceneNameInverseResult={accepted:true;inverse:GltfChangeSceneNameInverse}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfChangeSceneNameInverse=(base:GltfSnapshot,operation:GltfChangeSceneNamePayload):GltfChangeSceneNameInverseResult=>{const rejection=validateGltfChangeSceneName(operation,base);if(rejection)return{accepted:false,rejection};const before=base.document.scenes[operation.scene]!.name;return{accepted:true,inverse:{operation,before,touchedPaths:GltfChangeSceneNameInverseDescriptor.touchedPaths}};};
export const applyGltfChangeSceneNameInverse=(base:GltfSnapshot,inverse:GltfChangeSceneNameInverse):GltfSnapshot=>{const next=structuredClone(base);next.document.scenes[diff.operation.scene]!.name=inverse.before;return next;};
export const encodeGltfChangeSceneNameInverse=(inverse:GltfChangeSceneNameInverse):string=>JSON.stringify(inverse);
export const GltfChangeSceneNameInverseDescriptor={id:'s.stdio.gltf.mutation.change-scene-name.v1',version:1,touchedPaths:["document/scenes/*/name"]}as const;
