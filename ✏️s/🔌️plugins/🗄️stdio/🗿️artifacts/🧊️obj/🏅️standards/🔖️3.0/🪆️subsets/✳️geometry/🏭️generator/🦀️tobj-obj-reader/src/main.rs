//! 🔬️ Standalone `tobj`-only reader/recipe crate for `s.stdio.obj@3.0/✳️geometry` — the wrapper the
//! `tobj-obj-3-0-mutate-reader` oracle (`../../🔣️oracle.json`) points at.
//!
//! Two responsibilities, both marshalling-only:
//!   * `build <recipe-id> <out-dir>` writes `<out-dir>/<recipe-id>/before.obj` [and `after.obj`],
//!     hand-authored grammar text — never this repository's own `encode_obj` — for exactly the
//!     kinds `tobj` (a MESH reader) can actually witness (see `RECIPE_IDS` below and the ticket
//!     report this crate ships alongside for the full witnessability argument). Every byte written
//!     is ADMITTED through the real `tobj` 4 first, mirroring the sibling `../🦀️engine`'s already
//!     -established precedent (OBJ has no reference WRITER in the Rust ecosystem).
//!   * `project <path>` decodes a real `.obj` file with `tobj::load_obj_buf` (`triangulate: true,
//!     single_index: true`, the same options the shared `mesh::project_obj` uses) and prints a typed
//!     JSON projection on stdout: one entry per `tobj::Model` (name + resolved position/texcoord/
//!     normal/triangle-index arrays). `../../🔬️probes/📜️script.ts` is the only caller; it computes
//!     no OBJ semantics itself, only hashes/diffs what this binary already decoded.
//!
//! Why 12 of the 22 declared kinds have a recipe here and 10 do not: `tobj::Mesh` drops every
//! `v`/`vt`/`vn` row no face resolves through triangulation/single-indexing, and its material loader
//! here is a no-op (`|_| Ok(Default::default())`, matching `mesh::project_obj`), so `insert-vertex`/
//! `remove-vertex`/`insert-texcoord`/`remove-texcoord`/`insert-normal`/`remove-normal` (unreferenced
//! by construction — a bare insert never creates a face) and `set-mtllib`/`set-usemtl`/
//! `set-smoothing-groups`/`set-unknown-statements` (comments, material names and smoothing-group
//! integers `tobj` never surfaces) are STRUCTURALLY invisible to a pure mesh reader, verified
//! directly against real `tobj` 4 output before this file was written (see the ticket report). Those
//! 10 kinds keep an `oracleRequirements` entry naming `obj-3-0-mutate-uncarried` in the registry
//! instead of this oracle — the exact convention `gltf@2.0/✳️geometry` already uses for its own
//! reader-blind kinds.
//!
//! The remaining 12 (`no-mutation`, `set-snapshot`, `set-vertex`, `set-texcoord`, `set-normal`,
//! `insert-face`, `remove-face`, `set-face`, `set-group`, `remove-group`, `set-object`,
//! `remove-object`) ARE witnessable — confirmed empirically: `set-vertex`/`set-texcoord`/
//! `set-normal` target a row a face actually references (a clean in-place value change, no index
//! shift involved); `insert-face`/`remove-face`/`set-face` change triangle count or topology
//! directly; `set-group`/`remove-group`/`set-object`/`remove-object` change which named `tobj::Model`
//! a face's triangle lands in — real `tobj` 4 splits models at `g`/`o` boundaries by name (confirmed:
//! a bare `g`/`o` reset line falls back to `tobj`'s own `"unnamed_object"`, and two same-named `g`
//! bands with no reset between them merge into one model), so a membership change is visible as a
//! model-name or model-count difference even though `tobj` never parses "membership" as a concept.

use std::env;
use std::fs;
use std::io::Cursor;
use std::path::Path;

//#region 🔖️Json
fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn json_num(v: f32) -> String {
    if v.is_finite() {
        format!("{v}")
    } else {
        "0".to_string()
    }
}

fn json_vec3_array(flat: &[f32]) -> String {
    let mut out = String::from("[");
    for (i, chunk) in flat.chunks_exact(3).enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&format!("[{},{},{}]", json_num(chunk[0]), json_num(chunk[1]), json_num(chunk[2])));
    }
    out.push(']');
    out
}

