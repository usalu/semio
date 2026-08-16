/** 🔺️ bind-node-camera direct field-local sparse diff. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfBindNodeCamera, type GltfBindNodeCameraPayload } from '../../bind-node-camera/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfBindNodeCameraDiff { operation: GltfBindNodeCameraPayload; after: number | undefined; touchedPaths: readonly string[] }
export type GltfBindNodeCameraDiffResult={accepted:true;diff:GltfBindNodeCameraDiff}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfBindNodeCameraDiff=(base:GltfSnapshot,operation:GltfBindNodeCameraPayload):GltfBindNodeCameraDiffResult=>{const rejection=validateGltfBindNodeCamera(operation,base);if(rejection)return{accepted:false,rejection};const after=operation.camera;return{accepted:true,diff:{operation,after,touchedPaths:GltfBindNodeCameraDescriptor.touchedPaths}};};
export const applyGltfBindNodeCameraDiff=(base:GltfSnapshot,diff:GltfBindNodeCameraDiff):GltfSnapshot=>{const next=structuredClone(base);next.document.nodes[diff.operation.node]!.camera=diff.after;return next;};
export const encodeGltfBindNodeCameraDiff=(diff:GltfBindNodeCameraDiff):string=>JSON.stringify(diff);
export const GltfBindNodeCameraDescriptor={id:'s.stdio.gltf.mutation.bind-node-camera.v1',version:1,touchedPaths:["document/nodes/*/camera"]}as const;
