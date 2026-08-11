/** 🔺️ StepDiff schema — handcrafted sparse diff mirroring the Rust `StepDiff` shape 1:1. Three
 * HEADER records are whole-value replaced (weak, per the recipe); `entities` is an id-keyed
 * triple whose `args` sub-diff is a SEPARATE index-keyed triple (Part-21 argument lists are
 * positional). */

import type { StepComplexType, StepEntity, StepFileDescription, StepFileName, StepFileSchema, StepValue } from '../📸️snapshot/🟦️component.ts';

/** 🔺️ One `args.modified[]`/`args.added[]` entry — `StepValue` is weak, so the "diff" IS the
 * whole new value. */
export interface StepArgModified {
  index: number;
  value: StepValue;
}
export interface StepArgAdded {
  index: number;
  value: StepValue;
}

/** 🔺️ Index-keyed collection triple for `StepEntity.args`. */
export interface StepArgsDiff {
  removed?: number[];
  modified?: StepArgModified[];
  added?: StepArgAdded[];
}

/** 🔺️ Sparse per-field diff for one `StepEntity`. `complex` is a weak value list, whole-vec
 * replaced. */
export interface StepEntityDiff {
  name?: string;
  args?: StepArgsDiff;
  complex?: StepComplexType[];
}

/** 📦️ One `entities.modified[]` entity — `id` is stable Part-21 instance-number identity. */
export interface StepEntityModified {
  id: number;
  diff: StepEntityDiff;
}

/** 📦️ One `entities.added[]` entity — `index` is the position in the FINAL sequence. */
export interface StepEntityAdded {
  index: number;
  entity: StepEntity;
}

/** 📦️ Sparse id-keyed `entities` triple. */
export interface StepEntitiesDiff {
  removed?: number[];
  modified?: StepEntityModified[];
  added?: StepEntityAdded[];
}

/** 🔺️ Diff for `stdio.step`. `schema` is an identity field and never appears here. */
export interface StepDiff {
  fileDescription?: StepFileDescription;
  fileName?: StepFileName;
  fileSchema?: StepFileSchema;
  entities?: StepEntitiesDiff;
}
