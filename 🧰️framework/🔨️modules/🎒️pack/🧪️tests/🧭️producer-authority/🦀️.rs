//! 🧪️ The whole Pack owner requires canonical source-authored causes before production.
use semio_framework_pack_error::{PackRefusal,PackRetryDisposition,PackCauseKind};
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_diagnostic::TextError;
use semio_framework_diagnostic::TextSpan;
use semio_framework_value::list::{PagedListError,PagedListAllocationError,PagedListRefusalKind};

fn authored_kind(text:&str)->ValueRefusalKind {
 match text {
  "invalidValue"=>ValueRefusalKind::InvalidValue,"canceled"=>ValueRefusalKind::Canceled,
  "ownershipLimit"=>ValueRefusalKind::OwnershipLimit,"allocationFailed"=>ValueRefusalKind::AllocationFailed,
  "workLimit"=>ValueRefusalKind::WorkLimit,"depthLimit"=>ValueRefusalKind::DepthLimit,
  "unsupportedOwner"=>ValueRefusalKind::UnsupportedOwner,"invariantViolated"=>ValueRefusalKind::InvariantViolated,
  _=>panic!("unknown source-authored kind")
 }
}

#[test]
fn canonical_pack_producer_authority_is_required_and_preserves_owned_causes() {
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../⚠️error/🧫️fixtures/🧭️cause/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let expected=authored_kind(row["expectedKind"].as_str().unwrap());
  let message=row["message"].as_str().unwrap();
  let what="deflate";
  let offset=row["offset"].as_str().unwrap().parse::<u64>().unwrap();
  let mut owned_pointer=None;
  let error=match row["variant"].as_str().unwrap(){
   "BadMagic"=>PackRefusal::BadMagic,
   "UnsupportedVersion"=>PackRefusal::UnsupportedVersion{major:2,minor:3},
   "UnknownRequiredFlags"=>PackRefusal::UnknownRequiredFlags(4),
   "Truncated"=>PackRefusal::Truncated(offset),
   "ChecksumMismatch"=>PackRefusal::ChecksumMismatch{segment:what,offset},
   "ContentHashMismatch"=>PackRefusal::ContentHashMismatch,
   "NonCanonical"=>PackRefusal::NonCanonical("same deliberately misleading cancellation/limit prose"),
   "UnsupportedCodec"=>PackRefusal::UnsupportedCodec(7),
   "LimitExceeded"=>PackRefusal::LimitExceeded{kind:authored_kind(row["kind"].as_str().unwrap()),limit:"same deliberately misleading cancellation/limit prose"},
   "RetainedMalformed"=>PackRefusal::RetainedMalformed{kind:authored_kind(row["kind"].as_str().unwrap()),what,offset,detail:"same deliberately misleading cancellation/limit prose"},
   "RetainedAllocation"=>PackRefusal::RetainedAllocation{kind:authored_kind(row["kind"].as_str().unwrap()),allocated_bytes:usize::try_from(row["allocatedBytes"].as_u64().unwrap()).unwrap(),what,offset,detail:"same deliberately misleading cancellation/limit prose"},
   "Malformed"=>PackRefusal::Malformed{kind:authored_kind(row["kind"].as_str().unwrap()),what,offset,detail:message.to_owned()},
   "ValueRefusal"=>{let cause=ValueError::new(authored_kind(row["kind"].as_str().unwrap()),message);owned_pointer=Some(cause.message.as_ptr());PackRefusal::from(cause)},
   "TextRefusal"=>{let cause=TextError::new(authored_kind(row["kind"].as_str().unwrap()),message,TextSpan{line:2,column:3,length:1});owned_pointer=Some(cause.message.as_ptr());PackRefusal::from(cause)},
   "Io"=>{let cause=ValueError::new(authored_kind(row["kind"].as_str().unwrap()),message);owned_pointer=Some(cause.message.as_ptr());PackRefusal::Io{error:cause,retry:match row["retry"].as_str().unwrap(){"never"=>PackRetryDisposition::Never,"transient"=>PackRetryDisposition::Transient,_=>panic!("unknown source-authored retry policy")}}},
   _=>panic!("unknown authored Pack variant")
  };
  assert_eq!(Some(error.kind()),Some(expected));
  assert_eq!(PackCauseKind::Refusal(error.kind()),PackCauseKind::Refusal(expected));
  let retry=match &error{PackRefusal::Io{retry,..}=>Some(match retry{PackRetryDisposition::Never=>"never",PackRetryDisposition::Transient=>"transient"}),_=>None};
  let allocated_bytes=match &error{PackRefusal::RetainedAllocation{allocated_bytes,..}=>Some(*allocated_bytes),_=>None};
  let actual=serde_json::json!({"kind":Some(error.kind()).map(ValueRefusalKind::as_str),"display":error.to_string(),"retry":retry,"allocatedBytes":allocated_bytes});
  let reference=serde_json::json!({"kind":row["expectedKind"],"display":row["display"],"retry":row["retry"],"allocatedBytes":row["allocatedBytes"]});
  assert_eq!(actual,reference);
  if let Some(pointer)=owned_pointer{
   let source=std::error::Error::source(&error).unwrap();
   match &error{
    PackRefusal::ValueRefusal(cause)|PackRefusal::Io{error:cause,..}=>{assert_eq!(cause.message.as_ptr(),pointer);assert!(std::ptr::eq(source.downcast_ref::<ValueError>().unwrap(),cause));},
    PackRefusal::TextRefusal(cause)=>{assert_eq!(cause.message.as_ptr(),pointer);assert!(std::ptr::eq(source.downcast_ref::<TextError>().unwrap(),cause));},
    _=>panic!("owned source was erased")
   }
  }
  if let PackRefusal::RetainedAllocation{allocated_bytes,..}=&error{assert_eq!(*allocated_bytes as u64,row["allocatedBytes"].as_u64().unwrap());}
 }
 eprintln!("[DEBUG] canonical Pack projected {} source-authored causes",fixture["cases"].as_array().unwrap().len());
}

