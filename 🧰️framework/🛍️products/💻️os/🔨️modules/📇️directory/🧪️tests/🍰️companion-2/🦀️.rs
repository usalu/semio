//! 🍰️ Companion tests of os-directory (2 of 2): binary IO, browser actor, document check-in and root unit tests.
//!
//! 🧭️ The tests are mounted unchanged from the production tree and reach the production crate through its public API only.
#![cfg(test)]

pub use semio_framework_os_directory::*;

#[path = "../🔬️unit/🦀️.rs"]
mod tests;

#[path = "."]
pub mod io {
    pub use semio_framework_os_directory::io::*;

    #[path = "."]
    pub mod binary {
        pub use semio_framework_os_directory::io::binary::*;

        #[path = "../../🚪️io/🧱️binary/🪢️checkpoint-pair/🧪️tests/🔬️unit"]
        pub mod checkpoint_pair {
            pub use semio_framework_os_directory::io::binary::checkpoint_pair::*;
            use semio_framework_os_directory::schema::{ArtifactFrontier, ArtifactHash, DocumentScope};
            use semio_framework_os_directory::CanonicalCheckpointPairRefusalV1;

            #[path = "🦀️.rs"]
            mod tests;
        }

        #[path = "../../🚪️io/🧱️binary/🪪️artifact-hash/🧪️tests"]
        pub mod artifact_hash {
            pub use semio_framework_os_directory::io::binary::artifact_hash::*;
            use semio_framework_os_directory::schema::{ArtifactHash, EditedArtifactFrontierV1};

            #[path = "🦀️.rs"]
            mod tests;
        }

        #[path = "../../🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests"]
        pub mod descriptor_digest {
            pub use semio_framework_os_directory::io::binary::descriptor_digest::*;
            use semio_framework_os_directory::schema::{ArtifactHash, DocumentDescriptor, validate_document_descriptor_v1};

            #[path = "🦀️.rs"]
            mod tests;
        }
    }
}

#[path = "."]
pub mod schema {
    pub use semio_framework_os_directory::schema::*;

    #[path = "../../🧬️schema/🌐️browser-actor/🧪️tests/🔬️unit"]
    pub mod browser_actor {
        pub use semio_framework_os_directory_browser_actor::*;

        #[path = "🦀️.rs"]
        mod tests;
    }

    #[path = "../../🧬️schema/📌️document-check-in-v1/🧪️tests/🔬️unit"]
    pub mod document_check_in {
        pub use semio_framework_os_directory_check_in::*;
        use semio_framework_os_directory::schema::EditedArtifactFrontierV1;

        #[path = "🦀️.rs"]
        mod tests;
    }
}
