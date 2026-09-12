//! 🔮️ Mutation oracle for `ap214/1️⃣cc1` — ISO 10303-214 CC1 (config data only).
//!
//! Reference: `ruststep` 0.4, the registered independent READER (it has no writer at all), plus this
//! standard's own from-scratch Part-21 writer and an independent re-derivation of the §4.3
//! conformance ladder — both in `../../../🦀️oracle.rs`, shared by all seven `ap214`
//! subsets so no classification or serialization step is copied per class.
//!
//! 🎯️ This dispatcher performs the vocabulary of a CONFORMANCE CLASS, not of the Part-21 grammar.
//! The sibling `../../🧱️base` subset declares eleven grammar verbs (insert an entity, set an argument…) that
//! would read the same for any ISO 10303-21 file on earth; the 5 kinds here are one per axis
//! `check_cc1_conformance` actually reads, and the projection reports those axes and nothing else.
//!
//! 🪜️ **CC1 is the only class with no ceiling type.** `rung_of` never answers below 2, so
//! `ceiling_type_of(1)` is `None` and no representation is admissible at all. This dispatcher
//! therefore has no `set-shape-representation` arm to write one and no `demote-shape-representation`
//! arm to move one down: its single ladder verb is `remove-shape-representation`, and the
//! `aboveCeiling` member of the projection counts EVERY representation in the file rather than only
//! the ones above some rung. Against the real fixture that is both of them — the rung-6 `#13` and
//! the rung-2 `#836`.
//!
//! ⚠️ The ladder classification here is re-derived from ISO 10303-214 §4.3, never called out of the
//! production `engine::ladder` — the oracle crate cannot link the production crate, and an oracle
//! that asked the code under test how to classify would compare an implementation with itself.
//!
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the vocabulary this module is measured against.
//! @see 🔣️.json — the `step-ap214-cc1` catalog and the `ruststep` registration it carries.

use semio_repo_test_host::Json;

//#region 🔖️Class
/// 🪜️ This class's ceiling rung, matching `MAX_RUNG` in `../🧬️schema/🦀️component.rs`.
pub const MAX_RUNG: u8 = 1;

/// 🏷️ How this class names itself in a refusal.
const CLASS: &str = "ISO 10303-214 CC1 (config data only)";

/// 🏷️ The declared vocabulary, mirroring `StepCc1Mutation`'s own variants in declaration order.
/// Duplicated rather than imported: the oracle crate must never link the production crate.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-file-schema", "set-product-identity", "remove-shape-representation"];
//#endregion 🔖️Class

#[cfg(feature = "oracles")]
mod oracles {
    use super::{CLASS, MAX_RUNG};
    use crate::artifacts::step::standards::v_ap214::reference::{ladder, part21};
    use semio_repo_test_host::Json;

    fn params_of(spec: &Json) -> Json {
        spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()))
    }

    /// 🌱 The minimal exchange structure `set-snapshot` builds when it is given fields rather than a
    /// document: a header carrying the stated schema and, optionally, a product identity chain — and
    /// nothing on the ladder, which is the one shape every conformance class accepts.
    ///
    /// The seed carries all three records ISO 10303-21 §8.2 makes mandatory in `HEADER` —
    /// `FILE_DESCRIPTION`, `FILE_NAME`, `FILE_SCHEMA`. That is not decoration: `ruststep` rejects a
    /// header with none of them outright ("expected '(', found ;" at the `ENDSEC`), which is the
    /// reader correctly refusing a document the standard does not permit.
    fn minimal_document(params: &Json) -> Result<Vec<u8>, String> {
        let seed = b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;\n";
        let mut exchange = part21::read(seed)?;
        let schemas = part21::str_array(params, "fileSchema");
        if schemas.is_empty() {
            return Err("set-snapshot requires a non-empty fileSchema field, or a documentText to restore".to_string());
        }
        part21::set_file_schema_names(&mut exchange, &schemas);
        if let Some(identity) = params.get("productIdentity").filter(|value| !matches!(value, Json::Null)) {
            ladder::set_product_identity(&mut exchange, Some(identity))?;
        }
        Ok(part21::write(&exchange))
    }

    /// 🦠️ One declared kind, performed against a real independently-parsed exchange structure and
    /// re-serialized from the parsed model alone. An unrecognised kind is an error, never a silent
    /// no-op: a quietly skipped mutation reports as a passing test.
    pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        if kind == "set-snapshot" {
            return match params.get("documentText") {
                Some(Json::String(text)) => Ok(part21::write(&part21::read(text.as_bytes())?)),
                _ => minimal_document(params),
            };
        }
        let mut exchange = part21::read(input)?;
        match kind {
            "no-mutation" => {}
            "set-file-schema" => {
                let schemas = part21::str_array(params, "schemas");
                if schemas.is_empty() {
                    return Err(format!("{CLASS} requires FILE_SCHEMA to declare a schema -- an empty declaration is not an AP214 exchange structure"));
                }
                part21::set_file_schema_names(&mut exchange, &schemas);
            }
            "set-product-identity" => {
                let identity = params.get("identity").filter(|value| !matches!(value, Json::Null));
                ladder::set_product_identity(&mut exchange, identity)?;
            }
            "remove-shape-representation" => ladder::remove_representation(&mut exchange, part21::u64_field(params, "id")?)?,
            other => return Err(format!("mutation kind {other:?} has no oracle implementation in {CLASS}")),
        }
        Ok(part21::write(&exchange))
    }

    /// ↩️ The inverse of `(kind, params)` against the UNMUTATED `base`, computed independently here
    /// and mirroring `StepCc1Mutation::inverse()`'s own base-relative semantics.
    ///
