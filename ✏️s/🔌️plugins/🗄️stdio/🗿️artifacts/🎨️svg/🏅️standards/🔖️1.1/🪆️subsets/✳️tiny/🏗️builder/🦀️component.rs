//! 🏗️ SvgTinyBuilder (1.1/✳️tiny) — a builder whose `build()` can only return an
//! `SvgSnapshot` that is SVG Tiny 1.1-conformant BY CONSTRUCTION (D5 requirement #3): it
//! unconditionally injects `baseProfile="tiny"`/`version="1.1"` onto the document root, then
//! re-runs the SAME `check_svg_tiny_conformance` used by `SvgTinyComposer`, regardless of which
//! path (`from_snapshot`/`from_text`/`from_binary`/`mutate`) produced the in-flight snapshot -- so
//! a hard Tiny 1.1 violation can never leave this builder as an `Ok(SvgSnapshot)`.

use dsl::Diagnostic;
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::svg::standards::v1_1::subsets::tiny::analyzer::check_svg_tiny_conformance;
use crate::artifacts::svg::standards::v1_1::subsets::any::schema::snapshot::set_element_attr;
use crate::artifacts::svg::{SvgDiff, SvgMutation, SvgSnapshot};

//#region 🔖️Builder
#[derive(Clone, Debug, Default)]
pub struct SvgTinyBuilder {
    snapshot: SvgSnapshot,
}

impl ArtifactBuilder for SvgTinyBuilder {
    type Snapshot = SvgSnapshot;
    type Mutation = SvgMutation;
    type Diff = SvgDiff;

    fn empty() -> Self {
        Self::default()
    }

    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot }
    }

    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<SvgSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }

    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<SvgSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }

    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = crate::artifacts::svg::schema::mutations::apply_svg_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }

    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <SvgDiff as protocol::MutationDiff<SvgSnapshot>>::apply(&diff, &self.snapshot);
        self
    }

    /// 🛡️ The real construction gate: injects the profile metadata, then a hard Tiny 1.1
    /// violation (however `self.snapshot` got here) fails `build()` -- soft diagnostics (external
    /// `href`) pass through silently here since `ArtifactBuilder::build`'s `Err` path only ever
    /// carries the hard set, matching the PDF/A pilot's own `build()` shape.
    fn build(mut self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
        if let Some(root) = self.snapshot.doc.root.as_mut() {
            set_element_attr(root, "baseProfile", Some("tiny".into()));
            set_element_attr(root, "version", Some("1.1".into()));
        }
        let hard: Vec<Diagnostic> = check_svg_tiny_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, dsl::Severity::Error | dsl::Severity::Fatal)).collect();
        if hard.is_empty() {
            Ok(self.snapshot)
        } else {
            Err(hard)
        }
    }
}
//#endregion 🔖️Builder

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::svg::standards::v1_1::subsets::tiny::analyzer::CODE_ELEMENT;
    use crate::artifacts::xml::schema::snapshot::XmlNode;

    #[test]
    fn empty_builder_injects_profile_and_builds_clean() {
        let snapshot = SvgTinyBuilder::empty().build().expect("empty document builds clean");
        match &snapshot.doc.root {
            Some(XmlNode::Element { attrs, .. }) => {
                assert!(attrs.iter().any(|a| a.name == "baseProfile" && a.value == "tiny"));
                assert!(attrs.iter().any(|a| a.name == "version" && a.value == "1.1"));
            }
            other => panic!("expected element root, got {other:?}"),
        }
    }

    #[test]
    fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut snapshot = SvgTinyBuilder::empty().build().unwrap();
        if let Some(XmlNode::Element { children, .. }) = snapshot.doc.root.as_mut() {
            children.push(XmlNode::Element { name: "script".into(), attrs: vec![], children: vec![XmlNode::Text { text: "alert(1)".into() }] });
        }
        let (mutated, _diff) = SvgTinyBuilder::from_snapshot(SvgSnapshot::default()).mutate(SvgMutation::SetSnapshot { snapshot });
        let err = mutated.build().expect_err("a <script> element must fail build()");
        assert!(err.iter().any(|d| d.code.0 == CODE_ELEMENT));
    }

    #[test]
    fn from_text_round_trips_through_tiny_build() {
        let text = r#"<svg xmlns="http://www.w3.org/2000/svg"><circle cx="5" cy="5" r="5"/></svg>"#;
        let built = SvgTinyBuilder::from_text(text).expect("parses").build().expect("conforming document builds");
        assert!(matches!(built.doc.root, Some(XmlNode::Element { .. })));
    }
}