fn json_vec2_array(flat: &[f32]) -> String {
    let mut out = String::from("[");
    for (i, chunk) in flat.chunks_exact(2).enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&format!("[{},{}]", json_num(chunk[0]), json_num(chunk[1])));
    }
    out.push(']');
    out
}

fn json_triangles(indices: &[u32]) -> String {
    let mut out = String::from("[");
    for (i, chunk) in indices.chunks_exact(3).enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&format!("[{},{},{}]", chunk[0], chunk[1], chunk[2]));
    }
    out.push(']');
    out
}
//#endregion 🔖️Json

//#region 🔖️Project
/// 👁️ Decodes real bytes with the registered `tobj` 4 reader and emits one JSON object per
/// `tobj::Model`: name, resolved positions/texcoords/normals (post single-indexing — the same
/// per-corner-resolved shape `mesh::project_obj` reads) and triangle index triples. Nothing computed
/// here beyond straight field access on what `tobj` itself returned.
fn project(path: &str) -> i32 {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("[tobj-obj-reader] cannot read {path}: {e}");
            return 1;
        }
    };
    let mut cursor = Cursor::new(bytes);
    let loaded = tobj::load_obj_buf(&mut cursor, &tobj::LoadOptions { triangulate: true, single_index: true, ..Default::default() }, |_| Ok(Default::default()));
    let (models, _materials) = match loaded {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[tobj-obj-reader] tobj rejected {path}: {e}");
            return 1;
        }
    };
    let mut model_json: Vec<String> = Vec::new();
    let mut total_triangles = 0usize;
    for model in &models {
        let triangle_count = model.mesh.indices.len() / 3;
        total_triangles += triangle_count;
        model_json.push(format!(
            "{{\"name\":{},\"vertexCount\":{},\"triangleCount\":{},\"positions\":{},\"texcoords\":{},\"normals\":{},\"triangles\":{}}}",
            json_escape(&model.name),
            model.mesh.positions.len() / 3,
            triangle_count,
            json_vec3_array(&model.mesh.positions),
            json_vec2_array(&model.mesh.texcoords),
            json_vec3_array(&model.mesh.normals),
            json_triangles(&model.mesh.indices),
        ));
    }
    println!("{{\"modelCount\":{},\"totalTriangleCount\":{},\"models\":[{}]}}", models.len(), total_triangles, model_json.join(","));
    0
}
//#endregion 🔖️Project

//#region 🔖️Admit
/// 🔬️ The third-party admission gate every recipe byte must clear before being written — a fixture
/// the registered reader itself cannot parse is not evidence of anything (mirrors `../🦀️engine`'s
/// identical `admit` step for `pattern-shell.obj`).
fn admit_or_panic(label: &str, text: &str) {
    let mut cursor = Cursor::new(text.as_bytes().to_vec());
    let (_models, _materials) = tobj::load_obj_buf(&mut cursor, &tobj::LoadOptions { triangulate: true, single_index: true, ..Default::default() }, |_| Ok(Default::default())).unwrap_or_else(|error| panic!("tobj rejected recipe {label:?}'s own text — this is a bug in this crate, not a real mutation rejection: {error}"));
}
//#endregion 🔖️Admit

//#region 🔖️Recipes
/// 🍳️ One entry per WITNESSABLE kind (see the module header for why only these 12 of 22 are here).
/// `None` in the second slot means a `-rejected-` recipe: only `before.obj` is written, and the
/// notes name the exact out-of-range index / missing name a real subject would be handed alongside
/// it — this crate never invokes the real `ObjMutation`/`ObjDiff` dispatch, so it cannot execute the
/// rejection itself, only document which one the committed `before.obj` sets up.
#[allow(dead_code)]
struct Recipe {
    id: &'static str,
    before: &'static str,
    after: Option<&'static str>,
    notes: &'static str,
}

const BASE_PLAIN: &str = "\
v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\n\
vt 0 0\nvt 1 0\nvt 1 1\nvt 0 1\n\
vn 0 0 1\n\
f 1/1/1 2/2/1 3/3/1\nf 1/1/1 3/3/1 4/4/1\n";

