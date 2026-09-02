//! 🔮️ Mutation oracle for IFC2X3 / ✳️cv20 (Coordination View 2.0).
//!
//! 🎯️ This subset's vocabulary is NOT the `✳️any` subset's. `✳️any` speaks generic ISO 10303-21
//! graph editing (`upsert-instance`/`remove-instance`/`set-header`) and knows nothing about model
//! view definitions. `Ifc2x3Cv20Mutation` speaks Coordination View 2.0: every kind it declares is
//! one rule of the MVD conformance gate this repository already implements in production
//! (`../🧬️schema/🦀️component.rs`'s `check_cv20_conformance`) —
//!
//! | kind | production rule it addresses |
//! |---|---|
//! | `set-snapshot` | `CODE_FILE_SCHEMA` — the document must declare `IFC2X3` |
//! | `set-view-definition` | `CODE_VIEW_DEFINITION` — `FILE_DESCRIPTION` must name `CoordinationView` |
//! | `set-structural-entity` | `CODE_STRUCTURAL_ENTITY` — CV2.0's architectural scope forbids structural-analysis entities |
//! | `set-project-units` | `CODE_PROJECT_UNITS` — `IfcProject.UnitsInContext` must resolve |
//! | `set-product-placement` | `CODE_PRODUCT_PLACEMENT` — a geometry-bearing product must place through `IfcLocalPlacement` |
//!
//! Each of those kinds carries an OPTIONAL payload: a value sets the concept, `null` clears it. One
//! kind per MVD concept, total in both directions, so every scenario's inverse is a real inverse
//! rather than a whole-document restore.
//!
//! ## §6: `ruststep` is the independent READER, never a second producer
//! `ruststep` 0.4 parses real ISO 10303-21 clear text (IFC2X3 is that syntax under the IFC2X3
//! EXPRESS schema) but has no writer at all, so this module cannot claim a differential against a
//! third-party PRODUCER. Every scenario in `../../../../🧪️tests/mutate-ifc-2x3-cv20/🥒️.feature`
//! is typed `@mode-property`/`@mode-round-trip` accordingly. `ruststep` IS what reads every result
//! back before `semantic-ifc-v1` compares it, through `project_ifc_2x3_cv20` below.
//!
//! @see 🔣️.json — the oracle registration and the `ifc-2x3-cv20` mutation catalog.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the vocabulary itself (`Ifc2x3Cv20Mutation::KINDS`).
//! @see ../../../🦀️oracle.rs — the Part-21 reader/writer the three MVD subsets share.

use semio_repo_test_host::Json;

//#region 🔖️Oracles
#[cfg(feature = "oracles")]
mod oracles {
    use crate::artifacts::ifc::standards::v2x3::reference::part21;
    use ruststep::ast::{Exchange, Name, Parameter};
    use semio_repo_test_host::Json;

    //#region 🔖️MvdVocabulary
    /// 🚫️ Entity types Coordination View 2.0 excludes — mirrors the production
    /// `FORBIDDEN_STRUCTURAL_TYPES` list `check_cv20_conformance` hard-faults on.
    const FORBIDDEN_STRUCTURAL_TYPES: &[&str] = &["IFCSTRUCTURALANALYSISMODEL", "IFCSTRUCTURALCURVEMEMBER", "IFCSTRUCTURALLOADGROUP"];

    /// 🏗️ Geometry-bearing `IfcProduct` subtypes the placement rule applies to — mirrors the
    /// production `GEOMETRY_BEARING_PRODUCT_TYPES` list.
    const GEOMETRY_BEARING_PRODUCT_TYPES: &[&str] = &["IFCWALL", "IFCWALLSTANDARDCASE", "IFCDOOR", "IFCWINDOW", "IFCSLAB", "IFCBEAM", "IFCCOLUMN", "IFCROOF", "IFCSTAIR", "IFCBUILDINGELEMENTPROXY"];

    /// 📐️ `IfcProject.UnitsInContext` is attribute 9 of `IfcProject` (index 8).
    const PROJECT_UNITS_INDEX: usize = 8;
    /// 📐️ `IfcProduct.ObjectPlacement` is attribute 6 of every `IfcProduct` (index 5).
    const PRODUCT_PLACEMENT_INDEX: usize = 5;
    //#endregion 🔖️MvdVocabulary

