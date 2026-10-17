//! 💰️ IFC2x3 inline reconstruction requests no heap backing.
use super::*;
#[test]
fn sqlite_snapshot_ifc2x3_inline_root_has_no_heap_backing(){
    let mut progress=|_|true;let mut native=NativeDecodeControl::new(0,&mut progress);
    let frame=Frame{schema:String::new(),file_description:Vec::new(),file_name:Vec::new(),file_schema:Vec::new(),instances:Vec::new(),values:Vec::new(),edm:None};
    let result=reconstruct(frame,&mut native).expect("inline empty IFC2x3 root has no heap backing");assert_eq!(native.owned_bytes(),0);
    assert!(result.schema.is_empty());assert!(result.document.instances.is_empty());assert!(result.edm_preamble.is_none());close(result);
}
