//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered reference implementation so the subject's own mutation has an independent result to
//! be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared `mesh` module rather than by copying it. STL's OWN triangle-soup shape (no vertex
//! index, an explicit per-facet normal never recomputed from winding) is already exercised by the
//! shared `mesh::{oracle_create_stl, project_stl}` pair for creation/projection; mutation is
//! genuinely per-subset (`stl-ascii-mutate`, not the shared `stl-mesh` capability), so it lives here.
//!
//! **Standard mismatch, resolved**: this subset's OWN codec (`../🚪️io/🦀️.rs`'s
//! `decode_stl_ascii`/`encode_stl_ascii`) is genuinely ASCII text — matching the `ascii` standard
//! this subset is filed under — and `stl_io` 0.8 READS that form perfectly well, which is the half
//! that matters for an independent reader. Its WRITER cannot serve: its own top-level doc comment
//! states "Writing is limited to binary STL", confirmed in its source (`writer.rs` hardcodes a
//! zeroed 80-byte header with no name field at all; `read_stl`'s `IndexedMesh` carries no name
//! either). Routing output through it would therefore discard the solid name on every kind — the
//! very field `set-solid-name` exists to change — and would leave this `ascii` subset's oracle never
//! once emitting the ascii grammar. So `stl_io` is the READER for every arm here and the ascii
//! grammar is written by this module directly (`mod ascii`), the same precedent the OBJ subset's
//! oracle follows for a format whose Rust reference is a reader. Nothing here touches the subject's
//! own codec, so it stays a genuine second producer rather than a self-comparison.
//!
//! @see 🔣️.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

//#region 🔖️TriangleSoup
/// 🧊️ Independent triangle-soup reading behind `stl_io` — the reader half every dispatch arm below
/// starts from, including `no-mutation` (see this file's top doc comment for why the writer half
/// cannot come from the same crate).
#[cfg(feature = "oracles")]
mod triangle_soup {
    #[derive(Clone, Copy)]
    pub(super) struct RefTriangle {
        pub normal: [f32; 3],
        pub vertices: [[f32; 3]; 3],
    }

    /// 📥️ Independent read: resolves `stl_io`'s deduplicated `IndexedMesh` back into one triangle
    /// per original facet, in original facet order.
    pub(super) fn read(input: &[u8]) -> Result<Vec<RefTriangle>, String> {
        let mut cursor = std::io::Cursor::new(input.to_vec());
        let mesh = stl_io::read_stl(&mut cursor).map_err(|error| format!("independent reader could not parse the STL: {error}"))?;
        Ok(mesh.faces.iter().map(|face| RefTriangle { normal: face.normal.0, vertices: [mesh.vertices[face.vertices[0]].0, mesh.vertices[face.vertices[1]].0, mesh.vertices[face.vertices[2]].0] }).collect())
    }

}
//#endregion 🔖️TriangleSoup

//#region 🔖️Ascii
/// 🔤️ The ASCII STL grammar — this subset's own standard, and the form every arm below emits.
///
/// `stl_io` READS ascii perfectly well; its writer is the half that cannot serve here. That writer
/// emits binary only (its own doc comment says so, and `writer.rs` hardcodes a zeroed 80-byte
/// header), and binary STL has no solid-name field at all, so routing the output through it would
/// throw away the very thing `set-solid-name` exists to change and would leave this `ascii` subset's
/// oracle never once exercising the ascii grammar. Writing the grammar directly instead is the same
/// precedent the OBJ subset's oracle already follows for a format whose Rust reference is a reader:
/// the second producer is this module, the independent READER every result is projected through is
/// still `stl_io`, and nothing here touches the subject's `decode_stl_ascii`/`encode_stl_ascii`.
#[cfg(feature = "oracles")]
mod ascii {
    use super::triangle_soup::RefTriangle;

    fn text(input: &[u8]) -> Result<String, String> {
        String::from_utf8(input.to_vec()).map_err(|error| format!("ascii STL expected: {error}"))
    }

