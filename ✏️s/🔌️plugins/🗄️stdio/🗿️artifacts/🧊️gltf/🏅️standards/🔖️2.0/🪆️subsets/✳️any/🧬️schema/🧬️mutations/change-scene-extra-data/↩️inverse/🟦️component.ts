/** ↩️ change-scene-extra-data exact-base field-local inverse. */
import type { GltfJson, GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfChangeSceneExtraData, type GltfChangeSceneExtraDataPayload } from '../../change-scene-extra-data/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfChangeSceneExtraDataInverse { operation: GltfChangeSceneExtraDataPayload; before: GltfJson | undefined; touchedPaths: readonly string[] }
export type GltfChangeSceneExtraDataInverseResult={accepted:true;inverse:GltfChangeSceneExtraDataInverse}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfChangeSceneExtraDataInverse=(base:GltfSnapshot,operation:GltfChangeSceneExtraDataPayload):GltfChangeSceneExtraDataInverseResult=>{const rejection=validateGltfChangeSceneExtraData(operation,base);if(rejection)return{accepted:false,rejection};const before=base.document.scenes[operation.scene]!.extras;return{accepted:true,inverse:{operation,before,touchedPaths:[`document/scenes/${operation.scene}/extras`]}};};
export const applyGltfChangeSceneExtraDataInverse=(base:GltfSnapshot,inverse:GltfChangeSceneExtraDataInverse):GltfChangeSceneExtraDataInverseResult=>{if(!Number.isInteger(inverse.operation.scene)||inverse.operation.scene<0||inverse.operation.scene>=base.document.scenes.length)return{accepted:false,rejection:{code:'gltf.mutation.index-out-of-range',path:'document/scenes',detail:'scene is absent'}};const snapshot=structuredClone(base);snapshot.document.scenes[inverse.operation.scene]!.extras=inverse.before;return{accepted:true,inverse};};
export const encodeGltfChangeSceneExtraDataInverse=(inverse:GltfChangeSceneExtraDataInverse):string=>JSON.stringify(inverse);
export const GltfChangeSceneExtraDataInverseDescriptor={id:'s.stdio.gltf.mutation.change-scene-extra-data.v1',version:1,touchedPaths:["document/scenes/*/extras"]}as const;
