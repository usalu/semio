/** 🔺️ unbind-node-camera direct field-local sparse diff. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfUnbindNodeCamera, type GltfUnbindNodeCameraPayload } from '../../unbind-node-camera/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfUnbindNodeCameraDiff { operation: GltfUnbindNodeCameraPayload; after: number | undefined; touchedPaths: readonly string[] }
export type GltfUnbindNodeCameraDiffResult={accepted:true;diff:GltfUnbindNodeCameraDiff}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfUnbindNodeCameraDiff=(base:GltfSnapshot,operation:GltfUnbindNodeCameraPayload):GltfUnbindNodeCameraDiffResult=>{const rejection=validateGltfUnbindNodeCamera(operation,base);if(rejection)return{accepted:false,rejection};const after=undefined;return{accepted:true,diff:{operation,after,touchedPaths:GltfUnbindNodeCameraDescriptor.touchedPaths}};};
export const applyGltfUnbindNodeCameraDiff=(base:GltfSnapshot,diff:GltfUnbindNodeCameraDiff):GltfSnapshot=>{const next=structuredClone(base);next.document.nodes[diff.operation.node]!.camera=diff.after;return next;};
export const encodeGltfUnbindNodeCameraDiff=(diff:GltfUnbindNodeCameraDiff):string=>JSON.stringify(diff);
export const GltfUnbindNodeCameraDescriptor={id:'s.stdio.gltf.mutation.unbind-node-camera.v1',version:1,touchedPaths:["document/nodes/*/camera"]}as const;
