/** 🧪️ Executes the shared alpha-mode mutation, diff, inverse, stale, and path laws. */
import assert from 'node:assert/strict';
import vector from './🔣️component.json' with { type: 'json' };
import type { GltfAlphaMode, GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeMaterialAlphaMode, type GltfChangeMaterialAlphaModePayload } from '../🟦️component.ts';
import { applyGltfChangeMaterialAlphaModeDiff, deriveGltfChangeMaterialAlphaModeDiff, type GltfChangeMaterialAlphaModeDiff } from '../🔺️diff/🟦️component.ts';
import { applyGltfChangeMaterialAlphaModeInverse, reconstructGltfChangeMaterialAlphaModeInverse, type GltfChangeMaterialAlphaModeInverse } from '../↩️inverse/🟦️component.ts';
interface CanonicalVector {
  base: { material: number; alphaMode: GltfAlphaMode };
  mutation: GltfChangeMaterialAlphaModePayload;
  diff: GltfChangeMaterialAlphaModeDiff;
  inverse: GltfChangeMaterialAlphaModeInverse;
  after: { material: number; alphaMode: GltfAlphaMode };
  undo: { material: number; alphaMode: GltfAlphaMode };
  rejections: { staleDiff: string; staleInverse: string; forgedPath: string };
}
export const GltfChangeMaterialAlphaModeCanonicalVector = vector.vectors[0] as CanonicalVector;
const snapshot = (alphaMode: GltfAlphaMode): GltfSnapshot => ({
  schema: 'gltf/2.0', sourceForm: 'json', buffers: [],
  document: {
    asset: { version: '2.0' }, scenes: [], nodes: [], meshes: [], accessors: [], bufferViews: [], buffers: [],
    materials: [{ emissiveFactor: [0, 0, 0], alphaMode, alphaCutoff: 0.5, doubleSided: false }],
    textures: [], images: [], samplers: [], skins: [], animations: [], cameras: [], extensionsUsed: [], extensionsRequired: [],
  },
});
export const runGltfChangeMaterialAlphaModeContract = (): void => {
  const item = GltfChangeMaterialAlphaModeCanonicalVector;
  const base = snapshot(item.base.alphaMode);
  const mutationState = structuredClone(base);
  assert.equal(applyGltfChangeMaterialAlphaMode(mutationState, item.mutation).accepted, true);
  assert.equal(mutationState.document.materials[0].alphaMode, item.after.alphaMode);
  const planned = deriveGltfChangeMaterialAlphaModeDiff(base, item.mutation);
  const inverted = reconstructGltfChangeMaterialAlphaModeInverse(base, item.mutation);
  assert.equal(planned.accepted, true);
  assert.equal(inverted.accepted, true);
  assert.deepEqual(planned.diff, item.diff);
  assert.deepEqual(inverted.inverse, item.inverse);
  const diffState = structuredClone(base);
  assert.equal(applyGltfChangeMaterialAlphaModeDiff(diffState, planned.diff), undefined);
  assert.equal(diffState.document.materials[0].alphaMode, item.after.alphaMode);
  assert.equal(applyGltfChangeMaterialAlphaModeDiff(diffState, planned.diff)?.code, item.rejections.staleDiff);
  const forged = { ...planned.diff, touchedPaths: ['document/materials/9/alphaMode'] };
  assert.equal(applyGltfChangeMaterialAlphaModeDiff(structuredClone(base), forged)?.code, item.rejections.forgedPath);
  assert.equal(applyGltfChangeMaterialAlphaModeInverse(diffState, inverted.inverse), undefined);
  assert.equal(diffState.document.materials[0].alphaMode, item.undo.alphaMode);
  assert.equal(applyGltfChangeMaterialAlphaModeInverse(diffState, inverted.inverse)?.code, item.rejections.staleInverse);
};
