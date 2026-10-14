//! 🔗️ Native ConnectHandles inverse uses the shared typed literal-ID owner.

use super::ConnectHandles;
use crate::standards::v1::subsets::any::schema::mutations::{Puzzle2dMutation, literal_id_inverse::{Puzzle2dLiteralIdInverse, Puzzle2dLiteralIdInverseCursor}};
use semio_framework_value::paged::PagedUtf8;

#[cfg(test)]
use semio_framework_value::{list::PagedList, ValueError, retirement::controlled::ControlledRetirement, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
#[cfg(test)]
use std::mem::size_of;

impl Puzzle2dLiteralIdInverse for ConnectHandles {
    fn inverse_id(&self) -> &PagedUtf8<{usize::MAX}> { &self.id }
    fn inverse_payload(id: PagedUtf8<{usize::MAX}>) -> Puzzle2dMutation { Puzzle2dMutation::DisconnectHandles(crate::standards::v1::subsets::any::schema::mutations::disconnect_handles::DisconnectHandles { id }) }
}

pub type Puzzle2dConnectHandlesInverseCursor = Puzzle2dLiteralIdInverseCursor<ConnectHandles>;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
