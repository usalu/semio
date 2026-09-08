//! 🚪️ IO stdio.stl (ascii/✳️any) — registration now flows through the `s.stdio.stl`
//! `ArtifactDeclaration` (`crate::declaration`), not per-leaf register().
use crate::schema::snapshot::StlTriangle;
use crate::{StlSnapshot, STDIO_STL_DOCUMENT_SCHEMA};

//#region 🔖️Codec
//#region 🔖️Ascii
/// 📥 Parses `solid <name>`/`facet normal`/`outer loop`/`vertex`×3/`endloop`/`endfacet`/
/// `endsolid` ASCII STL. The facet normal is persisted exactly as written (real STL files often
/// carry a degenerate `facet normal 0 0 0` and rely on downstream tooling to recompute it — this
/// codec doesn't silently rewrite that on decode, matching the recipe's "nothing fabricated"
/// rule; `<StlTriangle as PartialEq>` sees whatever the file actually said).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_stl_ascii(text: &str) -> Result<StlSnapshot, String> {
    if !text.trim_start().starts_with("solid") {
        return Err("stl ascii: missing 'solid' header".into());
    }
    let mut lines = text.lines();
    let header = lines.next().unwrap_or("");
    let solid_name = header.trim().strip_prefix("solid").unwrap_or("").trim().to_string();

    let mut triangles = Vec::new();
    let mut normal: [f64; 3] = [0.0; 3];
    let mut verts: [Option<[f64; 3]>; 3] = [None, None, None];
    let mut slot = 0usize;
    let mut in_loop = false;
    for line in lines {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("facet normal") {
            let coords: Vec<f64> = rest.split_whitespace().map(|s| s.parse::<f64>().map_err(|e| e.to_string())).collect::<Result<Vec<_>, _>>()?;
            if coords.len() < 3 {
                return Err("stl ascii: facet normal needs 3 coords".into());
            }
            normal = [coords[0], coords[1], coords[2]];
        } else if line.starts_with("outer loop") {
            in_loop = true;
            slot = 0;
            verts = [None, None, None];
        } else if line.starts_with("endloop") {
            if slot != 3 {
                return Err(format!("stl ascii: facet had {slot} vertices, expected 3"));
            }
            let vertices = [verts[0].unwrap(), verts[1].unwrap(), verts[2].unwrap()];
            triangles.push(StlTriangle { normal, vertices });
            in_loop = false;
        } else if in_loop && line.starts_with("vertex") {
            let coords: Vec<f64> = line.trim_start_matches("vertex").split_whitespace().map(|s| s.parse::<f64>().map_err(|e| e.to_string())).collect::<Result<Vec<_>, _>>()?;
            if coords.len() < 3 {
                return Err("stl ascii: vertex needs 3 coords".into());
            }
            if slot >= 3 {
                return Err("stl ascii: more than 3 vertices in one facet".into());
            }
            verts[slot] = Some([coords[0], coords[1], coords[2]]);
            slot += 1;
        }
    }
    Ok(StlSnapshot { schema: STDIO_STL_DOCUMENT_SCHEMA.into(), solid_name, triangles })
}

/// 📤 Writes real ASCII STL, round-tripping each facet's persisted normal exactly (never
/// recomputed from vertex winding — see `decode_stl_ascii`'s doc comment).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_stl_ascii(snap: &StlSnapshot) -> String {
    let mut out = format!("solid {}\n", snap.solid_name);
    for f in &snap.triangles {
        let [nx, ny, nz] = f.normal;
        out.push_str(&format!("  facet normal {nx} {ny} {nz}\n"));
        out.push_str("    outer loop\n");
        for [x, y, z] in f.vertices {
            out.push_str(&format!("      vertex {x} {y} {z}\n"));
        }
        out.push_str("    endloop\n  endfacet\n");
    }
    out.push_str(&format!("endsolid {}\n", snap.solid_name));
    out
}
//#endregion 🔖️Ascii

