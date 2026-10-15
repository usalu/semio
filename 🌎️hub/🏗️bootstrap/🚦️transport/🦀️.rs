//! 🚦️ `transport` — transport middlewares: forwarded TLS, refusal typing, rate limiting, CORS, route metrics.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use futures::StreamExt;
use semio_framework_trace::record::{TraceOutcome, TraceRecord, Tracer};
use semio_hub::artifact_authority::chunk_cas::ArtifactChunkCasStorage;
use semio_hub::observability::HubRouteMetricsV1;
use semio_hub::auth::rate_limit::{RateLimitClassV1, RateLimitDecisionV1, RateLimitRefusalV1, RateLimitSubjectV1};
use semio_hub::auth::AuthErrorCodeV1;
use semio_hub::directory::SessionCapability;
use std::net::SocketAddr;
use std::sync::Arc;
use crate::startup::HubError;
use crate::support::bearer;
use crate::state::HubState;
use crate::session_agent::auth_error_response;
use db::db_storage::PayloadStorage as _;
use server::storage::SessionStore as _;

//#region 🔖️AdminAuth
/// 🔐️ Whether a reverse proxy in front of this hub is trusted to report how the *client*
/// reached it.
///
/// The hub speaks plain HTTP and always will (`grep rustls|native-tls|TlsAcceptor` → 0 hits); a real
/// deployment terminates TLS in front of it. That leaves the hub unable to tell, from the socket
/// alone, whether the bearer token it is about to mint travelled over TLS or over cleartext — the
/// proxy is the only thing that knows, and it says so in `X-Forwarded-Proto`. Trusting that header
/// is a decision an operator must take out loud, because anyone who can reach the socket directly
/// can also set it: it is only meaningful when nothing but the proxy can reach the port.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ForwardedTlsTrustV1 {
    /// 🚪️ Forwarding headers are ignored entirely. The hub is reachable only on loopback, so the
    /// transport is as private as the machine is.
    Untrusted,
    /// 🛡️ A TLS-terminating reverse proxy is the only thing that can reach this socket, and its
    /// `X-Forwarded-Proto`/`X-Forwarded-Host` are the truth about the client's connection.
    TerminatingProxy,
}

impl ForwardedTlsTrustV1 {
    /// 🔐️ `OS_HUB_TRUSTED_FORWARDING=none|proxy`, default `none`. An unrecognized value
    /// fails the boot rather than silently choosing the permissive answer.
    pub(crate) fn from_environment() -> Result<Self, HubError> {
        match std::env::var("OS_HUB_TRUSTED_FORWARDING").as_deref() {
            Err(std::env::VarError::NotPresent) | Ok("") | Ok("none") => Ok(Self::Untrusted),
            Ok("proxy") => Ok(Self::TerminatingProxy),
            Err(error) => Err(HubError::UnsafeAuthConfiguration(format!("OS_HUB_TRUSTED_FORWARDING is unreadable: {error}"))),
            Ok(_) => Err(HubError::UnsafeAuthConfiguration("OS_HUB_TRUSTED_FORWARDING must be none or proxy".into())),
        }
    }

    /// 🏷️ The stable word the startup line reports.
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Untrusted => "none",
            Self::TerminatingProxy => "proxy",
        }
    }
}

/// 🔐️ Whether one request reached the *client* over a secure transport, as far as this hub
/// is entitled to know.
///
/// Under [`ForwardedTlsTrustV1::Untrusted`] every request is accepted and the forwarding headers are
/// not even read — the hub is loopback-only there, and reading an untrusted header would be strictly
/// worse than ignoring it. Under [`ForwardedTlsTrustV1::TerminatingProxy`] the first
/// `X-Forwarded-Proto` value must be exactly `https`; a proxy that reports cleartext is reporting
/// that this request's bearer token crossed the network in the clear, and the hub refuses it rather
/// than participating.
pub(crate) fn forwarded_request_is_secure(headers: &HeaderMap, trust: ForwardedTlsTrustV1) -> bool {
    match trust {
        ForwardedTlsTrustV1::Untrusted => true,
        ForwardedTlsTrustV1::TerminatingProxy => headers
            .get("x-forwarded-proto")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.split(',').next())
            .is_some_and(|scheme| scheme.trim().eq_ignore_ascii_case("https")),
    }
}

