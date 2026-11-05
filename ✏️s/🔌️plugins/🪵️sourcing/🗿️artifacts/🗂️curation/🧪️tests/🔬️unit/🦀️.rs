
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
