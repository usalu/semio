//! ✍️ Selected released templates reduce to the existing IFC graph mutation leaves.
use super::{IfcTemplate, IfcTemplateEntry, IfcTemplateEntryKind as Kind};
use crate::validation::metadata::{self, is_subtype as ifc_is_subtype};
fn attributes(name: &str) -> Option<&'static [metadata::StaticAttribute]> {
    if name.len() > 100 {
        return None;
    }
    metadata::entity(name).map(|entity| entity.all_attributes)
}
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity, IfcSnapshot, IfcTypedValue, IfcValue};
use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::{insert_entity::InsertEntity, set_entity_arg::SetEntityArg, IfcMutation};

pub const APPLY_TEMPLATE_ACTION: &str = "applyIfcTemplate";
pub const APPLICATION_SCHEMA: &str = "https://semio.tech/s/stdio/ifc/4x3/template-application.schema.json";
pub const COMMAND_SCHEMA: &str = "https://semio.tech/s/stdio/ifc/4x3/template-command.schema.json";
pub const MAXIMUM_ENTRIES: usize = 26;
pub const MAXIMUM_IDENTITIES: usize = MAXIMUM_ENTRIES * 2 + 4;

#[derive(Default, Debug, semio_framework_value::RetireOwned)]
pub struct IfcTemplateApplication {
    pub template: String,
    pub target: u64,
    pub entries: Vec<IfcTemplateSelection>,
    pub identities: Vec<IfcTemplateIdentity>,
    pub owner_history: Option<u64>,
    pub description: Option<String>,
    pub method_of_measurement: Option<String>,
    pub performance: Option<IfcTemplatePerformance>,
}
#[derive(Debug, semio_framework_value::RetireOwned)]
pub struct IfcTemplateIdentity {
    pub key: String,
    pub id: u64,
    pub guid: Option<String>,
}
#[derive(Debug, semio_framework_value::RetireOwned)]
pub struct IfcTemplatePerformance {
    pub life_cycle_phase: String,
    pub predefined_type: String,
    pub object_type: Option<String>,
    pub identification: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}
