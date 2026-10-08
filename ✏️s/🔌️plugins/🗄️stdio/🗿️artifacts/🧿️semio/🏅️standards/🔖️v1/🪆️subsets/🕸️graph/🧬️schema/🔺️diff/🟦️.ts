/** 🔺️ SemioGraphDiff schema — keyed-row diff mirror of the Rust `🦀️.rs` sibling: per collection `removed` base indices, `modified` rows and `added` rows with their final index. */
export interface IndexModified<D> { index: number; diff: D }
export interface IndexAdded<T> { index: number; item: T }
export interface Replace<T> { value: T }
export interface IndexedTripleDiff<D, T> { removed?: number[]; modified?: IndexModified<D>[]; added?: IndexAdded<T>[] }
export interface SemioGraphEntryDiff { value?: import("../📸️snapshot/🟦️.ts").SemioValue | null }
export interface SemioGraphNodeDiff { id?: import("../📸️snapshot/🟦️.ts").GraphNodeId | null; kind?: string | null; label?: string | null; position?: import("../📸️snapshot/🟦️.ts").SemioPoint2 | null; width?: number | null; height?: number | null; ports?: IndexedTripleDiff<Replace<import("../📸️snapshot/🟦️.ts").SemioGraphPort>, import("../📸️snapshot/🟦️.ts").SemioGraphPort> | null; properties?: SemioGraphPropertiesDiff | null }
export interface SemioGraphEdgeDiff { source?: import("../📸️snapshot/🟦️.ts").GraphNodeId | null; target?: import("../📸️snapshot/🟦️.ts").GraphNodeId | null; kind?: string | null; label?: string | null; sourcePort?: string | null | null; targetPort?: string | null | null; properties?: SemioGraphPropertiesDiff | null }
export interface SemioGraphDiff { nodes?: IndexedTripleDiff<SemioGraphNodeDiff, import("../📸️snapshot/🟦️.ts").SemioGraphNode> | null; edges?: IndexedTripleDiff<SemioGraphEdgeDiff, import("../📸️snapshot/🟦️.ts").SemioGraphEdge> | null }
