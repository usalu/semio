//! 🔮️ Mutation oracle for IFC2X3 / ✳️sav (Structural Analysis View).
//!
//! 🎯️ This subset's vocabulary is NOT the `✳️any` subset's. `✳️any` speaks generic ISO 10303-21
//! graph editing and knows nothing about model view definitions. `Ifc2x3SavMutation` speaks the
//! structural analysis view, and every kind it declares is one rule of the conformance gate this
//! repository already implements in production (`../🧬️schema/🦀️component.rs`'s
//! `check_sav_conformance`) —
//!
//! | kind | production rule it addresses |
//! |---|---|
//! | `set-snapshot` | `CODE_FILE_SCHEMA` — the document must declare `IFC2X3` |
//! | `set-view-definition` | `CODE_VIEW_DEFINITION` — `FILE_DESCRIPTION` must name `StructuralAnalysisView` |
//! | `set-analysis-model` | `CODE_NO_ANALYSIS_MODEL` — at least one `IfcStructuralAnalysisModel` (hard) |
//! | `set-group-assignment` | `CODE_NO_GROUP_ASSIGNMENT` — members must relate to the model through `IfcRelAssignsToGroup` |
//! | `set-load-group` | `CODE_NO_LOADS` — loads live in an `IfcStructuralLoadGroup` |
//!
//! Each of those kinds carries an OPTIONAL payload: a value sets the concept, `null` clears it. One
//! kind per rule, total in both directions, so every scenario's inverse is a real inverse rather
//! than a whole-document restore.
//!
//! ## 🕳️ Honest limit: no real structural-analysis IFC2X3 file exists in this repository
//! The only real IFC2X3 file here is an architectural coordination export, and grepping the FULL
//! 21 MB source (not only the committed 3464-entity slice) for `IFCSTRUCTURAL*` returns **zero**
//! matches. A Structural Analysis View case therefore cannot be run against a native document of
//! its own MVD. The committed input this subset uses,
//! `shared://🏗️wellness-center-sama-structural-seed.ifc`, is the real 3464-entity export with its
//! header re-stamped to `ViewDefinition [StructuralAnalysisView]` and exactly THREE seeded
//! structural entities appended (`#9200001` analysis model, `#9200002` load group, `#9200003` group
//! assignment relating two REAL walls to the model). 3464 of its 3467 entities are real and
//! untouched; the structural half is seeded, and this case should be read as exercising the
//! vocabulary against a real building model rather than against a real structural analysis. A
//! genuine IFC2X3 `StructuralAnalysisView` export would fix that.
//!
//! ## §6: `ruststep` is the independent READER, never a second producer
//! `ruststep` 0.4 parses real ISO 10303-21 clear text but has no writer, so nothing here claims a
//! differential against a third-party PRODUCER; every scenario in
//! `../../../../🧪️tests/mutate-ifc-2x3-sav/🥒️.feature` is typed `@mode-property`/
//! `@mode-round-trip`. `ruststep` IS what reads every result back, through `project_ifc_2x3_sav`.
//!
//! @see 🔣️component.json — the oracle registration and the `ifc-2x3-sav` mutation catalog.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the vocabulary itself (`Ifc2x3SavMutation::KINDS`).
//! @see ../../../🧪️oracle/🦀️component.rs — the Part-21 reader/writer the three MVD subsets share.

use semio_repo_test_host::Json;

//#region 🔖️Oracles
#[cfg(feature = "oracles")]
mod oracles {
    use crate::artifacts::ifc::standards::v2x3::reference::part21;
    use ruststep::ast::{Exchange, Name, Parameter};
    use semio_repo_test_host::Json;

    //#region 🔖️SavVocabulary
    /// 🏗️ The analysis model itself — `check_sav_conformance`'s one HARD entity requirement.
    const ANALYSIS_MODEL: &str = "IFCSTRUCTURALANALYSISMODEL";
    /// ⚖️ The load container — `CODE_NO_LOADS`.
    const LOAD_GROUP: &str = "IFCSTRUCTURALLOADGROUP";
    /// 🔗️ The membership relationship — `CODE_NO_GROUP_ASSIGNMENT`.
    const GROUP_ASSIGNMENT: &str = "IFCRELASSIGNSTOGROUP";
    /// 👪️ Entity types an `IfcRelAssignsToGroup` may name as its `RelatingGroup` in this view.
    const GROUP_TYPES: &[&str] = &[ANALYSIS_MODEL, LOAD_GROUP, "IFCGROUP", "IFCSYSTEM"];

