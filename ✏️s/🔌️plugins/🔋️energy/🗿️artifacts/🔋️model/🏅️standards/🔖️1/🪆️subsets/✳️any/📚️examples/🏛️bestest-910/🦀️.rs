//! 🏛️ ANSI/ASHRAE 140 §5.2 case 910 as a bundled energy-model example.
//!
//! The model itself is authored once in `crate::bestest` — the standard defines every case as a
//! delta from case 600, so a per-example copy of the geometry would be a second source of truth.

/// @emoji 🪪 Example id.
pub const ID: &str = "bestest-910";

/// @emoji 🏷️ Example label.
pub const LABEL_EN: &str = "BESTEST 910";

/// @emoji 🏛️ The ANSI/ASHRAE 140 §5.2 case id this example carries.
pub const CASE: &str = "910";

/// @emoji 🏗️ This example's model.
pub fn model() -> crate::model::Model {
    crate::bestest::model(CASE).expect("ANSI/ASHRAE 140 §5.2 case 910 is registered")
}
