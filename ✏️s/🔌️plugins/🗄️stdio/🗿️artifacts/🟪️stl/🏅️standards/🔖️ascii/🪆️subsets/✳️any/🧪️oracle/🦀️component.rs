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
//! **Standard mismatch, resolved**: this subset's OWN codec (`../🚪️io/🦀️component.rs`'s
//! `decode_stl_ascii`/`encode_stl_ascii`) is genuinely ASCII text — matching the `ascii` standard
//! this subset is filed under — but `stl_io` 0.8's own top-level doc comment states "Writing is
//! limited to binary STL", confirmed in its source (`writer.rs` hardcodes a zeroed 80-byte header
//! with no name field at all; `read_stl`'s `IndexedMesh` carries no name either). `stl_io` therefore
//! cannot express the ASCII form OR the solid name in either direction. Six of the seven declared
//! kinds round-trip the triangle soup through `stl_io`'s `IndexedMesh`/`write_stl` regardless (the
//! byte FORM is writer freedom under `semantic-mesh-v1`'s projection-based comparison); the seventh,
//! `set-solid-name` — whose whole payload IS the field `stl_io` cannot touch — is instead applied as
//! a direct ASCII header/trailer substitution on the same real document.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

//#region 🔖️TriangleSoup
/// 🧊️ Independent triangle-soup reading/writing behind `stl_io`, shared by every dispatch arm below
/// except `set-solid-name` (see this file's top doc comment for why that one is ASCII-text-only).
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

    /// 📤️ Independent write: always binary (`stl_io` has no ASCII writer — see this file's top doc
    /// comment). Comparison happens on the projection, so the byte FORM never has to match the
    /// subject's own ASCII output.
    pub(super) fn write(triangles: &[RefTriangle]) -> Result<Vec<u8>, String> {
        let faces: Vec<stl_io::Triangle> = triangles.iter().map(|triangle| stl_io::Triangle { normal: stl_io::Normal::new(triangle.normal), vertices: triangle.vertices.map(stl_io::Vertex::new) }).collect();
        let mut out = std::io::Cursor::new(Vec::new());
        stl_io::write_stl(&mut out, faces.iter()).map_err(|error| format!("stl write: {error}"))?;
        Ok(out.into_inner())
    }
}
//#endregion 🔖️TriangleSoup

//#region 🔖️AsciiName
/// 🏷️ `set-solid-name`'s dedicated path: `stl_io` cannot represent the solid name (see this file's
/// top doc comment), so the header/trailer are substituted directly on the real ASCII text this
/// subset's own fixture is committed in — independent of `stl_io`, but still independent of the
/// subject under test.
#[cfg(feature = "oracles")]
mod ascii_name {
    fn text(input: &[u8]) -> Result<String, String> {
        String::from_utf8(input.to_vec()).map_err(|error| format!("set-solid-name requires ASCII STL text: {error}"))
    }

    pub(super) fn read(input: &[u8]) -> Result<String, String> {
        let source = text(input)?;
        let header = source.lines().next().ok_or_else(|| "stl ascii: empty document".to_string())?;
        if !header.trim_start().starts_with("solid") {
            return Err("stl ascii: missing 'solid' header".to_string());
        }
        Ok(header.trim().strip_prefix("solid").unwrap_or("").trim().to_string())
    }

