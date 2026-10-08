use super::*;
use semio_framework_job::{InteractiveJobCloseStep as Step,RetainedCloneGrant,RetainedCloneProgress};

#[test]
fn retained_wire_short_close_conserves_logical_bytes_and_physical_backing(){
 let fixture=semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let mut input=RetainedToolWireInput::try_new(8,8).unwrap();input.admit_page(ToolWirePage::try_copy_from(&42u64.to_le_bytes()).unwrap()).unwrap();input.seal().unwrap();input.begin_close();
 assert_eq!(fixture["shortClose"]["logicalBytes"].as_u64(),Some(8));let pointer=input.pages.as_ptr();let bytes=input.pages.capacity()*std::mem::size_of::<ToolWirePage>();
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:bytes,maximum_depth:1};
 for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_release_bytes:bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let step=input.close_step(denied);assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(input.pages.as_ptr(),pointer);assert_eq!(input.admitted_bytes,8);assert!(!input.terminal_is_empty());}
 assert_eq!(input.close_step(grant),Step::Complete{progress:RetainedCloneProgress{copied_items:1,copied_bytes:0,retained_capacity_bytes:0,released_bytes:bytes}});assert!(input.terminal_is_empty());
 assert_eq!(input.close_step(grant),Step::Complete{progress:RetainedCloneProgress::default()});
}

#[test]
fn retained_wire_input_small_grants_retire_initialized_bytes_and_backing_allocation(){
 let fixture=semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 for row in fixture["cases"].as_array().unwrap(){for copy in fixture["copyGrants"].as_array().unwrap(){
  let declared=row["declared"].as_u64().unwrap()as usize;let admitted=row["admitted"].as_u64().unwrap()as usize;
  let bytes:Vec<u8>=(0..admitted).map(|index|(index%251)as u8).collect();let mut input=RetainedToolWireInput::try_new(declared,declared).unwrap();for page in bytes.chunks(TOOL_WIRE_PAGE_BYTES){input.admit_page(ToolWirePage::try_copy_from(page).unwrap()).unwrap();}if row["sealed"].as_bool().unwrap(){input.seal().unwrap();}
  let physical=input.pages.capacity()*std::mem::size_of::<ToolWirePage>();let pointer=input.pages.as_ptr();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.as_u64().unwrap()as usize,maximum_capacity_bytes:0,maximum_release_bytes:physical,maximum_depth:1};
  assert_eq!(input.next_close_copy_byte_demand().unwrap(),0);assert_eq!(input.next_close_capacity_byte_demand(0).unwrap(),0);assert_eq!(input.next_close_release_byte_demand().unwrap(),physical);assert_eq!(input.next_close_depth_demand().unwrap(),usize::from(physical!=0));assert_eq!(input.pages.as_ptr(),pointer);assert_eq!(input.admitted_bytes,admitted);
  if physical!=0{for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_release_bytes:physical-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{assert_eq!(input.close_step(denied).progress(),RetainedCloneProgress::default());assert_eq!(input.pages.as_ptr(),pointer);assert_eq!(input.pages.iter().flat_map(|page|page.as_slice()).copied().collect::<Vec<_>>(),bytes);}}
  let step=input.close_step(grant);assert!(matches!(step,Step::Complete{..}));assert_eq!(step.progress(),RetainedCloneProgress{copied_items:usize::from(physical!=0),copied_bytes:0,retained_capacity_bytes:0,released_bytes:physical});assert!(input.terminal_is_empty());assert_eq!(input.pages.capacity(),0);assert_eq!(input.admitted_bytes,0);
  eprintln!("[DEBUG] original wire physical release={physical} copy={} admitted={admitted} terminalReceipt={:?}",grant.maximum_copy_bytes,step.progress());
 }}
}
