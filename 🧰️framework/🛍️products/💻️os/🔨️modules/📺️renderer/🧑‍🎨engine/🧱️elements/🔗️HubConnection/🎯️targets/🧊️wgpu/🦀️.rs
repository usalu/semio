//! 🔗️ wgpu twin of the `🔗️HubConnection` headless lane and of the `HubWorkspace` surface React
//! mounts on `/hub` (`🔗️HubConnection/🟦️.tsx` + `🔗️HubConnection/🏛️workspace/🟦️.tsx`).
//!
//! 🧩️ Three things live here and nothing else: (1) the hub session transport, written ONCE over the
//! shell's own [`DirectoryTransport`] so native and browser share it verbatim; (2) the pure
//! [`hub_connection_summary`] fold — the Rust twin of `hubConnectionSummaryV1`
//! (`📓️u1-progress-cancel-and-connection-status.md` §8), which the footer pill reads; (3)
//! [`build_hub_workspace_ui`], the retained `UiNode` tree the shell publishes as one panel leaf.
//!
//! ⚖️ Why a `UiNode` tree and not a paint program like `🛂️SpaceAdministration`'s wgpu target: the
//! accessibility projection this renderer publishes (`🖱️ui/🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs`)
//! walks the retained `UiTree`, so a declarative surface is announced to an assistive technology for
//! free while a hand-rolled paint program is announced not at all. The hub surface is also a PANEL,
//! not an overlay sheet, so it rides `publish_shell_panel_document` — the same ingress every other
//! shell-owned leaf uses.
//!
//! 🔑️ The minted capability crosses this module only as a `&str` argument to one transport call. It
//! is never returned to the surface, never placed in a `UiNode`, and never written to the book.

use crate::hub_sign_in::{
    hub_auth_error_retry_after_seconds, hub_session_mint_request_json, hub_sign_in_error_from_status, hub_sign_in_error_text, hub_sign_in_label, parse_hub_session_mint_result, selected_hub_connection, HubConnectionBook,
    HubSessionMintResult, HubSessionPhase, HubSessionState, HubSignInCredential, HubSignInErrorCode, HubSignInLabel, HUB_SESSION_MINT_PATH_V1,
};
use crate::space_browser::{
    filter_space_rows, invite_link, space_browser_label, space_row_invitable, space_row_summary, InviteRedemptionErrorCode, SpaceBrowserLabel, SpaceBrowserPhase, SpaceMemberPresence, SpaceRow,
};
use semio_framework::IconName;
use semio_framework_async::OperationContext;
use semio_framework_os_kernel::os_directory::client::{DirectoryTransport, HttpMethod, HttpResponse, TransportError};
use semio_framework_os_kernel::os_directory::DirectorySpaceRole;
use semio_framework_os_kernel::DslValue;
use std::collections::HashMap;
use ui_wgpu::wgpu::component::ui::UiState;
use ui_wgpu::wgpu::{ActionDescriptor, Label, Locale, UiButtonNode, UiInputNode, UiNode, UiPresence, UiStackNode, UiTextNode};

//#region 🆔️SurfaceIds
/// 🆔️ The panel leaf id the shell registers and routes `/hub` to. Byte-identical to the React
/// route's own surface id so one probe addresses both renderers.
pub const FRAMEWORK_HUB_PANEL_ID: &str = "framework.hub";
pub const HUB_SIGN_IN_FORM_ID: &str = "framework.hub.sign-in";
pub const HUB_SPACES_LIST_ID: &str = "framework.hub.spaces";
pub const HUB_MEMBERS_LIST_ID: &str = "framework.hub.members";
pub const HUB_EMAIL_INPUT_ID: &str = "framework.hub.email";
pub const HUB_PASSWORD_INPUT_ID: &str = "framework.hub.password";
pub const HUB_ADDRESS_INPUT_ID: &str = "framework.hub.address";
pub const HUB_SEARCH_INPUT_ID: &str = "framework.hub.search";
pub const HUB_SPACE_NAME_INPUT_ID: &str = "framework.hub.space-name";
pub const HUB_INVITE_INPUT_ID: &str = "framework.hub.invite";