/// 🌐️ The host the *client* used, for anything that has to name this deployment back to it.
/// `X-Forwarded-Host` is read only under [`ForwardedTlsTrustV1::TerminatingProxy`]; otherwise the
/// request's own `Host` is the whole truth.
pub(crate) fn forwarded_external_host(headers: &HeaderMap, trust: ForwardedTlsTrustV1) -> Option<String> {
    let forwarded = match trust {
        ForwardedTlsTrustV1::TerminatingProxy => headers.get("x-forwarded-host").and_then(|value| value.to_str().ok()).and_then(|value| value.split(',').next()).map(str::trim).filter(|host| !host.is_empty()),
        ForwardedTlsTrustV1::Untrusted => None,
    };
    forwarded.or_else(|| headers.get(axum::http::header::HOST).and_then(|value| value.to_str().ok()).map(str::trim).filter(|host| !host.is_empty())).map(str::to_string)
}

/// 🚧️ The router's outermost layer: every answer that is not a success leaves as a typed, signed
/// refusal. A response at or above 400 without `x-semio-refusal` gets the code its status declares
/// (`HubRefusalStatusCodesV1`), so a client can always tell the hub's own refusal from a proxy's; its
/// body is never touched, so a bare refusal stays body-free and discloses nothing. A handler that
/// panics answers a typed `500 internal` instead of dropping the connection, and the panic is traced.
/// The handler is polled in place, never spawned: it stays owned by its request, so a client that
/// disconnects still drops it at its next await (`canonical_pair_route_disconnect_deadline_and_progress_are_request_owned`).
pub(crate) async fn refusal_middleware(State(tracer): State<Tracer>, request: axum::extract::Request, next: axum::middleware::Next) -> Response {
    let mut handler = std::pin::pin!(next.run(request));
    let answered = std::future::poll_fn(|context| match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| std::future::Future::poll(handler.as_mut(), context))) {
        Ok(std::task::Poll::Ready(response)) => std::task::Poll::Ready(Some(response)),
        Ok(std::task::Poll::Pending) => std::task::Poll::Pending,
        Err(_) => std::task::Poll::Ready(None),
    })
    .await;
    let response = answered.unwrap_or_else(|| {
        let mut record = TraceRecord::new("server.request", TraceOutcome::Failed);
        record.detail = Some("handler-panicked".into());
        tracer.emit(record);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    });
    typed_refusal(response)
}

/// 🏷️ Stamps one response as a typed refusal (see [`refusal_middleware`]); a success passes unchanged.
fn typed_refusal(mut response: Response) -> Response {
    if let Some(code) = semio_hub::refusal::refusal_code(response.status().as_u16()) {
        if !response.headers().contains_key(semio_hub::refusal::HUB_REFUSAL_HEADER) {
            response.headers_mut().insert(semio_hub::refusal::HUB_REFUSAL_HEADER, axum::http::HeaderValue::from_static(code));
        }
    }
    response
}

/// 🛡️ Refuses any request a trusted proxy reports as having reached the client in cleartext.
///
/// Router-wide and outside every other layer: a hub whose transport is compromised must not answer
/// at all, not answer differently per route.
pub(crate) async fn transport_security_middleware(State(trust): State<ForwardedTlsTrustV1>, request: axum::extract::Request, next: axum::middleware::Next) -> Response {
    if !forwarded_request_is_secure(request.headers(), trust) {
        let mut response = Response::builder().status(StatusCode::FORBIDDEN).body(axum::body::Body::empty()).unwrap_or_default();
        response.headers_mut().insert("x-semio-refusal", axum::http::HeaderValue::from_static("insecure-transport"));
        return response;
    }
    next.run(request).await
}
//#endregion 🔖️AdminAuth

