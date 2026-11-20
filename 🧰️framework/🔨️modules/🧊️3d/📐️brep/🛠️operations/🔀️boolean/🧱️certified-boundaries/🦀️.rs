//! 🧱️ Exact boundary recipes for supported solid combinations.

use semio_framework_value::{ValueError, ValueRefusalKind};

#[path = "🧱️orthogonal-boundary/🦀️.rs"]
mod orthogonal_boundary;
pub use orthogonal_boundary::{OrthogonalBoundary, MAX_ORTHOGONAL_ENTITIES};

#[path = "🕳️bore-boundary/🦀️.rs"]
mod bore_boundary;
pub use bore_boundary::BoreBoundary;

#[path = "🥫️coaxial-cylinders/🦀️.rs"]
mod coaxial_cylinders;
pub use coaxial_cylinders::coaxial_cylinders;

pub fn empty_result() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "original Boolean result is empty")
}

pub fn unsupported() -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original Boolean requires a certified result topology")
}
