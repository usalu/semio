//! 🧐️ TiffBaselineAnalyzer (6.0/✳️baseline) — REAL Adobe TIFF 6.0 Part 1 "Baseline TIFF"
//! conformance checks against `TiffSnapshot`'s now-complete IFD/tag model. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION closed the schema
//! gap this subset's own doc comment (its earlier revision) named as the upgrade path:
//! `TiffSnapshot` now retains the real IFD tag/type/value triples, so Baseline TIFF's
//! `Compression`/`PhotometricInterpretation`/`BitsPerSample`/strip-vs-tile checks are
//! genuinely checkable from PERSISTED state, not just transiently during decode.
//!
//! Real Baseline TIFF conformance (Adobe TIFF 6.0 Part 1 §2): `Compression` in `{1 none, 2
//! CCITT G3 1D, 32773 PackBits}`; `PhotometricInterpretation` in `{0,1,2,3}`; strip
//! organization (`StripOffsets` present, no `Tile*` tags — Baseline forbids tiling);
//! `BitsPerSample` values each in `{1,4,8}`. All diagnostics stay SOFT (warnings) — this
//! analyzer only reports, it never hard-gates (see `🏗️builder`'s `build()`, which always
//! succeeds regardless of these findings).

use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalyzer, Dialect, IoConfidence, StandardId, SubsetId};
use crate::artifacts::tiff::standards::v6_0::subsets::any::analyzer::{TiffAnalyzer as TiffAnyAnalyzer, TiffParts};
use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::snapshot::{
    TiffSnapshot, TiffValues, TAG_BITS_PER_SAMPLE, TAG_COMPRESSION, TAG_PHOTOMETRIC, TAG_STRIP_OFFSETS, TAG_TILE_LENGTH, TAG_TILE_WIDTH,
};

/// 🎯️ This subset's dialect coordinate.
pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("baseline") };

//#region 🔖️Conformance
pub const CODE_DEGENERATE_RASTER: &str = "stdio.tiff.baseline.degenerate-raster";
pub const CODE_NO_IFD: &str = "stdio.tiff.baseline.no-ifd";
pub const CODE_UNSUPPORTED_COMPRESSION: &str = "stdio.tiff.baseline.unsupported-compression";
pub const CODE_UNSUPPORTED_PHOTOMETRIC: &str = "stdio.tiff.baseline.unsupported-photometric";
pub const CODE_UNSUPPORTED_BITS_PER_SAMPLE: &str = "stdio.tiff.baseline.unsupported-bits-per-sample";
pub const CODE_TILED_NOT_BASELINE: &str = "stdio.tiff.baseline.tiled-not-baseline";
pub const CODE_MISSING_STRIP_OFFSETS: &str = "stdio.tiff.baseline.missing-strip-offsets";

