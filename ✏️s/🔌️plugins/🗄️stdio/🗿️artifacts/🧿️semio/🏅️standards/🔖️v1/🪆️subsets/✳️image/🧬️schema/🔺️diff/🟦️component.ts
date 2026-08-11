/** 🔺️ SemioImageDiff schema — real facet mirror of the Rust `🦀️component.rs` sibling. Sparse
 * per-field diff; `frames`/`metadata` are index/name-keyed collection triples. */
import type { SemioColorspace, SemioImageFrame, SemioImageMetadataEntry } from "../📸️snapshot/🟦️component.ts";

export interface SemioImageFrameDiff {
  delayMs?: number;
  rgba8?: string;
}
export interface IndexModified<D> { index: number; diff: D }
export interface IndexAdded<T> { index: number; item: T }
export interface SemioImageFramesDiff {
  removed: number[];
  modified: IndexModified<SemioImageFrameDiff>[];
  added: IndexAdded<SemioImageFrame>[];
}

export interface NamedModified<D> { key: string; diff: D }
export interface SemioImageMetadataDiff {
  removed: string[];
  modified: NamedModified<string>[]; // weak collection: diff IS the whole new value
  added: SemioImageMetadataEntry[];
}

export interface SemioImageDiff {
  width?: number;
  height?: number;
  colorspace?: SemioColorspace;
  bitDepth?: number;
  /** tri-state: absent = unchanged, null = cleared, string = set (hex) */
  icc?: string | null;
  frames?: SemioImageFramesDiff;
  metadata?: SemioImageMetadataDiff;
}
