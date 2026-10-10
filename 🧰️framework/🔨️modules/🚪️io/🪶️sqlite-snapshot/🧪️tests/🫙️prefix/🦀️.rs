use super::*;
use semio_framework_value::{ErasedSnapshotRetirement,retirement::controlled::ControlledRetirement,native_decoding::NativeDecodeRetirementRecipient,retained_clone::{RetainedCloneGrant,RetainedCloneStep}};
use crate::test_allocation::observe_backing;
use std::cell::Cell;

fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:4194304,maximum_release_bytes:4194304,maximum_depth:128}}
fn drain(recipient:&mut NativeDecodeRetirementRecipient)->(usize,usize){
 let(mut total_requested,mut total_released)=(0,0);for _ in 0..10000{if recipient.terminal_is_empty(){return(total_requested,total_released);}let(copy,release,depth)=(recipient.next_copy_byte_demand().unwrap(),recipient.next_release_byte_demand().unwrap(),recipient.next_depth_demand().unwrap());let capacity=recipient.next_capacity_byte_demand(if copy==0{release}else{copy}).unwrap();let full=grant();assert!(copy<=full.maximum_copy_bytes&&release<=full.maximum_release_bytes&&capacity<=full.maximum_capacity_bytes&&depth<=full.maximum_depth);for axis in 0..5{let mut denied=full;match axis{0=>denied.maximum_items=0,1=>{if copy==0{continue;}denied.maximum_copy_bytes=copy-1;},2=>{if capacity==0{continue;}denied.maximum_capacity_bytes=capacity-1;},3=>{if release==0{continue;}denied.maximum_release_bytes=release-1;},_=>{if depth==0{continue;}denied.maximum_depth=depth-1;}}let(before,requested,released)=observe_backing(||recipient.close_step(denied));assert_eq!((requested,released),(0,0),"denied axis {axis}: copy={copy} capacity={capacity} release={release} depth={depth}");if let Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=before{assert_eq!(progress,Default::default());}assert!(!recipient.terminal_is_empty());}
 let(result,requested,released)=observe_backing(||recipient.close_step(full));let progress=match result.unwrap_or_else(|error|panic!("original close copy={copy} capacity={capacity} release={release} depth={depth}: {error}")){RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>progress};assert_eq!(requested,progress.retained_capacity_bytes);assert_eq!(released,progress.released_bytes);assert!(progress.copied_items<=1);total_requested+=requested;total_released+=released;
 }panic!("original recipient failed to reach physical emptiness");
}

