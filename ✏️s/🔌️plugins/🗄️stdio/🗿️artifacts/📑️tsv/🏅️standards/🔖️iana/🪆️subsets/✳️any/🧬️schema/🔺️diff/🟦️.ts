/** 🔺️ TsvDiff schema facet — mirrors 🦀️.rs field-for-field. `records` is an
 * index-keyed removed/modified/added triple with a sparse positional per-column patch. */
import type { TsvLineEnding } from '../📸️snapshot/🟦️.ts';

/** Positional per-column patch list; `null` at a position means that column is unchanged. */
export interface TsvRowDiff {
  fields?: (string | null)[];
}

export interface TsvRowModified { index: number; diff: TsvRowDiff; }
export interface TsvRowAdded { index: number; row: string[]; }
export interface TsvRowsDiff { removed?: number[]; modified?: TsvRowModified[]; added?: TsvRowAdded[]; }

export interface TsvDiff {
  trailingNewline?: boolean;
  lineEnding?: TsvLineEnding;
  records?: TsvRowsDiff;
}
