/** 🔺️ SemioValueTreeDiff facet mirror — the `🦀️.rs` sibling is the real source of truth;
 * this interface tracks its fields 1:1 (see `POLICY_FACET_MIRROR_DRIFT`). */
import type { ValueId, SemioValueEntry, SemioValueNode, SemioValue } from "../📸️snapshot/🟦️component";

export interface IndexModified<D> { index: number; diff: D }
export interface IndexAdded<T> { index: number; item: T }
export interface IndexedTripleDiff<D, T> { removed: number[]; modified: IndexModified<D>[]; added: IndexAdded<T>[] }

export interface NamedModified<K, D> { key: K; diff: D }
export interface NamedTripleDiff<K, D, T> { removed: K[]; modified: NamedModified<K, D>[]; added: T[] }
/** 🧷 Position-carrying "added" wrapper for name/id-keyed collections (the shared engine's
 * `NamedTripleDiff<K,D,T>.added: T[]` alone carries no position — see the `🦀️.rs`
 * sibling's `NamedAdded<T>` doc comment). */
export interface NamedAdded<T> { index: number; item: T }

export type SemioValueDiff =
  | { kind: "replace"; value: SemioValue }
  | { kind: "bool"; value: boolean }
  | { kind: "int"; lexeme: string }
  | { kind: "float"; lexeme: string }
  | { kind: "str"; value: string }
  | { kind: "bytes"; value: number[] }
  | { kind: "list"; diff: IndexedTripleDiff<SemioValueDiff, SemioValue> }
  | { kind: "map"; diff: NamedTripleDiff<string, SemioValueDiff, NamedAdded<SemioValueEntry>> }
  | { kind: "ref"; id: ValueId };

export interface SemioValueTreeDiff {
  /** @state artifact */ root?: SemioValueDiff;
  /** @state artifact */ nodes?: NamedTripleDiff<ValueId, SemioValueDiff, NamedAdded<SemioValueNode>>;
}
