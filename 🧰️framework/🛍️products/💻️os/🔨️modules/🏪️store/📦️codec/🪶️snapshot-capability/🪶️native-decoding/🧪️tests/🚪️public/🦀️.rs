use semio_framework_os_kernel::*;
use semio_framework_artifact_reference::{ArtifactDialect,ArtifactRef};
extern crate semio_framework_os_kernel as dsl;
pub use semio_framework_os_kernel as store;
#[path = "../../../../../../../../../🔨️modules/🌱️value/🛬️decode/🧪️tests/🪆️binding/🦀️.rs"]
mod native_binding;
pub use semio_framework_os_kernel::{io_schema, os_dsl, sqlite_snapshot};
pub use semio_framework_os_kernel::{io, os_pack};
pub use semio_framework_os_kernel::os_pack::codec;

/// 📥️ Reads a committed language-agnostic fixture at run time, by its path relative to the calling test file (`file!()`), so the test closure stays inside the source gate instead of embedding the fixtures with `include_str!`. The workspace root is the manifest ancestor that contains the `file!()` path.
/// @see <https://doc.rust-lang.org/std/macro.include_str.html>
pub(crate) fn fixture_json(source: &str, relative: &str) -> serde_json::Value {
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let root = manifest.ancestors().find(|ancestor| ancestor.join(source).is_file()).unwrap_or_else(|| panic!("test file {source} below {}", manifest.display()));
    let path = root.join(source).parent().unwrap().join(relative);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display()))
}

#[global_allocator]
static TEST_ALLOCATION_OBSERVER:semio_framework_trace_memory_testing::RequestedAllocator=semio_framework_trace_memory_testing::RequestedAllocator;

#[path = "../🦀️.rs"]
mod decoding;
#[path = "../../../🪶️native-encoding/🧪️tests/🦀️.rs"]
mod encoding;
#[path = "../../../🪶️native-retirement/🧪️tests/🦀️.rs"]
mod retirement;

#[path = "../../../../../../🧬️semio/🧪️tests/🚦️controlled/🦀️.rs"]
mod envelope;

#[path = "../../../../../../🗣️dsl/🧬️schema/🧪️tests/🛬️decoding/🦀️.rs"]
mod controlled_text;

#[test]
fn sqlite_snapshot_link_controlled_binding_preserves_all_pin_branches_and_cancels_before_copies(){
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    use semio_framework_dsl_record::DslField;
use semio_framework_dsl_record::FieldValue;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

    let references:serde_json::Value=crate::fixture_json(file!(), "../../../../../../../../../🔨️modules/🚪️io/📝️text/🗿️artifact-reference/🪆️binding/🧪️tests/🧫️fixtures/🔣️.json");
    for value in references["references"].as_array().unwrap(){
        let target=semio_framework_artifact_reference::ArtifactRef{artifact_id:value["artifactId"].as_str().unwrap().into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:value["dialect"]["artifactKind"].as_str().unwrap().into(),standard:value["dialect"]["standard"].as_str().unwrap().into(),subset:value["dialect"]["subset"].as_str().unwrap().into()}};
        let expected=store::ArtifactLink{target,pin:store::LinkPin::Head,role:"literal".into()};let field=<store::ArtifactLink as DslField>::to_value(&expected);
        assert_eq!(<store::ArtifactLink as DslField>::from_value(&field).unwrap(),expected);
        assert_eq!(<store::ArtifactLink as DslField>::from_value_controlled(&field,&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(4096,&mut |_|true)).unwrap(),expected);
    }
    let fixture:serde_json::Value=crate::fixture_json(file!(), "../../../../../🧪️tests/🧫️fixtures/🧩️composed-pack-schema/🔣️.json");let model=&fixture["documents"][0]["snapshot"];
    for row in [ &model["cover"], &model["links"][0], &model["links"][1] ]{
        let pin=match row["pin"]["kind"].as_str().unwrap(){"head"=>store::LinkPin::Head,"checkpoint"=>store::LinkPin::Checkpoint{id:row["pin"]["id"].as_str().unwrap().into()},"snapshot"=>store::LinkPin::Snapshot{blob:store::BlobRef{hash:row["pin"]["hash"].as_str().unwrap().into(),size:u64::MAX,media_type:row["pin"]["mediaType"].as_str().unwrap().into()}},_=>unreachable!()};
        let expected=store::ArtifactLink{target:semio_framework_artifact_reference::ArtifactRef::parse_uri(row["target"].as_str().unwrap()).unwrap(),pin,role:row["role"].as_str().unwrap().into()};let field=<store::ArtifactLink as DslField>::to_value(&expected);
        let decoded=<store::ArtifactLink as DslField>::from_value_controlled(&field,&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(4096,&mut |_|true)).unwrap();assert_eq!(decoded,expected);
        let semio_framework_dsl_record::Shape::Record(spec)=<store::ArtifactLink as DslField>::shape()else{unreachable!()};let semio_framework_dsl_record::FieldValue::Record(record)=&field else{unreachable!()};
        let bytes=os_pack::record::encode_document(&(spec.ordinary)(),record,&store::PackEncodeOptions::default()).unwrap();let(decoded_record,report)=os_pack::record::decode_document(&bytes,&(spec.ordinary)(),&store::PackDecodeOptions::default()).unwrap();assert!(!report.schema_drift);let mut decoded_field=semio_framework_dsl_record::FieldValue::Record(decoded_record);
        assert_eq!(<store::ArtifactLink as DslField>::from_value(&decoded_field).unwrap(),expected);assert_eq!(<store::ArtifactLink as DslField>::from_value_controlled(&decoded_field,&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(4096,&mut |_|true)).unwrap(),expected);
        let semio_framework_dsl_record::FieldValue::Record(root)=&mut decoded_field else{unreachable!()};let Some(semio_framework_dsl_record::FieldValue::Record(pin))=root.fields.get_mut(&1)else{unreachable!()};pin.fields.insert(99,semio_framework_dsl_record::FieldValue::Absent);assert!(<store::ArtifactLink as DslField>::from_value_controlled(&decoded_field,&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(4096,&mut |_|true)).is_err());

        assert!(<store::ArtifactLink as DslField>::from_value_controlled(&field,&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1,&mut |_|true)).is_err());
        assert!(<store::ArtifactLink as DslField>::from_value_controlled(&field,&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(4096,&mut |_|false)).is_err());
        let mut malformed=field;let semio_framework_dsl_record::FieldValue::Record(record)=&mut malformed else{unreachable!()};record.fields.insert(99,semio_framework_dsl_record::FieldValue::Text("unknown".into()));assert!(<store::ArtifactLink as DslField>::from_value_controlled(&malformed,&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(4096,&mut |_|true)).is_err());
    }
}