//#region 🔖️Directory
/// 🚦️ Charges every rate-limited route family before its handler runs: credential sign-in
/// and sign-out, directory commands, invite redemption and socket-grant issuance. Each request is
/// charged to its remote address and, when it carries one, to its session capability's public
/// selector; the handler for `POST /auth/sessions` adds the claimed identity's own bucket, which
/// only exists after the body is decoded. Every other route passes through untouched.
pub(crate) async fn rate_limit_middleware(State(state): State<HubState>, request: axum::extract::Request, next: axum::middleware::Next) -> Response {
    let Some(class) = rate_limit_class(request.method(), request.uri().path()) else { return next.run(request).await };
    let mut subjects = Vec::with_capacity(2);
    match request.extensions().get::<axum::extract::ConnectInfo<SocketAddr>>() {
        Some(axum::extract::ConnectInfo(peer)) => subjects.push(RateLimitSubjectV1::remote_address(&peer.ip())),
        None => subjects.push(RateLimitSubjectV1::principal("connect-info-absent")),
    }
    if let Some(selector) = bearer(request.headers()).and_then(|token| SessionCapability::parse(&token).ok()).map(|capability| capability.selector().to_string()) {
        subjects.push(RateLimitSubjectV1::principal(&selector));
    }
    match state.rate_limits.admit(class, &subjects) {
        RateLimitDecisionV1::Admitted => next.run(request).await,
        RateLimitDecisionV1::Refused { retry_after_ms } => match class {
            RateLimitClassV1::Auth => {
                state.note("server.rate-limit", TraceOutcome::Refused, "auth");
                auth_error_response(AuthErrorCodeV1::RateLimited, Some(retry_after_ms))
            }
            _ => {
                state.note("server.rate-limit", TraceOutcome::Refused, class.as_str());
                rate_limit_refusal_response(RateLimitRefusalV1::new(class, retry_after_ms))
            }
        },
    }
}

/// 🚦️ The `429` of every non-auth rate-limited route family and of every stream-limited one: the typed refusal body, never
/// cached, with its wait also as `retry-after` in whole seconds.
pub(crate) fn rate_limit_refusal_response(refusal: RateLimitRefusalV1) -> Response {
    let retry_after_ms = refusal.retry_after_ms;
    let mut response = (StatusCode::TOO_MANY_REQUESTS, Json(refusal)).into_response();
    response.headers_mut().insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("no-store"));
    if let Ok(value) = axum::http::HeaderValue::from_str(&RateLimitDecisionV1::Refused { retry_after_ms }.retry_after_secs().to_string()) {
        response.headers_mut().insert(axum::http::header::RETRY_AFTER, value);
    }
    response
}

/// 🏷️ Which rate-limit family a request belongs to, by method and path alone.
fn rate_limit_class(method: &axum::http::Method, path: &str) -> Option<RateLimitClassV1> {
    match (method, path) {
        (&axum::http::Method::POST, semio_hub::auth::SESSION_MINT_ROUTE) | (&axum::http::Method::POST, semio_hub::auth::CREDENTIAL_ROUTE) | (&axum::http::Method::POST, semio_hub::auth::SESSION_SIGN_OUT_ROUTE) => Some(RateLimitClassV1::Auth),
        (&axum::http::Method::POST, "/directory/commands") => Some(RateLimitClassV1::DirectoryCommand),
        (&axum::http::Method::POST, path) if path.starts_with("/directory/invites/") && path.ends_with("/redeem") => Some(RateLimitClassV1::InviteRedemption),
        (&axum::http::Method::POST, path) if path.ends_with("/socket-grants") => Some(RateLimitClassV1::SocketGrant),
        _ => None,
    }
}

/// 🌐️ Which browser origins this deployment admits for *credentialed* cross-origin
/// requests, resolved once at startup from `OS_HUB_ALLOWED_ORIGINS` and the bind address.
///
/// The hub's dev topology serves the shell from a different port than the hub itself, so it needs
/// a real cross-origin grant, not `*` — `access-control-allow-credentials: true` and `*` are
/// mutually exclusive by specification. A configured allowlist is therefore the only shape that
/// serves both, and the default is decided by **where the socket is**: a loopback bind is a
/// developer's own machine and admits any loopback origin whatever port Vite picked; a bind that
/// answers on a network interface admits nothing until a deployer names the origins out loud.
///
/// See {@link CrossOriginPolicyV1::from_environment} for the exact precedence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum CrossOriginPolicyV1 {
    /// 🏠️ Any `http`/`https` origin whose host is `localhost` or a loopback address, on any port.
    LoopbackDevelopment,
    /// 📋️ Exactly the origins a deployer named, compared whole and ASCII-case-insensitively.
    Allowlist(Arc<[String]>),
    /// 🚪️ No cross-origin credentialed access at all — same-origin requests are unaffected.
    Closed,
}

