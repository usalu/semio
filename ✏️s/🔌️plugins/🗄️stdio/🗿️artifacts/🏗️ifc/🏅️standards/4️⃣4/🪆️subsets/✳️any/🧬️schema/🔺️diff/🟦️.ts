import type { IfcComplexType, IfcEntity, IfcValue } from "../📸️snapshot/🟦️.ts";

/** 🔺️ One `args.modified[]`/`added[]` entry — `IfcValue` is a weak/value leaf. */
export interface IfcArgModified {
  index: number;
  value: IfcValue;
}
export interface IfcArgAdded {
  index: number;
  value: IfcValue;
}

/** 🔺️ Index-keyed collection triple for one entity's positional `args`. */
export interface IfcArgsDiff {
  removed?: number[];
  modified?: IfcArgModified[];
  added?: IfcArgAdded[];
}

/** 🔺️ Sparse per-field diff for one `IfcEntity`. `id` is identity, never diffed. */
export interface IfcEntityDiff {
  name?: string;
  args?: IfcArgsDiff;
  complex?: IfcComplexType[];
}

export interface IfcEntityModified {
  id: number;
  diff: IfcEntityDiff;
}
export interface IfcEntityAdded {
  index: number;
  entity: IfcEntity;
}

/** 📦️ Sparse id-keyed `entities` triple. */
export interface IfcEntitiesDiff {
  removed?: number[];
  modified?: IfcEntityModified[];
  added?: IfcEntityAdded[];
}

/** 🔺️ IfcDiff — no `snapshot` full-replace slot. `schema` is identity and never appears. */
export interface IfcDiff {
  fileDescription?: IfcValue[];
  fileName?: IfcValue[];
  fileSchema?: IfcValue[];
  entities?: IfcEntitiesDiff;
}
