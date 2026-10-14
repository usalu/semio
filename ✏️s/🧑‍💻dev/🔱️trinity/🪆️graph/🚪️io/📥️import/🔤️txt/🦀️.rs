//! 📦️ Specific Jack text carrier retains the complete original native archive.
use store::io::io_mechanism::IoRunControl;
use store::io_schema::{IoPayload,IoResult};
pub(super) fn run(payload:&IoPayload,control:&mut IoRunControl<'_,'_>)->IoResult<IoPayload>{super::import(payload,super::Format::Text,control)}
