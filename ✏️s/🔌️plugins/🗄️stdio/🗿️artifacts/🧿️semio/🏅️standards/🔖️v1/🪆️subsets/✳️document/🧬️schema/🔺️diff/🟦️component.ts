/** 🔺️ SemioDocumentDiff — real TS mirror of the hand-rolled sparse diff (see `🦀️component.rs`).
 * `styles`/`images` are name-keyed triples; `blocks` is an index-keyed recursive triple whose
 * per-item diff shape mirrors `DocBlock`'s own kind, `replace` covering a block-kind change. */

export interface IndexModified<D> { index: number; diff: D; }
export interface IndexAdded<T> { index: number; item: T; }
export interface IndexedTripleDiff<D, T> { removed: number[]; modified: IndexModified<D>[]; added: IndexAdded<T>[]; }

export interface NamedModified<K, D> { key: K; diff: D; }
export interface NamedTripleDiff<K, D, T> { removed: K[]; modified: NamedModified<K, D>[]; added: T[]; }

export interface DocStyleDiff { name?: string; basedOn?: string | null; }
export interface DocImageDiff { mime?: string; bytes?: number[]; }
export interface RunStyleDiff { bold?: boolean; italic?: boolean; underline?: boolean; size?: number | null; font?: string | null; color?: string | null; link?: string | null; }
export interface DocRunDiff { text?: string; style?: RunStyleDiff; }
export interface DocListItemDiff { blocks?: BlocksDiff; }
export interface DocTableCellDiff { blocks?: BlocksDiff; }
export interface DocTableRowDiff { cells?: IndexedTripleDiff<DocTableCellDiff, unknown>; }

export type DocBlockDiff =
  | { kind: "paragraph"; styleId?: string | null; runs?: IndexedTripleDiff<DocRunDiff, unknown> }
  | { kind: "heading"; level?: number; styleId?: string | null; runs?: IndexedTripleDiff<DocRunDiff, unknown> }
  | { kind: "list"; ordered?: boolean; items?: IndexedTripleDiff<DocListItemDiff, unknown> }
  | { kind: "table"; rows?: IndexedTripleDiff<DocTableRowDiff, unknown> }
  | { kind: "code"; language?: string | null; text?: string }
  | { kind: "quote"; blocks?: BlocksDiff }
  | { kind: "image"; imageId?: string; alt?: string; width?: number | null; height?: number | null }
  | { kind: "replace"; block: unknown };

export type BlocksDiff = IndexedTripleDiff<DocBlockDiff, unknown>;

export interface SemioDocumentDiff {
  styles?: NamedTripleDiff<string, DocStyleDiff, unknown>;
  images?: NamedTripleDiff<string, DocImageDiff, unknown>;
  blocks?: BlocksDiff;
}
