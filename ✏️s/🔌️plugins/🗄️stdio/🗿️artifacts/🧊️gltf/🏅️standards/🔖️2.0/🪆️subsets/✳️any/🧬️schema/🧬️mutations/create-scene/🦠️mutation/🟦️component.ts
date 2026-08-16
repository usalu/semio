/** 🦠️ Creates one empty top-level glTF scene at an explicit position. */
import type { GltfScene, GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { clone, insert, position, type GltfMutationRejection, type GltfStructuralResult } from '../../🔒️top-level-collections-private/🟦️component.ts';
export const GltfCreateSceneDescriptor = { id: 's.stdio.gltf.mutation.create-scene.v1', version: 1, touchedPathPattern: 'document/scenes/{position}' } as const;
export interface GltfCreateScenePayload { position: number }
export const validateGltfCreateScene = (payload: GltfCreateScenePayload, base: GltfSnapshot): GltfMutationRejection | undefined => position(payload.position, base.document.scenes.length, 'document/scenes', true);
export const applyGltfCreateScene = (base: GltfSnapshot, payload: GltfCreateScenePayload): GltfStructuralResult => { const rejection = validateGltfCreateScene(payload, base); if (rejection) return { accepted: false, rejection }; const snapshot = clone(base); const scene: GltfScene = { nodes: [] }; insert(snapshot, 'scenes', payload.position, scene); return { accepted: true, snapshot: clone(snapshot) }; };
