//! 🧒️ Bounded borrowed child restore admission independent of Store lifecycle.

/// 📏️ Fixed restore admission bounds; sparse container walks consume their own step budget.
pub const CHILD_RESTORE_MAXIMUM_REFERENCES: usize = 64;
pub const CHILD_RESTORE_MAXIMUM_FIELD_BYTES: usize = 256;
pub const CHILD_RESTORE_MAXIMUM_VISIT_STEPS: usize = 256;

/// 🛑️ Exact loaded-parent projection failures, before opening or publishing any child store.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildRestoreProjectionError {
    InvalidSlot,
    InvalidReference,
    DuplicateChild,
    SingularSlot,
    ReferenceLimit,
    TraversalLimit,
    IncompleteSet,
}

impl std::fmt::Display for ChildRestoreProjectionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "child restore projection: {self:?}")
    }
}

impl std::error::Error for ChildRestoreProjectionError {}

/// 🌳️ Bounded borrowed authority derived only from the loaded parent's declared child fields.
#[derive(Debug)]
pub struct ChildRestoreProjection<'a> {
    slots: &'static [semio_framework_schema_composition::ChildSlotSpec],
    rows: [Option<(&'static str, semio_framework_schema_composition::ChildRefFields<'a>)>; CHILD_RESTORE_MAXIMUM_REFERENCES],
    length: usize,
    steps: usize,
}

impl<'a> ChildRestoreProjection<'a> {
    /// 🔎️ Visits actual typed snapshot fields without cloning handles, leases or child content.
    pub fn from_snapshot<S: semio_framework_schema_composition::ArtifactCompositionFields>(snapshot: &'a S) -> Result<Self, ChildRestoreProjectionError> {
        let slots = S::child_slots();
        if slots.len() > CHILD_RESTORE_MAXIMUM_REFERENCES {
            return Err(ChildRestoreProjectionError::ReferenceLimit);
        }
        for (index, slot) in slots.iter().enumerate() {
            if slot.name.is_empty()
                || slot.name.len() > CHILD_RESTORE_MAXIMUM_FIELD_BYTES
                || slot.selection.id().len() > CHILD_RESTORE_MAXIMUM_FIELD_BYTES
                || !match slot.selection { semio_framework_schema_composition::ChildSlotSelection::ArtifactKind(kind) => semio_framework_artifact_reference_borrow::is_canonical_artifact_kind(kind), semio_framework_schema_composition::ChildSlotSelection::Capability(capability) => semio_framework_schema_composition::is_canonical_child_capability(capability) }
                || slots[..index].iter().any(|prior| prior.name == slot.name)
            {
                return Err(ChildRestoreProjectionError::InvalidSlot);
            }
        }
        let mut projection = Self { slots, rows: [None; CHILD_RESTORE_MAXIMUM_REFERENCES], length: 0, steps: 0 };
        snapshot.visit_child_refs(&mut projection)?;
        Ok(projection)
    }

    /// 🧮️ Number of exact child references carried by this loaded parent.
    pub fn len(&self) -> usize {
        self.length
    }

    /// 🍃️ Whether this projection has no populated child fields.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// 🌿️ Reads one admitted typed child row without copying handles or payload owners.
    pub fn get(&self, index: usize) -> Option<(&'static str, semio_framework_schema_composition::ChildRefFields<'a>)> {
        self.rows.get(index).copied().flatten()
    }

    /// 🪪️ Checks one member against the actual loaded parent, without treating it as a full batch.
    pub fn admits_member(&self, slot: &str, expected: semio_framework_artifact_reference_borrow::BorrowedArtifactRef<'_>) -> bool {
        self.rows[..self.length]
            .iter()
            .flatten()
            .any(|(name, fields)| *name == slot && fields.artifact_id == expected.artifact_id && fields.artifact_kind == expected.artifact_kind && fields.standard == expected.standard && fields.subset == expected.subset)
    }

    /// ⚖️ Admits exactly one unordered complete set: no omission, substitution or duplicate.
    pub fn admit_complete<'b>(&self, incoming: impl IntoIterator<Item = (&'b str, semio_framework_schema_composition::ChildRefFields<'b>)>) -> Result<(), ChildRestoreProjectionError> {
        let mut seen = [false; CHILD_RESTORE_MAXIMUM_REFERENCES];
        let mut count = 0;
        for (slot, fields) in incoming {
            if count == CHILD_RESTORE_MAXIMUM_REFERENCES {
                return Err(ChildRestoreProjectionError::ReferenceLimit);
            }
            let index = self.rows[..self.length].iter().position(|entry| entry.as_ref().is_some_and(|(name, expected)| *name == slot && *expected == fields)).ok_or(ChildRestoreProjectionError::IncompleteSet)?;
            if seen[index] {
                return Err(ChildRestoreProjectionError::DuplicateChild);
            }
            seen[index] = true;
            count += 1;
        }
        if count != self.length {
            return Err(ChildRestoreProjectionError::IncompleteSet);
        }
        Ok(())
    }
}

impl<'a> semio_framework_schema_composition::ChildRefVisitor<'a> for ChildRestoreProjection<'a> {
    type Error = ChildRestoreProjectionError;
    fn step(&mut self) -> Result<(), Self::Error> {
        if self.steps == CHILD_RESTORE_MAXIMUM_VISIT_STEPS {
            return Err(Self::Error::TraversalLimit);
        }
        self.steps += 1;
        Ok(())
    }
    fn child(&mut self, slot: &'static str, fields: semio_framework_schema_composition::ChildRefFields<'a>) -> Result<(), Self::Error> {
        if self.length == CHILD_RESTORE_MAXIMUM_REFERENCES {
            return Err(Self::Error::ReferenceLimit);
        }
        let spec = self.slots.iter().find(|spec| spec.name == slot).ok_or(Self::Error::InvalidSlot)?;
        if !spec.selection.accepts_kind(fields.artifact_kind) || !semio_framework_artifact_reference_borrow::is_canonical_artifact_kind(fields.artifact_kind) || [fields.child_id, fields.artifact_id, fields.artifact_kind, fields.standard, fields.subset].iter().any(|field| field.is_empty() || field.len() > CHILD_RESTORE_MAXIMUM_FIELD_BYTES) {
            return Err(Self::Error::InvalidReference);
        }
        for (name, prior) in self.rows[..self.length].iter().flatten() {
            if prior.child_id == fields.child_id || prior.artifact_id == fields.artifact_id {
                return Err(Self::Error::DuplicateChild);
            }
            if *name == slot && !spec.many {
                return Err(Self::Error::SingularSlot);
            }
        }
        self.rows[self.length] = Some((slot, fields));
        self.length += 1;
        Ok(())
    }
}

