//! 📝️ Own14 path operation uses its original canonical patch text.
use crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::PatchSnapshot;
use crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation;
pub const OPCODE:&str="patch-snapshot";
pub fn print(mutation:&PdfMutation)->Option<String>{let PdfMutation::PatchSnapshot(payload)=mutation else{return None};Some(protocol::OpText::print_op(&payload.patch))}
pub fn parse(payload:&str)->Result<PdfMutation,semio_framework_diagnostic::TextError>{protocol::OpText::parse_op(payload).map(|patch|PdfMutation::PatchSnapshot(PatchSnapshot{patch}))}
