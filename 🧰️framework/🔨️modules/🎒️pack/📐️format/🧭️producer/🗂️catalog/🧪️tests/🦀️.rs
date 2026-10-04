//! 🗂️ Unmounted actual retained Catalog source metadata and constructor laws.
use semio_framework_pack_error::PackRefusal;
use super::*;
use semio_framework_value::ValueRefusalKind;
use semio_framework_value::list::{PagedListError,PagedListAllocationError,PagedListRefusalKind};

#[test]
fn retained_catalog_factories_preserve_actual_source_kind_reason_and_allocated_witness(){
 const REASON:&str="same deliberately misleading canceled/work/allocation prose";
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let kind=match row["cause"]["kind"].as_str().unwrap(){"invalidValue"=>ValueRefusalKind::InvalidValue,"canceled"=>ValueRefusalKind::Canceled,"ownershipLimit"=>ValueRefusalKind::OwnershipLimit,"allocationFailed"=>ValueRefusalKind::AllocationFailed,"workLimit"=>ValueRefusalKind::WorkLimit,"depthLimit"=>ValueRefusalKind::DepthLimit,"unsupportedOwner"=>ValueRefusalKind::UnsupportedOwner,"invariantViolated"=>ValueRefusalKind::InvariantViolated,_=>panic!("unknown closed native catalog kind")};
  assert_eq!(row["cause"]["reason"].as_str().unwrap(),REASON);
  let fault=match row["cause"]["type"].as_str().unwrap(){
   "value"=>RetainedPackCatalogFault{cause:RetainedPackCatalogCause::Value{kind,reason:REASON},code:"retained-pack.catalog-closed",offset:7},
   "paged"|"allocation"=>{
    let source_kind=match kind{ValueRefusalKind::OwnershipLimit=>PagedListRefusalKind::OwnershipLimit,ValueRefusalKind::AllocationFailed=>PagedListRefusalKind::AllocationFailed,ValueRefusalKind::InvariantViolated=>PagedListRefusalKind::InvariantViolated,_=>panic!("unexpected closed PagedList kind")};
    if row["cause"]["type"]=="paged"{RetainedPackCatalogFault::from_paged_refusal(PagedListError{kind:source_kind,reason:REASON},"retained-pack.catalog-closed",7)}else{RetainedPackCatalogFault::from_paged_allocation(PagedListAllocationError{kind:source_kind,reason:REASON,allocated_bytes:row["cause"]["allocatedBytes"].as_u64().unwrap() as usize},"retained-pack.catalog-closed",7)}
   },
   _=>panic!("unknown closed catalog cause"),
  };
  assert_eq!(fault.kind(),kind);assert_eq!(fault.code,"retained-pack.catalog-closed");
  let error=fault.into_pack_refusal("retained-record-body-symbol");assert_eq!(Some(error.kind()),Some(kind));
  match error{
   PackRefusal::RetainedMalformed{kind:actual,what,offset,detail}=>{assert!(row["expected"]["allocatedBytes"].is_null());assert_eq!(actual,kind);assert_eq!(what,"retained-record-body-symbol");assert_eq!(offset,7);assert_eq!(detail.as_ptr(),REASON.as_ptr());},
   PackRefusal::RetainedAllocation{kind:actual,allocated_bytes,what,offset,detail}=>{assert_eq!(allocated_bytes,row["expected"]["allocatedBytes"].as_u64().unwrap() as usize);assert_eq!(actual,kind);assert_eq!(what,"retained-record-body-symbol");assert_eq!(offset,7);assert_eq!(detail.as_ptr(),REASON.as_ptr());},
   _=>panic!("borrowed Catalog cause erased"),
  }
 }
 assert!(RetainedPackSymbolTable::try_new(0,4,4,4096).map(|mut symbols|{while symbols.close_step(64,4096).unwrap()!=RetainedPackCloseStep::Complete{}}).is_ok());
 let fault=RetainedPackSymbolTable::try_new(RETAINED_PACK_MAXIMUM_SYMBOL_SPANS+1,4,4,4096).err().unwrap();assert_eq!(fault.kind(),ValueRefusalKind::OwnershipLimit);
 let mut symbols=RetainedPackSymbolTable::try_new(4,4,4,0).unwrap();assert_eq!(symbols.next_symbol_allocation_bytes(1,0).err().unwrap().kind(),ValueRefusalKind::OwnershipLimit);while symbols.close_step(64,4096).unwrap()!=RetainedPackCloseStep::Complete{}
 let mut symbols=RetainedPackSymbolTable::try_new(1,4,4,4096).unwrap();
 assert_eq!(symbols.next_symbol_allocation_bytes(2,0).err().unwrap().kind(),ValueRefusalKind::WorkLimit);while symbols.close_step(64,4096).unwrap()!=RetainedPackCloseStep::Complete{}
 let mut symbols=RetainedPackSymbolTable::try_new(1,4,4,4096).unwrap();while symbols.close_step(64,4096).unwrap()!=RetainedPackCloseStep::Complete{}
 assert_eq!(symbols.next_symbol_allocation_bytes(1,0).err().unwrap().kind(),ValueRefusalKind::InvariantViolated);
 let mut varint=RetainedVarintCursor::default();for offset in 40..49{assert!(matches!(varint.admit(0x80,offset),Ok(RetainedVarintStep::Pending)));}
 let fault=varint.admit(0x80,49).err().unwrap().into_catalog_fault("retained-pack.catalog-manifest");assert_eq!(fault.offset,40);assert_eq!(fault.kind(),ValueRefusalKind::InvalidValue);assert!(matches!(fault.cause,RetainedPackCatalogCause::Value{reason:"overlong retained varint",..}));
 let mut utf8=RetainedUtf8Cursor::default();assert_eq!(utf8.admit(0xc2,70).unwrap(),None);let fault=utf8.admit(0x20,71).err().unwrap();assert_eq!(fault.kind(),ValueRefusalKind::InvalidValue);assert_eq!(fault.offset,71);
 let mut symbols=RetainedPackSymbolTable::try_new(1,4,4,4096).unwrap();assert_eq!(symbols.symbol_chars(9).err().unwrap().kind(),ValueRefusalKind::InvalidValue);while symbols.close_step(64,4096).unwrap()!=RetainedPackCloseStep::Complete{}
 let mut catalog=RetainedPackCatalogCursor::try_new(PackLimits::default(),1,4,4,1,4096).unwrap();let segment=RetainedPackSegmentHeader{offset:80,kind:crate::KIND_SYMBOLS,flags:0,stored_len:0,raw_len:0,payload_offset:83};catalog.process_event(RetainedPackSegmentEvent::Begin(segment)).unwrap();assert_eq!(catalog.process_event(RetainedPackSegmentEvent::Begin(segment)).err().unwrap().kind(),ValueRefusalKind::InvariantViolated);while catalog.close_step(64,4096).unwrap()!=RetainedPackCloseStep::Complete{};assert_eq!(catalog.grant().err().unwrap().kind(),ValueRefusalKind::InvariantViolated);
 eprintln!("[DEBUG] native Catalog metadata and real constructor/lifecycle causes retained independent authority");
}
