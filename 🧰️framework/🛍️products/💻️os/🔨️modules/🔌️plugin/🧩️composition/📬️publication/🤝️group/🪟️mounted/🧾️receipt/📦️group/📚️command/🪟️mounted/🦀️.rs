use crate::mounted_child_group_receipt_group::command::CommandPrune;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use store::MemberFactory;
use store::SpaceMember;
#[doc(hidden)]
pub struct MountedCommandPrune {
    pub cursor: CommandPrune,
    pub requested: u64,
    pub revision: [u8; 32],
    pub generation: u64,
    pub edits: usize,
    pub transitions: usize,
    pub field: usize,
    pub index: usize,
    pub offset: usize,
    pub keep: bool,
}

impl MountedCommandPrune {
    pub fn equal_prefix(left: &str, right: &str, offset: &mut usize) -> Option<bool> {
        if left.len() != right.len() { *offset = 0; return Some(false); }
        let end = (*offset + (left.len() - *offset).min(64)).min(left.len());
        if left.as_bytes()[*offset..end] != right.as_bytes()[*offset..end] { *offset = 0; return Some(false); }
        *offset = end;
        (end == left.len()).then_some(true)
    }
}