#[derive(Debug, semio_framework_value::RetireOwned)]
pub struct IfcTemplateSelection {
    pub name: String,
    pub description: Option<String>,
    pub form: IfcTemplateForm,
}
#[derive(Debug, semio_framework_value::RetireOwned)]
pub enum IfcTemplateForm {
    Single { value: IfcValue, unit: Option<u64> },
    Reference { value: u64, usage_name: Option<String> },
    Enumerated { values: Vec<IfcValue>, unit: Option<u64> },
    List { values: Vec<IfcValue>, unit: Option<u64> },
    Bounded { lower: Option<IfcValue>, upper: Option<IfcValue>, set_point: Option<IfcValue>, unit: Option<u64> },
    Table { defining: Vec<IfcValue>, defined: Vec<IfcValue>, defining_unit: Option<u64>, defined_unit: Option<u64>, expression: Option<String>, interpolation: Option<String> },
    Quantity { kind: IfcTemplateQuantityKind, value: IfcValue, unit: Option<u64>, formula: Option<String> },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum IfcTemplateQuantityKind {
    Length,
    Area,
    Volume,
    Weight,
    Count,
    Time,
}
impl IfcTemplateQuantityKind {
    pub fn entry_kind(self) -> Kind {
        match self {
            Self::Length => Kind::Length,
            Self::Area => Kind::Area,
            Self::Volume => Kind::Volume,
            Self::Weight => Kind::Weight,
            Self::Count => Kind::Count,
            Self::Time => Kind::Time,
        }
    }
    pub fn declaration(self) -> (&'static str, &'static str) {
        match self {
            Self::Length => ("IFCQUANTITYLENGTH", "LengthValue"),
            Self::Area => ("IFCQUANTITYAREA", "AreaValue"),
            Self::Volume => ("IFCQUANTITYVOLUME", "VolumeValue"),
            Self::Weight => ("IFCQUANTITYWEIGHT", "WeightValue"),
            Self::Count => ("IFCQUANTITYCOUNT", "CountValue"),
            Self::Time => ("IFCQUANTITYTIME", "TimeValue"),
        }
    }
}
impl IfcTemplateForm {
    pub fn kind(&self) -> Kind {
        match self {
            Self::Single { .. } => Kind::SingleValue,
            Self::Reference { .. } => Kind::ReferenceValue,
            Self::Enumerated { .. } => Kind::EnumeratedValue,
            Self::List { .. } => Kind::ListValue,
            Self::Bounded { .. } => Kind::BoundedValue,
            Self::Table { .. } => Kind::TableValue,
            Self::Quantity { kind, .. } => kind.entry_kind(),
        }
    }
}
#[derive(Debug)]
pub struct IfcTemplateError {
    pub code: &'static str,
    pub message: &'static str,
}
impl std::fmt::Display for IfcTemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}
impl std::error::Error for IfcTemplateError {}
pub(crate) fn error(code: &'static str, message: &'static str) -> IfcTemplateError {
    IfcTemplateError { code, message }
}
pub type TemplateResult<T> = Result<T, IfcTemplateError>;
pub(crate) fn field<'a>(entity: &'a IfcEntity, name: &str) -> Option<&'a IfcValue> {
    if entity.name.len() > 100 {
        return None;
    }
    attributes(&entity.name)?.iter().position(|attribute| attribute.name == name).and_then(|index| entity.args.get(index))
}
pub(crate) fn text<'a>(entity: &'a IfcEntity, name: &str) -> Option<&'a str> {
    field(entity, name).and_then(IfcValue::as_str)
}
pub(crate) fn reference(id: Option<u64>) -> IfcValue {
    id.map_or(IfcValue::Unset, IfcValue::Reference)
}
pub(crate) fn optional_text(value: Option<String>) -> IfcValue {
    value.map_or(IfcValue::Unset, IfcValue::String)
}
pub(crate) fn typed(value: IfcValue, domain: &str) -> TemplateResult<IfcValue> {
    match value {
        IfcValue::TypedValue(mut value) if value.name.eq_ignore_ascii_case(domain) && value.items.len() == 1 => {
            value.name.make_ascii_uppercase();
            Ok(IfcValue::TypedValue(value))
        }
        IfcValue::Unset | IfcValue::Derived | IfcValue::Reference(_) | IfcValue::TypedValue(_) => Err(error("invalidDomain", "Selected value requires exactly its released scalar domain")),
        value => Ok(IfcValue::TypedValue(IfcTypedValue { name: domain.to_ascii_uppercase(), items: vec![value] })),
    }
}
pub(crate) fn unit_kind(domain: &str) -> Option<&'static str> {
    let mut key = [0u8; 128];
    if domain.len() > key.len() {
        return None;
    }
    for (slot, byte) in key.iter_mut().zip(domain.bytes()) {
        *slot = byte.to_ascii_uppercase();
    }
    let mut name = std::str::from_utf8(&key[..domain.len()]).ok()?;
    for _ in 0..64 {
        if name == "IFCMONETARYMEASURE" {
            return Some("MONETARY");
        }
        if ["IFCRATIOMEASURE", "IFCCOUNTMEASURE", "IFCNUMERICMEASURE"].contains(&name) {
            return Some("DIMENSIONLESS");
        }
        if name == "IFCTHERMALCONDUCTIVITYMEASURE" {
            return Some("THERMALCONDUCTANCEUNIT");
        }
        let candidate = name.strip_prefix("IFC").unwrap_or(name).strip_suffix("MEASURE");
        if let Some(candidate) = candidate {
            for kind in ["IFCUNITENUM", "IFCDERIVEDUNITENUM"] {
                if let Some(member) = metadata::defined_type(kind).and_then(|declaration| declaration.members.iter().find(|member| member.strip_suffix("UNIT") == Some(candidate))) {
                    return Some(member);
                }
            }
        }
        let Some(expression) = metadata::defined_type(name).and_then(|declaration| declaration.type_expr).filter(|expression| expression.kind == "named") else {
            return None;
        };
        let Some(next) = expression.name else {
            return None;
        };
        if next == name {
            return None;
        }
        name = next;
    }
    None
}

