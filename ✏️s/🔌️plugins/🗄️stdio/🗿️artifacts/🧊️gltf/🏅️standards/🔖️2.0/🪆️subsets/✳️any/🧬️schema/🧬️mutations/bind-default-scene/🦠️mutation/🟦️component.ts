/** 🦠️ bind-default-scene executable glTF command. */
import type { GltfJson, GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { clone, reject, run, same, type GltfLeafResult, type GltfMutationRejection } from '../../🔒️top-level-private/🟦️component.ts';
export const GltfBindDefaultSceneDescriptor = { id: 's.stdio.gltf.mutation.bind-default-scene.v1', version: 1, touchedPaths: ["document/scene"], referencePolicy: 'validates one explicit scene reference' } as const;
export interface GltfBindDefaultScenePayload { scene: number }
export const validateGltfBindDefaultScene = (payload: GltfBindDefaultScenePayload, base: GltfSnapshot): GltfMutationRejection | undefined => { if (!Number.isInteger(payload.scene) || payload.scene < 0 || payload.scene >= base.document.scenes.length) return reject('gltf.mutation.index-out-of-range', 'document/scenes', 'scene must exist'); if (payload.scene === base.document.scene) return reject('gltf.mutation.no-observable-change', 'document/scene', 'scene is already default'); return undefined; };
export const applyGltfBindDefaultScene = (base: GltfSnapshot, payload: GltfBindDefaultScenePayload): GltfLeafResult => run(base, payload, validateGltfBindDefaultScene, (next, payload) => { next.document.scene = payload.scene; });