/// 🎬️ Every verb the hub surface dispatches on the `"framework"` controller. One closed list, so the
/// shell's dispatch arm and this builder cannot drift into two vocabularies.
pub mod action {
    pub const CLOSE_WORKSPACE: &str = "hubCloseWorkspace";
    pub const REFRESH_SPACES: &str = "hubRefreshSpaces";
    pub const SELECT_CONNECTION: &str = "hubSelectConnection";
    pub const ADD_CONNECTION: &str = "hubAddConnection";
    pub const FORGET_CONNECTION: &str = "hubForgetConnection";
    pub const SET_EMAIL: &str = "hubSetEmail";
    pub const SET_PASSWORD: &str = "hubSetPassword";
    pub const SET_ADDRESS: &str = "hubSetAddress";
    pub const SET_SEARCH: &str = "hubSetSearch";
    pub const SET_SPACE_NAME: &str = "hubSetSpaceName";
    pub const SET_INVITE_TEXT: &str = "hubSetInviteText";
    pub const SIGN_IN: &str = "hubSignIn";
    pub const CANCEL_SIGN_IN: &str = "hubCancelSignIn";
    pub const SIGN_OUT: &str = "hubSignOut";
    pub const OPEN_SPACE: &str = "hubOpenSpace";
    pub const CREATE_SPACE: &str = "hubCreateSpace";
    pub const CREATE_INVITE: &str = "hubCreateInvite";
    pub const COPY_INVITE_LINK: &str = "hubCopyInviteLink";
    pub const DISCARD_INVITE_LINK: &str = "hubDiscardInviteLink";
    pub const REDEEM_INVITE: &str = "hubRedeemInvite";
}
//#endregion 🆔️SurfaceIds

//#region 📶️ConnectionSummary
/// 📶️ One attached document's remote state, in this module's own vocabulary rather than
/// `store_sync`'s. The sync backbone is native-only in this crate, so a fold written against
/// `RemoteState` could not compile for the browser arm at all; the shell maps its own statuses onto
/// these four on the target that has them, and the fold stays one implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubDocumentRemote {
    Live { peer_count: usize },
    Connecting,
    Backoff,
    Detached,
}

/// 🪪️ Whether a hub session exists at all. `None` — no sign-in surface mounted — is deliberately not
/// the same as `SignedOut`: only the latter is actionable, and only it offers an entry point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubSessionPresence {
    SignedIn,
    SignedOut,
    None,
}

/// 📶️ The aggregate the footer pill paints.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubConnectionState {
    SignedOut,
    Live { peer_count: usize },
    Connecting,
    Reconnecting,
    Offline,
}

/// 📶️ The whole fold result: what to paint, and whether the pill is an actionable entry point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HubConnectionSummary {
    pub state: HubConnectionState,
    pub actionable: bool,
}

/// 📶️ Folds every attached document's remote state into one aggregate, best state first.
///
/// 🧮️ The three laws U1 §8 pinned, restated so a reader need not chase the TypeScript: one `Live`
/// document means the hub is reachable; a document still dialling outranks one already in backoff;
/// everything detached — or nothing attached at all — is `Offline`. `peer_count` is the **max** over
/// live documents, never a sum: a sum would double-count a peer who has two documents open, which no
/// reader could interpret. `SignedOut` outranks every transport state, because no transport state
/// means anything without a session.
pub fn hub_connection_summary(statuses: &[HubDocumentRemote], session: HubSessionPresence) -> HubConnectionSummary {
    if session == HubSessionPresence::SignedOut {
        return HubConnectionSummary { state: HubConnectionState::SignedOut, actionable: true };
    }
    let actionable = session == HubSessionPresence::SignedIn;
    let mut peer_count: Option<usize> = None;
    let mut connecting = false;
    let mut backoff = false;
    for status in statuses {
        match status {
            HubDocumentRemote::Live { peer_count: peers } => peer_count = Some(peer_count.unwrap_or(0).max(*peers)),
            HubDocumentRemote::Connecting => connecting = true,
            HubDocumentRemote::Backoff => backoff = true,
            HubDocumentRemote::Detached => continue,
        }
    }
    let state = match (peer_count, connecting, backoff) {
        (Some(peer_count), _, _) => HubConnectionState::Live { peer_count },
        (None, true, _) => HubConnectionState::Connecting,
        (None, false, true) => HubConnectionState::Reconnecting,
        (None, false, false) => HubConnectionState::Offline,
    };
    HubConnectionSummary { state, actionable }
}