    /// 🏷️ The solid name off the `solid <name>` header — the one field `stl_io`'s `IndexedMesh` has
    /// no slot for, so it is read out of the grammar directly.
    pub(super) fn read_name(input: &[u8]) -> Result<String, String> {
        let source = text(input)?;
        let header = source.lines().next().ok_or_else(|| "stl ascii: empty document".to_string())?;
        if !header.trim_start().starts_with("solid") {
            return Err("stl ascii: missing 'solid' header".to_string());
        }
        Ok(header.trim().strip_prefix("solid").unwrap_or("").trim().to_string())
    }

    /// 📤️ Independent write of the whole ascii document from the model alone: header, one
    /// `facet normal`/`outer loop`/`vertex`×3 block per triangle in order, trailer.
    pub(super) fn write(name: &str, triangles: &[RefTriangle]) -> Result<Vec<u8>, String> {
        let mut out = format!("solid {name}\n");
        for triangle in triangles {
            out.push_str(&format!("  facet normal {} {} {}\n    outer loop\n", triangle.normal[0], triangle.normal[1], triangle.normal[2]));
            for vertex in &triangle.vertices {
                out.push_str(&format!("      vertex {} {} {}\n", vertex[0], vertex[1], vertex[2]));
            }
            out.push_str("    endloop\n  endfacet\n");
        }
        out.push_str(&format!("endsolid {name}\n"));
        Ok(out.into_bytes())
    }
}
//#endregion 🔖️Ascii

