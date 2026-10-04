//! 🎯️ Bound selected-group contribution separately from locked leaf picking.
use super::{validate_scene_source_address,DocumentSceneError};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct SceneSelectionRelation{pub pickable:bool,pub bounds_selection:Option<usize>}
/// 🧭️ Preserve the outermost unlocked selected prefix through ordinary groups.
pub fn scene_selection_relation(source_path:&[u16],locked_ancestors:u32,selected_paths:&[impl AsRef<[u16]>])->Result<SceneSelectionRelation,DocumentSceneError>{
 validate_scene_source_address(source_path,locked_ancestors)?;
 if selected_paths.len()>256{return Err(DocumentSceneError::Invalid("Scene selection exceeds ancestry capacity".into()));}
 let mut bounds_selection=None;let mut depth=33;
 for(index,selected)in selected_paths.iter().enumerate(){let selected=selected.as_ref();
  validate_scene_source_address(selected,0)?;
  let mask=if selected.len()==32{u32::MAX}else{(1u32<<selected.len())-1};
  if selected.len()<depth&&source_path.starts_with(selected)&&locked_ancestors&mask==0{bounds_selection=Some(index);depth=selected.len();}
 }
 Ok(SceneSelectionRelation{pickable:locked_ancestors==0,bounds_selection})
}
