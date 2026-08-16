/** 🔺️ unbind-node-mesh direct field-local sparse diff. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfUnbindNodeMesh, type GltfUnbindNodeMeshPayload } from '../../unbind-node-mesh/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfUnbindNodeMeshDiff { operation: GltfUnbindNodeMeshPayload; after: number | undefined; touchedPaths: readonly string[] }
export type GltfUnbindNodeMeshDiffResult={accepted:true;diff:GltfUnbindNodeMeshDiff}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfUnbindNodeMeshDiff=(base:GltfSnapshot,operation:GltfUnbindNodeMeshPayload):GltfUnbindNodeMeshDiffResult=>{const rejection=validateGltfUnbindNodeMesh(operation,base);if(rejection)return{accepted:false,rejection};const after=undefined;return{accepted:true,diff:{operation,after,touchedPaths:GltfUnbindNodeMeshDescriptor.touchedPaths}};};
export const applyGltfUnbindNodeMeshDiff=(base:GltfSnapshot,diff:GltfUnbindNodeMeshDiff):GltfSnapshot=>{const next=structuredClone(base);next.document.nodes[diff.operation.node]!.mesh=diff.after;return next;};
export const encodeGltfUnbindNodeMeshDiff=(diff:GltfUnbindNodeMeshDiff):string=>JSON.stringify(diff);
export const GltfUnbindNodeMeshDescriptor={id:'s.stdio.gltf.mutation.unbind-node-mesh.v1',version:1,touchedPaths:["document/nodes/*/mesh"]}as const;
