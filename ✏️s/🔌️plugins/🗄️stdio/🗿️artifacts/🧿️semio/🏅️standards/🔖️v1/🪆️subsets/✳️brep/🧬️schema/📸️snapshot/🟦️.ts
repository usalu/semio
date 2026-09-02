/** 🧬️ SemioBrepSnapshot facet mirror — the `🦀️.rs` sibling is the real source of truth
 * (matches repo convention); field names/shapes must stay in lock-step (POLICY_FACET_MIRROR_DRIFT). */

export interface SemioPoint3 { x: number; y: number; z: number; }

export type BrepCurve =
  | { kind: "line"; origin: SemioPoint3; direction: SemioPoint3 }
  | { kind: "circle"; center: SemioPoint3; axis: SemioPoint3; radius: number }
  | { kind: "ellipse"; center: SemioPoint3; axis: SemioPoint3; radiusMajor: number; radiusMinor: number }
  | { kind: "nurbs"; controlPoints: SemioPoint3[]; weights: number[]; degree: number; knots: number[] };

export type BrepSurface =
  | { kind: "plane"; origin: SemioPoint3; normal: SemioPoint3 }
  | { kind: "cylinder"; origin: SemioPoint3; axis: SemioPoint3; radius: number }
  | { kind: "cone"; origin: SemioPoint3; axis: SemioPoint3; radius: number; halfAngle: number }
  | { kind: "sphere"; center: SemioPoint3; radius: number }
  | { kind: "torus"; center: SemioPoint3; axis: SemioPoint3; majorRadius: number; minorRadius: number }
  | {
      kind: "nurbs";
      controlPoints: SemioPoint3[];
      weights: number[];
      uCount: number;
      vCount: number;
      degreeU: number;
      degreeV: number;
      knotsU: number[];
      knotsV: number[];
    };

export interface BrepVertex { id: string; point: SemioPoint3; }
export interface BrepEdge { id: string; startVertex: string; endVertex: string; curve: BrepCurve; }
export interface BrepLoopEdge { edge: string; orientation: boolean; }
export interface BrepLoop { id: string; edges: BrepLoopEdge[]; }
export interface BrepFace { id: string; outerLoop: string; innerLoops: string[]; surface: BrepSurface; orientation: boolean; }
export interface BrepShellFace { face: string; orientation: boolean; }
export interface BrepShell { id: string; faces: BrepShellFace[]; }
export interface BrepSolidShell { shell: string; isVoid: boolean; }
export interface BrepSolid { id: string; shells: BrepSolidShell[]; }

export interface SemioBrepSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ vertices: BrepVertex[];
  /** @state artifact */ edges: BrepEdge[];
  /** @state artifact */ loops: BrepLoop[];
  /** @state artifact */ faces: BrepFace[];
  /** @state artifact */ shells: BrepShell[];
  /** @state artifact */ solids: BrepSolid[];
}
