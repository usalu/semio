//! 🧪️ Source-owned presence and transient leaves for publication-pipeline laws.

#[path = "../📢️publication-fixtures-presence/🦀️.rs"]
pub mod presence;
#[path = "../📢️publication-fixtures-transient/🦀️.rs"]
pub mod transient;

pub use crate::publication_fixture::presence::mutations::change_publication_presence::ChangePublicationPresence;
pub use crate::publication_fixture::presence::{PublicationPresence, PublicationPresenceDiff};
pub use crate::publication_fixture::presence::mutations::PublicationPresenceMutation;
pub use crate::publication_fixture::transient::mutations::change_publication_transient::ChangePublicationTransient;
pub use crate::publication_fixture::transient::{PublicationTransient, PublicationTransientDiff};
pub use crate::publication_fixture::transient::mutations::PublicationTransientMutation;

