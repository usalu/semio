/** 🔺️ SemioBrepDiff facet mirror — handcrafted sparse diff, one `NamedTripleDiff` per collection
 * (never a full-replace `replacement` slot). See the `🦀️component.rs` sibling for the real source
 * of truth. */
import { BrepCurve, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex, SemioPoint3 } from "../📸️snapshot/🟦️component.ts";

export interface NamedModified<K, D> { key: K; diff: D; }
export interface NamedTripleDiff<D, T> { removed: string[]; modified: NamedModified<string, D>[]; added: T[]; }

export interface BrepVertexDiff { point?: SemioPoint3; }
export interface BrepEdgeDiff { startVertex?: string; endVertex?: string; curve?: BrepCurve; }
export interface BrepLoopDiff { edges?: BrepLoopEdge[]; }
export interface BrepFaceDiff { outerLoop?: string; innerLoops?: string[]; surface?: BrepSurface; orientation?: boolean; }
export interface BrepShellDiff { faces?: BrepShellFace[]; }
export interface BrepSolidDiff { shells?: BrepSolidShell[]; }

export interface SemioBrepDiff {
  vertices?: NamedTripleDiff<BrepVertexDiff, BrepVertex>;
  edges?: NamedTripleDiff<BrepEdgeDiff, BrepEdge>;
  loops?: NamedTripleDiff<BrepLoopDiff, BrepLoop>;
  faces?: NamedTripleDiff<BrepFaceDiff, BrepFace>;
  shells?: NamedTripleDiff<BrepShellDiff, BrepShell>;
  solids?: NamedTripleDiff<BrepSolidDiff, BrepSolid>;
}
