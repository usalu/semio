//! 🔮️ Mutation oracle for this subset — a `pub use` of AC1024's, because this subset declares no
//! vocabulary of its own to write an oracle against.
//!
//! `../🧬️schema/🧬️mutations/🦀️component.rs` is `pub use crate::artifacts::dwg::standards::v_ac1024::
//! subsets::any::schema::mutations::*;`, and so are this subset's `🧬️schema` and `📸️snapshot`
//! facets. `DwgMutation` is therefore ONE Rust enum shared by both standards, not two enums that
//! happen to agree: the `dwg-ac1018-any` and `dwg-ac1024-any` catalogs declare the same three kinds
//! because they are the same three kinds, identical by construction rather than by a copy that
//! could silently rot. `every_ac1018_facet_is_a_re_export_of_this_one`, in the AC1024 oracle
//! module's own tests, checks that claim against the committed sources instead of asserting it in
//! prose — the moment any of those facets stops being a re-export, that test fails and the two
//! catalogs stop being entitled to be identical.
//!
//! The oracle logic is likewise not duplicated. It is version-agnostic by construction: it reads
//! and writes the six-character version string wherever it finds it rather than requiring `AC1018`
//! or `AC1024`, which is exactly what a `set-version-info` oracle has to do — the version label is
//! the DATA this vocabulary mutates, never a precondition for reading the file.
//!
//! @see ../../../🔖️ac1024/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs — the implementation itself.
//! @see ../🧪️oracle/🔣️component.json — this subset's own catalog and no-oracle decision.

pub use crate::artifacts::dwg::standards::v_ac1024::subsets::any::*;
