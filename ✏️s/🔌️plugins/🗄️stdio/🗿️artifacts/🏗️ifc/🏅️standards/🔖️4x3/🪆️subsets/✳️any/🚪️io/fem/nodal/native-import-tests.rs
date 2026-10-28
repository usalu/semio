//! 🧪️ Component identities reserve their complete fixed stack-copy frontier before initialization.
use super::*;
use semio_framework_value::{NativeDecodeControl,RetainedCloneGrant};
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;
#[test]
fn component_identity_stack_copy_denial_precedes_every_initialized_word(){
    let cases:serde_json::Value=serde_json::from_str(include_str!("import-cases.json")).unwrap();
    assert_eq!(COMPONENT_KEY_STACK_BYTES,cases["componentIdentity"]["maximumStackCopyBytes"].as_u64().unwrap()as usize);
    let grant=RetainedCloneGrant{maximum_items:1000,maximum_copy_bytes:cases["componentIdentity"]["deniedCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:64};
    let mut observe=|_|true;
    let mut recipient=NativeDecodeRetirementRecipient::new();
    let mut native=NativeDecodeControl::new(1048576,&mut observe);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut original=Owner::new(&mut native,grant);
    let error=component_id(u64::MAX,load_query::Component::MomentZ,&mut original).unwrap_err();
    assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);
    assert_eq!(original.progress().copied_items,0);
    assert_eq!(original.progress().copied_bytes,0);
    assert_eq!(original.progress().retained_capacity_bytes,0);
    assert_eq!(original.remaining_grant().maximum_copy_bytes,grant.maximum_copy_bytes);
    drop(original);
    assert!(!native.has_retirement_owner());
}
