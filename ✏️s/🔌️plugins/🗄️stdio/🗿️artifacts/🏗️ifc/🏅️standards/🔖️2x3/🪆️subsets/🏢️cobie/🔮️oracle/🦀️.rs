//! 🔮️ Mutation oracle for IFC2X3 / 🏢️cobie (Basic FM Handover, the view that carries COBie 2.4).
//!
//! 🎯️ This subset's vocabulary is NOT the `✳️base` subset's. `✳️base` speaks generic ISO 10303-21
//! graph editing and knows nothing about model view definitions. `Ifc2x3CobieMutation` speaks the
//! COBie handover sheets, and every kind it declares is one rule of the conformance gate this
//! repository already implements in production (`../🧬️schema/🦀️component.rs`'s
//! `check_cobie_conformance`) —
//!
//! | kind | COBie sheet | production rule it addresses |
//! |---|---|---|
//! | `set-snapshot` | — | `CODE_FILE_SCHEMA` — the document must declare `IFC2X3` |
//! | `set-view-definition` | — | `CODE_VIEW_DEFINITION` — `FILE_DESCRIPTION` must name `FMHandOverView` |
//! | `set-facility-name` | Facility | `CODE_BUILDING_STOREY` — the handover needs a named `IfcBuilding` |
//! | `set-floor-elevation` | Floor | `CODE_BUILDING_STOREY` — the Floor sheet is an `IfcBuildingStorey` with an elevation |
//! | `set-space` | Space | `CODE_SPACE_NAME` — the Space sheet is keyed by a non-empty `IfcSpace.Name` |
//! | `set-type-assignment` | Type | `CODE_TYPE_ASSIGNMENT` — maintainable products must relate to a type through `IfcRelDefinesByType` |
//!
//! Each of those kinds carries an OPTIONAL payload: a value sets the sheet row, `null` clears it.
//! One kind per COBie concept, total in both directions, so every scenario's inverse is a real
//! inverse rather than a whole-document restore.
//!
//! ## 🕳️ What the real input does and does not carry
//! The one real IFC2X3 file in this repository is an architectural coordination export. It carries
//! real `IFCBUILDING`, `IFCBUILDINGSTOREY`, `IFC*TYPE` and `IFCRELDEFINESBYTYPE` instances — the
//! Facility, Floor and Type sheets are genuinely populated — but **zero `IFCSPACE` instances**
//! (confirmed against the full 21 MB source, not only the committed slice), so its COBie Space
//! sheet is empty. `set-space` is therefore the one kind whose forward direction inserts rather
//! than edits; that is a real property of the export, recorded here rather than papered over with a
//! synthesised space.
//!
//! ## §6: `ruststep` is the independent READER, never a second producer
//! `ruststep` 0.4 parses real ISO 10303-21 clear text but has no writer, so nothing here claims a
//! differential against a third-party PRODUCER; every scenario in
//! `../../../../🧪️tests/🏢️mutate-ifc-2x3-cobie/🥒️.feature` is typed `@mode-property`/
//! `@mode-round-trip`. `ruststep` IS what reads every result back, through `project_ifc_2x3_cobie`.
//!
//! @see 🔣️.json — the oracle registration and the `ifc-2x3-cobie` mutation catalog.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the vocabulary itself (`Ifc2x3CobieMutation::KINDS`).
//! @see ../../../🦀️oracle.rs — the Part-21 reader/writer the three MVD subsets share.

use semio_repo_test_host::Json;

//#region 🔖️Oracles
#[cfg(feature = "oracles")]
mod oracles {
    use crate::standards::v2x3::reference::part21;
    use ruststep::ast::{Exchange, Name, Parameter};
    use semio_repo_test_host::Json;

    //#region 🔖️CobieVocabulary
    /// 📐️ `IfcRoot.Name` is attribute 3 of every rooted entity (index 2) — the COBie key column.
    const NAME_INDEX: usize = 2;
    /// 📐️ `IfcBuildingStorey.Elevation` is attribute 10 (index 9) — the Floor sheet's elevation.
    const STOREY_ELEVATION_INDEX: usize = 9;
    /// 📐️ `IfcRelDefinesByType.RelatedObjects` is attribute 5 (index 4).
    const RELATED_OBJECTS_INDEX: usize = 4;
    /// 📐️ `IfcRelDefinesByType.RelatingType` is attribute 6 (index 5).
    const RELATING_TYPE_INDEX: usize = 5;
    //#endregion 🔖️CobieVocabulary

    //#region 🔖️Apply
    fn set_facility_name(exchange: &mut Exchange, params: &Json) -> Result<(), String> {
        let building = part21::u64_field(params, "building")?;
        let value = part21::opt_str_field(params, "name")?.map(Parameter::String).unwrap_or(Parameter::NotProvided);
        part21::set_arg(exchange, building, &["IFCBUILDING"], NAME_INDEX, value)
    }

    fn set_floor_elevation(exchange: &mut Exchange, params: &Json) -> Result<(), String> {
        let storey = part21::u64_field(params, "storey")?;
        let value = part21::opt_num_field(params, "elevation")?.map(Parameter::Real).unwrap_or(Parameter::NotProvided);
        part21::set_arg(exchange, storey, &["IFCBUILDINGSTOREY"], STOREY_ELEVATION_INDEX, value)
    }

