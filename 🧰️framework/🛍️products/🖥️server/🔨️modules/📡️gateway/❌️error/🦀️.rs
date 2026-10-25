//! ❌️ The server error, its wire body and response mapping, and the wire JSON extractor.

use super::*;

//#region 🔖️Error
/// 💥️ The one error every transport surface answers with. An instance maps its own domain errors
/// into these ten shapes; the status code is derived here so no handler ever picks one by hand.
///
/// **What the caller is told.** The detail a variant carries is the server's own account of the
/// fault and is written for its operator ([`Display`](std::fmt::Display)). The body a caller
/// receives ([`message`](Self::message)) repeats it only where it describes the caller's own
/// request; a policy denial answers `forbidden` and an internal fault `internal error`, and the
/// detail of the latter goes to the operator's log instead.
#[derive(Clone, Debug)]
pub enum ServerError {
    /// 🔒️ The caller could not be authenticated — 401.
    Unauthorized(String),
    /// ⛔️ The caller is known and policy still says no — 403.
    Forbidden(String),
    /// 🕳️ Nothing is addressed by this path — 404.
    NotFound(String),
    /// ⚔️ The write contradicts what is already stored — 409.
    Conflict(String),
    /// 🚧️ The request itself is malformed — 400.
    BadRequest(String),
    /// 🔥️ The server failed to answer at all — 500.
    Internal(String),
    /// 📦️ The request body is larger than this instance accepts — 413.
    PayloadTooLarge,
    /// 🐌️ The request body did not arrive in the time this instance waits for one — 408.
    Stalled,
    /// 🐇️ The caller's address spent an allowance — 429, worth retrying after the wait: its
    /// allowance of requests, or the named one ([`Allowance`](crate::throttle::Allowance)) the
    /// refused command is counted against.
    Throttled { wait: Duration, allowance: Option<&'static str> },
    /// 🌊️ The instance is at one of its caps — 503, worth retrying after the wait.
    Overloaded(Duration),
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unauthorized(detail) => write!(formatter, "unauthorized: {detail}"),
            Self::Forbidden(detail) => write!(formatter, "forbidden: {detail}"),
            Self::NotFound(detail) => write!(formatter, "not found: {detail}"),
            Self::Conflict(detail) => write!(formatter, "conflict: {detail}"),
            Self::BadRequest(detail) => write!(formatter, "bad request: {detail}"),
            Self::Internal(detail) => write!(formatter, "internal error: {detail}"),
            Self::PayloadTooLarge => formatter.write_str("payload too large"),
            Self::Stalled => formatter.write_str("the request body did not arrive in time"),
            Self::Throttled { wait, allowance: None } => write!(formatter, "too many requests: retry in {} ms", wait.as_millis()),
            Self::Throttled { wait, allowance: Some(allowance) } => write!(formatter, "the {allowance} allowance of this address is spent: retry in {} ms", wait.as_millis()),
            Self::Overloaded(wait) => write!(formatter, "overloaded: retry in {} ms", wait.as_millis()),
        }
    }
}

impl std::error::Error for ServerError {}

impl ServerError {
    /// 🔢️ The HTTP status this error is answered with.
    pub fn status(&self) -> StatusCode {
        match self {
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::PayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            Self::Stalled => StatusCode::REQUEST_TIMEOUT,
            Self::Throttled { .. } => StatusCode::TOO_MANY_REQUESTS,
            Self::Overloaded(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    /// 🏷️ The stable machine-readable tag a client branches on instead of the status code.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Unauthorized(_) => "unauthorized",
            Self::Forbidden(_) => "forbidden",
            Self::NotFound(_) => "notFound",
            Self::Conflict(_) => "conflict",
            Self::BadRequest(_) => "badRequest",
            Self::Internal(_) => "internal",
            Self::PayloadTooLarge => "payloadTooLarge",
            Self::Stalled => "stalled",
            Self::Throttled { .. } => "throttled",
            Self::Overloaded(_) => "overloaded",
        }
    }

    /// 💬️ What the caller is told: the detail of its own request, never the server's.
    pub fn message(&self) -> String {
        match self {
            Self::Forbidden(_) => "forbidden".to_string(),
            Self::Internal(_) => "internal error".to_string(),
            Self::Throttled { allowance: None, .. } => "too many requests".to_string(),
            Self::Throttled { allowance: Some(allowance), .. } => format!("the {allowance} allowance of this address is spent"),
            Self::Overloaded(_) => "overloaded".to_string(),
            Self::Unauthorized(_) | Self::NotFound(_) | Self::Conflict(_) | Self::BadRequest(_) | Self::PayloadTooLarge | Self::Stalled => self.to_string(),
        }
    }

    /// ⏳️ The wait after which the request is worth repeating, for the two errors that pass.
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::Throttled { wait, .. } | Self::Overloaded(wait) => Some(*wait),
            _ => None,
        }
    }

    /// 💰️ The named allowance whose exhaustion this error is, when it is one.
    pub fn allowance(&self) -> Option<&'static str> {
        match self {
            Self::Throttled { allowance, .. } => *allowance,
            _ => None,
        }
    }
}

