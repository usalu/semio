//! 📝️ Text representation marker for `stdio.semio.mesh.mutations` — the real grammar is
//! `impl protocol::OpText for SemioMeshMutation` (`../../🦀️component.rs`'s `OpCodecs` region):
//! either the literal `no-mutation`, or `keyword arg=value ...` (space-separated), one keyword
//! per mutation variant (kebab-case). See sibling `.g4`/`.ebnf`/`.grammar.semio` leaves for the
//! formal grammar.
pub const TEXT_MARKER: &str = "stdio.semio.mesh.mutations";
