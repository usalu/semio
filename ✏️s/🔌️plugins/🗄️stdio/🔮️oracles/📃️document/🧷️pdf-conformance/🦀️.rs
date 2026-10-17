//! 🧷️ The PDF conformance-class engine of the document test oracle: the profile, the object-graph scans and builders, the forward mutations and the arrange, inverse and projection halves. See [`super`].

use lopdf::{Dictionary, Document, Object, ObjectId, Stream};
use semio_repo_test_host::{digest, Json};

#[path = "🔎️profile-container-and-scans/🦀️.rs"]
mod profile_container_and_scans;
pub use profile_container_and_scans::*;

#[path = "⚙️forward-mutation/🦀️.rs"]
mod forward_mutation;
pub use forward_mutation::*;

#[path = "🔁️arrange-inverse-and-projection/🦀️.rs"]
mod arrange_inverse_and_projection;
pub use arrange_inverse_and_projection::*;
