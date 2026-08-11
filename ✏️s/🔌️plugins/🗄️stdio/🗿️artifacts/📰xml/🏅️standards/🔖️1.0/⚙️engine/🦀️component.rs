//! ⚙️ XmlEngine — owns a real `XmlArtifact`.

use crate::artifacts::xml::{XmlArtifact, XmlDiff, XmlMutation, XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_xml_snapshot() -> XmlSnapshot {
    XmlSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::xml::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<XmlSnapshot, XmlMutation>(STDIO_XML_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.xml",
        extension: Some("xml"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::xml::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::xml::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::xml::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::xml::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.xml"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.xml`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::xml::schema::xml_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.xml` artifact engine.
pub struct XmlEngine {
    artifact_state: XmlArtifact,
    snapshot_state: XmlSnapshot,
}

impl XmlEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: XmlSnapshot) -> Self {
        let artifact_state = XmlArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::xml::schema::diff::{XmlChildAdded, XmlNodeDiff};
    use crate::artifacts::xml::schema::mutations::XmlNodePath;
    use crate::artifacts::xml::schema::snapshot::{XmlAttr, XmlDeclaration, XmlDocument, XmlNode};
    use protocol::command::DiffAlgebra;
    use protocol::{Mutation, MutationDiff};

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_xml_snapshot();
        assert_eq!(snapshot.schema, STDIO_XML_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_xml_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <XmlSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <XmlSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️Fixtures
    fn sample_snapshot() -> XmlSnapshot {
        XmlSnapshot {
            schema: STDIO_XML_DOCUMENT_SCHEMA.into(),
            doc: XmlDocument {
                declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: None, standalone: None }),
                doctype: None,
                root: Some(XmlNode::Element {
                    name: "root".into(),
                    attrs: vec![XmlAttr { name: "a".into(), value: "1".into() }],
                    children: vec![
                        XmlNode::Text { text: "hello".into() },
                        XmlNode::Element { name: "child".into(), attrs: Vec::new(), children: Vec::new() },
                    ],
                }),
            },
        }
    }

    /// 🌱 `sweep_a`/`sweep_b`: differ in EVERY mutable field. `declaration`/`doctype` both go
    /// `Some(x) -> None` (tri-state `Some(None)`). `root`'s attrs (name-keyed, so a single triple
    /// can show all three flavors at once) exercise removed+modified+added simultaneously. The
    /// naive positional `between_children` (recipe-specified: pairwise `0..min`, base-tail
    /// removed, other-tail added) can only ever show ONE of {removed-tail, added-tail} per
    /// instance -- so `removed` is exercised at the top-level children triple and `added` at the
    /// nested triple inside the modified child, while that same modified child's OWN diff
    /// (name+attributes+children all `Some`) is the "modified-in-every-field" collection entry.
    fn sweep_a() -> XmlSnapshot {
        XmlSnapshot {
            schema: STDIO_XML_DOCUMENT_SCHEMA.into(),
            doc: XmlDocument {
                declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true) }),
                doctype: Some("<!DOCTYPE html>".into()),
                root: Some(XmlNode::Element {
                    name: "root".into(),
                    attrs: vec![
                        XmlAttr { name: "keep".into(), value: "k".into() },
                        XmlAttr { name: "toRemove".into(), value: "r".into() },
                        XmlAttr { name: "toModify".into(), value: "old".into() },
                    ],
                    children: vec![
                        XmlNode::Element {
                            name: "modifyMe".into(),
                            attrs: vec![XmlAttr { name: "x".into(), value: "1".into() }],
                            children: vec![XmlNode::Element { name: "inner".into(), attrs: Vec::new(), children: Vec::new() }],
                        },
                        XmlNode::Text { text: "stay".into() },
                        XmlNode::Element { name: "toDrop".into(), attrs: Vec::new(), children: Vec::new() },
                    ],
                }),
            },
        }
    }

    fn sweep_b() -> XmlSnapshot {
        XmlSnapshot {
            schema: STDIO_XML_DOCUMENT_SCHEMA.into(),
            doc: XmlDocument {
                declaration: None,
                doctype: None,
                root: Some(XmlNode::Element {
                    name: "rootRenamed".into(),
                    attrs: vec![
                        XmlAttr { name: "keep".into(), value: "k".into() },
                        XmlAttr { name: "toModify".into(), value: "new".into() },
                        XmlAttr { name: "added".into(), value: "a".into() },
                    ],
                    children: vec![
                        XmlNode::Element {
                            name: "modifiedNow".into(),
                            attrs: vec![XmlAttr { name: "x".into(), value: "2".into() }, XmlAttr { name: "y".into(), value: "3".into() }],
                            children: vec![
                                XmlNode::Element { name: "inner".into(), attrs: Vec::new(), children: Vec::new() },
                                XmlNode::Element { name: "innerNew".into(), attrs: Vec::new(), children: Vec::new() },
                            ],
                        },
                        XmlNode::Text { text: "stay".into() },
                    ],
                }),
            },
        }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️MutationDiffLaw
    fn sample_mutations() -> Vec<XmlMutation> {
        vec![
            XmlMutation::NoMutation,
            XmlMutation::SetSnapshot { snapshot: sweep_b() },
            XmlMutation::SetDeclaration { declaration: Some(XmlDeclaration { version: "1.1".into(), encoding: Some("UTF-8".into()), standalone: Some(false) }) },
            XmlMutation::SetDeclaration { declaration: None },
            XmlMutation::SetDoctype { doctype: Some("<!DOCTYPE foo>".into()) },
            XmlMutation::InsertElement { path: XmlNodePath::root(), index: 2, node: XmlNode::Text { text: "new".into() } },
            XmlMutation::RemoveElement { path: XmlNodePath::root(), index: 1 },
            XmlMutation::SetAttribute { path: XmlNodePath::root(), name: "a".into(), value: Some("2".into()) },
            XmlMutation::SetAttribute { path: XmlNodePath::root(), name: "b".into(), value: Some("new".into()) },
            XmlMutation::SetAttribute { path: XmlNodePath::root(), name: "a".into(), value: None },
            XmlMutation::SetText { path: XmlNodePath(vec![0]), text: "world".into() },
        ]
    }

    #[test]
    fn mutation_diff_law() {
        for mutation in sample_mutations() {
            let base = sample_snapshot();
            let diff_direct = Mutation::diff(&mutation, &base);
            let applied_via_diff = MutationDiff::apply(&diff_direct, &base);

            let mut via_apply = base.clone();
            let diff_from_apply = crate::artifacts::xml::schema::mutations::apply_xml_mutation(&mut via_apply, &mutation);

            assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
            assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[test]
    fn inverse_law() {
        for mutation in sample_mutations() {
            let base = sample_snapshot();

            // Mutation-level round-trip.
            let mut round_tripped = base.clone();
            crate::artifacts::xml::schema::mutations::apply_xml_mutation(&mut round_tripped, &mutation);
            for inverse_mutation in <XmlMutation as Mutation<XmlSnapshot>>::inverse(&mutation, &base) {
                crate::artifacts::xml::schema::mutations::apply_xml_mutation(&mut round_tripped, &inverse_mutation);
            }
            assert_eq!(round_tripped, base, "inverse_law (mutation-level) failed for {mutation:?}");

            // Diff-level round-trip.
            let diff = Mutation::diff(&mutation, &base);
            let next = MutationDiff::apply(&diff, &base);
            let inverse_diff = DiffAlgebra::inverse(&diff, &base);
            let restored = MutationDiff::apply(&inverse_diff, &next);
            assert_eq!(restored, base, "inverse_law (diff-level) failed for {mutation:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    fn two_child_root(a_name: &str, b_name: &str) -> XmlSnapshot {
        XmlSnapshot {
            schema: STDIO_XML_DOCUMENT_SCHEMA.into(),
            doc: XmlDocument {
                declaration: None,
                doctype: None,
                root: Some(XmlNode::Element {
                    name: "root".into(),
                    attrs: Vec::new(),
                    children: vec![
                        XmlNode::Element { name: a_name.into(), attrs: Vec::new(), children: Vec::new() },
                        XmlNode::Element { name: b_name.into(), attrs: Vec::new(), children: Vec::new() },
                    ],
                }),
            },
        }
    }

    fn assert_absorb_matches_sequential(base: &XmlSnapshot, d1: &XmlDiff, d2: &XmlDiff) -> XmlDiff {
        let sequential = MutationDiff::apply(d2, &MutationDiff::apply(d1, base));
        let mut absorbed = d1.clone();
        MutationDiff::absorb(&mut absorbed, d2.clone());
        assert_eq!(MutationDiff::apply(&absorbed, base), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
        absorbed
    }

    fn root_children_diff(diff: &XmlDiff) -> &crate::artifacts::xml::schema::diff::XmlChildrenDiff {
        match diff.root.as_ref().expect("root diff present") {
            XmlNodeDiff::Element(e) => e.children.as_ref().expect("children diff present"),
            other => panic!("expected element diff, got {other:?}"),
        }
    }

    #[test]
    fn absorb_law() {
        // Canonical: Insert(2)+Remove(0) -> {removed:[0], added:[(1,f)]}.
        {
            let base = two_child_root("a", "b");
            let d1 = Mutation::diff(&XmlMutation::InsertElement { path: XmlNodePath::root(), index: 2, node: XmlNode::Element { name: "f".into(), attrs: Vec::new(), children: Vec::new() } }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&XmlMutation::RemoveElement { path: XmlNodePath::root(), index: 0 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = root_children_diff(&absorbed);
            assert_eq!(triple.removed, vec![0]);
            assert_eq!(triple.added.len(), 1);
            assert_eq!(triple.added[0].index, 1);
            let XmlNode::Element { name, .. } = &triple.added[0].item else { panic!("expected element") };
            assert_eq!(name, "f");
        }

        // Canonical: Insert(2,f)+Insert(2,g) -> both survive.
        {
            let base = two_child_root("a", "b");
            let d1 = Mutation::diff(&XmlMutation::InsertElement { path: XmlNodePath::root(), index: 2, node: XmlNode::Element { name: "f".into(), attrs: Vec::new(), children: Vec::new() } }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&XmlMutation::InsertElement { path: XmlNodePath::root(), index: 2, node: XmlNode::Element { name: "g".into(), attrs: Vec::new(), children: Vec::new() } }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = root_children_diff(&absorbed);
            assert_eq!(triple.added.len(), 2, "both inserts must survive absorb, not LWW-clobber");
            let names: Vec<&str> = triple.added.iter().map(|a| match &a.item { XmlNode::Element { name, .. } => name.as_str(), _ => "" }).collect();
            assert!(names.contains(&"f"));
            assert!(names.contains(&"g"));
        }

        // Canonical: Insert(1,f)+SetField(1,v) -> patch into the added payload.
        {
            let base = two_child_root("a", "b");
            let d1 = Mutation::diff(
                &XmlMutation::InsertElement { path: XmlNodePath::root(), index: 1, node: XmlNode::Element { name: "f".into(), attrs: Vec::new(), children: Vec::new() } },
                &base,
            );
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&XmlMutation::SetAttribute { path: XmlNodePath(vec![1]), name: "k".into(), value: Some("v".into()) }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = root_children_diff(&absorbed);
            assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
            assert_eq!(triple.added.len(), 1);
            let XmlNode::Element { attrs, .. } = &triple.added[0].item else { panic!("expected element") };
            assert!(attrs.iter().any(|a| a.name == "k" && a.value == "v"));
        }

        // Canonical: Modify+Remove -> the modify is annihilated by the later remove.
        {
            let base = two_child_root("a", "b");
            let d1 = Mutation::diff(&XmlMutation::SetAttribute { path: XmlNodePath(vec![1]), name: "k".into(), value: Some("v".into()) }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&XmlMutation::RemoveElement { path: XmlNodePath::root(), index: 1 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = root_children_diff(&absorbed);
            assert!(triple.modified.is_empty(), "modify of a since-removed item must not survive absorb");
            assert_eq!(triple.removed, vec![1]);
        }

        // Associativity over a triple.
        {
            let base = two_child_root("a", "b");
            let d1 = Mutation::diff(&XmlMutation::InsertElement { path: XmlNodePath::root(), index: 2, node: XmlNode::Element { name: "f".into(), attrs: Vec::new(), children: Vec::new() } }, &base);
            let mid1 = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&XmlMutation::InsertElement { path: XmlNodePath::root(), index: 2, node: XmlNode::Element { name: "g".into(), attrs: Vec::new(), children: Vec::new() } }, &mid1);
            let mid2 = MutationDiff::apply(&d2, &mid1);
            let d3 = Mutation::diff(&XmlMutation::RemoveElement { path: XmlNodePath::root(), index: 0 }, &mid2);
            let sequential = MutationDiff::apply(&d3, &mid2);

            let mut left = d1.clone();
            MutationDiff::absorb(&mut left, d2.clone());
            MutationDiff::absorb(&mut left, d3.clone());

            let mut d2_then_d3 = d2.clone();
            MutationDiff::absorb(&mut d2_then_d3, d3.clone());
            let mut right = d1.clone();
            MutationDiff::absorb(&mut right, d2_then_d3);

            assert_eq!(MutationDiff::apply(&left, &base), sequential, "absorb associativity (left) failed");
            assert_eq!(MutationDiff::apply(&right, &base), sequential, "absorb associativity (right) failed");
        }
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[test]
    fn between_roundtrip_law() {
        // Synthetic pairs.
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(MutationDiff::apply(&DiffAlgebra::between(&a, &b), &a), b);
        assert_eq!(MutationDiff::apply(&DiffAlgebra::between(&b, &a), &b), a);

        let sample = sample_snapshot();
        assert_eq!(MutationDiff::apply(&DiffAlgebra::between(&sample, &sample), &sample), sample);

        // Real fixture (the demo's `example.xml`) diffed against a mutated variant.
        let fixture_text = include_str!("../../../📚️examples/🎬️demo/🖼️assets/example.xml");
        let fixture_doc = crate::artifacts::xml::schema::snapshot::xml_document_from_text(fixture_text).expect("fixture parses");
        let fixture = XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: fixture_doc };
        let mut mutated = fixture.clone();
        crate::artifacts::xml::schema::mutations::apply_xml_mutation(
            &mut mutated,
            &XmlMutation::SetAttribute { path: XmlNodePath::root(), name: "id".into(), value: Some("1".into()) },
        );
        assert_ne!(fixture, mutated);
        assert_eq!(MutationDiff::apply(&DiffAlgebra::between(&fixture, &mutated), &fixture), mutated);
        assert_eq!(MutationDiff::apply(&DiffAlgebra::between(&mutated, &fixture), &mutated), fixture);
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️CodecRetentionLaw
    #[test]
    fn codec_retention_law() {
        let fixture_text = include_str!("../../../📚️examples/🎬️demo/🖼️assets/example.xml");
        let doc = crate::artifacts::xml::schema::snapshot::xml_document_from_text(fixture_text).expect("fixture parses");
        // Documented normal form: leading/trailing whitespace around the document is trimmed (the
        // codec re-emits no trailing newline); the fixture has neither internal whitespace nor
        // empty elements, so the byte content otherwise round-trips exactly.
        let re_encoded = crate::artifacts::xml::schema::snapshot::xml_document_to_text(&doc);
        assert_eq!(re_encoded, fixture_text.trim());

        let snap = XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc };
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <XmlSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
    //#endregion 🔖️CodecRetentionLaw

    //#region 🔖️FieldSweep
    /// 🎯️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable field (see the
    /// fixtures' doc comment for exactly how each collection flavor -- removed/modified/added --
    /// is exercised given the recipe's naive positional `between_children`).
    #[test]
    fn field_sweep_law() {
        let a = sweep_a();
        let b = sweep_b();

        let diff_ab = <XmlDiff as DiffAlgebra<XmlSnapshot>>::between(&a, &b);
        assert_eq!(MutationDiff::apply(&diff_ab, &a), b);
        let diff_ba = <XmlDiff as DiffAlgebra<XmlSnapshot>>::between(&b, &a);
        assert_eq!(MutationDiff::apply(&diff_ba, &b), a);
        assert!(<XmlDiff as DiffAlgebra<XmlSnapshot>>::between(&a, &a).is_empty());

        // Hand-written per-field assertion: every top-level XmlDiff field is populated, and both
        // tri-state scalars exercise `Some(None)`.
        assert_eq!(diff_ab.declaration, Some(None));
        assert_eq!(diff_ab.doctype, Some(None));
        assert!(diff_ab.root.is_some());

        let XmlNodeDiff::Element(root_diff) = diff_ab.root.as_ref().unwrap() else { panic!("expected element diff") };
        assert!(root_diff.name.is_some());
        let attrs_diff = root_diff.attributes.as_ref().expect("attrs diff present");
        assert!(!attrs_diff.removed.is_empty(), "attrs: removed not exercised");
        assert!(!attrs_diff.modified.is_empty(), "attrs: modified not exercised");
        assert!(!attrs_diff.added.is_empty(), "attrs: added not exercised");

        let children_diff = root_diff.children.as_ref().expect("children diff present");
        assert!(!children_diff.removed.is_empty(), "children: removed not exercised");
        assert_eq!(children_diff.modified.len(), 1);
        let modified_entry = &children_diff.modified[0];
        let XmlNodeDiff::Element(modified_element) = &modified_entry.diff else { panic!("expected element diff") };
        assert!(modified_element.name.is_some(), "modified child: name not exercised");
        assert!(modified_element.attributes.is_some(), "modified child: attributes not exercised");
        let nested_children: &crate::artifacts::xml::schema::diff::XmlChildrenDiff =
            modified_element.children.as_ref().expect("nested children diff present");
        let nested_added: &Vec<XmlChildAdded> = &nested_children.added;
        assert!(!nested_added.is_empty(), "children: added (nested) not exercised");
    }
    //#endregion 🔖️FieldSweep
}
//#endregion 🧪️Tests
