/** 🧬️ `s.stdio.semio.mesh` OpText grammar — real hand-rolled grammar (see
 * `impl protocol::OpText for SemioMeshMutation` in the sibling `../🦀️.rs`): either the
 * literal `"no-mutation"`, or `"keyword arg=value ..."` (space-separated), one keyword per
 * `SemioMeshMutation` variant (kebab-case). */
export interface SemioMeshMutationTextArg { name: string; value: string; }
export interface SemioMeshMutationText {
  keyword: string;
  args: SemioMeshMutationTextArg[];
}
