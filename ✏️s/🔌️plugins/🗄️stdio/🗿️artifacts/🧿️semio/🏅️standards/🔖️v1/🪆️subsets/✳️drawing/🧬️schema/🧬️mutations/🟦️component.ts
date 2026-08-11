/** 🧬️ SemioDrawingMutation — mirrors the real Rust 🧬️mutations/🦀️component.rs (18 named
 * variants, tagged union on the `mutation` field, gif/svg precedent). `NodePath` addresses a
 * scene-graph node: `layer` selects `layers[layer]`, `path` is a chain of child indices from
 * that layer's root (`path == []` = the root itself). Variant fields target `canvas`/`styles`
 * (name-`key`ed `added`/`modified`/`removed`, base upsert)/`layers`/node `children`, and read
 * back the `base` snapshot's `schema`/`translation` (Transform) for `inverse()`; `line` is the
 * one-line `OpText` wire form; `item` names a triple's inserted payload. */
import type { DrawLayer, DrawNode, PathSegment, Rgba, SemioPoint2, SemioDrawingSnapshot, Transform } from "../📸️snapshot/🟦️component";

export interface NodePath {
  layer: number;
  path: number[];
}

export type SemioDrawingMutation =
  | { mutation: "noMutation" }
  | { mutation: "setSnapshot"; snapshot: SemioDrawingSnapshot }
  | { mutation: "setCanvasSize"; width: number; height: number }
  | { mutation: "setCanvasBackground"; background?: Rgba }
  | { mutation: "setStyle"; name: string; fill?: Rgba; stroke?: Rgba; strokeWidth?: number; opacity?: number }
  | { mutation: "removeStyle"; name: string }
  | { mutation: "insertLayer"; index: number; layer: DrawLayer }
  | { mutation: "removeLayer"; index: number }
  | { mutation: "setLayerMeta"; index: number; id: string; name: string; visible: boolean }
  | { mutation: "moveLayer"; from: number; to: number }
  | { mutation: "setGroupTransform"; path: NodePath; transform: Transform }
  | { mutation: "setPathSegments"; path: NodePath; segments: PathSegment[] }
  | { mutation: "setNodeStyle"; path: NodePath; style?: string }
  | { mutation: "setText"; path: NodePath; value: string; at: SemioPoint2 }
  | { mutation: "setImage"; path: NodePath; at: SemioPoint2; width: number; height: number; mime: string; bytes: Uint8Array }
  | { mutation: "insertNode"; path: NodePath; index: number; node: DrawNode }
  | { mutation: "removeNode"; path: NodePath; index: number }
  | { mutation: "replaceNode"; path: NodePath; node: DrawNode };
