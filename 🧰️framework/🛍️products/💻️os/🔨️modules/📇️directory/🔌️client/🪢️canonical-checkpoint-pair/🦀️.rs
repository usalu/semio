//! 🪢️ One document's active canonical checkpoint pair as the hub serves it — `GET
//! /spaces/{space}/documents/{document}/active-checkpoint/pair` with `Accept:
//! application/vnd.semio.canonical-checkpoint-pair.v1` — the ONE seed a native, a wasm32 and a React shell open a
//! hub document on. The native twin of the browser worker's `seedColdPairFromCanonicalCheckpoint`
//! (`🏪️store/👷️worker/🟦️.ts`): the answer is bounded before it is decoded, its digests are verified, and it is
//! admitted only as the checkpoint the hub authorized for this open.

use super::super::schema::{decode_canonical_checkpoint_pair_v1, CanonicalCheckpointPairV1, DocumentOpenCheckpointV1, DocumentScope, CANONICAL_CHECKPOINT_PAIR_MAX_WIRE_BYTES, CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1};
use super::{encode_url_component, execution_target_refusal, DirectoryClient, DirectoryClientError, DirectoryTransport};
use semio_framework_async::OperationContext;

/// 🔁️ How often a pair request the hub refused transiently (busy, unavailable) is asked again before the open fails.
pub const CANONICAL_CHECKPOINT_PAIR_TRANSIENT_ATTEMPTS: u32 = 3;

/// ⏳️ Statuses that mean "the hub could not answer now", never "this pair is wrong" — React's
/// `CANONICAL_CHECKPOINT_PAIR_TRANSIENT_STATUSES`.
pub const CANONICAL_CHECKPOINT_PAIR_TRANSIENT_STATUSES: [u16; 7] = [408, 425, 429, 500, 502, 503, 504];

/// 🛤️ The exact route of one document's active pair.
pub fn canonical_checkpoint_pair_path(scope: &DocumentScope) -> String {
    format!("/spaces/{}/documents/{}/active-checkpoint/pair", encode_url_component(&scope.space_id), encode_url_component(&scope.document_id))
}

impl<T: DirectoryTransport> DirectoryClient<T> {
    /// 🪢️ Fetches, verifies and admits the document's active canonical checkpoint pair against `expected` (the
    /// execution-target lease's `checkpoint`). A transient refusal is asked again up to
    /// [`CANONICAL_CHECKPOINT_PAIR_TRANSIENT_ATTEMPTS`] times while `ctx` is live; any other refusal is final.
    pub async fn document_canonical_checkpoint_pair(&self, ctx: &OperationContext, scope: &DocumentScope, expected: &DocumentOpenCheckpointV1) -> Result<CanonicalCheckpointPairV1, DirectoryClientError> {
        let url = self.url(&canonical_checkpoint_pair_path(scope));
        let mut attempt = 0;
        let response = loop {
            if ctx.cancel.is_cancelled().await {
                return Err(DirectoryClientError::Cancelled);
            }
            let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose()?;
            let response = self.transport.get_accepting(ctx, &url, bearer, CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1).await?;
            attempt += 1;
            if !CANONICAL_CHECKPOINT_PAIR_TRANSIENT_STATUSES.contains(&response.status) || attempt >= CANONICAL_CHECKPOINT_PAIR_TRANSIENT_ATTEMPTS {
                break response;
            }
        };
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        match response.status {
            401 => return Err(DirectoryClientError::Unauthorized),
            200 => {}
            status => return Err(DirectoryClientError::Http { status, body: execution_target_refusal(&response.body) }),
        }
        if response.body.len() > CANONICAL_CHECKPOINT_PAIR_MAX_WIRE_BYTES {
            return Err(DirectoryClientError::Decode("canonical-checkpoint-pair.oversized".into()));
        }
        let pair = decode_canonical_checkpoint_pair_v1(&response.body).map_err(|refusal| DirectoryClientError::Decode(refusal.code().into()))?;
        pair.admit(scope, expected).map_err(|refusal| DirectoryClientError::Decode(refusal.code().into()))?;
        Ok(pair)
    }
}
