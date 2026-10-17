//! 🧰️ `Puzzle5dMutation` support: the diff builders and base accessors its mutation leaves call, hosted below the leaves so that no leaf has to import the aggregate module.

/// 🎛️ The ONE board↔world scale this artifact places and moves paired parts with — the linear inverse of
/// `🧬️schema/💡️inferences/🎛️flat-position`'s plan projection, so a flat point and a world origin stay one
/// consistent pair whichever pane the gesture came from. A flat unit is one board pixel; 48 of them make
/// one world metre (the board's own default part box, `part_2d.width`/`height`).
pub const PUZZLE5D_FLAT_TO_WORLD: f64 = 1.0 / 48.0;

/// 📏️ A pose scale multiplied per axis by `factors`, always as a per-axis triple: a uniform scalar broadcasts
/// first and an absent scale reads as `[1, 1, 1]`.
pub fn puzzle5d_scaled(scale: Option<crate::standards::v1::subsets::any::schema::document_records::Puzzle5dScale>, factors: [f64; 3]) -> crate::standards::v1::subsets::any::schema::document_records::Puzzle5dScale {
    let current = match scale {
        Some(crate::standards::v1::subsets::any::schema::document_records::Puzzle5dScale::Uniform(value)) => [value; 3],
        Some(crate::standards::v1::subsets::any::schema::document_records::Puzzle5dScale::Vec3(value)) => value,
        None => [1.0; 3],
    };
    crate::standards::v1::subsets::any::schema::document_records::Puzzle5dScale::Vec3([current[0] * factors[0], current[1] * factors[1], current[2] * factors[2]])
}
