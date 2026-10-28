/** 🧮️ IDML semantic decisions, central sparse application and inverse algebra. */
import { IDML_ADMISSION } from '../../../🚪️io/🛂️admission/🟦️.ts';
import { parseIdmlPackageValue, type IdmlPart, type IdmlPackage } from '../../🟦️.ts';
import type { IdmlDiff, IdmlPatch } from '../🟦️.ts';
import type { IdmlMutation } from '../../🧬️mutations/🟦️.ts';

/** 🔌️ The canonical XML/package owner admits members and returns the designmap DOMVersion. */
export interface IdmlAdmission { admit(parts: readonly IdmlPart[]): string }

/** ⚠️ A refused package operation leaves its supplied snapshots and events untouched. */
export class IdmlDiffError extends Error {
  constructor(readonly code: string) { super(code); this.name = 'IdmlDiffError'; }
}

const copy = <T>(value: T): T => structuredClone(value);
const bytesEqual = (a: readonly number[], b: readonly number[]): boolean => a.length === b.length && a.every((byte, index) => byte === b[index]);
const fail = (code: string): never => { throw new IdmlDiffError('stdio.idml.' + code); };

function admit(value: IdmlPackage, admission: IdmlAdmission, deriveVersion = false): IdmlPackage {
  const result = parseIdmlPackageValue(value);
  const version = admission.admit(result.parts);
  if (!/^[0-9]+\.[0-9]+$/.test(version) || !deriveVersion && version !== result.domVersion) fail('dom-version-mismatch');
  result.domVersion = version;
  return result;
}

function changing(before: IdmlPackage, after: IdmlPackage): IdmlPatch {
  const old = new Map(before.parts.map(part => [part.path, part.bytes])), keys = new Set(after.parts.map(part => part.path));
  const patch: IdmlPatch = {parts: before.parts.filter(part => !keys.has(part.path)).map(part => ({path: part.path, bytes: null}))};
  for (const part of after.parts) if (!old.has(part.path) || !bytesEqual(old.get(part.path)!, part.bytes)) patch.parts.push({path: part.path, bytes: [...part.bytes]});
  if (before.domVersion !== after.domVersion) patch.domVersion = after.domVersion;
  if (before.parts.length !== after.parts.length || before.parts.some((part, index) => part.path !== after.parts[index]?.path)) patch.partOrder = after.parts.map(part => part.path);
  return patch;
}

function applyStep(base: IdmlPackage, patch: IdmlPatch, admission: IdmlAdmission = IDML_ADMISSION): IdmlPackage {
  const next = copy(base), seen = new Set<string>();
  for (const change of patch.parts) {
    if (seen.has(change.path)) fail('duplicate-part-patch');
    seen.add(change.path);
    const position = next.parts.findIndex(part => part.path === change.path);
    if (change.bytes === null) {
      if (position < 0) fail('missing-part-patch');
      next.parts.splice(position, 1);
    } else if (position < 0) next.parts.push({path: change.path, bytes: [...change.bytes]});
    else next.parts[position]!.bytes = [...change.bytes];
  }
  if (patch.partOrder !== undefined) {
    if (patch.partOrder.length !== next.parts.length || new Set(patch.partOrder).size !== patch.partOrder.length) fail('part-order');
    const parts = new Map(next.parts.map(part => [part.path, part]));
    next.parts = patch.partOrder.map(path => parts.get(path) ?? fail('part-order'));
  }
  if (patch.domVersion !== undefined) next.domVersion = patch.domVersion;
  return admit(next, admission);
}

/** 🛡️ Applies every event step to an unpublished candidate and admits each intermediate package. */
export function applyIdmlDiff(base: IdmlPackage, diff: IdmlDiff, admission: IdmlAdmission = IDML_ADMISSION): IdmlPackage {
  let next = copy(base);
  for (const patch of diff.steps) next = applyStep(next, patch, admission);
  return next;
}

/** 🔺️ Projects exact ordered member changes between two admitted package snapshots. */
export function diffIdmlSnapshots(before: IdmlPackage, after: IdmlPackage): IdmlDiff {
  const patch = changing(before, after);
  return {steps: patch.parts.length || patch.domVersion !== undefined || patch.partOrder !== undefined ? [patch] : []};
}

/** 🧬️ Decides a package replacement or member edit through the canonical admission owner. */
export function mutationIdmlDiff(base: IdmlPackage, mutation: IdmlMutation, admission: IdmlAdmission = IDML_ADMISSION): IdmlDiff {
  let candidate: IdmlPackage;
  if (mutation.mutation === 'replacePackage') candidate = admit(mutation.document, admission);
  else {
    candidate = copy(base);
    const part = candidate.parts.find(part => part.path === mutation.path);
    if (!part) fail('missing-member');
    part!.bytes = [...mutation.bytes];
    candidate = admit(candidate, admission, true);
  }
  return diffIdmlSnapshots(base, candidate);
}

/** ↩️ Captures every prior package field in reverse event order. */
export function inverseIdmlDiff(base: IdmlPackage, diff: IdmlDiff, admission: IdmlAdmission = IDML_ADMISSION): IdmlDiff {
  let next = copy(base);
  const inverse: IdmlPatch[] = [];
  for (const patch of diff.steps) { const after = applyStep(next, patch, admission); inverse.push(changing(after, next)); next = after; }
  return {steps: inverse.reverse()};
}

/** 🎞️ Preserves sequential event admission while composing a stream against its exact base. */
export function composeIdmlDiffs(base: IdmlPackage, diffs: readonly IdmlDiff[], admission: IdmlAdmission = IDML_ADMISSION): IdmlDiff {
  const combined = {steps: diffs.flatMap(diff => copy(diff.steps))};
  applyIdmlDiff(base, combined, admission);
  return combined;
}

/** ⏮️ Produces semantic inverse operations after a successful, effective command. */
export function inverseIdmlMutation(base: IdmlPackage, mutation: IdmlMutation, admission: IdmlAdmission = IDML_ADMISSION): IdmlMutation[] {
  if (mutationIdmlDiff(base, mutation, admission).steps.length === 0) return [];
  if (mutation.mutation === 'replacePackage') return [{mutation: 'replacePackage', document: copy(base)}];
  const part = base.parts.find(part => part.path === mutation.path) ?? fail('missing-member');
  return [{mutation: 'replacePart', path: part.path, bytes: [...part.bytes]}];
}
