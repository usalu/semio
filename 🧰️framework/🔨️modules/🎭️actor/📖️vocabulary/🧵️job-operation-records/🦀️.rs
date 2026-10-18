//! 🧵️ Job operation records: the job operation, checkpoint, commit candidate, step outcome, turn and publication records. See [`super`].

use super::*;

//#region 🪪️JobBridge
/// 🪪️ Stable operation identity carried by every actor job turn and publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct JobOperation {
    pub operation: u64,
    pub base_revision: u64,
    pub generation: u64,
    pub preview_sequence: u64,
    pub seed: u64,
}

impl JobOperation {
    pub fn from_job(operation: semio_framework_job::Operation) -> Self {
        Self { operation: operation.operation.0, base_revision: operation.base_revision.0, generation: operation.generation.0, preview_sequence: operation.preview_sequence, seed: operation.seed }
    }

    pub fn into_job(self) -> semio_framework_job::Operation {
        semio_framework_job::Operation { operation: semio_framework_job::OperationId(self.operation), base_revision: semio_framework_job::RevisionId(self.base_revision), generation: semio_framework_job::Generation(self.generation), preview_sequence: self.preview_sequence, seed: self.seed }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        semio_framework_actor_pack::write_u64(out, self.operation).await;
        semio_framework_actor_pack::write_u64(out, self.base_revision).await;
        semio_framework_actor_pack::write_u64(out, self.generation).await;
        semio_framework_actor_pack::write_u64(out, self.preview_sequence).await;
        semio_framework_actor_pack::write_u64(out, self.seed).await;
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        Ok(Self {
            operation: semio_framework_actor_pack::read_u64(bytes, pos, "JobOperation::operation").await?,
            base_revision: semio_framework_actor_pack::read_u64(bytes, pos, "JobOperation::base_revision").await?,
            generation: semio_framework_actor_pack::read_u64(bytes, pos, "JobOperation::generation").await?,
            preview_sequence: semio_framework_actor_pack::read_u64(bytes, pos, "JobOperation::preview_sequence").await?,
            seed: semio_framework_actor_pack::read_u64(bytes, pos, "JobOperation::seed").await?,
        })
    }
}

/// 📸️ Opaque resumable state plus the committed progress boundary it represents.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct JobCheckpoint {
    pub state: Vec<u8>,
    pub applied_progress: u64,
}

impl JobCheckpoint {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        semio_framework_actor_pack::write_bytes(out, &self.state).await;
        semio_framework_actor_pack::write_u64(out, self.applied_progress).await;
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        Ok(Self { state: semio_framework_actor_pack::read_bytes(bytes, pos, "JobCheckpoint::state").await?, applied_progress: semio_framework_actor_pack::read_u64(bytes, pos, "JobCheckpoint::applied_progress").await? })
    }
}

/// 🏁️ Final persisted job state and its authoritative output candidate.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct JobCommitCandidate {
    pub state: Vec<u8>,
    pub output: Vec<u8>,
}

impl JobCommitCandidate {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        semio_framework_actor_pack::write_bytes(out, &self.state).await;
        semio_framework_actor_pack::write_bytes(out, &self.output).await;
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        Ok(Self { state: semio_framework_actor_pack::read_bytes(bytes, pos, "JobCommitCandidate::state").await?, output: semio_framework_actor_pack::read_bytes(bytes, pos, "JobCommitCandidate::output").await? })
    }
}

/// 🚦️ Lossless actor-wire mirror of one universal `StepOutcome`.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase"))]
#[value(crate = "::semio_framework_value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum JobStepOutcome {
    Yield,
    PreviewReady { preview: Vec<u8> },
    CheckpointReady { checkpoint: JobCheckpoint },
    Complete { candidate: JobCommitCandidate },
    Cancelled,
    Fault { detail: Vec<u8> },
}

impl JobStepOutcome {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            Self::Yield => semio_framework_actor_pack::write_u8(out, 0).await,
            Self::PreviewReady { preview } => {
                semio_framework_actor_pack::write_u8(out, 1).await;
                semio_framework_actor_pack::write_bytes(out, preview).await;
            }
            Self::CheckpointReady { checkpoint } => {
                semio_framework_actor_pack::write_u8(out, 2).await;
                checkpoint.pack_encode(out).await;
            }
            Self::Complete { candidate } => {
                semio_framework_actor_pack::write_u8(out, 3).await;
                candidate.pack_encode(out).await;
            }
            Self::Cancelled => semio_framework_actor_pack::write_u8(out, 4).await,
            Self::Fault { detail } => {
                semio_framework_actor_pack::write_u8(out, 5).await;
                semio_framework_actor_pack::write_bytes(out, detail).await;
            }
        }
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        match semio_framework_actor_pack::read_u8(bytes, pos, "JobStepOutcome").await? {
            0 => Ok(Self::Yield),
            1 => Ok(Self::PreviewReady { preview: semio_framework_actor_pack::read_bytes(bytes, pos, "JobStepOutcome::PreviewReady").await? }),
            2 => Ok(Self::CheckpointReady { checkpoint: JobCheckpoint::pack_decode(bytes, pos).await? }),
            3 => Ok(Self::Complete { candidate: JobCommitCandidate::pack_decode(bytes, pos).await? }),
            4 => Ok(Self::Cancelled),
            5 => Ok(Self::Fault { detail: semio_framework_actor_pack::read_bytes(bytes, pos, "JobStepOutcome::Fault").await? }),
            tag => Err(semio_framework_actor_pack::PackError::InvalidTag { what: "JobStepOutcome", tag, offset: *pos }),
        }
    }
}

/// 🎫️ One explicitly-addressed bounded job turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct JobTurn {
    pub job: u64,
    pub operation: JobOperation,
    pub step_sequence: u64,
}

impl JobTurn {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        semio_framework_actor_pack::write_u64(out, self.job).await;
        self.operation.pack_encode(out).await;
        semio_framework_actor_pack::write_u64(out, self.step_sequence).await;
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        Ok(Self { job: semio_framework_actor_pack::read_u64(bytes, pos, "JobTurn::job").await?, operation: JobOperation::pack_decode(bytes, pos).await?, step_sequence: semio_framework_actor_pack::read_u64(bytes, pos, "JobTurn::step_sequence").await? })
    }
}

/// 📡️ One validated, replay-addressable publication from a bounded job turn.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[value(crate = "::semio_framework_value")]
pub struct JobPublication {
    pub turn: JobTurn,
    pub outcome: JobStepOutcome,
}

impl JobPublication {
    pub fn turn_status(&self) -> TurnStatus {
        match &self.outcome {
            JobStepOutcome::Yield => TurnStatus::MoreWork,
            JobStepOutcome::PreviewReady { preview } => TurnStatus::PreviewReady { preview: preview.clone(), sequence: self.turn.operation.preview_sequence.saturating_sub(1) },
            JobStepOutcome::CheckpointReady { checkpoint } => TurnStatus::CheckpointReady { checkpoint: checkpoint.clone() },
            JobStepOutcome::Complete { candidate } => TurnStatus::CommitReady { candidate: candidate.clone() },
            JobStepOutcome::Cancelled => TurnStatus::Cancelled,
            JobStepOutcome::Fault { detail } => TurnStatus::Faulted { detail: detail.clone() },
        }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        self.turn.pack_encode(out).await;
        self.outcome.pack_encode(out).await;
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor_pack::PackError> {
        Ok(Self { turn: JobTurn::pack_decode(bytes, pos).await?, outcome: JobStepOutcome::pack_decode(bytes, pos).await? })
    }
}
//#endregion 🪪️JobBridge
