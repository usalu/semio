/** ↩️ change-node-morph-weights exact-base field-local inverse. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfChangeNodeMorphWeights, type GltfChangeNodeMorphWeightsPayload } from '../../change-node-morph-weights/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfChangeNodeMorphWeightsInverse { operation: GltfChangeNodeMorphWeightsPayload; before: number[]; touchedPaths: readonly string[] }
export type GltfChangeNodeMorphWeightsInverseResult={accepted:true;inverse:GltfChangeNodeMorphWeightsInverse}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfChangeNodeMorphWeightsInverse=(base:GltfSnapshot,operation:GltfChangeNodeMorphWeightsPayload):GltfChangeNodeMorphWeightsInverseResult=>{const rejection=validateGltfChangeNodeMorphWeights(operation,base);if(rejection)return{accepted:false,rejection};const before=[...base.document.nodes[operation.node]!.weights];return{accepted:true,inverse:{operation,before,touchedPaths:GltfChangeNodeMorphWeightsInverseDescriptor.touchedPaths}};};
export const applyGltfChangeNodeMorphWeightsInverse=(base:GltfSnapshot,inverse:GltfChangeNodeMorphWeightsInverse):GltfSnapshot=>{const next=structuredClone(base);next.document.nodes[diff.operation.node]!.weights=[...inverse.before];return next;};
export const encodeGltfChangeNodeMorphWeightsInverse=(inverse:GltfChangeNodeMorphWeightsInverse):string=>JSON.stringify(inverse);
export const GltfChangeNodeMorphWeightsInverseDescriptor={id:'s.stdio.gltf.mutation.change-node-morph-weights.v1',version:1,touchedPaths:["document/nodes/*/weights"]}as const;