#[derive(Clone, Copy)]
pub(crate) struct PreparedEntry {
    pub declaration: &'static IfcTemplateEntry,
    pub enumeration: Option<u64>,
    pub enum_key: Option<usize>,
}
pub(crate) struct Prepared {
    pub template: &'static IfcTemplate,
    pub target: usize,
    pub entries: [Option<PreparedEntry>; MAXIMUM_ENTRIES],
    pub type_target: bool,
    pub performance: bool,
}
pub(crate) struct Reader<'a> {
    pub snapshot: &'a IfcSnapshot,
    pub order: &'a [usize],
}
impl Reader<'_> {
    pub fn contains(&self, id: u64) -> bool {
        self.order.binary_search_by_key(&id, |index| self.snapshot.entities[*index].id).is_ok()
    }
    pub fn entity(&self, id: u64) -> TemplateResult<&IfcEntity> {
        self.order.binary_search_by_key(&id, |index| self.snapshot.entities[*index].id).ok().map(|index| &self.snapshot.entities[self.order[index]]).ok_or_else(|| error("missingEntity", "Requested entity does not exist"))
    }
    pub fn position(&self, id: u64) -> TemplateResult<usize> {
        self.order.binary_search_by_key(&id, |index| self.snapshot.entities[*index].id).ok().map(|index| self.order[index]).ok_or_else(|| error("missingEntity", "Requested entity does not exist"))
    }
    pub fn unit(&self, id: Option<u64>, domain: &str, quantity: bool) -> TemplateResult<()> {
        let Some(id) = id else {
            return Ok(());
        };
        let entity = self.entity(id)?;
        match unit_kind(domain).as_deref() {
            Some("MONETARY") if entity.name.eq_ignore_ascii_case("IFCMONETARYUNIT") => Ok(()),
            Some("DIMENSIONLESS") if ifc_is_subtype(&entity.name, "IFCNAMEDUNIT") => {
                let dimensions = field(entity, "Dimensions").and_then(IfcValue::as_reference).ok_or_else(|| error("invalidUnit", "Dimensionless named units require explicit zero exponents"))?;
                let dimensions = self.entity(dimensions)?;
                if dimensions.args.len() == 7 && dimensions.args.iter().all(|value| matches!(value, IfcValue::Integer(0))) {
                    Ok(())
                } else {
                    Err(error("invalidUnit", "Dimensionless named units require seven zero exponents"))
                }
            }
            Some(expected)
                if field(entity, "UnitType").and_then(IfcValue::as_enum).is_some_and(|actual| actual.eq_ignore_ascii_case(expected))
                    && (ifc_is_subtype(&entity.name, "IFCNAMEDUNIT") || entity.name.eq_ignore_ascii_case("IFCDERIVEDUNIT"))
                    && (!quantity || ifc_is_subtype(&entity.name, "IFCNAMEDUNIT")) =>
            {
                Ok(())
            }
            None if (ifc_is_subtype(&entity.name, "IFCNAMEDUNIT") || entity.name.eq_ignore_ascii_case("IFCDERIVEDUNIT") || entity.name.eq_ignore_ascii_case("IFCMONETARYUNIT")) && (!quantity || ifc_is_subtype(&entity.name, "IFCNAMEDUNIT")) => Ok(()),
            _ => Err(error("invalidUnit", "Explicit unit does not match its released measure domain")),
        }
    }
}