#[test]
fn sqlite_original_recipient_denies_missing_occupied_nested_before_progress_or_heap(){
 let events=Cell::new(0);let mut callback=|_|{events.set(events.get()+1);true};let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());
 let(result,requested,released)=observe_backing(||control.with_retirement_owner(SqliteSnapshotPhase::ReconstructSnapshot,64,|_|(Ok::<_,ValueError>(()),None)));
 assert_eq!(result.unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!((events.get(),control.ledger.allocation_bytes,requested,released),(0,0,0,0));
 let mut recipient=NativeDecodeRetirementRecipient::new();let mut other=NativeDecodeRetirementRecipient::new();let recipient_pointer=&recipient as *const _ as usize;control.install_native_retirement_recipient(&mut recipient).unwrap();
 control.with_retirement_owner(SqliteSnapshotPhase::ReconstructSnapshot,0,|control|{
  let prior=control.ledger.allocation_bytes;let events_before=events.get();let(result,requested,released)=observe_backing(||control.with_retirement_owner(SqliteSnapshotPhase::ReconstructSnapshot,64,|_|(Ok::<_,ValueError>(()),None)));assert_eq!(result.unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!((control.ledger.allocation_bytes,events.get(),requested,released),(prior,events_before,0,0));assert_eq!(control.install_native_retirement_recipient(&mut other).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);(Ok::<_,ValueError>(()),None)
 }).unwrap();assert_eq!((&**control.native_retirement.as_ref().unwrap()) as *const _ as usize,recipient_pointer);
 let owner=ControlledRetirement::new(Vec::<u8>::new()).unwrap();control.with_retirement_owner(SqliteSnapshotPhase::ReconstructSnapshot,size_of_val(&owner),|_|(Ok::<_,ValueError>(()),Some(Box::new(owner)))).unwrap();let prior=control.ledger.allocation_bytes;let event=events.get();let(result,requested,released)=observe_backing(||control.with_retirement_owner(SqliteSnapshotPhase::ReconstructSnapshot,64,|_|(Ok::<_,ValueError>(()),None)));assert_eq!(result.unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!((control.ledger.allocation_bytes,events.get(),requested,released),(prior,event,0,0));drop(control);drain(&mut recipient);assert!(other.terminal_is_empty());
 println!("[DEBUG] Original recipient denies missing, occupied, nested scopes and nested replacement before callback, ledger or heap change");
}

#[test]
fn sqlite_original_recipient_returns_cancelled_prefix_and_restores_same_reference(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫙️prefix/🔣️.json")).unwrap();
 for stop in fixture["cancellations"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize).chain([usize::MAX]){
  let source=fixture["samples"][2]["text"].as_str().unwrap().repeat(8193);let pointer=source.as_ptr();let mut recipient=NativeDecodeRetirementRecipient::new();let original_ref=&recipient as *const _ as usize;let events=Cell::new(0);let mut callback=|_|{let event=events.get();events.set(event+1);event!=stop};let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());control.install_native_retirement_recipient(&mut recipient).unwrap();
  let (result,requested,released)=observe_backing(||control.with_retirement_owner(SqliteSnapshotPhase::ReconstructSnapshot,size_of::<ControlledRetirement<Vec<u8>>>(),|control|{
   let mut owner=Box::new(ControlledRetirement::new(Vec::<u8>::new()).unwrap());let result:std::result::Result<usize,ValueError>=(||{let original=owner.original_mut().unwrap();control.admit_allocation_bytes(source.len())?;original.reserve_exact(source.len());for bytes in source.as_bytes().chunks(1024){original.extend_from_slice(bytes);control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,original.len(),source.len())?;}Ok(original.len())})();(result,Some(owner as Box<dyn ErasedSnapshotRetirement>))
  }));assert_eq!(released,0);if stop==0{assert_eq!(requested,0);assert_eq!(control.ledger.allocation_bytes,size_of::<ControlledRetirement<Vec<u8>>>());}else{assert_eq!(requested,control.ledger.allocation_bytes);}assert_eq!((&**control.native_retirement.as_ref().unwrap()) as *const _ as usize,original_ref);assert_eq!(source.as_ptr(),pointer);if stop==usize::MAX{assert_eq!(result.unwrap(),source.len());}else{assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);}drop(control);drain(&mut recipient);
  let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits{max_allocation_bytes:0,..SqliteDatabaseLimits::default()});control.install_native_retirement_recipient(&mut recipient).unwrap();let(result,requested,released)=observe_backing(||control.with_retirement_owner(SqliteSnapshotPhase::ReconstructSnapshot,1,|_|(Ok::<_,ValueError>(()),None)));assert_eq!(result.unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!((control.ledger.allocation_bytes,requested,released),(0,0,0));assert_eq!((&**control.native_retirement.as_ref().unwrap()) as *const _ as usize,original_ref);drop(control);assert!(recipient.terminal_is_empty());
 }
 println!("[DEBUG] Original typed prefix survives five cancellations and completion; every physical release is funded and every Result restores the same recipient");
}

#[derive(Debug,semio_framework_value::RetireOwned)]
struct Fields{text:String,blob:Vec<u8>}

