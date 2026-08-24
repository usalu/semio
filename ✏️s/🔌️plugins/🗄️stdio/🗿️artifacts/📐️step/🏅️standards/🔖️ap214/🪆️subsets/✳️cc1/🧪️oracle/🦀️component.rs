//! 🔮️ Mutation oracle for `ap214/✳️cc1` — ISO 10303-214 CC1 (config data only).
//!
//! Reference: `ruststep` 0.4, the registered independent READER (it has no writer at all), plus this
//! standard's own from-scratch Part-21 writer and an independent re-derivation of the §4.3
//! conformance ladder — both in `../../../🧪️oracle/🦀️component.rs`, shared by all seven `ap214`
//! subsets so no classification or serialization step is copied per class.
//!
//! 🎯️ This dispatcher performs the vocabulary of a CONFORMANCE CLASS, not of the Part-21 grammar.
//! The sibling `../../✳️any` subset declares eleven grammar verbs (insert an entity, set an argument…) that
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
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the vocabulary this module is measured against.
//! @see 🔣️component.json — the `step-ap214-cc1` catalog and the `ruststep` registration it carries.

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
mod tests {
    use super::*;

    fn fixture() -> Vec<u8> {
        include_bytes!("../../../../../🧫️fixtures/📐️hexagonal-cut-concrete-forest-left-ap214.stp").to_vec()
    }

    fn object(entries: Vec<(&str, Json)>) -> Json {
        Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    fn spec(kind: &str, params: Json) -> Json {
        object(vec![("kind", Json::String(kind.to_string())), ("params", params)])
    }

    fn project(bytes: &[u8]) -> Result<Json, String> {
        project_step_ap214_cc1(bytes)
    }

    /// 🧫️ What this class actually sees in the real committed export, read off the file rather than
    /// assumed — this is the number every scenario's observability depends on.
    #[test]
    fn the_real_export_reads_the_way_this_class_predicts() {
        let before = project(&fixture()).unwrap();
        assert_eq!(before.get("aboveCeiling"), Some(&Json::Number(2.0)), "CC1 admits neither of the real file's two representations");
        assert_eq!(before.get("conformsToClass"), Some(&Json::Bool(false)));
        assert_eq!(before.get("hasProductChain"), Some(&Json::Bool(true)), "the formation rung is the ISO 10303-41 subtype a real exporter writes");
        assert_eq!(before.get("fileSchema").unwrap().clone(), Json::Array(vec![Json::String("AUTOMOTIVE_DESIGN".to_string())]));
    }

    /// ⚖️ Every declared kind must MOVE this class's projection, and its own inverse must put it
    /// back exactly. A kind that leaves the projection where it was is a scenario that proves
    /// nothing, so the observability half is asserted here as well as in the case adapter.
    #[test]
    fn every_kind_is_observable_and_its_own_inverse_restores_the_projection() {
        let input = fixture();
        let original = project(&input).unwrap();
        for case in exercised_specs() {
            let kind = case.str("kind");
            let mutated = oracle_apply_mutation(&input, &case).unwrap_or_else(|error| panic!("{kind} failed: {error}"));
            let after = project(&mutated).unwrap();
            if kind != "no-mutation" {
                assert_ne!(after, original, "{kind} left the conformance projection unchanged -- a mutation that is not observable proves nothing");
            }
            let inverse = oracle_inverse_spec(&input, &case).unwrap();
            let restored = oracle_apply_mutation(&mutated, &inverse).unwrap_or_else(|error| panic!("{kind} inverse failed: {error}"));
            assert_eq!(project(&restored).unwrap(), original, "applying {kind} and then its own inverse must restore the original projection");
        }
    }

    /// 📇️ One spec per declared kind, with parameters chosen against the REAL fixture's own content:
    /// `#13` is a real representation, `#827`/`#822`/`#821` are the real product chain.
    fn exercised_specs() -> Vec<Json> {
        vec![
            spec("no-mutation", Json::Object(Vec::new())),
            spec(
                "set-snapshot",
                object(vec![
                    ("fileSchema", Json::Array(vec![Json::String("AUTOMOTIVE_DESIGN".to_string())])),
                    (
                        "productIdentity",
                        object(vec![
                            ("product", Json::Number(1.0)),
                            ("productName", Json::String("Document".to_string())),
                            ("formation", Json::Number(2.0)),
                            ("formationId", Json::String("A".to_string())),
                            ("definition", Json::Number(3.0)),
                            ("definitionId", Json::String("A".to_string())),
                        ]),
                    ),
                ]),
            ),
            spec("set-file-schema", object(vec![("schemas", Json::Array(vec![Json::String("CONFIG_CONTROL_DESIGN".to_string())]))])),
            spec("set-product-identity", object(vec![("identity", Json::Null)])),
            spec("remove-shape-representation", object(vec![("id", Json::Number(13.0))])),
        ]
    }

    #[test]
    fn the_round_trip_reparses_rather_than_copies() {
        let input = fixture();
        let output = oracle_round_trip(&input).unwrap();
        assert_ne!(output, input, "ISO 10303-21 clear text is regenerated from the parsed model, so identical bytes would mean the input was copied");
        assert_eq!(project(&output).unwrap(), project(&input).unwrap());
    }

    #[test]
    fn a_ladder_edit_refuses_an_entity_that_is_not_on_the_ladder() {
        let refusal = oracle_apply_mutation(&fixture(), &spec("remove-shape-representation", object(vec![("id", Json::Number(827.0))]))).unwrap_err();
        assert!(refusal.contains("never an arbitrary entity"), "a conformance repair must never delete a product record: {refusal}");
    }

    #[test]
    fn an_unknown_kind_is_an_error_not_a_silent_no_op() {
        assert!(oracle_apply_mutation(&fixture(), &spec("insert-entity", Json::Object(Vec::new()))).is_err(), "the Part-21 grammar verbs belong to the ✳️any subset, not to a conformance class");
    }

    /// 🏷️ `KINDS` must equal the committed catalog AND the committed vocabulary. The framework never
    /// parses Rust, so this reads both files as text and fails the moment they drift apart.
    #[test]
    fn kinds_match_the_catalog_and_the_vocabulary() {
        let manifest = include_str!("🔣️component.json");
        let vocabulary = include_str!("../🧬️schema/🧬️mutations/🦀️component.rs");
        let feature = include_str!("../../../../../🧪️tests/mutate-step-ap214-cc1/component.feature");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "the catalog is missing kind {kind:?}");
            assert!(vocabulary.contains(&format!("\"{kind}\"")), "StepCc1Mutation::KINDS is missing {kind:?}");
            assert!(feature.contains(&format!("| {kind} ")), "the case's Examples table is missing kind {kind:?}");
        }
        assert_eq!(KINDS.len(), 5);
    }

    /// 🎯️ The repair CC1 asks for: delete every representation. Two removals bring the real export
    /// into the class, and nothing else in the file is on the ladder.
    #[test]
    fn deleting_both_representations_is_what_makes_the_real_export_conform() {
        let mut bytes = fixture();
        for id in [13u64, 836] {
            bytes = oracle_apply_mutation(&bytes, &spec("remove-shape-representation", object(vec![("id", Json::Number(id as f64))]))).unwrap();
        }
        let projection = project(&bytes).unwrap();
        assert_eq!(projection.get("aboveCeiling"), Some(&Json::Number(0.0)));
        assert_eq!(projection.get("conformsToClass"), Some(&Json::Bool(true)));
        assert_eq!(projection.get("hasProductChain"), Some(&Json::Bool(true)), "the repair must not touch the product chain");
    }
}
//#endregion 🧪️Tests
