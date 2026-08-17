/** ↩️ Exact create-scene removal inverse with exhaustive post-state restoration. */
import type { GltfScene, GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { clone, defaultAfter, equal, existingPosition, insertionPosition, reject, removeCreatedScene, type GltfCreateSceneRejection } from '../🔒️private/🟦️component.ts';

export const GltfCreateSceneInverseDescriptor = { id: 's.stdio.gltf.mutation.create-scene.v1', version: 1, phase: 'inverse', touchedPathPatterns: ['document/scenes/{position}', 'document/scene'] } as const;

export interface GltfCreateSceneInverse { readonly id: 's.stdio.gltf.mutation.create-scene.v1'; readonly version: 1; readonly phase: 'inverse'; readonly touchedPaths: readonly string[]; readonly position: number; readonly expectedSceneCountAfter: number; readonly expectedScene: GltfScene; readonly expectedScenesAfter: readonly GltfScene[]; readonly defaultSceneBefore: number | null; readonly expectedDefaultSceneAfter: number | null; }

const paths = (position: number, before: number | null, after: number | null): readonly string[] => before === after ? [`document/scenes/${position}`] : [`document/scenes/${position}`, 'document/scene'];
const expectedScene = (): GltfScene => ({ nodes: [] });

export const touchedPathsForGltfCreateSceneInverse = (inverse: Pick<GltfCreateSceneInverse, 'position'|'defaultSceneBefore'|'expectedDefaultSceneAfter'>): readonly string[] => paths(inverse.position, inverse.defaultSceneBefore, inverse.expectedDefaultSceneAfter);

const validateRestoredDefault = (inverse: GltfCreateSceneInverse, after: GltfSnapshot): GltfCreateSceneRejection | undefined => {
  const restoredCount = after.document.scenes.length - 1;
  if (inverse.defaultSceneBefore !== null && (!Number.isInteger(inverse.defaultSceneBefore) || inverse.defaultSceneBefore < 0 || inverse.defaultSceneBefore >= restoredCount)) return reject('gltf.mutation.reference-out-of-range', 'inverse/defaultSceneBefore', 'restored default scene must name a surviving scene');
  return inverse.expectedDefaultSceneAfter === defaultAfter(inverse.defaultSceneBefore, inverse.position) ? undefined : reject('gltf.mutation.invalid-inverse-reference', 'inverse/expectedDefaultSceneAfter', 'default-scene repair must match the restored default scene');
};

export const validateGltfCreateSceneInverse = (inverse: GltfCreateSceneInverse, after: GltfSnapshot): GltfCreateSceneRejection | undefined => {
  if (inverse.id !== GltfCreateSceneInverseDescriptor.id || inverse.version !== 1 || inverse.phase !== 'inverse') return reject('gltf.mutation.invalid-inverse-envelope', 'inverse', 'canonical identity or phase does not match');
  const range = existingPosition(inverse.position, after);
  if (range) return range;
  if (inverse.expectedSceneCountAfter !== after.document.scenes.length) return reject('gltf.mutation.stale-inverse', 'inverse/expectedSceneCountAfter', 'scene collection no longer matches the forward-created state');
  const restoredDefault = validateRestoredDefault(inverse, after);
  if (restoredDefault) return restoredDefault;
  if (!equal(inverse.touchedPaths, touchedPathsForGltfCreateSceneInverse(inverse))) return reject('gltf.mutation.invalid-touched-paths', 'inverse/touchedPaths', 'paths must name every concrete changed location');
  if (!equal(inverse.expectedScene, expectedScene())) return reject('gltf.mutation.invalid-created-scene', 'inverse/expectedScene', 'inverse must target the canonical empty scene');
  if (inverse.expectedDefaultSceneAfter !== (after.document.scene ?? null)) return reject('gltf.mutation.stale-inverse', 'document/scene', 'default scene does not match the forward-created state');
  if (!equal(after.document.scenes[inverse.position], inverse.expectedScene)) return reject('gltf.mutation.stale-inverse', `document/scenes/${inverse.position}`, 'current scene does not match the forward-created scene');
  return equal(inverse.expectedScenesAfter, after.document.scenes) ? undefined : reject('gltf.mutation.stale-inverse', 'document/scenes', 'scene sequence no longer matches the forward-created state');
};

export const applyGltfCreateSceneInverse = (after: GltfSnapshot, inverse: GltfCreateSceneInverse): { accepted: true; snapshot: GltfSnapshot } | { accepted: false; rejection: GltfCreateSceneRejection } => {
  const rejection = validateGltfCreateSceneInverse(inverse, after);
  if (rejection) return { accepted: false, rejection };
  const snapshot = clone(after);
  removeCreatedScene(snapshot, inverse.position, inverse.defaultSceneBefore);
  return { accepted: true, snapshot };
};

export const encodeGltfCreateSceneInverse = (inverse: GltfCreateSceneInverse): string => JSON.stringify(inverse);

export const deriveGltfCreateSceneInverse = (base: GltfSnapshot, payload: { position: number }): { accepted: true; inverse: GltfCreateSceneInverse; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfCreateSceneRejection } => {
  const range = insertionPosition(payload.position, base);
  if (range) return { accepted: false, rejection: range };
  const defaultSceneBefore = base.document.scene ?? null;
  const expectedDefaultSceneAfter = defaultAfter(defaultSceneBefore, payload.position);
  const expectedScenesAfter = structuredClone(base.document.scenes);
  expectedScenesAfter.splice(payload.position, 0, expectedScene());
  const preliminary = { position: payload.position, defaultSceneBefore, expectedDefaultSceneAfter };
  const touchedPaths = touchedPathsForGltfCreateSceneInverse(preliminary);
  return { accepted: true, inverse: { id: 's.stdio.gltf.mutation.create-scene.v1', version: 1, phase: 'inverse', touchedPaths, position: payload.position, expectedSceneCountAfter: expectedScenesAfter.length, expectedScene: expectedScene(), expectedScenesAfter, defaultSceneBefore, expectedDefaultSceneAfter }, touchedPaths };
};
