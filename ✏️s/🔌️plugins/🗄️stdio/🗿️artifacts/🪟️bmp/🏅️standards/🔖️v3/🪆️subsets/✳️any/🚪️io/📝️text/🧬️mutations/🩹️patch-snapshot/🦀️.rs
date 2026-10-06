//! 📝️ Direct patch-snapshot text codec: `patch-snapshot patch=<hex of the patch JSON>`.

use crate::standards::v_v3::subsets::any::io::text::mutations::*;
use crate::standards::v_v3::subsets::any::io::text::mutations::Entry;

pub const TEXT_OPCODE: &str = "patch-snapshot";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };
pub fn print(value: &BmpMutation) -> Option<String> {
    let BmpMutation::PatchSnapshot(PatchSnapshot { patch }) = value else { return None };
    Some(editing::snapshot_patch_text(patch))
}
pub fn parse(line: &str) -> Result<BmpMutation, semio_framework_diagnostic::TextError> {
    editing::snapshot_patch_from_text(line).map(|patch| BmpMutation::PatchSnapshot(PatchSnapshot { patch })).map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
