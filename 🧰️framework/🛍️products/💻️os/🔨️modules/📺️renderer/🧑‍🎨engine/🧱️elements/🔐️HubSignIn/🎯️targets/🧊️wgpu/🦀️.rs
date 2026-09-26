//! 🔐️ wgpu twin of the `🔐️HubSignIn` pane and of the pure contract underneath it
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔐️sign-in/🟦️.ts`, 653 lines).
//!
//! 🧩️ Split of ownership: everything in this file is PURE — routes and bounds, the closed denial
//! classes, the `session.v1.<32hex>.<64hex>` capability parser, the origin normalizer, the
//! local-only hub connection book, and the session reducer. No transport, no renderer, no clock.
//! The transport lives in `🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs` (generic over the shell's own
//! `DirectoryTransport`), the retained tree lives there too, and the shell only ever calls those.
//!
//! ⚖️ Why a second implementation rather than a schema read: the token grammar, the reducer's
//! precedence laws and the mint body's field order are hand-written contract on the TS side with no
//! schema-driven generator behind them (AU2 §2 "Decisions worth defending"). The parity mechanism is
//! therefore the LANGUAGE-AGNOSTIC FIXTURE both sides read — `📇️directory/🔐️sign-in/🔣️.json` — which
//! this module's tests drive through these functions exactly as the vitest suite drives the TS ones.
//!
//! 🔑️ The minted capability never enters this module's persistable state. [`HubConnectionBook`]
//! carries hub identity only (`id, kind, label, lastUserId, origin`); a test asserts the serialized
//! book contains no token, no password and no session id.

use serde::{Deserialize, Serialize};
use ui_wgpu::wgpu::Locale;

//#region 🔖️Routes
/// 🛣️ `POST` — AU1's `semio_hub::auth::SESSION_MINT_ROUTE`.
pub const HUB_SESSION_MINT_PATH_V1: &str = "/auth/sessions";
/// 🛣️ `GET` — AU1's `SESSION_ME_ROUTE`; the only place the session's own expiry is served.
pub const HUB_SESSION_ME_PATH_V1: &str = "/auth/sessions/me";
/// 🚪️ `POST` — AU1's `SESSION_SIGN_OUT_ROUTE`, the self sign-out command.
pub const HUB_SESSION_SIGN_OUT_PATH_V1: &str = "/auth/sessions/me/sign-out";
/// 🏷️ The mint request's declared schema.
pub const HUB_SIGN_IN_REQUEST_SCHEMA_V1: &str = "semio.hub.auth.credential-sign-in/v1";
/// 🏷️ AU1's error body schema, read only to recover a rate-limit countdown.
pub const HUB_AUTH_ERROR_SCHEMA_V1: &str = "semio.hub.auth.error/v1";
/// 🏷️ The `me` record's schema (`📇️directory/🧬️schema/🪪️session-authority-v1`).
pub const HUB_SESSION_AUTHORITY_SCHEMA_V1: &str = "semio.directory.session-authority.v1";

/// 📏️ `SIGN_IN_REQUEST_MAX_BYTES` in `🌎️hub/🔐️auth/🦀️.rs` — axum refuses a larger body with 413.
pub const HUB_SESSION_MINT_REQUEST_MAX_BYTES: usize = 1024;
/// 📏️ Ceiling on the mint answer, so a proxy's HTML page is refused before it is parsed.
pub const HUB_SESSION_MINT_RESPONSE_MAX_BYTES: usize = 8 * 1024;
/// 📏️ `DIRECTORY_SESSION_AUTHORITY_MAX_BYTES`.
pub const HUB_SESSION_AUTHORITY_MAX_BYTES: usize = 2048;
/// ⏳️ One sign-in attempt's whole deadline.
pub const HUB_SIGN_IN_TIMEOUT_MS: u64 = 10_000;
/// ⏳️ A countdown longer than a day is a hub defect, not a value to render.
pub const HUB_SIGN_IN_RATE_LIMIT_MAX_SECONDS: u64 = 24 * 60 * 60;
pub const HUB_SIGN_IN_EMAIL_MIN_BYTES: usize = 3;
pub const HUB_SIGN_IN_EMAIL_MAX_BYTES: usize = 254;
pub const HUB_SIGN_IN_PASSWORD_MIN_BYTES: usize = 8;
pub const HUB_SIGN_IN_PASSWORD_MAX_BYTES: usize = 256;
pub const HUB_SIGN_IN_DEVICE_INSTANCE_MAX_BYTES: usize = 128;
//#endregion 🔖️Routes

//#region 🌐️Text
/// 🌐️ Every string the sign-in surface can show, in the two languages this product owns. An
/// exhaustive match over the generated [`Locale`], so adding a third locale is a compile error here
/// rather than a silently English screen — the Rust equivalent of AU2's `registerUiTranslationBundles`
/// type, and of its `hubSignInTextV1("fr")` refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HubSignInText {
    pub invalid_credentials: &'static str,
    pub rate_limited: &'static str,
    pub unreachable: &'static str,
    pub hub_refused: &'static str,
    pub invalid_response: &'static str,
    pub cancelled: &'static str,
    pub expired: &'static str,
    pub invalid_origin: &'static str,
    pub password_disabled: &'static str,
    pub malformed_request: &'static str,
    pub short_password: &'static str,
    pub invalid_email: &'static str,
}

const HUB_SIGN_IN_TEXT_EN: HubSignInText = HubSignInText {
    invalid_credentials: "That email and password do not match an account on this hub.",
    rate_limited: "Too many sign-in attempts. Try again in {{seconds}} s.",
    unreachable: "This hub cannot be reached. You can keep working locally; sign in again when the connection is back.",
    hub_refused: "This hub refused the sign-in request.",
    invalid_response: "This hub answered with something this app cannot read.",
    cancelled: "Sign-in was cancelled.",
    expired: "Your session on this hub expired. Sign in again to continue collaborating.",
    invalid_origin: "Enter a hub address such as https://hub.example.org.",
    password_disabled: "This hub does not accept password sign-in. Open it from its own launcher instead.",
    malformed_request: "This app sent something this hub could not read. Check the address and try again.",
    short_password: "Passwords on this hub are at least 8 characters.",
    invalid_email: "Enter the email address you use on this hub.",
};

const HUB_SIGN_IN_TEXT_DE: HubSignInText = HubSignInText {
    invalid_credentials: "E-Mail und Passwort passen zu keinem Konto auf diesem Hub.",
    rate_limited: "Zu viele Anmeldeversuche. Versuche es in {{seconds}} s erneut.",
    unreachable: "Dieser Hub ist nicht erreichbar. Du kannst lokal weiterarbeiten und dich erneut anmelden, sobald die Verbindung wieder steht.",
    hub_refused: "Dieser Hub hat die Anmeldung abgelehnt.",
    invalid_response: "Dieser Hub hat mit etwas geantwortet, das diese App nicht lesen kann.",
    cancelled: "Die Anmeldung wurde abgebrochen.",
    expired: "Deine Sitzung auf diesem Hub ist abgelaufen. Melde dich erneut an, um weiter zusammenzuarbeiten.",
    invalid_origin: "Gib eine Hub-Adresse ein, zum Beispiel https://hub.example.org.",
    password_disabled: "Dieser Hub akzeptiert keine Passwort-Anmeldung. Öffne ihn stattdessen über seinen eigenen Starter.",
    malformed_request: "Diese App hat etwas gesendet, das dieser Hub nicht lesen konnte. Prüfe die Adresse und versuche es erneut.",
    short_password: "Passwörter auf diesem Hub haben mindestens 8 Zeichen.",
    invalid_email: "Gib die E-Mail-Adresse ein, die du auf diesem Hub verwendest.",
};

/// 🔡️ This locale's whole string table.
pub fn hub_sign_in_text(locale: Locale) -> HubSignInText {
    match locale {
        Locale::En => HUB_SIGN_IN_TEXT_EN,
        Locale::De => HUB_SIGN_IN_TEXT_DE,
    }
}

/// 🏷️ The chrome labels around the form. Kept beside the denial texts so one locale can never be
/// half-translated: both columns are written in the same match.
pub fn hub_sign_in_label(key: HubSignInLabel, locale: Locale) -> &'static str {
    match (key, locale) {
        (HubSignInLabel::Title, Locale::En) => "Sign in to a hub",
        (HubSignInLabel::Title, Locale::De) => "Bei einem Hub anmelden",
        (HubSignInLabel::Hub, Locale::En) => "Hub",
        (HubSignInLabel::Hub, Locale::De) => "Hub",
        (HubSignInLabel::Email, Locale::En) => "Email",
        (HubSignInLabel::Email, Locale::De) => "E-Mail",
        (HubSignInLabel::Password, Locale::En) => "Password",
        (HubSignInLabel::Password, Locale::De) => "Passwort",
        (HubSignInLabel::Submit, Locale::En) => "Sign in",
        (HubSignInLabel::Submit, Locale::De) => "Anmelden",
        (HubSignInLabel::Cancel, Locale::En) => "Cancel",
        (HubSignInLabel::Cancel, Locale::De) => "Abbrechen",
        (HubSignInLabel::SignOut, Locale::En) => "Sign out",
        (HubSignInLabel::SignOut, Locale::De) => "Abmelden",
        (HubSignInLabel::AddHub, Locale::En) => "Add a hub",
        (HubSignInLabel::AddHub, Locale::De) => "Hub hinzufügen",
        (HubSignInLabel::ForgetHub, Locale::En) => "Forget this hub",
        (HubSignInLabel::ForgetHub, Locale::De) => "Hub vergessen",
        (HubSignInLabel::HubAddress, Locale::En) => "Hub address",
        (HubSignInLabel::HubAddress, Locale::De) => "Hub-Adresse",
        (HubSignInLabel::LocalOnly, Locale::En) => "Working on this device only.",
        (HubSignInLabel::LocalOnly, Locale::De) => "Es wird nur auf diesem Gerät gearbeitet.",
        (HubSignInLabel::SigningIn, Locale::En) => "Signing in…",
        (HubSignInLabel::SigningIn, Locale::De) => "Anmeldung läuft…",
        (HubSignInLabel::SigningOut, Locale::En) => "Signing out…",
        (HubSignInLabel::SigningOut, Locale::De) => "Abmeldung läuft…",
        (HubSignInLabel::SignedInAs, Locale::En) => "Signed in as",
        (HubSignInLabel::SignedInAs, Locale::De) => "Angemeldet als",
        (HubSignInLabel::ThisDevice, Locale::En) => "This device",
        (HubSignInLabel::ThisDevice, Locale::De) => "Dieses Gerät",
    }
}

/// 🏷️ The closed set of chrome labels the sign-in surface names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubSignInLabel {
    Title,
    Hub,
    Email,
    Password,
    Submit,
    Cancel,
    SignOut,
    AddHub,
    ForgetHub,
    HubAddress,
    LocalOnly,
    SigningIn,
    SigningOut,
    SignedInAs,
    ThisDevice,
}
//#endregion 🌐️Text

//#region 🚫️Errors
/// 🚫️ Closed sign-in denial classes. `InvalidCredentials` deliberately covers unknown email, absent
/// credential and wrong password alike — AU1 §1.1 emits one uniform code so the route cannot be used
/// to enumerate users, and a UI that distinguished them would undo that.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubSignInErrorCode {
    InvalidCredentials,
    RateLimited,
    Unreachable,
    HubRefused,
    InvalidResponse,
    Cancelled,
    PasswordSignInDisabled,
    MalformedRequest,
}

impl HubSignInErrorCode {
    /// 🏷️ The wire spelling the React twin stamps as its error class, so one probe reads both.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InvalidCredentials => "invalid-credentials",
            Self::RateLimited => "rate-limited",
            Self::Unreachable => "unreachable",
            Self::HubRefused => "hub-refused",
            Self::InvalidResponse => "invalid-response",
            Self::Cancelled => "cancelled",
            Self::PasswordSignInDisabled => "password-sign-in-disabled",
            Self::MalformedRequest => "malformed-request",
        }
    }

    fn text(self, table: HubSignInText) -> &'static str {
        match self {
            Self::InvalidCredentials => table.invalid_credentials,
            Self::RateLimited => table.rate_limited,
            Self::Unreachable => table.unreachable,
            Self::HubRefused => table.hub_refused,
            Self::InvalidResponse => table.invalid_response,
            Self::Cancelled => table.cancelled,
            Self::PasswordSignInDisabled => table.password_disabled,
            Self::MalformedRequest => table.malformed_request,
        }
    }
}

/// 🗣️ One denial class as the human reads it, with the rate-limit countdown substituted.
pub fn hub_sign_in_error_text(locale: Locale, code: HubSignInErrorCode, retry_after_seconds: Option<u64>) -> String {
    let text = code.text(hub_sign_in_text(locale));
    if code == HubSignInErrorCode::RateLimited {
        return text.replace("{{seconds}}", &retry_after_seconds.unwrap_or(0).to_string());
    }
    text.to_string()
}

/// 🌐️ AU1 §1.1's status table. 413 is axum's body-limit rejection, which never reaches the handler
/// and therefore carries no error body — it is still this app's own malformed request.
pub fn hub_sign_in_error_from_status(status: u16) -> HubSignInErrorCode {
    match status {
        400 | 413 | 415 => HubSignInErrorCode::MalformedRequest,
        401 => HubSignInErrorCode::InvalidCredentials,
        403 => HubSignInErrorCode::PasswordSignInDisabled,
        429 => HubSignInErrorCode::RateLimited,
        408 => HubSignInErrorCode::Unreachable,
        other if other >= 500 => HubSignInErrorCode::Unreachable,
        _ => HubSignInErrorCode::HubRefused,
    }
}

/// ⏳️ Reads `retryAfterMs` out of AU1's `semio.hub.auth.error/v1` body. This is the ONLY countdown
/// source on this renderer: the shell's `DirectoryTransport::http` answers `{status, body}` with no
/// header map, so the `Retry-After` header the React twin prefers is structurally unavailable here.
/// [`hub_retry_after_seconds`] is ported beside it and unit-tested against the shared fixture so the
/// two implementations still agree on the grammar, but the live path reads the body.
pub fn hub_auth_error_retry_after_seconds(body: &str) -> Option<u64> {
    let value: serde_json::Value = serde_json::from_str(body).ok()?;
    let object = value.as_object()?;
    if object.get("schema").and_then(serde_json::Value::as_str) != Some(HUB_AUTH_ERROR_SCHEMA_V1) {
        return None;
    }
    let ms = object.get("retryAfterMs")?.as_u64()?;
    let seconds = ms.div_ceil(1000);
    (seconds <= HUB_SIGN_IN_RATE_LIMIT_MAX_SECONDS).then_some(seconds)
}

/// ⏳️ A `Retry-After` delta-seconds header, clamped to a day. An HTTP-date form, a sign or any
/// non-digit yields `None` rather than a fabricated countdown.
pub fn hub_retry_after_seconds(header: Option<&str>) -> Option<u64> {
    let raw = header?.trim();
    if raw.is_empty() || raw.len() > 6 || !raw.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let seconds = raw.parse::<u64>().ok()?;
    (seconds <= HUB_SIGN_IN_RATE_LIMIT_MAX_SECONDS).then_some(seconds)
}
//#endregion 🚫️Errors

//#region 🎫️MintWire
/// 🎫️ `POST /auth/sessions`'s answer. The wire is snake_case here and only here: that route predates
/// the camelCase wave and `SessionMintResponse` (`📇️directory/🔌️client/🦀️.rs:216`) documents it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HubSessionMintResult {
    pub token: String,
    pub user_id: String,
}

/// 🔐️ The credential posted for exactly one attempt. Held for the duration of that call and dropped;
/// never persisted, never logged, never placed in a URL.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HubSignInCredential {
    pub email: String,
    pub password: String,
    pub device_instance_id: String,
    pub client_class: HubSignInClientClass,
}

/// 🖥️ Which kind of client is asking. A wgpu shell is `native` when it owns a window and `browser`
/// when it runs in the page's worker — the same distinction the door itself is split on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubSignInClientClass {
    Browser,
    Native,
    Cli,
}

impl HubSignInClientClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Browser => "browser",
            Self::Native => "native",
            Self::Cli => "cli",
        }
    }
}

#[derive(Serialize)]
struct HubSessionMintRequestBody<'a> {
    schema: &'a str,
    email: String,
    password: &'a str,
    #[serde(rename = "deviceInstanceId")]
    device_instance_id: &'a str,
    #[serde(rename = "clientClass")]
    client_class: &'a str,
}

/// 🎫️ AU1 §1.1's capability grammar `session.v1.<32 lower-hex>.<64 lower-hex>`, checked exactly, so
/// a proxy's login page or an error string can never be installed as a session.
pub fn valid_hub_session_token(token: &str) -> bool {
    valid_capability_token(token, "session.v1.")
}

fn valid_capability_token(token: &str, prefix: &str) -> bool {
    let Some(rest) = token.strip_prefix(prefix) else { return false };
    let Some((selector, secret)) = rest.split_once('.') else { return false };
    selector.len() == 32 && secret.len() == 64 && lower_hex(selector) && lower_hex(secret)
}

fn lower_hex(value: &str) -> bool {
    value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn bounded_opaque_id(value: &str, max_bytes: usize) -> bool {
    !value.is_empty() && value.len() <= max_bytes && !value.chars().any(char::is_control)
}

/// ✉️ The hub's own `$defs/CredentialSignInRequestV1.email` admission, mirrored so an obviously wrong
/// field never costs a round trip against a rate-limited route: 3..=254 bytes, exactly one `@`, and
/// neither part carrying a byte at or below U+0020 or U+007F
/// (`🌎️hub/🔐️auth/🧬️schema/🔣️.json` pattern `^[^\x00- @\x7f]+@[^\x00- @\x7f]+$`).
pub fn valid_hub_sign_in_email(email: &str) -> bool {
    if email.len() < HUB_SIGN_IN_EMAIL_MIN_BYTES || email.len() > HUB_SIGN_IN_EMAIL_MAX_BYTES {
        return false;
    }
    let Some((local, domain)) = email.split_once('@') else { return false };
    valid_email_part(local) && valid_email_part(domain)
}

fn valid_email_part(part: &str) -> bool {
    !part.is_empty() && !part.chars().any(|character| character <= '\u{0020}' || character == '@' || character == '\u{007f}')
}

/// 🔑️ AU1 §1.1's password admission: 8..=256 bytes, no control characters.
pub fn valid_hub_sign_in_password(password: &str) -> bool {
    password.len() >= HUB_SIGN_IN_PASSWORD_MIN_BYTES && password.len() <= HUB_SIGN_IN_PASSWORD_MAX_BYTES && !password.chars().any(char::is_control)
}

/// 🖥️ The device-instance grammar `^[A-Za-z0-9._:-]+$`, bounded at 128 bytes.
pub fn valid_hub_device_instance_id(device: &str) -> bool {
    !device.is_empty() && device.len() <= HUB_SIGN_IN_DEVICE_INSTANCE_MAX_BYTES && device.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-'))
}

/// 📤️ Builds the exact mint body in AU1 §1.1's declared field order
/// (`schema, email, password, deviceInstanceId, clientClass`). An out-of-bounds field is refused
/// locally so a malformed credential costs neither a round trip nor a token from the sign-in bucket.
pub fn hub_session_mint_request_json(credential: &HubSignInCredential) -> Result<String, HubSignInErrorCode> {
    let email = credential.email.trim();
    if !valid_hub_sign_in_email(email) || !valid_hub_sign_in_password(&credential.password) || !valid_hub_device_instance_id(&credential.device_instance_id) {
        return Err(HubSignInErrorCode::MalformedRequest);
    }
    let body = HubSessionMintRequestBody { schema: HUB_SIGN_IN_REQUEST_SCHEMA_V1, email: email.to_lowercase(), password: &credential.password, device_instance_id: &credential.device_instance_id, client_class: credential.client_class.as_str() };
    let json = serde_json::to_string(&body).map_err(|_| HubSignInErrorCode::MalformedRequest)?;
    if json.len() > HUB_SESSION_MINT_REQUEST_MAX_BYTES {
        return Err(HubSignInErrorCode::MalformedRequest);
    }
    Ok(json)
}

/// 📥️ Parses exactly `{token, user_id}`. An extra field, a short token or a control character is a
/// refusal, so a proxy's error page can never be mistaken for a session.
pub fn parse_hub_session_mint_result(source: &str) -> Option<HubSessionMintResult> {
    if source.len() > HUB_SESSION_MINT_RESPONSE_MAX_BYTES {
        return None;
    }
    let value: serde_json::Value = serde_json::from_str(source).ok()?;
    let object = value.as_object()?;
    if object.len() != 2 {
        return None;
    }
    let token = object.get("token")?.as_str()?;
    let user_id = object.get("user_id")?.as_str()?;
    if !valid_hub_session_token(token) || !bounded_opaque_id(user_id, 256) {
        return None;
    }
    Some(HubSessionMintResult { token: token.to_string(), user_id: user_id.to_string() })
}

/// 🪪️ The exact `GET /auth/sessions/me` record the shell reads back. `expires_at_ms` is the whole
/// reason the call exists: AU1 keeps the expiry out of the mint answer, so the deadline the shell
/// re-authenticates against can only come from here (AU3 §4.2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HubSessionAuthority {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub expires_at_ms: i64,
    pub session_kind: HubSessionKind,
    pub authorization_generation: u64,
}

/// 🪪️ Which authority minted this session — a credential sign-in or the dev launcher's own pipe.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubSessionKind {
    External,
    DevelopmentLocal,
}

/// 📥️ Decodes `DirectorySessionAuthorityV1`, refusing an unknown schema, a non-positive `expiresAt`,
/// an unknown `sessionKind` or a proxy's HTML — so nothing but the hub's own record installs an
/// identity.
pub fn parse_hub_session_authority(source: &str) -> Option<HubSessionAuthority> {
    if source.len() > HUB_SESSION_AUTHORITY_MAX_BYTES {
        return None;
    }
    let value: serde_json::Value = serde_json::from_str(source).ok()?;
    let object = value.as_object()?;
    if object.get("schema").and_then(serde_json::Value::as_str) != Some(HUB_SESSION_AUTHORITY_SCHEMA_V1) {
        return None;
    }
    let user_id = object.get("userId")?.as_str()?;
    let email = object.get("email")?.as_str()?;
    let display_name = object.get("displayName")?.as_str()?;
    let expires_at_ms = object.get("expiresAt")?.as_i64()?;
    let session_kind = match object.get("sessionKind")?.as_str()? {
        "external" => HubSessionKind::External,
        "development-local" => HubSessionKind::DevelopmentLocal,
        _ => return None,
    };
    let authorization_generation = object.get("authorizationGeneration")?.as_u64()?;
    if !bounded_opaque_id(user_id, 256) || !bounded_opaque_id(email, 320) || !bounded_opaque_id(display_name, 128) || expires_at_ms < 1 || authorization_generation < 1 {
        return None;
    }
    Some(HubSessionAuthority { user_id: user_id.to_string(), email: email.to_string(), display_name: display_name.to_string(), expires_at_ms, session_kind, authorization_generation })
}
//#endregion 🎫️MintWire

//#region 📕️Connections
/// 🏛️ Which kind of hub an entry names: the same-origin one the dev launcher inherits a credential
/// for, or one this human typed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HubConnectionKind {
    LocalBootstrap,
    Remote,
}

/// 🏛️ One hub this device knows about. `origin` is a bare scheme+authority — no path, query,
/// fragment or embedded credentials — so it can never smuggle a capability through the book.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HubConnection {
    pub id: String,
    pub kind: HubConnectionKind,
    pub label: String,
    #[serde(rename = "lastUserId")]
    pub last_user_id: Option<String>,
    pub origin: String,
}

pub const HUB_CONNECTION_BOOK_SCHEMA_V1: &str = "semio.os.hub-connection-book.v1";
pub const HUB_CONNECTION_BOOK_STORAGE_KEY_V1: &str = "semio.os.hub-connection-book.v1";
pub const HUB_CONNECTION_BOOK_MAX_ENTRIES: usize = 16;
pub const LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1: &str = "local-bootstrap";

/// 📕️ The persisted-local-only hub book: which hubs this profile knows and which one is selected.
/// It carries no token, no password and no session id — only hub identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HubConnectionBook {
    pub connections: Vec<HubConnection>,
    pub selected_id: String,
}

#[derive(Serialize)]
struct HubConnectionBookWire<'a> {
    schema: &'a str,
    connections: Vec<&'a HubConnection>,
    #[serde(rename = "selectedId")]
    selected_id: &'a str,
}

/// 🏠️ The always-present local bootstrap entry. Kept first, so an offline device still has a usable
/// selection and a corrupt store can never leave the book empty.
pub fn local_bootstrap_hub_connection(origin: &str, locale: Locale) -> HubConnection {
    HubConnection { id: LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1.to_string(), kind: HubConnectionKind::LocalBootstrap, label: hub_sign_in_label(HubSignInLabel::ThisDevice, locale).to_string(), last_user_id: None, origin: origin.to_string() }
}

/// 🌐️ Normalizes typed hub text to a bare origin: `https://` is added when no scheme is typed, and
/// anything carrying a path, query, fragment, username or password is refused rather than trimmed —
/// a hub address that silently loses a component is how a capability ends up on the wrong host.
pub fn parse_hub_origin(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() || trimmed.len() > 512 || trimmed.chars().any(|character| character.is_control() || character.is_whitespace()) {
        return None;
    }
    let (scheme, authority) = match trimmed.split_once("://") {
        Some((scheme, rest)) => (scheme.to_ascii_lowercase(), rest),
        None => ("https".to_string(), trimmed),
    };
    if scheme != "https" && scheme != "http" {
        return None;
    }
    if authority.contains('@') || authority.contains('?') || authority.contains('#') {
        return None;
    }
    let host = match authority.split_once('/') {
        Some((host, path)) if path.is_empty() => host,
        Some(_) => return None,
        None => authority,
    };
    if host.is_empty() || !valid_hub_authority(host) {
        return None;
    }
    Some(format!("{scheme}://{}", host.to_ascii_lowercase()))
}

