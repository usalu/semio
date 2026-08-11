//! 🧬️ JsonSnapshot schema (rfc8259/✳️i-json) — reuses the ✳️any subset's `JsonSnapshot` verbatim
//! (the SAME Rust type, same `s.stdio.json` schema id). RFC 7493 I-JSON is a validation-gated
//! dialect STAMP on top of that existing schema, not a new one -- see D4's Tier-1 "same snapshot
//! type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so
//! `🪆️subsets/✳️i-json/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without
//! duplicating the schema definition. The underlying `JsonValue::Object(Vec<JsonMember>)` shape
//! (see the ✳️any schema) is what makes duplicate member names genuinely representable/checkable
//! here -- a `serde_json::Value`-style `Map` would have silently collapsed them on parse.

pub use crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::*;