impl HubConnectionState {
    /// 🏷️ The pill's icon. Icon PLUS text, never colour alone — the tone is decoration on top of a
    /// readable word, which is what makes the pill legible to a colour-blind reader.
    pub fn icon_id(self) -> &'static str {
        match self {
            Self::SignedOut => "user",
            Self::Live { .. } => "cloud",
            Self::Connecting => "loader-2",
            Self::Reconnecting => "rotate-ccw",
            Self::Offline => "link-2-off",
        }
    }

    /// 🏷️ The state's own wire spelling, so a probe reads the state without parsing a translation.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SignedOut => "signedOut",
            Self::Live { .. } => "live",
            Self::Connecting => "connecting",
            Self::Reconnecting => "reconnecting",
            Self::Offline => "offline",
        }
    }
}
//#endregion 📶️ConnectionSummary

//#region 🚚️Transport
/// 🎫️ One completed mint, or the closed class that ended it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HubSignInOutcome {
    Minted(HubSessionMintResult),
    Failed { code: HubSignInErrorCode, retry_after_seconds: Option<u64> },
}

async fn hub_http<T: DirectoryTransport>(transport: &T, ctx: &OperationContext, method: HttpMethod, url: String, bearer: Option<&str>, body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError> {
    transport.http(ctx, method, &url, bearer, body).await
}

/// 🔐️ Runs exactly one sign-in attempt against one hub origin and classifies every ending.
/// Cancellation is a first-class outcome rather than an exception, so the caller's reducer stays
/// total, and a thrown transport (DNS, TLS, timeout, offline) becomes `Unreachable` — the app stays
/// usable locally either way.
pub async fn run_hub_sign_in<T: DirectoryTransport>(transport: &T, ctx: &OperationContext, origin: &str, credential: &HubSignInCredential) -> HubSignInOutcome {
    let body = match hub_session_mint_request_json(credential) {
        Ok(body) => body,
        Err(code) => return HubSignInOutcome::Failed { code, retry_after_seconds: None },
    };
    let response = match hub_http(transport, ctx, HttpMethod::Post, format!("{origin}{HUB_SESSION_MINT_PATH_V1}"), None, Some(body.into_bytes())).await {
        Ok(response) => response,
        Err(TransportError::Cancelled) => return HubSignInOutcome::Failed { code: HubSignInErrorCode::Cancelled, retry_after_seconds: None },
        Err(_) => return HubSignInOutcome::Failed { code: HubSignInErrorCode::Unreachable, retry_after_seconds: None },
    };
    let text = String::from_utf8_lossy(&response.body).to_string();
    if response.status != 200 {
        let code = hub_sign_in_error_from_status(response.status);
        let retry_after_seconds = (code == HubSignInErrorCode::RateLimited).then(|| hub_auth_error_retry_after_seconds(&text)).flatten();
        return HubSignInOutcome::Failed { code, retry_after_seconds };
    }
    match parse_hub_session_mint_result(&text) {
        Some(result) => HubSignInOutcome::Minted(result),
        None => HubSignInOutcome::Failed { code: HubSignInErrorCode::InvalidResponse, retry_after_seconds: None },
    }
}

//#endregion 🚚️Transport

//#region 🏛️WorkspaceState
/// 🏛️ Everything the hub workspace paints, in one owned value the shell holds and this module reads.
/// Drafts are plain strings, so every keystroke is one `on_change` dispatch and no hidden editor
/// state exists between the surface and the reducer.
#[derive(Clone, Debug, PartialEq)]
pub struct HubWorkspaceState {
    pub book: HubConnectionBook,
    pub session: HubSessionState,
    pub display_name: Option<String>,
    pub email_draft: String,
    pub password_draft: String,
    pub address_draft: String,
    pub search_draft: String,
    pub space_name_draft: String,
    pub invite_draft: String,
    pub phase: SpaceBrowserPhase,
    pub rows: Vec<SpaceRow>,
    pub open_space_id: Option<String>,
    pub members: Vec<SpaceMemberPresence>,
    pub invite_capability: Option<String>,
    pub redemption_error: Option<InviteRedemptionErrorCode>,
    /// 📋️ Whether this renderer has a clipboard door at all. The wgpu frame Worker owns no
    /// `navigator.clipboard` and there is no shell-side runner for `ui_host::ClipboardIoJob`
    /// (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`'s own note), so the copy control is structurally ABSENT
    /// here rather than painted and inert — a hit target no click may act on is worse than none.
    /// The link itself is always rendered, so it can be read and selected either way.
    pub clipboard_available: bool,
}

impl HubWorkspaceState {
    /// 🆕️ A workspace that knows one hub and no session — the state a shell starts in.
    pub fn new(book: HubConnectionBook) -> Self {
        let session = crate::hub_sign_in::hub_session_initial_state(&book.selected_id);
        Self {
            book,
            session,
            display_name: None,
            email_draft: String::new(),
            password_draft: String::new(),
            address_draft: String::new(),
            search_draft: String::new(),
            space_name_draft: String::new(),
            invite_draft: String::new(),
            phase: SpaceBrowserPhase::Loading,
            rows: Vec::new(),
            open_space_id: None,
            members: Vec::new(),
            invite_capability: None,
            redemption_error: None,
            clipboard_available: false,
        }
    }

    /// 🌐️ The origin every call this workspace makes is addressed to — the selected hub's own, never
    /// the page's. AU3 §4.4 is the defect this avoids by construction on this renderer.
    pub fn origin(&self) -> &str {
        selected_hub_connection(&self.book).origin.as_str()
    }

    /// 🪪️ What the footer pill's session half reads.
    pub fn presence(&self) -> HubSessionPresence {
        match self.session.phase {
            HubSessionPhase::SignedIn => HubSessionPresence::SignedIn,
            _ => HubSessionPresence::SignedOut,
        }
    }
}
//#endregion 🏛️WorkspaceState

//#region 🌲️RetainedTree
fn descriptor(action: &str, args: Option<DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: "framework".into(), action: action.into(), args }
}

