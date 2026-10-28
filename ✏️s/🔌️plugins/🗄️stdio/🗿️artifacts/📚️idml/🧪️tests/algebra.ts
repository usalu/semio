import { deepStrictEqual, throws } from 'node:assert/strict';
import jsonPatch from 'fast-json-patch';
import type { Operation } from 'fast-json-patch';
import corpus from './mutations.json' with { type: 'json' };
import { applyIdmlDiff, composeIdmlDiffs, diffIdmlSnapshots, inverseIdmlDiff, mutationIdmlDiff, inverseIdmlMutation, type IdmlAdmission } from '../🧬️schema/🔺️diff/🧮️algebra/🟦️.ts';
import type { IdmlMutation } from '../🧬️schema/🧬️mutations/🟦️.ts';

/** 🧪️ Executes canonical package events through a separately supplied XML package oracle. */
export function verifyIdmlAlgebra(admission: IdmlAdmission): { cases: number; refused: number } {
  let refused = 0;
  for (const vector of corpus.cases) {
    const before = structuredClone(vector.before), saved = structuredClone(before);
    const mutation = vector.mutation as IdmlMutation;
    if (!vector.accepted) { throws(() => mutationIdmlDiff(before, mutation, admission), vector.name); refused++; }
    else {
      deepStrictEqual(jsonPatch.applyPatch(structuredClone(before), vector.patch as Operation[], true, false).newDocument, vector.after, vector.name + ': RFC6902');
      const diff = mutationIdmlDiff(before, mutation, admission), after = applyIdmlDiff(before, diff, admission);
      deepStrictEqual(after, vector.after, vector.name + ': apply');
      deepStrictEqual(applyIdmlDiff(after, inverseIdmlDiff(before, diff, admission), admission), before, vector.name + ': diff inverse');
      let restored = after;
      for (const inverse of inverseIdmlMutation(before, mutation, admission)) restored = applyIdmlDiff(restored, mutationIdmlDiff(restored, inverse, admission), admission);
      deepStrictEqual(restored, before, vector.name + ': semantic inverse');
      deepStrictEqual(applyIdmlDiff(before, composeIdmlDiffs(before, [diff, inverseIdmlDiff(before, diff, admission)], admission), admission), before);
    }
    deepStrictEqual(before, saved, vector.name + ': immutable');
  }
  const before = corpus.cases[0].before;
  const bad = { steps: [{ parts: [{ path: 'designmap.xml', bytes: [60, 68, 111, 99, 62] }] }, { parts: [{ path: 'designmap.xml', bytes: before.parts[1]!.bytes }] }] };
  throws(() => applyIdmlDiff(before, bad, admission), 'invalid intermediate graph remains refused');
  throws(() => applyIdmlDiff(before, { steps: [{ parts: [{ path: 'missing', bytes: null }] }] }, admission), 'missing removal');
  throws(() => applyIdmlDiff(before, { steps: [{ parts: [], partOrder: ['mimetype', 'mimetype'] }] }, admission), 'duplicate order');
  throws(() => applyIdmlDiff(before, { steps: [{ parts: [{ path: 'designmap.xml', bytes: before.parts[1]!.bytes }, { path: 'designmap.xml', bytes: before.parts[1]!.bytes }] }] }, admission), 'duplicate path');
  deepStrictEqual(diffIdmlSnapshots(before, before), { steps: [] });
  return { cases: corpus.cases.length, refused: refused + 4 };
}
