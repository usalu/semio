//! 🧬️ PptxSnapshot schema (ecma-376/🔒️strict) — reuses the 🧱️base subset's `PptxSnapshot` verbatim
//! (the SAME Rust type, same `s.stdio.pptx` schema id). ISO/IEC 29500-1:2016 Strict is a
//! validation-gated dialect STAMP on top of that existing schema, not a new one -- see D4's
//! Tier-1 "same snapshot type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This
//! leaf exists so `🪆️subsets/🔒️strict/🧬️schema/` is present per `🔣️taxonomy.json`'s
//! `subsetChildDirs`, without duplicating the schema definition.
//!
//! Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES: real ISO/IEC 29500-1 Strict
//! conformance-class subset, same shared pattern as `📜️docx`/`📕️xlsx` ecma-376 🔒️strict.

pub use crate::standards::v_ecma_376::subsets::base::schema::*;

/// 🏛️ Authors the Strict presentation, master, layout, theme and their complete OPC relationships.
pub fn blank_strict_pptx_snapshot() -> crate::PptxSnapshot {
    use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr,XmlNode};
    let mut snapshot = crate::standards::v_ecma_376::subsets::base::schema::construction::minimal::build_minimal_pptx(Default::default());
    for part in &mut snapshot.xml_parts {
        let mut nodes = part.document.root.iter_mut().collect::<Vec<_>>();
        while let Some(node) = nodes.pop() {
            if let XmlNode::Element { attrs, children, .. } = node {
                for attr in attrs {
                    for (transitional,strict) in [
                        ("http://schemas.openxmlformats.org/presentationml/2006/main","http://purl.oclc.org/ooxml/presentationml/main"),
                        ("http://schemas.openxmlformats.org/drawingml/2006/main","http://purl.oclc.org/ooxml/drawingml/main"),
                        ("http://schemas.openxmlformats.org/officeDocument/2006/relationships","http://purl.oclc.org/ooxml/officeDocument/relationships"),
                    ] { if attr.value == transitional { attr.value = strict.into(); } }
                }
                nodes.extend(children.iter_mut());
            }
        }
        if part.path == "ppt/presentation.xml" {
            if let Some(XmlNode::Element { attrs, .. }) = &mut part.document.root {
                attrs.push(XmlAttr { name: "conformance".into(), value: "strict".into() });
            }
        }
    }
    for (_,relationships) in snapshot.opc.relationships.groups_mut() {
        for relationship in relationships {
            if let Some(kind) = relationship.rel_type.strip_prefix("http://schemas.openxmlformats.org/officeDocument/2006/relationships") {
                relationship.rel_type = format!("http://purl.oclc.org/ooxml/officeDocument/relationships{kind}");
            }
        }
    }
    snapshot
}

//#region 🧬️Mutations
// 🧬️ This subset's OWN conformance-class vocabulary, mounted here rather than in the crate's shared
// `🦀️.rs`: that file is one wiring file for every stdio artifact at once, and the rationale the
// 🧱️base subset already records for its own test mount — leave the shared file alone, let an artifact
// own the subtree it owns — applies to a production leaf of this subset just as well. `#[path]` on a
// non-inline module resolves against this file's own directory. The explicit declaration shadows the
// glob re-export of 🧱️base's `mutations` above, which is what puts this subset's own vocabulary at
// `subsets::strict::schema::mutations` while 🧱️base's document vocabulary stays reachable at its own
// address.
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
//#endregion 🧬️Mutations

//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

#[path = "💡️inferences/🛡️conformance/🦀️.rs"]
pub mod conformance;