    //#region 🔖️Apply
    fn set_structural_entity(exchange: &mut Exchange, params: &Json) -> Result<(), String> {
        let id = part21::u64_field(params, "id")?;
        match part21::opt_obj_field(params, "entity") {
            None => part21::remove(exchange, id, FORBIDDEN_STRUCTURAL_TYPES),
            Some(entity) => {
                let type_name = part21::str_field(entity, "typeName")?;
                if !FORBIDDEN_STRUCTURAL_TYPES.contains(&type_name.as_str()) {
                    return Err(format!("{type_name} is not one of the structural types Coordination View 2.0 excludes ({FORBIDDEN_STRUCTURAL_TYPES:?})"));
                }
                let args = vec![Parameter::String(part21::str_field(entity, "globalId")?), Parameter::NotProvided, Parameter::String(part21::str_field(entity, "name")?)];
                part21::upsert_simple(exchange, id, &type_name, args)
            }
        }
    }

    fn set_project_units(exchange: &mut Exchange, params: &Json) -> Result<(), String> {
        let project = part21::u64_field(params, "project")?;
        let units = part21::opt_u64_field(params, "units")?;
        let value = match units {
            None => Parameter::NotProvided,
            Some(id) => {
                if part21::find(exchange, id).is_none() {
                    return Err(format!("no instance #{id} to serve as the project's IfcUnitAssignment"));
                }
                Parameter::Ref(Name::Entity(id))
            }
        };
        part21::set_arg(exchange, project, &["IFCPROJECT"], PROJECT_UNITS_INDEX, value)
    }

    fn set_product_placement(exchange: &mut Exchange, params: &Json) -> Result<(), String> {
        let product = part21::u64_field(params, "product")?;
        let placement = part21::opt_u64_field(params, "placement")?;
        let value = match placement {
            None => Parameter::NotProvided,
            Some(id) => {
                let resolved = part21::find(exchange, id).map(part21::type_name).unwrap_or("");
                if resolved != "IFCLOCALPLACEMENT" {
                    return Err(format!("#{id} is {resolved:?}, not an IFCLOCALPLACEMENT -- Coordination View 2.0 places products through IfcLocalPlacement"));
                }
                Parameter::Ref(Name::Entity(id))
            }
        };
        part21::set_arg(exchange, product, GEOMETRY_BEARING_PRODUCT_TYPES, PRODUCT_PLACEMENT_INDEX, value)
    }

    /// 🦠️ One arm per declared kind, matched by its kebab-case spelling. An unrecognised kind is an
    /// error, never a silent no-op: a quietly skipped mutation reports as a passing test.
    fn apply(exchange: &mut Exchange, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "no-mutation" => Ok(()),
            "set-snapshot" => part21::set_file_schema(exchange, &part21::str_array(params, "fileSchema")),
            "set-view-definition" => part21::set_view_definition(exchange, &part21::str_field(params, "view")?),
            "set-structural-entity" => set_structural_entity(exchange, params),
            "set-project-units" => set_project_units(exchange, params),
            "set-product-placement" => set_product_placement(exchange, params),
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
    fn reference_or_null(exchange: &Exchange, id: u64, index: usize) -> Json {
        match part21::arg(exchange, id, index) {
            Some(Parameter::Ref(Name::Entity(target))) => Json::Number(*target as f64),
            _ => Json::Null,
        }
    }

    fn concept_map(exchange: &Exchange, types: &[&str], index: usize) -> Json {
        Json::Array(
            part21::ids_of_types(exchange, types)
                .into_iter()
                .map(|id| Json::Object(vec![("id".to_string(), Json::Number(id as f64)), ("resolves".to_string(), reference_or_null(exchange, id, index))]))
                .collect(),
        )
    }

    /// 👁️ The independently read projection: the shared Part-21 graph plus the three CV2.0 concepts
    /// this subset's own vocabulary edits, so a comparison states the MVD's own surface and not
    /// only the raw entity list.
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        let exchange = part21::read(bytes)?;
        let mut fields = part21::project_graph(&exchange);
        fields.push(("structuralEntities".to_string(), part21::concept_ids(&exchange, FORBIDDEN_STRUCTURAL_TYPES)));
        fields.push(("projectUnits".to_string(), concept_map(&exchange, &["IFCPROJECT"], PROJECT_UNITS_INDEX)));
        fields.push(("productPlacements".to_string(), concept_map(&exchange, GEOMETRY_BEARING_PRODUCT_TYPES, PRODUCT_PLACEMENT_INDEX)));
        Ok(Json::Object(fields))
    }
    //#endregion 🔖️Projection
}
//#endregion 🔖️Oracles

