//! 📡️ Directory client: the client record over a directory transport with its commands, space and administration reads, event paging, session and document socket calls and the hub socket grant source implementations. See [`super`].

use super::*;

//#region 🔖️Client
pub struct DirectoryClient<T: DirectoryTransport> {
    pub(super) transport: T,
    pub(super) base_url: String,
    pub(super) credential: Option<Arc<LocalHubCredential>>,
}

/// 🧾️ The first bytes of an execution-target refusal body — the hub's closed error code, never document
/// data — so a shell can say why a route refused instead of only its status.
pub(super) fn execution_target_refusal(body: &[u8]) -> String {
    String::from_utf8_lossy(&body[..body.len().min(256)]).into_owned()
}

impl<T: DirectoryTransport> DirectoryClient<T> {
    pub fn new(transport: T, base_url: impl Into<String>) -> Self {
        Self { transport, base_url: base_url.into(), credential: None }
    }

    pub fn authenticated(transport: T, credential: Arc<LocalHubCredential>) -> Self {
        Self { transport, base_url: credential.hub_origin.clone(), credential: Some(credential) }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub(super) fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url.trim_end_matches('/'), path)
    }

    /// 🛑️ Checked BEFORE every call reaches `self.transport` — an already-cancelled `ctx` never
    /// builds a request at all (`TransportError::Cancelled`, via the SAME transport call, is the
    /// other half: cancelled WHILE the call was in flight — see that variant's own doc).
    async fn request_json<R: FromValue>(&self, ctx: &OperationContext, method: HttpMethod, path: &str, body: Option<Vec<u8>>) -> Result<R, DirectoryClientError> {
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose()?;
        let response = self.transport.http(ctx, method, &self.url(path), bearer, body).await?;
        match response.status {
            200..=299 => Ok(decode_json_bytes(&response.body)?),
            401 => Err(DirectoryClientError::Unauthorized),
            status => Err(DirectoryClientError::Http { status, body: String::from_utf8_lossy(&response.body).into_owned() }),
        }
    }

    /// 🛡️ Bounded request/response variant used by the closed command boundary: it caps raw bytes
    /// BEFORE decoding and maps every failure into a closed code, never a raw server body.
    async fn request_bytes_limited(&self, ctx: &OperationContext, method: HttpMethod, path: &str, body: Option<Vec<u8>>, response_max_bytes: usize) -> Result<Vec<u8>, DirectoryCommandErrorCodeV1> {
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryCommandErrorCodeV1::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose().map_err(|_| DirectoryCommandErrorCodeV1::Unauthorized)?;
        let response = match self.transport.http(ctx, method, &self.url(path), bearer, body).await {
            Ok(response) => response,
            Err(TransportError::Cancelled) => return Err(DirectoryCommandErrorCodeV1::Cancelled),
            Err(_) => return Err(DirectoryCommandErrorCodeV1::Transport),
        };
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryCommandErrorCodeV1::Cancelled);
        }
        if !(200..=299).contains(&response.status) {
            return Err(DirectoryCommandErrorCodeV1::from_status(response.status));
        }
        if response.body.len() > response_max_bytes {
            return Err(DirectoryCommandErrorCodeV1::TooLarge);
        }
        Ok(response.body)
    }

    pub async fn spaces(&self, ctx: &OperationContext) -> Result<Vec<DirectorySpaceListEntryV1>, DirectoryClientError> {
        let spaces: Vec<DirectorySpaceListEntryV1> = self.request_json(ctx, HttpMethod::Get, "/directory/spaces", None).await?;
        if spaces.iter().all(DirectorySpaceListEntryV1::validate) {
            Ok(spaces)
        } else {
            Err(DirectoryClientError::Decode("directory space list access discriminator mismatch".into()))
        }
    }

    /// 🏛️ Fetches one bounded canonical administration page for exactly one space. `cursor`
    /// advances precisely the window it was issued for; every other window restarts at its head.
    pub async fn space_administration_page(&self, ctx: &OperationContext, id: &str, cursor: Option<&str>) -> Result<CanonicalDirectorySpaceAdministrationPageV1, DirectoryClientError> {
        if let Some(cursor) = cursor {
            if cursor.is_empty() || cursor.len() > DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES || !cursor.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_')) {
                return Err(DirectoryClientError::Decode("directory space administration cursor is not opaque-safe".into()));
            }
        }
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose()?;
        let path = match cursor {
            Some(cursor) => format!("/directory/spaces/{id}?cursor={cursor}"),
            None => format!("/directory/spaces/{id}"),
        };
        let response = self.transport.http(ctx, HttpMethod::Get, &self.url(&path), bearer, None).await?;
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        match response.status {
            401 => return Err(DirectoryClientError::Unauthorized),
            200..=299 => {}
            status => return Err(DirectoryClientError::Http { status, body: String::from_utf8_lossy(&response.body).into_owned() }),
        }
        if response.body.len() > DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES {
            return Err(DirectoryClientError::Decode("directory space administration page response exceeded 48 KiB".into()));
        }
        let canonical_json = String::from_utf8(response.body).map_err(|error| DirectoryClientError::Decode(error.to_string()))?;
        let page = DirectorySpaceAdministrationPageV1::parse_canonical_json(&canonical_json).map_err(|_| DirectoryClientError::Decode("directory space administration page response is not canonical".into()))?;
        if page.space_id() != id {
            return Err(DirectoryClientError::Decode("directory space administration page space mismatch".into()));
        }
        Ok(CanonicalDirectorySpaceAdministrationPageV1 { canonical_json, page })
    }

    pub async fn events(&self, ctx: &OperationContext, since: u64) -> Result<Vec<DirectoryEvent>, DirectoryClientError> {
        self.request_json(ctx, HttpMethod::Get, &format!("/directory/events?since={since}"), None).await
    }

    /// 📄️ Fetches one bounded canonical page while preserving the exact response bytes for Home.
    pub async fn event_page(&self, ctx: &OperationContext, after: u64) -> Result<CanonicalDirectoryEventPageV1, DirectoryClientError> {
        if after > DOCUMENT_OPEN_MAX_SAFE_INTEGER {
            return Err(DirectoryClientError::Decode("directory event page after frontier is not wire-safe".into()));
        }
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose()?;
        let path = format!("/directory/event-page/v1?after={after}");
        let response = self.transport.http(ctx, HttpMethod::Get, &self.url(&path), bearer, None).await?;
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        match response.status {
            401 => return Err(DirectoryClientError::Unauthorized),
            200..=299 => {}
            status => return Err(DirectoryClientError::Http { status, body: String::from_utf8_lossy(&response.body).into_owned() }),
        }
        if response.body.len() > DIRECTORY_EVENT_PAGE_MAX_BYTES {
            return Err(DirectoryClientError::Decode("directory event page response exceeded 64 KiB".into()));
        }
        let canonical_json = String::from_utf8(response.body).map_err(|error| DirectoryClientError::Decode(error.to_string()))?;
        let page = DirectoryEventPageV1::parse_canonical_json(&canonical_json).map_err(|_| DirectoryClientError::Decode("directory event page response is not canonical".into()))?;
        if page.after_seq_exclusive != after {
            return Err(DirectoryClientError::Decode("directory event page response frontier mismatch".into()));
        }
        Ok(CanonicalDirectoryEventPageV1 {
            canonical_json,
            session_binding_sha256: page.session_binding_sha256,
            authorization_generation: page.authorization_generation,
            after_seq_exclusive: page.after_seq_exclusive,
            through_seq_inclusive: page.through_seq_inclusive,
            has_more: page.has_more,
            receipt_sha256: page.receipt_sha256,
        })
    }

    pub async fn me(&self, ctx: &OperationContext) -> Result<SessionView, DirectoryClientError> {
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose()?;
        let response = self.transport.http(ctx, HttpMethod::Get, &self.url("/auth/sessions/me"), bearer, None).await?;
        match response.status {
            401 => return Err(DirectoryClientError::Unauthorized),
            200..=299 => {}
            status => return Err(DirectoryClientError::Http { status, body: String::from_utf8_lossy(&response.body).into_owned() }),
        }
        if response.body.len() > DIRECTORY_SESSION_AUTHORITY_MAX_BYTES {
            return Err(DirectoryClientError::Decode("directory session authority response exceeded 2 KiB".into()));
        }
        let source = String::from_utf8(response.body).map_err(|error| DirectoryClientError::Decode(error.to_string()))?;
        DirectorySessionAuthorityV1::parse_canonical_json(&source).ok_or_else(|| DirectoryClientError::Decode("directory session authority response is not canonical".into()))
    }

    /// 🚪️ Sends the `POST /auth/sessions/me/sign-out` command for the held session (or the browser
    /// cookie session when this client has no bearer). A successful empty `204` is accepted without
    /// attempting JSON decoding.
    pub async fn sign_out(&self, ctx: &OperationContext) -> Result<(), DirectoryClientError> {
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose()?;
        let response = self.transport.http(ctx, HttpMethod::Post, &self.url("/auth/sessions/me/sign-out"), bearer, Some(Vec::new())).await?;
        match response.status {
            200 | 204 | 401 => Ok(()),
            status => Err(DirectoryClientError::Http { status, body: String::from_utf8_lossy(&response.body).into_owned() }),
        }
    }

    /// 🎟️ Redeems one bounded invitation capability through the authenticated client. The
    /// capability is path-safe by admission and never appears in a query string.
    pub async fn redeem_invite(&self, ctx: &OperationContext, capability: &str) -> Result<(), DirectoryClientError> {
        if capability.is_empty() || capability.len() > 256 || !capability.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'~' | b'-')) {
            return Err(DirectoryClientError::Decode("directory invitation capability is invalid".into()));
        }
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose()?;
        let response = self.transport.http(ctx, HttpMethod::Post, &self.url(&format!("/directory/invites/{capability}/redeem")), bearer, Some(Vec::new())).await?;
        match response.status {
            200 | 204 => Ok(()),
            401 => Err(DirectoryClientError::Unauthorized),
            status => Err(DirectoryClientError::Http { status, body: String::from_utf8_lossy(&response.body).into_owned() }),
        }
    }

    /// 🪪️ Fetches the authenticated server-selected execution-target manifest for one exact
    /// document scope. The caller supplies only structural intent; package, descriptor and component
    /// identities all come back from the protected Hub route.
    pub async fn document_execution_target_manifest(&self, ctx: &OperationContext, intent: &DocumentOpenIntentV1) -> Result<DocumentExecutionTargetLeaseFieldsV1, DirectoryClientError> {
        intent.validate().map_err(|_| DirectoryClientError::Decode("document execution-target intent is invalid".into()))?;
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose()?;
        let path = format!("/spaces/{}/documents/{}/execution-target/manifest", encode_url_component(&intent.scope.space_id), encode_url_component(&intent.scope.document_id),);
        let response = self.transport.http(ctx, HttpMethod::Post, &self.url(&path), bearer, Some(semio_framework_pack_json::to_json_string(intent).into_bytes())).await?;
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        match response.status {
            401 => return Err(DirectoryClientError::Unauthorized),
            200..=299 => {}
            status => return Err(DirectoryClientError::Http { status, body: execution_target_refusal(&response.body) }),
        }
        if response.body.len() > 64 * 1024 {
            return Err(DirectoryClientError::Decode("document execution-target manifest exceeded its fixed byte limit".into()));
        }
        let manifest: DocumentExecutionTargetLeaseFieldsV1 = decode_json_bytes(&response.body)?;
        manifest.validate().map_err(|_| DirectoryClientError::Decode("document execution-target manifest is invalid".into()))?;
        Ok(manifest)
    }

    /// 🧩️ Fetches the exact authorized COMPONENT bytes for the same protected structural intent —
    /// the plugin wasm the Hub itself selected for this document. It is the third leg of the same
    /// chain the browser shell already walks (`open-plan` → `execution-target/{manifest, component,
    /// descriptor}` → `socket-grants` → the document socket); before this method existed, a
    /// `--hub`-bound native client had manifest and descriptor and no way to obtain the component,
    /// so it could only run a plugin it found on a local disk — which a hub-bound agent has none of
    /// (measured 2026-09-21: every `action_prepare` from the MCP gateway answered `repo root not
    /// found`). Bounded by the same fixed component ceiling the Hub's own route enforces; the
    /// declared length and SHA-256 stay the caller's comparison input, exactly as for the descriptor.
    pub async fn document_execution_target_component(&self, ctx: &OperationContext, intent: &DocumentOpenIntentV1) -> Result<Vec<u8>, DirectoryClientError> {
        intent.validate().map_err(|_| DirectoryClientError::Decode("document execution-target intent is invalid".into()))?;
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose()?;
        let path = format!("/spaces/{}/documents/{}/execution-target/component", encode_url_component(&intent.scope.space_id), encode_url_component(&intent.scope.document_id),);
        let response = self.transport.http(ctx, HttpMethod::Post, &self.url(&path), bearer, Some(semio_framework_pack_json::to_json_string(intent).into_bytes())).await?;
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        match response.status {
            401 => Err(DirectoryClientError::Unauthorized),
            200..=299 if response.body.len() <= DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES as usize => Ok(response.body),
            200..=299 => Err(DirectoryClientError::Decode("document execution-target component exceeded its fixed byte limit".into())),
            status => Err(DirectoryClientError::Http { status, body: execution_target_refusal(&response.body) }),
        }
    }

    /// 📖 Fetches exact descriptor bytes for the same protected structural intent. Raw bytes are
    /// bounded before return; their declared length and SHA-256 remain the caller's comparison input.
    pub async fn document_execution_target_descriptor(&self, ctx: &OperationContext, intent: &DocumentOpenIntentV1) -> Result<Vec<u8>, DirectoryClientError> {
        intent.validate().map_err(|_| DirectoryClientError::Decode("document execution-target intent is invalid".into()))?;
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose()?;
        let path = format!("/spaces/{}/documents/{}/execution-target/descriptor", encode_url_component(&intent.scope.space_id), encode_url_component(&intent.scope.document_id),);
        let response = self.transport.http(ctx, HttpMethod::Post, &self.url(&path), bearer, Some(semio_framework_pack_json::to_json_string(intent).into_bytes())).await?;
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        match response.status {
            401 => Err(DirectoryClientError::Unauthorized),
            200..=299 if response.body.len() <= DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES as usize => Ok(response.body),
            200..=299 => Err(DirectoryClientError::Decode("document execution-target descriptor exceeded its fixed byte limit".into())),
            status => Err(DirectoryClientError::Http { status, body: execution_target_refusal(&response.body) }),
        }
    }

    /// 🧾️ Posts one sealed V1 request and parses only a raw-byte-capped canonical receipt bound to
    /// it. Every non-2xx becomes a closed {@link DirectoryCommandErrorCodeV1}; the response body is
    /// never decoded into a UI-facing error. `ctx.cancel` cancels the HTTP wait only — a command
    /// already past its server linearization point stays committed and the operation is indeterminate.
    pub async fn command(&self, ctx: &OperationContext, request: &DirectoryCommandRequestV1) -> Result<CanonicalDirectoryCommandReceiptV1, DirectoryCommandErrorCodeV1> {
        request.validate()?;
        let body = request.canonical_json().into_bytes();
        let response = self.request_bytes_limited(ctx, HttpMethod::Post, "/directory/commands", Some(body), DIRECTORY_COMMAND_RECEIPT_MAX_BYTES).await?;
        let canonical_json = String::from_utf8(response).map_err(|_| DirectoryCommandErrorCodeV1::Invalid)?;
        let receipt = DirectoryCommandReceiptV1::parse_canonical_json(&canonical_json, request)?;
        Ok(CanonicalDirectoryCommandReceiptV1 { canonical_json, receipt })
    }

    /// 📡 Executes one owner-prepared bounded document call and returns only its validated reply.
    pub async fn document_http_call<R>(&self, ctx: &OperationContext, scope: &DocumentScope, request: &DocumentHttpRequestV1, decode: impl FnOnce(&[u8]) -> Result<R, DocumentHttpPortCodeV1>) -> Result<R, DocumentHttpPortCodeV1> {
        if scope.space_id.is_empty() || scope.document_id.is_empty() || scope.space_id.len() > 256 || scope.document_id.len() > 256 || request.segments.is_empty() || request.segments.len() > 8 || request.segments.iter().any(|part| part.is_empty() || part.len() > 256 || matches!(part.as_str(), "." | "..")) {
            return Err(DocumentHttpPortCodeV1::Invalid);
        }
        if request.request_max_bytes == 0 || request.request_max_bytes > DOCUMENT_HTTP_REQUEST_MAX_BYTES || request.response_max_bytes == 0 || request.response_max_bytes > DOCUMENT_HTTP_RESPONSE_MAX_BYTES || request.body.as_ref().is_some_and(|bytes| bytes.len() > request.request_max_bytes) || request.after.is_some_and(|after| after > DOCUMENT_OPEN_MAX_SAFE_INTEGER) {
            return Err(DocumentHttpPortCodeV1::Bounds);
        }
        if ctx.cancel.is_cancelled().await {
            return Err(DocumentHttpPortCodeV1::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose().map_err(|_| DocumentHttpPortCodeV1::Denied)?;
        let suffix = request.segments.iter().map(|part| encode_url_component(part)).collect::<Vec<_>>().join("/");
        let mut path = format!("/spaces/{}/documents/{}/{suffix}", encode_url_component(&scope.space_id), encode_url_component(&scope.document_id));
        if let Some(after) = request.after {
            path.push_str(&format!("?after={after}"));
        }
        let response = match self.transport.http(ctx, request.method, &self.url(&path), bearer, request.body.clone()).await {
            Ok(response) => response,
            Err(TransportError::Cancelled) => return Err(DocumentHttpPortCodeV1::Cancelled),
            Err(_) => return Err(DocumentHttpPortCodeV1::Transport),
        };
        if ctx.cancel.is_cancelled().await {
            return Err(DocumentHttpPortCodeV1::Cancelled);
        }
        if !(200..=299).contains(&response.status) {
            return Err(DocumentHttpPortCodeV1::from_status(response.status));
        }
        if response.body.len() > request.response_max_bytes {
            return Err(DocumentHttpPortCodeV1::Bounds);
        }
        decode(&response.body)
    }


    pub fn stream(self: &Arc<Self>, since: u64) -> DirectoryStream<T>
    where
        T: Clone,
    {
        DirectoryStream::new(self.clone(), None, since, true)
    }

    /// 📌️ Creates a global wakeup stream whose reconnect cursor advances only by explicit page ACK.
    pub fn stream_acknowledged(self: &Arc<Self>, since: u64) -> Result<DirectoryStream<T>, DirectoryClientError>
    where
        T: Clone,
    {
        if since > DOCUMENT_OPEN_MAX_SAFE_INTEGER {
            return Err(DirectoryClientError::Decode("directory stream acknowledged frontier is not wire-safe".into()));
        }
        Ok(DirectoryStream::new(self.clone(), None, since, false))
    }

    /// 🎯️ Creates one exact scope-bound stream whose membership revocation is terminal.
    pub fn stream_scoped(self: &Arc<Self>, scope: DocumentScope, since: u64) -> DirectoryStream<T>
    where
        T: Clone,
    {
        DirectoryStream::new(self.clone(), Some(scope), since, true)
    }

    pub fn open_stream_ws(&self, ctx: &OperationContext, since: u64, timeout_ms: u64) -> Result<T::Ws, DirectoryClientError> {
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }
        let mut receipt = self.issue_socket_grant(ctx, "/directory/socket-grants", b"", timeout_ms)?;
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }
        let url = directory_ws_url(&self.base_url, since);
        let mut protocols = vec![std::mem::take(&mut receipt.protocol), std::mem::take(&mut receipt.grant)];
        let connection = self.transport.open_ws(ctx, &url, &protocols, timeout_ms);
        wipe_string(&mut protocols[1]);
        let mut connection = connection?;
        if ctx.cancel.is_cancelled_now() {
            connection.close();
            return Err(DirectoryClientError::Cancelled);
        }
        if let Err(error) = connection.send_binary(directory_socket_hello_v1()) {
            connection.close();
            return Err(error.into());
        }
        if ctx.cancel.is_cancelled_now() {
            connection.close();
            return Err(DirectoryClientError::Cancelled);
        }
        Ok(connection)
    }

    /// 🎯️ Issues and consumes one membership-bound grant for an exact document scope.
    pub fn open_scoped_stream_ws(&self, ctx: &OperationContext, scope: &DocumentScope, since: u64, timeout_ms: u64) -> Result<T::Ws, DirectoryClientError> {
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }
        if scope.space_id.is_empty() || scope.document_id.is_empty() || scope.space_id.len() > DIRECTORY_SCOPE_ID_MAX_BYTES || scope.document_id.len() > DIRECTORY_SCOPE_ID_MAX_BYTES {
            return Err(DirectoryClientError::Decode("directory stream scope invalid".into()));
        }
        let prefix = format!("/directory/spaces/{}/documents/{}", encode_url_component(&scope.space_id), encode_url_component(&scope.document_id),);
        let mut receipt = self.issue_socket_grant(ctx, &format!("{prefix}/socket-grants"), b"", timeout_ms)?;
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }
        let url = directory_scoped_ws_url(&self.base_url, scope, since);
        let mut protocols = vec![std::mem::take(&mut receipt.protocol), std::mem::take(&mut receipt.grant)];
        let connection = self.transport.open_ws(ctx, &url, &protocols, timeout_ms);
        wipe_string(&mut protocols[1]);
        let mut connection = connection?;
        if ctx.cancel.is_cancelled_now() {
            connection.close();
            return Err(DirectoryClientError::Cancelled);
        }
        if let Err(error) = connection.send_binary(directory_socket_hello_v1()) {
            connection.close();
            return Err(error.into());
        }
        Ok(connection)
    }

    /// 🛡️ The exact url and bearer of one protected admission POST: a path under the credential's own
    /// origin only, so a caller can never aim the capability at a foreign host.
    fn protected_post_target(&self, ctx: &OperationContext, path: &str) -> Result<(String, &str), DirectoryClientError> {
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }
        let credential = self.credential.as_ref().ok_or(DirectoryClientError::Unauthorized)?;
        let origin = credential.hub_origin().trim_end_matches('/');
        if self.base_url.trim_end_matches('/') != origin || !path.starts_with('/') || path.starts_with("//") {
            return Err(DirectoryClientError::Unauthorized);
        }
        Ok((format!("{origin}{path}"), credential.capability()?))
    }

    fn protected_post(&self, ctx: &OperationContext, path: &str, body: &[u8], timeout_ms: u64) -> Result<WipeBytes<'static>, DirectoryClientError> {
        let (url, bearer) = self.protected_post_target(ctx, path)?;
        let response = self.transport.issue_socket_grant(ctx, &url, bearer, body, timeout_ms.clamp(1, 5_000))?;
        Self::protected_post_answer(ctx, response)
    }

    /// 🌐️ The browser twin of [`Self::protected_post`]: the same target and answer checks around the
    /// transport's own asynchronous `http` hop, because a browser isolate cannot block on a grant.
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    async fn protected_post_async(&self, ctx: &OperationContext, path: &str, body: &[u8]) -> Result<WipeBytes<'static>, DirectoryClientError> {
        let (url, bearer) = self.protected_post_target(ctx, path)?;
        let response = self.transport.http(ctx, HttpMethod::Post, &url, Some(bearer), Some(body.to_vec())).await?;
        Self::protected_post_answer(ctx, response)
    }

    /// 🧾️ Bounds and classifies one protected admission answer without ever surfacing its body.
    fn protected_post_answer(ctx: &OperationContext, response: HttpResponse) -> Result<WipeBytes<'static>, DirectoryClientError> {
        let response_body = WipeBytes { bytes: response.body, observer: None };
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }
        if response_body.bytes.len() > DOCUMENT_ADMISSION_RESPONSE_MAX_BYTES {
            return Err(DirectoryClientError::Decode("protected document admission response exceeded 64 KiB".into()));
        }
        if response.status == 401 {
            return Err(DirectoryClientError::Unauthorized);
        }
        if !(200..=299).contains(&response.status) {
            return Err(DirectoryClientError::Http { status: response.status, body: "protected document admission rejected".into() });
        }
        Ok(response_body)
    }

    pub(super) fn issue_socket_grant(&self, ctx: &OperationContext, path: &str, body: &[u8], timeout_ms: u64) -> Result<SocketGrantReceiptV1, DirectoryClientError> {
        let response_body = self.protected_post(ctx, path, body, timeout_ms)?;
        let receipt: SocketGrantReceiptV1 = decode_json_bytes(&response_body.bytes).map_err(|_| DirectoryClientError::Decode("socket grant receipt invalid".into()))?;
        if receipt.schema != "semio.hub.socket-grant/v1" || receipt.protocol != "semio.socket.v1" || !valid_socket_grant(&receipt.grant) || !valid_socket_actor(&receipt.actor_id) || receipt.expires_at_ms <= wall_now_ms() {
            return Err(DirectoryClientError::Decode("socket grant receipt binding invalid".into()));
        }
        emit_socket_grant_probe(path, &receipt.grant);
        Ok(receipt)
    }

    /// 🎫 Admission leg one: the validated open intent, its scope and the document route it posts under.
    fn document_admission_intent(space_id: &str, document_id: &str, expectation: &DocumentSocketExpectationV1, client_instance_id: &str) -> Result<(DocumentScope, String, Vec<u8>), DirectoryClientError> {
        let scope = DocumentScope::new(space_id, document_id);
        let intent = DocumentOpenIntentV1 { schema: "semio.hub.document-open-intent/v1".into(), version: 1, scope: scope.clone(), requested_surface_id: expectation.requested_surface_id.clone(), client_instance_id: client_instance_id.to_string() };
        if intent.validate().is_err() || expectation.artifact_schema.is_empty() {
            return Err(DirectoryClientError::Decode("document open intent invalid".into()));
        }
        let prefix = format!("/spaces/{}/documents/{}", encode_url_component(space_id), encode_url_component(document_id));
        Ok((scope, prefix, semio_framework_pack_json::to_json_string(&intent).into_bytes()))
    }

    /// 🧾 Admission leg two: binds the answered plan to the local expectation, derives the receipt-free
    /// authority and seals the plan-receipt exchange that buys the socket grant.
    fn document_admission_exchange(&self, ctx: &OperationContext, plan_body: &[u8], scope: &DocumentScope, expectation: &DocumentSocketExpectationV1) -> Result<(DocumentSocketAuthorityV1, WipeBytes<'static>), DirectoryClientError> {
        let mut plan: DocumentOpenPlanV1 = decode_json_bytes(plan_body).map_err(|_| DirectoryClientError::Decode("document open plan response invalid".into()))?;
        let now_ms = u64::try_from(wall_now_ms()).map_err(|_| DirectoryClientError::Decode("document open clock invalid".into()))?;
        let pack_schema_hash = decode_lower_hex_32(&plan.artifact.pack_schema_hash);
        let surface_matches = expectation
            .lease
            .as_ref()
            .is_none_or(|expected| lease_fields_from_plan_v1(&plan, expected.component.byte_length, expected.descriptor.byte_length, expected.browser_actor.byte_length()).is_ok_and(|fields| same_lease_fields_v1(&fields, expected)))
            && expectation.requested_surface_id.as_ref().is_none_or(|surface| plan.surface.surface_id == *surface);
        if plan.validate(now_ms).is_err() || plan.scope != *scope || plan.artifact.schema != expectation.artifact_schema || pack_schema_hash != Some(expectation.pack_schema_hash) || !surface_matches {
            wipe_string(&mut plan.receipt);
            return Err(DirectoryClientError::Decode("document open plan binding invalid".into()));
        }
        if ctx.cancel.is_cancelled_now() {
            wipe_string(&mut plan.receipt);
            return Err(DirectoryClientError::Cancelled);
        }
        let credential_origin = self.credential.as_ref().ok_or(DirectoryClientError::Unauthorized)?.hub_origin().trim_end_matches('/');
        let authority = DocumentSocketAuthorityV1::from_plan(credential_origin.to_string(), &plan, expectation, now_ms).map_err(|_| DirectoryClientError::Decode("document open authority invalid".into()))?;
        let exchange = WipeDocumentPlanSocketGrantIntent(DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: std::mem::take(&mut plan.receipt) });
        Ok((authority, WipeBytes { bytes: semio_framework_pack_json::to_json_string(&exchange.0).into_bytes(), observer: None }))
    }

    /// 🎟 Admission leg three: the grant receipt, bound to `semio.session.v1` and a hub socket actor,
    /// joined with the authority that is still unexpired and uncancelled at this instant.
    fn document_admission_finish(ctx: &OperationContext, path: &str, grant_body: &[u8], authority: DocumentSocketAuthorityV1) -> Result<DocumentSocketAdmissionV1, DirectoryClientError> {
        let socket: DocumentSocketGrantReceiptV1 = decode_json_bytes(grant_body).map_err(|_| DirectoryClientError::Decode("document socket grant receipt invalid".into()))?;
        if socket.schema != "semio.hub.document-socket-grant/v1" || socket.protocol != "semio.session.v1" || !valid_socket_actor(&socket.actor_id) || socket.expires_at_ms <= wall_now_ms() {
            return Err(DirectoryClientError::Decode("document socket grant receipt binding invalid".into()));
        }
        emit_document_socket_grant_probe(path);
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }
        if authority.expires_at_unix_ms <= u64::try_from(wall_now_ms()).map_err(|_| DirectoryClientError::Decode("document open clock invalid".into()))? {
            return Err(DirectoryClientError::Decode("document open authority expired".into()));
        }
        Ok(DocumentSocketAdmissionV1 { socket, authority })
    }

    /// 🌐️ The browser admission: the same three legs as the native one, each protected POST awaited on
    /// the transport's asynchronous `http` hop.
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    async fn admit_document_socket_async(&self, ctx: &OperationContext, space_id: &str, document_id: &str, expectation: &DocumentSocketExpectationV1, client_instance_id: &str) -> Result<DocumentSocketAdmissionV1, DirectoryClientError> {
        let (scope, prefix, intent_body) = Self::document_admission_intent(space_id, document_id, expectation, client_instance_id)?;
        let plan_body = self.protected_post_async(ctx, &format!("{prefix}/open-plan"), &intent_body).await?;
        let (authority, exchange_body) = self.document_admission_exchange(ctx, &plan_body.bytes, &scope, expectation)?;
        let path = format!("{prefix}/socket-grants");
        let grant_body = self.protected_post_async(ctx, &path, &exchange_body.bytes).await?;
        Self::document_admission_finish(ctx, &path, &grant_body.bytes, authority)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<T: DirectoryTransport + Send + Sync> HubSocketGrantSource for DirectoryClient<T> {
    fn admit_document_socket(&self, ctx: &OperationContext, space_id: &str, document_id: &str, expectation: &DocumentSocketExpectationV1, client_instance_id: &str, timeout_ms: u64) -> Result<DocumentSocketAdmissionV1, DirectoryClientError> {
        let (scope, prefix, intent_body) = Self::document_admission_intent(space_id, document_id, expectation, client_instance_id)?;
        let plan_body = self.protected_post(ctx, &format!("{prefix}/open-plan"), &intent_body, timeout_ms)?;
        let (authority, exchange_body) = self.document_admission_exchange(ctx, &plan_body.bytes, &scope, expectation)?;
        let path = format!("{prefix}/socket-grants");
        let grant_body = self.protected_post(ctx, &path, &exchange_body.bytes, timeout_ms)?;
        Self::document_admission_finish(ctx, &path, &grant_body.bytes, authority)
    }
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
impl<T: DirectoryTransport + Send + Sync> HubSocketGrantSource for DirectoryClient<T> {
    fn admit_document_socket<'a>(&'a self, ctx: &'a OperationContext, space_id: &'a str, document_id: &'a str, expectation: &'a DocumentSocketExpectationV1, client_instance_id: &'a str, _timeout_ms: u64) -> DocumentSocketAdmissionFuture<'a> {
        Box::pin(self.admit_document_socket_async(ctx, space_id, document_id, expectation, client_instance_id))
    }
}
//#endregion 🔖️Client