/// `remove-shape-representation` inverts to a whole-document restore: putting a representation back
/// is a state CC1 forbids, so no in-class verb can express it. That is the production vocabulary's
/// own answer too (`StepCc1Mutation::inverse`), independently arrived at here.
    pub fn inverse_spec(base: &[u8], kind: &str, _params: &Json) -> Result<Json, String> {
        let exchange = part21::read(base)?;
        let object = |entries: Vec<(&str, Json)>| Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
        let restore = || object(vec![("documentText", Json::String(String::from_utf8_lossy(&part21::write(&exchange)).to_string()))]);
        let (inverse_kind, inverse_params) = match kind {
            "no-mutation" => ("no-mutation", Json::Object(Vec::new())),
            "set-snapshot" => ("set-snapshot", restore()),
            "set-file-schema" => ("set-file-schema", object(vec![("schemas", Json::Array(part21::file_schema_names(&exchange).into_iter().map(Json::String).collect()))])),
            "set-product-identity" => ("set-product-identity", object(vec![("identity", ladder::product_identity_json(&exchange))])),
            "remove-shape-representation" => ("set-snapshot", restore()),
            other => return Err(format!("mutation kind {other:?} has no oracle inverse in {CLASS}")),
        };
        Ok(object(vec![("kind", Json::String(inverse_kind.to_string())), ("params", inverse_params)]))
    }

    /// 👁️ The conformance-class projection — the three axes `check_cc1_conformance` reads, measured
    /// at THIS class's ceiling.
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        ladder::project(bytes, MAX_RUNG)
    }

    /// 🔁️ Decode and re-encode through the independent reader and this standard's own writer.
    pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
        Ok(part21::write(&part21::read(input)?))
    }

    pub fn dispatch(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let kind = spec.str("kind");
        if kind.is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        apply_mutation(input, &kind, &params_of(spec))
    }

    pub fn dispatch_inverse(base: &[u8], spec: &Json) -> Result<Json, String> {
        let kind = spec.str("kind");
        if kind.is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        inverse_spec(base, &kind, &params_of(spec))
    }
}

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    oracles::dispatch(input, spec)
}

/// ↩️ The independently computed inverse spec of `spec` against the untouched `base`.
#[cfg(feature = "oracles")]
pub fn oracle_inverse_spec(base: &[u8], spec: &Json) -> Result<Json, String> {
    oracles::dispatch_inverse(base, spec)
}

/// 🔁️ Decode and re-encode without passing bytes through.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    oracles::round_trip(input)
}

/// 👁️ This subset's conformance-class projection.
#[cfg(feature = "oracles")]
pub fn project_step_ap214_cc1(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is not enabled, so no reference implementation is linked".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_inverse_spec(_base: &[u8], _spec: &Json) -> Result<Json, String> {
    Err("the `oracles` feature is not enabled, so no reference implementation is linked".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is not enabled, so no reference implementation is linked".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_step_ap214_cc1(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is not enabled, so no reference implementation is linked".to_string())
}
//#endregion 🔖️Dispatch

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