const SET_VERTEX_AFTER: &str = "\
v 0 0 0\nv 1 0 5\nv 1 1 0\nv 0 1 0\n\
vt 0 0\nvt 1 0\nvt 1 1\nvt 0 1\n\
vn 0 0 1\n\
f 1/1/1 2/2/1 3/3/1\nf 1/1/1 3/3/1 4/4/1\n";

const SET_TEXCOORD_AFTER: &str = "\
v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\n\
vt 0 0\nvt 0.9 0.1\nvt 1 1\nvt 0 1\n\
vn 0 0 1\n\
f 1/1/1 2/2/1 3/3/1\nf 1/1/1 3/3/1 4/4/1\n";

const SET_NORMAL_AFTER: &str = "\
v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\n\
vt 0 0\nvt 1 0\nvt 1 1\nvt 0 1\n\
vn 0 0 -1\n\
f 1/1/1 2/2/1 3/3/1\nf 1/1/1 3/3/1 4/4/1\n";

const INSERT_FACE_AFTER: &str = "\
v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\n\
vt 0 0\nvt 1 0\nvt 1 1\nvt 0 1\n\
vn 0 0 1\n\
f 1/1/1 2/2/1 3/3/1\nf 2/2/1 3/3/1 4/4/1\nf 1/1/1 3/3/1 4/4/1\n";

const REMOVE_FACE_AFTER: &str = "\
v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\n\
vt 0 0\nvt 1 0\nvt 1 1\nvt 0 1\n\
vn 0 0 1\n\
f 1/1/1 3/3/1 4/4/1\n";

const SET_FACE_AFTER: &str = "\
v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\n\
vt 0 0\nvt 1 0\nvt 1 1\nvt 0 1\n\
vn 0 0 1\n\
f 3/3/1 2/2/1 1/1/1\nf 1/1/1 3/3/1 4/4/1\n";

const SET_SNAPSHOT_AFTER: &str = "\
v 0 0 0\nv 2 0 0\nv 0 2 0\n\
vt 0 0\nvt 1 0\nvt 0 1\n\
vn 0 0 1\n\
f 1/1/1 2/2/1 3/3/1\n";

const BASE_GROUPED_ONE: &str = "\
v 0 0 0\nv 1 0 0\nv 0 1 0\nv 2 2 2\nv 3 2 2\nv 2 3 2\n\
g alpha\nf 1 2 3\n\
g\nf 4 5 6\n";

const SET_GROUP_AFTER: &str = "\
v 0 0 0\nv 1 0 0\nv 0 1 0\nv 2 2 2\nv 3 2 2\nv 2 3 2\n\
g alpha\nf 1 2 3\nf 4 5 6\n";

const BASE_GROUPED_TWO: &str = "\
v 0 0 0\nv 1 0 0\nv 0 1 0\nv 2 2 2\nv 3 2 2\nv 2 3 2\n\
g alpha\nf 1 2 3\n\
g beta\nf 4 5 6\n";

const REMOVE_GROUP_AFTER: &str = "\
v 0 0 0\nv 1 0 0\nv 0 1 0\nv 2 2 2\nv 3 2 2\nv 2 3 2\n\
g alpha\nf 1 2 3\n\
g\nf 4 5 6\n";

const BASE_OBJECT_ONE: &str = "\
v 0 0 0\nv 1 0 0\nv 0 1 0\nv 2 2 2\nv 3 2 2\nv 2 3 2\n\
o alpha\nf 1 2 3\nf 4 5 6\n";

const SET_OBJECT_AFTER: &str = "\
v 0 0 0\nv 1 0 0\nv 0 1 0\nv 2 2 2\nv 3 2 2\nv 2 3 2\n\
o alpha\nf 1 2 3\n\
o beta\nf 4 5 6\n";

const BASE_OBJECT_TWO: &str = "\
v 0 0 0\nv 1 0 0\nv 0 1 0\nv 2 2 2\nv 3 2 2\nv 2 3 2\n\
o alpha\nf 1 2 3\n\
o beta\nf 4 5 6\n";

const REMOVE_OBJECT_AFTER: &str = "\
v 0 0 0\nv 1 0 0\nv 0 1 0\nv 2 2 2\nv 3 2 2\nv 2 3 2\n\
o alpha\nf 1 2 3\n\
o\nf 4 5 6\n";

