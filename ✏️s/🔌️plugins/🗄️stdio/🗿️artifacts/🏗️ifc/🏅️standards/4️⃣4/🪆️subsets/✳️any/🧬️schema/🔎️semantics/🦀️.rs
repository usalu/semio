//! 🔎️ Declared IFC4 semantic attributes, stable identity and bounded graph projection.
use super::catalog::ifc4_is_subtype;
use super::snapshot::{IfcEntity, IfcSnapshot, IfcValue};
use semio_framework_value::DslValue;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

pub const DOMAIN: &str = "ifc-entities";
pub const GRANULARITY: &str = "entity";
pub const MAXIMUM_TEXT_BYTES: usize = 8192;

#[derive(Debug)]
pub struct SemanticDescriptor {
    pub id: String,
    pub family: String,
    pub index: usize,
    pub label_en: String,
    pub label_de: String,
    pub kind: String,
    pub optional: bool,
    pub editable: bool,
    pub max_characters: Option<usize>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticFieldState { Valid, Unset, Invalid, TooLarge }
#[derive(Debug)]
pub struct SemanticField { pub descriptor: &'static SemanticDescriptor, pub value: Option<String>, pub state: SemanticFieldState, pub reference: Option<u64> }
#[derive(Debug)]
pub struct SemanticProjection { pub id: u64, pub entity_type: String, pub fields: Vec<SemanticField>, pub forward: Vec<u64>, pub back: Vec<u64>, pub missing: Vec<u64> }
#[derive(Clone, Debug, PartialEq)]
pub struct SemanticEdit { pub id: u64, pub index: usize, pub value: IfcValue }

fn definition() -> &'static semio_framework_pack_json::Value {
    static DEFINITION: OnceLock<semio_framework_pack_json::Value> = OnceLock::new();
    DEFINITION.get_or_init(|| semio_framework_pack_json::parse(include_str!("🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned IFC4 semantic descriptor schema"))
}
fn catalog() -> &'static [SemanticDescriptor] {
    static CATALOG: OnceLock<Vec<SemanticDescriptor>> = OnceLock::new();
    CATALOG.get_or_init(|| definition()["attributes"].as_array().unwrap().iter().map(|row| SemanticDescriptor {
        id: row["id"].as_str().unwrap().into(), family: row["family"].as_str().unwrap().into(), index: row["index"].as_u64().unwrap() as usize,
        label_en: row["label"]["en"].as_str().unwrap().into(), label_de: row["label"]["de"].as_str().unwrap().into(), kind: row["kind"].as_str().unwrap().into(),
        optional: row["optional"].as_bool().unwrap(), editable: row["editable"].as_bool().unwrap(), max_characters: row.get("maxCharacters").and_then(semio_framework_pack_json::Value::as_u64).map(|value| value as usize),
    }).collect())
}
pub fn descriptors(entity: &IfcEntity) -> Vec<&'static SemanticDescriptor> {
    if !entity.complex.is_empty() { return Vec::new(); }
    catalog().iter().filter(|row|definition()["families"][row.family.as_str()].as_array().is_some_and(|bases|bases.iter().any(|base|ifc4_is_subtype(&entity.name,base.as_str().unwrap())))).collect()
}
pub fn semantic_identity(id: u64) -> String { format!("ifc-{id}") }
pub fn parse_semantic_identity(value: &str) -> Option<u64> {
    let number = value.strip_prefix("ifc-")?;
    let id: u64 = number.parse().ok()?;
    (id > 0 && id.to_string() == number).then_some(id)
}
fn valid_guid(value: &str) -> bool {
    value.len() == 22 && matches!(value.as_bytes()[0], b'0'..=b'3') && value.bytes().all(|value| value.is_ascii_alphanumeric() || value == b'_' || value == 36)
}
fn references(entity: &IfcEntity, tick: &mut dyn FnMut() -> Result<(), String>) -> Result<BTreeSet<u64>, String> {
    fn visit(value: &IfcValue, depth: usize, ids: &mut BTreeSet<u64>, tick: &mut dyn FnMut() -> Result<(), String>) -> Result<(), String> {
        tick()?;
        if depth > 128 { return Err("semantic-value-depth".into()); }
        match value {
            IfcValue::Reference(id) => { ids.insert(*id); },
            IfcValue::Aggregate(values) => { for value in values { visit(value, depth + 1, ids, tick)?; } },
            IfcValue::TypedValue(value) => { for value in &value.items { visit(value, depth + 1, ids, tick)?; } },
            _ => (),
        }
        Ok(())
    }
    let mut ids = BTreeSet::new();
    for value in &entity.args { visit(value, 0, &mut ids, tick)?; }
    for constituent in &entity.complex { for value in &constituent.args { visit(value, 0, &mut ids, tick)?; } }
    Ok(ids)
}
/// 🧭️ Project supported attributes and both reference directions under original caller control.
pub fn project_semantics(snapshot: &IfcSnapshot, id: u64, tick: &mut dyn FnMut() -> Result<(), String>) -> Result<SemanticProjection, String> {
    let mut entities = BTreeMap::new();
    for entity in &snapshot.entities { tick()?; if entities.insert(entity.id, entity).is_some() { return Err("semantic-duplicate-entity".into()); } }
    let entity = entities.get(&id).ok_or("semantic-entity-missing")?;
    let mut fields = Vec::new();
    for descriptor in descriptors(entity) {
        tick()?;
        let (value, state, reference) = match entity.args.get(descriptor.index) {
            Some(IfcValue::Unset) if descriptor.optional => (None, SemanticFieldState::Unset, None),
            Some(IfcValue::Reference(id)) if descriptor.kind == "reference" => (Some(format!("#{id}")), if entities.contains_key(id) {SemanticFieldState::Valid} else {SemanticFieldState::Invalid}, Some(*id)),
            Some(IfcValue::String(value)) if descriptor.kind != "reference" => {
                if value.len() > MAXIMUM_TEXT_BYTES { (Some("…".into()), SemanticFieldState::TooLarge, None) } else {
                    let valid = descriptor.max_characters.is_none_or(|maximum| value.chars().count() <= maximum) && (descriptor.kind != "global-id" || valid_guid(value));
                    (Some(value.clone()), if valid { SemanticFieldState::Valid } else { SemanticFieldState::Invalid }, None)
                }
            },
            _ => (Some("?".into()), SemanticFieldState::Invalid, None),
        };
        fields.push(SemanticField { descriptor, value, state, reference });
    }
    let mut forward=Vec::new();let mut missing=Vec::new();
    for reference in references(entity,tick)?{tick()?;if entities.contains_key(&reference){forward.push(reference);}else{missing.push(reference);}}
    let mut back = Vec::new();
    for candidate in &snapshot.entities { tick()?; if references(candidate, tick)?.contains(&id) { back.push(candidate.id); } }
    back.sort_unstable();
    Ok(SemanticProjection { id, entity_type: entity.name.clone(), fields, forward, back, missing })
}
/// 🖋️ Plan an existing stable-id argument edit after IFC4 scalar and optionality checks.
pub fn edit_semantic_controlled(snapshot: &IfcSnapshot, id: u64, field: &str, input: &DslValue,tick:&mut dyn FnMut()->Result<(),String>) -> Result<SemanticEdit, String> {
    let mut selected=None;for entity in &snapshot.entities{tick()?;if entity.id==id{selected=Some(entity);break;}}
    let entity = selected.ok_or("semantic-entity-missing")?;
    let descriptor = descriptors(entity).into_iter().find(|row| row.id == field).ok_or("semantic-unknown-attribute")?;
    if !descriptor.editable { return Err("semantic-read-only-attribute".into()); }
    match entity.args.get(descriptor.index){None=>return Err("semantic-attribute-missing".into()),Some(IfcValue::String(value)) if value.len()>MAXIMUM_TEXT_BYTES=>return Err("semantic-text-limit".into()),Some(IfcValue::String(_)|IfcValue::Unset)=>(),_=>return Err("semantic-current-value-kind".into())}
    let value = match input {
        DslValue::Null => { if !descriptor.optional { return Err("semantic-required-value".into()); } IfcValue::Unset },
        DslValue::String(value) => {
            if value.len() > MAXIMUM_TEXT_BYTES || descriptor.max_characters.is_some_and(|maximum| value.chars().count() > maximum) { return Err("semantic-text-limit".into()); }
            if descriptor.kind == "global-id" {
                if !valid_guid(value) { return Err("semantic-invalid-global-id".into()); }
                for candidate in &snapshot.entities{tick()?;if candidate.id==id{continue;}if ifc4_is_subtype(&candidate.name,"IFCROOT")&&candidate.args.first().and_then(IfcValue::as_str)==Some(value.as_str()){return Err("semantic-duplicate-global-id".into());}for constituent in &candidate.complex{tick()?;if constituent.name=="IFCROOT"&&constituent.args.first().and_then(IfcValue::as_str)==Some(value.as_str()){return Err("semantic-duplicate-global-id".into());}}}
            }
            IfcValue::String(value.clone())
        },
        _ => return Err("semantic-value-kind".into()),
    };
    Ok(SemanticEdit { id, index: descriptor.index, value })
}

pub fn edit_semantic(snapshot:&IfcSnapshot,id:u64,field:&str,input:&DslValue)->Result<SemanticEdit,String>{edit_semantic_controlled(snapshot,id,field,input,&mut ||Ok(()))}
pub fn editable_field(field:&str)->bool{catalog().iter().any(|row|row.id==field&&row.editable)}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
