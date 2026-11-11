//! 🧪️ Actual Source command cursors match their binary encoder and retire all original allocations.
use semio_os_channel::*;
use protocol::wire::command_ingress::{CommandPageSet,FixedCommandPage,PagedCommand,COMMAND_PAGE_MAXIMUM_BYTES};
use semio_framework_job::{OperationId,Generation,RetainedCloneGrant};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
fn clock()->Option<u64>{Some(1)}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:128,maximum_copy_bytes:32768,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:256}}
fn observed<T>(run:impl FnOnce()->T)->(T,(usize,usize)){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);(result,(heap.requested_bytes,heap.released_bytes))}
fn run(bytes:&[u8],expected:Option<&AppCommand>){
 let(cursor,heap)=observed(||{let mut pages=CommandPageSet::try_new(bytes.len().div_ceil(COMMAND_PAGE_MAXIMUM_BYTES)).unwrap();for page in bytes.chunks(COMMAND_PAGE_MAXIMUM_BYTES){pages.try_push(FixedCommandPage::try_copy_from(page).unwrap()).unwrap()}PagedAppCommandDecodeCursor::new(PagedCommand::try_from_pages(pages).unwrap())});run_cursor(cursor,heap,expected)
}
fn run_encoded(command:&AppCommand){let(cursor,heap)=observed(||PagedAppCommandDecodeCursor::new(semio_framework_async::block_on(encode_app_command(command))));run_cursor(cursor,heap,Some(command))}
fn run_cursor(mut cursor:PagedAppCommandDecodeCursor,heap:(usize,usize),expected:Option<&AppCommand>){
 let stock=grant();let authority=RetainedCloneGrant{maximum_items:1000000,maximum_copy_bytes:268435456,..stock};let mut caller=OriginalJobFixtureCaller::admit(OperationId(818),Generation(1),authority,clock,10000000,10000000);
 let(mut born,mut freed)=heap;let mut output=None;let mut failed=false;
 for _ in 0..100000{if output.is_some(){break}for denied in [RetainedCloneGrant{maximum_items:0,..stock},RetainedCloneGrant{maximum_depth:0,..stock},RetainedCloneGrant{maximum_copy_bytes:0,..stock}]{let before=format!("{cursor:?}");let(result,heap)=observed(||caller.turn(denied,|cx|cursor.step_original(&mut output,cx)));if let Ok(ready)=result{assert!(!ready)}assert_eq!(heap,(0,0));assert_eq!(caller.receipt(),Default::default());assert_eq!(format!("{cursor:?}"),before);assert!(output.is_none())}
  let(result,heap)=observed(||caller.turn(stock,|cx|cursor.step_original(&mut output,cx)));assert_eq!(heap,(caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes));born+=heap.0;freed+=heap.1;if result.is_err(){failed=true;break}
 }
 assert_eq!(!failed,expected.is_some());if let Some(expected)=expected{assert_eq!(output.as_ref().unwrap().original_command(),Some(expected))}
 if let Some(owner)=output.as_mut(){let mut complete=false;for _ in 0..100000{let(result,heap)=observed(||caller.turn(stock,|cx|owner.close_original_step(cx)));assert_eq!(heap,(caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes));born+=heap.0;freed+=heap.1;if result.unwrap(){complete=true;break}}assert!(complete);assert!(owner.terminal_is_empty())}
 let mut complete=false;for _ in 0..100000{let(result,heap)=observed(||caller.turn(stock,|cx|cursor.close_original_step(cx)));assert_eq!(heap,(caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes));born+=heap.0;freed+=heap.1;if result.unwrap(){complete=true;break}}assert!(complete);assert!(cursor.terminal_is_empty());let(result,heap)=observed(||caller.turn(stock,|cx|cursor.close_original_step(cx)));assert!(result.unwrap());assert_eq!(heap,(0,0));assert_eq!(caller.receipt(),Default::default());assert_eq!(born,freed);caller.retire(stock);
}
#[test]
fn original_cursor_decodes_actual_binary_commands_and_keeps_refused_originals(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let seed=[AppCommand::ReadDocument{seq:1},AppCommand::CommandText{seq:300,line:"€".into()},AppCommand::LoadConfig{seq:2,pack:vec![1,2],spr:vec![3]}];
 for(command,row)in seed.iter().zip(law["cases"].as_array().unwrap()){let encoded=semio_framework_async::block_on(encode_app_command(command));let fixture:Vec<u8>=row["wire"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect();assert_eq!(encoded.page_len(),1);assert_eq!(encoded.front_page().unwrap().as_slice(),fixture);run(&fixture,Some(command));run_encoded(command);let mut trailing=fixture;trailing.push(0);run(&trailing,None)}
 let commands=[
 AppCommand::ConfigCommand{seq:1,command:vec![1;5000]},AppCommand::Command{seq:2,command:vec![1;17],view_state:vec![2;31]},AppCommand::ContextMenu{seq:3,request:vec![1;7]},AppCommand::ArtifactCommand{seq:4,command:vec![1;9]},
 AppCommand::PureCommand{seq:5,command:vec![1;3],head:vec![2;5]},AppCommand::MediaIn{seq:6,port:"é".into(),descriptor:vec![1;7],data:vec![2;11]},AppCommand::MediaOut{seq:7,port:"o".into(),request:vec![1;13]},AppCommand::MediaFingerprint{seq:8,port:"f".into()},
 AppCommand::TransactionPrepare{seq:9,txn_id:"t".into(),mutation_id:"m".into(),payload:vec![1;17],prepared_ops:vec![vec![1;3],vec![2;5]],origin:vec![3;7],prepared_child_ops:vec![4;11]},AppCommand::TransactionCommit{seq:10,txn_id:"t".into()},AppCommand::TransactionRollback{seq:11,txn_id:"t".into()},AppCommand::TransactionUndo{seq:12,group_id:"g".into()},AppCommand::TransactionRedo{seq:13,group_id:"g".into()},
 AppCommand::LoadWindowConfig{seq:14,entry:WindowConfigPackEntry{window_id:"w".into(),window_kind_id:"k".into(),envelope_pack:vec![1;23]}},
 AppCommand::LoadDocumentArchive{seq:15,archive:DocumentArchivePack{parent_pack:vec![1;3],parent_spr:vec![2;5],members:vec![OwnedDocumentMemberPackEntry{ordinal:7,envelope_pack:vec![3;11],..Default::default()}]}},AppCommand::MergeDocumentArchive{seq:16,archive:DocumentArchivePack::default()},
 AppCommand::SubmitMediaExport{seq:17,port:"p".into(),expected_parent_document_id:"d".into(),expected_base_revision:8,checkpoint_id:Some("😀".into())},AppCommand::SubmitMediaExport{seq:18,port:"p".into(),expected_parent_document_id:"d".into(),expected_base_revision:8,checkpoint_id:None},
 AppCommand::PollMediaExport{seq:19,handle:MediaExportHandleWire{app_instance_id:4,parent_document_id:"d".into(),operation_id:1,base_revision:2,generation:3}},AppCommand::LocalInteractionQuery{seq:20,command:protocol::LocalInteractionQueryCommand::Read{request_id:9}},
 AppCommand::ReadConfig{seq:21},AppCommand::ReadChildren{seq:22},AppCommand::ReadHistory{seq:23},AppCommand::ReadConflicts{seq:24},AppCommand::ReadWindowConfigs{seq:25},AppCommand::ReadDocumentArchive{seq:26},AppCommand::ReadDocumentIdentity{seq:27},AppCommand::ReadChildHeads{seq:28},AppCommand::PollDocumentArchiveLoad{seq:29,operation:2},AppCommand::CancelDocumentArchiveLoad{seq:30,operation:2},AppCommand::AcknowledgeDocumentArchiveLoad{seq:31,operation:2}
 ];
 for command in commands{run_encoded(&command)}
 run(&[2,1,2,0xc0,0x80],None);run(&[7,128,128,128,128,128,128,128,128,128,2],None);run(&[37,1,1,b'p',1,b'd',0,2],None);
}
