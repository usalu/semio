
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
