mod tests {
    use super::*;
    use crate::schema::snapshot::{JpgFrameComponent, JpgFrameHeader, JpgHuffmanTable};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn conforming_snapshot() -> JpgSnapshot {
        JpgSnapshot {
            frame: Some(JpgFrameHeader {
                precision: 8,
                width: 16,
                height: 16,
                components: vec![
                    JpgFrameComponent { id: 1, h_sampling: 2, v_sampling: 2, quant_table_id: 0 },
                    JpgFrameComponent { id: 2, h_sampling: 1, v_sampling: 1, quant_table_id: 1 },
                    JpgFrameComponent { id: 3, h_sampling: 1, v_sampling: 1, quant_table_id: 1 },
                ],
            }),
            sof_marker: SOF0,
            arithmetic: false,
            huffman_tables: vec![
                JpgHuffmanTable { id: 0, class: JpgHuffmanClass::Dc, bits: [0; 16], values: vec![] },
                JpgHuffmanTable { id: 1, class: JpgHuffmanClass::Dc, bits: [0; 16], values: vec![] },
                JpgHuffmanTable { id: 0, class: JpgHuffmanClass::Ac, bits: [0; 16], values: vec![] },
                JpgHuffmanTable { id: 1, class: JpgHuffmanClass::Ac, bits: [0; 16], values: vec![] },
            ],
            width: 16,
            height: 16,
            pixels: vec![0u8; 16 * 16 * 4],
            ..JpgSnapshot::default()
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_snapshot_has_no_diagnostics() {
        let diagnostics = check_baseline_conformance(&conforming_snapshot());
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_frame_is_hard() {
        let snapshot = JpgSnapshot::default();
        let diagnostics = check_baseline_conformance(&snapshot);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code.0, CODE_NO_FRAME);
        assert_eq!(diagnostics[0].severity, Severity::Error);
    }

    #[semio_framework_async_macros::async_test]
    async fn non_sof0_marker_is_hard() {
        let mut snapshot = conforming_snapshot();
        snapshot.sof_marker = 0xC2; // SOF2 (progressive)
        let diagnostics = check_baseline_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_SOF_MARKER && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn non_8bit_precision_is_hard() {
        let mut snapshot = conforming_snapshot();
        snapshot.frame.as_mut().unwrap().precision = 12;
        let diagnostics = check_baseline_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_PRECISION && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn arithmetic_conditioning_present_is_hard() {
        let mut snapshot = conforming_snapshot();
        snapshot.arithmetic = true;
        let diagnostics = check_baseline_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ARITHMETIC && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn excess_huffman_tables_is_soft() {
        let mut snapshot = conforming_snapshot();
        snapshot.huffman_tables.push(JpgHuffmanTable { id: 2, class: JpgHuffmanClass::Dc, bits: [0; 16], values: vec![] });
        let diagnostics = check_baseline_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_HUFFMAN_TABLE_COUNT && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn excess_components_is_soft() {
        let mut snapshot = conforming_snapshot();
        let frame = snapshot.frame.as_mut().unwrap();
        frame.components.push(JpgFrameComponent { id: 4, h_sampling: 1, v_sampling: 1, quant_table_id: 1 });
        frame.components.push(JpgFrameComponent { id: 5, h_sampling: 1, v_sampling: 1, quant_table_id: 1 });
        let diagnostics = check_baseline_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_COMPONENT_SAMPLING && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn out_of_range_sampling_is_soft() {
        let mut snapshot = conforming_snapshot();
        snapshot.frame.as_mut().unwrap().components[0].h_sampling = 8;
        let diagnostics = check_baseline_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_COMPONENT_SAMPLING && d.severity == Severity::Warning), "got {diagnostics:?}");
    }
}