fn one_arg(key: &str, value: &str) -> Option<DslValue> {
    Some(DslValue::Object(vec![(key.to_string(), DslValue::String(value.into()))]))
}

fn text_row(value: &str) -> UiNode {
    UiNode::Text(UiTextNode { presence: UiPresence::default(), value: Label::data(value), emphasize: Some(false), data_attributes: None, menu: None })
}

fn tagged_row(value: &str, attributes: &[(&str, &str)]) -> UiNode {
    let data_attributes = attributes.iter().map(|(key, value)| ((*key).to_string(), (*value).to_string())).collect::<HashMap<String, String>>();
    UiNode::Text(UiTextNode { presence: UiPresence::default(), value: Label::data(value), emphasize: Some(true), data_attributes: Some(data_attributes), menu: None })
}

fn stack(id: &str, children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "column".into(),
        gap: None,
        padding: None,
        id: Some(id.into()),
        children,
        presence: UiPresence::default(),
        activate: None,
        drop_action: None,
        drop_overlay: None,
        menu: None,
    })
}

fn button(id: &str, icon_id: IconName, label: &str, action: ActionDescriptor, enabled: bool) -> UiNode {
    UiNode::Button(UiButtonNode {
        id: Some(id.into()),
        icon_id,
        label: Label::data(label),
        action,
        style: None,
        presence: UiPresence { state: if enabled { UiState::Normal } else { UiState::Disabled }, ..UiPresence::default() },
        menu: None,
    })
}

fn input(id: &str, value: &str, placeholder: &str, action: &str, submit: Option<&str>) -> UiNode {
    UiNode::Input(UiInputNode {
        id: id.into(),
        input_kind: if id == HUB_PASSWORD_INPUT_ID { "password".into() } else { "text".into() },
        value: value.to_string(),
        placeholder: Some(Label::data(placeholder)),
        commit: None,
        min: None,
        max: None,
        step: None,
        accept: None,
        on_change: descriptor(action, None),
        on_submit: submit.map(|action| descriptor(action, None)),
        on_abort: None,
        on_repeat_last: None,
        presence: UiPresence::default(),
        menu: None,
    })
}

