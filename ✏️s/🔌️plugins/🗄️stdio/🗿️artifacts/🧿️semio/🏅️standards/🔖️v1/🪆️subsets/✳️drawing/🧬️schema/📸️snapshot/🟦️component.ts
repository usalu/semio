/** 🧬️ SemioDrawingSnapshot — mirrors the real Rust `📸️snapshot/🦀️component.rs` (source of truth).
 * `DrawNode` is the recursive scene-graph node (`Path`/`Text`/`Group`/`Image`), matching svg's
 * `SvgNodeDiff` recursive-diff template per the master plan. The `ArtifactDsl`/`ArtifactPack`
 * codec (see the Rust sibling) hex-encodes the JSON `body`, honoring `options` where the pack
 * encoder accepts them, and wraps `text` in the `semio_format` envelope. */

export interface SemioPoint2 { x: number; y: number; }
export interface Rgba { r: number; g: number; b: number; a: number; }
export interface Transform {
  translation: { x: number; y: number; z: number };
  rotation: { x: number; y: number; z: number; w: number };
  scale: { x: number; y: number; z: number };
}

export type PathSegment =
  | { kind: "moveTo"; to: SemioPoint2 }
  | { kind: "lineTo"; to: SemioPoint2 }
  | { kind: "cubicTo"; c1: SemioPoint2; c2: SemioPoint2; to: SemioPoint2 }
  | { kind: "quadTo"; c: SemioPoint2; to: SemioPoint2 }
  | { kind: "arcTo"; rx: number; ry: number; xRotation: number; largeArc: boolean; sweep: boolean; to: SemioPoint2 }
  | { kind: "close" };

export type DrawNode =
  | { kind: "path"; segments: PathSegment[]; style?: string }
  | { kind: "text"; value: string; at: SemioPoint2; style?: string }
  | { kind: "group-nodes"; transform: Transform; children: DrawNode[] }
  | { kind: "image"; at: SemioPoint2; width: number; height: number; mime: string; bytes: Uint8Array };

export interface DrawStyle {
  name: string;
  fill?: Rgba;
  stroke?: Rgba;
  strokeWidth?: number;
  opacity?: number;
}

export interface DrawLayer {
  id: string;
  name: string;
  visible: boolean;
  root: DrawNode;
}

export interface DrawCanvas {
  width: number;
  height: number;
  background?: Rgba;
}

export interface SemioDrawingSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ canvas: DrawCanvas;
  /** @state artifact */ styles: DrawStyle[];
  /** @state artifact */ layers: DrawLayer[];
}