#[test]
fn sqlite_original_recipient_nested_field_preserves_original_typed_child(){
 let mut bytes=Vec::<u8>::with_capacity(65537);bytes.resize(65537,7);let source=bytes.as_ptr();let capacity=bytes.capacity();let mut inner=NativeDecodeRetirementRecipient::new();inner.with_reserved_owner(||{let owner=Box::new(ControlledRetirement::new(bytes).unwrap());assert_eq!(owner.original().unwrap().as_ptr(),source);(Ok::<_,ValueError>(()),Some(owner as Box<dyn ErasedSnapshotRetirement>))}).unwrap();let original=capacity+size_of::<ControlledRetirement<Vec<u8>>>();let mut outer=NativeDecodeRetirementRecipient::new();let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());control.install_native_retirement_recipient(&mut outer).unwrap();
 let(result,born,released)=observe_backing(||control.with_retirement_owner(SqliteSnapshotPhase::ReconstructSnapshot,size_of::<ControlledRetirement<NativeDecodeRetirementRecipient>>(),|_|{let owner=Box::new(ControlledRetirement::new(inner).unwrap_or_else(|(error,_)|panic!("original recipient field refused: {error}")));assert!(owner.original().unwrap().has_owner());(Ok::<_,ValueError>(()),Some(owner as Box<dyn ErasedSnapshotRetirement>))}));result.unwrap();assert_eq!(released,0);assert_eq!(born,control.ledger.allocation_bytes);drop(control);let(requested,released)=drain(&mut outer);assert_eq!(original+born+requested,released);assert!(outer.terminal_is_empty());
 println!("[DEBUG] Actual nested recipient field preserves original typed Vec pointer/backing and delegates all5 funded currencies: original={original} wrapper={born} retirement={requested} physical={released}");
}

#[test]
fn sqlite_original_recipient_field_copies_retain_completed_and_partial_originals(){
 use super::artifact::{reconstruct_text_into,reconstruct_blob_into};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫙️prefix/🔣️.json")).unwrap();let text=fixture["samples"][2]["text"].as_str().unwrap().repeat(32769);let blob=text.as_bytes().repeat(2);
 for mode in 0..3{
  let reached=Cell::new(false);let mut callback=|event:SqliteSnapshotProgress|{let total=if mode==1{text.len()}else{blob.len()};let cancel=mode!=0&&event.total==total&&event.completed>=65536&&event.completed<total;if cancel{reached.set(true);}!cancel};let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let mut recipient=NativeDecodeRetirementRecipient::new();control.install_native_retirement_recipient(&mut recipient).unwrap();let text_pointer=text.as_ptr();let blob_pointer=blob.as_ptr();let text_capacity=Cell::new(0);let blob_capacity=Cell::new(0);
  let(result,requested,released)=observe_backing(||control.with_retirement_owner(SqliteSnapshotPhase::ReconstructSnapshot,size_of::<ControlledRetirement<Fields>>(),|control|{
   let mut owner=Box::new(ControlledRetirement::new(Fields{text:String::new(),blob:Vec::new()}).unwrap());let fields=owner.original_mut().unwrap();let result=(||->std::result::Result<(),ValueError>{reconstruct_text_into(&mut fields.text,control,&text)?;reconstruct_blob_into(&mut fields.blob,control,&blob)?;Ok(())})();text_capacity.set(fields.text.capacity());blob_capacity.set(fields.blob.capacity());if result.is_ok(){assert_eq!(fields.text,text);assert_eq!(fields.blob,blob);}else{assert!(!fields.text.is_empty());if mode==2{assert_eq!(fields.text,text);assert!(!fields.blob.is_empty());}}assert_ne!(fields.text.as_ptr(),text_pointer);if fields.blob.capacity()!=0{assert_ne!(fields.blob.as_ptr(),blob_pointer);}(result,Some(owner as Box<dyn ErasedSnapshotRetirement>))
  }));assert_eq!(released,0);assert_eq!(requested,control.ledger.allocation_bytes);assert_eq!(requested,size_of::<ControlledRetirement<Fields>>()+text_capacity.get()+blob_capacity.get());assert_eq!((text.as_ptr(),blob.as_ptr()),(text_pointer,blob_pointer));assert_eq!(reached.get(),mode!=0);if mode==0{result.unwrap();}else{assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);}drop(control);drain(&mut recipient);
 }
 println!("[DEBUG] Original field copies preserve completed UTF8 and interior64KiB text/blob prefixes, exact backing ledger and caller-funded physical retirement");
}

