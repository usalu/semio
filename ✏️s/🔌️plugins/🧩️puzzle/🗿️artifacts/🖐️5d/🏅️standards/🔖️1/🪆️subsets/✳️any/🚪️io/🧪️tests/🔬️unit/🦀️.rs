use crate::standards::v1::subsets::any::io::export::serializers::artifacts::{png::v1_2::any as png_out, txt::v_utf_8::any as txt_out, zip::v2_0::any as zip_out};
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::{txt::v_utf_8::any as txt_in, zip::v2_0::any as zip_in};
use crate::Puzzle5dSnapshot;
use std::io::Read;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assembly() -> Puzzle5dSnapshot {
    let mut assembly = Puzzle5dSnapshot { label: Some("frame".into()), ..Puzzle5dSnapshot::default() };
    assembly.domain = "furniture".into();
    assembly
}

/// 🔮️ The third-party `zip` reader (test-only) opens the archive and finds the DSL member.
#[test]
fn zip_is_a_real_archive_of_the_exact_document() {
    let bytes = zip_out::serialize_bytes(&assembly()).expect("zip export");
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes.clone())).expect("the zip crate opens the archive");
    let mut member = String::new();
    archive.by_name(&semio_s_artifact_stdio_zip::io::document_archive_member::<Puzzle5dSnapshot>()).expect("dsl member").read_to_string(&mut member).expect("utf-8 member");
    assert_eq!(member, <Puzzle5dSnapshot as store::ArtifactDsl>::print_dsl(&assembly()));
    assert!(archive.by_name("snapshot.json").is_ok());
    assert_eq!(zip_in::deserialize_bytes(&bytes).expect("zip import"), assembly());
}

#[test]
fn png_draws_the_board() {
    let png = semio_s_artifact_stdio_png::io::decode_png(&png_out::serialize_bytes(&assembly()).expect("png")).expect("decodes as png");
    assert!(png.width >= 64 && png.height >= 64);
}

#[test]
fn txt_is_the_exact_dsl_carrier() {
    let bytes = txt_out::serialize_bytes(&assembly()).expect("txt export");
    assert_eq!(txt_in::deserialize_bytes(&bytes).expect("txt import"), assembly());
}
