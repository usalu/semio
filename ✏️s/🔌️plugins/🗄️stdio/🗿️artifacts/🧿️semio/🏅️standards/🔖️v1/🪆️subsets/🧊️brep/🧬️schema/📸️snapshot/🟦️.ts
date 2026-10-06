/** 🧬️ SemioBrepSnapshot facet mirror — the `🦀️.rs` sibling is the real source of truth
 * (matches repo convention); field names/shapes must stay in lock-step (POLICY_FACET_MIRROR_DRIFT). */

import type {SemioPoint3,SemioPoint2} from "../../../✉️base/🧬️schema/🧮️geometry/🟦️.ts";
export type {SemioPoint3,SemioPoint2} from "../../../✉️base/🧬️schema/🧮️geometry/🟦️.ts";
import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";

export type BrepCurve2 =
  | { kind: "line"; origin: SemioPoint2; direction: SemioPoint2 }
  | { kind: "circle"; center: SemioPoint2; radius: Binary64 }
  | { kind: "ellipse"; center: SemioPoint2; xAxis: SemioPoint2; radiusMajor: Binary64; radiusMinor: Binary64 }
  | { kind: "nurbs"; controlPoints: SemioPoint2[]; weights: Binary64[]; degree: number; knots: Binary64[] };

export type BrepCurve =
  | { kind: "line"; origin: SemioPoint3; direction: SemioPoint3 }
  | { kind: "circle"; center: SemioPoint3; axis: SemioPoint3; radius: Binary64 }
  | { kind: "ellipse"; center: SemioPoint3; axis: SemioPoint3; radiusMajor: Binary64; radiusMinor: Binary64 }
  | { kind: "nurbs"; controlPoints: SemioPoint3[]; weights: Binary64[]; degree: number; knots: Binary64[] };

export type BrepSurface =
  | { kind: "plane"; origin: SemioPoint3; normal: SemioPoint3 }
  | { kind: "cylinder"; origin: SemioPoint3; axis: SemioPoint3; radius: Binary64 }
  | { kind: "cone"; origin: SemioPoint3; axis: SemioPoint3; radius: Binary64; halfAngle: Binary64 }
  | { kind: "sphere"; center: SemioPoint3; radius: Binary64 }
  | { kind: "torus"; center: SemioPoint3; axis: SemioPoint3; majorRadius: Binary64; minorRadius: Binary64 }
  | {
      kind: "nurbs";
      controlPoints: SemioPoint3[];
      weights: Binary64[];
      uCount: number;
      vCount: number;
      degreeU: number;
      degreeV: number;
      knotsU: Binary64[];
      knotsV: Binary64[];
    };

export interface BrepVertex { id: string; point: SemioPoint3; tol: Binary64; }
export interface BrepEdge { id: string; startVertex: string; endVertex: string; curve: BrepCurve; tol: Binary64; }
export interface BrepLoopEdge { edge: string; orientation: boolean; }
export interface BrepLoop { id: string; edges: BrepLoopEdge[]; }
export interface BrepFace { id: string; outerLoop: string; innerLoops: string[]; surface: BrepSurface; orientation: boolean; tol: Binary64; }
export interface BrepShellFace { face: string; orientation: boolean; }
export interface BrepShell { id: string; faces: BrepShellFace[]; }
export interface BrepSolidShell { shell: string; isVoid: boolean; }
export interface BrepSolid { id: string; shells: BrepSolidShell[]; }
/** First-class coedge — see the `🦀️.rs` sibling's `BrepCoedge` doc comment for why this is a
 * separate collection rather than a widened `BrepLoopEdge`. */
export interface BrepCoedge { id: string; edge: string; forward: boolean; pcurve: BrepCurve2 | null; prange: [Binary64, Binary64]; loopId: string; next: string; prev: string; }

export interface SemioBrepSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ vertices: BrepVertex[];
  /** @state artifact */ edges: BrepEdge[];
  /** @state artifact */ loops: BrepLoop[];
  /** @state artifact */ faces: BrepFace[];
  /** @state artifact */ shells: BrepShell[];
  /** @state artifact */ solids: BrepSolid[];
  /** @state artifact */ coedges: BrepCoedge[];
  /** @state artifact */ nextLabel: bigint;
}
