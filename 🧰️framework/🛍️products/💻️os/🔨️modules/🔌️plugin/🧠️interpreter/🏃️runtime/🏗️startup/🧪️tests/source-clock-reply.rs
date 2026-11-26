//! 🕰️ The actual owned Core import returns identical clock data and physical work under both funded host schedules.
use semio_framework_actor::{OriginalGuestExecutionBudget,RetainedTurnInput};
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
use semio_plugin_interpreter::{CoreInstanceOpening,CoreModule,ExportKind,OwnedSemioExport};
use semio_plugin_interpreter_imports::{OriginalHostState,advance};
use std::cell::Cell;
use std::sync::Arc;
thread_local!{
 static HOST_CLOCK_NOW:Cell<u64>=const {Cell::new(1000)};
 static HOST_CLOCK_OBSERVATIONS:Cell<u64>=const {Cell::new(0)};
}
fn counted_clock()->Option<u64>{
 HOST_CLOCK_OBSERVATIONS.with(|value|value.set(value.get()+1));
 Some(HOST_CLOCK_NOW.with(Cell::get))
}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:2_000_000,maximum_capacity_bytes:2_000_000,maximum_release_bytes:2_000_000,maximum_depth:64}}

fn first_core(component:&[u8])->&[u8]{
 assert_eq!(component[8],1);
 let mut length=0;
 for (index,&byte) in component[9..].iter().enumerate(){
  length|=usize::from(byte&127)<<(7*index);
  if byte&128==0{return &component[10+index..10+index+length]}
 }
 unreachable!()
}