impl CrossOriginPolicyV1 {
    /// 🌐️ `OS_HUB_ALLOWED_ORIGINS=https://s.example.com,https://admin.example.com` wins
    /// whenever it is set and non-empty. With it unset the policy follows `OS_HUB_BIND`: loopback
    /// (the default *development* posture, and what `OS_HUB_BIND` must be for `OS_HUB_MODE=production`
    /// to boot at all) admits loopback origins, and any other bind admits none.
    pub(crate) fn from_environment(bind: std::net::IpAddr) -> Result<Self, HubError> {
        const ALLOWED_ORIGINS_MAX: usize = 32;
        let Some(encoded) = std::env::var("OS_HUB_ALLOWED_ORIGINS").ok().filter(|value| !value.trim().is_empty()) else {
            return Ok(if bind.is_loopback() { Self::LoopbackDevelopment } else { Self::Closed });
        };
        let mut origins: Vec<String> = Vec::new();
        for entry in encoded.split(',') {
            let origin = entry.trim();
            if origin.is_empty() {
                continue;
            }
            if origins.len() == ALLOWED_ORIGINS_MAX {
                return Err(HubError::UnsafeAuthConfiguration("OS_HUB_ALLOWED_ORIGINS exceeds 32 entries".into()));
            }
            if !is_browser_origin(origin) {
                return Err(HubError::UnsafeAuthConfiguration(format!("OS_HUB_ALLOWED_ORIGINS entry {origin:?} is not a scheme://host[:port] origin")));
            }
            if !origins.iter().any(|existing| existing.eq_ignore_ascii_case(origin)) {
                origins.push(origin.to_string());
            }
        }
        if origins.is_empty() {
            return Err(HubError::UnsafeAuthConfiguration("OS_HUB_ALLOWED_ORIGINS names no origin".into()));
        }
        Ok(Self::Allowlist(origins.into()))
    }

    /// ✅️ Whether one request's `Origin` header value earns the credentialed grant.
    pub(crate) fn admits(&self, origin: &str) -> bool {
        match self {
            Self::LoopbackDevelopment => is_loopback_origin(origin),
            Self::Allowlist(origins) => origins.iter().any(|allowed| allowed.eq_ignore_ascii_case(origin)),
            Self::Closed => false,
        }
    }

    /// 🏷️ The stable word the startup line and the report use for this posture.
    pub(crate) fn label(&self) -> &'static str {
        match self {
            Self::LoopbackDevelopment => "loopback-development",
            Self::Allowlist(_) => "allowlist",
            Self::Closed => "closed",
        }
    }
}

/// 🌐️ A serialized origin: `scheme://host[:port]` with no userinfo, path, query, fragment or
/// wildcard. Anything else in an allowlist is a deployer's mistake, not a pattern to interpret.
pub(crate) fn is_browser_origin(value: &str) -> bool {
    let Some((scheme, rest)) = value.split_once("://") else { return false };
    if !scheme.eq_ignore_ascii_case("http") && !scheme.eq_ignore_ascii_case("https") {
        return false;
    }
    if rest.is_empty() || rest.contains(['/', '?', '#', '@', '*', ' ', '\t']) {
        return false;
    }
    let host = match rest.strip_prefix('[') {
        Some(bracketed) => {
            let Some((inside, port)) = bracketed.split_once(']') else { return false };
            if inside.parse::<std::net::Ipv6Addr>().is_err() || !is_origin_port(port) {
                return false;
            }
            return true;
        }
        None => rest,
    };
    match host.split_once(':') {
        Some((name, port)) => !name.is_empty() && is_origin_port(&format!(":{port}")),
        None => true,
    }
}

