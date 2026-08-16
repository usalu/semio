/** 🔺️ change-node-morph-weights direct field-local sparse diff. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfChangeNodeMorphWeights, type GltfChangeNodeMorphWeightsPayload } from '../../change-node-morph-weights/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfChangeNodeMorphWeightsDiff { operation: GltfChangeNodeMorphWeightsPayload; after: number[]; touchedPaths: readonly string[] }
export type GltfChangeNodeMorphWeightsDiffResult={accepted:true;diff:GltfChangeNodeMorphWeightsDiff}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfChangeNodeMorphWeightsDiff=(base:GltfSnapshot,operation:GltfChangeNodeMorphWeightsPayload):GltfChangeNodeMorphWeightsDiffResult=>{const rejection=validateGltfChangeNodeMorphWeights(operation,base);if(rejection)return{accepted:false,rejection};const after=[...operation.weights];return{accepted:true,diff:{operation,after,touchedPaths:GltfChangeNodeMorphWeightsDescriptor.touchedPaths}};};
export const applyGltfChangeNodeMorphWeightsDiff=(base:GltfSnapshot,diff:GltfChangeNodeMorphWeightsDiff):GltfSnapshot=>{const next=structuredClone(base);next.document.nodes[diff.operation.node]!.weights=[...diff.after];return next;};
export const encodeGltfChangeNodeMorphWeightsDiff=(diff:GltfChangeNodeMorphWeightsDiff):string=>JSON.stringify(diff);
export const GltfChangeNodeMorphWeightsDescriptor={id:'s.stdio.gltf.mutation.change-node-morph-weights.v1',version:1,touchedPaths:["document/nodes/*/weights"]}as const;
