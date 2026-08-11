/** 🧬️ SemioBrepArtifact schema — full artifact state, mirrors `SemioBrepSnapshot` field for
 * field (see the `🦀️component.rs` sibling for the real source of truth). */
export {
  BrepCurve, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex, SemioPoint3,
} from "./📸️snapshot/🟦️component.ts";
import { BrepEdge, BrepFace, BrepLoop, BrepShell, BrepSolid, BrepVertex } from "./📸️snapshot/🟦️component.ts";

export interface SemioBrepArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ vertices: BrepVertex[];
  /** @state persistent */ edges: BrepEdge[];
  /** @state persistent */ loops: BrepLoop[];
  /** @state persistent */ faces: BrepFace[];
  /** @state persistent */ shells: BrepShell[];
  /** @state persistent */ solids: BrepSolid[];
}
