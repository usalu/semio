use super::*;

fn declaration(name: &str, bytes: &[u8]) -> AssetDeclaration {
    AssetDeclaration {
        name: name.to_string(),
        media_type: semio_framework::MediaType { class: semio_framework::MediaClass::Text, form: semio_framework::MediaForm::Document },
        size_bytes: bytes.len() as u64,
        sha256: semio_framework_hash::sha256_hex(bytes),
    }
}

fn scratch(label: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("semio-package-assets-{label}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("fonts")).expect("scratch directory");
    root
}

#[test]
fn a_declared_file_asset_is_delivered_with_its_exact_bytes_in_declaration_order() {
    let root = scratch("delivered");
    std::fs::write(root.join("fonts/a.ttf"), b"first").expect("write");
    std::fs::write(root.join("fonts/b.ttf"), b"second").expect("write");
    let declared = [declaration("fonts/b.ttf", b"second"), declaration("fonts/a.ttf", b"first")];
    assert_eq!(package_file_assets(&root, &declared), Ok(vec![("fonts/b.ttf".to_string(), b"second".to_vec()), ("fonts/a.ttf".to_string(), b"first".to_vec())]));
}

#[test]
fn a_declared_name_that_is_no_file_below_the_root_is_skipped() {
    let root = scratch("skipped");
    std::fs::write(root.join("fonts/a.ttf"), b"first").expect("write");
    let declared = [declaration("📚️examples/s.puzzle.5d.1.-/body.json", b"{}"), declaration("../outside.bin", b"x"), declaration("/etc/hosts", b"x"), declaration("fonts/a.ttf", b"first")];
    assert_eq!(package_file_assets(&root, &declared), Ok(vec![("fonts/a.ttf".to_string(), b"first".to_vec())]));
}

#[test]
fn a_file_that_differs_from_its_declaration_is_refused_by_name() {
    let root = scratch("refused");
    std::fs::write(root.join("fonts/a.ttf"), b"first").expect("write");
    let wrong_size = AssetDeclaration { size_bytes: 99, ..declaration("fonts/a.ttf", b"first") };
    assert!(package_file_assets(&root, &[wrong_size]).expect_err("size mismatch").contains("fonts/a.ttf"));
    let wrong_hash = AssetDeclaration { sha256: semio_framework_hash::sha256_hex(b"other"), ..declaration("fonts/a.ttf", b"first") };
    assert!(package_file_assets(&root, &[wrong_hash]).expect_err("hash mismatch").contains("hashes to"));
}

#[test]
fn a_module_descriptor_beside_the_wasm_names_the_assets_a_host_delivers() {
    let root = scratch("beside");
    std::fs::write(root.join("📋️project.json"), b"{}").expect("write");
    std::fs::write(root.join("package.json"), br#"{"workspaces":[]}"#).expect("write");
    std::fs::create_dir_all(root.join("modules/layout")).expect("module directory");
    std::fs::write(root.join("fonts/a.ttf"), b"first").expect("write");
    let descriptor = serde_json::json!({ "packageId": "p", "assets": [declaration("fonts/a.ttf", b"first")] });
    std::fs::write(root.join("modules/layout/🔣️.json"), descriptor.to_string()).expect("write descriptor");
    assert_eq!(package_assets_beside_module(&root.join("modules/layout/layout.wasm")), Ok(vec![("fonts/a.ttf".to_string(), b"first".to_vec())]));
    assert_eq!(package_assets_beside_module(&root.join("modules/none/none.wasm")), Ok(Vec::new()), "a module without a descriptor delivers nothing");
    assert_eq!(repository_root_above(&root.join("modules/layout")), Some(root));
}
