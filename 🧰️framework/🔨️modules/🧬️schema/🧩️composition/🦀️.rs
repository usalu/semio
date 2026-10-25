//! 🧩️ Canonical allocation-free child and link schema composition vocabulary.

/// 🎯️ Explicit serialization-kind or consumer-capability selection for an owned child slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildSlotSelection {
    ArtifactKind(&'static str),
    Capability(&'static str),
}

impl ChildSlotSelection {
    /// 📝️ Borrows the authored kind or capability identifier.
    pub const fn id(self) -> &'static str { match self { Self::ArtifactKind(id) | Self::Capability(id) => id } }
    /// 🔎️ Checks a concrete-kind constraint; a capability is admitted by its runtime composition.
    pub fn accepts_kind(self, kind: &str) -> bool { match self { Self::ArtifactKind(id) => id == kind, Self::Capability(_) => true } }
    /// 🗂️ Names serialization only when the slot declares a concrete serialization kind.
    pub const fn artifact_kind(self) -> Option<&'static str> { match self { Self::ArtifactKind(id) => Some(id), Self::Capability(_) => None } }
}

/// 🔡️ Consumer capability identifiers contain two or more canonical lowercase dotted segments.
pub fn is_canonical_child_capability(id: &str) -> bool {
    let mut count = 0;
    for segment in id.split('.') {
        if segment.is_empty() || segment.starts_with('-') || segment.ends_with('-') || segment.contains("--") || !segment.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-') { return false; }
        count += 1;
    }
    count >= 2
}

/// 🧒️ An explicitly selected owned sub-artifact with its own document and lifecycle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChildSlotSpec {
    pub name: &'static str,
    pub selection: ChildSlotSelection,
    pub many: bool,
}

/// 🪪️ Allocation-free child identity borrowed from the loaded parent value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChildRefFields<'a> {
    pub child_id: &'a str,
    pub artifact_id: &'a str,
    pub artifact_kind: &'a str,
    pub standard: &'a str,
    pub subset: &'a str,
}

/// 🚦️ Charges every field/container step before visiting a borrowed child identity.
pub trait ChildRefVisitor<'a> {
    type Error;
    fn step(&mut self) -> Result<(), Self::Error>;
    fn child(&mut self, slot: &'static str, fields: ChildRefFields<'a>) -> Result<(), Self::Error>;
}

/// 🧸️ Typed child-field projection shared by aliases and nested optional/collection fields.
pub trait ChildFieldRefs {
    const MANY: bool;
    fn visit_child_field<'a, V: ChildRefVisitor<'a>>(&'a self, slot: &'static str, visitor: &mut V) -> Result<(), V::Error>;
}

impl<T: ChildFieldRefs> ChildFieldRefs for Option<T> {
    const MANY: bool = T::MANY;
    fn visit_child_field<'a, V: ChildRefVisitor<'a>>(&'a self, slot: &'static str, visitor: &mut V) -> Result<(), V::Error> {
        visitor.step()?;
        match self {
            Some(value) => value.visit_child_field(slot, visitor),
            None => Ok(()),
        }
    }
}

impl<T: ChildFieldRefs> ChildFieldRefs for Vec<T> {
    const MANY: bool = true;
    fn visit_child_field<'a, V: ChildRefVisitor<'a>>(&'a self, slot: &'static str, visitor: &mut V) -> Result<(), V::Error> {
        visitor.step()?;
        for value in self {
            value.visit_child_field(slot, visitor)?;
        }
        Ok(())
    }
}

/// 🔗 One declared LINK slot on an artifact snapshot — a reference to an independent artifact, never
/// owned (`ArtifactLink` / `Vec<ArtifactLink>` at the field level).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkSlotSpec {
    pub name: &'static str,
    pub roles: &'static [&'static str],
    pub many: bool,
}

/// ✨️ Static composition declarations and bounded typed value projection.
pub trait ArtifactCompositionFields {
    fn visit_child_refs<'a, V: ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error>;
    fn child_slots() -> &'static [ChildSlotSpec] {
        &[]
    }
    fn link_slots() -> &'static [LinkSlotSpec] {
        &[]
    }
}

#[path = "👁️read/🦀️.rs"]
mod child_read;
pub use child_read::{ArtifactChildReadAdmission,ChildFieldReadAdmission,ChildReadSource};
