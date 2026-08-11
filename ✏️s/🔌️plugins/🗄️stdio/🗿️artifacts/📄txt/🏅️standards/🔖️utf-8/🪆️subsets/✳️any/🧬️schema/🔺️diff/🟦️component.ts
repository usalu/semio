import type { LineEnding } from '../📸️snapshot/🟦️component.ts';

/** ➕️ A line added at a FINAL-state index, carrying its full text. */
export interface TxtLineAdded {
  index: number;
  text: string;
}

/** ✏️ A line at a BASE-state index whose text changed. */
export interface TxtLineModified {
  index: number;
  text: string;
}

/** 🧮 Index-keyed triple over `TxtSnapshot.lines`. */
export interface TxtLinesDiff {
  removed: number[];
  modified: TxtLineModified[];
  added: TxtLineAdded[];
}

/** 🔺️ TxtDiff schema — every mutable field is optional (present = changed). */
export interface TxtDiff {
  trailingNewline?: boolean;
  lineEnding?: LineEnding;
  lines?: TxtLinesDiff;
}
