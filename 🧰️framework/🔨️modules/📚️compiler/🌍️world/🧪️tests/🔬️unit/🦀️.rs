use super::*;
use semio_framework_pack_json::{JsonMemberPolicy, Value};
use semio_framework_test_fixtures::fixture_path;
use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::OnceLock,
};

fn font_directory() -> PathBuf {
    fixture_path!("../../🌍️world/🔤️fonts")
}

pub(crate) fn test_fonts() -> FontSet<'static> {
    static FONTS: OnceLock<FontSet<'static>> = OnceLock::new();
    *FONTS.get_or_init(|| FontSet::from_directory(&font_directory()).expect("compiler font assets must be readable from the repository"))
}

fn fixture() -> Value {
    semio_framework_pack_json::parse(include_str!("../🧫️fixtures/🔣️.json"), JsonMemberPolicy::Reject).expect("closed font asset fixture")
}

fn role_of(id: &str) -> FontRole {
    match id {
        "math" => FontRole::Math,
        "serif" => FontRole::Serif,
        "serif-italic" => FontRole::SerifItalic,
        "mono" => FontRole::Mono,
        "emoji" => FontRole::Emoji,
        other => panic!("unknown font role {other:?}"),
    }
}

fn sfnt_tag(hex: &str) -> [u8; 4] {
    match hex {
        "OTTO" => *b"OTTO",
        "00010000" => [0x00, 0x01, 0x00, 0x00],
        other => panic!("unknown sfnt tag {other:?}"),
    }
}

#[test]
fn font_assets_loaded_through_the_port_match_the_shared_fixture() {
    let fixture = fixture();
    let fonts = test_fonts();
    let rows = fixture["fonts"].as_array().unwrap();
    assert_eq!(rows.len(), FontRole::ALL.len());
    for row in rows {
        let role = role_of(row["role"].as_str().unwrap());
        assert_eq!(role.asset_file_name(), row["file"].as_str().unwrap(), "{role:?}");
        let bytes = fonts.get(role);
        assert_eq!(bytes.len() as u64, row["bytes"].as_u64().unwrap(), "{role:?} byte length");
        assert_eq!(bytes[0..4], sfnt_tag(row["sfntTag"].as_str().unwrap()), "{role:?} sfnt tag");
        let on_disk = fs::read(font_directory().join(role.asset_file_name())).unwrap();
        assert_eq!(bytes, on_disk.as_slice(), "{role:?} bytes equal the asset file");
    }
}

#[test]
fn font_set_get_matches_field_by_role() {
    let fonts = test_fonts();
    assert_eq!(fonts.get(FontRole::Math).as_ptr(), fonts.math.as_ptr());
    assert_eq!(fonts.get(FontRole::Serif).as_ptr(), fonts.serif.as_ptr());
    assert_eq!(fonts.get(FontRole::SerifItalic).as_ptr(), fonts.serif_italic.as_ptr());
    assert_eq!(fonts.get(FontRole::Mono).as_ptr(), fonts.mono.as_ptr());
    assert_eq!(fonts.get(FontRole::Emoji).as_ptr(), fonts.emoji.as_ptr());
}

#[test]
fn font_roles_name_distinct_asset_files() {
    let mut names: Vec<&str> = FontRole::ALL.iter().map(|role| role.asset_file_name()).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), FontRole::ALL.len());
}

#[test]
fn try_from_loader_asks_every_role_in_order() {
    let mut asked = Vec::new();
    let set = FontSet::try_from_loader(|role| {
        asked.push(role);
        Ok::<&[u8], ()>(&[1, 2, 3])
    })
    .unwrap();
    assert_eq!(asked, FontRole::ALL);
    for role in FontRole::ALL {
        assert_eq!(set.get(role), [1, 2, 3]);
    }
}

#[test]
fn try_from_loader_stops_at_the_first_failure() {
    let mut asked = Vec::new();
    let failure = FontSet::try_from_loader(|role| {
        asked.push(role);
        if role == FontRole::Serif {
            Err("serif missing")
        } else {
            Ok::<&[u8], &str>(&[0])
        }
    })
    .unwrap_err();
    assert_eq!(failure, "serif missing");
    assert_eq!(asked, [FontRole::Math, FontRole::Serif]);
}

#[test]
fn from_directory_names_the_first_missing_asset_file() {
    let missing = std::env::temp_dir().join(format!("semio-compiler-no-fonts-{}", std::process::id()));
    let error = FontSet::from_directory(&missing).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::NotFound);
    assert!(error.to_string().contains(FontRole::Math.asset_file_name()), "{error}");
}

fn rust_sources_outside_tests(directory: &Path, found: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        if path.is_dir() {
            if !matches!(name.as_str(), "node_modules" | "target") && !name.contains("tests") {
                rust_sources_outside_tests(&path, found);
            }
        } else if name.ends_with(".rs") {
            found.push(path);
        }
    }
}

#[test]
fn compiler_sources_embed_no_font_bytes() {
    let mut sources = Vec::new();
    rust_sources_outside_tests(&fixture_path!("../.."), &mut sources);
    assert!(sources.len() >= 10, "scan must reach the crate sources: {sources:?}");
    let needle = concat!("include_", "bytes!");
    for source in sources {
        let text = fs::read_to_string(&source).unwrap();
        assert!(!text.contains(needle), "{} embeds bytes", source.display());
    }
}
