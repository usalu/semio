//! 🪪️ Admitted native stream meanings bound to exact retained logical identities.
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfObject, PdfIndirectObject, PdfOp, PdfToUnicode, PdfEmbeddedCMap, PdfFontProgram, PdfImage};

/// 🧭️ An exact location within an indirect logical object.
#[derive(Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfGraphIdentity {
    pub owner: ObjRef,
    pub path: Vec<PdfGraphPath>,
}

/// 🗺️ Dictionary keys and array ordinals occupy distinct namespaces.
#[derive(Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfGraphPath {
    Entry { key: String },
    Item { index: usize },
}

/// 🧬️ Semantic stream words admitted by native IO.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfStreamRoleValue {
    Operators { content: Vec<PdfOp> },
    SampledWords { samples: Vec<u32> },
    CalculatorProgram { code: String },
    UnicodeMap { mapping: PdfToUnicode },
    CharacterMap { cmap: PdfEmbeddedCMap },
    FontProgram { program: PdfFontProgram },
    Image { image: PdfImage },
    MetadataText { text: String },
    AttachmentBytes { bytes: Vec<u8> },
    PaletteComponents { components: Vec<u8> },
    GlyphIds { glyphs: Vec<u16> },
    ReferenceBody { reference: semio_framework_artifact_reference::ArtifactRef },
}

/// 🏷️ Role selection is semantic and never selects by native codec or stream bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdfStreamRoleKind { Operators, SampledWords, CalculatorProgram, UnicodeMap, CharacterMap, FontProgram, Image, MetadataText, AttachmentBytes, PaletteComponents, GlyphIds, ReferenceBody }

impl PdfStreamRoleValue {
    /// 🎛️ Selects the declared semantic role.
    pub fn kind(&self) -> PdfStreamRoleKind {
        match self {
            Self::Operators { .. } => PdfStreamRoleKind::Operators,
            Self::SampledWords { .. } => PdfStreamRoleKind::SampledWords,
            Self::CalculatorProgram { .. } => PdfStreamRoleKind::CalculatorProgram,
            Self::UnicodeMap { .. } => PdfStreamRoleKind::UnicodeMap,
            Self::CharacterMap { .. } => PdfStreamRoleKind::CharacterMap,
            Self::FontProgram { .. } => PdfStreamRoleKind::FontProgram,
            Self::Image { .. } => PdfStreamRoleKind::Image,
            Self::MetadataText { .. } => PdfStreamRoleKind::MetadataText,
            Self::AttachmentBytes { .. } => PdfStreamRoleKind::AttachmentBytes,
            Self::PaletteComponents { .. } => PdfStreamRoleKind::PaletteComponents,
            Self::GlyphIds { .. } => PdfStreamRoleKind::GlyphIds,
            Self::ReferenceBody { .. } => PdfStreamRoleKind::ReferenceBody,
        }
    }
}

/// 🧷️ Every role names its owner and the graph inputs its admission consumed.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfAdmittedStreamRole {
    pub identity: PdfGraphIdentity,
    pub dependencies: Vec<PdfGraphIdentity>,
    pub value: PdfStreamRoleValue,
}

/// 🎯️ Resolves an exact generation and structural path without decoding native data.
pub fn resolve_identity<'a>(objects: &'a [PdfIndirectObject], identity: &PdfGraphIdentity) -> Option<&'a PdfObject> {
    let mut value = &objects.iter().find(|object| object.id == identity.owner)?.value;
    for part in &identity.path {
        value = match part {
            PdfGraphPath::Entry { key } => value.dict_get(key)?,
            PdfGraphPath::Item { index } => value.as_array()?.get(*index)?,
        };
    }
    Some(value)
}

/// 🔐️ A changed admitted input requires the corresponding explicit semantic payload.
pub fn validate_role_inputs(base: &[PdfIndirectObject], next: &[PdfIndirectObject], roles: &[PdfAdmittedStreamRole], replacements: &[PdfAdmittedStreamRole]) -> Result<(), String> {
    let original=crate::standards::v1_7::subsets::base::schema::graph_source::GraphSource::new(base);
    let updated=crate::standards::v1_7::subsets::base::schema::graph_source::GraphSource::new(next);
    for role in roles {
        let inputs = original.dependencies_of(&role.identity);
        let changed = role.dependencies.iter().chain(&inputs).any(|identity| resolve_identity(base, identity) != resolve_identity(next, identity));
        if changed && !replacements.iter().any(|replacement| replacement.identity == role.identity && replacement.value.kind() == role.value.kind()) {
            return Err(format!("known stream role at {} {} requires a semantic replacement", role.identity.owner.num, role.identity.owner.gen));
        }
    }
    for (index, replacement) in replacements.iter().enumerate() {
        if replacements[..index].iter().any(|current| current.identity == replacement.identity && current.value.kind() == replacement.value.kind()) { return Err("semantic stream role replacement is duplicated".into()); }
        let required = updated.dependencies_of(&replacement.identity);
        if required.iter().any(|input| !replacement.dependencies.contains(input)) { return Err("semantic stream role payload omits a consumed logical dependency".into()); }
        if resolve_identity(next, &replacement.identity).is_none() || replacement.dependencies.iter().any(|identity| resolve_identity(next, identity).is_none()) {
            return Err("semantic stream role identity or dependency does not resolve".into());
        }
    }
    Ok(())
}

/// 🔁️ Protects still-present native inputs while permitting explicit logical role deletion.
pub fn validate_role_transition(base: &[PdfIndirectObject], next: &[PdfIndirectObject], roles: &[PdfAdmittedStreamRole], next_roles: &[PdfAdmittedStreamRole], replacements: &[PdfAdmittedStreamRole]) -> Result<(), String> {
    let original=crate::standards::v1_7::subsets::base::schema::graph_source::GraphSource::new(base);
    let retained:Vec<_>=roles.iter().filter(|role| {
        resolve_identity(next,&role.identity).is_some()
            || next_roles.iter().any(|candidate|candidate.identity==role.identity&&candidate.value.kind()==role.value.kind())
            || role.dependencies.iter().chain(&original.dependencies_of(&role.identity)).any(|input|resolve_identity(next,input).is_some()&&resolve_identity(base,input)!=resolve_identity(next,input))
    }).cloned().collect();
    validate_role_inputs(base,next,&retained,replacements)?;
    validate_role_inputs(next,next,&[],next_roles)
}

/// 📄️ Reads an admitted page-content role by its actual owner.
pub fn page_content(roles: &[PdfAdmittedStreamRole], owner: ObjRef) -> Option<&[PdfOp]> {
    let path = vec![PdfGraphPath::Entry { key: "Contents".into() }];
    roles.iter().find_map(|role| {
        if role.identity.owner != owner || role.identity.path != path { return None; }
        match &role.value { PdfStreamRoleValue::Operators { content } => Some(content.as_slice()), _ => None }
    })
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
