//! ✂️ `splice-source` — authored as its own mutation leaf: the change set an editor made to the HTML source text, carried verbatim.
//! It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;
use semio_s_artifact_stdio_html_snapshot::standards::v5::subsets::any::io::text::snapshot::{parse_html_document, write_html_document};

//#region 🔖️Payload
/// ✂️ One range of the edited source: `delete` Unicode scalars at `offset` of the document's HTML source (its canonical serialization)
/// are replaced by `insert`.
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceSplice {
    pub offset: u32,
    pub delete: u32,
    pub insert: String,
}

/// ✂️ The ranges of one edit, ascending and disjoint, in the coordinates of the source they were taken from.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SpliceSource {
    pub splices: Vec<SourceSplice>,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl SpliceSource {
    /// 🧭️ The document the ranges mean on `base`, or the refusal reason: the base source with the ranges applied, parsed again.
    fn plan(&self, base: &HtmlSnapshot) -> Result<HtmlSnapshot, String> {
        let source: Vec<char> = write_html_document(base).chars().collect();
        let mut text = String::with_capacity(source.len());
        let mut position = 0usize;
        for splice in &self.splices {
            let (offset, delete) = (splice.offset as usize, splice.delete as usize);
            if offset < position || offset + delete > source.len() {
                return Err("the splices must ascend, never overlap and stay inside the source".to_string());
            }
            text.extend(&source[position..offset]);
            text.push_str(&splice.insert);
            position = offset + delete;
        }
        text.extend(&source[position..]);
        parse_html_document(&text).map_err(|error| error.to_string())
    }
}

impl protocol::MutationKind<HtmlSnapshot, HtmlMutation> for SpliceSource {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "source", kind: "splice-source", record: "SpliceSource" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Edit source", "Quelltext bearbeiten")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<HtmlSnapshot, HtmlMutation> for SpliceSource {
    fn diff(&self, base: &HtmlSnapshot) -> protocol::MutationOutcome<<HtmlMutation as protocol::ColdMutation<HtmlSnapshot>>::ReferenceDiff> {
        match self.plan(base) {
            Err(reason) => protocol::MutationOutcome::fatal("mutation.invariant", reason, Vec::<String>::new()),
            Ok(next) => protocol::MutationOutcome::new(HtmlDiff {
                doctype: (next.doctype != base.doctype).then_some(next.doctype),
                root: (next.root != base.root).then_some(HtmlNodeDiff::Replace { node: next.root }),
            }),
        }
    }
    /// ↩️ One range over the whole edited source that puts the base source back.
    fn inverse(&self, base: &HtmlSnapshot) -> Result<Vec<HtmlMutation>, semio_framework_value::ValueError> {
        let Ok(next) = self.plan(base) else { return Ok(Vec::new()) };
        if next == *base {
            return Ok(Vec::new());
        }
        let restore = |value: usize| u32::try_from(value).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "html source exceeds the supported size"));
        Ok(vec![HtmlMutation::SpliceSource(SpliceSource { splices: vec![SourceSplice { offset: 0, delete: restore(write_html_document(&next).chars().count())?, insert: write_html_document(base) }] })])
    }
}
//#endregion ⚙️Semantics