#[test]
fn sqlite_snapshot_reference_controlled_projection_preserves_literal_identity_and_stops_before_ownership(){
    use semio_framework_dsl_record::DslField;
use semio_framework_value::NativeEncodeControl;
    let fixture:serde_json::Value=crate::fixture_json(file!(), "../../../../../../../../../🔨️modules/🚪️io/📝️text/🗿️artifact-reference/🪆️binding/🧪️tests/🧫️fixtures/🔣️.json");
    for value in fixture["references"].as_array().unwrap(){
        let reference=semio_framework_artifact_reference::ArtifactRef{artifact_id:value["artifactId"].as_str().unwrap().into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:value["dialect"]["artifactKind"].as_str().unwrap().into(),standard:value["dialect"]["standard"].as_str().unwrap().into(),subset:value["dialect"]["subset"].as_str().unwrap().into()}};
        let field=<semio_framework_artifact_reference::ArtifactRef as DslField>::to_value_controlled(&reference,&mut semio_framework_value::NativeEncodeControl::new(8192,&mut |_|true)).unwrap();assert_eq!(<semio_framework_artifact_reference::ArtifactRef as DslField>::from_value(&field).unwrap(),reference);
        let mut admit=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(1,&mut admit);assert!(<semio_framework_artifact_reference::ArtifactRef as DslField>::to_value_controlled(&reference,&mut control).is_err());assert_eq!(control.owned_bytes(),0);
        let mut cancel=|_|false;let mut control=semio_framework_value::NativeEncodeControl::new(8192,&mut cancel);assert!(<semio_framework_artifact_reference::ArtifactRef as DslField>::to_value_controlled(&reference,&mut control).is_err());assert_eq!(control.owned_bytes(),0);
        for pin in [store::LinkPin::Head,store::LinkPin::Checkpoint{id:String::new()},store::LinkPin::Snapshot{blob:store::BlobRef{hash:"!/@\0".into(),size:u64::MAX,media_type:String::new()}}]{let expected=store::ArtifactLink{target:reference.clone(),pin,role:"\0 !/@".into()};let field=<store::ArtifactLink as DslField>::to_value_controlled(&expected,&mut semio_framework_value::NativeEncodeControl::new(8192,&mut |_|true)).unwrap();assert_eq!(<store::ArtifactLink as DslField>::from_value(&field).unwrap(),expected);}
    }
}

#[test]
fn sqlite_snapshot_reference_controlled_value_preserves_derived_literal_fields(){
    let fixture:serde_json::Value=crate::fixture_json(file!(), "../../../../../../../../../🔨️modules/🚪️io/📝️text/🗿️artifact-reference/🪆️binding/🧪️tests/🧫️fixtures/🔣️.json");
    for value in fixture["references"].as_array().unwrap(){let reference=semio_framework_artifact_reference::ArtifactRef{artifact_id:value["artifactId"].as_str().unwrap().into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:value["dialect"]["artifactKind"].as_str().unwrap().into(),standard:value["dialect"]["standard"].as_str().unwrap().into(),subset:value["dialect"]["subset"].as_str().unwrap().into()}};let encoded=<semio_framework_artifact_reference::ArtifactRef as semio_framework_value::ToValue>::to_value_controlled(&reference,&mut semio_framework_value::NativeEncodeControl::new(8192,&mut |_|true)).unwrap();assert_eq!(<semio_framework_artifact_reference::ArtifactRef as semio_framework_value::FromValue>::from_value(encoded).unwrap(),reference);let mut admit=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(1,&mut admit);assert!(<semio_framework_artifact_reference::ArtifactRef as semio_framework_value::ToValue>::to_value_controlled(&reference,&mut control).is_err());assert_eq!(control.owned_bytes(),0);}
}



#[path = "../../../🧪️tests/💰️allocation/🦀️.rs"]
mod native_allocation_bridge;

#[path = "../../../../../../🚪️io/🧪️tests/🪶️transfer/🦀️.rs"]
mod transfer_allocation;
