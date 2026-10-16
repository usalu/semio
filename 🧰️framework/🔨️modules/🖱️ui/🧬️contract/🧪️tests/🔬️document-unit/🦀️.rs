//! 🧪️ Actual canonical record fixture shared by private document arena laws.
use super::*;
pub(super)fn leaf_record(id:u64,key:&str)->UiNodeRecord{UiNodeRecord::with_component(UiNodeId(id),crate::UiText::try_from_str(key).unwrap(),crate::Component::Separator(crate::SeparatorProps{}))}
