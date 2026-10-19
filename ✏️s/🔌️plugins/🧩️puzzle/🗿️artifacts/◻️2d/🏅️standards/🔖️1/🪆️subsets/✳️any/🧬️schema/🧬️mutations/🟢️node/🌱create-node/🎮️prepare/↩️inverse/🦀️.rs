//! 🌱️ Native CreateNode inverse uses the shared typed literal-ID owner.

use crate::standards::v1::subsets::any::schema::mutations::{CreateNode};
use crate::standards::v1::subsets::any::schema::mutations::{Puzzle2dMutation};
use crate::standards::v1::subsets::any::schema::preparation::literal_id_inverse::{Puzzle2dLiteralIdInverse, Puzzle2dLiteralIdInverseCursor};
use semio_framework_value::paged::PagedUtf8;

#[cfg(test)]
use semio_framework_value::{ValueError, retirement::controlled::ControlledRetirement, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
#[cfg(test)]
use semio_framework_value_persistent::list::PagedList;
#[cfg(test)]
use std::mem::size_of;

impl Puzzle2dLiteralIdInverse for CreateNode {
    fn inverse_id(&self) -> &PagedUtf8<{usize::MAX}> { &self.node.id }
    fn inverse_payload(id: PagedUtf8<{usize::MAX}>) -> Puzzle2dMutation { Puzzle2dMutation::DeleteNode(crate::standards::v1::subsets::any::schema::mutations::delete_node::DeleteNode { id }) }
}

pub type Puzzle2dCreateNodeInverseCursor = Puzzle2dLiteralIdInverseCursor<CreateNode>;