    pub(super) fn write(input: &[u8], name: &str) -> Result<Vec<u8>, String> {
        let source = text(input)?;
        let lines: Vec<&str> = source.lines().collect();
        let trailer = lines.iter().rposition(|line| line.trim_start().starts_with("endsolid")).ok_or_else(|| "stl ascii: missing 'endsolid' trailer".to_string())?;
        let mut out = format!("solid {name}\n");
        for line in &lines[1..trailer] {
            out.push_str(line);
            out.push('\n');
        }
        out.push_str(&format!("endsolid {name}\n"));
        Ok(out.into_bytes())
    }
}
//#endregion 🔖️AsciiName

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
        "no-mutation" => Ok(input.to_vec()),
        "set-solid-name" => ascii_name::write(input, &string(&params, "name").ok_or("set-solid-name: missing `name`")?),
        "insert-triangle" => {
            let mut triangles = triangle_soup::read(input)?;
            let index = number(&params, "index").ok_or("insert-triangle: missing `index`")? as usize;
            let triangle = triangle_of(params.get("triangle").ok_or("insert-triangle: missing `triangle`")?).ok_or("insert-triangle: malformed `triangle`")?;
            triangles.insert(index.min(triangles.len()), triangle);
            triangle_soup::write(&triangles)
        }
        "remove-triangle" => {
            let mut triangles = triangle_soup::read(input)?;
            let index = number(&params, "index").ok_or("remove-triangle: missing `index`")? as usize;
            if index < triangles.len() {
                triangles.remove(index);
            }
            triangle_soup::write(&triangles)
        }
        "set-triangle-normal" => {
            let mut triangles = triangle_soup::read(input)?;
            let index = number(&params, "index").ok_or("set-triangle-normal: missing `index`")? as usize;
            let normal = vec3(&params, "normal").ok_or("set-triangle-normal: missing `normal`")?;
            if let Some(triangle) = triangles.get_mut(index) {
                triangle.normal = normal;
            }
            triangle_soup::write(&triangles)
        }
        "set-triangle-vertices" => {
            let mut triangles = triangle_soup::read(input)?;
            let index = number(&params, "index").ok_or("set-triangle-vertices: missing `index`")? as usize;
            let vertices = vertices3(&params, "vertices").ok_or("set-triangle-vertices: missing `vertices`")?;
            if let Some(triangle) = triangles.get_mut(index) {
                triangle.vertices = vertices;
            }
            triangle_soup::write(&triangles)
        }
        "set-snapshot" => triangle_soup::write(&triangles_of(&params, "triangles").ok_or("set-snapshot: missing/malformed `triangles`")?),
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
/// `StlMutation::inverse()` (`../🧬️schema/🧬️mutations/🦀️component.rs`) independently: an
/// out-of-range index that the forward mutation would have rejected inverts to `no-mutation`,
/// exactly as that hand-rolled method does.
#[cfg(feature = "oracles")]
pub fn oracle_inverse_spec(base: &[u8], spec: &Json) -> Result<Json, String> {
    let params = mutation_params(spec);
    Ok(match spec.str("kind").as_str() {
        "" => return Err("mutation spec carries no `kind`".to_string()),
        "no-mutation" => spec_of("no-mutation", Json::Object(vec![])),
        "set-solid-name" => spec_of("set-solid-name", Json::Object(vec![("name".to_string(), Json::String(ascii_name::read(base)?))])),
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

//#region 🔖️RoundTrip
/// 🔁️ A genuine independent decode + re-encode with no shortcut — unlike `no-mutation`'s literal
/// passthrough above, this is what the identity-round-trip scenario needs: real bytes out of a real
/// parse, guaranteed to differ from ASCII input (`stl_io` writes binary only).
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    triangle_soup::write(&triangle_soup::read(input)?)
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
    fn no_mutation_is_a_true_byte_identity() {
        let output = oracle_apply_mutation(FIXTURE.as_bytes(), &spec("no-mutation", Json::Object(vec![]))).unwrap();
        assert_eq!(output, FIXTURE.as_bytes());
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

    #[test]
    fn round_trip_never_passes_bytes_through() {
        let output = oracle_round_trip(FIXTURE.as_bytes()).unwrap();
        assert_ne!(output, FIXTURE.as_bytes());
        assert_eq!(triangle_soup::read(&output).unwrap().len(), 2);
    }

    #[test]
    fn unknown_kind_is_an_error_never_a_silent_no_op() {
        let result = oracle_apply_mutation(FIXTURE.as_bytes(), &spec("not-a-real-kind", Json::Object(vec![])));
        assert!(result.is_err(), "an unrecognised kind must fail loudly");
    }
}
//#endregion 🧪️Tests
