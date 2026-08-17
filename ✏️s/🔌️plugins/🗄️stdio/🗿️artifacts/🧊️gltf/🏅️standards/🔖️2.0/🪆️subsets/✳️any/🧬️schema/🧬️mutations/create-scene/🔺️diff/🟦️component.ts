/** 🔺️ Exact create-scene insertion delta with exhaustive pre-state protection. */
import type { GltfScene, GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { clone, defaultAfter, equal, insertEmptyScene, insertionPosition, reject, type GltfCreateSceneRejection } from '../🔒️private/🟦️component.ts';

export const GltfCreateSceneDiffDescriptor = { id: 's.stdio.gltf.mutation.create-scene.v1', version: 1, phase: 'diff', touchedPathPatterns: ['document/scenes/{position}', 'document/scene'] } as const;

export interface GltfCreateSceneDiff { readonly id: 's.stdio.gltf.mutation.create-scene.v1'; readonly version: 1; readonly phase: 'diff'; readonly touchedPaths: readonly string[]; readonly position: number; readonly expectedSceneCount: number; readonly expectedDefaultSceneBefore: number | null; readonly expectedScenesBefore: readonly GltfScene[]; readonly scene: GltfScene; }

const paths = (position: number, expectedDefaultSceneBefore: number | null): readonly string[] => expectedDefaultSceneBefore === defaultAfter(expectedDefaultSceneBefore, position) ? [`document/scenes/${position}`] : [`document/scenes/${position}`, 'document/scene'];
const expectedScene = (): GltfScene => ({ nodes: [] });
const stale = (path: string, detail: string): GltfCreateSceneRejection => reject('gltf.mutation.stale-diff', path, detail);

export const touchedPathsForGltfCreateSceneDiff = (diff: Pick<GltfCreateSceneDiff, 'position'|'expectedDefaultSceneBefore'>): readonly string[] => paths(diff.position, diff.expectedDefaultSceneBefore);

export const validateGltfCreateSceneDiff = (diff: GltfCreateSceneDiff, base: GltfSnapshot): GltfCreateSceneRejection | undefined => {
  if (diff.id !== GltfCreateSceneDiffDescriptor.id || diff.version !== 1 || diff.phase !== 'diff') return reject('gltf.mutation.invalid-diff-envelope', 'diff', 'canonical identity or phase does not match');
  const range = insertionPosition(diff.position, base);
  if (range) return range;
  if (diff.expectedSceneCount !== base.document.scenes.length) return stale('diff/expectedSceneCount', 'scene collection no longer matches the planned pre-state');
  if (diff.expectedDefaultSceneBefore !== (base.document.scene ?? null)) return stale('document/scene', 'default scene no longer matches the planned pre-state');
  if (!equal(diff.expectedScenesBefore, base.document.scenes)) return stale('document/scenes', 'scene sequence no longer matches the planned pre-state');
  if (!equal(diff.touchedPaths, touchedPathsForGltfCreateSceneDiff(diff))) return reject('gltf.mutation.invalid-touched-paths', 'diff/touchedPaths', 'paths must name every concrete changed location');
  return equal(diff.scene, expectedScene()) ? undefined : reject('gltf.mutation.invalid-created-scene', 'diff/scene', 'create-scene may only insert the canonical empty scene');
};

export const applyGltfCreateSceneDiff = (base: GltfSnapshot, diff: GltfCreateSceneDiff): { accepted: true; snapshot: GltfSnapshot } | { accepted: false; rejection: GltfCreateSceneRejection } => {
  const rejection = validateGltfCreateSceneDiff(diff, base);
  if (rejection) return { accepted: false, rejection };
  const snapshot = clone(base);
  insertEmptyScene(snapshot, diff.position);
  return { accepted: true, snapshot };
};

export const encodeGltfCreateSceneDiff = (diff: GltfCreateSceneDiff): string => JSON.stringify(diff);

export const deriveGltfCreateSceneDiff = (base: GltfSnapshot, payload: { position: number }): { accepted: true; diff: GltfCreateSceneDiff; touchedPaths: readonly string[] } | { accepted: false; rejection: GltfCreateSceneRejection } => {
  const range = insertionPosition(payload.position, base);
  if (range) return { accepted: false, rejection: range };
  const expectedDefaultSceneBefore = base.document.scene ?? null;
  const preliminary = { position: payload.position, expectedDefaultSceneBefore };
  const touchedPaths = touchedPathsForGltfCreateSceneDiff(preliminary);
  return { accepted: true, diff: { id: 's.stdio.gltf.mutation.create-scene.v1', version: 1, phase: 'diff', touchedPaths, position: payload.position, expectedSceneCount: base.document.scenes.length, expectedDefaultSceneBefore, expectedScenesBefore: structuredClone(base.document.scenes), scene: expectedScene() }, touchedPaths };
};
