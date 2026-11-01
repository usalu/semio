//! 📖️ Immutable ADD2 view-scope rows retained from the official listings, decoded once from the committed IFCJ resource `mvd.ifcj` whose neutral form is `../🔣️.json`.
mod pack;
pub use pack::{decode_mvd, embedded_mvd};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IfcMvdEntry {
    pub id: &'static str,
    pub name: &'static str,
    pub href: &'static str,
    pub universal: bool,
    pub reference: Option<&'static [&'static str]>,
    pub alignment: Option<&'static [&'static str]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IfcMvdSource {
    pub url: &'static str,
    pub sha256: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IfcMvdCatalog {
    pub schema: &'static str,
    pub source_commit: &'static str,
    pub sources: &'static [IfcMvdSource],
    pub conformance_evaluated: bool,
    pub entities: &'static [IfcMvdEntry],
    pub concepts: &'static [IfcMvdEntry],
}

