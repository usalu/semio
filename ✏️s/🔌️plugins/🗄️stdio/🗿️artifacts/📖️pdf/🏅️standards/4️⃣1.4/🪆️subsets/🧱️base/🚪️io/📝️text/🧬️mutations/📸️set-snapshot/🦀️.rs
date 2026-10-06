//! 📝️ Own14 readable replacement using its original structured snapshot DSL.
use crate::standards::v1_4::subsets::base::schema::mutations::SetSnapshot;
use crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation;
use store::ArtifactDsl;
pub const OPCODE:&str="set-snapshot";
pub fn print(mutation:&PdfMutation)->Option<String>{let PdfMutation::SetSnapshot(payload)=mutation else{return None};Some(payload.snapshot.print_dsl())}
pub fn parse(payload:&str)->Result<PdfMutation,semio_framework_diagnostic::TextError>{crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot::parse_dsl(payload).map(|snapshot|PdfMutation::SetSnapshot(SetSnapshot{snapshot}))}
