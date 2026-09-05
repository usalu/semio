/** 🔺️ SemioPresentationDiff — handcrafted sparse diff. Generic triple types are this subset's own
 * local copy (see the Rust file's module doc comment for why). */
import type { SlideFrame, SlidePictureImage, PlaceholderKind, SlideShape, Slide, SlideMaster, SlideLayout } from "../📸️snapshot/🟦️";
import type { DocBlock } from "../../../✳️document/🧬️schema/📸️snapshot/🟦️";

export interface IndexModified<D> { index: number; diff: D; }
export interface IndexAdded<T> { index: number; item: T; }
export interface IndexedTripleDiff<D, T> { removed: number[]; modified: IndexModified<D>[]; added: IndexAdded<T>[]; }
export interface NamedModified<K, D> { key: K; diff: D; }
export interface NamedTripleDiff<K, D, T> { removed: K[]; modified: NamedModified<K, D>[]; added: T[]; }

export interface SlideFrameDiff { origin?: { x: number; y: number }; width?: number; height?: number; }
export interface SlidePictureImageDiff { assetId?: string; mime?: string; bytes?: number[]; }
export type DocBlocksDiff = IndexedTripleDiff<DocBlock, DocBlock>; // whole-value (D = T), see doc comment

export type SlideShapeDiff =
  | { shapeKind: "textBox"; frame?: SlideFrameDiff; blocks?: DocBlocksDiff }
  | { shapeKind: "picture"; frame?: SlideFrameDiff; image?: SlidePictureImageDiff }
  | { shapeKind: "table"; frame?: SlideFrameDiff; rows?: IndexedTripleDiff<SlideTableRowDiff, unknown> }
  | { shapeKind: "placeholder"; frame?: SlideFrameDiff; kind?: PlaceholderKind }
  | { shapeKind: "replace"; shape: SlideShape };

export interface SlideTableCellDiff { blocks?: DocBlocksDiff; }
export interface SlideTableRowDiff { cells?: IndexedTripleDiff<SlideTableCellDiff, unknown>; }

export type SlideShapesDiff = IndexedTripleDiff<SlideShapeDiff, SlideShape>;
export interface SlideMasterDiff { shapes?: SlideShapesDiff; }
export interface SlideLayoutDiff { masterId?: string; shapes?: SlideShapesDiff; }
export interface SlideDiff {
  /** tri-state: absent = unchanged, null = cleared, string = set */
  layoutId?: string | null;
  shapes?: SlideShapesDiff;
  notes?: DocBlocksDiff;
}

export interface SemioPresentationDiff {
  masters?: NamedTripleDiff<string, SlideMasterDiff, SlideMaster>;
  layouts?: NamedTripleDiff<string, SlideLayoutDiff, SlideLayout>;
  slides?: IndexedTripleDiff<SlideDiff, Slide>;
}
