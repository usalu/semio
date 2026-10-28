//! ⏪️ An original one-edit history transition maps a private replay order to unchanged source ledger slots.
use super::ArtifactHistoryKey;
/// 🪪️ The enclosing Store validates the genuine ledger key and canonical insertion or removal position before replay birth.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalHistoryEditSelection{pub key:ArtifactHistoryKey,pub position:usize,pub redo_position:usize,pub restoring:bool}
impl OriginalHistoryEditSelection{
 /// 📐️ The new order changes by exactly one canonical original edit.
 pub fn selected_len(self,original:usize)->Option<usize>{if self.restoring{original.checked_add(1).filter(|_|self.position<=original)}else{original.checked_sub(1).filter(|_|self.position<original)}}
 /// 📍️ None names the reinstated original ledger key; every other row keeps its original applied ordinal.
 pub fn original_position(self,selected:usize)->Option<usize>{if self.restoring{if selected==self.position{None}else{Some(selected-usize::from(selected>self.position))}}else{Some(selected+usize::from(selected>=self.position))}}
}
semio_framework_value::artifact_retire_leaf!(OriginalHistoryEditSelection);
#[cfg(test)]mod tests{use super::*;#[test]fn original_history_edit_order_selection_matches_neutral_rows(){let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();for row in fixture["cases"].as_array().unwrap(){let selection=OriginalHistoryEditSelection{key:ArtifactHistoryKey{index:1,generation:1},position:row["position"].as_u64().unwrap()as usize,redo_position:0,restoring:row["restoring"].as_bool().unwrap()};let original=row["original"].as_array().unwrap();let result=(0..selection.selected_len(original.len()).unwrap()).map(|index|selection.original_position(index).map_or(1,|index|original[index].as_u64().unwrap())).collect::<Vec<_>>();assert_eq!(serde_json::to_value(result).unwrap(),row["selected"]);}}}
