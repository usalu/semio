/** 🔺️ CsvDiff schema facet — mirrors 🦀️component.rs field-for-field. No full-replace slot:
 * every field is a sparse patch, `records` is an index-keyed removed/modified/added triple. */

export interface CsvFieldDiff {
  value?: string;
  quoted?: boolean;
}

/** Positional per-field patch list; `null` at a position means that field is unchanged. */
export interface CsvRecordDiff {
  fields?: (CsvFieldDiff | null)[];
}

export interface CsvRecordModified {
  index: number;
  diff: CsvRecordDiff;
}

export interface CsvRecordAdded {
  index: number;
  record: import('../📸️snapshot/🟦️component.ts').CsvRecord;
}

export interface CsvRecordsDiff {
  removed?: number[];
  modified?: CsvRecordModified[];
  added?: CsvRecordAdded[];
}

export interface CsvDiff {
  hasHeader?: boolean;
  records?: CsvRecordsDiff;
}