#[derive(Debug,semio_framework_value::RetireOwned)]
struct IndexPrefix{rows:super::artifact::RowIndexStorage,order:Vec<usize>,unique:Vec<usize>,parents:Vec<(Option<usize>,u8)>}

#[test]
fn sqlite_original_recipient_schema_validation_preserves_semantics_and_cancelled_scratch(){
 use super::transfer::{SchemaValidationStorage,validate_database_into,validate_table_into,validate_component_into};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🏛️validation/🔣️.json")).unwrap();
 for case in fixture["cases"].as_array().unwrap(){let declared=case["declared"].as_str().unwrap();let mut database=SqliteDatabase::from_schema(declared).unwrap();database.tables[0].sql=case["actual"].as_str().unwrap().into();let table_pointer=database.tables.as_ptr();let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let mut recipient=NativeDecodeRetirementRecipient::new();control.install_native_retirement_recipient(&mut recipient).unwrap();
  let(result,requested,released)=observe_backing(||control.with_retirement_owner(SqliteSnapshotPhase::ReconstructSnapshot,size_of::<ControlledRetirement<SchemaValidationStorage>>(),|control|{let mut owner=Box::new(ControlledRetirement::new(SchemaValidationStorage::empty()).unwrap());let result=validate_database_into(owner.original_mut().unwrap(),&database,declared,SqliteSnapshotPhase::ReconstructSnapshot,control);(result,Some(owner as Box<dyn ErasedSnapshotRetirement>))}));assert_eq!(result.is_ok(),case["accepted"].as_bool().unwrap(),"{}: {result:?}",case["name"]);assert_eq!(released,0);assert_eq!(requested,control.ledger.allocation_bytes);assert_eq!(database.tables.as_ptr(),table_pointer);drop(control);drain(&mut recipient);
 }
 let literal="Grüße 🧬 '".repeat(8193).replace('\'',"''");let sql=format!("CREATE TABLE original_schema (id INTEGER PRIMARY KEY, text TEXT DEFAULT '{literal}');");let mut database=SqliteDatabase::from_schema(&sql).unwrap();database.tables.push(SqliteDatabase::from_schema("CREATE TABLE sibling(id INTEGER PRIMARY KEY);").unwrap().tables.remove(0));let source=database.tables[0].sql.as_ptr();
 for stop in [1,7,13,31,71,usize::MAX]{let events=Cell::new(0);let mut callback=|_|{let current=events.get();events.set(current+1);current!=stop};let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let mut recipient=NativeDecodeRetirementRecipient::new();control.install_native_retirement_recipient(&mut recipient).unwrap();let(result,requested,released)=observe_backing(||control.with_retirement_owner(SqliteSnapshotPhase::ReconstructSnapshot,size_of::<ControlledRetirement<SchemaValidationStorage>>(),|control|{let mut owner=Box::new(ControlledRetirement::new(SchemaValidationStorage::empty()).unwrap());let result=(||{validate_component_into(owner.original_mut().unwrap(),&database,&sql,SqliteSnapshotPhase::ReconstructSnapshot,control)?;validate_table_into(owner.original_mut().unwrap(),&database.tables[0],&sql,SqliteSnapshotPhase::ReconstructSnapshot,control)})();(result,Some(owner as Box<dyn ErasedSnapshotRetirement>))}));assert_eq!(released,0);assert_eq!(requested,control.ledger.allocation_bytes);assert_eq!(database.tables[0].sql.as_ptr(),source);if stop==usize::MAX{result.unwrap();}else{assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);}drop(control);drain(&mut recipient);}
 println!("[DEBUG] Shared schema corpus preserves identifier quotes and literal spelling; five lexical/structural cancellation boundaries and composed-table validation retain every original scratch allocation until funded close");
}

