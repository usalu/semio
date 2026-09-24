use super::*;
use crate::artifact_authority::trusted_catalog::schema::TrustedPluginModuleIndexV1;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🧫️fixtures/🧩️plugin-module/🔣️.json")).expect("plugin module fixture")
}

fn source(fixture: &serde_json::Value) -> TrustedPluginModuleSourceV1<'_> {
    let record = &fixture["record"];
    TrustedPluginModuleSourceV1 {
        plugin_id: record["pluginId"].as_str().unwrap(),
        package_id: record["packageId"].as_str().unwrap(),
        version: record["version"].as_str().unwrap(),
        component_sha256: record["componentSha256"].as_str().unwrap(),
        descriptor_byte_sha256: record["descriptorByteSha256"].as_str().unwrap(),
    }
}

fn hex_bytes(value: &str) -> Vec<u8> {
    (0..value.len()).step_by(2).map(|index| u8::from_str_radix(&value[index..index + 2], 16).unwrap()).collect()
}

#[test]
fn every_manifest_case_is_accepted_exactly_when_the_fixture_says_so() {
    let fixture = fixture();
    for case in fixture["cases"].as_array().unwrap() {
        let accepted = serde_json::from_value::<TrustedPluginModuleBundleV1>(case["manifest"].clone()).map_err(catalog_error).and_then(|bundle| validate_plugin_module_bundle(&bundle, source(&fixture))).is_ok();
        assert_eq!(accepted, case["accepted"].as_bool().unwrap(), "{}", case["id"]);
    }
}

#[test]
fn only_the_canonical_manifest_bytes_decode_and_both_digests_agree_with_the_fixture() {
    let fixture = fixture();
    let canonical = fixture["canonical"]["bytesUtf8"].as_str().unwrap().as_bytes();
    let decoded = decode_plugin_module_bundle(canonical, source(&fixture)).expect("canonical manifest");
    assert_eq!(serde_json::to_value(&decoded).unwrap(), fixture["cases"][0]["manifest"]);
    let (sha256, blake3) = digests(canonical);
    assert_eq!(hex_lower(&sha256), fixture["canonical"]["sha256"].as_str().unwrap());
    assert_eq!(hex_lower(&blake3), fixture["canonical"]["blake3"].as_str().unwrap());
    assert!(decode_plugin_module_bundle(fixture["canonical"]["pretty"].as_str().unwrap().as_bytes(), source(&fixture)).is_err(), "pretty manifest bytes are not canonical");
    assert!(decode_plugin_module_bundle(&canonical[..canonical.len() - 1], source(&fixture)).is_err(), "manifest without its newline is not canonical");
}

#[test]
fn every_listed_file_verifies_against_its_own_bytes_and_tampering_is_refused() {
    let fixture = fixture();
    let bundle: TrustedPluginModuleBundleV1 = serde_json::from_value(fixture["cases"][0]["manifest"].clone()).unwrap();
    for file in &bundle.files {
        let bytes = hex_bytes(fixture["contents"][&file.path].as_str().unwrap());
        let (sha256, blake3) = digests(&bytes);
        verify_plugin_module_file(file, bytes.len(), sha256, blake3).unwrap_or_else(|error| panic!("{}: {error}", file.path));
        assert_eq!(plugin_module_blob_path(&file.sha256), format!("plugin-modules/{}", file.sha256));
    }
    for case in fixture["fileCases"].as_array().unwrap() {
        let file = bundle.files.iter().find(|file| file.path == case["path"].as_str().unwrap()).unwrap();
        let bytes = hex_bytes(case["contentHex"].as_str().unwrap());
        let (sha256, blake3) = digests(&bytes);
        assert_eq!(verify_plugin_module_file(file, bytes.len(), sha256, blake3).is_ok(), case["accepted"].as_bool().unwrap(), "{}", case["id"]);
    }
}

#[test]
fn every_index_case_is_accepted_exactly_when_the_fixture_says_so() {
    for case in fixture()["index"].as_array().unwrap() {
        let accepted = serde_json::from_value::<TrustedPluginModuleIndexV1>(case["index"].clone()).map_err(catalog_error).and_then(|index| validate_plugin_module_index(&index)).is_ok();
        assert_eq!(accepted, case["accepted"].as_bool().unwrap(), "{}", case["id"]);
    }
}

#[test]
fn every_file_is_served_with_the_fixture_media_type() {
    for row in fixture()["contentTypes"].as_array().unwrap() {
        assert_eq!(plugin_module_content_type(row["path"].as_str().unwrap()), row["contentType"].as_str().unwrap(), "{}", row["path"]);
    }
}

#[test]
fn an_assembled_module_is_the_fixture_manifest_and_writes_content_addressed_files() {
    let fixture = fixture();
    let files = fixture["contents"].as_object().unwrap().iter().map(|(path, hex)| (path.clone(), hex_bytes(hex.as_str().unwrap()))).collect::<Vec<_>>();
    let assembled = assemble_plugin_module_bundle(source(&fixture), "🗒️note", &files).expect("assembled fixture module");
    assert_eq!(serde_json::to_value(&assembled).unwrap(), fixture["cases"][0]["manifest"]);
    let root = crate::test_artifact_root::test_artifact_root().join(format!("plugin-module-write-{}", std::process::id()));
    let record = write_plugin_module_bundle(&root, "packages/note/plugin-module.json", source(&fixture), "🗒️note", &files).expect("written fixture module");
    assert_eq!(record.sha256, fixture["canonical"]["sha256"].as_str().unwrap());
    assert_eq!(std::fs::read(root.join("packages/note/plugin-module.json")).unwrap(), fixture["canonical"]["bytesUtf8"].as_str().unwrap().as_bytes());
    for file in &assembled.files {
        assert_eq!(std::fs::read(root.join(plugin_module_blob_path(&file.sha256))).unwrap(), hex_bytes(fixture["contents"][&file.path].as_str().unwrap()));
    }
    std::fs::remove_dir_all(root).unwrap();
}
