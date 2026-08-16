/** ↩️ bind-node-mesh exact-base field-local inverse. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfBindNodeMesh, type GltfBindNodeMeshPayload } from '../../bind-node-mesh/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfBindNodeMeshInverse { operation: GltfBindNodeMeshPayload; before: number | undefined; touchedPaths: readonly string[] }
export type GltfBindNodeMeshInverseResult={accepted:true;inverse:GltfBindNodeMeshInverse}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfBindNodeMeshInverse=(base:GltfSnapshot,operation:GltfBindNodeMeshPayload):GltfBindNodeMeshInverseResult=>{const rejection=validateGltfBindNodeMesh(operation,base);if(rejection)return{accepted:false,rejection};const before=base.document.nodes[operation.node]!.mesh;return{accepted:true,inverse:{operation,before,touchedPaths:GltfBindNodeMeshInverseDescriptor.touchedPaths}};};
export const applyGltfBindNodeMeshInverse=(base:GltfSnapshot,inverse:GltfBindNodeMeshInverse):GltfSnapshot=>{const next=structuredClone(base);next.document.nodes[diff.operation.node]!.mesh=inverse.before;return next;};
export const encodeGltfBindNodeMeshInverse=(inverse:GltfBindNodeMeshInverse):string=>JSON.stringify(inverse);
export const GltfBindNodeMeshInverseDescriptor={id:'s.stdio.gltf.mutation.bind-node-mesh.v1',version:1,touchedPaths:["document/nodes/*/mesh"]}as const;