/// 🔐️ The sign-in section: hub choice, the credential form, the live phase line, the denial (if
/// any), and the permanently visible local-only statement.
///
/// 🏠️ That last line is not decoration. `hub_session_allows_local_work` is `true` in every phase by
/// construction, and the surface says so in every phase too, so a human who cannot reach a hub is
/// never left guessing whether their work is going anywhere.
fn hub_sign_in_section(state: &HubWorkspaceState, locale: Locale) -> UiNode {
    let signed_in = state.session.phase == HubSessionPhase::SignedIn;
    let busy = matches!(state.session.phase, HubSessionPhase::SigningIn | HubSessionPhase::SigningOut);
    let mut children = vec![tagged_row(
        hub_sign_in_label(HubSignInLabel::Title, locale),
        &[("data-semio-hub-phase", state.session.phase.as_str()), ("data-semio-hub-connection", selected_hub_connection(&state.book).id.as_str())],
    )];
    for connection in &state.book.connections {
        let selected = connection.id == state.book.selected_id;
        // 🏠️ The bootstrap entry's label is resolved at PAINT time, not stored: the book is built
        // once, before the shell knows its locale, so a persisted label would pin whichever language
        // the process happened to start in.
        let label = match connection.kind {
            crate::hub_sign_in::HubConnectionKind::LocalBootstrap => hub_sign_in_label(HubSignInLabel::ThisDevice, locale).to_string(),
            crate::hub_sign_in::HubConnectionKind::Remote => connection.label.clone(),
        };
        children.push(button(
            &format!("{HUB_SIGN_IN_FORM_ID}.connection.{}", connection.id),
            if selected { IconName::Check } else { IconName::CircleDot },
            &label,
            descriptor(action::SELECT_CONNECTION, one_arg("connectionId", &connection.id)),
            !busy,
        ));
    }
    if selected_hub_connection(&state.book).kind == crate::hub_sign_in::HubConnectionKind::Remote {
        children.push(button(
            &format!("{HUB_SIGN_IN_FORM_ID}.forget"),
            IconName::Trash2,
            hub_sign_in_label(HubSignInLabel::ForgetHub, locale),
            descriptor(action::FORGET_CONNECTION, None),
            !busy,
        ));
    }
    children.push(input(HUB_ADDRESS_INPUT_ID, &state.address_draft, hub_sign_in_label(HubSignInLabel::HubAddress, locale), action::SET_ADDRESS, Some(action::ADD_CONNECTION)));
    children.push(button(
        &format!("{HUB_SIGN_IN_FORM_ID}.add"),
        IconName::Plus,
        hub_sign_in_label(HubSignInLabel::AddHub, locale),
        descriptor(action::ADD_CONNECTION, None),
        !state.address_draft.trim().is_empty(),
    ));
    if signed_in {
        let who = state.display_name.clone().or_else(|| state.session.user_id.clone()).unwrap_or_default();
        children.push(text_row(&format!("{} {who}", hub_sign_in_label(HubSignInLabel::SignedInAs, locale))));
        children.push(button(&format!("{HUB_SIGN_IN_FORM_ID}.sign-out"), IconName::X, hub_sign_in_label(HubSignInLabel::SignOut, locale), descriptor(action::SIGN_OUT, None), true));
    } else if crate::hub_sign_in::hub_sign_in_form_offered(&state.session) {
        children.push(input(HUB_EMAIL_INPUT_ID, &state.email_draft, hub_sign_in_label(HubSignInLabel::Email, locale), action::SET_EMAIL, None));
        children.push(input(HUB_PASSWORD_INPUT_ID, &state.password_draft, hub_sign_in_label(HubSignInLabel::Password, locale), action::SET_PASSWORD, Some(action::SIGN_IN)));
        let ready = crate::hub_sign_in::valid_hub_sign_in_email(state.email_draft.trim()) && crate::hub_sign_in::valid_hub_sign_in_password(&state.password_draft);
        children.push(button(&format!("{HUB_SIGN_IN_FORM_ID}.submit"), IconName::User, hub_sign_in_label(HubSignInLabel::Submit, locale), descriptor(action::SIGN_IN, None), ready && !busy));
    }
    if busy {
        let phrase = if state.session.phase == HubSessionPhase::SigningIn { HubSignInLabel::SigningIn } else { HubSignInLabel::SigningOut };
        children.push(text_row(hub_sign_in_label(phrase, locale)));
        children.push(button(&format!("{HUB_SIGN_IN_FORM_ID}.cancel"), IconName::X, hub_sign_in_label(HubSignInLabel::Cancel, locale), descriptor(action::CANCEL_SIGN_IN, None), true));
    }
    if let Some(code) = state.session.error {
        children.push(tagged_row(&hub_sign_in_error_text(locale, code, state.session.retry_after_seconds), &[("data-semio-hub-error", code.as_str())]));
    }
    if state.session.phase == HubSessionPhase::Expired {
        children.push(text_row(crate::hub_sign_in::hub_sign_in_text(locale).expired));
    }
    children.push(tagged_row(hub_sign_in_label(HubSignInLabel::LocalOnly, locale), &[("data-semio-hub-local-only", "true")]));
    stack(HUB_SIGN_IN_FORM_ID, children)
}