/// 🔢️ A bare authority: a host, optionally followed by a decimal port. A bracketed IPv6 literal is
/// accepted whole rather than split on its own colons.
fn valid_hub_authority(host: &str) -> bool {
    if host.starts_with('[') {
        return host.contains(']');
    }
    match host.rsplit_once(':') {
        Some((name, port)) => !name.is_empty() && !port.is_empty() && port.bytes().all(|byte| byte.is_ascii_digit()),
        None => true,
    }
}

/// 🆔️ A stable, collision-free id for a remote hub: its own origin. Two labels for one origin are
/// the same hub, so re-adding it updates rather than duplicates.
pub fn hub_connection_id_for_origin(origin: &str) -> String {
    format!("remote:{origin}")
}

fn valid_hub_connection(value: &serde_json::Value) -> Option<HubConnection> {
    let object = value.as_object()?;
    if object.len() != 5 {
        return None;
    }
    let id = object.get("id")?.as_str()?;
    let label = object.get("label")?.as_str()?;
    let origin = object.get("origin")?.as_str()?;
    let kind = match object.get("kind")?.as_str()? {
        "local-bootstrap" => HubConnectionKind::LocalBootstrap,
        "remote" => HubConnectionKind::Remote,
        _ => return None,
    };
    let last_user_id = match object.get("lastUserId")? {
        serde_json::Value::Null => None,
        serde_json::Value::String(value) => {
            if !bounded_opaque_id(value, 256) {
                return None;
            }
            Some(value.clone())
        }
        _ => return None,
    };
    if !bounded_opaque_id(id, 600) || !bounded_opaque_id(label, 128) || parse_hub_origin(origin).as_deref() != Some(origin) {
        return None;
    }
    Some(HubConnection { id: id.to_string(), kind, label: label.to_string(), last_user_id, origin: origin.to_string() })
}

