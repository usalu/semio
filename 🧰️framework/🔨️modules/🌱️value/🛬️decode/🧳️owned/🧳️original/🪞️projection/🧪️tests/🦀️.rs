use super::*;
#[test]
fn native_receiving_retained_projection_preserves_parent_and_exact_cancellation_receipts(){
 type Owner=NativeReceivingOriginal<DslValue,Row>;
 let cases:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();
 for case in cases.as_array().unwrap(){let mut finished=0;
 for cut in 0..128{
  let mut bank=Bank::new();let original=DslValue::Object(vec![("value".to_owned(),DslValue::String(case["source"]["value"].as_str().unwrap().to_owned()))]);let original=if case["wrap"]=="array"{DslValue::Array(vec![DslValue::Null,original])}else{original};let source=crate::source_fixture::capture(original,&mut bank,Bank::offer,|bank|bank.fuel,|bank,p,work,heap,g|account(bank,p,work,g,heap));let pointer=std::ptr::from_ref(source.borrow().get());let child_pointer=match source.borrow().get(){DslValue::Array(rows)=>std::ptr::from_ref(&rows[1]),value=>std::ptr::from_ref(value)};if case["wrap"]=="array"{assert_ne!(pointer,child_pointer)}
  let d=crate::retained_clone::RetainedOwnedProjection::<DslValue>::constructor_demand();let g=bank.quoted(d);
  let(turn,heap)=observe(||source.project_owned(1,|value|match value{DslValue::Array(rows)=>&rows[1],value=>value},g));let work=turn.work;let(projection,p)=turn.result.unwrap();account(&mut bank,p,work,g,heap);
  let(mut owner,heap)=observe(||Owner::from_projection(projection));assert_eq!(heap,(0,0));assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),child_pointer);
  let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);let mut turns=0;
  while turns<cut&&!owner.output_ready(){
   let d=owner.demands(4096,bank.fuel).unwrap();let g=bank.quoted(d);
   for denied in denied(d,g){let(turn,heap)=observe(||receive(&mut control,denied,crate::source_fixture::WorkLoan::new(&bank.fuel,bank.fuel),|control|{let maximum=control.available_work_units().unwrap();owner.advance(control,4096,maximum,denied)}));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.demands(4096,bank.fuel).unwrap(),d);}
   let(turn,heap)=observe(||receive(&mut control,g,crate::source_fixture::WorkLoan::new(&bank.fuel,bank.fuel),|control|{let maximum=control.available_work_units().unwrap();owner.advance(control,4096,maximum,g)}));let work=turn.work;let p=turn.result.unwrap().progress();account(&mut bank,p,work,g,heap);turns+=1;
  }
  if owner.output_ready(){
   finished=turns;
   if cut%2==0{let g=bank.quoted(owner.take_demand().unwrap());let(turn,heap)=observe(||owner.take(g));let work=turn.work;let(row,p)=turn.result.unwrap().unwrap();account(&mut bank,p,work,g,heap);assert_eq!(row.value.as_str(),case["expected"].as_str().unwrap());drain(row,&mut bank);}
  }
  owner.begin_close();let mut actual=None;
  for _ in 0..10000{
   if owner.terminal_is_empty()&&actual.is_none(){break}
   let d=crate::source_fixture::close_demands(&actual,owner.close_demands(4096)).unwrap();let g=bank.quoted(d);
   for denied in denied(d,g){let(turn,heap)=observe(||crate::source_fixture::close_step(&mut actual,denied,bank.fuel,owner.close_demands(4096),owner.close_work_demand(4096),|g,a|owner.close_step(g,a)));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(crate::source_fixture::close_demands(&actual,owner.close_demands(4096)).unwrap(),d);}
   let(turn,heap)=observe(||crate::source_fixture::close_step(&mut actual,g,bank.fuel,owner.close_demands(4096),owner.close_work_demand(4096),|g,a|owner.close_step(g,a)));let work=turn.work;let p=turn.result.unwrap().progress();account(&mut bank,p,work,g,heap);
   assert_eq!(std::ptr::from_ref(source.borrow().get()),pointer);
  }
  assert!(owner.terminal_is_empty());assert_eq!(std::ptr::from_ref(source.borrow().get()),pointer);drain(source,&mut bank);
  if finished>0&&cut>finished{break}
 }
 assert!(finished>0);}
}

