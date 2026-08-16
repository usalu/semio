/** 🦠️ Deletes one top-level glTF scene with typed default-scene reference repair. */
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { clone, position, reject, remove, type GltfMutationRejection, type GltfStructuralResult } from '../../🔒️top-level-collections-private/🟦️component.ts';
export const GltfDeleteSceneDescriptor = { id: 's.stdio.gltf.mutation.delete-scene.v1', version: 1, touchedPathPatterns: ['document/scenes/{index}', 'document/scene'] } as const;
export interface GltfDeleteScenePayload { index: number }
export const validateGltfDeleteScene = (payload: GltfDeleteScenePayload, base: GltfSnapshot): GltfMutationRejection | undefined => { const range = position(payload.index, base.document.scenes.length, 'document/scenes'); if (range) return range; return base.document.scene !== undefined && base.document.scene >= base.document.scenes.length ? reject('gltf.reference.invalid-default-scene', 'document/scene', 'default scene must address an existing scene') : undefined; };
export const applyGltfDeleteScene = (base: GltfSnapshot, payload: GltfDeleteScenePayload): GltfStructuralResult => { const rejection = validateGltfDeleteScene(payload, base); if (rejection) return { accepted: false, rejection }; const snapshot = clone(base); remove(snapshot, 'scenes', payload.index); return { accepted: true, snapshot: clone(snapshot) }; };