/// 📖️ Reads the book, repairing rather than failing: unreadable or tampered storage yields the
/// bootstrap-only book, so a corrupt profile never blocks the app from starting locally.
pub fn parse_hub_connection_book(source: Option<&str>, bootstrap_origin: &str, locale: Locale) -> HubConnectionBook {
    let fallback = HubConnectionBook { connections: vec![local_bootstrap_hub_connection(bootstrap_origin, locale)], selected_id: LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1.to_string() };
    let Some(source) = source else { return fallback };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(source) else { return fallback };
    let Some(object) = value.as_object() else { return fallback };
    if object.get("schema").and_then(serde_json::Value::as_str) != Some(HUB_CONNECTION_BOOK_SCHEMA_V1) {
        return fallback;
    }
    let Some(entries) = object.get("connections").and_then(serde_json::Value::as_array) else { return fallback };
    let Some(selected) = object.get("selectedId").and_then(serde_json::Value::as_str) else { return fallback };
    let mut connections = vec![local_bootstrap_hub_connection(bootstrap_origin, locale)];
    for entry in entries {
        if connections.len() >= HUB_CONNECTION_BOOK_MAX_ENTRIES {
            break;
        }
        match valid_hub_connection(entry) {
            Some(connection) if connection.kind == HubConnectionKind::Remote => connections.push(connection),
            _ => continue,
        }
    }
    let selected_id = if connections.iter().any(|entry| entry.id == selected) { selected.to_string() } else { LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1.to_string() };
    HubConnectionBook { connections, selected_id }
}