//#region 🔖️Dispatch
/// 🦠️ Applies one declared `ifc-2x3-cv20` kind to a real artifact and returns the re-serialized
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
pub fn project_ifc_2x3_cv20(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_ifc_2x3_cv20(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🧪️Tests
/// 🧪️ Exercises every declared kind against the real committed IFC2X3 fixture — a genuine
/// `ViewDefinition [CoordinationView_V2.0]` EDM export, which is what makes this the one subset in
/// this standard whose real input already IS a document of its own model view definition.
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::{oracle_apply_mutation, project_ifc_2x3_cv20};
    use semio_repo_test_host::Json;

    const FIXTURE: &[u8] = include_bytes!("../../../../../🧫️fixtures/🏗️wellness-center-sama-street-level.ifc");

    fn obj(entries: Vec<(&str, Json)>) -> Json {
        Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }
    fn num(value: f64) -> Json {
        Json::Number(value)
    }
    fn text(value: &str) -> Json {
        Json::String(value.to_string())
    }
    fn spec(kind: &str, params: Json) -> Json {
        obj(vec![("kind", text(kind)), ("params", params)])
    }
    fn field<'j>(projection: &'j Json, key: &str) -> &'j Json {
        projection.get(key).unwrap_or_else(|| panic!("projection carries no {key}"))
    }
    fn resolves(projection: &Json, key: &str, id: f64) -> Json {
        match field(projection, key) {
            Json::Array(items) => items.iter().find(|entry| matches!(entry.get("id"), Some(Json::Number(n)) if *n == id)).and_then(|entry| entry.get("resolves")).cloned().expect("concept row present"),
            other => panic!("expected a concept array, got {other:?}"),
        }
    }

    #[test]
    fn the_real_fixture_is_a_coordination_view_2_0_document_with_no_structural_entities() {
        let projection = project_ifc_2x3_cv20(FIXTURE).expect("project the real fixture");
        assert_eq!(field(&projection, "viewDefinition"), &text("ViewDefinition [CoordinationView_V2.0]"));
        assert_eq!(field(&projection, "fileSchema"), &Json::Array(vec![text("IFC2X3")]));
        assert_eq!(field(&projection, "entityCount"), &num(3464.0));
        assert_eq!(field(&projection, "structuralEntities"), &Json::Array(vec![]), "a real CV2.0 export carries no structural-analysis entities");
        assert_eq!(resolves(&projection, "projectUnits", 120.0), num(107.0), "the real IFCPROJECT #120 resolves UnitsInContext to the real IFCUNITASSIGNMENT #107");
        assert_eq!(resolves(&projection, "productPlacements", 270549.0), num(270529.0), "the real wall #270549 places through the real IFCLOCALPLACEMENT #270529");
    }

    #[test]
    fn no_mutation_round_trips_through_our_own_writer_without_passing_bytes_through() {
        let output = oracle_apply_mutation(FIXTURE, &spec("no-mutation", obj(vec![]))).expect("no-mutation");
        assert_ne!(output, FIXTURE, "our own writer must not reproduce the source writer's exact bytes");
        assert_eq!(project_ifc_2x3_cv20(&output).unwrap(), project_ifc_2x3_cv20(FIXTURE).unwrap());
    }

    #[test]
    fn set_snapshot_rewrites_the_declared_schema_and_inverts() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("set-snapshot", obj(vec![("fileSchema", Json::Array(vec![text("IFC2X3"), text("IFC2X3-CV20-MARKER")]))]))).expect("set-snapshot");
        assert_eq!(field(&project_ifc_2x3_cv20(&mutated).unwrap(), "fileSchema"), &Json::Array(vec![text("IFC2X3"), text("IFC2X3-CV20-MARKER")]));
        let restored = oracle_apply_mutation(&mutated, &spec("set-snapshot", obj(vec![("fileSchema", Json::Array(vec![text("IFC2X3")]))]))).expect("inverse");
        assert_eq!(project_ifc_2x3_cv20(&restored).unwrap(), project_ifc_2x3_cv20(FIXTURE).unwrap());
    }

    #[test]
    fn set_view_definition_de_stamps_the_mvd_and_inverts() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("set-view-definition", obj(vec![("view", text("StructuralAnalysisView"))]))).expect("set-view-definition");
        assert_eq!(field(&project_ifc_2x3_cv20(&mutated).unwrap(), "viewDefinition"), &text("ViewDefinition [StructuralAnalysisView]"));
        let restored = oracle_apply_mutation(&mutated, &spec("set-view-definition", obj(vec![("view", text("CoordinationView_V2.0"))]))).expect("inverse");
        assert_eq!(project_ifc_2x3_cv20(&restored).unwrap(), project_ifc_2x3_cv20(FIXTURE).unwrap());
    }

    #[test]
    fn set_structural_entity_violates_the_mvd_exclusion_and_inverts() {
        let entity = obj(vec![("typeName", text("IFCSTRUCTURALANALYSISMODEL")), ("globalId", text("2Cv20StructuralProbe0001")), ("name", text("CV20 exclusion probe"))]);
        let mutated = oracle_apply_mutation(FIXTURE, &spec("set-structural-entity", obj(vec![("id", num(9_000_001.0)), ("entity", entity)]))).expect("set-structural-entity");
        let projection = project_ifc_2x3_cv20(&mutated).unwrap();
        assert_eq!(field(&projection, "structuralEntities"), &Json::Array(vec![num(9_000_001.0)]));
        assert_eq!(field(&projection, "entityCount"), &num(3465.0));
        let restored = oracle_apply_mutation(&mutated, &spec("set-structural-entity", obj(vec![("id", num(9_000_001.0)), ("entity", Json::Null)]))).expect("inverse");
        assert_eq!(project_ifc_2x3_cv20(&restored).unwrap(), project_ifc_2x3_cv20(FIXTURE).unwrap());
    }

    #[test]
    fn set_project_units_clears_the_real_unit_assignment_and_inverts() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("set-project-units", obj(vec![("project", num(120.0)), ("units", Json::Null)]))).expect("set-project-units");
        assert_eq!(resolves(&project_ifc_2x3_cv20(&mutated).unwrap(), "projectUnits", 120.0), Json::Null);
        let restored = oracle_apply_mutation(&mutated, &spec("set-project-units", obj(vec![("project", num(120.0)), ("units", num(107.0))]))).expect("inverse");
        assert_eq!(project_ifc_2x3_cv20(&restored).unwrap(), project_ifc_2x3_cv20(FIXTURE).unwrap());
    }

    #[test]
    fn set_product_placement_clears_the_real_wall_placement_and_inverts() {
        let mutated = oracle_apply_mutation(FIXTURE, &spec("set-product-placement", obj(vec![("product", num(270549.0)), ("placement", Json::Null)]))).expect("set-product-placement");
        assert_eq!(resolves(&project_ifc_2x3_cv20(&mutated).unwrap(), "productPlacements", 270549.0), Json::Null);
        let restored = oracle_apply_mutation(&mutated, &spec("set-product-placement", obj(vec![("product", num(270549.0)), ("placement", num(270529.0))]))).expect("inverse");
        assert_eq!(project_ifc_2x3_cv20(&restored).unwrap(), project_ifc_2x3_cv20(FIXTURE).unwrap());
    }

    #[test]
    fn the_mvd_guards_are_real_errors_not_silent_no_ops() {
        assert!(oracle_apply_mutation(FIXTURE, &spec("not-a-real-kind", obj(vec![]))).is_err(), "an unknown kind must be an error");
        assert!(oracle_apply_mutation(FIXTURE, &spec("set-project-units", obj(vec![("project", num(130.0)), ("units", num(107.0))]))).is_err(), "#130 is an IFCBUILDING, not an IFCPROJECT");
        assert!(oracle_apply_mutation(FIXTURE, &spec("set-product-placement", obj(vec![("product", num(270549.0)), ("placement", num(120.0))]))).is_err(), "#120 is not an IFCLOCALPLACEMENT");
        assert!(
            oracle_apply_mutation(FIXTURE, &spec("set-structural-entity", obj(vec![("id", num(9_000_002.0)), ("entity", obj(vec![("typeName", text("IFCWALL")), ("globalId", text("x")), ("name", text("x"))]))]))).is_err(),
            "IFCWALL is not one of the types CV2.0 excludes"
        );
        assert!(oracle_apply_mutation(FIXTURE, &spec("set-structural-entity", obj(vec![("id", num(270549.0)), ("entity", Json::Null)]))).is_err(), "clearing a structural entity must not delete a real wall");
    }
}
//#endregion 🧪️Tests
