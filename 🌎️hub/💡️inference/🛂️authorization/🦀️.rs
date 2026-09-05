//! 🛂️ Rechecks durable Author membership; a successful read is not a retained submission grant.

use super::{schema::InferenceIdentityV1, InferenceErrorV1, InferenceOperationControlV1};
use crate::directory::{
    model::{SocketSessionBindingStatus, SpaceRole},
    HubDirectories, HubDirectory,
};
use directory::os_directory::DocumentScope;

pub(crate) async fn check_live_inference_author(directory: &HubDirectories, identity: &InferenceIdentityV1, scope: &DocumentScope, now: impl Fn() -> i64, control: &InferenceOperationControlV1) -> Result<(), InferenceErrorV1> {
    control.checkpoint(0)?;
    identity.validate()?;
    let now_ms = now();
    if now_ms < 0 || identity.space_id != scope.space_id || identity.document_id != scope.document_id {
        return Err(InferenceErrorV1::Denied);
    }
    let binding = tokio::select! {
        biased;
        error = control.interruption() => return Err(error),
        result = directory.socket_session_binding(&identity.session_id, &identity.user_id, identity.authorization_generation, Some(&scope.space_id), now_ms) => result.map_err(|_| InferenceErrorV1::Storage)?,
    };
    let returned_at_ms = now();
    control.checkpoint(1)?;
    if returned_at_ms < now_ms {
        return Err(InferenceErrorV1::Denied);
    }
    match binding {
        SocketSessionBindingStatus::Active { role: Some(SpaceRole::Author), expires_at_ms } if expires_at_ms > returned_at_ms => Ok(()),
        SocketSessionBindingStatus::Unavailable => Err(InferenceErrorV1::Storage),
        _ => Err(InferenceErrorV1::Denied),
    }
}

#[cfg(all(test, feature = "sqlite"))]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
