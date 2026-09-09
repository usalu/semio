//! 📚️ Bundled energy-model demo example.
//!
//! The demo carries a REAL building: ANSI/ASHRAE 140 §5.2 case 600, the standard's own base case.
//! It used to be an id/label pair with no model behind it at all, which left the committed
//! `../../🖼️assets/🎬️demo/🗣️.dsl.semio` without a `model=` line and therefore unparseable by this subset's own
//! `ArtifactDsl` codec — the identity round-trip law had nothing to round-trip.

/// @emoji 🪪 Example id.
pub const ID: &str = "demo";

/// @emoji 🏷️ Example label.
pub const LABEL_EN: &str = "Demo";

/// @emoji 🏛️ The ANSI/ASHRAE 140 §5.2 case this demo carries.
pub const CASE: &str = "600";

/// @emoji 🏗️ This example's model.
pub fn model() -> crate::model::Model {
    crate::bestest::model(CASE).expect("ANSI/ASHRAE 140 §5.2 case 600 is registered")
}