/// 💾️ Serializes the book in a fixed key order, so two writes of the same state are byte-identical
/// and a diff over the profile store means a real change. The bootstrap entry is derived, never
/// written: persisting it would pin a stale origin across a relocated dev hub.
pub fn serialize_hub_connection_book(book: &HubConnectionBook) -> String {
    let wire = HubConnectionBookWire { schema: HUB_CONNECTION_BOOK_SCHEMA_V1, connections: book.connections.iter().filter(|entry| entry.kind == HubConnectionKind::Remote).collect(), selected_id: &book.selected_id };
    serde_json::to_string(&wire).unwrap_or_else(|_| format!("{{\"schema\":\"{HUB_CONNECTION_BOOK_SCHEMA_V1}\",\"connections\":[],\"selectedId\":\"{LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1}\"}}"))
}

/// ➕️ Adds or updates one remote hub and selects it. The bootstrap entry is never replaced, and the
/// book is capped so a scripted loop cannot grow the profile without bound.
pub fn upsert_hub_connection(book: &HubConnectionBook, connection: HubConnection) -> HubConnectionBook {
    if connection.kind != HubConnectionKind::Remote {
        return HubConnectionBook { connections: book.connections.clone(), selected_id: LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1.to_string() };
    }
    let bootstrap = book.connections.first().cloned().unwrap_or_else(|| local_bootstrap_hub_connection(&connection.origin, Locale::En));
    let selected_id = connection.id.clone();
    let mut connections = vec![bootstrap, connection.clone()];
    for entry in book.connections.iter().filter(|entry| entry.kind == HubConnectionKind::Remote && entry.id != connection.id) {
        if connections.len() >= HUB_CONNECTION_BOOK_MAX_ENTRIES {
            break;
        }
        connections.push(entry.clone());
    }
    HubConnectionBook { connections, selected_id }
}

