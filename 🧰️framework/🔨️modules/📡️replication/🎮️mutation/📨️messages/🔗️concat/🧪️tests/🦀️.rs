//! 🧪️ Original row joins agree with the portable ordered-batch corpus at every cancellation cut.
use semio_framework_test_fixtures::fixture_text;
use super::*;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn corpus()->serde_json::Value{serde_json::from_str(fixture_text!("../../🔗️concat/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
fn batches(c:&serde_json::Value)->[Vec<MutationMessage>;2]{std::array::from_fn(|i|c["batches"][i].as_array().unwrap().iter().enumerate().map(|(j,s)|{let mut m=MutationMessage::warning("portable.join",s.as_str().unwrap());if i==1{m.op_index=Some(17);}m.target=vec![format!("target-{j}")];m}).collect())}
fn observe<T>(f:impl FnOnce()->T)->(T,(usize,usize)){let(v,h)=semio_framework_trace::observe_heap_allocations_on_this_thread(f);(v,(h.requested_bytes,h.released_bytes))}
fn close(o:&mut MutationMessageConcat<2>){if !o.close_started(){let turn=o.begin_close_paid(grant(o.begin_close_demand()));assert!(turn.result.is_ok());assert_eq!(turn.work,1)}for turn in 0..1000{if o.terminal_is_empty(){return;}let g=grant(o.close_demand().unwrap());let(s,h)=observe(||o.close_step(g).result.unwrap());assert!(s.progress().fits(g));assert_eq!(h,(s.progress().retained_capacity_bytes,s.progress().released_bytes));assert!(turn<999);}}
/// 🧫️ Joining preserves ordered bodies, indices and every original string allocation.
#[test]
fn mutation_message_concat_preserves_original_rows_and_allocation_identity(){
 let c=corpus();let rows=batches(&c);let pointers:Vec<_>=rows.iter().flat_map(|b|b.iter().map(|m|(m.message.as_ptr(),m.target[0].as_ptr()))).collect();
 let(mut o,p)=MutationMessageConcat::admit(rows,grant(MutationMessageConcat::<2>::constructor_demand())).unwrap_or_else(|_|panic!("original batch capture"));assert_eq!(p.retained_capacity_bytes,0);
 for turn in 0..1000{let g=grant(o.demand().unwrap());let(s,h)=observe(||o.advance(Default::default()).unwrap());assert_eq!(s.progress(),Default::default());assert_eq!(h,(0,0));let(s,h)=observe(||o.advance(g).unwrap());assert_eq!(h,(s.progress().retained_capacity_bytes,s.progress().released_bytes));if matches!(s,RetainedCloneStep::Complete(_)){break;}assert!(turn<999);}
 let(joined,p)=o.take(grant(o.demand().unwrap())).unwrap().unwrap();assert_eq!(p.retained_capacity_bytes,0);assert_eq!(joined.iter().map(|m|m.message.as_str()).collect::<Vec<_>>(),c["expected"].as_array().unwrap().iter().map(|s|s.as_str().unwrap()).collect::<Vec<_>>());for(i,m)in joined.iter().enumerate(){assert_eq!((m.message.as_ptr(),m.target[0].as_ptr()),pointers[i]);assert_eq!(m.op_index,Some(if i==2{17}else{0}));}
 let(mut ledger,_)=MutationMessageConcat::admit([joined,vec![]],grant(MutationMessageConcat::<2>::constructor_demand())).unwrap_or_else(|_|panic!("original joined close"));close(&mut ledger);close(&mut o);
}
/// 🛑️ Every partial backing or pending-row cut conserves typed cancellation custody.
#[test]
fn mutation_message_concat_cancels_every_original_row_frontier(){let c=corpus();for cut in 0..c["cancelFrontiers"].as_u64().unwrap(){let(mut o,_)=MutationMessageConcat::admit(batches(&c),grant(MutationMessageConcat::<2>::constructor_demand())).unwrap_or_else(|_|panic!("original batch capture"));for _ in 0..cut{if matches!(o.advance(grant(o.demand().unwrap())).unwrap(),RetainedCloneStep::Complete(_)){break;}}close(&mut o);assert!(o.terminal_is_empty());}}

#[test]
fn original_message_close_visits_at_most_one_input_slot_per_turn(){
 let batches:[Vec<MutationMessage>;4]=std::array::from_fn(|_|Vec::new());
 let(mut owner,_)=MutationMessageConcat::<4>::admit(batches,grant(MutationMessageConcat::<4>::constructor_demand())).unwrap_or_else(|_|panic!("original capture"));
 owner.begin_close();
 for _ in 0..4{
  let demand=owner.close_demand().unwrap();assert_eq!(owner.close_work_demand().unwrap(),1);
  let denied=owner.close_step(RetainedCloneGrant{maximum_items:0,..grant(demand)});assert_eq!(denied.work,0);
  let turn=owner.close_step(grant(demand));assert_eq!(turn.work,1);turn.result.unwrap();
 }
 assert!(owner.terminal_is_empty());
}

#[test]
fn concat_close_start_preserves_custody_on_denial_and_posts_exact_action(){let c=corpus();let law=&c["closeStart"];let witness=law["witnessBytes"].as_u64().unwrap()as usize;let(_,heap)=observe(||{let bytes=std::hint::black_box(vec![0u8;witness]);std::hint::black_box(&bytes);drop(bytes)});assert_eq!(heap,(witness,witness));let(mut owner,_)=MutationMessageConcat::admit(batches(&c),grant(MutationMessageConcat::<2>::constructor_demand())).unwrap_or_else(|_|panic!("funded original batch"));let exact=grant(owner.begin_close_demand());let denied=owner.begin_close_paid(RetainedCloneGrant{maximum_items:0,..exact});assert_eq!(denied.work,u128::from(law["deniedWork"].as_u64().unwrap()));assert!(!owner.close_started());let turn=owner.begin_close_paid(exact);assert_eq!(turn.work,u128::from(law["selectedWork"].as_u64().unwrap()));assert_eq!(turn.result.unwrap().progress().copied_bytes,exact.maximum_copy_bytes);assert!(owner.close_started());close(&mut owner);}
