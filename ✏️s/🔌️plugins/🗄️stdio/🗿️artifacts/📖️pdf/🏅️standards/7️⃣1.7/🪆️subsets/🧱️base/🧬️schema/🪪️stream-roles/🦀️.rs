//! 🪪️ Admitted native stream meanings bound to exact retained logical identities.
use crate::standards::v1_7::subsets::base::schema::graph_source::GraphSource;
use crate::standards::v1_7::subsets::base::schema::snapshot::model::{ObjRef, PdfObject, PdfIndirectObject, PdfContent, PdfToUnicode, PdfEmbeddedCMap, PdfFontProgram, PdfImage};
use std::collections::HashMap;

/// 🧭️ An exact location within an indirect logical object.
#[derive(semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfGraphIdentity {
    pub owner: ObjRef,
    pub path: Vec<PdfGraphPath>,
}

/// 🗺️ Dictionary keys and array ordinals occupy distinct namespaces.
#[derive(semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfGraphPath {
    Entry { key: String },
    Item { index: usize },
}

/// 🧬️ Semantic stream words admitted by native IO.
#[derive(semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfStreamRoleValue {
    Operators { content: PdfContent },
    SampledWords { samples: Vec<u32> },
    CalculatorProgram { code: String },
    UnicodeMap { mapping: PdfToUnicode },
    CharacterMap { cmap: PdfEmbeddedCMap },
    FontProgram { program: PdfFontProgram },
    Image { image: Box<PdfImage> },
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

/// 🧷️ Every role names its owner and the generating graph inputs its admission consumed: its own identity, the inherited font dictionary and every font whose codec it read. The transitive inputs are the closure of these generators in the graph (`GraphSource::closure_of`), derived when a mutation is validated and never stored, so a document of thousands of roles over dozens of fonts does not retain thousands of copies of the same font closures.
#[derive(semio_framework_value::RetireOwned, semio_framework_value::NativeReceive, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PdfAdmittedStreamRole {
    pub identity: PdfGraphIdentity,
    pub dependencies: Vec<PdfGraphIdentity>,
    pub value: PdfStreamRoleValue,
}

/// 🎯️ Resolves an exact generation and structural path without decoding native data.
pub fn resolve_identity<'a>(objects: &'a [PdfIndirectObject], identity: &PdfGraphIdentity) -> Option<&'a PdfObject> {
    let value = &objects.iter().find(|object| object.id == identity.owner)?.value;
    resolve_object_path(value, &identity.path)
}

/// 🧭️ Follows the typed path from an already admitted object owner.
pub fn resolve_object_path<'a>(mut value: &'a PdfObject, path: &[PdfGraphPath]) -> Option<&'a PdfObject> {
    for part in path {
        value = match part {
            PdfGraphPath::Entry { key } => value.dict_get(key)?,
            PdfGraphPath::Item { index } => value.as_array()?.get(*index)?,
        };
    }
    Some(value)
}

/// 🧵️ Whether any input a generator consumes reads differently in `next` than in `base`, memoized per generator: stored dependencies are generators, so thousands of roles share a few dozen of them.
struct InputChanges<'a> { original: GraphSource<'a>, updated: GraphSource<'a>, memo: HashMap<PdfGraphIdentity, (bool, bool)> }

impl<'a> InputChanges<'a> {
    fn new(base: &'a [PdfIndirectObject], next: &'a [PdfIndirectObject]) -> Self { Self { original: GraphSource::new(base), updated: GraphSource::new(next), memo: HashMap::new() } }

    /// 🔍️ `(differs, differs and still resolves in next)` over the closure of `generator` in the base graph.
    fn of(&mut self, generator: &PdfGraphIdentity) -> (bool, bool) {
        if let Some(known) = self.memo.get(generator) { return *known; }
        let mut result = (false, false);
        for input in self.original.dependencies_of(generator) {
            let (before, after) = (self.original.resolve(&input), self.updated.resolve(&input));
            if before != after { result = (true, result.1 || after.is_some()); }
        }
        self.memo.insert(generator.clone(), result);
        result
    }

    /// 🧬️ The role's generators and its own identity.
    fn role(&mut self, role: &PdfAdmittedStreamRole) -> (bool, bool) {
        role.dependencies.iter().chain(std::iter::once(&role.identity)).fold((false, false), |sum, generator| { let one = self.of(generator); (sum.0 || one.0, sum.1 || one.1) })
    }
}

/// 🔐️ A changed admitted input requires the corresponding explicit semantic payload. A role stores the generators of its inputs; the inputs are their closure in the graph.
pub fn validate_role_inputs(base: &[PdfIndirectObject], next: &[PdfIndirectObject], roles: &[PdfAdmittedStreamRole], replacements: &[PdfAdmittedStreamRole]) -> Result<(), String> {
    let mut changes = InputChanges::new(base, next);
    for role in roles {
        if changes.role(role).0 && !replacements.iter().any(|replacement| replacement.identity == role.identity && replacement.value.kind() == role.value.kind()) {
            return Err(format!("known stream role at {} {} requires a semantic replacement", role.identity.owner.num, role.identity.owner.gen));
        }
    }
    let updated = &changes.updated;
    for (index, replacement) in replacements.iter().enumerate() {
        if replacements[..index].iter().any(|current| current.identity == replacement.identity && current.value.kind() == replacement.value.kind()) { return Err("semantic stream role replacement is duplicated".into()); }
        let required = updated.dependencies_of(&replacement.identity);
        let covered = updated.closure_of(&replacement.dependencies);
        if required.iter().any(|input| !covered.contains(input)) { return Err("semantic stream role payload omits a consumed logical dependency".into()); }
        if updated.resolve(&replacement.identity).is_none() || replacement.dependencies.iter().any(|identity| updated.resolve(identity).is_none()) {
            return Err("semantic stream role identity or dependency does not resolve".into());
        }
    }
    Ok(())
}

/// 🔁️ Protects still-present native inputs while permitting explicit logical role deletion.
pub fn validate_role_transition(base: &[PdfIndirectObject], next: &[PdfIndirectObject], roles: &[PdfAdmittedStreamRole], next_roles: &[PdfAdmittedStreamRole], replacements: &[PdfAdmittedStreamRole]) -> Result<(), String> {
    let mut changes = InputChanges::new(base, next);
    let retained:Vec<_>=roles.iter().filter(|role| {
        changes.updated.resolve(&role.identity).is_some()
            || next_roles.iter().any(|candidate|candidate.identity==role.identity&&candidate.value.kind()==role.value.kind())
            || changes.role(role).1
    }).cloned().collect();
    validate_role_inputs(base,next,&retained,replacements)?;
    validate_role_inputs(next,next,&[],next_roles)
}

/// 📄️ Reads an admitted page-content role by its actual owner.
pub fn page_content(roles: &[PdfAdmittedStreamRole], owner: ObjRef) -> Option<&PdfContent> {
    let path = vec![PdfGraphPath::Entry { key: "Contents".into() }];
    roles.iter().find_map(|role| {
        if role.identity.owner != owner || role.identity.path != path { return None; }
        match &role.value { PdfStreamRoleValue::Operators { content } => Some(content), _ => None }
    })
}