#[test]
fn sqlite_original_recipient_scalar_indexes_preserve_original_cancellation_workspaces(){
 use super::artifact::{RowIndex,RowIndexStorage};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧩️artifact/🧫️fixtures/🗂️paid-row-index/🔣️.json")).unwrap();let database=SqliteDatabase{tables:vec![SqliteTable{name:fixture["table"].as_str().unwrap().into(),sql:fixture["schema"].as_str().unwrap().into(),rows:fixture["rows"].as_array().unwrap().iter().map(|row|SqliteRow{rowid:row["id"].as_i64().unwrap(),values:vec![SqliteValue::Integer(row["id"].as_i64().unwrap()),SqliteValue::Integer(row["owner"].as_i64().unwrap()),SqliteValue::Integer(row["ordinal"].as_i64().unwrap()),SqliteValue::Text(row["literal"].as_str().unwrap().into()),row["parent"].as_i64().map(SqliteValue::Integer).unwrap_or(SqliteValue::Null)]}).collect()}]};let source=database.tables[0].rows.as_ptr();
 for cancelled in 0..5{
  let stage=Cell::new(usize::MAX);let reached=Cell::new(false);let mut callback=|event:SqliteSnapshotProgress|{let cancel=stage.get()==cancelled&&event.completed>=1;if cancel{reached.set(true);}!cancel};let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let mut recipient=NativeDecodeRetirementRecipient::new();control.install_native_retirement_recipient(&mut recipient).unwrap();
  let(result,requested,released)=observe_backing(||control.with_retirement_owner(SqliteSnapshotPhase::ReconstructSnapshot,size_of::<ControlledRetirement<IndexPrefix>>(),|control|{
   let mut owner=Box::new(ControlledRetirement::new(IndexPrefix{rows:RowIndexStorage::empty(),order:Vec::new(),unique:Vec::new(),parents:Vec::new()}).unwrap());let prefix=owner.original_mut().unwrap();let result=(||->std::result::Result<(),ValueError>{stage.set(0);let rows=RowIndex::new(&mut prefix.rows,&database,"neutral_row",5,&[],control,"invalid neutral row")?;stage.set(1);rows.grouped_by_into(&mut prefix.order,2,control,"invalid neutral ordinal",|row|Ok((0,Some(row.integer(1)?))))?;stage.set(2);rows.unique_text(&mut prefix.unique,rows.indices(),3,control,"duplicate neutral literal")?;stage.set(3);rows.parent_positions_into(&mut prefix.parents,4,control,"dangling neutral parent")?;RowIndex::cycles(&mut prefix.parents,control,"cyclic neutral parent")?;let mut ids=[0;4];for(index,position)in prefix.order.iter().enumerate(){ids[index]=rows.row(*position)?.rowid;}for(index,id)in ids.iter().enumerate(){assert_eq!(*id,fixture["groupedIds"][index].as_i64().unwrap());}Ok(())})();(result,Some(owner as Box<dyn ErasedSnapshotRetirement>))
  }));assert_eq!(released,0);assert_eq!(requested,control.ledger.allocation_bytes);assert_eq!(database.tables[0].rows.as_ptr(),source);assert_eq!(reached.get(),cancelled<4);if cancelled<4{assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);}else{result.unwrap();}drop(control);drain(&mut recipient);
 }
 println!("[DEBUG] Original row index, grouped positions, uniqueness and parent buffers survive four distinct cancellations and completed SQLite-equivalent ordering until actual funded retirement");
}

