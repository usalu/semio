//! 🧩️ Pure schema composition vocabulary, mounted once by the kernel and reexported by schema.

/// 🧒️ One declared CHILD slot on an artifact snapshot — an owned sub-artifact with its own document
/// and lifecycle (`ArtifactChild<T>` / `Vec<ArtifactChild<T>>` at the field level).
///
/// `kind` is a plain `&'static str` holding a canonical artifact kind id, grammar `s.<plugin>.<artifact>`
/// (e.g. `"s.stdio.mesh"`) — deliberately NOT the `ArtifactKindId` newtype from `🚪️io`'s `semio-framework`
/// crate: this crate (`semio-framework-schema`) must not gain a dependency on `semio-framework` merely to
/// name a kind inside a slot table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChildSlotSpec {
    pub name: &'static str,
    pub kind: &'static str,
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
        for value in self { value.visit_child_field(slot, visitor)?; }
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
