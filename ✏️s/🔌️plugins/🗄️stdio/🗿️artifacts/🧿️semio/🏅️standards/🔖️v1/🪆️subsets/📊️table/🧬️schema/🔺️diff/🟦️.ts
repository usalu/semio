/** 🔺️ SemioTableDiff schema — keyed-row diff mirror of the Rust `🦀️.rs` sibling: per collection `removed` base indices, `modified` rows and `added` rows with their final index. */
export interface IndexModified<D> { index: number; diff: D }
export interface IndexAdded<T> { index: number; item: T }
export interface Replace<T> { value: T }
export interface IndexedTripleDiff<D, T> { removed?: number[]; modified?: IndexModified<D>[]; added?: IndexAdded<T>[] }
export interface SemioTableColumnDiff { name?: string | null; kind?: import("../📸️snapshot/🟦️.ts").SemioTableCellKind | null }
export interface SemioTableRowDiff { cells?: IndexedTripleDiff<Replace<import("../📸️snapshot/🟦️.ts").SemioValue>, import("../📸️snapshot/🟦️.ts").SemioValue> | null }
export interface SemioTableDiff { columns?: IndexedTripleDiff<SemioTableColumnDiff, import("../📸️snapshot/🟦️.ts").SemioTableColumn> | null; rows?: IndexedTripleDiff<SemioTableRowDiff, import("../📸️snapshot/🟦️.ts").SemioTableRow> | null }