#[test]
fn original_source_clock_same_callback_matches_paid_staging(){
 fn run(maximum_items:usize)->(usize,[u8;16],RetainedCloneProgress,u128){
  HOST_CLOCK_NOW.with(|clock|clock.set(1000));
  let stock=grant();
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(799),Generation(1),stock,counted_clock,1_000_000,1_000_000);
  let bytes=crate::channel_fixture::owned_clock_guest(semio_os_kernel::CHANNEL_VERSION,caller.original_clock_binding());
  let module=&Arc::new(CoreModule::parse(first_core(&bytes)).unwrap());
  let mut epoch=1;
  let (mut core,mut host)=caller.turn(stock,|cx|{
   cx.bind_original_source_epoch(epoch)?;
   Ok((CoreInstanceOpening::begin(module,cx)?.unwrap(),OriginalHostState::new(cx)?))
  }).unwrap();
  let core=&mut core;
  let host=&mut host;
  for _ in 0..100{
   epoch+=1;
   if caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;core.advance(module,cx)}).unwrap(){break}
  }
  assert!(core.resources_ready());
  let Some(ExportKind::Function(function))=module.export(OwnedSemioExport::ChannelVersion.core_name()) else{panic!("fixture channel export")};
  for _ in 0..100{
   epoch+=1;
   if caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;core.begin_function_step(module,*function,&[],cx)}).unwrap(){break}
  }
  for _ in 0..250{
   if core.original_host_call_pending(){break}
   epoch+=1;
   caller.turn(stock,|cx|{
    cx.bind_original_source_epoch(epoch)?;
    let input=RetainedTurnInput::borrow_original(cx)?;
    let execution=OriginalGuestExecutionBudget{input,fuel:1,deadline_us:cx.deadline_us()};
    core.advance_scalar(module,execution,cx)?.result?;
    Ok(())
   }).unwrap();
  }
  assert!(core.original_host_call_pending());
  epoch+=1;
  caller.turn(RetainedCloneGrant{maximum_items:0,..stock},|cx|{
   cx.bind_original_source_epoch(epoch)?;
   let actual=core.original_host_call(module,cx)?.unwrap();
   assert_eq!((actual.module,actual.name),("semio:framework/pure@1.0.0","operation-clock-us"));
   let before=cx.retained_progress();
   assert!(!advance(host,core,module,None,cx)?);
   assert_eq!(cx.retained_progress(),before);
   assert!(!host.reply_pending());
   assert!(core.original_host_call_pending());
   Ok(())
  }).unwrap();
  let mut receipt=RetainedCloneProgress::default();
  let mut work=0;
  let mut turns=0;
  for round in 1..=8{
   epoch+=1;
   let ready=caller.turn(RetainedCloneGrant{maximum_items,..stock},|cx|{
    cx.bind_original_source_epoch(epoch)?;
    let before=cx.retained_progress();
    let work_before=cx.original_work_units_spent();
    let reads_before=HOST_CLOCK_OBSERVATIONS.with(Cell::get);
    let ready=advance(host,core,module,None,cx)?;
    if round==1{assert!(HOST_CLOCK_OBSERVATIONS.with(Cell::get)>reads_before);}
    let after=cx.retained_progress();
    receipt.copied_items+=after.copied_items-before.copied_items;
    receipt.copied_bytes+=after.copied_bytes-before.copied_bytes;
    receipt.retained_capacity_bytes+=after.retained_capacity_bytes-before.retained_capacity_bytes;
    receipt.released_bytes+=after.released_bytes-before.released_bytes;
    work+=cx.original_work_units_spent()-work_before;
    Ok(ready)
   }).unwrap();
   if ready{turns=round;break}
   HOST_CLOCK_NOW.with(|clock|clock.set(1246));
  }
  assert_ne!(turns,0);
  assert!(!host.reply_pending()&&!core.original_host_call_pending());
  let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
  let pointer=fixture["clockImport"]["pointer"].as_u64().unwrap() as usize;
  let output=core.resources().unwrap().memory(0).unwrap()[pointer..pointer+16].try_into().unwrap();
  caller.cancel().cancel_now();
  for _ in 0..200{
   if core.terminal_is_empty(){break}
   epoch+=1;
   caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;host.close_step(cx)?;core.close_step(module,cx)}).unwrap();
  }
  assert!(host.terminal_is_empty()&&core.terminal_is_empty());
  caller.retire(stock);
  (turns,output,receipt,work)
 }
 let(eager,reply,receipt,work)=run(16);
 let(staged,staged_reply,staged_receipt,staged_work)=run(1);
 assert_eq!(eager,1);
 assert!(staged>eager);
 assert_eq!((reply,receipt,work),(staged_reply,staged_receipt,staged_work));
 assert_eq!(u32::from_le_bytes(reply[..4].try_into().unwrap()),1);
 assert_eq!(u64::from_le_bytes(reply[8..].try_into().unwrap()),1000);
 assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(0,0));
}

fn retirement_scalar_core(module: &str, name: &str) -> Vec<u8> {
 fn name_bytes(value: &str) -> Vec<u8> {
  assert!(value.len() < 128);
  [vec![value.len() as u8], value.as_bytes().to_vec()].concat()
 }
 fn append_section(core: &mut Vec<u8>, kind: u8, contents: &[u8]) {
  assert!(contents.len() < 128);
  core.extend([kind, contents.len() as u8]);
  core.extend(contents);
 }
 let mut core = b"\0asm\x01\0\0\0".to_vec();
 append_section(&mut core, 1, &[1, 0x60, 0, 1, 0x7e]);
 append_section(&mut core, 2, &[vec![1], name_bytes(module), name_bytes(name), vec![0, 0]].concat());
 append_section(&mut core, 3, &[1, 0]);
 append_section(&mut core, 5, &[1, 0, 1]);
 let export = OwnedSemioExport::PublishActorFailure.core_name();
 append_section(&mut core, 7, &[vec![2], name_bytes("memory"), vec![2, 0], name_bytes(export), vec![0, 1]].concat());
 append_section(&mut core, 10, &[1, 4, 0, 0x10, 0, 0x0b]);
 core
}

