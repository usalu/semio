//! 🧬️ `XmlValidMutation` — the XML 1.0 ✳️valid mutation vocabulary. Handcrafted for THIS subset,
//! derived rule by rule from its own [`check_valid_conformance`], not inherited from `✳️any`.
//!
//! `✳️valid` is not "`✳️any` plus a stamp". W3C XML 1.0 Fifth Edition §2.8 defines a *valid*
//! document as one that has a document type declaration AND satisfies the constraints it expresses,
//! and this subset's own conformance checker turns that into four axes: the DOCTYPE must be present
//! (`stdio.xml.valid.doctype-missing`), its Name must equal the document element's name
//! (`stdio.xml.valid.root-name-mismatch`), `standalone="yes"` beside an external subset reference is
//! suspicious per §2.9 (`stdio.xml.valid.standalone-external-subset`), and full DTD content-model
//! validation is out of scope from this data because the schema models only §4.2 `<!ENTITY>`
//! declarations of the internal subset (`stdio.xml.valid.validity-not-fully-verified`).
//!
//! `✳️any`'s vocabulary can leave all three checkable axes in one step and has no notion that it
//! did: `XmlMutation::SetDoctype { doctype: None }` deletes the declaration outright,
//! `SetDoctype { doctype: Some(other_name) }` desynchronises §2.8, and nothing in `✳️any` can rename
//! the document element at all. So every authoring mutation here is subset-closed — it either
//! preserves what §2.8 requires or is rejected with a real diagnostic:
//!
//! * [`XmlValidMutation::DeclareDoctype`] takes NO name. The Name is derived from the actual
//!   document element, so §2.8 holds by construction rather than by the caller's care.
//! * [`XmlValidMutation::RenameDocumentElement`] retags the DOCTYPE Name in the SAME step, which is
//!   the only way to rename the root of a valid document without passing through an invalid one.
//! * [`XmlValidMutation::SetExternalSubset`] and [`XmlValidMutation::SetStandalone`] are the two
//!   halves of the §2.9 axis, addressable independently because that is how the diagnostic reads
//!   them.
//! * [`XmlValidMutation::DeclareEntity`] and [`XmlValidMutation::SetInternalSubset`] edit the
//!   internal subset §4.2 declares, positionally — the first declaration of a name binds, so WHERE
//!   a declaration sits is semantic and `DeclareEntity` takes an index rather than appending.
//! * [`XmlValidMutation::SetSnapshot`] is gated: a whole-document replacement that carries a hard
//!   violation is refused, where `✳️any`'s is ungated by design.
//!
//! ⚠️ There is deliberately no `undeclare-doctype`: every application of it would make the document
//! hard-invalid, so the kind would exist only to be rejected. Removing the DOCTYPE is `✳️any`'s
//! operation, reached by migrating the dialect down, not a `✳️valid` edit.
//!
//! ⚠️ [`XmlValidMutation::SetText`] carries no entity gate, and the reason is a property of the
//! shared schema rather than an omission: `XmlNode::Text` holds LITERAL character data
//! (`📸️snapshot/🦀️component.rs`'s `xml_unescape_text` resolves the five predefined entities and
//! numeric character references on read and rejects any other `&name;` outright), so a general
//! entity reference cannot survive into the model and §4.1's *Entity Declared* validity constraint
//! has nothing in this schema to bite on. Stated here rather than silently skipped.
//!
//! `blocked_snapshot_violation` below is held against the subset's own checker by
//! `gate_agrees_with_the_subset_conformance_checker`, so the gate and the diagnostic cannot drift.
//!
//! Leaf-per-variant shape mirrored from `🖼️tiff`'s `TiffBaselineMutation` (ticket
//! `26/08/29/S-END-TO-END`): `NoMutation` was dropped (`#[derive(dsl::Mutations)]` requires every
//! variant to wrap exactly one leaf payload and a unit variant wraps none; `no` is not an approved
//! semantic verb either), and every remaining variant now wraps its own `dsl::MutationLeaf` struct
//! instead of carrying its fields as a struct-literal directly. `#[serde(tag = "mutation", ...)]`
//! is kept so the wire shape this artifact's committed fixtures and the `OpText`/`OpBinary` codec
//! `impl_serde_op_codec!` synthesizes stay byte-for-byte identical — serde's internally-tagged
//! representation supports a newtype variant wrapping a plain struct.
//!
//! @see ../../🧪️oracle/🔣️.json — every entry of `KINDS` below must appear in its `kinds` catalog
//! (the manifest also carries `no-mutation`, the identity-probe row `mutate-xml-1-0-valid`'s own
//! test adapter registers directly; it names no `XmlValidMutation` variant of its own).
//! @see ../../../../../../🧪️tests/mutate-xml-1-0-valid/🥒️.feature — the case that exercises it.

