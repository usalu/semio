//! ⚙️ Semio v1 engine — holds the two REAL shared submodules (🧮️geometry, 🧰️triples — see their
//! own doc comments) and `register()`, which registers all 14 subsets' schema descriptors +
//! document codecs + SubsetValidators (13 domain subsets each register a validator; the ✳️any
//! envelope does not — subset `"*"` is exempt per `policyStandardsSubsetVocabularyBreaches`,
//! matching every other stdio artifact's `✳️any`).

pub fn register() {
    crate::artifacts::semio::standards::v1::subsets::brep::composer::register();
    crate::artifacts::semio::standards::v1::subsets::mesh::composer::register();
    crate::artifacts::semio::standards::v1::subsets::model::composer::register();
    crate::artifacts::semio::standards::v1::subsets::object::composer::register();
    crate::artifacts::semio::standards::v1::subsets::document::composer::register();
    crate::artifacts::semio::standards::v1::subsets::cad::composer::register();
    crate::artifacts::semio::standards::v1::subsets::drawing::composer::register();
    crate::artifacts::semio::standards::v1::subsets::image::composer::register();
    crate::artifacts::semio::standards::v1::subsets::video::composer::register();
    crate::artifacts::semio::standards::v1::subsets::audio::composer::register();
    crate::artifacts::semio::standards::v1::subsets::animation::composer::register();
    crate::artifacts::semio::standards::v1::subsets::presentation::composer::register();
    crate::artifacts::semio::standards::v1::subsets::workflow::composer::register();
    crate::artifacts::semio::standards::v1::subsets::any::composer::register();
}