pub(crate) fn prepare(reader: &Reader<'_>, application: &IfcTemplateApplication) -> TemplateResult<Prepared> {
    let template = super::ifc4x3_template(&application.template).ok_or_else(|| error("unknownTemplate", "Unknown released IFC4x3 template"))?;
    if application.entries.is_empty() {
        return Err(error("emptySelection", "Select at least one published entry"));
    }
    if application.entries.len() > template.entries.len() || application.entries.len() > MAXIMUM_ENTRIES || application.identities.len() > MAXIMUM_IDENTITIES {
        return Err(error("wrongForm", "Selected entries exceed the published template"));
    }
    let position = reader.position(application.target)?;
    let target = &reader.snapshot.entities[position];
    if !target.complex.is_empty() {
        return Err(error("wrongCarrier", "Complex template targets require constituent coordinate authoring"));
    }
    let applicability = super::ifc4x3_template_applicability(template, &target.name, field(target, "PredefinedType").and_then(IfcValue::as_enum));
    if applicability != super::IfcTemplateApplicability::Applicable {
        return Err(error(applicability.source_key(), "Target does not satisfy released template applicability"));
    }
    let material = template.kind == "PSET_MATERIALDRIVEN";
    let profile = template.kind == "PSET_PROFILEDRIVEN";
    let performance = template.kind == "PSET_PERFORMANCEDRIVEN";
    let type_target = ifc_is_subtype(&target.name, "IFCTYPEOBJECT");
    let carrier = if material {
        ifc_is_subtype(&target.name, "IFCMATERIALDEFINITION")
    } else if profile {
        ifc_is_subtype(&target.name, "IFCPROFILEDEF")
    } else if performance {
        ifc_is_subtype(&target.name, "IFCOBJECTDEFINITION")
    } else if type_target {
        template.kind.contains("TYPEDRIVENOVERRIDE")
    } else {
        ifc_is_subtype(&target.name, "IFCOBJECT")
    };
    if !carrier {
        return Err(error("wrongCarrier", "Target does not support this published carrier"));
    }
    if performance != application.performance.is_some() {
        return Err(error("performanceRequired", "Only performance templates require explicit lifecycle and classification"));
    }
    if application
        .performance
        .as_ref()
        .is_some_and(|performance| performance.predefined_type != "NOTDEFINED" && performance.predefined_type != "USERDEFINED" || performance.predefined_type == "USERDEFINED" && performance.object_type.as_ref().is_none_or(String::is_empty))
    {
        return Err(error("invalidDomain", "Performance history classification requires its explicit ObjectType"));
    }
    if !template.kind.starts_with("QTO_") && application.method_of_measurement.is_some() {
        return Err(error("wrongForm", "MethodOfMeasurement belongs only to quantity sets"));
    }
    if let Some(id) = application.owner_history {
        if !ifc_is_subtype(&reader.entity(id)?.name, "IFCOWNERHISTORY") {
            return Err(error("invalidDomain", "OwnerHistory must reference IfcOwnerHistory"));
        }
    }
    let mut entries = [None; MAXIMUM_ENTRIES];
    for (index, selected) in application.entries.iter().enumerate() {
        if application.entries[..index].iter().any(|entry| entry.name == selected.name) {
            return Err(error("duplicateEntry", "An entry is selected more than once"));
        }
        let declaration = template.entries.iter().find(|entry| entry.name == selected.name).ok_or_else(|| error("unknownEntry", "Selected entry is not published by this template"))?;
        if declaration.kind != selected.form.kind() {
            return Err(error("wrongForm", "Selected entry form differs from the released template"));
        }
        match &selected.form {
            IfcTemplateForm::Reference { value, .. } => {
                if !ifc_is_subtype(&reader.entity(*value)?.name, declaration.data_type) {
                    return Err(error("invalidDomain", "Reference does not match the published resource domain"));
                }
            }
            IfcTemplateForm::Single { unit, .. } | IfcTemplateForm::List { unit, .. } | IfcTemplateForm::Enumerated { unit, .. } | IfcTemplateForm::Bounded { unit, .. } => reader.unit(*unit, declaration.data_type, false)?,
            IfcTemplateForm::Quantity { unit, .. } => reader.unit(*unit, declaration.data_type, true)?,
            IfcTemplateForm::Table { defining, defined, defining_unit, defined_unit, .. } => {
                if defining.is_empty() || defining.len() != defined.len() {
                    return Err(error("tableDimensions", "Table columns require equally sized nonempty rows"));
                }
                reader.unit(*defining_unit, declaration.data_type, false)?;
                reader.unit(*defined_unit, declaration.secondary_type.ok_or_else(|| error("wrongForm", "Published table requires its secondary domain"))?, false)?;
            }
        }
        if matches!(&selected.form, IfcTemplateForm::Bounded { lower: None, upper: None, set_point: None, .. }) {
            return Err(error("invalidDomain", "A bounded value requires a bound or set point"));
        }
        if matches!(&selected.form, IfcTemplateForm::Enumerated { values, .. } | IfcTemplateForm::List { values, .. } if values.is_empty()) {
            return Err(error("invalidDomain", "Selected lists cannot be empty"));
        }
        entries[index] = Some(PreparedEntry { declaration, enumeration: None, enum_key: None });
    }
    Ok(Prepared { template, target: position, entries, type_target, performance })
}

