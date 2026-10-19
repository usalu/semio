use super::*;
use crate::observe_retirement_allocations as observe;
struct Payload {text:String}
struct PayloadCursor {text:Option<String>}
impl RetireOwned for Payload{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(PayloadCursor{text:Some(self.text)})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<PayloadCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for PayloadCursor{
 fn next_demand(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:if self.text.is_some(){size_of::<Option<String>>()}else{0},release_bytes:self.text.as_ref().map_or(0,String::capacity),depth:usize::from(self.text.is_some()),..Default::default()})}
 fn close_step(&mut self,g:RetainedCloneGrant)->RetirementStep{let d=self.next_demand(g.maximum_copy_bytes).unwrap();if !crate::retained_clone::retained_clone_grant_admits(g,d){return RetirementStep::BudgetExhausted;}drop(self.text.take());RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})}
 fn terminal_is_empty(&self)->bool{self.text.is_none()}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn step(cursor:&mut dyn RetirementCursor,g:RetainedCloneGrant)->(RetirementStep,(usize,usize)){
 for axis in 0..5{let mut denied=g;let positive=match axis{0=>{denied.maximum_items=0;true},1=>{denied.maximum_copy_bytes=g.maximum_copy_bytes.saturating_sub(1);g.maximum_copy_bytes>0},2=>{denied.maximum_capacity_bytes=g.maximum_capacity_bytes.saturating_sub(1);g.maximum_capacity_bytes>0},3=>{denied.maximum_release_bytes=g.maximum_release_bytes.saturating_sub(1);g.maximum_release_bytes>0},_=>{denied.maximum_depth=g.maximum_depth.saturating_sub(1);g.maximum_depth>0}};if positive{let(result,heap)=observe(||cursor.close_step(denied));assert!(matches!(result,RetirementStep::BudgetExhausted|RetirementStep::Progress(RetainedCloneProgress{copied_items:0,copied_bytes:0,retained_capacity_bytes:0,released_bytes:0})));assert_eq!(heap,(0,0));}}
 observe(||cursor.close_step(g))
}
/// 🚚️ Last original aliases preserve exact native payload custody on the caller frontier without nested cleanup queues.
#[test]
fn original_sealed_owned_payload_hands_native_child_to_caller_frontier(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){let((source,text_bytes),mut total)=observe(||{let text=row["text"].as_str().unwrap().to_owned();let bytes=text.capacity();let g=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:SealedShared::<Payload>::birth_bytes(),maximum_depth:1,..Default::default()};(SealedShared::admit(Payload{text},SharedIssuer::owned(),g).unwrap_or_else(|_|panic!("original payload birth")).0,bytes)});let identity=source.identity();let mut aliases=Vec::new();for _ in 1..row["aliases"].as_u64().unwrap(){aliases.push(source.try_duplicate(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<SealedShared<Payload>>(),maximum_depth:1,..Default::default()}).unwrap().0);}aliases.push(source);let mut handoffs=0;
 for alias in aliases{assert_eq!(alias.identity(),identity);let birth=alias.retirement_birth_bytes().unwrap();let(mut cursor,heap)=observe(||alias.retirement());assert_eq!(heap,(birth,0));total.0+=heap.0;
 let d=cursor.next_demand(4096).unwrap();let g=grant(d);let(result,heap)=step(&mut*cursor,g);let RetirementStep::Progress(p)=result else{panic!("original strong alias turn requires physical receipt")};assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert!(p.fits(g));total.0+=heap.0;total.1+=heap.1;
 if !cursor.terminal_is_empty(){let d=cursor.next_demand(4096).unwrap();assert_eq!(d.capacity_bytes,size_of::<PayloadCursor>(),"original typed cursor birth excludes an extra owned wrapper and nested frontier");let g=grant(d);let(result,heap)=step(&mut*cursor,g);let RetirementStep::ChildProgress(mut child,p)=result else{panic!("original typed payload must transfer its native child to caller frontier")};handoffs+=1;assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert!(p.fits(g));total.0+=heap.0;total.1+=heap.1;let g=grant(child.next_demand(4096).unwrap());let(result,heap)=step(&mut*child,g);let RetirementStep::Progress(p)=result else{panic!("native payload original physical close")};assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert_eq!(p.released_bytes,text_bytes);total.0+=heap.0;total.1+=heap.1;assert!(child.terminal_is_empty());let release=child.terminal_release_bytes().unwrap();let(_,heap)=observe(||drop(child));assert_eq!(heap,(0,release));total.1+=heap.1;}
 assert!(cursor.terminal_is_empty());let release=cursor.terminal_release_bytes().unwrap();let(_,heap)=observe(||drop(cursor));assert_eq!(heap,(0,release));total.1+=heap.1;}
 assert_eq!(handoffs,row["expectedChildHandoffs"].as_u64().unwrap());assert_eq!(total.0,total.1);eprintln!("[DEBUG] original sealed native handoffs={handoffs} nestedQueues=0 physicalBirth={} physicalRelease={}",total.0,total.1);
 }
}
