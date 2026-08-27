/** 🧪️ Executes create-scene laws from the canonical JSON vector. */
import assert from 'node:assert/strict';
import contractJson from './🔣️component.json' with { type: 'json' };
import { applyGltfCreateScene, decodeGltfCreateScenePayload, type GltfCreateScenePayload } from '../🟦️component.ts';
import { applyGltfCreateSceneDiff, deriveGltfCreateSceneDiff, encodeGltfCreateSceneDiff, type GltfCreateSceneDiff } from '../🔺️diff/🟦️component.ts';
import { applyGltfCreateSceneInverse, deriveGltfCreateSceneInverse, encodeGltfCreateSceneInverse, type GltfCreateSceneInverse } from '../↩️inverse/🟦️component.ts';
import type { GltfScene, GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';

interface SceneState { scene: number | null; scenes: GltfScene[] }
interface Rejection { code: string; payload?: GltfCreateScenePayload; base?: SceneState; scene?: GltfScene; touchedPaths?: string[]; position?: number }
interface Vector {
  name: string;
  base: SceneState;
  payload: GltfCreateScenePayload;
  after: SceneState;
  undo: SceneState;
  diff: GltfCreateSceneDiff;
  inverse: GltfCreateSceneInverse;
  malformedPayload: { encoded: string; code: string };
  rejections: Record<string, Rejection>;
}
interface Contract { id: string; vectors: Vector[]; laws: string[] }

export const gltfCreateSceneContract = contractJson as Contract;

const snapshot = (state: SceneState): GltfSnapshot => ({
  schema: 'gltf/2.0',
  sourceForm: 'json',
  buffers: [],
  document: {
    asset: { version: '2.0' },
    ...(state.scene === null ? {} : { scene: state.scene }),
    scenes: structuredClone(state.scenes),
    nodes: [], meshes: [], accessors: [], bufferViews: [], buffers: [], materials: [], textures: [], images: [], samplers: [], skins: [], animations: [], cameras: [], extensionsUsed: [], extensionsRequired: [],
  },
});

const state = (value: GltfSnapshot): SceneState => ({ scene: value.document.scene ?? null, scenes: value.document.scenes });
const rejected = <T extends { accepted: boolean }>(value: T): Extract<T, { accepted: false }> => {
  assert.equal(value.accepted, false, 'operation must reject');
  return value as Extract<T, { accepted: false }>;
};
const accepted = <T extends { accepted: boolean }>(value: T): Extract<T, { accepted: true }> => {
  assert.equal(value.accepted, true, 'operation must accept');
  return value as Extract<T, { accepted: true }>;
};

export const runGltfCreateSceneContract = (): void => {
  assert.equal(gltfCreateSceneContract.id, 's.stdio.gltf.mutation.create-scene.v1');
  for (const vector of gltfCreateSceneContract.vectors) {
    const base = snapshot(vector.base);
    const direct = accepted(applyGltfCreateScene(base, vector.payload));
    assert.deepEqual(state(direct.snapshot), vector.after, `${vector.name} mutation produces its canonical after-state`);
    const forward = accepted(deriveGltfCreateSceneDiff(base, vector.payload));
    assert.deepEqual(forward.diff, vector.diff, `${vector.name} diff derives its complete base sequence`);
    assert.deepEqual(accepted(applyGltfCreateSceneDiff(base, forward.diff)).snapshot, direct.snapshot, `${vector.name} diff application equals mutation`);
    const inverse = accepted(deriveGltfCreateSceneInverse(base, vector.payload));
    assert.deepEqual(inverse.inverse, vector.inverse, `${vector.name} inverse derives its complete post-state sequence`);
    assert.deepEqual(state(accepted(applyGltfCreateSceneInverse(direct.snapshot, inverse.inverse)).snapshot), vector.undo, `${vector.name} inverse restores the exact base`);
  }
  const vector = gltfCreateSceneContract.vectors[0]!;
  const rejection = (name: string): Rejection => {
    const value = vector.rejections[name];
    assert.ok(value, `${name} vector exists`);
    return value;
  };
  const base = snapshot(vector.base);

  const malformed = rejected(decodeGltfCreateScenePayload(vector.malformedPayload.encoded));
  assert.equal(malformed.rejection.code, vector.malformedPayload.code, 'malformed payload code is stable');

  const direct = accepted(applyGltfCreateScene(base, vector.payload));
  assert.deepEqual(state(direct.snapshot), vector.after, 'mutation produces canonical after-state');
  const range = rejection('outOfRangePosition');
  assert.equal(rejected(applyGltfCreateScene(base, range.payload!)).rejection.code, range.code, 'range rejection code is stable');
  const invalidReference = rejection('invalidDefaultReference');
  assert.equal(rejected(applyGltfCreateScene(snapshot(invalidReference.base!), vector.payload)).rejection.code, invalidReference.code, 'invalid default reference rejects atomically');

  const forward = accepted(deriveGltfCreateSceneDiff(base, vector.payload));
  assert.deepEqual(forward.diff, vector.diff, 'diff derives exact canonical payload');
  assert.deepEqual(forward.touchedPaths, ['document/scenes/0', 'document/scene'], 'diff path is concrete and indexed');
  const applied = accepted(applyGltfCreateSceneDiff(base, forward.diff));
  assert.deepEqual(applied.snapshot, direct.snapshot, 'diff application equals mutation');
  const replay = rejection('staleDiffReplay');
  assert.equal(rejected(applyGltfCreateSceneDiff(direct.snapshot, forward.diff)).rejection.code, replay.code, 'replay rejects against post-state');
  const staleDefault = rejection('staleDefaultScene');
  assert.equal(rejected(applyGltfCreateSceneDiff(snapshot({ ...vector.base, scene: null }), forward.diff)).rejection.code, staleDefault.code, 'changed default-scene precondition rejects');
  const staleAnchor = rejection('staleInsertionAnchor');
  assert.equal(rejected(applyGltfCreateSceneDiff(snapshot({ ...vector.base, scenes: [staleAnchor.scene!] }), forward.diff)).rejection.code, staleAnchor.code, 'changed insertion anchor rejects');
  const forgedDiff = rejection('forgedDiffTouchedPaths');
  assert.equal(rejected(applyGltfCreateSceneDiff(base, { ...forward.diff, touchedPaths: forgedDiff.touchedPaths! })).rejection.code, forgedDiff.code, 'forged forward paths reject');
  assert.deepEqual(JSON.parse(encodeGltfCreateSceneDiff(forward.diff)), vector.diff, 'diff serialization is stable');

  const undo = accepted(deriveGltfCreateSceneInverse(base, vector.payload));
  assert.deepEqual(undo.inverse, vector.inverse, 'inverse derives exact canonical payload');
  const restored = accepted(applyGltfCreateSceneInverse(direct.snapshot, undo.inverse));
  assert.deepEqual(state(restored.snapshot), vector.undo, 'inverse restores canonical base state');
  const inverseIndex = rejection('inverseIndex');
  assert.equal(rejected(applyGltfCreateSceneInverse(direct.snapshot, { ...undo.inverse, position: inverseIndex.position! })).rejection.code, inverseIndex.code, 'invalid inverse index rejects');
  const staleInverse = rejection('staleInverse');
  assert.equal(rejected(applyGltfCreateSceneInverse({ ...direct.snapshot, document: { ...direct.snapshot.document, scenes: [staleInverse.scene!, ...direct.snapshot.document.scenes.slice(1)] } }, undo.inverse)).rejection.code, staleInverse.code, 'changed created scene rejects inverse');
  const staleInverseAnchor = rejection('staleInverseAnchor');
  assert.equal(rejected(applyGltfCreateSceneInverse({ ...direct.snapshot, document: { ...direct.snapshot.document, scenes: [direct.snapshot.document.scenes[0]!, staleInverseAnchor.scene!] } }, undo.inverse)).rejection.code, staleInverseAnchor.code, 'changed post-insertion anchor rejects inverse');
  const forgedInverse = rejection('forgedInverseTouchedPaths');
  assert.equal(rejected(applyGltfCreateSceneInverse(direct.snapshot, { ...undo.inverse, touchedPaths: forgedInverse.touchedPaths! })).rejection.code, forgedInverse.code, 'forged inverse paths reject');
  assert.deepEqual(JSON.parse(encodeGltfCreateSceneInverse(undo.inverse)), vector.inverse, 'inverse serialization is stable');

  const distant = gltfCreateSceneContract.vectors.find(candidate => candidate.name === 'rejectsDistantSceneStaleness')!;
  assert.ok(distant, 'distant scene vector exists');
  const distantBase = snapshot(distant.base);
  const distantForward = accepted(deriveGltfCreateSceneDiff(distantBase, distant.payload));
  const staleDistantDiff = distant.rejections.staleDistantDiff!;
  assert.equal(rejected(applyGltfCreateSceneDiff(snapshot({ ...distant.base, scenes: [...distant.base.scenes.slice(0, -1), staleDistantDiff.scene!] }), distantForward.diff)).rejection.code, staleDistantDiff.code, 'distant pre-state scene rejects the forward diff');
  const distantAfter = accepted(applyGltfCreateScene(distantBase, distant.payload)).snapshot;
  const distantInverse = accepted(deriveGltfCreateSceneInverse(distantBase, distant.payload));
  const staleDistantInverse = distant.rejections.staleDistantInverse!;
  assert.equal(rejected(applyGltfCreateSceneInverse({ ...distantAfter, document: { ...distantAfter.document, scenes: [...distantAfter.document.scenes.slice(0, -1), staleDistantInverse.scene!] } }, distantInverse.inverse)).rejection.code, staleDistantInverse.code, 'distant post-state scene rejects the inverse');
  const forgedDistantDiff = distant.rejections.forgedDiffTouchedPaths!;
  assert.equal(rejected(applyGltfCreateSceneDiff(distantBase, { ...distantForward.diff, touchedPaths: forgedDistantDiff.touchedPaths! })).rejection.code, forgedDistantDiff.code, 'forged distant forward paths reject');
  const forgedDistantInverse = distant.rejections.forgedInverseTouchedPaths!;
  assert.equal(rejected(applyGltfCreateSceneInverse(distantAfter, { ...distantInverse.inverse, touchedPaths: forgedDistantInverse.touchedPaths! })).rejection.code, forgedDistantInverse.code, 'forged distant inverse paths reject');
};