/// 🧾️ The JSON body every [`ServerError`] renders as, so a client never has to parse a status line.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBody {
    /// 🔖️ The [`ServerError::kind`] tag.
    pub kind: String,
    /// ✉️ The human-facing detail.
    pub message: String,
    /// 🔁️ The wait in milliseconds after which the request is worth repeating, when it is. Carried
    /// in the body as well as in `Retry-After`, which a cross-origin page cannot read unless the
    /// response exposes it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_after_ms: Option<u64>,
    /// 🧧️ The named allowance of the caller's address that is spent, when the refusal is one of a
    /// class of commands the instance counts by what they make it keep rather than of requests as
    /// such: a client tells its user what cannot be done for now instead of only slowing down.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowance: Option<String>,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        if let Self::Internal(detail) = &self {
            report("gateway", detail);
        }
        let wait = self.retry_after();
        let body = ErrorBody { kind: self.kind().to_string(), message: self.message(), retry_after_ms: wait.map(|wait| u64::try_from(wait.as_millis()).unwrap_or(u64::MAX).max(1)), allowance: self.allowance().map(str::to_string) };
        let mut response = (self.status(), Json(body)).into_response();
        if let Some(wait) = wait {
            response.headers_mut().insert(header::RETRY_AFTER, HeaderValue::from(wait.as_secs() + u64::from(wait.subsec_nanos() > 0 || wait.is_zero())));
        }
        response
    }
}

impl From<StorageError> for ServerError {
    /// 🗄️ A store's refusal as the caller may see it: its wording is the backend's and stays in the
    /// operator's log.
    fn from(error: StorageError) -> Self {
        match error {
            StorageError::NotFound => Self::NotFound("storage entry not found".to_string()),
            StorageError::Conflict(_) | StorageError::SequenceGap { .. } | StorageError::LeaseLost => {
                report("gateway storage conflict", &error);
                Self::Conflict("the write contradicts what is stored".to_string())
            }
            StorageError::Backend(detail) => Self::Internal(detail),
        }
    }
}

impl From<AuthorityError> for ServerError {
    fn from(error: AuthorityError) -> Self {
        match error {
            AuthorityError::LeaseLost => Self::Conflict(error.to_string()),
            AuthorityError::UnknownActorKind(kind) => Self::NotFound(format!("unknown actor kind: {kind}")),
            AuthorityError::Storage(detail) => Self::Internal(detail),
        }
    }
}

impl From<Refusal> for ServerError {
    fn from(refusal: Refusal) -> Self {
        match refusal {
            Refusal::Throttled { retry_after } => Self::Throttled { wait: retry_after, allowance: None },
            Refusal::Overloaded { retry_after } => Self::Overloaded(retry_after),
        }
    }
}

/// 📥️ A JSON request body as every route of this gateway takes one: the transport's own extractor,
/// answering a body it cannot take with the gateway's own [`ErrorBody`] — `payloadTooLarge` past the
/// instance's body limit, `badRequest` for anything that is not the expected JSON document sent as
/// `application/json`.
pub struct WireJson<T>(pub T);

impl<S: Send + Sync, T: serde::de::DeserializeOwned> FromRequest<S> for WireJson<T> {
    type Rejection = ServerError;

    async fn from_request(request: Request, state: &S) -> Result<Self, ServerError> {
        match Json::<T>::from_request(request, state).await {
            Ok(Json(value)) => Ok(Self(value)),
            Err(rejection) if rejection.status() == StatusCode::PAYLOAD_TOO_LARGE => Err(ServerError::PayloadTooLarge),
            Err(JsonRejection::MissingJsonContentType(_)) => Err(ServerError::BadRequest("the body must be sent as application/json".to_string())),
            Err(rejection) => Err(ServerError::BadRequest(rejection.body_text())),
        }
    }
}
//#endregion 🔖️Error