/// 🏘️ The spaces section: the live phase line, the search field, one row per space with its open,
/// invite and archive affordances, then create and redeem.
///
/// 🏠️ Rows stay rendered and openable in the `Stale` phase — the local-first rule AU2 defended: a
/// hub that stopped answering must not empty a list the human was reading.
fn hub_spaces_section(state: &HubWorkspaceState, locale: Locale) -> UiNode {
    let rows = filter_space_rows(&state.rows, &state.search_draft);
    let usable = crate::space_browser::space_browser_rows_usable(state.phase, rows.len());
    let mut children = vec![
        tagged_row(space_browser_label(SpaceBrowserLabel::Title, locale), &[("data-semio-hub-spaces-phase", state.phase.as_str())]),
        text_row(state.phase.text(locale)),
        button(
            &format!("{HUB_SPACES_LIST_ID}.refresh"),
            IconName::RotateCw,
            match locale { Locale::En => "Refresh", Locale::De => "Aktualisieren" },
            descriptor(action::REFRESH_SPACES, None),
            !matches!(state.phase, SpaceBrowserPhase::Loading | SpaceBrowserPhase::Submitting),
        ),
        input(HUB_SEARCH_INPUT_ID, &state.search_draft, space_browser_label(SpaceBrowserLabel::Search, locale), action::SET_SEARCH, None),
    ];
    if rows.is_empty() {
        children.push(text_row(space_browser_label(SpaceBrowserLabel::Empty, locale)));
    }
    for row in &rows {
        let current = state.open_space_id.as_deref() == Some(row.id.as_str());
        children.push(tagged_row(&row.name, &[("data-semio-hub-space", row.id.as_str()), ("data-semio-hub-space-access", row.access.as_str()), ("data-semio-hub-space-current", if current { "true" } else { "false" })]));
        children.push(text_row(&space_row_summary(row, locale)));
        children.push(button(&format!("{HUB_SPACES_LIST_ID}.open.{}", row.id), IconName::ArrowRight, space_browser_label(SpaceBrowserLabel::Open, locale), descriptor(action::OPEN_SPACE, one_arg("spaceId", &row.id)), usable));
        if space_row_invitable(row) {
            children.push(button(
                &format!("{HUB_SPACES_LIST_ID}.invite.{}", row.id),
                IconName::Users,
                space_browser_label(SpaceBrowserLabel::Invite, locale),
                descriptor(action::CREATE_INVITE, one_arg("spaceId", &row.id)),
                state.phase != SpaceBrowserPhase::Submitting,
            ));
        }
    }
    children.push(input(HUB_SPACE_NAME_INPUT_ID, &state.space_name_draft, space_browser_label(SpaceBrowserLabel::CreateName, locale), action::SET_SPACE_NAME, Some(action::CREATE_SPACE)));
    children.push(button(
        &format!("{HUB_SPACES_LIST_ID}.create"),
        IconName::Plus,
        space_browser_label(SpaceBrowserLabel::Create, locale),
        descriptor(action::CREATE_SPACE, None),
        !state.space_name_draft.trim().is_empty() && state.session.phase == HubSessionPhase::SignedIn,
    ));
    if let Some(capability) = state.invite_capability.as_deref() {
        let link = invite_link(state.origin(), capability).unwrap_or_default();
        children.push(tagged_row(space_browser_label(SpaceBrowserLabel::InviteLink, locale), &[("data-semio-hub-invite", "one-shot")]));
        children.push(text_row(&link));
        if state.clipboard_available {
            children.push(button(&format!("{HUB_SPACES_LIST_ID}.copy-invite"), IconName::Copy, space_browser_label(SpaceBrowserLabel::CopyLink, locale), descriptor(action::COPY_INVITE_LINK, None), true));
        }
        children.push(button(&format!("{HUB_SPACES_LIST_ID}.discard-invite"), IconName::Trash2, space_browser_label(SpaceBrowserLabel::DiscardLink, locale), descriptor(action::DISCARD_INVITE_LINK, None), true));
    }
    children.push(input(HUB_INVITE_INPUT_ID, &state.invite_draft, space_browser_label(SpaceBrowserLabel::RedeemPlaceholder, locale), action::SET_INVITE_TEXT, Some(action::REDEEM_INVITE)));
    children.push(button(
        &format!("{HUB_SPACES_LIST_ID}.redeem"),
        IconName::Link,
        space_browser_label(SpaceBrowserLabel::Redeem, locale),
        descriptor(action::REDEEM_INVITE, None),
        crate::space_browser::parse_invite_token(&state.invite_draft).is_some(),
    ));
    if let Some(code) = state.redemption_error {
        children.push(tagged_row(code.text(locale), &[("data-semio-hub-redemption-error", code.as_str())]));
    }
    stack(HUB_SPACES_LIST_ID, children)
}

