//! 🧬️ IDML package replacement and member editing publish validated sparse diffs.
use semio_framework_os_kernel as protocol;
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::{IdmlSnapshot, IdmlDiff};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase", deny_unknown_fields)]
pub enum IdmlMutation {
    ReplacePackage { document: IdmlSnapshot },
    ReplacePart { path: String, bytes: Vec<u8> },
}

const PACKAGE_SCHEMA: &str = include_str!("📦️replace-package/🧬️schema/🔣️.json");
const PART_SCHEMA: &str = include_str!("📄️replace-part/🧬️schema/🔣️.json");
const DOCUMENTS: &[&str] = &[include_str!("../🔣️.json")];
const DESCRIPTORS: &[protocol::MutationLeafDescriptor] = &[
    protocol::MutationLeafDescriptor { schema_version: 1, owner: concat!("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/🧬️mutations", "/📦️replace-package"), semantic_kind: "replace-package", display_name: "Replace IDML Package", emoji: "📦️", aggregate_variant: "ReplacePackage", payload_schema: "🧬️schema/🔣️.json", text_opcode: Some("replace-package"), binary_tag: Some(1), invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied, protocol::MutationOutcomeClass::NoOp, protocol::MutationOutcomeClass::Rejected], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::Typescript, protocol::MutationLanguageSurface::Graphql, protocol::MutationLanguageSurface::JsonSchema, protocol::MutationLanguageSurface::Protobuf, protocol::MutationLanguageSurface::Text, protocol::MutationLanguageSurface::Binary] },
    protocol::MutationLeafDescriptor { schema_version: 1, owner: concat!("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/🧬️mutations", "/📄️replace-part"), semantic_kind: "replace-part", display_name: "Replace IDML Part", emoji: "📄️", aggregate_variant: "ReplacePart", payload_schema: "🧬️schema/🔣️.json", text_opcode: Some("replace-part"), binary_tag: Some(2), invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied, protocol::MutationOutcomeClass::NoOp, protocol::MutationOutcomeClass::Rejected], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::Typescript, protocol::MutationLanguageSurface::Graphql, protocol::MutationLanguageSurface::JsonSchema, protocol::MutationLanguageSurface::Protobuf, protocol::MutationLanguageSurface::Text, protocol::MutationLanguageSurface::Binary] },
];

impl IdmlMutation {
    fn candidate(&self, base: &IdmlSnapshot) -> Result<IdmlSnapshot, ValueError> {
        match self {
            Self::ReplacePackage { document } => { document.package()?; Ok(document.clone()) }
            Self::ReplacePart { path, bytes } => {
                let mut parts = base.parts.clone();
                let part = parts.iter_mut().find(|part| part.path == *path).ok_or_else(|| crate::schema::snapshot::invalid("stdio.idml.missing-member"))?;
                part.bytes = bytes.clone();
                let package = crate::IdmlPackage::from_parts(parts.into_iter().map(|part| (part.path, part.bytes)).collect()).map_err(crate::schema::snapshot::invalid)?;
                Ok(IdmlSnapshot::from_package(package))
            }
        }
    }
}

impl protocol::Mutation<IdmlSnapshot> for IdmlMutation {
    type Diff = IdmlDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = DESCRIPTORS;
    const INPUT_SCHEMAS: &'static [&'static str] = &[PACKAGE_SCHEMA, PART_SCHEMA];
    const INPUT_SCHEMA_DOCUMENTS: &'static [&'static [&'static str]] = &[DOCUMENTS, DOCUMENTS];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &DESCRIPTORS[match self { Self::ReplacePackage { .. } => 0, Self::ReplacePart { .. } => 1 }] }
    fn input_schema(&self) -> Option<&'static str> { Some(match self { Self::ReplacePackage { .. } => PACKAGE_SCHEMA, Self::ReplacePart { .. } => PART_SCHEMA }) }
    fn conflict_target(&self) -> Vec<String> { match self { Self::ReplacePackage { .. } => Vec::new(), Self::ReplacePart { path, .. } => vec!["parts".into(), path.clone()] } }
    fn may_emit_foreign_steps(&self) -> bool { false }
    fn retire_cold(self) { drop(self); }
    fn payload_value(&self) -> DslValue { match self {
        Self::ReplacePackage { document } => DslValue::object(vec![("document".into(), document.to_value())]),
        Self::ReplacePart { path, bytes } => DslValue::object(vec![("path".into(), path.to_value()), ("bytes".into(), bytes.to_value())]),
    } }
    fn with_payload_value(&self, value: DslValue) -> Result<Self, ValueError> { Self::from_payload_value(self.descriptor().semantic_kind, value) }
    fn from_payload_value(kind: &str, value: DslValue) -> Result<Self, ValueError> {
        let tag = match kind { "replace-package" => "replacePackage", "replace-part" => "replacePart", _ => return Err(crate::schema::snapshot::invalid("stdio.idml.unknown-mutation")) };
        let mut fields = value.into_object()?;
        fields.push(("mutation".into(), DslValue::String(tag.into())));
        Self::from_value(DslValue::object(fields))
    }
}
impl protocol::ColdMutation<IdmlSnapshot> for IdmlMutation {
    type ReferenceDiff = IdmlDiff;
    fn diff(&self, base: &IdmlSnapshot) -> protocol::MutationOutcome<IdmlDiff> {
        let next = match self.candidate(base) { Ok(next) => next, Err(error) => return protocol::MutationOutcome::fatal("mutation.invariant", error.to_string(), std::iter::empty::<String>()) };
        if next == *base { return protocol::MutationOutcome::empty().warning("mutation.no-op", "The IDML package already has this content."); }
        protocol::MutationOutcome::new(IdmlDiff::changing(base, &next))
    }
    fn inverse(&self, base: &IdmlSnapshot) -> Result<Vec<Self>, ValueError> {
        let Ok(next) = self.candidate(base) else { return Ok(Vec::new()); };
        if next == *base { return Ok(Vec::new()); }
        Ok(vec![match self {
            Self::ReplacePackage { .. } => Self::ReplacePackage { document: base.clone() },
            Self::ReplacePart { path, .. } => Self::ReplacePart { path: path.clone(), bytes: base.parts.iter().find(|part| part.path == *path).ok_or_else(|| crate::schema::snapshot::invalid("stdio.idml.missing-member"))?.bytes.clone() },
        }])
    }
}

#[path="🧳️retained/🦀️.rs"]pub mod retained;
