//! 🧪️ Runs private Original receiving custody laws on their authored owner.

use semio_framework_pack_json::{
    ArtifactCanonicalJsonTree, ArtifactCanonicalJsonTreeCursor, JsonError, JsonGrammarCursor, JsonMemberPolicy,
};

#[path = "."]
mod original_receiving {
    include!("../../🦀️.rs");
    #[path="../../🧪️tests/🧰️support/🦀️.rs"]
    mod test_support;

    #[path="../../🧪️tests/🦀️.rs"]
    mod tests;

    #[path="../../🎟️close/🧪️tests/🦀️.rs"]
    mod close_work_tests;
}
