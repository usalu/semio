//! 🛂️ Semantic record sequencing delegates every identifier byte to the retained tagged-ID cursor.

use super::{MemberHistoryDictionaryLimits, MemberOpenDiagnostic, MemberOpenRequest};

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
    Checkpoint,
    PinArtifact,
    PinCheckpoint,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stage {
    Format,
    Presence,
    Id(Field),
    Groups,
    Pins,
    Done,
}

pub(super) struct SemanticRecord {
    kind: u8,
    stage: Stage,
    presence: u8,
    number: u64,
    digits: u8,
    groups: u64,
    pins: u64,
    matched: bool,
}

impl SemanticRecord {
    pub(super) fn new(kind: u8) -> Self {
        Self { kind, stage: Stage::Format, presence: 0, number: 0, digits: 0, groups: 0, pins: 0, matched: true }
    }
    pub(super) fn needs_id(&self) -> bool {
        matches!(self.stage, Stage::Id(_))
    }

    pub(super) fn push(&mut self, byte: u8, request: &MemberOpenRequest, limits: MemberHistoryDictionaryLimits, groups: &mut u64, pins: &mut u64) -> Result<(), MemberOpenDiagnostic> {
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
            Stage::Groups | Stage::Pins => {
                if self.digits == 9 && byte > 1 {
                    return Err(MemberOpenDiagnostic::Malformed);
                }
                self.number |= u64::from(byte & 127) << (u32::from(self.digits) * 7);
                self.digits += 1;
                if byte < 128 {
                    if self.digits > 1 && byte == 0 {
                        return Err(MemberOpenDiagnostic::Malformed);
                    }
                    let value = self.number;
                    self.number = 0;
                    self.digits = 0;
                    if self.stage == Stage::Groups {
                        if value > limits.pin_groups - *groups {
                            return Err(MemberOpenDiagnostic::Capacity);
                        }
                        *groups += value;
                        self.groups = value;
                        self.stage = if value == 0 { Stage::Done } else { Stage::Id(Field::Checkpoint) };
                    } else {
                        if value > limits.pins - *pins {
                            return Err(MemberOpenDiagnostic::Capacity);
                        }
                        *pins += value;
                        self.pins = value;
                        if value == 0 {
                            self.finish_group();
                        } else {
                            self.stage = Stage::Id(Field::PinArtifact);
                        }
                    }
                }
            }
            Stage::Id(_) | Stage::Done => return Err(MemberOpenDiagnostic::Malformed),
        }
        Ok(())
    }

    fn dialect_stage(&self) -> Stage {
        if self.presence & 2 != 0 {
            Stage::Id(Field::Kind)
        } else {
            Stage::Groups
        }
    }
    fn finish_group(&mut self) {
        self.groups -= 1;
        self.stage = if self.groups == 0 { Stage::Done } else { Stage::Id(Field::Checkpoint) };
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
            Field::Checkpoint | Field::PinArtifact | Field::PinCheckpoint => true,
        };
        self.stage = match field {
            Field::Document => Stage::Id(Field::Schema),
            Field::Schema => Stage::Done,
            Field::Parent => Stage::Id(Field::Slot),
            Field::Slot => Stage::Id(Field::Child),
            Field::Child => self.dialect_stage(),
            Field::Kind => Stage::Id(Field::Standard),
            Field::Standard => Stage::Id(Field::Subset),
            Field::Subset => Stage::Groups,
            Field::Checkpoint => Stage::Pins,
            Field::PinArtifact => Stage::Id(Field::PinCheckpoint),
            Field::PinCheckpoint => {
                self.pins -= 1;
                if self.pins == 0 {
                    self.finish_group();
                    self.stage
                } else {
                    Stage::Id(Field::PinArtifact)
                }
            }
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
