//! 📑️ Jack snapshot clone: the retained snapshot clone authority and its step machine.

use super::*;
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll};

//#region 🔖️RetainedStoreInitialization
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JackSnapshotCloneStep {
    Pending { copied_bytes: usize },
    Complete,
}

pub struct JackSnapshotCloneAuthority {
    pub value: std::mem::ManuallyDrop<Option<JackSnapshot>>,
    pub retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub phase: u8,
    retain_local_owner: bool,
    pub terminal: bool,
}

impl Default for JackSnapshotCloneAuthority {
    fn default() -> Self {
        Self::new()
    }
}

impl JackSnapshotCloneAuthority {
    pub fn new() -> Self {
        Self::with_local_owner(true)
    }

    pub fn metadata_only() -> Self {
        Self::with_local_owner(false)
    }

    fn with_local_owner(retain_local_owner: bool) -> Self {
        let content = store::os_store::artifact_child::ArtifactChild::new(String::new(), semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } });
        Self {
            value: std::mem::ManuallyDrop::new(Some(JackSnapshot { schema: String::new(), name: String::new(), manifest_id: String::new(), camera: Default::default(), content, root_node_id: None, query: String::new() })),
            retirement: std::mem::ManuallyDrop::new(None),
            phase: 0,
            retain_local_owner,
            terminal: false,
        }
    }

    fn clone_string(source: &str, maximum_bytes: usize) -> Result<String, ValueError> {
        if source.len() > maximum_bytes {
            return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"jack-store.initializer-field-too-large"));
        }
        let mut value = String::new();
        value.try_reserve_exact(source.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"jack-store.initializer-string-admission"))?;
        value.push_str(source);
        Ok(value)
    }

    pub fn advance(&mut self, source: &JackSnapshot, maximum_bytes: usize) -> Result<JackSnapshotCloneStep, ValueError> {
        if maximum_bytes == 0 {
            return Ok(JackSnapshotCloneStep::Pending { copied_bytes: 0 });
        }
        let target = self.value.as_mut().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"jack-store.initializer-clone-target"))?;
        let copied_bytes = match self.phase {
            0 => {
                target.schema = Self::clone_string(&source.schema, maximum_bytes)?;
                source.schema.len()
            }
            1 => {
                target.name = Self::clone_string(&source.name, maximum_bytes)?;
                source.name.len()
            }
            2 => {
                target.manifest_id = Self::clone_string(&source.manifest_id, maximum_bytes)?;
                source.manifest_id.len()
            }
            3 => {
                target.camera = source.camera.clone();
                0
            }
            4 => {
                target.content.child_id = Self::clone_string(&source.content.child_id, maximum_bytes)?;
                if self.retain_local_owner {
                    if let Some(owner) = source.content.local_owner::<crate::standards::v1::subsets::any::schema::content::JackContentOwner>() {
                        target.content.set_local_owner(owner);
                    }
                }
                source.content.child_id.len()
            }
            5 => {
                target.content.target.artifact_id = Self::clone_string(&source.content.target.artifact_id, maximum_bytes)?;
                source.content.target.artifact_id.len()
            }
            6 => {
                target.content.target.dialect.artifact_kind = Self::clone_string(&source.content.target.dialect.artifact_kind, maximum_bytes)?;
                source.content.target.dialect.artifact_kind.len()
            }
            7 => {
                target.content.target.dialect.standard = Self::clone_string(&source.content.target.dialect.standard, maximum_bytes)?;
                source.content.target.dialect.standard.len()
            }
            8 => {
                target.content.target.dialect.subset = Self::clone_string(&source.content.target.dialect.subset, maximum_bytes)?;
                source.content.target.dialect.subset.len()
            }
            9 => {
                target.root_node_id = source.root_node_id.as_deref().map(|value| Self::clone_string(value, maximum_bytes)).transpose()?;
                source.root_node_id.as_deref().map_or(0, str::len)
            }
            10 => {
                target.query = Self::clone_string(&source.query, maximum_bytes)?;
                source.query.len()
            }
            _ => {
                self.terminal = true;
                return Ok(JackSnapshotCloneStep::Complete);
            }
        };
        self.phase += 1;
        Ok(JackSnapshotCloneStep::Pending { copied_bytes })
    }

    pub fn step(&mut self, source: &JackSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, ValueError> {
        let step = self.advance(source, JACK_OWNED_FIELD_BYTES)?;
        match step {
            JackSnapshotCloneStep::Pending { copied_bytes } => {
                cx.consume_fuel(copied_bytes.max(1) as u64);
                Ok(false)
            }
            JackSnapshotCloneStep::Complete => Ok(true),
        }
    }

    pub fn take_value(&mut self) -> Option<JackSnapshot> {
        if !self.terminal {
            return None;
        }
        self.value.take()
    }

    /// 📏️ Quotes the next close turn of the partial clone: the retained child cursor, else the partial snapshot.
    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(owner) = self.retirement.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(owner, body);
        }
        if self.value.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.value);
        }
        Ok(Default::default())
    }

    pub fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        use store::os_store::snapshot_read::{artifact_retirement_admit_owned, artifact_retirement_box_close_step};
        if self.retirement.is_some() {
            return artifact_retirement_box_close_step(&mut self.retirement, effect.grant().maximum_copy_bytes, effect);
        }
        let turn = if self.value.is_some() {
            artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, effect)
        } else {
            self.terminal = true;
            RetainedWorkOutcome { result: Ok(RetainedCloneStep::Complete(Default::default())), work: 0 }
        };
        RetainedWorkPoll::Complete(turn).map_err(Into::into)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for JackSnapshotCloneAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "Jack snapshot clone reached Drop before exact handoff or cursor retirement");
    }
}
//#endregion 🔖️RetainedStoreInitialization