use crate::artifacts::xml::schema::diff::{diff_at_path, diff_set_snapshot, XmlDiff, XmlElementDiff, XmlNodeDiff};
use crate::artifacts::xml::schema::snapshot::{XmlDeclaration, XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlNode};
use crate::artifacts::xml::standards::v1_0::subsets::base::schema::mutations::XmlNodePath;
use crate::artifacts::xml::standards::v1_0::subsets::valid::schema::check_valid_conformance;
use crate::artifacts::xml::XmlSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.xml` 1.0/✳️valid. The snapshot type is the `✳️any` subset's
/// own `XmlSnapshot` verbatim; only the vocabulary is this subset's.
//#region 🔖️Leaves
#[path = "📄set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "📜declare-doctype/🦀️.rs"]
pub mod declare_doctype;
#[path = "🌳rename-document-element/🦀️.rs"]
pub mod rename_document_element;
#[path = "🔗set-external-subset/🦀️.rs"]
pub mod set_external_subset;
#[path = "🏳set-standalone/🦀️.rs"]
pub mod set_standalone;
#[path = "🏷declare-entity/🦀️.rs"]
pub mod declare_entity;
#[path = "📚set-internal-subset/🦀️.rs"]
pub mod set_internal_subset;
#[path = "✍set-text/🦀️.rs"]
pub mod set_text;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[mutations(snapshot = XmlSnapshot, diff = XmlDiff, schema = "XmlValidMutation")]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum XmlValidMutation {
    /// 🔁 Replaces the whole document, REJECTED when the replacement carries a hard §2.8 violation.
    SetSnapshot(set_snapshot::SetSnapshot),
    /// 📜️ Installs (or replaces) the document type declaration, its Name taken from the actual
    /// document element so §2.8 cannot be violated. REJECTED when there is no document element to
    /// take a Name from. Existing internal `<!ENTITY>` declarations are carried across.
    DeclareDoctype(declare_doctype::DeclareDoctype),
    /// 🌳 Renames the document element AND retags the DOCTYPE Name to match, in one step. REJECTED
    /// when there is no document element, or no DOCTYPE to keep in step with it.
    RenameDocumentElement(rename_document_element::RenameDocumentElement),
    /// 🔗️ Sets (or, with `None`, clears) the DOCTYPE's SYSTEM/PUBLIC external subset reference —
    /// the half of §2.9 the doctype owns. REJECTED when there is no DOCTYPE to attach it to.
    SetExternalSubset(set_external_subset::SetExternalSubset),
    /// 🏳️ Sets (or, with `None`, clears) the XML declaration's `standalone` pseudo-attribute — the
    /// half of §2.9 the declaration owns. A document with no declaration at all gains a
    /// `version="1.0"` one; clearing `standalone` on a document that had no declaration removes it
    /// again, so the operation is its own exact inverse in every combination.
    SetStandalone(set_standalone::SetStandalone),
    /// 🏷️ Declares a §4.2 general or parameter entity at `index` of the internal subset. REJECTED
    /// when there is no DOCTYPE, and when a declaration of that name already exists — the FIRST
    /// declaration binds, so a second one is dead markup rather than an edit.
    DeclareEntity(declare_entity::DeclareEntity),
    /// 📚️ Replaces the internal subset's declaration list wholesale — the operation that can also
    /// EMPTY it, which is why the vocabulary carries no `undeclare-entity`: a per-name removal is
    /// only ever applicable to a document that already declares that name, and there is no such
    /// document to hold this subset's vocabulary against. REJECTED when there is no DOCTYPE.
    SetInternalSubset(set_internal_subset::SetInternalSubset),
    /// ✍️ Replaces the literal character data of the `Text` node at `path`.
    SetText(set_text::SetText),
}

/// 📇️ Kebab-case spelling of every `XmlValidMutation` variant, in declaration order — the exact
/// `kinds` list `../../🧪️oracle/🔣️.json`'s `mutationCatalogs` entry declares. The framework
/// never parses this enum; `kinds_matches_enum_variants_in_declaration_order` below is what keeps
/// the two declarations honest against each other.
pub const KINDS: &[&str] = &["set-snapshot", "declare-doctype", "rename-document-element", "set-external-subset", "set-standalone", "declare-entity", "set-internal-subset", "set-text"];

crate::impl_serde_op_codec!(XmlValidMutation, "xml-valid-mutation");

/// 🏷️ The `KINDS` spelling of one mutation's own variant. An exhaustive match (no wildcard arm), so
/// a new variant that forgets its kebab spelling fails to compile rather than failing silently.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn kind_of(mutation: &XmlValidMutation) -> &'static str {
    match mutation {
        XmlValidMutation::SetSnapshot(_) => "set-snapshot",
        XmlValidMutation::DeclareDoctype(_) => "declare-doctype",
        XmlValidMutation::RenameDocumentElement(_) => "rename-document-element",
        XmlValidMutation::SetExternalSubset(_) => "set-external-subset",
        XmlValidMutation::SetStandalone(_) => "set-standalone",
        XmlValidMutation::DeclareEntity(_) => "declare-entity",
        XmlValidMutation::SetInternalSubset(_) => "set-internal-subset",
        XmlValidMutation::SetText(_) => "set-text",
    }
}
//#endregion 🔖️Mutations

//#region 🔖️Gate
/// 🚫 The fault code every rejected `✳️valid` mutation reports under.
pub const CODE_REJECTED: &str = "stdio.xml.valid.mutation-outside-subset";

/// 🌳️ The document element's tag name, when the document has one at all.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn document_element_name(snapshot: &XmlSnapshot) -> Option<&str> {
    match &snapshot.doc.root {
        Some(XmlNode::Element { name, .. }) => Some(name.as_str()),
        _ => None,
    }
}

/// 🛡️ The gate `SetSnapshot` passes through: the message naming the first HARD `✳️valid` violation
/// the candidate document carries, or `None` when it is one this subset may hold. Soft diagnostics
/// (§2.9's suspicion, the always-on "validity not fully verified" advisory) are not violations and
/// never block — the same Error/Fatal split `XmlValidBuilder::build` itself gates on.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn blocked_snapshot_violation(candidate: &XmlSnapshot) -> Option<String> {
    check_valid_conformance(candidate).into_iter().find(|d| matches!(d.severity, dsl::Severity::Error | dsl::Severity::Fatal)).map(|d| format!("{} — {}", d.code.0, d.message))
}

/// 🔎️ The position of the internal-subset entity declaration named `name`, if it is declared.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_index(doctype: &XmlDoctype, name: &str) -> Option<usize> {
    doctype.declarations.iter().position(|declaration| matches!(declaration, XmlDtdDeclaration::Entity { name: declared, .. } if declared == name))
}
//#endregion 🔖️Gate

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: the diff is the single semantics source, never a separate
/// imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_xml_valid_mutation(snapshot: &mut XmlSnapshot, mutation: &XmlValidMutation) -> protocol::MutationOutcome<XmlDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

/// ↩️ This subset's own inverse algebra as a free function, so a caller that legitimately drives the
/// vocabulary from outside the crate — an owner-root test adapter, for one — can reach it without
/// naming the `protocol::Mutation` trait, which it has no reason to link.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_xml_valid_mutation(mutation: &XmlValidMutation, base: &XmlSnapshot) -> Vec<XmlValidMutation> {
    Mutation::inverse(mutation, base)
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn doctype_diff(doctype: XmlDoctype) -> XmlDiff {
    XmlDiff { prolog: None, declaration: None, doctype: Some(Some(doctype)), root: None }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rejected(message: String) -> protocol::MutationOutcome<XmlDiff> {
    protocol::MutationOutcome::error(CODE_REJECTED, message, Vec::<String>::new())
}

/// 🧷️ Lifted verbatim from the former `impl Mutation<XmlSnapshot> for XmlValidMutation`'s own
/// `diff` body — only each match arm's pattern head changed, from `XmlValidMutation::Variant { .. }`
/// to `XmlValidMutation::Variant(variant_mod::Variant { .. })`, to destructure the leaf payload each
/// variant now wraps.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn agg_diff(this: &XmlValidMutation, base: &XmlSnapshot) -> protocol::MutationOutcome<XmlDiff> {
    match this {
        XmlValidMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => match blocked_snapshot_violation(snapshot) {
            Some(message) => rejected(format!("set-snapshot: the replacement document is not XML 1.0 valid — {message}")),
            None => protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot)),
        },
        XmlValidMutation::DeclareDoctype(declare_doctype::DeclareDoctype { external_id }) => match document_element_name(base) {
            None => rejected("declare-doctype: the document has no document element, so §2.8 gives the DOCTYPE no Name to carry".to_string()),
            Some(name) => protocol::MutationOutcome::new(doctype_diff(XmlDoctype { name: name.to_string(), external_id: external_id.clone(), declarations: base.doc.doctype.as_ref().map(|d| d.declarations.clone()).unwrap_or_default() })),
        },
        XmlValidMutation::RenameDocumentElement(rename_document_element::RenameDocumentElement { name }) => match (document_element_name(base), base.doc.doctype.as_ref()) {
            (None, _) => rejected("rename-document-element: the document has no document element to rename".to_string()),
            (Some(_), None) => rejected("rename-document-element: the document has no DOCTYPE to keep in step with the new name — declare one first".to_string()),
            (Some(_), Some(doctype)) => {
                let mut diff = diff_at_path(&[], XmlNodeDiff::Element(XmlElementDiff { name: Some(name.clone()), attributes: None, children: None }));
                diff.doctype = Some(Some(XmlDoctype { name: name.clone(), external_id: doctype.external_id.clone(), declarations: doctype.declarations.clone() }));
                protocol::MutationOutcome::new(diff)
            }
        },
        XmlValidMutation::SetExternalSubset(set_external_subset::SetExternalSubset { external_id }) => match base.doc.doctype.as_ref() {
            None => rejected("set-external-subset: the document has no DOCTYPE to attach an external subset reference to".to_string()),
            Some(doctype) => protocol::MutationOutcome::new(doctype_diff(XmlDoctype { name: doctype.name.clone(), external_id: external_id.clone(), declarations: doctype.declarations.clone() })),
        },
        XmlValidMutation::SetStandalone(set_standalone::SetStandalone { standalone }) => {
            let next = match (&base.doc.declaration, standalone) {
                (None, None) => None,
                (None, Some(value)) => Some(XmlDeclaration { version: "1.0".to_string(), encoding: None, standalone: Some(*value) }),
                (Some(declaration), value) => Some(XmlDeclaration { version: declaration.version.clone(), encoding: declaration.encoding.clone(), standalone: *value }),
            };
            protocol::MutationOutcome::new(XmlDiff { prolog: None, declaration: Some(next), doctype: None, root: None })
        }
        XmlValidMutation::DeclareEntity(declare_entity::DeclareEntity { index, parameter, name, value }) => match base.doc.doctype.as_ref() {
            None => rejected("declare-entity: the document has no DOCTYPE, so there is no internal subset to declare an entity in".to_string()),
            Some(doctype) if entity_index(doctype, name).is_some() => rejected(format!("declare-entity: '{name}' is already declared — XML 1.0 §4.2 binds the FIRST declaration, so a second one is dead markup rather than an edit")),
            Some(doctype) => {
                let mut declarations = doctype.declarations.clone();
                let at = (*index).min(declarations.len());
                declarations.insert(at, XmlDtdDeclaration::Entity { parameter: *parameter, name: name.clone(), value: value.clone() });
                protocol::MutationOutcome::new(doctype_diff(XmlDoctype { name: doctype.name.clone(), external_id: doctype.external_id.clone(), declarations }))
            }
        },
        XmlValidMutation::SetInternalSubset(set_internal_subset::SetInternalSubset { declarations }) => match base.doc.doctype.as_ref() {
            None => rejected("set-internal-subset: the document has no DOCTYPE, so there is no internal subset to replace".to_string()),
            Some(doctype) => protocol::MutationOutcome::new(doctype_diff(XmlDoctype { name: doctype.name.clone(), external_id: doctype.external_id.clone(), declarations: declarations.clone() })),
        },
        XmlValidMutation::SetText(set_text::SetText { path, text }) => protocol::MutationOutcome::new(diff_at_path(&path.0, XmlNodeDiff::Text { text: Some(text.clone()) })),
    }
}

/// ↩️ Lifted verbatim from the former `impl Mutation<XmlSnapshot> for XmlValidMutation`'s own
/// `inverse` body. Every arm restores the exact prior state rather than an approximation of it.
/// `DeclareDoctype` and `SetExternalSubset` need only the prior external id back, because the Name
/// is re-derived from the (untouched) document element and the internal subset is carried across
/// unchanged; both internal-subset kinds invert to the prior declaration LIST, because §4.2's
/// first-declaration-binds rule makes position semantic and a name-keyed undo would not restore it.
/// A mutation with nothing to invert against inverts to the EMPTY vec (`NoMutation`, dropped by this
/// migration, used to carry this case as a no-op sentinel; there is nothing to undo, so there is
/// nothing to return).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn agg_inverse(this: &XmlValidMutation, base: &XmlSnapshot) -> Vec<XmlValidMutation> {
    match this {
        XmlValidMutation::SetSnapshot(_) => vec![XmlValidMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        XmlValidMutation::DeclareDoctype(_) => match base.doc.doctype.as_ref() {
            Some(doctype) => vec![XmlValidMutation::DeclareDoctype(declare_doctype::DeclareDoctype { external_id: doctype.external_id.clone() })],
            None => vec![XmlValidMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        },
        XmlValidMutation::RenameDocumentElement(_) => match document_element_name(base) {
            Some(name) => vec![XmlValidMutation::RenameDocumentElement(rename_document_element::RenameDocumentElement { name: name.to_string() })],
            None => Vec::new(),
        },
        XmlValidMutation::SetExternalSubset(_) => vec![XmlValidMutation::SetExternalSubset(set_external_subset::SetExternalSubset { external_id: base.doc.doctype.as_ref().and_then(|doctype| doctype.external_id.clone()) })],
        XmlValidMutation::SetStandalone(_) => vec![XmlValidMutation::SetStandalone(set_standalone::SetStandalone { standalone: base.doc.declaration.as_ref().and_then(|declaration| declaration.standalone) })],
        XmlValidMutation::DeclareEntity(_) | XmlValidMutation::SetInternalSubset(_) => match base.doc.doctype.as_ref() {
            Some(doctype) => vec![XmlValidMutation::SetInternalSubset(set_internal_subset::SetInternalSubset { declarations: doctype.declarations.clone() })],
            None => Vec::new(),
        },
        XmlValidMutation::SetText(set_text::SetText { path, .. }) => {
            let prior = path
                .resolve(base.doc.root.as_ref())
                .and_then(|node| match node {
                    XmlNode::Text { text } => Some(text.clone()),
                    _ => None,
                })
                .unwrap_or_default();
            vec![XmlValidMutation::SetText(set_text::SetText { path: path.clone(), text: prior })]
        }
    }
}
//#endregion 🔖️MutationTrait

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::xml::standards::v1_0::subsets::valid::schema::{CODE_DOCTYPE_MISSING, CODE_ROOT_NAME_MISMATCH};

    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    fn valid_document() -> XmlSnapshot {
        <XmlSnapshot as store::ArtifactDsl>::parse_dsl("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\"><key>Name</key></plist>").expect("the fixture document parses")
    }

    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    fn applied(base: &XmlSnapshot, mutation: &XmlValidMutation) -> (XmlSnapshot, protocol::MutationOutcome<XmlDiff>) {
        let mut next = base.clone();
        let outcome = apply_xml_valid_mutation(&mut next, mutation);
        (next, outcome)
    }

    #[test]
    fn kinds_matches_enum_variants_in_declaration_order() {
        let one_per_variant = vec![
            XmlValidMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: XmlSnapshot::default() }),
            XmlValidMutation::DeclareDoctype(declare_doctype::DeclareDoctype { external_id: None }),
            XmlValidMutation::RenameDocumentElement(rename_document_element::RenameDocumentElement { name: "x".into() }),
            XmlValidMutation::SetExternalSubset(set_external_subset::SetExternalSubset { external_id: None }),
            XmlValidMutation::SetStandalone(set_standalone::SetStandalone { standalone: None }),
            XmlValidMutation::DeclareEntity(declare_entity::DeclareEntity { index: 0, parameter: false, name: "e".into(), value: "v".into() }),
            XmlValidMutation::SetInternalSubset(set_internal_subset::SetInternalSubset { declarations: Vec::new() }),
            XmlValidMutation::SetText(set_text::SetText { path: XmlNodePath(vec![0]), text: "t".into() }),
        ];
        assert_eq!(one_per_variant.len(), KINDS.len(), "one_per_variant must cover every KINDS entry exactly once");
        for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
            assert_eq!(kind_of(mutation), *kind, "KINDS order must match the enum's own declaration order for {mutation:?}");
        }
    }

    #[test]
    fn gate_agrees_with_the_subset_conformance_checker() {
        let no_doctype = <XmlSnapshot as store::ArtifactDsl>::parse_dsl("<plist/>").expect("parses");
        assert!(blocked_snapshot_violation(&no_doctype).expect("a document with no DOCTYPE is hard-invalid").contains(CODE_DOCTYPE_MISSING));
        let mismatched = <XmlSnapshot as store::ArtifactDsl>::parse_dsl("<!DOCTYPE book>\n<plist/>").expect("parses");
        assert!(blocked_snapshot_violation(&mismatched).expect("a desynchronised DOCTYPE Name is hard-invalid").contains(CODE_ROOT_NAME_MISMATCH));
        assert_eq!(blocked_snapshot_violation(&valid_document()), None, "the always-on advisory is a Warning and must never block");
    }

    #[test]
    fn set_snapshot_refuses_a_replacement_that_is_not_valid() {
        let base = valid_document();
        let (next, outcome) = applied(&base, &XmlValidMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: <XmlSnapshot as store::ArtifactDsl>::parse_dsl("<plist/>").expect("parses") }));
        assert!(outcome.messages().iter().any(|message| message.code.0 == CODE_REJECTED), "got {:?}", outcome.messages());
        assert_eq!(next, base, "a rejected mutation must leave the document untouched");
    }

    #[test]
    fn declare_doctype_derives_the_name_from_the_document_element() {
        let base = <XmlSnapshot as store::ArtifactDsl>::parse_dsl("<requests version=\"1.0\"/>").expect("parses");
        let (next, _) = applied(&base, &XmlValidMutation::DeclareDoctype(declare_doctype::DeclareDoctype { external_id: Some(XmlExternalId::System { system_id: "logreq.dtd".into() }) }));
        assert_eq!(next.doc.doctype.as_ref().map(|doctype| doctype.name.as_str()), Some("requests"), "§2.8 must hold by construction, not by the caller's care");
        assert_eq!(blocked_snapshot_violation(&next), None);
    }

    #[test]
    fn rename_document_element_retags_the_doctype_in_the_same_step() {
        let base = valid_document();
        let (next, _) = applied(&base, &XmlValidMutation::RenameDocumentElement(rename_document_element::RenameDocumentElement { name: "propertyList".into() }));
        assert_eq!(document_element_name(&next), Some("propertyList"));
        assert_eq!(next.doc.doctype.as_ref().map(|doctype| doctype.name.as_str()), Some("propertyList"));
        assert_eq!(blocked_snapshot_violation(&next), None, "a rename that passes through an invalid state is exactly what this subset exists to prevent");
    }

    #[test]
    fn declare_entity_refuses_a_duplicate_name() {
        let base = valid_document();
        let (with_entity, _) = applied(&base, &XmlValidMutation::DeclareEntity(declare_entity::DeclareEntity { index: 0, parameter: false, name: "semio".into(), value: "Semio".into() }));
        let (_, duplicate) = applied(&with_entity, &XmlValidMutation::DeclareEntity(declare_entity::DeclareEntity { index: 0, parameter: false, name: "semio".into(), value: "Other".into() }));
        assert!(duplicate.messages().iter().any(|message| message.code.0 == CODE_REJECTED), "§4.2 binds the first declaration, so a second one is dead markup");
        let (_, no_doctype) = applied(&<XmlSnapshot as store::ArtifactDsl>::parse_dsl("<plist/>").expect("parses"), &XmlValidMutation::SetInternalSubset(set_internal_subset::SetInternalSubset { declarations: Vec::new() }));
        assert!(no_doctype.messages().iter().any(|message| message.code.0 == CODE_REJECTED), "there is no internal subset without a DOCTYPE to hold it");
    }

    #[test]
    fn declare_entity_inserts_at_the_declared_index_and_inverts_to_the_prior_list() {
        let base = valid_document();
        let seeded = XmlValidMutation::SetInternalSubset(set_internal_subset::SetInternalSubset {
            declarations: vec![XmlDtdDeclaration::Entity { parameter: false, name: "first".into(), value: "1".into() }, XmlDtdDeclaration::Entity { parameter: false, name: "third".into(), value: "3".into() }],
        });
        let (with_two, _) = applied(&base, &seeded);
        let insertion = XmlValidMutation::DeclareEntity(declare_entity::DeclareEntity { index: 1, parameter: false, name: "second".into(), value: "2".into() });
        let (mut with_three, _) = applied(&with_two, &insertion);
        let names: Vec<&str> = with_three
            .doc
            .doctype
            .as_ref()
            .expect("doctype")
            .declarations
            .iter()
            .map(|declaration| match declaration {
                XmlDtdDeclaration::Entity { name, .. } => name.as_str(),
            })
            .collect();
        assert_eq!(names, vec!["first", "second", "third"], "position is semantic under §4.2");
        for step in inverse_xml_valid_mutation(&insertion, &with_two) {
            let (undone, _) = applied(&with_three, &step);
            with_three = undone;
        }
        assert_eq!(with_three, with_two, "the inverse must restore the prior declaration LIST, order included");
    }

    #[test]
    fn every_kind_round_trips_through_its_own_inverse() {
        let base = valid_document();
        let cases = vec![
            XmlValidMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: <XmlSnapshot as store::ArtifactDsl>::parse_dsl("<!DOCTYPE root>\n<root><child>text</child></root>").expect("parses") }),
            XmlValidMutation::DeclareDoctype(declare_doctype::DeclareDoctype { external_id: Some(XmlExternalId::System { system_id: "plist.dtd".into() }) }),
            XmlValidMutation::RenameDocumentElement(rename_document_element::RenameDocumentElement { name: "propertyList".into() }),
            XmlValidMutation::SetExternalSubset(set_external_subset::SetExternalSubset { external_id: None }),
            XmlValidMutation::SetStandalone(set_standalone::SetStandalone { standalone: Some(true) }),
            XmlValidMutation::DeclareEntity(declare_entity::DeclareEntity { index: 0, parameter: false, name: "semio".into(), value: "Semio".into() }),
            XmlValidMutation::SetInternalSubset(set_internal_subset::SetInternalSubset { declarations: vec![XmlDtdDeclaration::Entity { parameter: true, name: "shared".into(), value: "<!ELEMENT dummy EMPTY>".into() }] }),
            XmlValidMutation::SetText(set_text::SetText { path: XmlNodePath(vec![0, 0]), text: "Renamed".into() }),
        ];
        for mutation in cases {
            let (mut next, outcome) = applied(&base, &mutation);
            assert!(!outcome.messages().iter().any(|message| message.code.0 == CODE_REJECTED), "{mutation:?} must apply against the fixture: {:?}", outcome.messages());
            for step in inverse_xml_valid_mutation(&mutation, &base) {
                let (undone, _) = applied(&next, &step);
                next = undone;
            }
            assert_eq!(next, base, "applying {mutation:?} and then its own inverse must land back on the original document");
        }
    }

    #[test]
    fn set_standalone_is_exact_in_every_declaration_combination() {
        for source in ["<!DOCTYPE root>\n<root/>", "<?xml version=\"1.0\"?>\n<!DOCTYPE root>\n<root/>", "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<!DOCTYPE root>\n<root/>"] {
            let base = <XmlSnapshot as store::ArtifactDsl>::parse_dsl(source).expect("parses");
            for target in [None, Some(true), Some(false)] {
                let mutation = XmlValidMutation::SetStandalone(set_standalone::SetStandalone { standalone: target });
                let (mut next, _) = applied(&base, &mutation);
                for step in inverse_xml_valid_mutation(&mutation, &base) {
                    let (undone, _) = applied(&next, &step);
                    next = undone;
                }
                assert_eq!(next, base, "set-standalone({target:?}) on {source:?} must invert exactly");
            }
        }
    }
}
//#endregion 🧪️Tests