    /// 📐️ `IfcRoot.Name` is attribute 3 of every rooted entity (index 2).
    const NAME_INDEX: usize = 2;
    /// 📐️ `IfcRelAssignsToGroup.RelatedObjects` is attribute 5 (index 4).
    const RELATED_OBJECTS_INDEX: usize = 4;
    /// 📐️ `IfcRelAssignsToGroup.RelatingGroup` is attribute 7 (index 6).
    const RELATING_GROUP_INDEX: usize = 6;
    //#endregion 🔖️SavVocabulary

    //#region 🔖️Apply
    fn owner_history(payload: &Json) -> Result<Parameter, String> {
        Ok(part21::opt_u64_field(payload, "ownerHistory")?.map(|id| Parameter::Ref(Name::Entity(id))).unwrap_or(Parameter::NotProvided))
    }

    fn enumeration(payload: &Json, key: &str, fallback: &str) -> Result<Parameter, String> {
        Ok(Parameter::Enumeration(part21::opt_str_field(payload, key)?.unwrap_or_else(|| fallback.to_string())))
    }

    fn set_analysis_model(exchange: &mut Exchange, params: &Json) -> Result<(), String> {
        let id = part21::u64_field(params, "id")?;
        match part21::opt_obj_field(params, "model") {
            None => part21::remove(exchange, id, &[ANALYSIS_MODEL]),
            Some(model) => {
                let args = vec![
                    Parameter::String(part21::str_field(model, "globalId")?),
                    owner_history(model)?,
                    Parameter::String(part21::str_field(model, "name")?),
                    Parameter::NotProvided,
                    Parameter::NotProvided,
                    enumeration(model, "predefinedType", "NOTDEFINED")?,
                    Parameter::NotProvided,
                    Parameter::NotProvided,
                    Parameter::NotProvided,
                ];
                part21::upsert_simple(exchange, id, ANALYSIS_MODEL, args)
            }
        }
    }

    fn set_load_group(exchange: &mut Exchange, params: &Json) -> Result<(), String> {
        let id = part21::u64_field(params, "id")?;
        match part21::opt_obj_field(params, "group") {
            None => part21::remove(exchange, id, &[LOAD_GROUP]),
            Some(group) => {
                let args = vec![
                    Parameter::String(part21::str_field(group, "globalId")?),
                    owner_history(group)?,
                    Parameter::String(part21::str_field(group, "name")?),
                    Parameter::NotProvided,
                    Parameter::NotProvided,
                    enumeration(group, "predefinedType", "LOAD_GROUP")?,
                    enumeration(group, "actionType", "VARIABLE_Q")?,
                    enumeration(group, "actionSource", "LIVE_LOAD_Q")?,
                    Parameter::NotProvided,
                    Parameter::NotProvided,
                ];
                part21::upsert_simple(exchange, id, LOAD_GROUP, args)
            }
        }
    }

    fn set_group_assignment(exchange: &mut Exchange, params: &Json) -> Result<(), String> {
        let id = part21::u64_field(params, "id")?;
        match part21::opt_obj_field(params, "assignment") {
            None => part21::remove(exchange, id, &[GROUP_ASSIGNMENT]),
            Some(assignment) => {
                let relating = part21::u64_field(assignment, "relatingGroup")?;
                let resolved = part21::find(exchange, relating).map(part21::type_name).unwrap_or("");
                if !GROUP_TYPES.contains(&resolved) {
                    return Err(format!("#{relating} is {resolved:?} -- a Structural Analysis View assignment relates members to one of {GROUP_TYPES:?}"));
                }
                let related = part21::u64_array(assignment, "relatedObjects");
                if related.is_empty() {
                    return Err("an IFCRELASSIGNSTOGROUP with no RelatedObjects assigns nothing".to_string());
                }
                for object in &related {
                    if part21::find(exchange, *object).is_none() {
                        return Err(format!("no instance #{object} to assign to the group"));
                    }
                }
                let args = vec![
                    Parameter::String(part21::str_field(assignment, "globalId")?),
                    owner_history(assignment)?,
                    Parameter::NotProvided,
                    Parameter::NotProvided,
                    Parameter::List(related.into_iter().map(|object| Parameter::Ref(Name::Entity(object))).collect()),
                    Parameter::NotProvided,
                    Parameter::Ref(Name::Entity(relating)),
                ];
                part21::upsert_simple(exchange, id, GROUP_ASSIGNMENT, args)
            }
        }
    }