/// 👥️ The roster of the open space, joined with live presence. A member the directory does not know
/// can never appear here: the rows come from the hub's own `members` window, and presence only
/// decorates them.
fn hub_members_section(state: &HubWorkspaceState, locale: Locale) -> UiNode {
    let member_count = state.members.len().to_string();
    let mut children = vec![tagged_row(space_browser_label(SpaceBrowserLabel::Members, locale), &[("data-semio-hub-member-count", member_count.as_str())])];
    for member in &state.members {
        let role = if member.owner {
            SpaceBrowserLabel::Owner
        } else if member.role == DirectorySpaceRole::Author {
            SpaceBrowserLabel::RoleAuthor
        } else {
            SpaceBrowserLabel::RoleSpectator
        };
        let presence = if member.online { SpaceBrowserLabel::Online } else { SpaceBrowserLabel::Offline };
        children.push(tagged_row(
            &format!("{} · {} · {}", member.display_name, space_browser_label(role, locale), space_browser_label(presence, locale)),
            &[("data-semio-hub-member", member.user_id.as_str()), ("data-semio-hub-member-online", if member.online { "true" } else { "false" })],
        ));
    }
    stack(HUB_MEMBERS_LIST_ID, children)
}

/// 🏛️ The whole hub workspace as one retained document — the wgpu twin of React's `HubWorkspace`
/// composition. Pure: it reads the state and names verbs, and owns no state of its own, so the same
/// tree can be asserted in a unit test with no shell, no hub and no GPU.
pub fn build_hub_workspace_ui(state: &HubWorkspaceState, locale: Locale) -> UiNode {
    let mut children = vec![
        button(
            "framework.hub.close",
            IconName::X,
            hub_sign_in_label(HubSignInLabel::Cancel, locale),
            descriptor(action::CLOSE_WORKSPACE, None),
            true,
        ),
        hub_sign_in_section(state, locale),
    ];
    if state.session.phase == HubSessionPhase::SignedIn || !state.rows.is_empty() {
        children.push(hub_spaces_section(state, locale));
    }
    if state.open_space_id.is_some() {
        children.push(hub_members_section(state, locale));
    }
    stack(FRAMEWORK_HUB_PANEL_ID, children)
}
//#endregion 🌲️RetainedTree

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🧪️tests/🔬️wgpu-unit/🦀️.rs"]
mod tests;