pub(crate) fn key_matches(key: usize, value: &str) -> bool {
    if key >= MAXIMUM_ENTRIES * 2 {
        return ["set", "relation", "performance", "control"].get(key - MAXIMUM_ENTRIES * 2).is_some_and(|expected| *expected == value);
    }
    let (prefix, index) = if key < MAXIMUM_ENTRIES { ("entry:", key) } else { ("enum:", key - MAXIMUM_ENTRIES) };
    let Some(number) = value.strip_prefix(prefix) else {
        return false;
    };
    let bytes = number.as_bytes();
    if index < 10 {
        bytes.len() == 1 && bytes[0] == b'0' + index as u8
    } else {
        bytes.len() == 2 && bytes[0] == b'0' + (index / 10) as u8 && bytes[1] == b'0' + (index % 10) as u8
    }
}
pub(crate) fn identity(application: &mut IfcTemplateApplication, key: usize) -> TemplateResult<&mut IfcTemplateIdentity> {
    application.identities.iter_mut().find(|identity| key_matches(key, &identity.key)).ok_or_else(|| error("identityMissing", "Retain every planned resource identity"))
}
pub(crate) fn entity<const N: usize>(application: &mut IfcTemplateApplication, key: usize, name: &str, mut fields: [(&str, IfcValue); N]) -> TemplateResult<IfcEntity> {
    let owner = application.owner_history;
    let identity = identity(application, key)?;
    let rooted = ifc_is_subtype(name, "IFCROOT");
    let guid = if rooted {
        Some(identity.guid.take().ok_or_else(|| error("identityMissing", "Retain GlobalId for every new IfcRoot"))?)
    } else {
        if identity.guid.is_some() {
            return Err(error("identityUnexpected", "Resource entities cannot own GlobalId"));
        }
        None
    };
    let attributes = attributes(name).ok_or_else(|| error("invalidDomain", "Unknown released carrier entity"))?;
    let mut args = Vec::with_capacity(attributes.len());
    let mut guid = guid;
    for attribute in attributes {
        args.push(if attribute.name == "GlobalId" {
            optional_text(guid.take())
        } else if attribute.name == "OwnerHistory" {
            reference(owner)
        } else if let Some((_, value)) = fields.iter_mut().find(|(name, _)| *name == attribute.name) {
            std::mem::take(value)
        } else if attribute.derived {
            IfcValue::Derived
        } else {
            IfcValue::Unset
        });
    }
    Ok(IfcEntity { id: identity.id, name: name.into(), args, complex: Vec::new() })
}
pub(crate) fn insert(mutations: &mut Vec<IfcMutation>, source_count: usize, entity: IfcEntity) {
    mutations.push(IfcMutation::InsertEntity(InsertEntity { index: source_count + mutations.iter().filter(|mutation| matches!(mutation, IfcMutation::InsertEntity(_))).count(), entity }));
}
pub(crate) fn type_attachment(target: &IfcEntity, set: u64, mut properties: Vec<IfcValue>) -> TemplateResult<IfcMutation> {
    let index = attributes(&target.name).and_then(|attributes| attributes.iter().position(|attribute| attribute.name == "HasPropertySets")).ok_or_else(|| error("wrongCarrier", "Type target has no HasPropertySets"))?;
    properties.push(IfcValue::Reference(set));
    Ok(IfcMutation::SetEntityArg(SetEntityArg { id: target.id, index, value: IfcValue::Aggregate(properties) }))
}

pub(crate) fn independent_batch(reader: &Reader<'_>, mutations: &[IfcMutation]) -> TemplateResult<()> {
    if mutations.len() > MAXIMUM_IDENTITIES {
        return Err(error("publicationCapacity", "Template batch exceeds released identity capacity"));
    }
    let mut insertions = 0;
    for (ordinal, mutation) in mutations.iter().enumerate() {
        match mutation {
            IfcMutation::InsertEntity(value) => {
                if value.index != reader.snapshot.entities.len() + insertions
                    || reader.contains(value.entity.id)
                    || mutations[..ordinal].iter().any(|previous| matches!(previous, IfcMutation::InsertEntity(previous) if previous.entity.id == value.entity.id))
                {
                    return Err(error("publicationPreimage", "Template insertions must be distinct original append coordinates"));
                }
                insertions += 1;
            }
            IfcMutation::SetEntityArg(value) => {
                let target = reader.entity(value.id).map_err(|_| error("publicationPreimage", "Template field edits require an original source preimage"))?;
                if value.index >= target.args.len() || mutations[..ordinal].iter().any(|previous| matches!(previous, IfcMutation::SetEntityArg(previous) if previous.id == value.id && previous.index == value.index)) {
                    return Err(error("publicationPreimage", "Template batches may write each original field once"));
                }
            }
            _ => return Err(error("publicationPreimage", "Template publication accepts only final insertions and distinct original field writes")),
        }
    }
    Ok(())
}

#[path = "receive.rs"]
pub mod receive;
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
#[path = "work.rs"]
pub mod work;
