//! ⏳️ Live pending reprojection owners for the replay runtime.
use super::{EditReplay,ReplayProgress};
use protocol::mutation::Mutation as MutationContract;

/// 🐢️ A history change this store admitted but has not adopted yet (design §16.6): the remote `transitions` it admitted
/// ([`ArtifactStore::defer_remote_replays`]) and at most one `local` step this replica authored
/// ([`ArtifactStore::defer_local_replays`]) wait outside the log while their Report replay steps turn by turn; the replay
/// restarts whenever the history it started from changed (the content revision it names).
pub struct PendingReprojection<P, Mutation>
where
    Mutation: MutationContract<P>,
{
    pub transitions: Vec<semio_framework_os_spr::MutationEnvelope>,
    pub local: Option<PendingLocalStep>,
    pub replay: Option<([u8; 32], EditReplay<P, Mutation>)>,
}

impl<P, Mutation> PendingReprojection<P, Mutation>
where
    Mutation: MutationContract<P>,
{
    /// 🫗️ Nothing admitted yet.
    pub fn empty() -> Self {
        Self { transitions: Vec::new(), local: None, replay: None }
    }
}

/// 🪃️ A local history step whose Report replay outlasts its dispatch: the transitions it authored (neither recorded nor
/// announced yet), the head `(line, checkpoint)` it moves this replica to, and whether it finalizes a supersession — whose
/// report must not block finalizing, or the step is refused with nothing of it recorded.
pub struct PendingLocalStep {
    pub envelopes: Vec<semio_framework_os_spr::MutationEnvelope>,
    pub head: (Option<String>, Option<String>),
    pub finalizing: bool,
}

/// ⚡️ A local undo or redo that changes nothing but the applied tail ([`ArtifactStore::tail_step`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TailStep {
    Undo,
    Redo,
}

/// ⏩️ Where one advance of a deferred reprojection left it.
pub enum ReprojectionAdvance {
    Pending(ReplayProgress),
    Adopted(Vec<semio_framework_os_spr::EditMessages>),
}

