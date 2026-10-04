//! 🪪️ All captured source authority fields belong to one publication.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct SceneIdentity{pub instance:u32,pub base:u64,pub generation:u64,pub revision:[u8;32]}
impl SceneIdentity{
 pub fn matches(self,live:Self)->bool{self.instance!=0&&self==live}
}
/// 🔢️ Admit only the exact decimal spelling of a full-width native source lane.
pub fn parse_scene_identity_u64(value:&str)->Option<u64>{if value.is_empty()||value.len()>20||value.len()>1&&value.starts_with('0')||!value.bytes().all(|byte|byte.is_ascii_digit()){return None;}value.parse().ok()}
#[path="🚦️admission/🦀️.rs"]
pub mod admission;
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