/// ➖️ Forgets one remote hub; removing the selected hub falls back to the local bootstrap.
pub fn remove_hub_connection(book: &HubConnectionBook, id: &str) -> HubConnectionBook {
    if id == LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1 {
        return book.clone();
    }
    let connections: Vec<HubConnection> = book.connections.iter().filter(|entry| entry.id != id).cloned().collect();
    let selected_id = if book.selected_id == id { LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1.to_string() } else { book.selected_id.clone() };
    HubConnectionBook { connections, selected_id }
}

/// 🎯️ Selects a known hub; an unknown id leaves the book untouched.
pub fn select_hub_connection(book: &HubConnectionBook, id: &str) -> HubConnectionBook {
    if book.connections.iter().any(|entry| entry.id == id) {
        return HubConnectionBook { connections: book.connections.clone(), selected_id: id.to_string() };
    }
    book.clone()
}

/// 🔎️ The currently selected connection — always present, because the bootstrap entry cannot be
/// removed and the book is never empty.
pub fn selected_hub_connection(book: &HubConnectionBook) -> &HubConnection {
    book.connections.iter().find(|entry| entry.id == book.selected_id).unwrap_or(&book.connections[0])
}
//#endregion 📕️Connections

//#region 🪪️SessionState
/// 🪪️ Where one hub session is in its life. `Offline` is deliberately NOT a phase: losing the
/// network while signed in must never sign the human out, because the app keeps working locally.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum HubSessionPhase {
    #[default]
    SignedOut,
    SigningIn,
    SignedIn,
    Expired,
    SigningOut,
}

