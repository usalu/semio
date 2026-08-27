/** 🧪️ Executes the shared double-sided mutation, diff, inverse, stale, and path laws. */
import assert from 'node:assert/strict';
import vector from './🔣️component.json' with { type: 'json' };
import type { GltfSnapshot } from '../../../📸️snapshot/🟦️component.ts';
import { applyGltfChangeMaterialDoubleSided, type GltfChangeMaterialDoubleSidedPayload } from '../🟦️component.ts';
import { applyGltfChangeMaterialDoubleSidedDiff, deriveGltfChangeMaterialDoubleSidedDiff, type GltfChangeMaterialDoubleSidedDiff } from '../🔺️diff/🟦️component.ts';
import { applyGltfChangeMaterialDoubleSidedInverse, reconstructGltfChangeMaterialDoubleSidedInverse, type GltfChangeMaterialDoubleSidedInverse } from '../↩️inverse/🟦️component.ts';
interface CanonicalVector { base: { material: number; doubleSided: boolean }; mutation: GltfChangeMaterialDoubleSidedPayload; diff: GltfChangeMaterialDoubleSidedDiff; inverse: GltfChangeMaterialDoubleSidedInverse; after: { material: number; doubleSided: boolean }; undo: { material: number; doubleSided: boolean }; rejections: { staleDiff: string; staleInverse: string; forgedPath: string } }
export const GltfChangeMaterialDoubleSidedCanonicalVector = vector.vectors[0] as CanonicalVector;
const snapshot = (doubleSided: boolean): GltfSnapshot => ({ schema: 'gltf/2.0', sourceForm: 'json', buffers: [], document: { asset: { version: '2.0' }, scenes: [], nodes: [], meshes: [], accessors: [], bufferViews: [], buffers: [], materials: [{ emissiveFactor: [0, 0, 0], alphaMode: 'OPAQUE', alphaCutoff: 0.5, doubleSided }], textures: [], images: [], samplers: [], skins: [], animations: [], cameras: [], extensionsUsed: [], extensionsRequired: [] } });
export const runGltfChangeMaterialDoubleSidedContract = (): void => {
  const item = GltfChangeMaterialDoubleSidedCanonicalVector; const base = snapshot(item.base.doubleSided); const mutationState = structuredClone(base);
  assert.equal(applyGltfChangeMaterialDoubleSided(mutationState, item.mutation).accepted, true); assert.equal(mutationState.document.materials[0].doubleSided, item.after.doubleSided);
  const planned = deriveGltfChangeMaterialDoubleSidedDiff(base, item.mutation); const inverted = reconstructGltfChangeMaterialDoubleSidedInverse(base, item.mutation); assert.equal(planned.accepted, true); assert.equal(inverted.accepted, true); assert.deepEqual(planned.diff, item.diff); assert.deepEqual(inverted.inverse, item.inverse);
  const diffState = structuredClone(base); assert.equal(applyGltfChangeMaterialDoubleSidedDiff(diffState, planned.diff), undefined); assert.equal(diffState.document.materials[0].doubleSided, item.after.doubleSided); assert.equal(applyGltfChangeMaterialDoubleSidedDiff(diffState, planned.diff)?.code, item.rejections.staleDiff);
  assert.equal(applyGltfChangeMaterialDoubleSidedDiff(structuredClone(base), { ...planned.diff, touchedPaths: ['document/materials/9/doubleSided'] })?.code, item.rejections.forgedPath);
  assert.equal(applyGltfChangeMaterialDoubleSidedInverse(diffState, inverted.inverse), undefined); assert.equal(diffState.document.materials[0].doubleSided, item.undo.doubleSided); assert.equal(applyGltfChangeMaterialDoubleSidedInverse(diffState, inverted.inverse)?.code, item.rejections.staleInverse);
};
