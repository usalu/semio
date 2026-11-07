//! 📥️ Every persisted sourcing operation, binary-encoded, is received by its own original cursor into the value the decode oracle returns.
use semio_s_artifact_sourcing_curation::{CuratedItem,SourcingMutation};
use semio_s_artifact_sourcing_curation::schema::mutations::{ChangeCuratedItemCount,CreateCuratedItem,DeleteCuratedItem};

fn item(id:&str,count:u32)->CuratedItem{CuratedItem{object_id:id.into(),count}}

#[test]
fn every_sourcing_operation_receives_to_its_decode_oracle(){
 let values=vec![
  SourcingMutation::CreateCuratedItem(CreateCuratedItem{item:item("object-a",3),index:None}),
  SourcingMutation::CreateCuratedItem(CreateCuratedItem{item:item("object-b",1),index:Some(7)}),
  SourcingMutation::DeleteCuratedItem(DeleteCuratedItem{object_id:"object-a".into()}),
  SourcingMutation::ChangeCuratedItemCount(ChangeCuratedItemCount{object_id:"object-b".into(),new_count:9}),
 ];
 semio_os_kernel::os_dsl::original_receiving::check_receiving_round_trip(&values).unwrap();
}
