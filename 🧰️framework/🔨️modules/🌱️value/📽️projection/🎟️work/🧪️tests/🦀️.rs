use super::*;
use crate::retained_clone::RetainedCloneSource;
#[test]
fn genuine_scalar_projection_preserves_binding_copy_and_take_work_under_one_bank(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let mut stock:RetainedCloneGrant=serde_json::from_value(law["stock"].clone()).unwrap();let mut cpu=law["maximumCpuWork"].as_u64().unwrap();
 let value=law["value"].as_u64().unwrap();let oracle=std::sync::Arc::new(value);
 let birth=RetainedCloneSource::<u64>::owned_constructor_demand::<()>();
 let turn=RetainedCloneSource::admit_owned(value,(),RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:birth.capacity_bytes,maximum_depth:birth.depth,..Default::default()});cpu-=turn.work;let(mut source,p)=turn.result.map_err(|(e,_,_)|e).unwrap();stock.maximum_items-=p.copied_items;stock.maximum_capacity_bytes-=p.retained_capacity_bytes;
 let d=u64::projection_birth_demand();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
 let turn=u64::begin_projection(RetainedCloneGrant{maximum_items:0,..g});assert_eq!(turn.work,law["expected"]["deniedCpu"].as_u64().unwrap());assert!(turn.result.is_err());
 let turn=u64::begin_projection(g);assert_eq!(turn.work,law["expected"]["birthCpu"].as_u64().unwrap());cpu-=turn.work;let(mut cursor,p)=turn.result.unwrap();stock.maximum_items-=p.copied_items;stock.maximum_copy_bytes-=p.copied_bytes;
 for key in ["bindingCpu","copyCpu"]{
  let d=cursor.demands(source.borrow(),stock.maximum_copy_bytes).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
  let turn=cursor.advance(source.borrow(),stock.maximum_copy_bytes,RetainedCloneGrant{maximum_items:0,..g});assert_eq!(turn.work,law["expected"]["deniedCpu"].as_u64().unwrap());assert_eq!(turn.result.unwrap().progress(),Default::default());
  let turn=cursor.advance(source.borrow(),stock.maximum_copy_bytes,g);assert_eq!(turn.work,law["expected"][key].as_u64().unwrap());cpu-=turn.work;let p=turn.result.unwrap().progress();assert!(p.fits(g));stock.maximum_items-=p.copied_items;stock.maximum_copy_bytes-=p.copied_bytes;stock.maximum_capacity_bytes-=p.retained_capacity_bytes;stock.maximum_release_bytes-=p.released_bytes;
 }
 assert!(cursor.ready());let d=cursor.take_demand().unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_depth:d.depth,..Default::default()};let turn=cursor.take(g);assert_eq!(turn.work,law["expected"]["takeCpu"].as_u64().unwrap());cpu-=turn.work;let(output,p)=turn.result.unwrap().unwrap();assert_eq!(output,*oracle);stock.maximum_items-=p.copied_items;stock.maximum_copy_bytes-=p.copied_bytes;
 cursor.begin_close();
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){
  if cursor.terminal_is_empty(){break}
  let d=cursor.close_demands(stock.maximum_copy_bytes).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
  assert!(g.maximum_copy_bytes<=stock.maximum_copy_bytes&&g.maximum_capacity_bytes<=stock.maximum_capacity_bytes&&g.maximum_release_bytes<=stock.maximum_release_bytes&&g.maximum_depth<=stock.maximum_depth);
  let turn=cursor.close_step(g);cpu=cpu.checked_sub(turn.work).unwrap();let p=turn.result.unwrap().progress();stock.maximum_items-=p.copied_items;stock.maximum_copy_bytes-=p.copied_bytes;stock.maximum_capacity_bytes-=p.retained_capacity_bytes;stock.maximum_release_bytes-=p.released_bytes;
 }
 assert!(cursor.terminal_is_empty());let turn=cursor.close_step(Default::default());assert_eq!(turn.work,law["expected"]["cachedCpu"].as_u64().unwrap());turn.result.unwrap();
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){
  if source.terminal_is_empty(){break}
  let d=source.next_close_demand(stock.maximum_copy_bytes).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
  assert!(g.maximum_copy_bytes<=stock.maximum_copy_bytes&&g.maximum_capacity_bytes<=stock.maximum_capacity_bytes&&g.maximum_release_bytes<=stock.maximum_release_bytes&&g.maximum_depth<=stock.maximum_depth);
  let turn=source.close_step(g);cpu=cpu.checked_sub(turn.work).unwrap();let p=turn.result.unwrap().progress();stock.maximum_items-=p.copied_items;stock.maximum_copy_bytes-=p.copied_bytes;stock.maximum_capacity_bytes-=p.retained_capacity_bytes;stock.maximum_release_bytes-=p.released_bytes;
 }
 assert!(source.terminal_is_empty());assert!(cpu>0);
}