    /// 🦠️ One arm per declared kind, matched by its kebab-case spelling. An unrecognised kind is an
    /// error, never a silent no-op.
    fn apply(exchange: &mut Exchange, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "no-mutation" => Ok(()),
            "set-snapshot" => part21::set_file_schema(exchange, &part21::str_array(params, "fileSchema")),
            "set-view-definition" => part21::set_view_definition(exchange, &part21::str_field(params, "view")?),
            "set-analysis-model" => set_analysis_model(exchange, params),
            "set-load-group" => set_load_group(exchange, params),
            "set-group-assignment" => set_group_assignment(exchange, params),
            other => Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
    }

    pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let mut exchange = part21::read(input)?;
        apply(&mut exchange, kind, params)?;
        Ok(part21::write(&exchange))
    }
    //#endregion 🔖️Apply

    //#region 🔖️Projection
    fn arg_json(exchange: &Exchange, id: u64, index: usize) -> Json {
        part21::arg(exchange, id, index).map(part21::value_to_json).unwrap_or(Json::Null)
    }

    fn concept(exchange: &Exchange, types: &[&str], columns: &[(&str, usize)]) -> Json {
        Json::Array(
            part21::ids_of_types(exchange, types)
                .into_iter()
                .map(|id| {
                    let mut row = vec![("id".to_string(), Json::Number(id as f64))];
                    for (column, index) in columns {
                        row.push((column.to_string(), arg_json(exchange, id, *index)));
                    }
                    Json::Object(row)
                })
                .collect(),
        )
    }

    /// 👁️ The independently read projection: the shared Part-21 graph plus the three structural
    /// concepts this subset's own vocabulary edits.
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        let exchange = part21::read(bytes)?;
        let mut fields = part21::project_graph(&exchange);
        fields.push(("analysisModels".to_string(), concept(&exchange, &[ANALYSIS_MODEL], &[("name", NAME_INDEX)])));
        fields.push(("loadGroups".to_string(), concept(&exchange, &[LOAD_GROUP], &[("name", NAME_INDEX)])));
        fields.push((
            "groupAssignments".to_string(),
            concept(&exchange, &[GROUP_ASSIGNMENT], &[("relatedObjects", RELATED_OBJECTS_INDEX), ("relatingGroup", RELATING_GROUP_INDEX)]),
        ));
        Ok(Json::Object(fields))
    }
    //#endregion 🔖️Projection
}
//#endregion 🔖️Oracles

//#region 🔖️Dispatch
/// 🦠️ Applies one declared `ifc-2x3-sav` kind to a real artifact and returns the re-serialized
/// bytes. An unrecognised kind is an error, never a silent no-op.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    let empty = Json::Object(Vec::new());
    oracles::apply_mutation(input, &kind, spec.get("params").unwrap_or(&empty))
}