#[test]
fn sqlite_original_recipient_database_keeps_all_original_cells_until_funded_close(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫙️prefix/🪶️database.json")).unwrap();
 let mut database=SqliteDatabase::from_schema(fixture["schemaSql"].as_str().unwrap()).unwrap();
 for row in fixture["rows"].as_array().unwrap(){database.tables[0].rows.push(SqliteRow{rowid:row["id"].as_i64().unwrap(),values:vec![SqliteValue::Integer(row["id"].as_i64().unwrap()),SqliteValue::Text(row["text"].as_str().unwrap().into()),SqliteValue::Blob(row["octets"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect()),row["optional"].as_f64().map(SqliteValue::Real).unwrap_or(SqliteValue::Null),SqliteValue::Real(row["value"].as_f64().unwrap())]});}
 let source_rows=database.tables[0].rows.as_ptr();let source_text=match &database.tables[0].rows[2].values[1]{SqliteValue::Text(value)=>value.as_ptr(),_=>unreachable!()};
 let original=database.tables.capacity()*size_of::<SqliteTable>()+database.tables.iter().map(|table|table.name.capacity()+table.sql.capacity()+table.rows.capacity()*size_of::<SqliteRow>()+table.rows.iter().map(|row|row.values.capacity()*size_of::<SqliteValue>()+row.values.iter().map(|value|match value{SqliteValue::Text(value)=>value.capacity(),SqliteValue::Blob(value)=>value.capacity(),_=>0}).sum::<usize>()).sum::<usize>()).sum::<usize>();
 let mut recipient=NativeDecodeRetirementRecipient::new();let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());control.install_native_retirement_recipient(&mut recipient).unwrap();
 let(result,born,released)=observe_backing(||control.with_retirement_owner(SqliteSnapshotPhase::ReconstructSnapshot,size_of::<ControlledRetirement<SqliteDatabase>>(),|_|{
  let mut owner=Box::new(ControlledRetirement::new(database).unwrap());let database=owner.original_mut().unwrap();assert_eq!(database.tables[0].rows.as_ptr(),source_rows);let SqliteValue::Text(text)=&database.tables[0].rows[2].values[1]else{unreachable!()};assert_eq!(text.as_ptr(),source_text);
  (Ok::<_,ValueError>(()),Some(owner as Box<dyn ErasedSnapshotRetirement>))
 }));result.unwrap();assert_eq!(released,0);assert_eq!(born,control.ledger.allocation_bytes);drop(control);
 let(requested,released)=drain(&mut recipient);assert_eq!(original+born+requested,released);let(result,terminal_requested,terminal_released)=observe_backing(||recipient.close_step(grant()));assert!(matches!(result.unwrap(),RetainedCloneStep::Complete(_)));assert_eq!((terminal_requested,terminal_released),(0,0));
 println!("[DEBUG] Complete original SQLite database fields preserve source identities and all independent cells: original={original} wrapper={born} retirement={requested} physical={released}, terminal zero");
}

