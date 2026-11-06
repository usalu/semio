use super::*;
#[test]
fn native_receiving_retained_projection_preserves_parent_and_exact_cancellation_receipts(){
 type Owner=NativeReceivingOriginal<DslValue,Row>;
 let cases:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();
 for case in cases.as_array().unwrap(){let mut finished=0;
 for cut in 0..128{
  let mut bank=Bank::new();let original=DslValue::Object(vec![("value".to_owned(),DslValue::String(case["source"]["value"].as_str().unwrap().to_owned()))]);let original=if case["wrap"]=="array"{DslValue::Array(vec![DslValue::Null,original])}else{original};let source=crate::source_fixture::capture(original,&mut bank,Bank::offer,|bank,p,work,heap,g|account(bank,p,work,g,heap));let pointer=std::ptr::from_ref(source.borrow().get());let child_pointer=match source.borrow().get(){DslValue::Array(rows)=>std::ptr::from_ref(&rows[1]),value=>std::ptr::from_ref(value)};if case["wrap"]=="array"{assert_ne!(pointer,child_pointer)}
  let d=crate::retained_clone::RetainedOwnedProjection::<DslValue>::constructor_demand();let g=bank.quoted(d);
  let(turn,heap)=observe(||source.project_owned(1,|value|match value{DslValue::Array(rows)=>&rows[1],value=>value},g));let work=turn.work;let(projection,p)=turn.result.unwrap();account(&mut bank,p,work,g,heap);
  let(mut owner,heap)=observe(||Owner::from_projection(projection));assert_eq!(heap,(0,0));assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),child_pointer);
  let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);let mut turns=0;
  while turns<cut&&!owner.output_ready(){
   let d=owner.demands(4096).unwrap();let g=bank.quoted(d);
   for denied in denied(d,g){let(turn,heap)=observe(||receive(&mut control,denied,crate::source_fixture::WorkLoan::new(&bank.fuel,1),|control|owner.advance(control,4096,denied)));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.demands(4096).unwrap(),d);}
   let(turn,heap)=observe(||receive(&mut control,g,crate::source_fixture::WorkLoan::new(&bank.fuel,1),|control|owner.advance(control,4096,g)));let work=turn.work;let p=turn.result.unwrap().progress();account(&mut bank,p,work,g,heap);turns+=1;
  }
  if owner.output_ready(){
   finished=turns;
   if cut%2==0{let g=bank.quoted(owner.take_demand().unwrap());let(turn,heap)=observe(||owner.take(g));let work=turn.work;let(row,p)=turn.result.unwrap().unwrap();account(&mut bank,p,work,g,heap);assert_eq!(row.value.as_str(),case["expected"].as_str().unwrap());drain(row,&mut bank);}
  }
  owner.begin_close();
  for _ in 0..10000{
   if owner.terminal_is_empty(){break}
   let d=owner.close_demands(4096).unwrap();let g=bank.quoted(d);
   for denied in denied(d,g){let(turn,heap)=observe(||{let mut actual=None;owner.close_step(&mut crate::RetainedAllocationEffect::new(denied,u64::MAX,&mut actual))});assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.close_demands(4096).unwrap(),d);}
   let(turn,heap)=observe(||{let mut actual=None;owner.close_step(&mut crate::RetainedAllocationEffect::new(g,u64::MAX,&mut actual))});let work=turn.work;let p=turn.result.unwrap().progress();account(&mut bank,p,work,g,heap);
   assert_eq!(std::ptr::from_ref(source.borrow().get()),pointer);
  }
  assert!(owner.terminal_is_empty());assert_eq!(std::ptr::from_ref(source.borrow().get()),pointer);drain(source,&mut bank);
  if finished>0&&cut>finished{break}
 }
 assert!(finished>0);}
}