#[test]
fn original_source_clock_retirement_admits_only_actual_clock_replies() {
 use semio_plugin_interpreter::Value;
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let law=&law["retirementScalarImports"];
 let microseconds=law["clockMicroseconds"].as_u64().unwrap();
 let nanoseconds=law["clockNanoseconds"].as_i64().unwrap();
 HOST_CLOCK_NOW.with(|clock|clock.set(microseconds));
 let stock=grant();
 let engine=wasmtime::Engine::default();
 for case in law["cases"].as_array().unwrap() {
  let import_module=case["module"].as_str().unwrap();
  let import_name=case["name"].as_str().unwrap();
  let allowed=case["allowed"].as_bool().unwrap();
  let bytes=retirement_scalar_core(import_module,import_name);
  let oracle=wasmtime::Module::new(&engine,&bytes).unwrap();
  let mut store=wasmtime::Store::new(&engine,());
  let clock=wasmtime::Func::wrap(&mut store,move||nanoseconds);
  let instance=wasmtime::Instance::new(&mut store,&oracle,&[clock.into()]).unwrap();
  let export=OwnedSemioExport::PublishActorFailure.core_name();
  let expected=instance.get_typed_func::<(),i64>(&mut store,export).unwrap().call(&mut store,()).unwrap();
  let module=&Arc::new(CoreModule::parse(&bytes).unwrap());
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(801),Generation(1),stock,counted_clock,1_000_000,1_000_000);
  let mut epoch=1;
  let(mut core,mut host)=caller.turn(stock,|cx|{
   cx.bind_original_source_epoch(epoch)?;
   Ok((CoreInstanceOpening::begin(module,cx)?.unwrap(),OriginalHostState::new(cx)?))
  }).unwrap();
  for _ in 0..100 {
   epoch+=1;
   if caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;core.advance(module,cx)}).unwrap(){break}
  }
  let Some(ExportKind::Function(function))=module.export(export) else{panic!("fixture retirement export")};
  caller.cancel().cancel_now();
  epoch+=1;
  caller.turn(stock,|cx|{
   cx.bind_original_source_epoch(epoch)?;
   cx.with_original_retirement(|cx|{
    assert!(core.begin_function_step(module,*function,&[],cx)?);
    let execution=OriginalGuestExecutionBudget{input:RetainedTurnInput::borrow_original(cx)?,fuel:16,deadline_us:cx.deadline_us()};
    core.advance_scalar(module,execution,cx)?.result?;
    assert!(core.original_host_call_pending());
    Ok(())
   })
  }).unwrap();
  epoch+=1;
  caller.turn(RetainedCloneGrant{maximum_items:0,..stock},|cx|{
   cx.bind_original_source_epoch(epoch)?;
   cx.with_original_retirement(|cx|{
    let before=cx.retained_progress();
    let result=advance(&mut host,&mut core,module,None,cx);
    if allowed{assert!(!result?);}else{assert!(result.is_err());}
    assert_eq!(cx.retained_progress(),before);
    assert!(!host.reply_pending());
    assert!(core.original_host_call_pending());
    Ok(())
   })
  }).unwrap();
  epoch+=1;
  caller.turn(stock,|cx|{
   cx.bind_original_source_epoch(epoch)?;
   cx.with_original_retirement(|cx|{
    if allowed {
     assert!(advance(&mut host,&mut core,module,None,cx)?);
     let execution=OriginalGuestExecutionBudget{input:RetainedTurnInput::borrow_original(cx)?,fuel:16,deadline_us:cx.deadline_us()};
     core.advance_scalar(module,execution,cx)?.result?;
     assert_eq!(core.invocation_values(),Some(&[Value::I64(expected)][..]));
    }else{
     let before=cx.retained_progress();
     assert!(core.reply_original_host_step(module,&[Value::I64(expected)],cx).is_err());
     assert_eq!(cx.retained_progress(),before);
     assert!(core.original_host_call_pending());
    }
    Ok(())
   })
  }).unwrap();
  for _ in 0..200 {
   if core.terminal_is_empty(){break}
   epoch+=1;
   caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;host.close_step(cx)?;core.close_step(module,cx)}).unwrap();
  }
  assert!(host.terminal_is_empty()&&core.terminal_is_empty());
  caller.retire(stock);
 }
}
