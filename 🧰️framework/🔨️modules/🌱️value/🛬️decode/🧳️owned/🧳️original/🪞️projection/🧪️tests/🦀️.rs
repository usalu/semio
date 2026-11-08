use super::*;
#[test]
fn native_receiving_retained_projection_preserves_parent_and_exact_cancellation_receipts(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();
 for case in cases.as_array().unwrap(){let mut finished=0;
 for cut in 0..128{
  let mut wallet=stock();let mut bank=scratch();let original=DslValue::Object(vec![("value".to_owned(),DslValue::String(case["source"]["value"].as_str().unwrap().to_owned()))]);let original=if case["wrap"]=="array"{DslValue::Array(vec![DslValue::Null,original])}else{original};let source=input(original,&mut wallet);let pointer=std::ptr::from_ref(source.borrow().get());let child_pointer=match source.borrow().get(){DslValue::Array(rows)=>std::ptr::from_ref(&rows[1]),value=>std::ptr::from_ref(value)};if case["wrap"]=="array"{assert_ne!(pointer,child_pointer)}
  let d=crate::retained_clone::RetainedOwnedProjection::<DslValue>::constructor_demand();let g=quoted(&wallet,d);
  let(poll,heap)=observe(||{let(turn,_)=bank.run(g,crate::retained_clone::RetainedOwnedProjection::<DslValue>::constructor_work_demand(),|effect|RetainedWorkPoll::Complete(source.project_owned(1,|value|match value{DslValue::Array(rows)=>&rows[1],value=>value},effect)));turn});assert_ne!(bank.take_admitted(),Some(false));let turn=settled(poll);let work=turn.work;let(projection,p)=turn.result.unwrap();wallet.post(p,work,g,heap);
  let(mut owner,heap)=observe(||Owner::from_projection(projection));assert_eq!(heap,(0,0));assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),child_pointer);
  let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);let mut turns=0;
  while turns<cut&&!owner.output_ready(){
   let fuel=wallet.work_remaining();let d=owner.demands(BODY,fuel).unwrap();let g=quoted(&wallet,d);
   for denied in fixture::denials(g,d){let(turn,heap)=advance(&mut owner,&mut control,&wallet,denied);assert_eq!(turn.work,0);assert_eq!(progress(&turn),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.demands(BODY,fuel).unwrap(),d);}
   let(turn,heap)=advance(&mut owner,&mut control,&wallet,g);let p=progress(&turn);wallet.post(p,turn.work,g,heap);turn.result.unwrap();turns+=1;
  }
  if owner.output_ready(){
   finished=turns;
   if cut%2==0{let g=quoted(&wallet,owner.take_demand().unwrap());let mut taken=None;let(turn,heap)=take(&mut owner,&mut bank,g,&mut taken);let p=progress(&turn);wallet.post(p,turn.work,g,heap);turn.result.unwrap();let row=taken.expect("funded publication fills the caller slot");assert_eq!(row.value.as_str(),case["expected"].as_str().unwrap());fixture::close_owned(row,&mut wallet,BODY,10000);}
  }
  close_all(&mut owner,&mut wallet,&mut bank,&mut Vec::new());assert_eq!(std::ptr::from_ref(source.borrow().get()),pointer);assert!(owner.terminal_is_empty());fixture::close_owned(source,&mut wallet,BODY,10000);
  if finished>0&&cut>finished{break}
 }
 assert!(finished>0);}
}
