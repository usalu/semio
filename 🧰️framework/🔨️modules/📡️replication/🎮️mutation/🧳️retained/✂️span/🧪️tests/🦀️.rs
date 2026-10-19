use semio_framework_test_fixtures::fixture_text;
use super::*;
use semio_framework_value::retained_clone::RetainedCloneSource;

fn quote(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn observe<T>(action:impl FnOnce()->T)->(T,(usize,usize)){let(value,retained,released)=crate::test_allocation::observe_backing(action);(value,(retained,released))}

/// ✂️ Exact original byte-range output and denied work remain independent of expanded physical copy authority.
#[test]
fn retained_byte_span_logical_body_survives_expanded_physical_headers(){
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../📏️body/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let row=&fixture["byteRange"];let bytes=row["source"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u8).collect::<Vec<_>>();let expected=row["expected"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u8).collect::<Vec<_>>();
 for policy in fixture["rows"].as_array().unwrap(){
  let body=policy["body"].as_u64().unwrap() as usize;
  let funding=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:8192,maximum_capacity_bytes:8192,maximum_release_bytes:8192,maximum_depth:128};
  let mut source=match RetainedCloneSource::admit_owned(bytes.clone(),(),funding){Ok((source,_))=>source,Err((error,_,_))=>panic!("original byte range source refused: {error}")};
  let(mut span,heap)=observe(||RetainedByteSpan::new(row["start"].as_u64().unwrap() as usize,row["length"].as_u64().unwrap() as usize));assert_eq!(heap,(0,0));let mut turns=0;
  while !span.ready(){
   let demand=span.demand(source.borrow(),body).unwrap();let exact=quote(demand);let grant=RetainedCloneGrant{maximum_copy_bytes:policy["physical"].as_u64().unwrap() as usize,..exact};
   let denied=[Some(RetainedCloneGrant{maximum_items:0,..grant}),(demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant}),(demand.depth>0).then_some(RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant})];
   for grant in denied.into_iter().flatten(){let(step,heap)=observe(||span.advance(source.borrow(),body,grant).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));}
   let before=span.copied;let(step,heap)=observe(||span.advance(source.borrow(),body,grant).unwrap());assert!(step.progress().fits(exact));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(span.copied-before<=body);turns+=1;assert!(turns<10000);
  }
  let grant=quote(span.take_demand().unwrap());let(value,receipt)=span.take(grant).unwrap().unwrap();assert!(receipt.fits(grant));assert_eq!(value,expected);assert_eq!(source.borrow().get(),&bytes);
  let mut value=ManuallyDrop::new(Some(value));let mut retirement=RetainedCloneClose::default();
  while value.is_some()||!retirement.is_empty(){let demand=retirement.next_owner_demands::<Vec<u8>>(value.is_some(),&None,body).unwrap();let grant=quote(demand);let(step,heap)=observe(||if retirement.is_empty(){retirement.begin_granted(&mut value,grant).unwrap().unwrap()}else{RetainedCloneStep::Progress(retirement.step_granted(grant).unwrap().progress())});assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
  span.begin_close();while !span.terminal_is_empty(){let grant=quote(span.close_demand(body).unwrap());let(step,heap)=observe(||span.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
  while !source.terminal_is_empty(){let grant=quote(source.next_close_demand(body).unwrap());assert!(source.close_step(grant).unwrap().progress().fits(grant));}
  assert_eq!(observe(||drop(span)).1,(0,0));unsafe{ManuallyDrop::drop(&mut value);}
  eprintln!("[DEBUG] Original byte span body={body} preserves exact neutral output and five independent denied currencies");
 }
}
