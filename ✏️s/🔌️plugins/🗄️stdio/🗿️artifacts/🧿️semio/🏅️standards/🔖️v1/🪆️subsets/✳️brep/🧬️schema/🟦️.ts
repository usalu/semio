/** 🧬️ SemioBrepArtifact schema — full artifact state, mirrors `SemioBrepSnapshot` field for
 * field (see the `🦀️.rs` sibling for the real source of truth). */
export {
  BrepCoedge, BrepCurve, BrepCurve2, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex, SemioPoint2, SemioPoint3,
} from "./📸️snapshot/🟦️.ts";
import { BrepCoedge, BrepEdge, BrepFace, BrepLoop, BrepShell, BrepSolid, BrepVertex } from "./📸️snapshot/🟦️.ts";

export interface SemioBrepArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ vertices: BrepVertex[];
  /** @state artifact */ edges: BrepEdge[];
  /** @state artifact */ loops: BrepLoop[];
  /** @state artifact */ faces: BrepFace[];
  /** @state artifact */ shells: BrepShell[];
  /** @state artifact */ solids: BrepSolid[];
  /** @state artifact */ coedges: BrepCoedge[];
  /** @state artifact */ nextLabel: number;
}
