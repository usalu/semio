/** ↩️ change-scene-extension-data exact-base field-local inverse. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfChangeSceneExtensionData, type GltfChangeSceneExtensionDataPayload } from '../../change-scene-extension-data/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfChangeSceneExtensionDataInverse { operation: GltfChangeSceneExtensionDataPayload; before: GltfJson | undefined; touchedPaths: readonly string[] }
export type GltfChangeSceneExtensionDataInverseResult={accepted:true;inverse:GltfChangeSceneExtensionDataInverse}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfChangeSceneExtensionDataInverse=(base:GltfSnapshot,operation:GltfChangeSceneExtensionDataPayload):GltfChangeSceneExtensionDataInverseResult=>{const rejection=validateGltfChangeSceneExtensionData(operation,base);if(rejection)return{accepted:false,rejection};const before=base.document.scenes[operation.scene]!.extensions;return{accepted:true,inverse:{operation,before,touchedPaths:GltfChangeSceneExtensionDataInverseDescriptor.touchedPaths}};};
export const applyGltfChangeSceneExtensionDataInverse=(base:GltfSnapshot,inverse:GltfChangeSceneExtensionDataInverse):GltfSnapshot=>{const next=structuredClone(base);next.document.scenes[diff.operation.scene]!.extensions=inverse.before;return next;};
export const encodeGltfChangeSceneExtensionDataInverse=(inverse:GltfChangeSceneExtensionDataInverse):string=>JSON.stringify(inverse);
export const GltfChangeSceneExtensionDataInverseDescriptor={id:'s.stdio.gltf.mutation.change-scene-extension-data.v1',version:1,touchedPaths:["document/scenes/*/extensions"]}as const;
