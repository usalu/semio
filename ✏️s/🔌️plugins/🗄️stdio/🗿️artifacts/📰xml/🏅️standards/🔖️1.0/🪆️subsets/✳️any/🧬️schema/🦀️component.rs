//! 🧬️ XmlArtifact schema — full artifact state.

use crate::artifacts::xml::XmlSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.xml` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xml")]
pub struct XmlArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub doc: crate::artifacts::xml::schema::snapshot::XmlDocument,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for XmlArtifact {
    fn default() -> Self {
        Self::from_snapshot(XmlSnapshot::default())
    }
}

impl XmlArtifact {
    /// 📸️ Persisted subset.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> XmlSnapshot {
        XmlSnapshot { schema: self.schema.clone(), doc: self.doc.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: XmlSnapshot) -> Self {
        Self { schema: snapshot.schema, doc: snapshot.doc }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: XmlSnapshot) {
        self.schema = snapshot.schema;
        self.doc = snapshot.doc;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.xml`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn xml_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.xml",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::xml::{XmlDiff, XmlMutation, XmlSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.xml` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct XmlBuilderConstruction {
        snapshot: XmlSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for XmlBuilderConstruction {
        type Snapshot = XmlSnapshot;
        type Mutation = XmlMutation;
        type Diff = XmlDiff;
        async fn empty() -> Self {
            Self { snapshot: XmlSnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<XmlSnapshot as store::ArtifactDsl>::parse_dsl(text).await?).await)
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<XmlSnapshot as store::ArtifactPack>::decode_pack(bytes).await?).await)
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::xml::schema::mutations::apply_xml_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <XmlDiff as protocol::MutationDiff<XmlSnapshot>>::apply(&diff, &self.snapshot).await?;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::xml::XmlSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.xml` parts.
    #[derive(Clone, Debug, Default)]
    pub struct XmlParts {
        pub snapshot: Option<XmlSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.xml` (1.0/✳️any) sources.
    pub struct XmlAnalyzerAnalysis;

    impl ArtifactAnalysis for XmlAnalyzerAnalysis {
        type Parts = XmlParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };

        async fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = XmlParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <XmlSnapshot as store::ArtifactDsl>::parse_dsl(text).await {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <XmlSnapshot as store::ArtifactPack>::decode_pack(bytes).await {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_xml_snapshot() -> XmlSnapshot {
    XmlSnapshot::default()
}

/// 📄️ The demo `stdio.xml` document -- exercises every real-syntax construct the W0 census row
/// names: an XML declaration, a simple `<!DOCTYPE name>`, a namespaced (`:`-qualified) attribute
/// name, both quote-delimiter styles (`"`/`'`, via `attribute`'s shared `TEXT` terminal), entity
/// decode (`Tom &amp; Jerry`), a self-closing element (carrying an attribute so its trailing `/`
/// never fuses with the preceding ident -- see `../…/📸️snapshot/📝️text/📖️component.grammar.semio`'s
/// own `name` doc comment), `<![CDATA[...]]>`, `<!--...-->`, and a `<?target data?>` processing
/// instruction. The single source of truth for
/// `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` (both are literally this
/// snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law` below).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_xml_snapshot() -> XmlSnapshot {
    use crate::artifacts::xml::schema::snapshot::{XmlAttr, XmlDeclaration, XmlDocument, XmlNode};
    use crate::artifacts::xml::STDIO_XML_DOCUMENT_SCHEMA;
    let root = XmlNode::Element {
        name: "catalog".into(),
        attrs: vec![XmlAttr { name: "xmlns:c".into(), value: "urn:example:catalog".into() }, XmlAttr { name: "version".into(), value: "2".into() }],
        children: vec![
            XmlNode::Comment { text: " demo catalog ".into() },
            XmlNode::ProcessingInstruction { target: "xml-stylesheet".into(), data: "text".into() },
            XmlNode::Element { name: "item".into(), attrs: vec![XmlAttr { name: "id".into(), value: "1".into() }], children: vec![XmlNode::Text { text: "Tom & Jerry".into() }] },
            XmlNode::Element { name: "empty".into(), attrs: vec![XmlAttr { name: "flag".into(), value: "true".into() }], children: vec![] },
            XmlNode::CData { text: "raw markup".into() },
        ],
    };
    XmlSnapshot {
        schema: STDIO_XML_DOCUMENT_SCHEMA.into(),
        doc: XmlDocument { declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true) }), doctype: Some("<!DOCTYPE catalog>".into()), prolog: Vec::new(), root: Some(root) },
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec XmlBuilderFacets {
        construction: XmlBuilderConstruction,
        analysis: XmlAnalyzerAnalysis,
        composition: super::super::io::derived_composition::XmlComposerComposition,
    }
    builder: XmlBuilder,
    analyzer: XmlAnalyzer,
    composer: XmlComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::xml::schema::diff::{XmlChildAdded, XmlNodeDiff};
    use crate::artifacts::xml::schema::mutations::XmlNodePath;
    use crate::artifacts::xml::schema::snapshot::{XmlAttr, XmlDeclaration, XmlDocument, XmlNode};
    use crate::artifacts::xml::{XmlDiff, XmlMutation, STDIO_XML_DOCUMENT_SCHEMA};
    use protocol::command::DiffAlgebra;
    use protocol::{Mutation, MutationDiff};

    #[semio_framework_async_macros::async_test]
    async fn schema_facets_reject_raw_doctype_and_source_shadow_state() {
        let facets = [
            include_str!("📸️snapshot/🦀️component.rs"),
            include_str!("📸️snapshot/🟦️component.ts"),
            include_str!("📸️snapshot/🔣️component.json"),
            include_str!("📸️snapshot/📝️text/🔗️component.graphql"),
            include_str!("📸️snapshot/📝️text/🛰️component.proto"),
            include_str!("🔺️diff/📝️text/📖️component.grammar.semio"),
            include_str!("🔺️diff/💾️binary/📡️component.protocol.semio"),
            include_str!("🧬️mutations/📝️text/📖️component.grammar.semio"),
            include_str!("🧬️mutations/💾️binary/📡️component.protocol.semio"),
        ];
        for facet in facets {
            for forbidden in [concat!("pub doctype: Option<", "String>"), concat!("raw ", "doctype"), concat!("source-", "field"), concat!("source-", "tok"), concat!("artifact-", "source"), concat!("semantic-", "blake3")] {
                assert!(!facet.to_ascii_lowercase().contains(&forbidden.to_ascii_lowercase()), "forbidden XML shadow-state facet: {forbidden}");
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_snapshot_matches_schema() {
        let snapshot = empty_xml_snapshot();
        assert_eq!(snapshot.schema, STDIO_XML_DOCUMENT_SCHEMA);
    }

    #[semio_framework_async_macros::async_test]
    async fn codec_round_trip() {
        let snap = empty_xml_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <XmlSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <XmlSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️Fixtures
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_snapshot() -> XmlSnapshot {
        XmlSnapshot {
            schema: STDIO_XML_DOCUMENT_SCHEMA.into(),
            doc: XmlDocument {
                declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: None, standalone: None }),
                doctype: None,
                prolog: Vec::new(),
                root: Some(XmlNode::Element {
                    name: "root".into(),
                    attrs: vec![XmlAttr { name: "a".into(), value: "1".into() }],
                    children: vec![XmlNode::Text { text: "hello".into() }, XmlNode::Element { name: "child".into(), attrs: Vec::new(), children: Vec::new() }],
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_a() -> XmlSnapshot {
        XmlSnapshot {
            schema: STDIO_XML_DOCUMENT_SCHEMA.into(),
            doc: XmlDocument {
                declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true) }),
                doctype: Some("<!DOCTYPE html>".into()),
                prolog: Vec::new(),
                root: Some(XmlNode::Element {
                    name: "root".into(),
                    attrs: vec![XmlAttr { name: "keep".into(), value: "k".into() }, XmlAttr { name: "toRemove".into(), value: "r".into() }, XmlAttr { name: "toModify".into(), value: "old".into() }],
                    children: vec![
                        XmlNode::Element { name: "modifyMe".into(), attrs: vec![XmlAttr { name: "x".into(), value: "1".into() }], children: vec![XmlNode::Element { name: "inner".into(), attrs: Vec::new(), children: Vec::new() }] },
                        XmlNode::Text { text: "stay".into() },
                        XmlNode::Element { name: "toDrop".into(), attrs: Vec::new(), children: Vec::new() },
                    ],
                }),
            },
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_b() -> XmlSnapshot {
        XmlSnapshot {
            schema: STDIO_XML_DOCUMENT_SCHEMA.into(),
            doc: XmlDocument {
                declaration: None,
                doctype: None,
                prolog: Vec::new(),
                root: Some(XmlNode::Element {
                    name: "rootRenamed".into(),
                    attrs: vec![XmlAttr { name: "keep".into(), value: "k".into() }, XmlAttr { name: "toModify".into(), value: "new".into() }, XmlAttr { name: "added".into(), value: "a".into() }],
                    children: vec![
                        XmlNode::Element {
                            name: "modifiedNow".into(),
                            attrs: vec![XmlAttr { name: "x".into(), value: "2".into() }, XmlAttr { name: "y".into(), value: "3".into() }],
                            children: vec![XmlNode::Element { name: "inner".into(), attrs: Vec::new(), children: Vec::new() }, XmlNode::Element { name: "innerNew".into(), attrs: Vec::new(), children: Vec::new() }],
                        },
                        XmlNode::Text { text: "stay".into() },
                    ],
                }),
            },
        }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️MutationDiffLaw
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        for mutation in sample_mutations() {
            let base = sample_snapshot();
            let diff_direct = Mutation::diff(&mutation, &base);
            let applied_via_diff = MutationDiff::apply(diff_direct.diff(), &base).unwrap();

            let mut via_apply = base.clone();
            let diff_from_apply = crate::artifacts::xml::schema::mutations::apply_xml_mutation(&mut via_apply, &mutation);

            assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
            assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
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
            let next = MutationDiff::apply(diff.diff(), &base).unwrap();
            let inverse_diff = DiffAlgebra::inverse(diff.diff(), &base);
            let restored = MutationDiff::apply(&inverse_diff, &next).unwrap();
            assert_eq!(restored, base, "inverse_law (diff-level) failed for {mutation:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn two_child_root(a_name: &str, b_name: &str) -> XmlSnapshot {
        XmlSnapshot {
            schema: STDIO_XML_DOCUMENT_SCHEMA.into(),
            doc: XmlDocument {
                declaration: None,
                doctype: None,
                prolog: Vec::new(),
                root: Some(XmlNode::Element {
                    name: "root".into(),
                    attrs: Vec::new(),
                    children: vec![XmlNode::Element { name: a_name.into(), attrs: Vec::new(), children: Vec::new() }, XmlNode::Element { name: b_name.into(), attrs: Vec::new(), children: Vec::new() }],
                }),
            },
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn assert_absorb_matches_sequential(base: &XmlSnapshot, d1: &XmlDiff, d2: &XmlDiff) -> XmlDiff {
        let sequential = MutationDiff::apply(d2, &MutationDiff::apply(d1, base).unwrap()).unwrap();
        let mut absorbed = d1.clone();
        MutationDiff::absorb(&mut absorbed, d2.clone());
        assert_eq!(MutationDiff::apply(&absorbed, base).unwrap(), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
        absorbed
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn root_children_diff(diff: &XmlDiff) -> &crate::artifacts::xml::schema::diff::XmlChildrenDiff {
        match diff.root.as_ref().expect("root diff present") {
            XmlNodeDiff::Element(e) => e.children.as_ref().expect("children diff present"),
            other => panic!("expected element diff, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law() {
        // Canonical: Insert(2)+Remove(0) -> {removed:[0], added:[(1,f)]}.
        {
            let base = two_child_root("a", "b");
            let d1 = Mutation::diff(&XmlMutation::InsertElement { path: XmlNodePath::root(), index: 2, node: XmlNode::Element { name: "f".into(), attrs: Vec::new(), children: Vec::new() } }, &base);
            let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&XmlMutation::RemoveElement { path: XmlNodePath::root(), index: 0 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
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
            let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&XmlMutation::InsertElement { path: XmlNodePath::root(), index: 2, node: XmlNode::Element { name: "g".into(), attrs: Vec::new(), children: Vec::new() } }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
            let triple = root_children_diff(&absorbed);
            assert_eq!(triple.added.len(), 2, "both inserts must survive absorb, not LWW-clobber");
            let names: Vec<&str> = triple
                .added
                .iter()
                .map(|a| match &a.item {
                    XmlNode::Element { name, .. } => name.as_str(),
                    _ => "",
                })
                .collect();
            assert!(names.contains(&"f"));
            assert!(names.contains(&"g"));
        }

        // Canonical: Insert(1,f)+SetField(1,v) -> patch into the added payload.
        {
            let base = two_child_root("a", "b");
            let d1 = Mutation::diff(&XmlMutation::InsertElement { path: XmlNodePath::root(), index: 1, node: XmlNode::Element { name: "f".into(), attrs: Vec::new(), children: Vec::new() } }, &base);
            let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&XmlMutation::SetAttribute { path: XmlNodePath(vec![1]), name: "k".into(), value: Some("v".into()) }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
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
            let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&XmlMutation::RemoveElement { path: XmlNodePath::root(), index: 1 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
            let triple = root_children_diff(&absorbed);
            assert!(triple.modified.is_empty(), "modify of a since-removed item must not survive absorb");
            assert_eq!(triple.removed, vec![1]);
        }

        // Associativity over a triple.
        {
            let base = two_child_root("a", "b");
            let d1 = Mutation::diff(&XmlMutation::InsertElement { path: XmlNodePath::root(), index: 2, node: XmlNode::Element { name: "f".into(), attrs: Vec::new(), children: Vec::new() } }, &base);
            let mid1 = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&XmlMutation::InsertElement { path: XmlNodePath::root(), index: 2, node: XmlNode::Element { name: "g".into(), attrs: Vec::new(), children: Vec::new() } }, &mid1);
            let mid2 = MutationDiff::apply(d2.diff(), &mid1).unwrap();
            let d3 = Mutation::diff(&XmlMutation::RemoveElement { path: XmlNodePath::root(), index: 0 }, &mid2);
            let sequential = MutationDiff::apply(d3.diff(), &mid2).unwrap();

            let mut left = d1.diff().clone();
            MutationDiff::absorb(&mut left, d2.diff().clone());
            MutationDiff::absorb(&mut left, d3.diff().clone());

            let mut d2_then_d3 = d2.diff().clone();
            MutationDiff::absorb(&mut d2_then_d3, d3.diff().clone());
            let mut right = d1.diff().clone();
            MutationDiff::absorb(&mut right, d2_then_d3);

            assert_eq!(MutationDiff::apply(&left, &base).unwrap(), sequential, "absorb associativity (left) failed");
            assert_eq!(MutationDiff::apply(&right, &base).unwrap(), sequential, "absorb associativity (right) failed");
        }
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        // Synthetic pairs.
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(MutationDiff::apply(&<XmlDiff as DiffAlgebra<XmlSnapshot>>::between(&a, &b), &a).unwrap(), b);
        assert_eq!(MutationDiff::apply(&<XmlDiff as DiffAlgebra<XmlSnapshot>>::between(&b, &a), &b).unwrap(), a);

        let sample = sample_snapshot();
        assert_eq!(MutationDiff::apply(&<XmlDiff as DiffAlgebra<XmlSnapshot>>::between(&sample, &sample), &sample).unwrap(), sample);

        // Real fixture (the demo's `📰️example.xml`) diffed against a mutated variant.
        let fixture_text = include_str!("../📚️examples/🎬️demo/🖼️assets/📰️example.xml");
        let fixture_doc = crate::artifacts::xml::schema::snapshot::xml_document_from_text(fixture_text).expect("fixture parses");
        let fixture = XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: fixture_doc };
        let mut mutated = fixture.clone();
        crate::artifacts::xml::schema::mutations::apply_xml_mutation(&mut mutated, &XmlMutation::SetAttribute { path: XmlNodePath::root(), name: "id".into(), value: Some("1".into()) });
        assert_ne!(fixture, mutated);
        assert_eq!(MutationDiff::apply(&<XmlDiff as DiffAlgebra<XmlSnapshot>>::between(&fixture, &mutated), &fixture).unwrap(), mutated);
        assert_eq!(MutationDiff::apply(&<XmlDiff as DiffAlgebra<XmlSnapshot>>::between(&mutated, &fixture), &mutated).unwrap(), fixture);
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️CodecRetentionLaw
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let fixture_text = include_str!("../📚️examples/🎬️demo/🖼️assets/📰️example.xml");
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
    #[semio_framework_async_macros::async_test]
    async fn field_sweep_law() {
        let a = sweep_a();
        let b = sweep_b();

        let diff_ab = <XmlDiff as DiffAlgebra<XmlSnapshot>>::between(&a, &b);
        assert_eq!(MutationDiff::apply(&diff_ab, &a).unwrap(), b);
        let diff_ba = <XmlDiff as DiffAlgebra<XmlSnapshot>>::between(&b, &a);
        assert_eq!(MutationDiff::apply(&diff_ba, &b).unwrap(), a);
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
        let nested_children: &crate::artifacts::xml::schema::diff::XmlChildrenDiff = modified_element.children.as_ref().expect("nested children diff present");
        let nested_added: &Vec<XmlChildAdded> = &nested_children.added;
        assert!(!nested_added.is_empty(), "children: added (nested) not exercised");
    }
    //#endregion 🔖️FieldSweep

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-FG1: per-artifact conformance laws (`📖️grammar-recipe.md` §4's checklist item) --
    /// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
    /// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff`
    /// bytes, and the fixture-honesty round-trip. Lives beside the rest of this artifact's schema
    /// tests (moved out of `⚙️engine`, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) --
    /// these tests are this artifact's OWN early-warning, plus direct coverage of the
    /// mutations/diff facets that harness does not auto-discover at all.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::xml::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect -- independent of, and cheaper than, the two `recognize`/
        /// `walk_protocol` laws below (a parse failure here fails fast with a clearer message).
        #[semio_framework_async_macros::async_test]
        async fn committed_facet_files_parse() {
            for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output for
        /// the demo document -- same preamble-stripped body reconstruction
        /// `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture` uses, so this is a
        /// direct proof this artifact will pass that harness once graduated, not merely an analogue.
        #[semio_framework_async_macros::async_test]
        async fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_xml_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op` output
        /// for every `XmlMutation` variant (`mutations::demo_mutation_cases()`).
        #[semio_framework_async_macros::async_test]
        async fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
        /// for every representative `XmlDiff` (`diff::demo_diff_cases()`), incl. the empty-line
        /// (all-`None`) diff and the `Replace` kind-change fallback.
        #[semio_framework_async_macros::async_test]
        async fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets --
        /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
        /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
        /// mutation's `encode_op`, and every demo diff's `encode_diff` -- asserting `consumed ==
        /// bytes.len()`.
        #[semio_framework_async_macros::async_test]
        async fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_xml_snapshot());
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

            let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            for mutation in mutations::demo_mutation_cases() {
                let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in diff::demo_diff_cases() {
                let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
        /// `print_dsl`/`encode_pack` output of `demo_xml_snapshot()` -- `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin -- so the
        /// fixtures can never silently drift back to a fake again.
        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_xml_snapshot();

            let parsed = <XmlSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_xml_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_xml_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <XmlSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_xml_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_xml_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
