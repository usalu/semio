//! 📝️ Text representation marker for `stdio.semio.mesh.diff` — the real grammar is
//! `impl protocol::DiffCodec for SemioMeshDiff` (`../../🦀️component.rs`): space-separated
//! `key=value` tokens (`meshes=`/`materials=`/`textures=`), each value a bracket-depth-aware
//! `[removed];[modified];[added]` named triple built on `engine::triples::enc_named_triple`/
//! `dec_named_triple`. See sibling `.g4`/`.ebnf`/`.grammar.semio` leaves for the formal grammar.
pub const TEXT_MARKER: &str = "stdio.semio.mesh.diff";
