//! 🛂️ Semantic record sequencing delegates every identifier byte to the retained tagged-ID cursor.

use super::{MemberOpenDiagnostic, MemberOpenRequest};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Field {
    Document,
    Schema,
    Parent,
    Slot,
    Child,
    Kind,
    Standard,
    Subset,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage {
    Format,
    Presence,
    Id(Field),
    Done,
}

pub(super) struct SemanticRecord {
    kind: u8,
    stage: Stage,
    presence: u8,
    matched: bool,
}

impl SemanticRecord {
    pub(super) fn new(kind: u8) -> Self {
        Self { kind, stage: Stage::Format, presence: 0, matched: true }
    }
    pub(super) fn needs_id(&self) -> bool {
        matches!(self.stage, Stage::Id(_))
    }

    pub(super) fn push(&mut self, byte: u8, request: &MemberOpenRequest) -> Result<(), MemberOpenDiagnostic> {
        match self.stage {
            Stage::Format => {
                if byte != 1 {
                    return Err(MemberOpenDiagnostic::Malformed);
                }
                self.stage = if self.kind == 1 { Stage::Id(Field::Document) } else { Stage::Presence };
            }
            Stage::Presence => {
                if byte & !3 != 0 {
                    return Err(MemberOpenDiagnostic::Malformed);
                }
                self.presence = byte;
                if byte & 1 == 0 {
                    self.matched &= request.owner().is_none();
                }
                if byte & 2 == 0 {
                    self.matched = false;
                }
                self.stage = if byte & 1 != 0 { Stage::Id(Field::Parent) } else { self.dialect_stage() };
            }
            Stage::Id(_) | Stage::Done => return Err(MemberOpenDiagnostic::Malformed),
        }
        Ok(())
    }

    fn dialect_stage(&self) -> Stage {
        if self.presence & 2 != 0 {
            Stage::Id(Field::Kind)
        } else {
            Stage::Done
        }
    }

    pub(super) fn accept_id(&mut self, value: &str, request: &MemberOpenRequest, schema: &str) -> Result<(), MemberOpenDiagnostic> {
        let expected = request.admitted_expected()?;
        let Stage::Id(field) = self.stage else {
            return Err(MemberOpenDiagnostic::Stale);
        };
        self.matched &= match field {
            Field::Document => value == expected.artifact_id,
            Field::Schema => value == schema,
            Field::Parent => request.owner().is_some_and(|owner| {
                value.bytes().eq(owner
                    .parent
                    .artifact_id
                    .bytes()
                    .chain(b"!".iter().copied())
                    .chain(owner.parent.dialect.artifact_kind.bytes())
                    .chain(b"@".iter().copied())
                    .chain(owner.parent.dialect.standard.bytes())
                    .chain(b"/".iter().copied())
                    .chain(owner.parent.dialect.subset.bytes()))
            }),
            Field::Slot => request.owner().is_some_and(|owner| value == owner.slot),
            Field::Child => request.owner().is_some_and(|owner| value == owner.child_id),
            Field::Kind => value == expected.dialect.artifact_kind,
            Field::Standard => value == expected.dialect.standard,
            Field::Subset => value == expected.dialect.subset,
        };
        self.stage = match field {
            Field::Document => Stage::Id(Field::Schema),
            Field::Schema => Stage::Done,
            Field::Parent => Stage::Id(Field::Slot),
            Field::Slot => Stage::Id(Field::Child),
            Field::Child => self.dialect_stage(),
            Field::Kind => Stage::Id(Field::Standard),
            Field::Standard => Stage::Id(Field::Subset),
            Field::Subset => Stage::Done,
        };
        Ok(())
    }

    pub(super) fn finish(&self) -> Result<bool, MemberOpenDiagnostic> {
        if self.stage != Stage::Done {
            return Err(MemberOpenDiagnostic::Malformed);
        }
        Ok(self.matched)
    }
}
