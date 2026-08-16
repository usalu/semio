/** ↩️ bind-node-camera exact-base field-local inverse. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { validateGltfBindNodeCamera, type GltfBindNodeCameraPayload } from '../../bind-node-camera/🦠️mutation/🟦️component.ts';
import type { GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export interface GltfBindNodeCameraInverse { operation: GltfBindNodeCameraPayload; before: number | undefined; touchedPaths: readonly string[] }
export type GltfBindNodeCameraInverseResult={accepted:true;inverse:GltfBindNodeCameraInverse}|{accepted:false;rejection:GltfMutationRejection};
export const deriveGltfBindNodeCameraInverse=(base:GltfSnapshot,operation:GltfBindNodeCameraPayload):GltfBindNodeCameraInverseResult=>{const rejection=validateGltfBindNodeCamera(operation,base);if(rejection)return{accepted:false,rejection};const before=base.document.nodes[operation.node]!.camera;return{accepted:true,inverse:{operation,before,touchedPaths:GltfBindNodeCameraInverseDescriptor.touchedPaths}};};
export const applyGltfBindNodeCameraInverse=(base:GltfSnapshot,inverse:GltfBindNodeCameraInverse):GltfSnapshot=>{const next=structuredClone(base);next.document.nodes[diff.operation.node]!.camera=inverse.before;return next;};
export const encodeGltfBindNodeCameraInverse=(inverse:GltfBindNodeCameraInverse):string=>JSON.stringify(inverse);
export const GltfBindNodeCameraInverseDescriptor={id:'s.stdio.gltf.mutation.bind-node-camera.v1',version:1,touchedPaths:["document/nodes/*/camera"]}as const;
