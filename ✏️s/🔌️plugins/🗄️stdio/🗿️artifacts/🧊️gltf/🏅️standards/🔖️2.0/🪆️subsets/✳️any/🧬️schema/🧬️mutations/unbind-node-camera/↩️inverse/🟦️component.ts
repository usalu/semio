/** ↩️ unbind-node-camera exact-base field-local inverse. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfUnbindNodeCamera, type GltfUnbindNodeCameraPayload } from '../../unbind-node-camera/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfUnbindNodeCameraInverse { operation: GltfUnbindNodeCameraPayload; before: number | undefined; touchedPaths: readonly string[] }
export type GltfUnbindNodeCameraInverseResult={accepted:true;inverse:GltfUnbindNodeCameraInverse}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfUnbindNodeCameraInverse=(base:GltfSnapshot,operation:GltfUnbindNodeCameraPayload):GltfUnbindNodeCameraInverseResult=>{const rejection=validateGltfUnbindNodeCamera(operation,base);if(rejection)return{accepted:false,rejection};const before=base.document.nodes[operation.node]!.camera;return{accepted:true,inverse:{operation,before,touchedPaths:GltfUnbindNodeCameraInverseDescriptor.touchedPaths}};};
export const applyGltfUnbindNodeCameraInverse=(base:GltfSnapshot,inverse:GltfUnbindNodeCameraInverse):GltfSnapshot=>{const next=structuredClone(base);next.document.nodes[diff.operation.node]!.camera=inverse.before;return next;};
export const encodeGltfUnbindNodeCameraInverse=(inverse:GltfUnbindNodeCameraInverse):string=>JSON.stringify(inverse);
export const GltfUnbindNodeCameraInverseDescriptor={id:'s.stdio.gltf.mutation.unbind-node-camera.v1',version:1,touchedPaths:["document/nodes/*/camera"]}as const;