#[test]
fn sqlite_original_recipient_child_loans_borrow_original_ledgers_callbacks_and_custody(){
 use super::artifact::{reconstruct_text_into,reconstruct_blob_into};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫙️prefix/🔣️.json")).unwrap();let text=fixture["samples"][2]["text"].as_str().unwrap().repeat(32769);let blob=text.as_bytes().repeat(2);
 let events=Cell::new(0);let mut callback=|_|{events.set(events.get()+1);true};let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let mut child=NativeDecodeRetirementRecipient::new();
 let(result,requested,released)=observe_backing(||control.with_retirement_child(&mut child,|_|Ok::<_,ValueError>(())));
 assert_eq!(result.unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!((events.get(),control.ledger.allocation_bytes,requested,released),(0,0,0,0));assert!(child.terminal_is_empty());drop(control);
 for mode in 0..3{
  let reached=Cell::new(false);let events=Cell::new(0);let mut callback=|event:SqliteSnapshotProgress|{events.set(events.get()+1);let total=if mode==1{text.len()}else{blob.len()};let cancel=mode!=0&&event.total==total&&event.completed>=65536&&event.completed<total;if cancel{reached.set(true);}!cancel};
  let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let mut parent=NativeDecodeRetirementRecipient::new();control.install_native_retirement_recipient(&mut parent).unwrap();let text_pointer=text.as_ptr();let blob_pointer=blob.as_ptr();
  let(result,requested,released)=observe_backing(||control.with_retirement_owner(SqliteSnapshotPhase::ReconstructSnapshot,size_of::<NativeDecodeRetirementRecipient>(),|control|{
   let mut owner=Box::new(NativeDecodeRetirementRecipient::new());let original_ledger=&control.ledger.allocation_bytes as *const _ as usize;let original_callback=(&mut *control.callback)as *mut dyn FnMut(SqliteSnapshotProgress)->bool as *mut ()as usize;
   let result=control.with_retirement_child(&mut owner,|child|{
    assert_eq!(&child.ledger.allocation_bytes as *const _ as usize,original_ledger);assert_eq!((&mut *child.callback)as *mut dyn FnMut(SqliteSnapshotProgress)->bool as *mut ()as usize,original_callback);
    child.with_retirement_owner(SqliteSnapshotPhase::ReconstructSnapshot,size_of::<ControlledRetirement<Fields>>(),|child|{
     let mut fields=Box::new(ControlledRetirement::new(Fields{text:String::new(),blob:Vec::new()}).unwrap());let original=fields.original_mut().unwrap();
     let result=(||->std::result::Result<(),ValueError>{reconstruct_text_into(&mut original.text,child,&text)?;reconstruct_blob_into(&mut original.blob,child,&blob)?;Ok(())})();(result,Some(fields as Box<dyn ErasedSnapshotRetirement>))
    })
   });
   let prior=control.ledger.allocation_bytes;let before=events.get();let(denied,requested,released)=observe_backing(||control.with_retirement_child(&mut owner,|_|Ok::<_,ValueError>(())));assert_eq!(denied.unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!((control.ledger.allocation_bytes,events.get(),requested,released),(prior,before,0,0));assert_eq!(&control.ledger.allocation_bytes as *const _ as usize,original_ledger);
   (result,Some(owner as Box<dyn ErasedSnapshotRetirement>))
  }));assert_eq!(released,0);assert_eq!(requested,control.ledger.allocation_bytes);assert_eq!((text.as_ptr(),blob.as_ptr()),(text_pointer,blob_pointer));assert_eq!(reached.get(),mode!=0);if mode==0{result.unwrap();}else{assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);}drop(control);drain(&mut parent);
 }
 println!("[DEBUG] Actual nested conversion producers borrow identical ledger/callback addresses, preserve complete/partial original fields across interior cancellation, reject occupied loans without progress or heap, and close under all five caller currencies");
}

#[test]
fn sqlite_original_projection_retains_schema_census_rows_and_interior_fields(){
 use crate::artifact::{Cell as SqlCell,RowWriter,ProjectionStorage,project_owned};
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧩️artifact/🧫️fixtures/🫙️projection/🔣️.json")).unwrap();
 let sql=corpus["schemaSql"].as_str().unwrap();
 let text=corpus["textUnit"].as_str().unwrap().repeat(corpus["textRepeats"].as_u64().unwrap()as usize);
 let blob=vec![0,255,17];let text_pointer=text.as_ptr();let blob_pointer=blob.as_ptr();let mut retained=0;let mut interior_seen=false;
 for stop in [0,1,7,31,89,233,usize::MAX-1,usize::MAX]{
  let events=Cell::new(0);let interior=Cell::new(false);let cancelled=Cell::new(false);
  let mut callback=|event:SqliteSnapshotProgress|{let index=events.get();events.set(index+1);let inside=event.phase==SqliteSnapshotPhase::ProjectSnapshot&&event.total==text.len()&&event.completed>=65536&&event.completed<text.len();let refuse=if stop==usize::MAX-1&&inside{interior.set(true);true}else{index==stop};cancelled.set(cancelled.get()||refuse);!refuse};
  let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let mut recipient=NativeDecodeRetirementRecipient::new();control.install_native_retirement_recipient(&mut recipient).unwrap();
  let(result,requested,released)=observe_backing(||project_owned(&mut control,ProjectionStorage::empty,|storage,control|{
   let owners=[[SqlCell::Text(""),SqlCell::Blob(&[])],[SqlCell::Text(&text),SqlCell::Blob(&blob)]];
   let relations=[[SqlCell::Integer(2),SqlCell::Integer(1)],[SqlCell::Integer(1),SqlCell::Integer(0)],[SqlCell::Integer(2),SqlCell::Integer(0)]];
   let mut census=RowWriter::census(storage,sql,control)?;
   for(index,cells)in owners.iter().enumerate(){assert_eq!(census.insert("owner",cells)?,index as i64+1);}
   for(index,cells)in relations.iter().enumerate(){assert_eq!(census.insert("relation",cells)?,index as i64+1);}
   census.finish_census()?;
   let mut writer=RowWriter::new_into(storage,control)?;
   for(index,cells)in owners.iter().enumerate(){assert_eq!(writer.insert("owner",cells)?,index as i64+1);}
   for(index,cells)in relations.iter().enumerate(){assert_eq!(writer.insert("relation",cells)?,index as i64+1);}
   writer.finish_into()?;
   assert_eq!(storage.database().table("owner")?.rows[0].text(1)?,"");
   assert_eq!(storage.database().table("owner")?.rows[1].text(1)?,text);
   assert_eq!(storage.database().table("owner")?.rows[1].blob(2)?,blob);
   assert_eq!(storage.database().table("relation")?.rows.len(),3);
   Ok(())
  },|_|()));
  assert_eq!(released,0,"original projected backing must remain through work");
  assert_eq!((text.as_ptr(),blob.as_ptr()),(text_pointer,blob_pointer));
  assert_eq!(result.is_err(),cancelled.get(),"actual cancellation reached at stop {stop} across {} events",events.get());
  if cancelled.get(){assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);}else{result.unwrap();}
  if interior.get(){interior_seen=true;}
  if control.native_retirement.as_ref().is_some_and(|recipient|recipient.has_owner()){retained+=1;assert_eq!(requested,control.ledger.allocation_bytes);}
  drop(control);
  let(close_requested,close_released)=drain(&mut recipient);assert_eq!(requested+close_requested,close_released);
  let(result,terminal_requested,terminal_released)=observe_backing(||recipient.close_step(grant()));assert!(matches!(result.unwrap(),RetainedCloneStep::Complete(_)));assert_eq!((terminal_requested,terminal_released),(0,0));
 }
 assert!(retained>=6);assert!(interior_seen);
 println!("[DEBUG] Original projected schema, per-table identities, row/cell prefixes and interior UTF8 preserve physical backing through all five denied close currencies");
}

#[test]
fn sqlite_original_projection_refuses_missing_recipient_before_original_birth(){
 use crate::artifact::{ProjectionStorage,project_owned};
 let entered=Cell::new(false);let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());
 let(result,requested,released)=observe_backing(||project_owned(&mut control,||{entered.set(true);ProjectionStorage::empty()},|_,_|Ok(()),|_|()));
 assert_eq!(result.unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert!(!entered.get());assert_eq!((requested,released,control.ledger.allocation_bytes),(0,0,0));
 println!("[DEBUG] Original projection refuses a missing recipient before any schema, census, row or field backing");
}
