//! 🧬️ TiffSnapshot schema (6.0/✳️baseline) — reuses the ✳️any subset's `TiffSnapshot` verbatim
//! (the SAME Rust type, same `s.stdio.tiff` schema id). A subset is a validation-gated dialect
//! STAMP on top of that existing schema, not a new one -- see D4's Tier-1 "same snapshot type,
//! subset moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so
//! `🪆️subsets/✳️baseline/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without
//! duplicating the schema definition.
//!
//! Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: `TiffSnapshot`
//! now retains the REAL IFD (tag/type/count/value entries) — `Compression`/
//! `PhotometricInterpretation`/`BitsPerSample`/`StripOffsets`/`Tile*` are all genuinely present
//! and checkable. `🧐️analyzer` here now implements real Baseline TIFF conformance checks
//! against those fields (superseding the earlier ticket 26/08/11's schema-gap-only revision).
//! See `🧐️analyzer` for the full accounting.

pub use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::*;