/// 🔢️ An origin's optional `:port` suffix — empty, or a colon and a decimal port.
fn is_origin_port(suffix: &str) -> bool {
    match suffix.strip_prefix(':') {
        None => suffix.is_empty(),
        Some(port) => !port.is_empty() && port.parse::<u16>().is_ok(),
    }
}

/// 🏠️ Whether an origin's host is this machine — `localhost`, `127.0.0.0/8` or `[::1]`.
pub(crate) fn is_loopback_origin(value: &str) -> bool {
    if !is_browser_origin(value) {
        return false;
    }
    let Some((_, rest)) = value.split_once("://") else { return false };
    let host = match rest.strip_prefix('[') {
        Some(bracketed) => match bracketed.split_once(']') {
            Some((inside, _)) => return inside.parse::<std::net::Ipv6Addr>().is_ok_and(|address| address.is_loopback()),
            None => return false,
        },
        None => rest.split_once(':').map_or(rest, |(name, _)| name),
    };
    host.eq_ignore_ascii_case("localhost") || host.parse::<std::net::IpAddr>().is_ok_and(|address| address.is_loopback())
}

/// 🌐️ Applies the hub's explicit cross-origin response policy. The only public issuance
/// route is `POST /auth/sessions`, which a deployment must enable explicitly and which is rate
/// limited per remote address and per claimed identity; every protected route still requires its
/// own typed capability.
pub(crate) async fn cors_middleware(State(policy): State<CrossOriginPolicyV1>, request: axum::extract::Request, next: axum::middleware::Next) -> Response {
    let origin = request.headers().get(axum::http::header::ORIGIN).cloned();
    if request.method() == axum::http::Method::OPTIONS {
        let mut response = Response::builder().status(StatusCode::NO_CONTENT).body(axum::body::Body::empty()).unwrap_or_default();
        apply_cors_headers(response.headers_mut(), origin.as_ref(), &policy);
        return response;
    }
    let mut response = next.run(request).await;
    apply_cors_headers(response.headers_mut(), origin.as_ref(), &policy);
    response
}

/// 🌐️ See {@link cors_middleware}. Reflects the request's own `Origin` (never `*`) only when
/// {@link CrossOriginPolicyV1} admits it, plus the bearer/JSON headers and verbs this control plane
/// actually uses. `Vary: Origin` is appended for every request that carried an `Origin` — including
/// a refused one — so a shared cache never serves one origin's grant to another.
fn apply_cors_headers(headers: &mut HeaderMap, origin: Option<&axum::http::HeaderValue>, policy: &CrossOriginPolicyV1) {
    if let Some(origin) = origin {
        headers.append(axum::http::header::VARY, axum::http::HeaderValue::from_static("Origin"));
        if origin.to_str().is_ok_and(|value| policy.admits(value)) {
            headers.insert(axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, origin.clone());
            headers.insert(axum::http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS, axum::http::HeaderValue::from_static("true"));
        }
    }
    headers.insert(axum::http::header::ACCESS_CONTROL_ALLOW_METHODS, axum::http::HeaderValue::from_static("GET, POST, PUT, HEAD, OPTIONS"));
    headers.insert(axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS, axum::http::HeaderValue::from_static("authorization, content-type"));
}
//#endregion 🔖️Directory

//#region 🔖️Admin
/// 🛣️ Counts every answer of a matched route under its method and route template, with the time the
/// route took to answer — the per-route table of `GET /admin/api/observability`. It runs inside routing
/// (`route_layer`), so the template is known and a concrete path never becomes a key, and outside
/// [`rate_limit_middleware`], so an address-bucket refusal is counted as the route's `rateLimited`.
pub(crate) async fn route_metrics_middleware(State(metrics): State<Arc<HubRouteMetricsV1>>, request: axum::extract::Request, next: axum::middleware::Next) -> Response {
    let method = request.method().as_str().to_string();
    let route = request.extensions().get::<axum::extract::MatchedPath>().map_or_else(|| "unmatched".to_string(), |matched| matched.as_str().to_string());
    let started = std::time::Instant::now();
    let response = next.run(request).await;
    metrics.observe(&method, &route, response.status().as_u16(), started.elapsed().as_micros().min(u128::from(u64::MAX)) as u64);
    response
}
//#endregion 🔖️Admin
