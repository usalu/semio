//! 🏭️ Generates `pattern-shell.obj`, one small, deliberately varied, fully deterministic Wavefront
//! OBJ document that carries every statement this subset's 22-kind `ObjMutation` vocabulary can
//! move — and ADMITS it through the real `tobj` 4 reader, the same crate registered as
//! `tobj-obj-3-0-mutate` in `../../🔣️oracle.json`, before writing a single byte.
//!
//! 📐️ Why the grammar is written here rather than by a library: OBJ has no reference WRITER in the
//! Rust ecosystem — `tobj` parses and never emits — which is the already-recorded reason this
//! subset's own oracle (`../../🦀️oracle.rs`) and the shared `mesh::oracle_create_obj`
//! both write the grammar directly and use `tobj` as the independent reader. This generator mirrors
//! that same precedent exactly: it never touches this repository's `encode_obj`/`ObjSnapshot`, and
//! the third-party crate is what decides whether the produced document is a real OBJ at all
//! (`load_obj_buf`, triangulating and single-indexing, exactly as `mesh::project_obj` does) — a
//! generator whose output the registered reader cannot parse fails here rather than downstream.
//!
//! 🎯️ What the document deliberately carries, statement by statement, so no declared kind has to be
//! exercised against a document that cannot express it:
//!   * 6 `v` rows, one of them with an explicit `w`, and one (`v6`) NO FACE REFERENCES — `tobj`
//!     re-indexes per model and drops every unreferenced row, so `insert-vertex`/`remove-vertex`/
//!     `set-vertex` on it move the DOCUMENT projection and nothing in the mesh projection. That
//!     asymmetry is the whole reason the comparison composes two readings.
//!   * 6 `vt` and 4 `vn` rows, likewise with one unreferenced each.
//!   * 5 `f` rows across 2 `g` bands and 2 `o` objects, so removing an interior face shifts a
//!     membership span that another entry sits after.
//!   * `mtllib`, two `usemtl` runs and two `s` runs (one of them `s off`), each starting at a
//!     DIFFERENT face index, so the run-start lists are lists rather than singletons.
//!   * 2 retained comment lines, which `set-unknown-statements` replaces wholesale.
//!
//! No wall-clock, no randomness, no counters: byte-for-byte identical on every run, which is what
//! `test fixture reproduce` checks per fixture.
//!
//! Usage: `generate <output.obj>`.

use std::env;

/// 🧊️ The document itself, written as the literal grammar so the bytes are the review surface
/// rather than the output of a formatter whose decimal rendering could drift between toolchains.
const PATTERN_SHELL: &str = "\
# semio obj@3.0/✳️any fixture — pattern-shell
# v6/vt6/vn4 are declared and referenced by no face: only the document projection sees them
mtllib pattern-shell.mtl
v 0 0 0
v 1 0 0
v 1 1 0
v 0 1 0
v 0.5 0.5 1
v 2 2 2 0.5
vt 0 0
vt 1 0
vt 1 1
vt 0 1
vt 0.25 0.75
vt 0.9 0.1
vn 0 0 1
vn 0 0 -1
vn 1 0 0
vn 0 1 0
o shell
g base
usemtl deck
s 1
f 1/1/1 2/2/1 3/3/1
f 1/1/1 3/3/1 4/4/1
g cap
usemtl roof
s off
f 1/1/2 2/2/2 5/5/2
f 2/2/3 3/3/3 5/5/3
o trim
g cap
f 3/3/3 4/4/3 5/5/3
";

fn main() {
    let out_path = env::args().nth(1).expect("usage: generate <output.obj>");
    let bytes = PATTERN_SHELL.as_bytes();
    let (models, vertices, indices) = admit(bytes);
    eprintln!("tobj 4 admitted the document: {models} model(s), {vertices} referenced vertex position(s), {indices} triangulated index/indices");
    std::fs::write(&out_path, bytes).unwrap_or_else(|error| panic!("writing {out_path}: {error}"));
    eprintln!("wrote {} bytes to {out_path}", bytes.len());
}

/// 🔬️ The third-party admission step: the registered `tobj` reader parses the produced bytes with
/// the SAME options the oracle's mesh projection uses. A parse failure aborts generation — a
/// fixture the registered reader rejects is not evidence of anything.
fn admit(bytes: &[u8]) -> (usize, usize, usize) {
    let mut cursor = std::io::Cursor::new(bytes.to_vec());
    let (models, _) = tobj::load_obj_buf(&mut cursor, &tobj::LoadOptions { triangulate: true, single_index: true, ..Default::default() }, |_| Ok(Default::default())).unwrap_or_else(|error| panic!("tobj rejected the generated OBJ: {error}"));
    let vertices = models.iter().map(|model| model.mesh.positions.len() / 3).sum();
    let indices = models.iter().map(|model| model.mesh.indices.len()).sum();
    (models.len(), vertices, indices)
}