    fn set_space(exchange: &mut Exchange, params: &Json) -> Result<(), String> {
        let id = part21::u64_field(params, "id")?;
        match part21::opt_obj_field(params, "space") {
            None => part21::remove(exchange, id, &["IFCSPACE"]),
            Some(space) => {
                let name = part21::str_field(space, "name")?;
                if name.trim().is_empty() {
                    return Err("COBie's Space sheet is keyed by name -- an IFCSPACE with a blank Name is not a handover row".to_string());
                }
                let placement = part21::u64_field(space, "placement")?;
                let resolved = part21::find(exchange, placement).map(part21::type_name).unwrap_or("");
                if resolved != "IFCLOCALPLACEMENT" {
                    return Err(format!("#{placement} is {resolved:?}, not an IFCLOCALPLACEMENT -- a handover space is placed in the real spatial structure"));
                }
                let args = vec![
                    Parameter::String(part21::str_field(space, "globalId")?),
                    Parameter::NotProvided,
                    Parameter::String(name.clone()),
                    Parameter::NotProvided,
                    Parameter::NotProvided,
                    Parameter::Ref(Name::Entity(placement)),
                    Parameter::NotProvided,
                    Parameter::String(name),
                    Parameter::Enumeration("ELEMENT".to_string()),
                    Parameter::Enumeration("INTERNAL".to_string()),
                    Parameter::NotProvided,
                ];
                part21::upsert_simple(exchange, id, "IFCSPACE", args)
            }
        }
    }

    fn set_type_assignment(exchange: &mut Exchange, params: &Json) -> Result<(), String> {
        let id = part21::u64_field(params, "id")?;
        match part21::opt_obj_field(params, "assignment") {
            None => part21::remove(exchange, id, &["IFCRELDEFINESBYTYPE"]),
            Some(assignment) => {
                let relating_type = part21::u64_field(assignment, "relatingType")?;
                if !part21::find(exchange, relating_type).map(part21::type_name).unwrap_or("").ends_with("TYPE") {
                    return Err(format!("#{relating_type} is not an IFC*TYPE -- COBie's Type sheet relates maintainable products to a real type"));
                }
                let related = part21::u64_array(assignment, "relatedObjects");
                if related.is_empty() {
                    return Err("an IFCRELDEFINESBYTYPE with no RelatedObjects assigns nothing".to_string());
                }
                for object in &related {
                    if part21::find(exchange, *object).is_none() {
                        return Err(format!("no instance #{object} to relate to the type"));
                    }
                }
                let owner = part21::opt_u64_field(assignment, "ownerHistory")?.map(|id| Parameter::Ref(Name::Entity(id))).unwrap_or(Parameter::NotProvided);
                let mut args = vec![Parameter::String(part21::str_field(assignment, "globalId")?), owner, Parameter::NotProvided, Parameter::NotProvided];
                args.push(Parameter::List(related.into_iter().map(|object| Parameter::Ref(Name::Entity(object))).collect()));
                args.push(Parameter::Ref(Name::Entity(relating_type)));
                part21::upsert_simple(exchange, id, "IFCRELDEFINESBYTYPE", args)
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
            "set-facility-name" => set_facility_name(exchange, params),
            "set-floor-elevation" => set_floor_elevation(exchange, params),
            "set-space" => set_space(exchange, params),
            "set-type-assignment" => set_type_assignment(exchange, params),
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

    fn sheet(exchange: &Exchange, types: &[&str], columns: &[(&str, usize)]) -> Json {
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

    /// 👁️ The independently read projection: the shared Part-21 graph plus the four COBie sheets
    /// this subset's own vocabulary edits, so a comparison states the handover's own surface.
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        let exchange = part21::read(bytes)?;
        let mut fields = part21::project_graph(&exchange);
        fields.push(("facilities".to_string(), sheet(&exchange, &["IFCBUILDING"], &[("name", NAME_INDEX)])));
        fields.push(("floors".to_string(), sheet(&exchange, &["IFCBUILDINGSTOREY"], &[("name", NAME_INDEX), ("elevation", STOREY_ELEVATION_INDEX)])));
        fields.push(("spaces".to_string(), sheet(&exchange, &["IFCSPACE"], &[("name", NAME_INDEX)])));
        fields.push((
            "typeAssignments".to_string(),
            sheet(&exchange, &["IFCRELDEFINESBYTYPE"], &[("relatedObjects", RELATED_OBJECTS_INDEX), ("relatingType", RELATING_TYPE_INDEX)]),
        ));
        Ok(Json::Object(fields))
    }
    //#endregion 🔖️Projection
}
//#endregion 🔖️Oracles

//#region 🔖️Dispatch
/// 🦠️ Applies one declared `ifc-2x3-cobie` kind to a real artifact and returns the re-serialized
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
pub fn project_ifc_2x3_cobie(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_ifc_2x3_cobie(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🧪️Tests
/// 🧪️ Exercises every declared kind against the real committed IFC2X3 fixture. The fixture is a
/// real `CoordinationView_V2.0` export, not a native FM handover file — no real `FMHandOverView`
/// document exists in this repository — so `set-view-definition` performs the real stamping step an
/// FM handover extraction begins with, and every other kind edits real handover data.
#[cfg(all(test, feature = "oracles"))]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
