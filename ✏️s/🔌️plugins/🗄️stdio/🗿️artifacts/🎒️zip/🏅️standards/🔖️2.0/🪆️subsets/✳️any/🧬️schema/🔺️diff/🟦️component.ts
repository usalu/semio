/** 🔺️ ZipDiff schema — handcrafted sparse diff mirroring the Rust `ZipDiff` shape 1:1. */

/** 🎒️ Per-field patch for one `ZipEntry`. `name` present = rename. `unixMtime` is tri-state:
 * absent = unchanged, `null` = the Info-ZIP `UT` timestamp was cleared, a number = set. */
export interface ZipEntryDiff {
  name?: string;
  data?: number[];
  method?: 'stored' | 'deflate';
  dosDate?: number;
  dosTime?: number;
  unixMtime?: number | null;
  flags?: number;
  versionMadeBy?: number;
  versionNeeded?: number;
  internalAttrs?: number;
  externalAttrs?: number;
  localExtra?: import('../📸️snapshot/🟦️component.ts').ZipExtraField[];
  centralExtra?: import('../📸️snapshot/🟦️component.ts').ZipExtraField[];
  comment?: string;
}

/** 📦️ One `entries.modified[]` entity — `name` is the entry's identity in BASE (pre-rename). */
export interface ZipEntryModified {
  name: string;
  diff: ZipEntryDiff;
}

/** 📦️ One `entries.added[]` entity — `index` is the position in the FINAL sequence. */
export interface ZipEntryAdded {
  index: number;
  entry: import('../📸️snapshot/🟦️component.ts').ZipEntry;
}

/** 📦️ Sparse name-keyed `entries` triple. */
export interface ZipEntriesDiff {
  removed: string[];
  modified: ZipEntryModified[];
  added: ZipEntryAdded[];
}

/** 🔺️ Diff for `stdio.zip`. `schema` is an identity field and never appears here. */
export interface ZipDiff {
  comment?: string;
  entries?: ZipEntriesDiff;
}
