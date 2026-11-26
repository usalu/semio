//! 🪆️ The Semio Model pane exposes its original read and one native insert-element command.

use super::SEMIO_MODEL_DIALECT;
use crate::schema::{mutations::SemioModelMutation, snapshot::SemioModelSnapshot};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, ValueError, retained_clone::retained_clone_grant_admits};
use semio_plugin::app::{
    ChildEmitPreparation,
    child_editor::{registration::ChildEditorRegistration, source::ChildEditorCapability},
    child_read::{ChildReadCapability, ChildReadRegistration, command::OriginalChildEditorCommandAuthority},
};

pub static SEMIO_MODEL_CHILD_READ: ChildReadRegistration<SemioModelSnapshot> = ChildReadRegistration::new(ChildReadCapability {
    artifact_kind: SEMIO_MODEL_DIALECT.artifact_kind,
    standard: SEMIO_MODEL_DIALECT.standard.0,
    subset: SEMIO_MODEL_DIALECT.subset.0,
});

pub static SEMIO_MODEL_CHILD_INSERT_ELEMENT: ChildEditorRegistration<SemioModelSnapshot> = ChildEditorRegistration::new(ChildEditorCapability {
    artifact_kind: SEMIO_MODEL_DIALECT.artifact_kind,
    standard: SEMIO_MODEL_DIALECT.standard.0,
    subset: SEMIO_MODEL_DIALECT.subset.0,
    editor_id: "s.stdio.semio@v1/model#editor",
    command_id: "insertElement",
});

/// 🧱️ Admits the Model insertion with its own issued authority and exactly one native leaf.
pub fn admit_semio_model_insert_element(
    slot: &str,
    child: &str,
    mutation: &mut Option<SemioModelMutation>,
    authority: &mut Option<OriginalChildEditorCommandAuthority>,
    grant: RetainedCloneGrant,
) -> Result<Option<(ChildEmitPreparation, RetainedCloneProgress)>, ValueError> {
    let demand = ChildEmitPreparation::owned_qualified_birth_demand::<SemioModelMutation>(slot.len(), child.len())?;
    if !retained_clone_grant_admits(grant, demand) {
        return Ok(None);
    }
    if !matches!(mutation, Some(SemioModelMutation::InsertElement(_))) || authority.as_ref().and_then(OriginalChildEditorCommandAuthority::stamp)
        .is_none_or(|stamp| stamp.capability_identity() != &SEMIO_MODEL_CHILD_INSERT_ELEMENT.identity as *const _ as usize)
    {
        return Err(ValueError::invariant("Semio Model insertion requires its original command authority and insert-element leaf"));
    }
    ChildEmitPreparation::admit_owned_qualified::<SemioModelSnapshot, SemioModelMutation>(slot, child, mutation, authority, grant)
}
