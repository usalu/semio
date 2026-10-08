//! 🫧️ Note composite-window transient mutation: the ephemeral lane travels by whole-root transfer, so its single setter kind carries the whole root and inverts to itself with the base root.

use super::{NoteCompositeWindowTransient, NoteCompositeWindowTransientDiff, NoteCompositeWindowTransientMutation};

impl protocol::Mutation<NoteCompositeWindowTransient> for NoteCompositeWindowTransientMutation {
    type Diff = NoteCompositeWindowTransientDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
        semantic_kind: "set-window-transient",
        display_name: "Set Note Composite Window Transient",
        emoji: "🫧️",
        aggregate_variant: "Snapshot",
        payload_schema: "note.compositewindowtransient",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, base: &NoteCompositeWindowTransient) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::Snapshot { transient } => {
                let diff = NoteCompositeWindowTransientDiff {
                    engagement_input: (base.engagement_input != transient.engagement_input).then(|| transient.engagement_input.clone()),
                    ink_tool: (base.ink_tool != transient.ink_tool).then(|| crate::schema::diff::NoteAssigned::new(transient.ink_tool.clone())),
                };
                match protocol::DiffAlgebra::<NoteCompositeWindowTransient>::is_empty(&diff) {
                    true => protocol::MutationOutcome::empty().warning("mutation.no-op", "Window transient is unchanged."),
                    false => protocol::MutationOutcome::new(diff),
                }
            }
        }
    }
    fn inverse(&self, base: &NoteCompositeWindowTransient) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok(vec![Self::Snapshot { transient: base.clone() }])
}
}
