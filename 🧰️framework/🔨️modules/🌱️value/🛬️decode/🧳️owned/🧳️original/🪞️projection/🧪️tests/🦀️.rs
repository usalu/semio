use super::*;
#[test]
fn native_receiving_retained_projection_preserves_parent_and_exact_cancellation_receipts(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("📁️cases.json")).unwrap();
 for case in cases.as_array().unwrap(){let mut finished=0;
 for cut in 0..512{
  let mut wallet=stock();let mut bank=scratch();let original=DslValue::Object(vec![("value".to_owned(),DslValue::String(case["source"]["value"].as_str().unwrap().to_owned()))]);let original=if case["wrap"]=="array"{DslValue::Array(vec![DslValue::Null,original])}else{original};let source=input(original,&mut bank);let pointer=std::ptr::from_ref(source.borrow().get());let child_pointer=match source.borrow().get(){DslValue::Array(rows)=>std::ptr::from_ref(&rows[1]),value=>std::ptr::from_ref(value)};if case["wrap"]=="array"{assert_ne!(pointer,child_pointer)}
  let projection=project_child(&source,if case["wrap"]=="array"{Some(1)}else{None},&mut wallet,&mut bank);
  let(mut owner,heap)=observe(||Owner::from_projection(projection));assert_eq!(heap,(0,0));assert_eq!(std::ptr::from_ref(owner.borrow_input().unwrap().get()),child_pointer);
  let mut payer=Authorities::new();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);let mut turns=0;
  while turns<cut&&!owner.output_ready(){
   let fuel=wallet.remaining_work();let d=owner.demands(BODY,fuel).unwrap();let g=quoted(&wallet,d);
   for denied in fixture::denials(g,d){let(turn,heap)=advance(&mut owner,&mut control,&mut payer,&wallet,denied);assert_eq!(turn.work,0);assert_eq!(progress(&turn),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.demands(BODY,fuel).unwrap(),d);}
   let(turn,heap)=advance(&mut owner,&mut control,&mut payer,&wallet,g);let p=progress(&turn);wallet.post(p,turn.work,g,heap);turn.result.unwrap();turns+=1;
  }
  settle_pending(&mut owner,&mut control,&mut payer,&mut wallet);
  if owner.output_ready(){
   finished=turns;
   if cut%2==0{let g=quoted(&wallet,owner.take_demand().unwrap());let mut taken=None;let(turn,heap)=take(&mut owner,&mut bank,g,&mut taken);let p=progress(&turn);wallet.post(p,turn.work,g,heap);turn.result.unwrap();let row=taken.expect("funded publication fills the caller slot");assert_eq!(row.value.as_str(),case["expected"].as_str().unwrap());close_owned(row,&mut payer,&mut control);}
  }
  close_all(owner,&mut wallet,&mut payer,&mut control,&mut Vec::new());assert_eq!(std::ptr::from_ref(source.borrow().get()),pointer);close_owned(source,&mut payer,&mut control);
  if finished>0&&cut>finished{break}
 }
 assert!(finished>0);}
}
