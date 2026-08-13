/** 🧬️ SemioBrepArtifact schema — full artifact state, mirrors `SemioBrepSnapshot` field for
 * field (see the `🦀️component.rs` sibling for the real source of truth). */
export {
  BrepCurve, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex, SemioPoint3,
} from "./📸️snapshot/🟦️component.ts";
import { BrepEdge, BrepFace, BrepLoop, BrepShell, BrepSolid, BrepVertex } from "./📸️snapshot/🟦️component.ts";

export interface SemioBrepArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ vertices: BrepVertex[];
  /** @state artifact */ edges: BrepEdge[];
  /** @state artifact */ loops: BrepLoop[];
  /** @state artifact */ faces: BrepFace[];
  /** @state artifact */ shells: BrepShell[];
  /** @state artifact */ solids: BrepSolid[];
}