const RECIPES: &[Recipe] = &[
    Recipe { id: "no-mutation-no-op", before: BASE_PLAIN, after: Some(BASE_PLAIN), notes: "Identity — the no-mutation scenario id applies nothing; before and after bytes are the same document, so the reader must witness zero difference." },
    Recipe { id: "set-snapshot-applied", before: BASE_PLAIN, after: Some(SET_SNAPSHOT_AFTER), notes: "SetSnapshot{snapshot:<a wholly different 1-triangle document>} — every declared row and the sole face differ; tobj sees a different vertexCount/triangleCount/positions." },
    Recipe { id: "set-vertex-applied", before: BASE_PLAIN, after: Some(SET_VERTEX_AFTER), notes: "SetVertex{index:1, vertex:{x:1,y:0,z:5}} on a vertex BOTH faces reference (in-place replace, no index shift) — tobj's resolved position for that corner moves from (1,0,0) to (1,0,5)." },
    Recipe { id: "set-vertex-rejected-out-of-bounds", before: BASE_PLAIN, after: None, notes: "SetVertex{index:9, ...} — base.vertices.len() is 4; validate_indexed_targets's invalid-modify-index rejects (🧬️schema/🔺️diff/🦀️.rs:843)." },
    Recipe { id: "set-texcoord-applied", before: BASE_PLAIN, after: Some(SET_TEXCOORD_AFTER), notes: "SetTexcoord{index:1, texcoord:{u:0.9,v:0.1}} on a texcoord a face references — tobj's per-corner texcoords array moves." },
    Recipe { id: "set-texcoord-rejected-out-of-bounds", before: BASE_PLAIN, after: None, notes: "SetTexcoord{index:9, ...} — base.texcoords.len() is 4; invalid-modify-index rejects." },
    Recipe { id: "set-normal-applied", before: BASE_PLAIN, after: Some(SET_NORMAL_AFTER), notes: "SetNormal{index:0, normal:{x:0,y:0,z:-1}} (the sole declared normal, referenced by both faces) — tobj's per-corner normals array flips." },
    Recipe { id: "set-normal-rejected-out-of-bounds", before: BASE_PLAIN, after: None, notes: "SetNormal{index:9, ...} — base.normals.len() is 1; invalid-modify-index rejects." },
    Recipe { id: "insert-face-applied", before: BASE_PLAIN, after: Some(INSERT_FACE_AFTER), notes: "InsertFace{index:1, face:(v2,v3,v4)} between the two declared faces — tobj's triangleCount goes 2 -> 3." },
    Recipe { id: "insert-face-rejected-out-of-bounds", before: BASE_PLAIN, after: None, notes: "InsertFace{index:9, ...} — base.faces.len() is 2 (evolving length 2); invalid-add-index rejects (index > length)." },
    Recipe { id: "remove-face-applied", before: BASE_PLAIN, after: Some(REMOVE_FACE_AFTER), notes: "RemoveFace{index:0} — tobj's triangleCount goes 2 -> 1, and the sole remaining triangle is what was face index 1." },
    Recipe { id: "remove-face-rejected-missing", before: BASE_PLAIN, after: None, notes: "RemoveFace{index:9} — base.faces.len() is 2; invalid-remove-index rejects." },
    Recipe { id: "set-face-applied", before: BASE_PLAIN, after: Some(SET_FACE_AFTER), notes: "SetFace{index:0, face:(v3,v2,v1)} — same 3 corners, reversed winding; tobj's triangles[0] index order changes even though vertexCount/triangleCount hold." },
    Recipe { id: "set-face-rejected-out-of-bounds", before: BASE_PLAIN, after: None, notes: "SetFace{index:9, ...} — base.faces.len() is 2; invalid-modify-index rejects." },
    Recipe { id: "set-group-applied", before: BASE_GROUPED_ONE, after: Some(SET_GROUP_AFTER), notes: "SetGroup{name:\"alpha\", faces:[0,1]} — face 1 was ungrouped (bare `g` reset renders as tobj's own \"unnamed_object\" fallback); after, both faces continuously fall under `g alpha` with no reset between them, so tobj merges what were 2 models into 1 named \"alpha\" with 2 triangles." },
    Recipe { id: "remove-group-applied", before: BASE_GROUPED_TWO, after: Some(REMOVE_GROUP_AFTER), notes: "RemoveGroup{name:\"beta\"} — face 1 falls back to no group (renders as a bare `g` reset); tobj's second model's NAME changes from \"beta\" to its own \"unnamed_object\" fallback." },
    Recipe { id: "remove-group-rejected-missing", before: BASE_GROUPED_TWO, after: None, notes: "RemoveGroup{name:\"gamma\"} — \"gamma\" is not in base.groups; validate_named_targets's invalid-remove-target rejects." },
    Recipe { id: "set-object-applied", before: BASE_OBJECT_ONE, after: Some(SET_OBJECT_AFTER), notes: "SetObject{name:\"beta\", faces:[1]} — face 1 was under \"alpha\" (single merged model, 2 triangles); \"beta\" is appended after \"alpha\" in the objects list, so faces_by_object's per-face overwrite lets beta win face 1, and tobj now reports 2 models, \"alpha\" (1 tri) and \"beta\" (1 tri)." },
    Recipe { id: "remove-object-applied", before: BASE_OBJECT_TWO, after: Some(REMOVE_OBJECT_AFTER), notes: "RemoveObject{name:\"beta\"} — face 1 falls back to no object (bare `o` reset); tobj's second model's NAME changes from \"beta\" to its own \"unnamed_object\" fallback." },
    Recipe { id: "remove-object-rejected-missing", before: BASE_OBJECT_TWO, after: None, notes: "RemoveObject{name:\"gamma\"} — \"gamma\" is not in base.objects; validate_named_targets's invalid-remove-target rejects." },
];

