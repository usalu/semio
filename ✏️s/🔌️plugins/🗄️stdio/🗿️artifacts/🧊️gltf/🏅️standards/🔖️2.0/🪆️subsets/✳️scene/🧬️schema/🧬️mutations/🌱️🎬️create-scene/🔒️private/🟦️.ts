/** 🔒 Command-local create-scene validation and scene-reference mechanics. */
import type { GltfScene, GltfSnapshot } from '../../../📸️snapshot/🟦️.ts';

//#region 🔖️Rejection
export interface GltfCreateSceneRejection { code: string; path: string; detail: string }
export type GltfCreateSceneResult<T> = { accepted: true; value: T } | { accepted: false; rejection: GltfCreateSceneRejection };
export const reject = (code: string, path: string, detail: string): GltfCreateSceneRejection => ({ code, path, detail });
//#endregion 🔖️Rejection

//#region 🔢️U32Domain
export const U32_MAX = 0xffff_ffff;
export const isU32 = (value: number): boolean => Number.isInteger(value) && value >= 0 && value <= U32_MAX;
export const validateSceneSequence = (scenes: readonly GltfScene[]): GltfCreateSceneRejection | undefined => {
  if (scenes.length > U32_MAX) return reject('gltf.mutation.collection-overflow', 'document/scenes', 'scene collection exceeds the u32 command domain');
  for (const [sceneIndex, scene] of scenes.entries()) if (scene.nodes.some(node => !isU32(node))) return reject('gltf.mutation.index-out-of-range', `document/scenes/${sceneIndex}/nodes`, 'node index exceeds the u32 command domain');
};
//#endregion 🔢️U32Domain

//#region 🎬️SceneState
export const defaultScene = (snapshot: GltfSnapshot): GltfCreateSceneRejection | undefined => {
  const sequence = validateSceneSequence(snapshot.document.scenes);
  if (sequence) return sequence;
  return snapshot.document.scene === undefined || isU32(snapshot.document.scene) && snapshot.document.scene < snapshot.document.scenes.length ? undefined : reject('gltf.mutation.reference-out-of-range', 'document/scene', 'default scene must name an existing scene');
};

export const insertionPosition = (position: number, snapshot: GltfSnapshot): GltfCreateSceneRejection | undefined => defaultScene(snapshot) ?? (snapshot.document.scenes.length >= U32_MAX ? reject('gltf.mutation.collection-overflow', 'document/scenes', 'creating a scene would exceed the u32 command domain') : isU32(position) && position <= snapshot.document.scenes.length ? undefined : reject('gltf.mutation.insert-out-of-range', 'document/scenes', 'position must be within the collection'));
export const existingPosition = (position: number, snapshot: GltfSnapshot): GltfCreateSceneRejection | undefined => defaultScene(snapshot) ?? (isU32(position) && position < snapshot.document.scenes.length ? undefined : reject('gltf.mutation.index-out-of-range', 'document/scenes', 'position must address an existing scene'));
export const defaultAfter = (scene: number | null, position: number): number | null => scene === null ? null : scene >= position ? scene + 1 : scene;
export const equal = (left: unknown, right: unknown): boolean => JSON.stringify(left) === JSON.stringify(right);
export const clone = (snapshot: GltfSnapshot): GltfSnapshot => structuredClone(snapshot);

export const insertEmptyScene = (snapshot: GltfSnapshot, position: number): void => {
  if (snapshot.document.scene !== undefined && snapshot.document.scene >= position) snapshot.document.scene += 1;
  snapshot.document.scenes.splice(position, 0, { nodes: [] });
};

export const removeCreatedScene = (snapshot: GltfSnapshot, position: number, defaultSceneBefore: number | null): void => {
  snapshot.document.scenes.splice(position, 1);
  if (defaultSceneBefore === null) delete snapshot.document.scene;
  else snapshot.document.scene = defaultSceneBefore;
};
//#endregion 🎬️SceneState