#[test]fn native_receiving_original_keeps_cancellation_and_denied_transfers_owned(){
 type Owner=NativeReceivingOriginal<DslValue,Row>;let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🧳️original/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();assert_eq!(law["closeOrder"],serde_json::json!(["receiver","output","original-source","frame"]));let mut finished=0;
 for cut in 0..128{let mut bank=Bank::new();let source=input(&mut bank);let source_pointer=std::ptr::from_ref(source.borrow().get());let(mut owner,heap)=observe(||Owner::new(source));assert_eq!(heap,(0,0));assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),source_pointer);let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);let mut turns=0;
  while turns<cut&&!owner.output_ready(){let d=owner.demands(4096,bank.fuel).unwrap();let g=bank.quoted(d);assert!(retained_clone_grant_admits(g,d));for denied in denied(d,g){let(turn,heap)=observe(||receive(&mut control,denied,crate::source_fixture::WorkLoan::new(&bank.fuel,bank.fuel),|control|{let maximum=control.available_work_units().unwrap();owner.advance(control,4096,maximum,denied)}));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.demands(4096,bank.fuel).unwrap(),d);}let(turn,heap)=observe(||receive(&mut control,g,crate::source_fixture::WorkLoan::new(&bank.fuel,bank.fuel),|control|{let maximum=control.available_work_units().unwrap();owner.advance(control,4096,maximum,g)}));let p=match &turn.result{Ok(s)=>s.progress(),Err(e)=>e.retained_progress()};account(&mut bank,p,turn.work,g,heap);turn.result.unwrap();assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),source_pointer);turns+=1;}
  if owner.output_ready(){finished=turns;let d=owner.take_demand().unwrap();let g=bank.quoted(d);assert_eq!(d.copy_bytes,0);for denied in denied(d,g){let(turn,heap)=observe(||owner.take(denied));assert_eq!(turn.work,0);assert!(turn.result.unwrap().is_none());assert_eq!(heap,(0,0));assert!(owner.output_ready());}if cut%2==0{let(turn,heap)=observe(||owner.take(RetainedCloneGrant{maximum_copy_bytes:0,..g}));let work=turn.work;let(row,p)=turn.result.unwrap().unwrap();account(&mut bank,p,work,g,heap);assert_eq!(row.value.as_str(),"Original receiving owner🧱");drain(row,&mut bank);}}
  owner.begin_close();let mut actual=None;for _ in 0..10000{if owner.terminal_is_empty()&&actual.is_none(){break}let d=crate::source_fixture::close_demands(&actual,owner.close_demands(4096)).unwrap();let g=bank.quoted(d);assert!(retained_clone_grant_admits(g,d));for denied in denied(d,g){let(turn,heap)=observe(||crate::source_fixture::close_step(&mut actual,denied,bank.fuel,owner.close_demands(4096),owner.close_work_demand(4096),|g,a|owner.close_step(g,a)));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(crate::source_fixture::close_demands(&actual,owner.close_demands(4096)).unwrap(),d);}let(turn,heap)=observe(||crate::source_fixture::close_step(&mut actual,g,bank.fuel,owner.close_demands(4096),owner.close_work_demand(4096),|g,a|owner.close_step(g,a)));let p=match &turn.result{Ok(s)=>s.progress(),Err(e)=>e.retained_progress()};account(&mut bank,p,turn.work,g,heap);turn.result.unwrap();}assert!(owner.terminal_is_empty());if finished>0&&cut>finished{break;}
 }assert!(finished>0);
}