//#region 🔖️Binary
/// 📥 Parses binary STL: 80-byte header (used as `solid_name`, trimmed of trailing NULs/
/// whitespace — real-world binary STL writers commonly stash a comment/name there even though
/// the spec calls it opaque) + u32 triangle count + N × (12-byte normal [f32×3] + 3×12-byte
/// vertices [f32×3] + 2-byte attribute-byte-count [dropped: no attribute-byte-count usage is
/// specified by the base format]). Normals/vertices widen `f32` -> `f64` (see `StlTriangle`'s
/// doc comment on the ASCII/binary precision-normalization tradeoff).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_stl_binary(bytes: &[u8]) -> Result<StlSnapshot, String> {
    if bytes.len() < 84 {
        return Err("stl binary: header too short".into());
    }
    let solid_name = String::from_utf8_lossy(&bytes[0..80]).trim_end_matches('\0').trim().to_string();
    let count = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
    let mut triangles = Vec::with_capacity(count);
    let mut off = 84usize;
    for _ in 0..count {
        if off + 50 > bytes.len() {
            return Err("stl binary: truncated facet record".into());
        }
        let read_vec3 =
            |b: &[u8], at: usize| -> [f64; 3] { [f32::from_le_bytes(b[at..at + 4].try_into().unwrap()) as f64, f32::from_le_bytes(b[at + 4..at + 8].try_into().unwrap()) as f64, f32::from_le_bytes(b[at + 8..at + 12].try_into().unwrap()) as f64] };
        let normal = read_vec3(bytes, off);
        off += 12;
        let mut vertices = [[0.0; 3]; 3];
        for v in vertices.iter_mut() {
            *v = read_vec3(bytes, off);
            off += 12;
        }
        off += 2; // attribute byte count
        triangles.push(StlTriangle { normal, vertices });
    }
    Ok(StlSnapshot { schema: STDIO_STL_DOCUMENT_SCHEMA.into(), solid_name, triangles })
}

/// 📤 Writes real binary STL: 80-byte header (`solid_name`, truncated to 80 bytes / zero-padded)
/// + u32 triangle count at offset 80..84 (the count belongs INSIDE the 84-byte header, not
///   appended after it — an 80-byte header vec here, not 84, is what makes the count land at the
///   right offset). Each facet's persisted `f64` normal/vertices narrow to `f32` (binary STL's
///   spec-mandated precision — a documented, lossy normalization, not fabrication).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_stl_binary(snap: &StlSnapshot) -> Vec<u8> {
    let mut out = vec![0u8; 80];
    let name_bytes = snap.solid_name.as_bytes();
    let n = name_bytes.len().min(80);
    out[..n].copy_from_slice(&name_bytes[..n]);
    out.extend_from_slice(&(snap.triangles.len() as u32).to_le_bytes());
    for f in &snap.triangles {
        for v in f.normal {
            out.extend_from_slice(&(v as f32).to_le_bytes());
        }
        for vertex in f.vertices {
            for v in vertex {
                out.extend_from_slice(&(v as f32).to_le_bytes());
            }
        }
        out.extend_from_slice(&0u16.to_le_bytes());
    }
    out
}
//#endregion 🔖️Binary

//#region 🔖️AutoDetect
/// 🔍 Dispatches on the `solid` ASCII magic; anything else is treated as binary STL.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_stl_auto(bytes: &[u8]) -> Result<StlSnapshot, String> {
    if bytes.len() >= 5 && &bytes[0..5] == b"solid" {
        // A binary STL's 80-byte header can coincidentally start with "solid" too;
        // disambiguate by checking whether the binary triangle-count framing actually
        // matches the file length before trusting the ASCII path.
        if bytes.len() >= 84 {
            let count = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
            let expected_binary_len = 84 + count * 50;
            if expected_binary_len == bytes.len() {
                return decode_stl_binary(bytes);
            }
        }
        decode_stl_ascii(std::str::from_utf8(bytes).map_err(|e| e.to_string())?)
    } else {
        decode_stl_binary(bytes)
    }
}
//#endregion 🔖️AutoDetect
//#endregion 🔖️Codec

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_ascii::subsets::any::schema::StlAnalyzer;
    use crate::StlSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct StlComposerComposition;

    impl ArtifactComposition for StlComposerComposition {
        type Snapshot = StlSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_TXT, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_TXT || s.dialect == DEP_BINARY)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "StlComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = StlAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "StlComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
/// 🚪️ Dissolved out of `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
pub mod io_registry {
    use crate::standards::v_ascii::subsets::any::schema::StlComposer as StlRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<StlRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
