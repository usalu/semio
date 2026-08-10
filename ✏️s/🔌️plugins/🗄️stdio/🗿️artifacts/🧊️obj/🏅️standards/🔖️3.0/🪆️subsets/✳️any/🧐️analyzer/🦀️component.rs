//! 🧐️ ObjAnalyzer (3.0/✳️any) — read-only analysis, successor to the pre-migration
//! ObjDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::obj::ObjSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.obj` parts.
#[derive(Clone, Debug, Default)]
pub struct ObjParts {
    pub snapshot: Option<ObjSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Sniff
/// 🔍 OBJ has no magic byte signature (it's plain text) — sniff by scanning the first
/// ~200 non-blank lines for real Wavefront keyword shapes (`v `/`f ` are the strong
/// signal; `vt`/`vn`/`o`/`g`/`usemtl`/`s`/`mtllib` are weaker corroborating signals).
fn looks_like_obj(text: &str) -> IoConfidence {
    let mut vertex_lines = 0u32;
    let mut face_lines = 0u32;
    let mut other_tokens = 0u32;
    for line in text.lines().take(200) {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match line.split_whitespace().next() {
            Some("v") => vertex_lines += 1,
            Some("f") => face_lines += 1,
            Some("vt") | Some("vn") | Some("o") | Some("g") | Some("usemtl") | Some("s") | Some("mtllib") => other_tokens += 1,
            _ => {}
        }
    }
    if vertex_lines > 0 && face_lines > 0 {
        IoConfidence::High
    } else if vertex_lines > 0 || face_lines > 0 || other_tokens > 0 {
        IoConfidence::Medium
    } else {
        IoConfidence::Low
    }
}
//#endregion 🔖️Sniff

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.obj` (3.0/✳️any) sources.
pub struct ObjAnalyzer;

impl ArtifactAnalyzer for ObjAnalyzer {
    type Parts = ObjParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        match source {
            AnalyzeSource::Text(text) => {
                let body = match store::semio_format::split_text_preamble(text) {
                    Ok((_, rest)) => rest,
                    Err(_) => text,
                };
                looks_like_obj(body)
            }
            AnalyzeSource::Binary(bytes) => match store::semio_format::unwrap_binary(bytes) {
                Ok((_, inner)) => match String::from_utf8(inner) {
                    Ok(text) => looks_like_obj(&text),
                    Err(_) => IoConfidence::Low,
                },
                Err(_) => match std::str::from_utf8(bytes) {
                    Ok(text) => looks_like_obj(text),
                    Err(_) => IoConfidence::Low,
                },
            },
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = ObjParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <ObjSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.analyze.text",
                            dsl::TextSpan::at(1, 1),
                            err.to_string(),
                        ));
                    }
                },
                AnalyzeSource::Binary(bytes) => match <ObjSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.analyze.binary",
                            dsl::TextSpan::at(1, 1),
                            err.to_string(),
                        ));
                    }
                },
            }
        }
        Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
    }
}
//#endregion 🔖️Analyzer

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sniff_real_obj_text_is_high() {
        let text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";
        assert_eq!(ObjAnalyzer::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
    }

    #[test]
    fn sniff_unrelated_text_is_low() {
        let text = "{\"not\": \"an obj file at all\"}";
        assert_eq!(ObjAnalyzer::sniff(&AnalyzeSource::Text(text)), IoConfidence::Low);
    }
}
//#endregion 🧪️Tests
