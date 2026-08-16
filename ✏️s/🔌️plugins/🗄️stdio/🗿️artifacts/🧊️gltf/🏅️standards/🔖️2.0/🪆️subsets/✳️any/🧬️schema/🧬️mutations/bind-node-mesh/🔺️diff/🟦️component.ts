/** 🔺️ bind-node-mesh direct field-local sparse diff. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfBindNodeMesh, type GltfBindNodeMeshPayload } from '../../bind-node-mesh/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfBindNodeMeshDiff { operation: GltfBindNodeMeshPayload; after: number | undefined; touchedPaths: readonly string[] }
export type GltfBindNodeMeshDiffResult={accepted:true;diff:GltfBindNodeMeshDiff}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfBindNodeMeshDiff=(base:GltfSnapshot,operation:GltfBindNodeMeshPayload):GltfBindNodeMeshDiffResult=>{const rejection=validateGltfBindNodeMesh(operation,base);if(rejection)return{accepted:false,rejection};const after=operation.mesh;return{accepted:true,diff:{operation,after,touchedPaths:GltfBindNodeMeshDescriptor.touchedPaths}};};
export const applyGltfBindNodeMeshDiff=(base:GltfSnapshot,diff:GltfBindNodeMeshDiff):GltfSnapshot=>{const next=structuredClone(base);next.document.nodes[diff.operation.node]!.mesh=diff.after;return next;};
export const encodeGltfBindNodeMeshDiff=(diff:GltfBindNodeMeshDiff):string=>JSON.stringify(diff);
export const GltfBindNodeMeshDescriptor={id:'s.stdio.gltf.mutation.bind-node-mesh.v1',version:1,touchedPaths:["document/nodes/*/mesh"]}as const;
