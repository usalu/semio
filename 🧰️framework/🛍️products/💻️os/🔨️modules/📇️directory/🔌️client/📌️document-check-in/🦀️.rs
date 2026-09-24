//! 📌️ One document's Check In routes as the hub serves them — `POST
//! /spaces/{space}/documents/{document}/check-ins`, `GET …/{request}`, `POST …/{request}/cancel` — over
//! the schema-owned `DocumentCheckIn*V1` contract (`🧬️schema/📌️document-check-in-v1`). The native twin
//! of the browser worker's Check In owner (`🏪️store/👷️worker/🟦️.ts` `driveDocumentCheckIn`): every
//! answer is bounded before it is decoded, must be canonical, and must name the request it was asked for.

use super::super::schema::document_check_in::{DocumentCheckInStatusV1, DocumentCheckInV1, DOCUMENT_CHECK_IN_MAX_BYTES};
use super::{encode_url_component, DirectoryClient, DirectoryClientError, DirectoryTransport, HttpMethod};
use semio_framework_async::OperationContext;

fn request_id_valid(value: &str) -> bool {
    value.len() == 32 && value.bytes().any(|byte| byte != b'0') && value.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

fn check_in_path(space_id: &str, document_id: &str, suffix: &str) -> Option<String> {
    if space_id.is_empty() || document_id.is_empty() {
        return None;
    }
    Some(format!("/spaces/{}/documents/{}/check-ins{suffix}", encode_url_component(space_id), encode_url_component(document_id)))
}

impl<T: DirectoryTransport> DirectoryClient<T> {
    /// 📌️ Submits one Check In of an acknowledged head and answers the hub's first status for it.
    pub async fn document_check_in(&self, ctx: &OperationContext, space_id: &str, request: &DocumentCheckInV1) -> Result<DocumentCheckInStatusV1, DirectoryClientError> {
        let body = request.canonical_json().ok_or_else(|| DirectoryClientError::Decode("document check-in request is invalid".into()))?.into_bytes();
        let path = check_in_path(space_id, &request.head.document_id, "").ok_or_else(|| DirectoryClientError::Decode("document check-in route is not addressable".into()))?;
        let answer = self.document_check_in_exchange(ctx, HttpMethod::Post, &path, Some(body)).await?;
        document_check_in_status(&answer, &request.request_id)
    }

    /// 🚦️ One progress read of an accepted Check In.
    pub async fn document_check_in_status(&self, ctx: &OperationContext, space_id: &str, document_id: &str, request_id: &str) -> Result<DocumentCheckInStatusV1, DirectoryClientError> {
        let path = request_id_valid(request_id).then(|| check_in_path(space_id, document_id, &format!("/{request_id}"))).flatten().ok_or_else(|| DirectoryClientError::Decode("document check-in route is not addressable".into()))?;
        let answer = self.document_check_in_exchange(ctx, HttpMethod::Get, &path, None).await?;
        document_check_in_status(&answer, request_id)
    }

    /// 🛑️ Asks the hub to stop an accepted Check In at its next checkpoint.
    pub async fn cancel_document_check_in(&self, ctx: &OperationContext, space_id: &str, document_id: &str, request_id: &str) -> Result<DocumentCheckInStatusV1, DirectoryClientError> {
        let path = request_id_valid(request_id).then(|| check_in_path(space_id, document_id, &format!("/{request_id}/cancel"))).flatten().ok_or_else(|| DirectoryClientError::Decode("document check-in route is not addressable".into()))?;
        let answer = self.document_check_in_exchange(ctx, HttpMethod::Post, &path, None).await?;
        document_check_in_status(&answer, request_id)
    }

    async fn document_check_in_exchange(&self, ctx: &OperationContext, method: HttpMethod, path: &str, body: Option<Vec<u8>>) -> Result<String, DirectoryClientError> {
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose()?;
        let response = self.transport.http(ctx, method, &self.url(path), bearer, body).await?;
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        match response.status {
            200..=299 => {}
            401 => return Err(DirectoryClientError::Unauthorized),
            status => return Err(DirectoryClientError::Http { status, body: String::new() }),
        }
        if response.body.len() > DOCUMENT_CHECK_IN_MAX_BYTES {
            return Err(DirectoryClientError::Decode("document check-in answer exceeds its bound".into()));
        }
        String::from_utf8(response.body).map_err(|error| DirectoryClientError::Decode(error.to_string()))
    }
}

fn document_check_in_status(answer: &str, request_id: &str) -> Result<DocumentCheckInStatusV1, DirectoryClientError> {
    let status = DocumentCheckInStatusV1::parse_canonical_json(answer).ok_or_else(|| DirectoryClientError::Decode("document check-in status is not canonical".into()))?;
    if status.request_id != request_id {
        return Err(DirectoryClientError::Decode("document check-in status names another request".into()));
    }
    Ok(status)
}
