//! 🌱️ A space's artifact-creation routes as the hub serves them — `GET|POST
//! /spaces/{space}/artifact-creations`, `GET …/{request}`, `POST …/{request}/cancel` — over the
//! schema-owned `SpaceArtifactCreation*V1` contract (`🧬️schema/🌱️space-artifact-creation-v1`). The
//! native twin of the browser worker's creation owner (`🏪️store/👷️worker/🟦️.ts`
//! `driveSpaceArtifactCreation`): every answer is bounded before it is decoded, must be canonical, and
//! must name exactly the space and request it was asked for.

use super::super::schema::space_artifact_creation::{SpaceArtifactCreateV1, SpaceArtifactCreationCatalogV1, SpaceArtifactCreationStatusV1, SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES, SPACE_ARTIFACT_CREATION_MAX_BYTES};
use super::{encode_url_component, DirectoryClient, DirectoryClientError, DirectoryTransport, HttpMethod};
use semio_framework_async::OperationContext;

/// 🧭️ The three addresses a creation owner uses under one space's collection.
#[derive(Clone, Copy)]
enum SpaceArtifactCreationRoute<'a> {
    Collection,
    Request(&'a str),
    Cancel(&'a str),
}

impl SpaceArtifactCreationRoute<'_> {
    fn path(self, space_id: &str) -> Option<String> {
        let request_id = |value: &str| value.len() == 32 && value.bytes().any(|byte| byte != b'0') && value.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'));
        if space_id.is_empty() || encode_url_component(space_id) != space_id {
            return None;
        }
        match self {
            Self::Collection => Some(format!("/spaces/{space_id}/artifact-creations")),
            Self::Request(id) => request_id(id).then(|| format!("/spaces/{space_id}/artifact-creations/{id}")),
            Self::Cancel(id) => request_id(id).then(|| format!("/spaces/{space_id}/artifact-creations/{id}/cancel")),
        }
    }
}

impl<T: DirectoryTransport> DirectoryClient<T> {
    /// 🗂️ The kinds one space's selected current catalog can create, with their en + de labels.
    pub async fn space_artifact_creation_catalog(&self, ctx: &OperationContext, space_id: &str) -> Result<SpaceArtifactCreationCatalogV1, DirectoryClientError> {
        let body = self.space_artifact_creation_exchange(ctx, HttpMethod::Get, space_id, SpaceArtifactCreationRoute::Collection, None, SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES).await?;
        let catalog = SpaceArtifactCreationCatalogV1::parse_canonical_json(&body).ok_or_else(|| DirectoryClientError::Decode("space artifact creation catalog is not canonical".into()))?;
        if catalog.space_id != space_id {
            return Err(DirectoryClientError::Decode("space artifact creation catalog names another space".into()));
        }
        Ok(catalog)
    }

    /// 📥️ Submits one creation intent and answers the hub's first receipt for it. A `409` means the
    /// intent named a catalog generation that is no longer current: re-read the catalog, never retry.
    pub async fn create_space_artifact(&self, ctx: &OperationContext, space_id: &str, request: &SpaceArtifactCreateV1) -> Result<SpaceArtifactCreationStatusV1, DirectoryClientError> {
        if !request.validate() {
            return Err(DirectoryClientError::Decode("space artifact creation intent is invalid".into()));
        }
        let body = crate::os_pack::json::to_json_string(request).into_bytes();
        let receipt = self.space_artifact_creation_exchange(ctx, HttpMethod::Post, space_id, SpaceArtifactCreationRoute::Collection, Some(body), SPACE_ARTIFACT_CREATION_MAX_BYTES).await?;
        space_artifact_creation_status(&receipt, space_id, &request.request_id)
    }

    /// 🚦️ One progress read of an accepted creation.
    pub async fn space_artifact_creation_status(&self, ctx: &OperationContext, space_id: &str, request_id: &str) -> Result<SpaceArtifactCreationStatusV1, DirectoryClientError> {
        let receipt = self.space_artifact_creation_exchange(ctx, HttpMethod::Get, space_id, SpaceArtifactCreationRoute::Request(request_id), None, SPACE_ARTIFACT_CREATION_MAX_BYTES).await?;
        space_artifact_creation_status(&receipt, space_id, request_id)
    }

    /// 🛑️ Asks the hub to cancel an accepted creation; its receipt says whether the cancel won.
    pub async fn cancel_space_artifact_creation(&self, ctx: &OperationContext, space_id: &str, request_id: &str) -> Result<SpaceArtifactCreationStatusV1, DirectoryClientError> {
        let receipt = self.space_artifact_creation_exchange(ctx, HttpMethod::Post, space_id, SpaceArtifactCreationRoute::Cancel(request_id), None, SPACE_ARTIFACT_CREATION_MAX_BYTES).await?;
        space_artifact_creation_status(&receipt, space_id, request_id)
    }

    async fn space_artifact_creation_exchange(&self, ctx: &OperationContext, method: HttpMethod, space_id: &str, route: SpaceArtifactCreationRoute<'_>, body: Option<Vec<u8>>, maximum_bytes: usize) -> Result<String, DirectoryClientError> {
        let path = route.path(space_id).ok_or_else(|| DirectoryClientError::Decode("space artifact creation route is not addressable".into()))?;
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose()?;
        let response = self.transport.http(ctx, method, &self.url(&path), bearer, body).await?;
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        match response.status {
            200..=299 => {}
            401 => return Err(DirectoryClientError::Unauthorized),
            status => return Err(DirectoryClientError::Http { status, body: String::new() }),
        }
        if response.body.len() > maximum_bytes {
            return Err(DirectoryClientError::Decode("space artifact creation answer exceeds its bound".into()));
        }
        String::from_utf8(response.body).map_err(|error| DirectoryClientError::Decode(error.to_string()))
    }
}

fn space_artifact_creation_status(receipt: &str, space_id: &str, request_id: &str) -> Result<SpaceArtifactCreationStatusV1, DirectoryClientError> {
    let status = SpaceArtifactCreationStatusV1::parse_canonical_json(receipt).ok_or_else(|| DirectoryClientError::Decode("space artifact creation receipt is not canonical".into()))?;
    if status.space_id != space_id || status.request_id != request_id {
        return Err(DirectoryClientError::Decode("space artifact creation receipt names another request".into()));
    }
    Ok(status)
}
