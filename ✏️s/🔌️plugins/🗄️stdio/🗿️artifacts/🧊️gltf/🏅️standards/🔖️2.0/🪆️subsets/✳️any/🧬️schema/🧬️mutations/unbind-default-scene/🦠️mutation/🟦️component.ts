/** 🦠️ unbind-default-scene executable glTF command. */
import type { GltfJson, GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { clone, reject, run, same, type GltfLeafResult, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const GltfUnbindDefaultSceneDescriptor = { id: 's.stdio.gltf.mutation.unbind-default-scene.v1', version: 1, touchedPaths: ["document/scene"], referencePolicy: 'none' } as const;
export interface GltfUnbindDefaultScenePayload {  }
export const validateGltfUnbindDefaultScene = (payload: GltfUnbindDefaultScenePayload, base: GltfSnapshot): GltfMutationRejection | undefined => { if (base.document.scene === undefined) return reject('gltf.mutation.relation-absent', 'document/scene', 'no default scene is bound'); return undefined; };
export const applyGltfUnbindDefaultScene = (base: GltfSnapshot, payload: GltfUnbindDefaultScenePayload): GltfLeafResult => run(base, payload, validateGltfUnbindDefaultScene, (next, payload) => { next.document.scene = undefined; });
