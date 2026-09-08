mod tests {
    use super::*;
    use crate::standards::v2_0::subsets::base::io::ZipCentralEntryHeader;
    use crate::standards::v2_0::subsets::base::schema::snapshot::ZipEntry;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn entry(name: &str) -> ZipEntry {
        ZipEntry { name: name.into(), data: b"payload".to_vec() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn header(name: &str, flags: u16, version_needed: u16) -> ZipCentralEntryHeader {
        ZipCentralEntryHeader { name: name.into(), flags, version_needed }
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_snapshot_has_no_diagnostics() {
        let snapshot = ZipSnapshot { entries: vec![entry("a.txt")], ..ZipSnapshot::default() };
        let diagnostics = check_iso21320_conformance(&snapshot);
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn encrypted_entry_is_hard() {
        let diagnostics = check_iso21320_entry_headers(&[header("secret.bin", FLAG_ENCRYPTED, 20)]);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ENCRYPTED && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn strong_encryption_bit_is_hard() {
        let diagnostics = check_iso21320_entry_headers(&[header("strong.bin", FLAG_STRONG_ENCRYPTION, 20)]);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRONG_ENCRYPTION && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn masked_local_header_bit_is_hard() {
        let diagnostics = check_iso21320_entry_headers(&[header("masked.bin", FLAG_MASKED_LOCAL_HEADERS, 20)]);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRONG_ENCRYPTION && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn data_descriptor_bit_is_soft() {
        let diagnostics = check_iso21320_entry_headers(&[header("streamed.bin", FLAG_DATA_DESCRIPTOR, 20)]);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DATA_DESCRIPTOR && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn high_version_needed_is_soft() {
        let diagnostics = check_iso21320_entry_headers(&[header("zip64.bin", 0, 63)]);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_VERSION_NEEDED && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn version_needed_at_ceiling_is_clean() {
        assert!(check_iso21320_entry_headers(&[header("boundary.bin", 0, VERSION_NEEDED_SOFT_CEILING)]).is_empty());
    }
}
