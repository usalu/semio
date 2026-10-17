//! 💰️ Inline IFC4 roots have no concrete heap storage request.
use super::*;
#[test]
fn sqlite_snapshot_ifc4_inline_root_has_only_concrete_backing(){
 let mut progress=|_|true;let mut control=NativeDecodeControl::new(0,&mut progress);
 let frame=Frame{schema:String::new(),file_description:Vec::new(),file_name:Vec::new(),file_schema:Vec::new(),entities:Vec::new(),values:Vec::new()};
 let result=reconstruct(frame,&mut control).expect("an inline IFC4 root has no heap backing");assert_eq!(control.owned_bytes(),0);close(result);
}
