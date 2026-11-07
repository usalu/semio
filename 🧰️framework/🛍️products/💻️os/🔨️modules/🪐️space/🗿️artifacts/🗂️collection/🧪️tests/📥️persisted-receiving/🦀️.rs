//! 📥️ Every persisted collection operation, binary-encoded, is received by its own original cursor into the value the decode oracle returns.
use semio_framework_artifact_space_collection::{ArtifactBody,CollectionEntry,CollectionFolder,CollectionMutation};

fn entry(id:&str,body:ArtifactBody)->CollectionEntry{CollectionEntry{id:id.into(),folder_id:Some("folder".into()),name:format!("name {id}"),kind_id:"kind".into(),body:Box::new(body)}}
fn document()->ArtifactBody{ArtifactBody::Document{schema:"s.note".into(),document_id:"note-1".into()}}
fn blob()->ArtifactBody{ArtifactBody::Blob{blob:semio_os_kernel::BlobRef{hash:"abc".into(),size:42,media_type:"model/stl".into()}}}

#[test]
fn every_collection_operation_receives_to_its_decode_oracle(){
 let values=vec![
  CollectionMutation::RenameCollection{new_name:"Collection".into()},
  CollectionMutation::CreateFolder{folder:CollectionFolder{id:"folder".into(),parent_id:None,name:"Folder".into()},index:0},
  CollectionMutation::CreateFolder{folder:CollectionFolder{id:"child".into(),parent_id:Some("folder".into()),name:"Child".into()},index:3},
  CollectionMutation::DeleteFolder{folder_id:"folder".into()},
  CollectionMutation::MoveToCollection{folder_id:"child".into(),new_parent:None},
  CollectionMutation::MoveToCollection{folder_id:"child".into(),new_parent:Some("folder".into())},
  CollectionMutation::RenameFolder{folder_id:"child".into(),new_name:"Renamed".into()},
  CollectionMutation::CreateEntry{entry:entry("doc",document()),index:1},
  CollectionMutation::CreateEntry{entry:entry("blob",blob()),index:2},
  CollectionMutation::DeleteEntry{entry_id:"doc".into()},
  CollectionMutation::MoveToFolder{entry_id:"doc".into(),new_folder:Some("folder".into())},
  CollectionMutation::MoveToFolder{entry_id:"doc".into(),new_folder:None},
  CollectionMutation::RenameEntry{entry_id:"doc".into(),new_name:"Renamed".into()},
  CollectionMutation::ReplaceEntryBody{entry_id:"doc".into(),new_body:Box::new(document())},
  CollectionMutation::ReplaceEntryBody{entry_id:"blob".into(),new_body:Box::new(blob())},
 ];
 semio_os_kernel::os_dsl::original_receiving::check_receiving_round_trip(&values).unwrap();
}
