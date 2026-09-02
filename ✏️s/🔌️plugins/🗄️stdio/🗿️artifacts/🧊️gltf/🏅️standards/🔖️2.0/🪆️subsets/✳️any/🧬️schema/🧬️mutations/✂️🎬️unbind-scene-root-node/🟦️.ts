/** 🦠️ unbind-scene-root-node is an atomic, typed glTF 2.0 command. */
import type { GltfSnapshot } from '../../📸️snapshot/🟦️.ts';
import { reject, run, type GltfLeafResult, type GltfMutationRejection } from './🟦️';
import { itemIndex } from './🟦️';
export const GltfUnbindSceneRootNodeDescriptor = { id: 's.stdio.gltf.mutation.unbind-scene-root-node.v1', version: 1, kind: 'unbind', touchedPaths: ["document/scenes/*/nodes"], referencePolicy: 'removes only the selected scene-root relationship' } as const;
export interface GltfUnbindSceneRootNodePayload { scene: number; node: number }
export type GltfUnbindSceneRootNodeResult = GltfLeafResult;
export const validateGltfUnbindSceneRootNode = (payload: GltfUnbindSceneRootNodePayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const scene = itemIndex(payload.scene, base.document.scenes.length, 'document/scenes'); if (scene) return scene; const node = itemIndex(payload.node, base.document.nodes.length, 'document/nodes'); if (node) return node; if (!base.document.scenes[payload.scene]!.nodes.includes(payload.node)) return reject('gltf.mutation.relation-absent', `document/scenes/${payload.scene}/nodes`, 'node is not a root of this scene'); return undefined; };
export const applyGltfUnbindSceneRootNode = (base: GltfSnapshot, payload: GltfUnbindSceneRootNodePayload): GltfUnbindSceneRootNodeResult => run(base, payload, validateGltfUnbindSceneRootNode, (next, payload) => { const position = next.document.scenes[payload.scene]!.nodes.indexOf(payload.node); next.document.scenes[payload.scene]!.nodes.splice(position, 1); });
