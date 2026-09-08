mod tests {
    use super::*;
    use crate::standards::v2_0::subsets::iso21320::schema::ZipIso21320BuilderConstruction as ZipIso21320Builder;
    use crate::standards::v2_0::subsets::iso21320::schema::{CODE_ENCRYPTED, FLAG_ENCRYPTED, check_iso21320_wire_conformance};
    use semio_framework_plugin::AnalyzeSource;
    use semio_framework_plugin::ArtifactBuilder as _;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn raw_zip_with_flags(flags: u16, version_needed: u16) -> Vec<u8> {
        let data = b"payload";
        let crc = crate::standards::v2_0::subsets::base::io::crc32(data);
        let name = b"secret.bin";
        let mut local = Vec::new();
        local.extend_from_slice(&0x0403_4b50u32.to_le_bytes());
        local.extend_from_slice(&version_needed.to_le_bytes());
        local.extend_from_slice(&flags.to_le_bytes());
        local.extend_from_slice(&0u16.to_le_bytes());
        local.extend_from_slice(&0u16.to_le_bytes());
        local.extend_from_slice(&0u16.to_le_bytes());
        local.extend_from_slice(&crc.to_le_bytes());
        local.extend_from_slice(&(data.len() as u32).to_le_bytes());
        local.extend_from_slice(&(data.len() as u32).to_le_bytes());
        local.extend_from_slice(&(name.len() as u16).to_le_bytes());
        local.extend_from_slice(&0u16.to_le_bytes());
        local.extend_from_slice(name);
        local.extend_from_slice(data);

        let offset = 0u32;
        let mut cen = Vec::new();
        cen.extend_from_slice(&0x0201_4b50u32.to_le_bytes());
        cen.extend_from_slice(&20u16.to_le_bytes());
        cen.extend_from_slice(&version_needed.to_le_bytes());
        cen.extend_from_slice(&flags.to_le_bytes());
        cen.extend_from_slice(&0u16.to_le_bytes());
        cen.extend_from_slice(&0u16.to_le_bytes());
        cen.extend_from_slice(&0u16.to_le_bytes());
        cen.extend_from_slice(&crc.to_le_bytes());
        cen.extend_from_slice(&(data.len() as u32).to_le_bytes());
        cen.extend_from_slice(&(data.len() as u32).to_le_bytes());
        cen.extend_from_slice(&(name.len() as u16).to_le_bytes());
        cen.extend_from_slice(&0u16.to_le_bytes());
        cen.extend_from_slice(&0u16.to_le_bytes());
        cen.extend_from_slice(&0u16.to_le_bytes());
        cen.extend_from_slice(&0u16.to_le_bytes());
        cen.extend_from_slice(&0u32.to_le_bytes());
        cen.extend_from_slice(&offset.to_le_bytes());
        cen.extend_from_slice(name);

        let cd_offset = local.len() as u32;
        let cd_size = cen.len() as u32;
        let mut eocd = Vec::new();
        eocd.extend_from_slice(&0x0605_4b50u32.to_le_bytes());
        eocd.extend_from_slice(&0u16.to_le_bytes());
        eocd.extend_from_slice(&0u16.to_le_bytes());
        eocd.extend_from_slice(&1u16.to_le_bytes());
        eocd.extend_from_slice(&1u16.to_le_bytes());
        eocd.extend_from_slice(&cd_size.to_le_bytes());
        eocd.extend_from_slice(&cd_offset.to_le_bytes());
        eocd.extend_from_slice(&0u16.to_le_bytes());

        let mut out = local;
        out.extend_from_slice(&cen);
        out.extend_from_slice(&eocd);
        out
    }

    #[semio_framework_async_macros::async_test]
    async fn clean_snapshot_composes_and_stamps_iso21320() {
        let snapshot = ZipIso21320Builder::new().with_stored_entry("a.txt", b"hello".to_vec()).build().unwrap();
        let bytes = <ZipSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = ZipIso21320ComposerComposition::compose(&sources).expect("clean archive must compose to iso21320");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn encrypted_wire_archive_composes_to_clean_logical_output() {
        let raw = raw_zip_with_flags(FLAG_ENCRYPTED, 20);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&raw) }];
        let composed = ZipIso21320ComposerComposition::compose(&sources).expect("decode+canonicalize must clear forbidden wire bits");
        let rematerialized = crate::standards::v2_0::subsets::base::io::encode_zip(&composed.snapshot).expect("encode canonical logical archive");
        assert!(check_iso21320_wire_conformance(&rematerialized).iter().all(|d| d.code.0 != CODE_ENCRYPTED));
    }

    #[semio_framework_async_macros::async_test]
    async fn subset_validator_flags_real_violations_without_normalizing() {
        let raw = raw_zip_with_flags(FLAG_ENCRYPTED, 20);
        let diagnostics = ZipIso21320Validator::validate(&IoPayload::Binary(raw)).await;
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ENCRYPTED && d.severity == Severity::Error), "got {diagnostics:?}");
    }
}