fn soft(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

fn ifd0_u32_list(snapshot: &TiffSnapshot, tag: u16) -> Vec<u32> {
    match snapshot.ifds.first().and_then(|ifd| ifd.entries.iter().find(|t| t.tag == tag)) {
        Some(t) => match &t.values {
            TiffValues::Short(v) => v.iter().map(|&x| x as u32).collect(),
            TiffValues::Long(v) => v.clone(),
            _ => Vec::new(),
        },
        None => Vec::new(),
    }
}

/// 🛡️ Real Baseline TIFF conformance check against one already-decoded `TiffSnapshot`. Shared
/// single source of truth: `TiffBaselineComposer::compose` and the registered
/// `SubsetValidator` both call this.
pub fn check_tiff_baseline_conformance(snapshot: &TiffSnapshot) -> Vec<Diagnostic> {
    let mut out = Vec::new();

    let Some(ifd0) = snapshot.ifds.first() else {
        out.push(soft(CODE_NO_IFD, "no IFD present -- Baseline TIFF conformance cannot be checked at all".into()));
        return out;
    };

    match (snapshot.width(), snapshot.height()) {
        (Some(width), Some(height)) => {
            let expected_len = width as usize * height as usize * 4;
            if width == 0 || height == 0 || snapshot.pixels.len() != expected_len {
                out.push(soft(
                    CODE_DEGENERATE_RASTER,
                    format!("raster is degenerate (width={width}, height={height}, pixels.len()={}, expected {expected_len})", snapshot.pixels.len()),
                ));
            }
        }
        _ => out.push(soft(CODE_DEGENERATE_RASTER, "IFD 0 has no ImageWidth/ImageLength tag".into())),
    }

    if let Some(&c) = ifd0_u32_list(snapshot, TAG_COMPRESSION).first() {
        if c != 1 && c != 2 && c != 32773 {
            out.push(soft(CODE_UNSUPPORTED_COMPRESSION, format!("Compression {c} is not one of Baseline TIFF's {{1 none, 2 CCITT G3 1D, 32773 PackBits}}")));
        }
    }
    if let Some(&p) = ifd0_u32_list(snapshot, TAG_PHOTOMETRIC).first() {
        if p > 3 {
            out.push(soft(CODE_UNSUPPORTED_PHOTOMETRIC, format!("PhotometricInterpretation {p} is not one of Baseline TIFF's {{0,1,2,3}}")));
        }
    }
    let bits = ifd0_u32_list(snapshot, TAG_BITS_PER_SAMPLE);
    if bits.iter().any(|&b| b != 1 && b != 4 && b != 8) {
        out.push(soft(CODE_UNSUPPORTED_BITS_PER_SAMPLE, format!("BitsPerSample {bits:?} has a value outside Baseline TIFF's {{1,4,8}}")));
    }
    let has_tile = ifd0.entries.iter().any(|t| t.tag == TAG_TILE_WIDTH || t.tag == TAG_TILE_LENGTH);
    if has_tile {
        out.push(soft(CODE_TILED_NOT_BASELINE, "Baseline TIFF requires strip organization; this IFD carries Tile* tags".into()));
    } else if !ifd0.entries.iter().any(|t| t.tag == TAG_STRIP_OFFSETS) {
        out.push(soft(CODE_MISSING_STRIP_OFFSETS, "IFD 0 has neither StripOffsets nor Tile* tags -- no recognizable pixel organization".into()));
    }

    out
}
//#endregion 🔖️Conformance

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.tiff` (6.0/✳️baseline): delegates the real parse to the ✳️any subset's
/// analyzer (same `TiffSnapshot`), then folds the real Baseline TIFF diagnostics on top.
pub struct TiffBaselineAnalyzer;

impl ArtifactAnalyzer for TiffBaselineAnalyzer {
    type Parts = TiffParts;
    const DIALECT: Dialect = DIALECT;

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        TiffAnyAnalyzer::sniff(source)
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let inner = TiffAnyAnalyzer::analyze(sources);
        let mut diagnostics = inner.diagnostics.clone();
        if let Some(snapshot) = &inner.parts.snapshot {
            diagnostics.extend(check_tiff_baseline_conformance(snapshot));
        }
        Analysis { parts: inner.parts, dialect: DIALECT, confidence: inner.confidence, diagnostics }
    }
}
//#endregion 🔖️Analyzer

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::snapshot::{TiffByteOrder, TiffFieldType, TiffIfd, TiffTag};

    fn tag(id: u16, kind: TiffFieldType, values: TiffValues) -> TiffTag {
        TiffTag { tag: id, kind, values }
    }

    fn snapshot_with(width: u32, height: u32, pixels: Vec<u8>) -> TiffSnapshot {
        TiffSnapshot {
            schema: "stdio.tiff".into(),
            byte_order: TiffByteOrder::LittleEndian,
            ifds: vec![TiffIfd {
                entries: vec![
                    tag(256, TiffFieldType::Long, TiffValues::Long(vec![width])),
                    tag(257, TiffFieldType::Long, TiffValues::Long(vec![height])),
                    tag(273, TiffFieldType::Long, TiffValues::Long(vec![8])), // StripOffsets
                    tag(259, TiffFieldType::Short, TiffValues::Short(vec![1])), // Compression: none
                    tag(262, TiffFieldType::Short, TiffValues::Short(vec![2])), // PhotometricInterpretation: RGB
                    tag(258, TiffFieldType::Short, TiffValues::Short(vec![8])), // BitsPerSample
                ],
            }],
            pixels,
        }
    }

    #[test]
    fn no_ifd_is_flagged_soft() {
        let snapshot = TiffSnapshot::default();
        let diagnostics = check_tiff_baseline_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_NO_IFD && d.severity == Severity::Warning), "got {diagnostics:?}");
        assert_eq!(diagnostics.len(), 1, "no-IFD short-circuits before any other check, got {diagnostics:?}");
    }

    #[test]
    fn degenerate_zero_dimensions_are_flagged_soft() {
        let snapshot = snapshot_with(0, 0, Vec::new());
        let diagnostics = check_tiff_baseline_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DEGENERATE_RASTER && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[test]
    fn rgba_length_mismatch_is_flagged_soft() {
        let snapshot = snapshot_with(4, 4, vec![0u8; 4]); // way too short for 4x4 RGBA
        let diagnostics = check_tiff_baseline_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DEGENERATE_RASTER), "got {diagnostics:?}");
    }

    #[test]
    fn well_formed_raster_has_no_findings() {
        let snapshot = snapshot_with(3, 2, vec![0u8; 3 * 2 * 4]);
        let diagnostics = check_tiff_baseline_conformance(&snapshot);
        assert!(diagnostics.is_empty(), "expected zero findings for a fully baseline-conformant IFD, got {diagnostics:?}");
    }

    #[test]
    fn unsupported_compression_is_flagged_soft() {
        let mut snapshot = snapshot_with(2, 2, vec![0u8; 2 * 2 * 4]);
        snapshot.ifds[0].entries.iter_mut().find(|t| t.tag == 259).unwrap().values = TiffValues::Short(vec![5]); // LZW
        let diagnostics = check_tiff_baseline_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_UNSUPPORTED_COMPRESSION), "got {diagnostics:?}");
    }

    #[test]
    fn unsupported_bits_per_sample_is_flagged_soft() {
        let mut snapshot = snapshot_with(2, 2, vec![0u8; 2 * 2 * 4]);
        snapshot.ifds[0].entries.iter_mut().find(|t| t.tag == 258).unwrap().values = TiffValues::Short(vec![16]);
        let diagnostics = check_tiff_baseline_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_UNSUPPORTED_BITS_PER_SAMPLE), "got {diagnostics:?}");
    }

    #[test]
    fn tiled_organization_is_flagged_soft() {
        let mut snapshot = snapshot_with(2, 2, vec![0u8; 2 * 2 * 4]);
        snapshot.ifds[0].entries.push(tag(322, TiffFieldType::Long, TiffValues::Long(vec![16]))); // TileWidth
        let diagnostics = check_tiff_baseline_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_TILED_NOT_BASELINE), "got {diagnostics:?}");
    }
}
