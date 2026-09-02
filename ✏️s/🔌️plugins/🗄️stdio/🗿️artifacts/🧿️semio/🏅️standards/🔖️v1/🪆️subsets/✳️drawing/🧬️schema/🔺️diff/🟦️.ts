/** 🔺️ SemioDrawingDiff — mirrors the real Rust 🔺️diff/🦀️.rs (handcrafted sparse diff,
 * source of truth). Collection triples reuse the shared engine's own TS shape (see
 * `⚙️engine/🧰️triples/🦀️.rs`'s facet mirrors) — `removed`/`modified`/`added`.
 * `between(base, other)` computes the `schema` delta against the base snapshot from scratch (no
 * snapshot-replace slot); `Transform` carries `translation`/`rotation`/`scale`; the hand-rolled
 * `DiffCodec`'s `line` is built from space-separated `tokens`; `NodePath{layer, path}` addresses
 * a node. */
import type { DrawLayer, DrawNode, DrawStyle, PathSegment, Rgba, SemioPoint2, Transform } from "../📸️snapshot/🟦️component";

export interface IndexedTripleDiff<D, T> {
  removed: number[];
  modified: { index: number; diff: D }[];
  added: { index: number; item: T }[];
}
export interface NamedTripleDiff<K, D, T> {
  removed: K[];
  modified: { key: K; diff: D }[];
  added: T[];
}

export interface DrawCanvasDiff {
  width?: number;
  height?: number;
  /** tri-state: absent = unchanged, null = cleared, value = set */
  background?: Rgba | null;
}

export interface DrawStyleDiff {
  fill?: Rgba | null;
  stroke?: Rgba | null;
  strokeWidth?: number | null;
  opacity?: number | null;
}

export type DrawNodeDiff =
  | { kind: "path"; segments?: PathSegment[]; style?: string | null }
  | { kind: "text"; value?: string; at?: SemioPoint2; style?: string | null }
  | { kind: "group-nodes"; transform?: Transform; children?: IndexedTripleDiff<DrawNodeDiff, DrawNode> }
  | { kind: "image"; at?: SemioPoint2; width?: number; height?: number; mime?: string; bytes?: Uint8Array }
  | { kind: "replace"; node: DrawNode };

export interface DrawLayerDiff {
  id?: string;
  name?: string;
  visible?: boolean;
  root?: DrawNodeDiff;
}

export interface SemioDrawingDiff {
  canvas?: DrawCanvasDiff;
  styles?: NamedTripleDiff<string, DrawStyleDiff, DrawStyle>;
  layers?: IndexedTripleDiff<DrawLayerDiff, DrawLayer>;
}

// 🧭️ Mutation-level node addressing (`NodePath`) lives in ../🧬️mutations/🟦️.ts.