impl HubSessionPhase {
    /// 🏷️ The wire spelling the React twin stamps, so one probe reads both renderers.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SignedOut => "signed-out",
            Self::SigningIn => "signing-in",
            Self::SignedIn => "signed-in",
            Self::Expired => "expired",
            Self::SigningOut => "signing-out",
        }
    }
}

/// 🪪️ The shell's whole hub-session state, minus the capability itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HubSessionState {
    pub phase: HubSessionPhase,
    pub connection_id: String,
    pub user_id: Option<String>,
    pub expires_at_ms: Option<i64>,
    pub error: Option<HubSignInErrorCode>,
    pub retry_after_seconds: Option<u64>,
    pub offline: bool,
}

/// 📨️ Every transition the session may take. CQRS-shaped: the surface raises an intent, the
/// transport emits a fact, and [`reduce_hub_session`] is the only place state changes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HubSessionEvent {
    SelectConnection { connection_id: String },
    Submit,
    Minted { user_id: String, expires_at_ms: Option<i64> },
    Failed { code: HubSignInErrorCode, retry_after_seconds: Option<u64> },
    Expired,
    SignOut,
    SignedOut,
    Connectivity { offline: bool },
}

/// 🪪️ A fresh session state for one hub.
pub fn hub_session_initial_state(connection_id: &str) -> HubSessionState {
    HubSessionState { phase: HubSessionPhase::SignedOut, connection_id: connection_id.to_string(), user_id: None, expires_at_ms: None, error: None, retry_after_seconds: None, offline: false }
}

