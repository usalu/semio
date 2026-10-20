//! 🧭️ Actual generic close dispatch and paid lifecycle observations shared by four semantic owners.
use super::*;
use std::cell::Cell;
thread_local!{static CALLS:Cell<(usize,[(usize,usize);64])>=const{Cell::new((0,[(0,0);64]))};}
pub(crate) fn record(cursor:usize,body:usize){CALLS.with(|state|{let(mut n,mut rows)=state.get();if n<rows.len(){rows[n]=(cursor,body);}n+=1;state.set((n,rows));});}
fn reset(){CALLS.with(|state|state.set((0,[(0,0);64])));}
fn calls(cursor:usize)->Vec<usize>{CALLS.with(|state|{let(n,rows)=state.get();assert!(n<=rows.len(),"selected close observation overflow");rows[..n].iter().filter_map(|(owner,body)|(*owner==cursor).then_some(*body)).collect()})}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
pub(super) fn verify<T:RetainedClone<Cursor=C>,C:RetainedCloneCursor<T>>(make:impl Fn()->T,select:fn(&C,usize)->Result<RetirementDemand,ValueError>){
    let policy:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let bound=policy["maximumTurns"].as_u64().unwrap()as usize;
    for cut in policy["cuts"].as_array().unwrap(){
        let owner=make();let birth=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<T>::constructor_copy_bytes(),maximum_capacity_bytes:RetainedCloneSource::<T>::owned_constructor_capacity_bytes::<()>(),maximum_depth:1,..Default::default()};
        let(mut source,p)=RetainedCloneSource::admit_owned(owner,(),birth).unwrap_or_else(|_|panic!("selected close source admission"));assert!(p.fits(birth));
        let identity=source.borrow().get()as*const T;let mut cursor=T::retained_clone_cursor();
        for _ in 0..cut.as_u64().unwrap(){if cursor.output_ready(){break;}let d=cursor.advance_demands(source.borrow(),4096).unwrap();let g=grant(d);assert!(cursor.advance(source.borrow(),4096,g).unwrap().progress().fits(g));}
        cursor.begin_close();let address=&cursor as*const T::Cursor as usize;
        for body in policy["bodies"].as_array().unwrap(){let body=body.as_u64().unwrap()as usize;let(expected,heap)=observe_retirement_allocations(||select(&cursor,body));assert_eq!(heap,(0,0));reset();let(actual,heap)=observe_retirement_allocations(||RetainedCloneCursor::<T>::close_demands(&cursor,body));assert_eq!(heap,(0,0));assert_eq!(actual.unwrap(),expected.unwrap());assert_eq!(calls(address),vec![body]);assert_eq!(source.borrow().get()as*const T,identity);}
        for turn in 0..bound{if cursor.terminal_is_empty(){break;}let d=cursor.close_demands(4096).unwrap();let g=grant(d);assert_eq!(cursor.close_demands(g.maximum_copy_bytes).unwrap(),d);
            let denied=[Some(RetainedCloneGrant{maximum_items:0,..g}),(d.copy_bytes>0).then(||RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g}),(d.capacity_bytes>0).then(||RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g}),(d.release_bytes>0).then(||RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..g}),(d.depth>0).then(||RetainedCloneGrant{maximum_depth:d.depth-1,..g})];
            for denied in denied.into_iter().flatten(){let(result,heap)=observe_retirement_allocations(||cursor.close_step(denied));assert_eq!(heap,(0,0),"selected close denial {denied:?} demand {d:?}");if let Ok(step)=result{assert_eq!(step.progress(),RetainedCloneProgress::default());}assert_eq!(cursor.close_demands(4096).unwrap(),d);assert_eq!(source.borrow().get()as*const T,identity);}
            let(result,heap)=observe_retirement_allocations(||cursor.close_step(g));let p=result.unwrap().progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert!(turn+1<bound||cursor.terminal_is_empty(),"selected close cursor did not finish");}
        assert!(cursor.terminal_is_empty());drop(cursor);assert_eq!(source.borrow().get()as*const T,identity);
        for turn in 0..bound{if source.terminal_is_empty(){break;}let d=source.next_close_demand(4096).unwrap();let g=grant(d);assert_eq!(source.next_close_demand(g.maximum_copy_bytes).unwrap(),d);let(result,heap)=observe_retirement_allocations(||source.close_step(g));let p=result.unwrap().progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert!(turn+1<bound||source.terminal_is_empty(),"selected close source did not finish");}
        assert!(source.terminal_is_empty());
    }
}
