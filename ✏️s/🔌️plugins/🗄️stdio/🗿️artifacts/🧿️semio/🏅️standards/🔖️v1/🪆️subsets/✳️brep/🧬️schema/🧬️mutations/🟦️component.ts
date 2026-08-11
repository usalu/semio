/** 🧬️ SemioBrepMutation facet mirror — named-variant enum, discriminated on `mutation`. See the
 * `🦀️component.rs` sibling for the real source of truth. */
import { BrepCurve, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex, SemioBrepSnapshot, SemioPoint3 } from "../📸️snapshot/🟦️component.ts";

export type SemioBrepMutation =
  | { mutation: "noMutation" }
  | { mutation: "setSnapshot"; snapshot: SemioBrepSnapshot }
  | { mutation: "addVertex"; vertex: BrepVertex }
  | { mutation: "removeVertex"; id: string }
  | { mutation: "setVertexPoint"; id: string; point: SemioPoint3 }
  | { mutation: "addEdge"; edge: BrepEdge }
  | { mutation: "removeEdge"; id: string }
  | { mutation: "setEdgeEndpoints"; id: string; startVertex: string; endVertex: string }
  | { mutation: "setEdgeCurve"; id: string; curve: BrepCurve }
  | { mutation: "addLoop"; brepLoop: BrepLoop }
  | { mutation: "removeLoop"; id: string }
  | { mutation: "setLoopEdges"; id: string; edges: BrepLoopEdge[] }
  | { mutation: "addFace"; face: BrepFace }
  | { mutation: "removeFace"; id: string }
  | { mutation: "setFaceSurface"; id: string; surface: BrepSurface }
  | { mutation: "setFaceOrientation"; id: string; orientation: boolean }
  | { mutation: "setFaceLoops"; id: string; outerLoop: string; innerLoops: string[] }
  | { mutation: "addShell"; shell: BrepShell }
  | { mutation: "removeShell"; id: string }
  | { mutation: "setShellFaces"; id: string; faces: BrepShellFace[] }
  | { mutation: "addSolid"; solid: BrepSolid }
  | { mutation: "removeSolid"; id: string }
  | { mutation: "setSolidShells"; id: string; shells: BrepSolidShell[] };
