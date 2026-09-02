/** 🔺️ `s.stdio.semio.mesh` DiffCodec text grammar — real hand-rolled grammar (see the
 * `impl protocol::DiffCodec for SemioMeshDiff` in the sibling `../🦀️.rs`): a
 * space-separated sequence of `key=value` tokens (`meshes=`/`materials=`/`textures=`), each
 * value a bracket-depth-aware `[removed];[modified];[added]` named triple. */
export interface SemioMeshDiffTextToken {
  key: "meshes" | "materials" | "textures";
  removed: string[];
  modified: string[]; // "key:diff" encoded
  added: string[];
}
export interface SemioMeshDiffText {
  tokens: SemioMeshDiffTextToken[];
}
