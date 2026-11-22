//! 🔀️ Validated reversible scene splices prepared as one snapshot.

use super::SpliceSceneRows;
use crate::Puzzle5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use semio_framework_value::{ValueError, ValueRefusalKind};

pub const SCENE_SPLICE_BATCH: usize = 32;
pub const SCENE_PART_SPLICE_BATCH: usize = 16;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_dsl_record_derive::DslReceiving)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "splice-scene-row-batch")]
pub struct SpliceSceneRowBatch {
    #[dsl(table)]
    pub edits: Vec<SpliceSceneRows>,
}

impl SpliceSceneRowBatch {
    fn apply(&self, base: &Puzzle5dSnapshot, mut inverse: Option<&mut Vec<SpliceSceneRows>>) -> Result<Puzzle5dSnapshot, &'static str> {
        if self.edits.is_empty() || self.edits.len() > SCENE_SPLICE_BATCH {
            return Err("scene splice batch exceeds its bounded extent");
        }
        let mut snapshot = base.clone();
        for edit in &self.edits {
            edit.apply_to_batch(&mut snapshot, inverse.as_deref_mut())?;
        }
        Ok(snapshot)
    }
}

impl<Op> protocol::MutationKind<Puzzle5dSnapshot, Op> for SpliceSceneRowBatch
where
    Op: protocol::Mutation<Puzzle5dSnapshot, Diff = Puzzle5dDiff> + From<Self>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "scene-rows", kind: "splice-scene-row-batch", record: "SplicedSceneRowBatch" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Splice scene row batch", "Gruppe von Szenenzeilen bearbeiten")
    }

    fn target(&self) -> Vec<String> { Vec::new() }
}

impl<Op> protocol::ColdMutationKind<Puzzle5dSnapshot, Op> for SpliceSceneRowBatch
where
    Op: protocol::ColdMutation<Puzzle5dSnapshot, ReferenceDiff = Puzzle5dDiff> + From<Self>,
{
    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        match self.apply(base, None) {
            Ok(snapshot) => protocol::MutationOutcome::new(Puzzle5dDiff::replace_snapshot(snapshot)),
            Err(reason) => protocol::MutationOutcome::fatal("mutation.invariant", reason, Vec::<String>::new()),
        }
    }

    fn inverse(&self, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, ValueError> {
        let mut edits = Vec::with_capacity(self.edits.len().min(SCENE_SPLICE_BATCH));
        self.apply(base, Some(&mut edits)).map_err(|reason| ValueError::literal(ValueRefusalKind::InvalidValue, reason))?;
        edits.reverse();
        Ok(vec![Self { edits }.into()])
    }
}