/// 👁️ This subset's own semantic projection, read back through the independent `ruststep` parser.
#[cfg(feature = "oracles")]
pub fn project_ifc_2x3_sav(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_ifc_2x3_sav(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🧪️Tests
/// 🧪️ Exercises every declared kind against the committed structural seed described in this
/// module's own doc comment — 3464 real entities plus three seeded structural ones.
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::{oracle_apply_mutation, project_ifc_2x3_sav};
    use semio_repo_test_host::Json;

    const FIXTURE: &[u8] = include_bytes!("../../../../../🧫️fixtures/🏗️wellness-center-sama-structural-seed.ifc");

    fn obj(entries: Vec<(&str, Json)>) -> Json {
        Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }
    fn num(value: f64) -> Json {
        Json::Number(value)
    }
    fn text(value: &str) -> Json {
        Json::String(value.to_string())
    }
    fn tagged(tag: &str, value: Json) -> Json {
        obj(vec![("t", text(tag)), ("v", value)])
    }
    fn spec(kind: &str, params: Json) -> Json {
        obj(vec![("kind", text(kind)), ("params", params)])
    }
    fn field<'j>(projection: &'j Json, key: &str) -> &'j Json {
        projection.get(key).unwrap_or_else(|| panic!("projection carries no {key}"))
    }
    fn seed_model() -> Json {
        obj(vec![("globalId", text("2SavAnalysisModelSeed001")), ("ownerHistory", num(41.0)), ("name", text("Street level analysis model"))])
    }
    fn seed_load_group() -> Json {
        obj(vec![("globalId", text("2SavLoadGroupSeed00000001")), ("ownerHistory", num(41.0)), ("name", text("Self weight"))])
    }
    fn seed_assignment() -> Json {
        obj(vec![
            ("globalId", text("2SavGroupAssignmentSeed01")),
            ("ownerHistory", num(41.0)),
            ("relatedObjects", Json::Array(vec![num(270549.0), num(523123.0)])),
            ("relatingGroup", num(9_200_001.0)),
        ])
    }

    #[test]
    fn the_committed_seed_is_a_structural_analysis_view_document_over_a_real_building_model() {
        let projection = project_ifc_2x3_sav(FIXTURE).expect("project the seed");
        assert_eq!(field(&projection, "viewDefinition"), &text("ViewDefinition [StructuralAnalysisView]"));
        assert_eq!(field(&projection, "fileSchema"), &Json::Array(vec![text("IFC2X3")]));
        assert_eq!(field(&projection, "entityCount"), &num(3467.0), "3464 real entities plus the three seeded structural ones");
        assert!(matches!(field(&projection, "analysisModels"), Json::Array(items) if items.len() == 1));
        assert!(matches!(field(&projection, "loadGroups"), Json::Array(items) if items.len() == 1));
        assert!(matches!(field(&projection, "groupAssignments"), Json::Array(items) if items.len() == 1));
    }

    #[test]
    fn no_mutation_round_trips_through_our_own_writer_without_passing_bytes_through() {
        let output = oracle_apply_mutation(FIXTURE, &spec("no-mutation", obj(vec![]))).expect("no-mutation");
        assert_ne!(output, FIXTURE, "our own writer must not reproduce the source writer's exact bytes");
        assert_eq!(project_ifc_2x3_sav(&output).unwrap(), project_ifc_2x3_sav(FIXTURE).unwrap());
    }

    #[test]
    fn set_snapshot_rewrites_the_declared_schema_and_inverts() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("set-snapshot", obj(vec![("fileSchema", Json::Array(vec![text("IFC2X3"), text("IFC2X3-SAV-MARKER")]))]))).expect("set-snapshot");
        assert_eq!(field(&project_ifc_2x3_sav(&mutated).unwrap(), "fileSchema"), &Json::Array(vec![text("IFC2X3"), text("IFC2X3-SAV-MARKER")]));
        let restored = oracle_apply_mutation(&mutated, &spec("set-snapshot", obj(vec![("fileSchema", Json::Array(vec![text("IFC2X3")]))]))).expect("inverse");
        assert_eq!(project_ifc_2x3_sav(&restored).unwrap(), project_ifc_2x3_sav(FIXTURE).unwrap());
    }

    #[test]
    fn set_view_definition_de_stamps_the_mvd_and_inverts() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("set-view-definition", obj(vec![("view", text("CoordinationView_V2.0"))]))).expect("set-view-definition");
        assert_eq!(field(&project_ifc_2x3_sav(&mutated).unwrap(), "viewDefinition"), &text("ViewDefinition [CoordinationView_V2.0]"));
        let restored = oracle_apply_mutation(&mutated, &spec("set-view-definition", obj(vec![("view", text("StructuralAnalysisView"))]))).expect("inverse");
        assert_eq!(project_ifc_2x3_sav(&restored).unwrap(), project_ifc_2x3_sav(FIXTURE).unwrap());
    }

    #[test]
    fn set_analysis_model_breaks_the_hard_rule_and_inverts() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("set-analysis-model", obj(vec![("id", num(9_200_001.0)), ("model", Json::Null)]))).expect("set-analysis-model");
        let projection = project_ifc_2x3_sav(&mutated).unwrap();
        assert_eq!(field(&projection, "analysisModels"), &Json::Array(vec![]), "removing the only analysis model is the hard SAV violation");
        assert_eq!(field(&projection, "entityCount"), &num(3466.0));
        let restored = oracle_apply_mutation(&mutated, &spec("set-analysis-model", obj(vec![("id", num(9_200_001.0)), ("model", seed_model())]))).expect("inverse");
        assert_eq!(project_ifc_2x3_sav(&restored).unwrap(), project_ifc_2x3_sav(FIXTURE).unwrap());
    }

    #[test]
    fn set_load_group_empties_the_loads_and_inverts() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("set-load-group", obj(vec![("id", num(9_200_002.0)), ("group", Json::Null)]))).expect("set-load-group");
        assert_eq!(field(&project_ifc_2x3_sav(&mutated).unwrap(), "loadGroups"), &Json::Array(vec![]));
        let restored = oracle_apply_mutation(&mutated, &spec("set-load-group", obj(vec![("id", num(9_200_002.0)), ("group", seed_load_group())]))).expect("inverse");
        assert_eq!(project_ifc_2x3_sav(&restored).unwrap(), project_ifc_2x3_sav(FIXTURE).unwrap());
    }

    #[test]
    fn set_group_assignment_detaches_the_real_members_and_inverts() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("set-group-assignment", obj(vec![("id", num(9_200_003.0)), ("assignment", Json::Null)]))).expect("set-group-assignment");
        assert_eq!(field(&project_ifc_2x3_sav(&mutated).unwrap(), "groupAssignments"), &Json::Array(vec![]));
        let restored = oracle_apply_mutation(&mutated, &spec("set-group-assignment", obj(vec![("id", num(9_200_003.0)), ("assignment", seed_assignment())]))).expect("inverse");
        assert_eq!(project_ifc_2x3_sav(&restored).unwrap(), project_ifc_2x3_sav(FIXTURE).unwrap());
    }

    #[test]
    fn the_seeded_assignment_names_two_real_walls() {
        let projection = project_ifc_2x3_sav(FIXTURE).unwrap();
        let Json::Array(rows) = field(&projection, "groupAssignments") else { panic!("expected an array") };
        assert_eq!(rows[0].get("relatingGroup"), Some(&tagged("reference", num(9_200_001.0))));
        assert_eq!(
            rows[0].get("relatedObjects"),
            Some(&tagged("aggregate", Json::Array(vec![tagged("reference", num(270549.0)), tagged("reference", num(523123.0))]))),
            "the seeded assignment relates the two REAL IFCWALLSTANDARDCASE instances of the real model"
        );
    }

    #[test]
    fn the_sav_guards_are_real_errors_not_silent_no_ops() {
        assert!(oracle_apply_mutation(FIXTURE, &spec("not-a-real-kind", obj(vec![]))).is_err(), "an unknown kind must be an error");
        assert!(oracle_apply_mutation(FIXTURE, &spec("set-analysis-model", obj(vec![("id", num(270549.0)), ("model", Json::Null)]))).is_err(), "clearing an analysis model must not delete a real wall");
        assert!(oracle_apply_mutation(FIXTURE, &spec("set-load-group", obj(vec![("id", num(9_200_001.0)), ("group", Json::Null)]))).is_err(), "the analysis model is not a load group");
        assert!(
            oracle_apply_mutation(
                FIXTURE,
                &spec("set-group-assignment", obj(vec![("id", num(9_200_004.0)), ("assignment", obj(vec![("globalId", text("x")), ("relatedObjects", Json::Array(vec![num(270549.0)])), ("relatingGroup", num(270549.0))]))]))
            )
            .is_err(),
            "a wall is not a structural group"
        );
    }
}
//#endregion 🧪️Tests
