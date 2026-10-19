//! 🧪️ Original row joins agree with the portable ordered-batch corpus at every cancellation cut.
use super::*;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn corpus()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn batches(c:&serde_json::Value)->[Vec<MutationMessage>;2]{std::array::from_fn(|i|c["batches"][i].as_array().unwrap().iter().enumerate().map(|(j,s)|{let mut m=MutationMessage::warning("portable.join",s.as_str().unwrap());if i==1{m.op_index=Some(17);}m.target=vec![format!("target-{j}")];m}).collect())}
fn observe<T>(f:impl FnOnce()->T)->(T,(usize,usize)){let(v,h)=semio_framework_trace::observe_heap_allocations_on_this_thread(f);(v,(h.requested_bytes,h.released_bytes))}
fn close(o:&mut MutationMessageConcat<2>){o.begin_close();for turn in 0..1000{if o.terminal_is_empty(){return;}let g=grant(o.close_demand().unwrap());let(s,h)=observe(||o.close_step(g).unwrap());assert!(s.progress().fits(g));assert_eq!(h,(s.progress().retained_capacity_bytes,s.progress().released_bytes));assert!(turn<999);}}
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
