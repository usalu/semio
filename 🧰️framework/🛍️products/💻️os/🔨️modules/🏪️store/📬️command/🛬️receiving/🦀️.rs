//! 📬️ Original binary command receiving retains every partial typed field in its initiating native recipient.
use crate::os_spr::{self,MutationId,TransactionRef};
use crate::os_vcs::Author;
use super::{ArtifactCommand,HistoryLane,SupersedeInput};
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,reason)}
fn lane(value:u8)->Result<HistoryLane,ValueError>{match value{0=>Ok(HistoryLane::Document),1=>Ok(HistoryLane::Interaction),_=>Err(invalid("original history lane is invalid"))}}
#[path="🪜️cursor/🦀️.rs"]
mod cursor;
pub use cursor::{RetainedArtifactCommandReceiving,RetainedCommandEnvelopeReceiving};
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
