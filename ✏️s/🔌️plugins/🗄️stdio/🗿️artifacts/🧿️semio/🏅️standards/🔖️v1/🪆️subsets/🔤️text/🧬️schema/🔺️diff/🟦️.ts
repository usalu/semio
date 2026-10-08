/** 🔺️ SemioTextDiff schema — keyed-row diff mirror of the Rust `🦀️.rs` sibling: per collection `removed` base indices, `modified` rows and `added` rows with their final index. */
export interface IndexModified<D> { index: number; diff: D }
export interface IndexAdded<T> { index: number; item: T }
export interface Replace<T> { value: T }
export interface IndexedTripleDiff<D, T> { removed?: number[]; modified?: IndexModified<D>[]; added?: IndexAdded<T>[] }
export interface SemioTextRunDiff { language?: string | null; content?: string | null; marks?: IndexedTripleDiff<Replace<import("../📸️snapshot/🟦️.ts").SemioTextMark>, import("../📸️snapshot/🟦️.ts").SemioTextMark> | null }
export interface SemioTextDiff { runs?: IndexedTripleDiff<SemioTextRunDiff, import("../📸️snapshot/🟦️.ts").SemioTextRun> | null }
