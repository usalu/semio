
use super::*;

/// 🪪️ `artifact_kind().schema` IS `SOURCING_CURATION_SCHEMA`: a document kind has ONE schema identity — the hub's codec rows, document-open targets and genesis, the MCP workspace
/// store and host-media contributions all key on it (ticket 26/09/23 W4: a distinct "media schema" left the package without a
/// codec owner, so the trusted catalog refused it). The former media string stays declared as `source_format`.
#[semio_framework_async_macros::async_test]
async fn artifact_kind_names_the_store_schema() {
    assert_eq!(artifact_kind().schema, SOURCING_CURATION_SCHEMA);
    assert_eq!(artifact_kind().source_format, "sourcing.curation");
}


/// 🏗 DSL txt carrier: serialize then deserialize must restore the snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn txt_dsl_carrier_round_trips_exactly() {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use semio_os_kernel::io::io_mechanism::{Deserializer, Serializer};
    use semio_framework::io_schema::IoPayload;
    let snapshot = crate::CurationSnapshot::default();
    let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 16 << 20, maximum_capacity_bytes: 16 << 20, maximum_release_bytes: 16 << 20, maximum_depth: 512 };
    let (mut receive, mut publish) = (|_| true, |_| true);
    let mut decode = semio_framework_value::NativeDecodeControl::new_retained(&mut receive);
    let mut encode = semio_framework_value::NativeEncodeControl::new_retained(&mut publish);
    let mut control = semio_os_kernel::io::io_mechanism::IoRunControl::new(&mut decode, &mut encode, grant);
    let exported = export::txt::v_utf_8::any::CurationIntoTxt::serialize(&snapshot, &semio_os_kernel::io::io_mechanism::ArchiveChildren::empty(), &mut control).await.expect("dsl txt export");
    let IoPayload::Text(text) = exported.value else { panic!("txt is a text payload") };
    let back = import::txt::v_utf_8::any::TxtIntoCuration::deserialize(&IoPayload::Text(text), &mut control).await.expect("dsl txt import");
    assert_eq!(back.value, snapshot);
}

/// 🧺️ The curated rows the three declared document formats carry: two kinds with distinct counts.
fn curated_snapshot() -> crate::CurationSnapshot {
    let mut snapshot = crate::CurationSnapshot::default();
    snapshot.curated = vec![crate::CuratedItem { object_id: "beam-kvh-c24".into(), count: 2 }, crate::CuratedItem { object_id: "beam-steel-ipe200".into(), count: 1 }];
    snapshot
}

/// 🎛️ The retained grant and run control every io hop of the tests runs under.
fn io_grant() -> semio_framework_value::RetainedCloneGrant {
    semio_framework_value::RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 16 << 20, maximum_capacity_bytes: 16 << 20, maximum_release_bytes: 16 << 20, maximum_depth: 512 }
}

/// 📝️ Text utf-8 export names every curated row and import restores the curated state exactly.
#[semio_framework_async_macros::async_test]
async fn curated_state_round_trips_through_text_utf8() {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use semio_framework::io_schema::IoPayload;
    use semio_os_kernel::io::io_mechanism::{Deserializer, Serializer};
    let snapshot = curated_snapshot();
    let (mut receive, mut publish) = (|_| true, |_| true);
    let mut decode = semio_framework_value::NativeDecodeControl::new_retained(&mut receive);
    let mut encode = semio_framework_value::NativeEncodeControl::new_retained(&mut publish);
    let mut control = semio_os_kernel::io::io_mechanism::IoRunControl::new(&mut decode, &mut encode, io_grant());
    let exported = export::txt::v_utf_8::any::CurationIntoTxt::serialize(&snapshot, &semio_os_kernel::io::io_mechanism::ArchiveChildren::empty(), &mut control).await.expect("txt export");
    let IoPayload::Text(text) = exported.value else { panic!("txt is a text payload") };
    assert!(text.contains("beam-kvh-c24") && text.contains("beam-steel-ipe200"), "the text names the curated rows: {text}");
    let back = import::txt::v_utf_8::any::TxtIntoCuration::deserialize(&IoPayload::Text(text), &mut control).await.expect("txt import");
    assert_eq!(back.value.curated, snapshot.curated);
}

