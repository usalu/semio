//! 🎛️ Only actual owned interpreter execution advances the lifecycle clock.
use semio_framework_actor::{OriginalGuestExecutionBudget,RetainedTurnInput};
use semio_framework_job::{Generation,OperationId,OriginalJobFixtureCaller,default_now_us};
use semio_framework_value::RetainedCloneGrant;
use semio_plugin_interpreter::{CoreInstanceOpening,CoreModule,ExportKind,OwnedSemioExport,Value};
use semio_plugin_interpreter_imports::{OriginalHostState,advance};
use std::{sync::Arc,time::Duration};

fn encode(mut number:usize,output:&mut Vec<u8>){
 while number>=128{output.push((number as u8&127)|128);number>>=7}
 output.push(number as u8);
}
fn name(output:&mut Vec<u8>,text:&str){encode(text.len(),output);output.extend(text.as_bytes())}
fn section(output:&mut Vec<u8>,kind:u8,body:&[u8]){output.push(kind);encode(body.len(),output);output.extend(body)}

fn actor_probe()->Vec<u8>{
 let mut wasm=b"\0asm\x01\0\0\0".to_vec();
 section(&mut wasm,1,&[3,0x60,2,0x7e,0x7f,0,0x60,0,1,0x7e,0x60,1,0x7e,0]);
 let mut imports=vec![3];
 for (module,import,signature) in [
  ("semio:framework/pure@1.0.0","actor-execution-clock-us",0),
  ("semio:framework/pure@1.0.0","operation-clock-us",0),
  ("wasi:clocks/monotonic-clock@0.2.0","now",1),
 ]{name(&mut imports,module);name(&mut imports,import);imports.extend([0,signature])}
 section(&mut wasm,2,&imports);
 section(&mut wasm,3,&[1,2]);
 section(&mut wasm,5,&[1,0,1]);
 let mut exports=vec![2];
 name(&mut exports,"memory");exports.extend([2,0]);
 name(&mut exports,OwnedSemioExport::Poll.core_name());exports.extend([0,3]);
 section(&mut wasm,7,&exports);
 let mut body=vec![0,0x20,0,0x41,0,0x10,0];
 body.extend(std::iter::repeat_n(0x01,16_384));
 body.extend([
  0x20,0,0x41,16,0x10,0,
  0x20,0,0x41,32,0x10,1,
  0x41,48,0x10,2,0x37,3,0,
  0x20,0,0x42,1,0x85,0x41,0xc0,0,0x10,0,
  0x0b,
 ]);
 let mut code=vec![1];encode(body.len(),&mut code);code.extend(body);
 section(&mut wasm,10,&code);
 wasm
}

#[test]
fn owned_core_actor_clock_excludes_a_real_host_pause_and_refuses_foreign_binding(){
 let bytes=actor_probe();
 wasmtime::Module::new(&wasmtime::Engine::default(),&bytes).unwrap();
 let module=&Arc::new(CoreModule::parse(&bytes).unwrap());
 let stock=RetainedCloneGrant{maximum_items:10_000,maximum_copy_bytes:2_000_000,maximum_capacity_bytes:2_000_000,maximum_release_bytes:2_000_000,maximum_depth:64};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(810),Generation(1),stock,default_now_us,60_000_000,1_000_000);
 let mut epoch=1;
 let (mut core,mut host)=caller.turn(stock,|cx|{
  cx.bind_original_source_epoch(epoch)?;
  Ok((CoreInstanceOpening::begin(module,cx)?.unwrap(),OriginalHostState::new(cx)?))
 }).unwrap();
 for _ in 0..128 {
  epoch+=1;
  if caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;core.advance(module,cx)}).unwrap(){break}
 }
 let binding=caller.original_clock_binding();
 let Some(ExportKind::Function(function))=module.export(OwnedSemioExport::Poll.core_name())else{panic!("owned Poll probe")};
 epoch+=1;
 assert!(caller.turn(stock,|cx|{
  cx.bind_original_source_epoch(epoch)?;
  core.begin_function_step(module,*function,&[Value::I64(binding as i64)],cx)
 }).unwrap());
 let wall=default_now_us().unwrap();
 let mut paused=false;
 for _ in 0..128{
  if core.invocation_values().is_some(){break}
  epoch+=1;
  if core.original_host_call_pending()||host.reply_pending(){
   if !paused{
    std::thread::sleep(Duration::from_millis(5100));
    paused=true;
   }
   caller.turn(stock,|cx|{
    cx.bind_original_source_epoch(epoch)?;
    advance(&mut host,&mut core,module,None,cx)?;
    Ok(())
   }).unwrap();
  }else{
   caller.turn(stock,|cx|{
    cx.bind_original_source_epoch(epoch)?;
    let execution=OriginalGuestExecutionBudget{input:RetainedTurnInput::borrow_original(cx)?,fuel:1024,deadline_us:cx.deadline_us()};
    core.advance_scalar(module,execution,cx)?.result?;
    Ok(())
   }).unwrap();
  }
 }
 assert!(paused&&core.invocation_values().is_some());
 let memory=core.resources().unwrap().memory(0).unwrap();
 let time=|at:usize|u64::from_le_bytes(memory[at..at+8].try_into().unwrap());
 for pointer in [0,16,32]{assert_eq!(u32::from_le_bytes(memory[pointer..pointer+4].try_into().unwrap()),1)}
 let active=time(24).checked_sub(time(8)).unwrap();
 assert!(active>0,"known paid instructions between the two owned clock reads were never counted");
 assert!(!semio_framework_trace::guest_lifecycle_turn_contract_violated(active),"{active}");
 assert!(time(40)>=wall+5_100_000);
 assert!(time(48)/1000>=wall+5_100_000);
 assert_eq!(&memory[64..80],&[0;16]);
 caller.cancel().cancel_now();
 for _ in 0..256{
  epoch+=1;
  if caller.retirement_turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;host.close_step(cx)?;core.close_step(module,cx)}).unwrap(){break}
 }
 assert!(host.terminal_is_empty()&&core.terminal_is_empty());
 caller.retire(stock);
}
