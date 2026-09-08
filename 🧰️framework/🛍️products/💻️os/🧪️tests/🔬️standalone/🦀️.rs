#[test]
fn every_path_mount_in_this_glue_resolves_to_an_existing_file() {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = include_str!("../../📦️packages/🦀️rust/🦀️.rs");
    let mut missing = Vec::new();
    for line in source.lines() {
        let Some(rest) = line.trim().strip_prefix("#[path = \"") else { continue };
        let Some(target) = rest.split('"').next() else { continue };
        if target == "." {
            continue;
        }
        if !here.join(target).exists() {
            missing.push(target.to_string());
        }
    }
    assert!(missing.is_empty(), "glue.rs mounts files that do not exist: {missing:?}");
}