/// 🧮️ Pure `state × event → state`. A `Connectivity` event never changes the phase; a `Failed` event
/// never clears a live session (a refresh failure while signed in is reported, not a sign-out) —
/// only `Expired` and `SignedOut` leave the signed-in phase. These three precedences are the whole
/// reason this is a reducer and not a set of setters.
pub fn reduce_hub_session(state: &HubSessionState, event: &HubSessionEvent) -> HubSessionState {
    match event {
        HubSessionEvent::SelectConnection { connection_id } => {
            if connection_id == &state.connection_id {
                return state.clone();
            }
            HubSessionState { offline: state.offline, ..hub_session_initial_state(connection_id) }
        }
        HubSessionEvent::Submit => HubSessionState { phase: HubSessionPhase::SigningIn, error: None, retry_after_seconds: None, ..state.clone() },
        HubSessionEvent::Minted { user_id, expires_at_ms } => HubSessionState { phase: HubSessionPhase::SignedIn, user_id: Some(user_id.clone()), expires_at_ms: *expires_at_ms, error: None, retry_after_seconds: None, ..state.clone() },
        HubSessionEvent::Failed { code, retry_after_seconds } => {
            if state.phase == HubSessionPhase::SignedIn {
                return HubSessionState { error: Some(*code), retry_after_seconds: *retry_after_seconds, ..state.clone() };
            }
            let phase = if state.phase == HubSessionPhase::Expired { HubSessionPhase::Expired } else { HubSessionPhase::SignedOut };
            HubSessionState { phase, error: Some(*code), retry_after_seconds: *retry_after_seconds, ..state.clone() }
        }
        HubSessionEvent::Expired => HubSessionState { phase: HubSessionPhase::Expired, expires_at_ms: None, error: None, retry_after_seconds: None, ..state.clone() },
        HubSessionEvent::SignOut => HubSessionState { phase: HubSessionPhase::SigningOut, error: None, retry_after_seconds: None, ..state.clone() },
        HubSessionEvent::SignedOut => HubSessionState { offline: state.offline, ..hub_session_initial_state(&state.connection_id) },
        HubSessionEvent::Connectivity { offline } => HubSessionState { offline: *offline, ..state.clone() },
    }
}