/// 🔣️ JSON rfc8259 export is valid for the third-party parser with the curated rows and import restores them exactly.
#[semio_framework_async_macros::async_test]
async fn curated_state_round_trips_through_json_rfc8259() {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use semio_framework::io_schema::IoPayload;
    use semio_os_kernel::io::io_mechanism::{Deserializer, Serializer};
    let snapshot = curated_snapshot();
    let (mut receive, mut publish) = (|_| true, |_| true);
    let mut decode = semio_framework_value::NativeDecodeControl::new_retained(&mut receive);
    let mut encode = semio_framework_value::NativeEncodeControl::new_retained(&mut publish);
    let mut control = semio_os_kernel::io::io_mechanism::IoRunControl::new(&mut decode, &mut encode, io_grant());
    let exported = export::json::v_rfc8259::any::CurationIntoJson::serialize(&snapshot, &semio_os_kernel::io::io_mechanism::ArchiveChildren::empty(), &mut control).await.expect("json export");
    let IoPayload::Text(text) = exported.value else { panic!("json is a text payload") };
    let oracle: serde_json::Value = serde_json::from_str(&text).expect("serde_json parses the exported rfc8259 document");
    let rows: Vec<(String, u64)> = oracle["curated"].as_array().expect("curated rows").iter().map(|row| (row["objectId"].as_str().expect("objectId").to_owned(), row["count"].as_u64().expect("count"))).collect();
    assert_eq!(rows, vec![("beam-kvh-c24".to_owned(), 2), ("beam-steel-ipe200".to_owned(), 1)]);
    let back = import::json::v_rfc8259::any::JsonIntoCuration::deserialize(&IoPayload::Text(text), &mut control).await.expect("json import");
    assert_eq!(back.value.curated, snapshot.curated);
}

/// 🎒️ ZIP 2.0 export is a real zip container (local file header magic) whose DSL member import restores the curated rows exactly.
#[semio_framework_async_macros::async_test]
async fn curated_state_round_trips_through_zip_2_0() {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use semio_framework::io_schema::IoPayload;
    use semio_os_kernel::io::io_mechanism::{Deserializer, Serializer};
    let snapshot = curated_snapshot();
    let (mut receive, mut publish) = (|_| true, |_| true);
    let mut decode = semio_framework_value::NativeDecodeControl::new_retained(&mut receive);
    let mut encode = semio_framework_value::NativeEncodeControl::new_retained(&mut publish);
    let mut control = semio_os_kernel::io::io_mechanism::IoRunControl::new(&mut decode, &mut encode, io_grant());
    let exported = export::zip::v2_0::any::CurationIntoZip::serialize(&snapshot, &semio_os_kernel::io::io_mechanism::ArchiveChildren::empty(), &mut control).await.expect("zip export");
    let IoPayload::Binary(bytes) = &exported.value else { panic!("zip is a binary payload") };
    assert_eq!(&bytes[..4], b"PK\x03\x04", "a zip container starts with the local file header signature");
    let back = import::zip::v2_0::any::ZipIntoCuration::deserialize(&exported.value, &mut control).await.expect("zip import");
    assert_eq!(back.value.curated, snapshot.curated);
}

/// 🚪️ The committed language-agnostic io fixture the Python `json`, `zipfile` and utf-8 oracle reproduces.
fn io_law() -> serde_json::Value {
    serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🚪️io-curation-1/🔣️.json")).unwrap()
}

