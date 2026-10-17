//! 🧪️ Laws of the wasip2 guest delivery: the declarations match the shared fixture and the files on disk, and a process that registers a
//! package asset reader installs the guest assets lazily, only once the host delivered all of them. One process, one test: the port installs once.

use semio_framework_canvas_assets::{compiler_fonts, emoji_font, guest_asset_declarations, icon_pack, label_font, register_guest_asset_reader, RuntimeAsset};
use semio_framework_test_fixtures::fixture_path;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

const FIXTURE: &str = include_str!("../🧫️fixtures/🔣️.json");

static DELIVERED: Mutex<Vec<(String, &'static [u8])>> = Mutex::new(Vec::new());

fn deliver(name: &str, bytes: &'static [u8]) {
    DELIVERED.lock().expect("delivery ledger").push((name.to_string(), bytes));
}

fn delivered(name: &str) -> Option<&'static [u8]> {
    DELIVERED.lock().expect("delivery ledger").iter().find(|(existing, _)| existing == name).map(|(_, bytes)| *bytes)
}

fn repository_root() -> PathBuf {
    fixture_path!("../../../../../..")
}

fn read_asset(asset: RuntimeAsset) -> &'static [u8] {
    Box::leak(fs::read(repository_root().join(asset.package_asset_name())).expect("asset file").into_boxed_slice())
}

#[test]
fn the_declarations_match_the_fixture_and_the_files_and_the_port_installs_lazily_once_the_host_delivered_everything() {
    let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("fixture");
    let rows = fixture["assets"].as_array().expect("assets");
    let by_role: HashMap<&str, &serde_json::Value> = rows.iter().map(|row| (row["role"].as_str().expect("role"), row)).collect();
    let guest_roles: Vec<&str> = fixture["guestAssets"].as_array().expect("guest assets").iter().map(|role| role.as_str().expect("role")).collect();
    let declarations = guest_asset_declarations();
    assert_eq!(declarations.len(), guest_roles.len());
    assert_eq!(declarations.iter().map(|declaration| declaration.name.clone()).collect::<Vec<_>>(), RuntimeAsset::GUEST.iter().map(|asset| asset.package_asset_name()).collect::<Vec<_>>());
    for (role, declaration) in guest_roles.iter().zip(&declarations) {
        let row = by_role[role];
        let file = repository_root().join(&declaration.name);
        let channel = fixture["channels"].as_array().expect("channels").iter().find(|channel| channel["id"] == row["channel"]).expect("channel");
        assert_eq!(declaration.name, format!("{}/{}", channel["directory"].as_str().expect("directory"), row["file"].as_str().expect("file")));
        let bytes = fs::read(&file).expect("asset file");
        assert_eq!(declaration.size_bytes, bytes.len() as u64, "{role}");
        assert_eq!(declaration.sha256, semio_framework_hash::sha256_hex(&bytes), "{role}: the declared digest is the file's digest");
        if let Some(expected) = row["sha256"].as_str() {
            assert_eq!(declaration.sha256, expected, "{role}: the declared digest is the fixture's independently computed digest");
        }
    }
    let digest_path = repository_root().join(RuntimeAsset::IconPack.repository_directory()).join(fixture["iconPackDigestFile"].as_str().expect("digest file"));
    let digest: serde_json::Value = serde_json::from_slice(&fs::read(digest_path).expect("pack digest")).expect("pack digest json");
    let pack = declarations.last().expect("pack declaration");
    assert_eq!((pack.size_bytes, pack.sha256.as_str()), (digest["sizeBytes"].as_u64().expect("size"), digest["sha256"].as_str().expect("sha")), "the builder's digest, computed by another implementation, is the declared one");

    assert!(emoji_font().is_none() && label_font().is_none() && icon_pack().is_none(), "nothing resolves before a reader is registered");
    register_guest_asset_reader(delivered);
    assert!(icon_pack().is_none() && emoji_font().is_none(), "a reader that was delivered nothing keeps the port empty");
    deliver(&RuntimeAsset::GUEST[0].package_asset_name(), read_asset(RuntimeAsset::GUEST[0]));
    deliver(&RuntimeAsset::GUEST[1].package_asset_name(), read_asset(RuntimeAsset::GUEST[1]));
    assert!(icon_pack().is_none(), "a partial delivery installs nothing");
    deliver(&RuntimeAsset::GUEST[2].package_asset_name(), read_asset(RuntimeAsset::GUEST[2]));

    let pack = icon_pack().expect("the complete delivery installs on first use");
    assert!(pack.plain_len() > 100 && pack.themed_len() > 0);
    assert_eq!(pack.plain("search").expect("catalog icon"), fs::read_to_string(repository_root().join("🧰️framework/🔨️modules/🖼️assets/🔣️icons/🔎️viewing/🔍️search.svg")).expect("source"));
    assert_eq!(emoji_font().expect("emoji face").len(), 152308);
    assert_eq!(label_font().expect("label face").len(), 26772);
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    assert!(compiler_fonts().is_some(), "the compiler keeps estimating text sizes without glyphs");
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    assert!(compiler_fonts().is_none(), "a guest delivery carries no parseable math, serif or mono face, so a glyph-painting target keeps none installed");
}