/// 🏠️ Whether the app may keep editing locally. Always true — the predicate exists so the shell and
/// the tests state the law once instead of re-deriving it: no hub phase ever disables local work.
pub fn hub_session_allows_local_work(_state: &HubSessionState) -> bool {
    true
}

/// ♻️ Whether the human must be asked for credentials again before hub work resumes.
pub fn hub_session_needs_reauthentication(state: &HubSessionState, now_ms: i64) -> bool {
    if state.phase == HubSessionPhase::Expired {
        return true;
    }
    if state.phase != HubSessionPhase::SignedIn {
        return false;
    }
    state.expires_at_ms.is_some_and(|deadline| deadline <= now_ms)
}

/// 🔐️ Whether this hub offers a password form at all. A hub that has refused credential sign-in
/// once has no password path, so the field is structurally ABSENT rather than offered and refused —
/// AU2's `hubSignInFormOfferedV1`.
pub fn hub_sign_in_form_offered(state: &HubSessionState) -> bool {
    state.error != Some(HubSignInErrorCode::PasswordSignInDisabled) && !matches!(state.phase, HubSessionPhase::SignedIn | HubSessionPhase::SigningOut)
}
//#endregion 🪪️SessionState

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🧪️tests/🔬️wgpu-unit/🦀️.rs"]
mod tests;
