/** ↩️ unbind-node-mesh exact-base field-local inverse. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfUnbindNodeMesh, type GltfUnbindNodeMeshPayload } from '../../unbind-node-mesh/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfUnbindNodeMeshInverse { operation: GltfUnbindNodeMeshPayload; before: number | undefined; touchedPaths: readonly string[] }
export type GltfUnbindNodeMeshInverseResult={accepted:true;inverse:GltfUnbindNodeMeshInverse}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfUnbindNodeMeshInverse=(base:GltfSnapshot,operation:GltfUnbindNodeMeshPayload):GltfUnbindNodeMeshInverseResult=>{const rejection=validateGltfUnbindNodeMesh(operation,base);if(rejection)return{accepted:false,rejection};const before=base.document.nodes[operation.node]!.mesh;return{accepted:true,inverse:{operation,before,touchedPaths:GltfUnbindNodeMeshInverseDescriptor.touchedPaths}};};
export const applyGltfUnbindNodeMeshInverse=(base:GltfSnapshot,inverse:GltfUnbindNodeMeshInverse):GltfSnapshot=>{const next=structuredClone(base);next.document.nodes[diff.operation.node]!.mesh=inverse.before;return next;};
export const encodeGltfUnbindNodeMeshInverse=(inverse:GltfUnbindNodeMeshInverse):string=>JSON.stringify(inverse);
export const GltfUnbindNodeMeshInverseDescriptor={id:'s.stdio.gltf.mutation.unbind-node-mesh.v1',version:1,touchedPaths:["document/nodes/*/mesh"]}as const;