/// 🗂️ The real derived timber-kit snapshot every io codec of the fixture carries.
fn io_kit() -> crate::CurationSnapshot {
    semio_framework_pack_json::from_json_str(semio_framework_test_fixtures::fixture_text!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🗂️mutate-curation-1/🔣️.snapshot.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}

/// 📤️ Writes a codec output into the directory `SEMIO_CURATION_IO_OUT` names so the Python oracle can re-read the very bytes Rust produced; a no-op when unset.
fn emit_io_output(name: &str, bytes: &[u8]) {
    if let Some(directory) = std::env::var_os("SEMIO_CURATION_IO_OUT") {
        std::fs::create_dir_all(&directory).unwrap();
        std::fs::write(std::path::Path::new(&directory).join(name), bytes).unwrap();
    }
}

/// 🎒️ The kit archive carries exactly the committed members, the json member parses to the committed shape and sizes, and the DSL member restores the whole kit.
#[semio_framework_async_macros::async_test]
async fn io_fixture_zip_archive_members_match_and_restore_the_kit() {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use semio_framework::io_schema::IoPayload;
    use semio_os_kernel::io::io_mechanism::{Deserializer, Serializer};
    let (law, kit) = (io_law(), io_kit());
    let (mut receive, mut publish) = (|_| true, |_| true);
    let mut decode = semio_framework_value::NativeDecodeControl::new_retained(&mut receive);
    let mut encode = semio_framework_value::NativeEncodeControl::new_retained(&mut publish);
    let mut control = semio_os_kernel::io::io_mechanism::IoRunControl::new(&mut decode, &mut encode, io_grant());
    let exported = export::zip::v2_0::any::CurationIntoZip::serialize(&kit, &semio_os_kernel::io::io_mechanism::ArchiveChildren::empty(), &mut control).await.expect("zip export");
    let IoPayload::Binary(bytes) = &exported.value else { panic!("zip is a binary payload") };
    emit_io_output("archive.zip", bytes);
    let archive = semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::decode_zip(bytes).expect("a real zip 2.0 container");
    let names: Vec<&str> = archive.entries.iter().map(|entry| entry.name.as_str()).collect();
    let zip = &law["zip"]["archive"];
    let member = semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::document_archive_member::<crate::CurationSnapshot>();
    assert_eq!(names, vec![member.as_str(), zip["jsonMember"].as_str().unwrap()]);
    assert!(member.starts_with(zip["dslPrefix"].as_str().unwrap()) && member.ends_with(zip["dslSuffix"].as_str().unwrap()), "{member}");
    let json: serde_json::Value = serde_json::from_slice(&archive.entries[1].data).expect("the json member is rfc8259");
    let mut members: Vec<&str> = json.as_object().unwrap().keys().map(String::as_str).collect();
    members.sort_unstable();
    let mut committed: Vec<&str> = law["json"]["members"].as_array().unwrap().iter().map(|name| name.as_str().unwrap()).collect();
    committed.sort_unstable();
    assert_eq!(members, committed);
    assert_eq!(json["stockExtra"].as_array().unwrap().len() as u64, law["json"]["stock"].as_u64().unwrap());
    assert_eq!(json["curated"].as_array().unwrap().len() as u64, law["json"]["curated"].as_u64().unwrap());
    let back = import::zip::v2_0::any::ZipIntoCuration::deserialize(&exported.value, &mut control).await.expect("zip import");
    assert_eq!(back.value, kit);
}

/// 📝️ The utf-8 text form restores the kit and every committed sample text exactly, and the committed malformed sequences are refused as text.
#[semio_framework_async_macros::async_test]
async fn io_fixture_text_utf8_round_trips_the_kit_and_every_sample() {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use semio_framework::io_schema::IoPayload;
    use semio_os_kernel::io::io_mechanism::{Deserializer, Serializer};
    let (law, kit) = (io_law(), io_kit());
    let (mut receive, mut publish) = (|_| true, |_| true);
    let mut decode = semio_framework_value::NativeDecodeControl::new_retained(&mut receive);
    let mut encode = semio_framework_value::NativeEncodeControl::new_retained(&mut publish);
    let mut control = semio_os_kernel::io::io_mechanism::IoRunControl::new(&mut decode, &mut encode, io_grant());
    let mut samples = vec![kit.clone()];
    for sample in law["text"]["samples"].as_array().unwrap().iter().map(|sample| sample["text"].as_str().unwrap()).filter(|text| !text.chars().any(char::is_control)) {
        let mut snapshot = kit.clone();
        snapshot.stock_extra[0].name = sample.into();
        samples.push(snapshot);
    }
    for (index, snapshot) in samples.into_iter().enumerate() {
        let exported = export::txt::v_utf_8::any::CurationIntoTxt::serialize(&snapshot, &semio_os_kernel::io::io_mechanism::ArchiveChildren::empty(), &mut control).await.expect("txt export");
        let IoPayload::Text(text) = exported.value else { panic!("txt is a text payload") };
        if index == 0 {
            emit_io_output("snapshot.txt", text.as_bytes());
        }
        let back = import::txt::v_utf_8::any::TxtIntoCuration::deserialize(&IoPayload::Text(text), &mut control).await.expect("txt import");
        assert_eq!(back.value, snapshot);
    }
    for malformed in law["text"]["malformed"].as_array().unwrap() {
        let bytes: Vec<u8> = (0..malformed.as_str().unwrap().len()).step_by(2).map(|at| u8::from_str_radix(&malformed.as_str().unwrap()[at..at + 2], 16).unwrap()).collect();
        assert!(std::str::from_utf8(&bytes).is_err(), "{malformed}");
    }
    for sample in law["text"]["samples"].as_array().unwrap() {
        let text = sample["text"].as_str().unwrap();
        assert_eq!((text.len(), text.chars().count(), text.encode_utf16().count()), (sample["utf8Bytes"].as_u64().unwrap() as usize, sample["scalars"].as_u64().unwrap() as usize, sample["utf16Units"].as_u64().unwrap() as usize));
    }
}

/// 🔣️ The rfc8259 json export of the kit parses to the committed members and sizes, keeps the curated order, and import restores the kit.
#[semio_framework_async_macros::async_test]
async fn io_fixture_json_rfc8259_round_trips_the_kit_exactly() {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use semio_framework::io_schema::IoPayload;
    use semio_os_kernel::io::io_mechanism::{Deserializer, Serializer};
    let (law, kit) = (io_law(), io_kit());
    let (mut receive, mut publish) = (|_| true, |_| true);
    let mut decode = semio_framework_value::NativeDecodeControl::new_retained(&mut receive);
    let mut encode = semio_framework_value::NativeEncodeControl::new_retained(&mut publish);
    let mut control = semio_os_kernel::io::io_mechanism::IoRunControl::new(&mut decode, &mut encode, io_grant());
    let exported = export::json::v_rfc8259::any::CurationIntoJson::serialize(&kit, &semio_os_kernel::io::io_mechanism::ArchiveChildren::empty(), &mut control).await.expect("json export");
    let IoPayload::Text(text) = exported.value else { panic!("json is a text payload") };
    emit_io_output("snapshot.json", text.as_bytes());
    let oracle: serde_json::Value = serde_json::from_str(&text).expect("serde_json parses the exported rfc8259 document");
    assert_eq!(oracle["stockExtra"].as_array().unwrap().len() as u64, law["json"]["stock"].as_u64().unwrap());
    let ids: Vec<&str> = oracle["curated"].as_array().unwrap().iter().map(|row| row["objectId"].as_str().unwrap()).collect();
    assert_eq!(ids, kit.curated.iter().map(|item| item.object_id.as_str()).collect::<Vec<_>>());
    let back = import::json::v_rfc8259::any::JsonIntoCuration::deserialize(&IoPayload::Text(text), &mut control).await.expect("json import");
    assert_eq!(back.value, kit);
}
