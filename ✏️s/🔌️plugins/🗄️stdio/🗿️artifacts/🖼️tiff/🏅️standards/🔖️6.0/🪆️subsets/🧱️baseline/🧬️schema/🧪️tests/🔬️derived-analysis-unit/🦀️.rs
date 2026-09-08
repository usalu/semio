mod tests {
    use super::*;
    use crate::standards::v6_0::subsets::document::schema::snapshot::{TiffByteOrder, TiffFieldType, TiffIfd, TiffTag};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn tag(id: u16, kind: TiffFieldType, values: TiffValues) -> TiffTag {
        TiffTag { tag: id, kind, values }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snapshot_with(width: u32, height: u32, pixels: Vec<u8>) -> TiffSnapshot {
        TiffSnapshot {
            schema: "stdio.tiff".into(),
            byte_order: TiffByteOrder::LittleEndian,
            ifds: vec![TiffIfd {
                pixels: Vec::new(),
                entries: vec![
                    tag(256, TiffFieldType::Long, TiffValues::Long(vec![width])),
                    tag(257, TiffFieldType::Long, TiffValues::Long(vec![height])),
                    tag(273, TiffFieldType::Long, TiffValues::Long(vec![8])),   // StripOffsets
                    tag(259, TiffFieldType::Short, TiffValues::Short(vec![1])), // Compression: none
                    tag(262, TiffFieldType::Short, TiffValues::Short(vec![2])), // PhotometricInterpretation: RGB
                    tag(258, TiffFieldType::Short, TiffValues::Short(vec![8])), // BitsPerSample
                ],
            }],
            pixels,
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn no_ifd_is_flagged_soft() {
        let snapshot = TiffSnapshot::default();
        let diagnostics = check_tiff_baseline_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_NO_IFD && d.severity == Severity::Warning), "got {diagnostics:?}");
        assert_eq!(diagnostics.len(), 1, "no-IFD short-circuits before any other check, got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn degenerate_zero_dimensions_are_flagged_soft() {
        let snapshot = snapshot_with(0, 0, Vec::new());
        let diagnostics = check_tiff_baseline_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DEGENERATE_RASTER && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn rgba_length_mismatch_is_flagged_soft() {
        let snapshot = snapshot_with(4, 4, vec![0u8; 4]); // way too short for 4x4 RGBA
        let diagnostics = check_tiff_baseline_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DEGENERATE_RASTER), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn well_formed_raster_has_no_findings() {
        let snapshot = snapshot_with(3, 2, vec![0u8; 3 * 2 * 4]);
        let diagnostics = check_tiff_baseline_conformance(&snapshot);
        assert!(diagnostics.is_empty(), "expected zero findings for a fully baseline-conformant IFD, got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn unsupported_compression_is_flagged_soft() {
        let mut snapshot = snapshot_with(2, 2, vec![0u8; 2 * 2 * 4]);
        snapshot.ifds[0].entries.iter_mut().find(|t| t.tag == 259).unwrap().values = TiffValues::Short(vec![5]); // LZW
        let diagnostics = check_tiff_baseline_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_UNSUPPORTED_COMPRESSION), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn unsupported_bits_per_sample_is_flagged_soft() {
        let mut snapshot = snapshot_with(2, 2, vec![0u8; 2 * 2 * 4]);
        snapshot.ifds[0].entries.iter_mut().find(|t| t.tag == 258).unwrap().values = TiffValues::Short(vec![16]);
        let diagnostics = check_tiff_baseline_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_UNSUPPORTED_BITS_PER_SAMPLE), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn tiled_organization_is_flagged_soft() {
        let mut snapshot = snapshot_with(2, 2, vec![0u8; 2 * 2 * 4]);
        snapshot.ifds[0].entries.push(tag(322, TiffFieldType::Long, TiffValues::Long(vec![16]))); // TileWidth
        let diagnostics = check_tiff_baseline_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_TILED_NOT_BASELINE), "got {diagnostics:?}");
    }
}