#[test]
fn canonical_pack_borrowed_factories_preserve_real_paged_metadata() {
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../⚠️error/🧫️fixtures/🧭️cause/📋️paged/🔣️.json")).unwrap();
 let reason="same deliberately misleading cancellation/limit prose";
 for row in fixture["cases"].as_array().unwrap(){
  let kind=match row["kind"].as_str().unwrap(){
   "ownershipLimit"=>PagedListRefusalKind::OwnershipLimit,
   "allocationFailed"=>PagedListRefusalKind::AllocationFailed,
   "invariantViolated"=>PagedListRefusalKind::InvariantViolated,
   _=>panic!("unknown lower source kind")
  };
  let error=match row["factory"].as_str().unwrap(){
   "refusal"=>PackRefusal::from_paged_refusal(PagedListError{kind,reason},"paged",71),
   "allocation"=>PackRefusal::from_paged_allocation(PagedListAllocationError{kind,reason,allocated_bytes:usize::try_from(row["allocatedBytes"].as_u64().unwrap()).unwrap()},"paged",71),
   _=>panic!("unknown source factory")
  };
  assert_eq!(Some(error.kind()),Some(authored_kind(row["expectedKind"].as_str().unwrap())));
  let actual_bytes=match &error{
   PackRefusal::RetainedMalformed{what,offset,detail,..}=>{assert_eq!((*what,*offset),("paged",71));assert!(std::ptr::eq(*detail,reason));None},
   PackRefusal::RetainedAllocation{what,offset,detail,allocated_bytes,..}=>{assert_eq!((*what,*offset),("paged",71));assert!(std::ptr::eq(*detail,reason));Some(*allocated_bytes)},
   _=>panic!("borrowed producer metadata was erased")
  };
  assert_eq!(serde_json::json!({"kind":Some(error.kind()).map(ValueRefusalKind::as_str),"allocatedBytes":actual_bytes}),serde_json::json!({"kind":row["expectedKind"],"allocatedBytes":row["allocatedBytes"]}));
  assert!(std::error::Error::source(&error).is_none());
 }
 eprintln!("[DEBUG] canonical Pack borrowed factories checked 9 real PagedList metadata cases");
}