//#region 🔖️SpecReaders
#[cfg(feature = "oracles")]
fn mutation_params(spec: &Json) -> Json {
    spec.get("params").cloned().unwrap_or(Json::Null)
}
#[cfg(feature = "oracles")]
fn number(value: &Json, key: &str) -> Option<f64> {
    match value.get(key) {
        Some(Json::Number(number)) => Some(*number),
        _ => None,
    }
}
#[cfg(feature = "oracles")]
fn string(value: &Json, key: &str) -> Option<String> {
    match value.get(key) {
        Some(Json::String(text)) => Some(text.clone()),
        _ => None,
    }
}
#[cfg(feature = "oracles")]
fn json_vec3(value: &Json) -> Option<[f32; 3]> {
    match value {
        Json::Array(items) if items.len() == 3 => {
            let mut out = [0f32; 3];
            for (slot, item) in out.iter_mut().zip(items.iter()) {
                *slot = match item {
                    Json::Number(number) => *number as f32,
                    _ => return None,
                };
            }
            Some(out)
        }
        _ => None,
    }
}
#[cfg(feature = "oracles")]
fn vec3(value: &Json, key: &str) -> Option<[f32; 3]> {
    json_vec3(value.get(key)?)
}
#[cfg(feature = "oracles")]
fn vertices3(value: &Json, key: &str) -> Option<[[f32; 3]; 3]> {
    match value.get(key) {
        Some(Json::Array(items)) if items.len() == 3 => Some([json_vec3(&items[0])?, json_vec3(&items[1])?, json_vec3(&items[2])?]),
        _ => None,
    }
}
#[cfg(feature = "oracles")]
fn triangle_of(value: &Json) -> Option<triangle_soup::RefTriangle> {
    Some(triangle_soup::RefTriangle { normal: vec3(value, "normal")?, vertices: vertices3(value, "vertices")? })
}
#[cfg(feature = "oracles")]
fn triangle_json(triangle: &triangle_soup::RefTriangle) -> Json {
    Json::Object(vec![
        ("normal".to_string(), Json::Array(triangle.normal.iter().map(|value| Json::Number(*value as f64)).collect())),
        ("vertices".to_string(), Json::Array(triangle.vertices.iter().map(|vertex| Json::Array(vertex.iter().map(|value| Json::Number(*value as f64)).collect())).collect())),
    ])
}
#[cfg(feature = "oracles")]
fn triangles_of(value: &Json, key: &str) -> Option<Vec<triangle_soup::RefTriangle>> {
    match value.get(key) {
        Some(Json::Array(items)) => items.iter().map(triangle_of).collect(),
        _ => None,
    }
}
#[cfg(feature = "oracles")]
fn spec_of(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}
//#endregion 🔖️SpecReaders

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let params = mutation_params(spec);
    match spec.str("kind").as_str() {
        "" => Err("mutation spec carries no `kind`".to_string()),
        "no-mutation" => ascii::write(&ascii::read_name(input)?, &triangle_soup::read(input)?),
        "set-solid-name" => ascii::write(&string(&params, "name").ok_or("set-solid-name: missing `name`")?, &triangle_soup::read(input)?),
        "insert-triangle" => {
            let mut triangles = triangle_soup::read(input)?;
            let index = number(&params, "index").ok_or("insert-triangle: missing `index`")? as usize;
            let triangle = triangle_of(params.get("triangle").ok_or("insert-triangle: missing `triangle`")?).ok_or("insert-triangle: malformed `triangle`")?;
            triangles.insert(index.min(triangles.len()), triangle);
            ascii::write(&ascii::read_name(input)?, &triangles)
        }
        "remove-triangle" => {
            let mut triangles = triangle_soup::read(input)?;
            let index = number(&params, "index").ok_or("remove-triangle: missing `index`")? as usize;
            if index < triangles.len() {
                triangles.remove(index);
            }
            ascii::write(&ascii::read_name(input)?, &triangles)
        }
        "set-triangle-normal" => {
            let mut triangles = triangle_soup::read(input)?;
            let index = number(&params, "index").ok_or("set-triangle-normal: missing `index`")? as usize;
            let normal = vec3(&params, "normal").ok_or("set-triangle-normal: missing `normal`")?;
            if let Some(triangle) = triangles.get_mut(index) {
                triangle.normal = normal;
            }
            ascii::write(&ascii::read_name(input)?, &triangles)
        }
        "set-triangle-vertices" => {
            let mut triangles = triangle_soup::read(input)?;
            let index = number(&params, "index").ok_or("set-triangle-vertices: missing `index`")? as usize;
            let vertices = vertices3(&params, "vertices").ok_or("set-triangle-vertices: missing `vertices`")?;
            if let Some(triangle) = triangles.get_mut(index) {
                triangle.vertices = vertices;
            }
            ascii::write(&ascii::read_name(input)?, &triangles)
        }
        "set-snapshot" => ascii::write(&ascii::read_name(input)?, &triangles_of(&params, "triangles").ok_or("set-snapshot: missing/malformed `triangles`")?),
        kind => Err(format!("mutation kind {kind:?} has no oracle implementation ({} input byte(s))", input.len())),
    }
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️Inverse
/// ↩️ The spec that undoes `spec` when applied AFTER `oracle_apply_mutation(base, spec)`'s own
/// result — index-aware and computed from `base` (the pre-mutation document), mirroring
/// `StlMutation::inverse()` (`../🧬️schema/🧬️mutations/🦀️.rs`) independently: an
/// out-of-range index that the forward mutation would have rejected inverts to `no-mutation`,
/// exactly as that hand-rolled method does.
#[cfg(feature = "oracles")]
pub fn oracle_inverse_spec(base: &[u8], spec: &Json) -> Result<Json, String> {
    let params = mutation_params(spec);
    Ok(match spec.str("kind").as_str() {
        "" => return Err("mutation spec carries no `kind`".to_string()),
        "no-mutation" => spec_of("no-mutation", Json::Object(vec![])),
        "set-solid-name" => spec_of("set-solid-name", Json::Object(vec![("name".to_string(), Json::String(ascii::read_name(base)?))])),
        "insert-triangle" => {
            let triangles = triangle_soup::read(base)?;
            let index = number(&params, "index").ok_or("insert-triangle: missing `index`")? as usize;
            spec_of("remove-triangle", Json::Object(vec![("index".to_string(), Json::Number(index.min(triangles.len()) as f64))]))
        }
        "remove-triangle" => {
            let triangles = triangle_soup::read(base)?;
            let index = number(&params, "index").ok_or("remove-triangle: missing `index`")? as usize;
            match triangles.get(index) {
                Some(triangle) => spec_of("insert-triangle", Json::Object(vec![("index".to_string(), Json::Number(index as f64)), ("triangle".to_string(), triangle_json(triangle))])),
                None => spec_of("no-mutation", Json::Object(vec![])),
            }
        }
        "set-triangle-normal" => {
            let triangles = triangle_soup::read(base)?;
            let index = number(&params, "index").ok_or("set-triangle-normal: missing `index`")? as usize;
            match triangles.get(index) {
                Some(triangle) => spec_of("set-triangle-normal", Json::Object(vec![("index".to_string(), Json::Number(index as f64)), ("normal".to_string(), Json::Array(triangle.normal.iter().map(|value| Json::Number(*value as f64)).collect()))])),
                None => spec_of("no-mutation", Json::Object(vec![])),
            }
        }
        "set-triangle-vertices" => {
            let triangles = triangle_soup::read(base)?;
            let index = number(&params, "index").ok_or("set-triangle-vertices: missing `index`")? as usize;
            match triangles.get(index) {
                Some(triangle) => spec_of(
                    "set-triangle-vertices",
                    Json::Object(vec![
                        ("index".to_string(), Json::Number(index as f64)),
                        ("vertices".to_string(), Json::Array(triangle.vertices.iter().map(|vertex| Json::Array(vertex.iter().map(|value| Json::Number(*value as f64)).collect())).collect())),
                    ]),
                ),
                None => spec_of("no-mutation", Json::Object(vec![])),
            }
        }
        "set-snapshot" => {
            let triangles = triangle_soup::read(base)?;
            spec_of("set-snapshot", Json::Object(vec![("triangles".to_string(), Json::Array(triangles.iter().map(triangle_json).collect()))]))
        }
        kind => return Err(format!("mutation kind {kind:?} has no oracle implementation ({} base byte(s))", base.len())),
    })
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_inverse_spec(_base: &[u8], _spec: &Json) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Inverse

//#region 🔖️DocumentProjection
/// 📐️ The two fields a triangle-soup projection cannot carry, and without which two of this
/// subset's seven declared kinds move nothing at all: the `solid <name>` header, read out of the
/// ascii grammar, and the EXPLICIT per-facet normal, read back through `stl_io` rather than
/// recomputed from winding. STL states its normals rather than deriving them, so `set-triangle-
/// normal` is a real change to the document even when every corner stays exactly where it was —
/// the shared mesh projection reports resolved corners only and cannot see it.
#[cfg(feature = "oracles")]
pub fn oracle_document_projection(input: &[u8]) -> Result<Json, String> {
    let triangles = triangle_soup::read(input)?;
    Ok(Json::Object(vec![
        ("solidName".to_string(), Json::String(ascii::read_name(input)?)),
        ("facetCount".to_string(), Json::Number(triangles.len() as f64)),
        ("facetNormals".to_string(), Json::Array(triangles.iter().map(|triangle| Json::Array(triangle.normal.iter().map(|value| Json::Number(*value as f64)).collect())).collect())),
    ]))
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_document_projection(_input: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️DocumentProjection

//#region 🔖️RoundTrip
/// 🔁️ The identity-round-trip scenario's own producer: `stl_io` parses the real ascii document into
/// its `IndexedMesh` and this module re-emits the whole grammar from that model alone. It is exactly
/// what `no-mutation` now does — that arm used to hand the input bytes straight back, which proves
/// nothing about either half — and it cannot coincidentally reproduce the input, because `stl_io`
/// resolves every coordinate through `f32` while the committed fixture carries `f64` decimals.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    ascii::write(&ascii::read_name(input)?, &triangle_soup::read(input)?)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️RoundTrip

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::*;

    const FIXTURE: &str = "solid box\n  facet normal 0 0 -1\n    outer loop\n      vertex 0 0 0\n      vertex 0 1 0\n      vertex 1 0 0\n    endloop\n  endfacet\n  facet normal 0 0 1\n    outer loop\n      vertex 0 0 1\n      vertex 1 0 1\n      vertex 0 1 1\n    endloop\n  endfacet\nendsolid box\n";

    fn spec(kind: &str, params: Json) -> Json {
        spec_of(kind, params)
    }

    #[test]
    fn no_mutation_re_emits_the_document_rather_than_handing_the_bytes_back() {
        let output = oracle_apply_mutation(FIXTURE.as_bytes(), &spec("no-mutation", Json::Object(vec![]))).unwrap();
        assert_eq!(ascii::read_name(&output).unwrap(), "box", "the solid name survives");
        assert_eq!(triangle_soup::read(&output).unwrap().len(), 2, "and so does every facet");
        assert_eq!(String::from_utf8(output).unwrap(), "solid box\n  facet normal 0 0 -1\n    outer loop\n      vertex 0 0 0\n      vertex 0 1 0\n      vertex 1 0 0\n    endloop\n  endfacet\n  facet normal 0 0 1\n    outer loop\n      vertex 0 0 1\n      vertex 1 0 1\n      vertex 0 1 1\n    endloop\n  endfacet\nendsolid box\n", "re-emitted from the parsed model, not copied");
    }

    #[test]
    fn every_kind_emits_ascii_that_keeps_the_solid_name() {
        for (kind, params) in [
            ("no-mutation", Json::Object(vec![])),
            ("remove-triangle", Json::Object(vec![("index".to_string(), Json::Number(0.0))])),
            ("set-triangle-normal", Json::Object(vec![("index".to_string(), Json::Number(0.0)), ("normal".to_string(), Json::Array(vec![Json::Number(0.0), Json::Number(1.0), Json::Number(0.0)]))])),
        ] {
            let output = oracle_apply_mutation(FIXTURE.as_bytes(), &spec(kind, params)).unwrap();
            assert_eq!(ascii::read_name(&output).unwrap(), "box", "{kind} must not lose the solid name the way a binary re-encode would");
        }
    }

    #[test]
    fn the_projection_sees_the_two_fields_a_triangle_soup_reader_cannot() {
        let renamed = oracle_apply_mutation(FIXTURE.as_bytes(), &spec("set-solid-name", Json::Object(vec![("name".to_string(), Json::String("renamed".to_string()))]))).unwrap();
        let turned = oracle_apply_mutation(
            FIXTURE.as_bytes(),
            &spec("set-triangle-normal", Json::Object(vec![("index".to_string(), Json::Number(0.0)), ("normal".to_string(), Json::Array(vec![Json::Number(0.0), Json::Number(1.0), Json::Number(0.0)]))])),
        )
        .unwrap();
        let base = oracle_document_projection(FIXTURE.as_bytes()).unwrap();
        assert_ne!(oracle_document_projection(&renamed).unwrap(), base, "set-solid-name has to be visible somewhere");
        assert_ne!(oracle_document_projection(&turned).unwrap(), base, "and so does set-triangle-normal");
    }

    #[test]
    fn set_solid_name_rewrites_only_the_header_and_trailer() {
        let output = oracle_apply_mutation(FIXTURE.as_bytes(), &spec("set-solid-name", Json::Object(vec![("name".to_string(), Json::String("renamed".to_string()))]))).unwrap();
        let text = String::from_utf8(output).unwrap();
        assert!(text.starts_with("solid renamed\n"));
        assert!(text.trim_end().ends_with("endsolid renamed"));
        assert_eq!(text.matches("facet normal").count(), 2);
    }

    #[test]
    fn insert_and_remove_triangle_are_inverse_on_a_real_shaped_mesh() {
        let triangle = Json::Object(vec![
            ("normal".to_string(), Json::Array(vec![Json::Number(1.0), Json::Number(0.0), Json::Number(0.0)])),
            (
                "vertices".to_string(),
                Json::Array(vec![
                    Json::Array(vec![Json::Number(9.0), Json::Number(0.0), Json::Number(0.0)]),
                    Json::Array(vec![Json::Number(10.0), Json::Number(0.0), Json::Number(0.0)]),
                    Json::Array(vec![Json::Number(9.0), Json::Number(1.0), Json::Number(0.0)]),
                ]),
            ),
        ]);
        let insert_spec = spec("insert-triangle", Json::Object(vec![("index".to_string(), Json::Number(1.0)), ("triangle".to_string(), triangle)]));
        let inserted = oracle_apply_mutation(FIXTURE.as_bytes(), &insert_spec).unwrap();
        assert_eq!(triangle_soup::read(&inserted).unwrap().len(), 3);

        let inverse = oracle_inverse_spec(FIXTURE.as_bytes(), &insert_spec).unwrap();
        assert_eq!(inverse.str("kind"), "remove-triangle");
        let restored = oracle_apply_mutation(&inserted, &inverse).unwrap();
        let before = triangle_soup::read(FIXTURE.as_bytes()).unwrap();
        let after = triangle_soup::read(&restored).unwrap();
        assert_eq!(before.len(), after.len());
        for (a, b) in before.iter().zip(after.iter()) {
            assert_eq!(a.normal, b.normal);
            assert_eq!(a.vertices, b.vertices);
        }
    }

    #[test]
    fn remove_triangle_inverse_reinserts_the_original_triangle() {
        let remove_spec = spec("remove-triangle", Json::Object(vec![("index".to_string(), Json::Number(0.0))]));
        let removed = oracle_apply_mutation(FIXTURE.as_bytes(), &remove_spec).unwrap();
        assert_eq!(triangle_soup::read(&removed).unwrap().len(), 1);

        let inverse = oracle_inverse_spec(FIXTURE.as_bytes(), &remove_spec).unwrap();
        assert_eq!(inverse.str("kind"), "insert-triangle");
        let restored = oracle_apply_mutation(&removed, &inverse).unwrap();
        assert_eq!(triangle_soup::read(&restored).unwrap().len(), 2);
    }

    #[test]
    fn set_snapshot_replaces_the_whole_triangle_list() {
        let one_triangle = Json::Array(vec![Json::Object(vec![
            ("normal".to_string(), Json::Array(vec![Json::Number(0.0), Json::Number(0.0), Json::Number(1.0)])),
            (
                "vertices".to_string(),
                Json::Array(vec![
                    Json::Array(vec![Json::Number(0.0), Json::Number(0.0), Json::Number(0.0)]),
                    Json::Array(vec![Json::Number(1.0), Json::Number(0.0), Json::Number(0.0)]),
                    Json::Array(vec![Json::Number(0.0), Json::Number(1.0), Json::Number(0.0)]),
                ]),
            ),
        ])]);
        let output = oracle_apply_mutation(FIXTURE.as_bytes(), &spec("set-snapshot", Json::Object(vec![("triangles".to_string(), one_triangle)]))).unwrap();
        assert_eq!(triangle_soup::read(&output).unwrap().len(), 1);
    }

    /// 🔁️ The real committed fixture is written by a `f64` producer — `0.0`, `-8.881784197001252e-16`
    /// — while `stl_io` resolves every coordinate through `f32`, so a genuine re-emission cannot
    /// reproduce it. This vector carries that same shape, which `FIXTURE`'s tidy integers do not:
    /// against `FIXTURE` the writer legitimately lands on the input again, and asserting otherwise
    /// there would be asserting a coincidence of formatting rather than the law.
    #[test]
    fn round_trip_re_emits_a_real_producer_document_rather_than_copying_it() {
        const REAL_SHAPED: &str = "solid forest\n  facet normal -0.8660253933154181 0.0 -0.5000000181328751\n    outer loop\n      vertex 0.0 2.734999895095825 -8.881784197001252e-16\n      vertex 0.0 3.0 -8.881784197001252e-16\n      vertex 2.700000047683716 3.0 -4.676537036895752\n    endloop\n  endfacet\nendsolid forest\n";
        let output = oracle_round_trip(REAL_SHAPED.as_bytes()).unwrap();
        assert_ne!(output, REAL_SHAPED.as_bytes(), "the document was re-emitted from the parsed model, so the f64 source decimals cannot survive verbatim");
        assert_eq!(triangle_soup::read(&output).unwrap().len(), 1);
        assert_eq!(ascii::read_name(&output).unwrap(), "forest");
    }

    #[test]
    fn unknown_kind_is_an_error_never_a_silent_no_op() {
        let result = oracle_apply_mutation(FIXTURE.as_bytes(), &spec("not-a-real-kind", Json::Object(vec![])));
        assert!(result.is_err(), "an unrecognised kind must fail loudly");
    }
}
//#endregion 🧪️Tests