fn find_recipe(id: &str) -> Option<&'static Recipe> {
    RECIPES.iter().find(|r| r.id == id)
}
//#endregion 🔖️Recipes

//#region 🔖️Entry
fn cmd_build(id: &str, out_dir: &str) -> i32 {
    let Some(recipe) = find_recipe(id) else {
        eprintln!("[tobj-obj-reader] unknown recipe {id:?} — known: {}", RECIPES.iter().map(|r| r.id).collect::<Vec<_>>().join(", "));
        return 1;
    };
    admit_or_panic(&format!("{}/before", recipe.id), recipe.before);
    let dir = Path::new(out_dir).join(recipe.id);
    fs::create_dir_all(&dir).expect("create fixture recipe directory");
    fs::write(dir.join("before.obj"), recipe.before).expect("write before.obj");
    match recipe.after {
        Some(after) => {
            admit_or_panic(&format!("{}/after", recipe.id), after);
            fs::write(dir.join("after.obj"), after).expect("write after.obj");
            eprintln!("[tobj-obj-reader] {}: before.obj + after.obj -> {}", recipe.id, dir.display());
        }
        None => {
            eprintln!("[tobj-obj-reader] {}: before.obj only (rejected) -> {}", recipe.id, dir.display());
        }
    }
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = match args.get(1).map(String::as_str) {
        Some("build") => {
            let (Some(id), Some(out_dir)) = (args.get(2), args.get(3)) else {
                eprintln!("usage: tobj-obj-reader build <recipe-id> <out-dir>");
                std::process::exit(2);
            };
            cmd_build(id, out_dir)
        }
        Some("project") => {
            let Some(path) = args.get(2) else {
                eprintln!("usage: tobj-obj-reader project <path-to-obj>");
                std::process::exit(2);
            };
            project(path)
        }
        Some("list-recipes") => {
            for recipe in RECIPES {
                println!("{}", recipe.id);
            }
            0
        }
        _ => {
            eprintln!("usage: tobj-obj-reader build <recipe-id> <out-dir> | project <path-to-obj> | list-recipes");
            2
        }
    };
    std::process::exit(code);
}
//#endregion 🔖️Entry

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_recipe_id_resolves_and_before_admits() {
        for recipe in RECIPES {
            assert!(find_recipe(recipe.id).is_some());
            admit_or_panic(recipe.id, recipe.before);
            if let Some(after) = recipe.after {
                admit_or_panic(recipe.id, after);
            }
        }
    }
}
//#endregion 🧪️Tests
