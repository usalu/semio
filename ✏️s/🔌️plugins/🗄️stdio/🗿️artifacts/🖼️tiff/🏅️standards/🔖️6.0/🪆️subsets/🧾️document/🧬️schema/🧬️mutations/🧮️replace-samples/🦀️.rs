//! 🧮️ `replace-samples` — rewrites a contiguous run of owned sample words inside one block of one image page. It builds its own sparse run diff and concrete inverse from its payload and reads of `base`.

use crate::schema::diff::{differing_runs, TiffDiff, TiffIfdDiff, TiffIfdModified, TiffIfdsDiff};
use crate::schema::mutations::TiffMutation;
use crate::schema::snapshot::{TiffSnapshot, TiffWord64};

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplaceSamplesMutation {
    pub ifd_index: usize,
    pub block: usize,
    pub offset: usize,
    pub samples: Vec<TiffWord64>,
}
//#endregion Payload

//#region Semantics
impl ReplaceSamplesMutation {
    /// 🔍️ The words `base` carries over the span this payload addresses, or `None` when the span leaves its block.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn current<'a>(&self, base: &'a TiffSnapshot) -> Option<&'a [TiffWord64]> {
        let end = self.offset.checked_add(self.samples.len())?;
        base.ifds.get(self.ifd_index)?.blocks.get(self.block)?.samples.get(self.offset..end)
    }
}

impl protocol::MutationKind<TiffSnapshot, TiffMutation> for ReplaceSamplesMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "samples", kind: "replace-samples", record: "ReplaceSamples" };
    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<TiffDiff> {
        let Some(current) = self.current(base) else {
            return protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, "tiff: sample run leaves its block", [format!("ifd:{}:block:{}:offset:{}", self.ifd_index, self.block, self.offset)]);
        };
        let runs = differing_runs(self.block, self.offset, current, &self.samples);
        match runs.is_empty() {
            true => protocol::MutationOutcome::new(TiffDiff::default()),
            false => protocol::MutationOutcome::new(TiffDiff { ifds: Some(TiffIfdsDiff { modified: vec![TiffIfdModified { index: self.ifd_index, diff: TiffIfdDiff { runs, ..TiffIfdDiff::default() } }], ..TiffIfdsDiff::default() }) }),
        }
    }
    fn inverse(&self, base: &TiffSnapshot) -> Result<Vec<TiffMutation>, semio_framework_value::ValueError> {
        Ok(self.current(base).map(|current| TiffMutation::ReplaceSamples(Self { ifd_index: self.ifd_index, block: self.block, offset: self.offset, samples: current.to_vec() })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace samples", "Abtastwerte ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec![format!("ifd:{}:block:{}:samples:{}", self.ifd_index, self.block, self.offset)]
    }
}
//#endregion Semantics
