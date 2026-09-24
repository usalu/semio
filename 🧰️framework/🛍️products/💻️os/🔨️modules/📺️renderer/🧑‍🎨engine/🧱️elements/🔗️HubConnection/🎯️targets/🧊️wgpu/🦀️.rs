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
    hub_auth_error_retry_after_seconds, hub_session_mint_request_json, hub_sign_in_error_from_status, hub_sign_in_error_text, hub_sign_in_label, parse_hub_session_mint_result, selected_hub_connection, HubConnectionBook, HubSessionMintResult,
    HubSessionPhase, HubSessionState, HubSignInCredential, HubSignInErrorCode, HubSignInLabel, HUB_SESSION_MINT_PATH_V1,
};
use crate::space_browser::{filter_space_rows, invite_link, space_browser_label, space_row_invitable, space_row_summary, InviteRedemptionErrorCode, SpaceBrowserLabel, SpaceBrowserPhase, SpaceMemberPresence, SpaceRow};
use semio_framework::IconName;
use semio_framework_async::OperationContext;
use semio_framework_os_kernel::os_directory::client::{DirectoryTransport, HttpMethod, HttpResponse, TransportError};
use semio_framework_os_kernel::os_directory::schema::space_artifact_creation::{SpaceArtifactCreateV1, SpaceArtifactCreationCatalogV1, SpaceArtifactCreationPhaseV1, SpaceArtifactCreationReadyV1};
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
pub const HUB_ARTIFACT_CREATION_ID: &str = "framework.hub.artifact-creation";
pub const HUB_ARTIFACT_NAME_INPUT_ID: &str = "framework.hub.artifact-name";

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
    pub const SELECT_ARTIFACT_KIND: &str = "hubSelectArtifactKind";
    pub const SET_ARTIFACT_NAME: &str = "hubSetArtifactName";
    pub const CREATE_ARTIFACT: &str = "hubCreateArtifact";
    pub const CANCEL_ARTIFACT_CREATION: &str = "hubCancelArtifactCreation";
    pub const OPEN_CREATED_ARTIFACT: &str = "hubOpenCreatedArtifact";
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
    /// 🌱️ The open space's artifact-creation door (catalog, choice, name, the creation in flight).
    pub creation: HubArtifactCreationState,
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
            creation: HubArtifactCreationState::default(),
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

//#region 🌱️ArtifactCreation
/// 📚️ Where the open space's selected current creation catalog is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HubArtifactCatalogPhase {
    #[default]
    Idle,
    Loading,
    Ready,
    Unavailable,
}

impl HubArtifactCatalogPhase {
    /// 🏷️ The phase's own wire spelling, so a probe reads it without parsing a translation.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Loading => "loading",
            Self::Ready => "ready",
            Self::Unavailable => "unavailable",
        }
    }
}

/// 🚪️ What happened to a ready artifact's opening.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HubArtifactOpening {
    #[default]
    Idle,
    Opening,
    Opened,
    Failed,
}

/// 🌱️ The one creation this workspace drives: the sealed intent, the hub's latest receipt, whether a
/// cancel was asked and sent, and the opening of its result. `deadline_at_ms` bounds the whole
/// operation; past it the outcome is `Indeterminate`, never silently `Failed`.
#[derive(Clone, Debug, PartialEq)]
pub struct HubArtifactCreation {
    pub intent: SpaceArtifactCreateV1,
    pub space_id: String,
    pub phase: SpaceArtifactCreationPhaseV1,
    pub submitted: bool,
    pub cancel_requested: bool,
    pub cancel_sent: bool,
    pub ready: Option<SpaceArtifactCreationReadyV1>,
    pub opening: HubArtifactOpening,
    pub deadline_at_ms: u64,
    pub next_poll_at_ms: u64,
}

/// 🌱️ The open space's artifact-creation door: the hub's selected current catalog, the chosen kind,
/// the name draft, the idempotency key the next Create will carry (minted before the click, so a
/// retried submission can never become a second creation) and at most one creation in flight — the
/// wgpu twin of React's Space-index create dialog plus `🏛️ShellHost/🌱️artifact-creation`'s progress
/// surface.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HubArtifactCreationState {
    pub catalog_phase: HubArtifactCatalogPhase,
    pub catalog: Option<SpaceArtifactCreationCatalogV1>,
    pub kind_id: Option<String>,
    pub name_draft: String,
    pub next_request_id: String,
    pub operation: Option<HubArtifactCreation>,
}

/// ⏱️ The whole-operation bound and the poll cadence — the browser worker's
/// `SPACE_ARTIFACT_CREATION_DEADLINE_MS` / `SPACE_ARTIFACT_CREATION_POLL_MS` (`🏪️store/👷️worker/🟦️.ts`).
pub const HUB_ARTIFACT_CREATION_DEADLINE_MS: u64 = 120_000;
pub const HUB_ARTIFACT_CREATION_POLL_MS: u64 = 100;

/// 🏁️ A phase after which the hub will not change the receipt again.
pub fn hub_artifact_creation_terminal(phase: SpaceArtifactCreationPhaseV1) -> bool {
    matches!(phase, SpaceArtifactCreationPhaseV1::Ready | SpaceArtifactCreationPhaseV1::Indeterminate | SpaceArtifactCreationPhaseV1::Failed | SpaceArtifactCreationPhaseV1::Cancelled)
}

/// 🏷️ A phase's wire spelling.
pub fn hub_artifact_creation_phase_str(phase: SpaceArtifactCreationPhaseV1) -> &'static str {
    match phase {
        SpaceArtifactCreationPhaseV1::Accepted => "accepted",
        SpaceArtifactCreationPhaseV1::Preparing => "preparing",
        SpaceArtifactCreationPhaseV1::Ready => "ready",
        SpaceArtifactCreationPhaseV1::Indeterminate => "indeterminate",
        SpaceArtifactCreationPhaseV1::Failed => "failed",
        SpaceArtifactCreationPhaseV1::Cancelled => "cancelled",
    }
}

/// 📚️ The catalog phase the door shows: a `Ready` answer that carries no valid choice is
/// `Unavailable`, exactly as React's `selectedSpaceArtifactCreationCatalogV1` projects it.
pub fn hub_artifact_catalog_effective_phase(creation: &HubArtifactCreationState) -> HubArtifactCatalogPhase {
    match creation.catalog_phase {
        HubArtifactCatalogPhase::Ready if !creation.catalog.as_ref().is_some_and(SpaceArtifactCreationCatalogV1::validate) => HubArtifactCatalogPhase::Unavailable,
        phase => phase,
    }
}

/// 📢️ How urgently an assistive technology announces the creation line: an `alert` for every ending
/// that needs the human (a failed opening, an unknown, failed or cancelled creation), a `status`
/// otherwise — the twin of React's `artifactCreationProgressRoleV1`.
pub fn hub_artifact_creation_role(phase: SpaceArtifactCreationPhaseV1, opening: HubArtifactOpening) -> &'static str {
    if opening == HubArtifactOpening::Failed || matches!(phase, SpaceArtifactCreationPhaseV1::Indeterminate | SpaceArtifactCreationPhaseV1::Failed | SpaceArtifactCreationPhaseV1::Cancelled) {
        "alert"
    } else {
        "status"
    }
}

/// 📥️ The sealed intent a Create click issues under the door's minted `next_request_id`, or `None`
/// while the door may not create: the catalog must be ready, the chosen kind one of its rows, the name
/// valid, and no creation still in flight.
pub fn hub_artifact_creation_intent(state: &HubWorkspaceState) -> Option<SpaceArtifactCreateV1> {
    let creation = &state.creation;
    if state.session.phase != HubSessionPhase::SignedIn || hub_artifact_catalog_effective_phase(creation) != HubArtifactCatalogPhase::Ready || creation.operation.as_ref().is_some_and(|operation| !hub_artifact_creation_terminal(operation.phase)) {
        return None;
    }
    let catalog = creation.catalog.as_ref()?;
    let kind = catalog.kinds.iter().find(|kind| Some(kind.kind_id.as_str()) == creation.kind_id.as_deref())?;
    let intent = SpaceArtifactCreateV1 {
        schema: "semio.hub.space-artifact-create/v1".into(),
        request_id: creation.next_request_id.clone(),
        expected_catalog_generation_id: catalog.catalog_generation_id.clone(),
        kind_id: kind.kind_id.clone(),
        name: creation.name_draft.trim_matches(' ').to_string(),
    };
    intent.validate().then_some(intent)
}

/// 🏷️ The closed set of texts the creation door names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubArtifactCreationLabel {
    Title,
    Name,
    Create,
    Heading,
    Cancel,
    Cancelling,
    Opening,
    OpeningFailed,
    OpenRetry,
    CatalogLoading,
    CatalogReady,
    CatalogUnavailable,
}

/// 🏷️ One door text in both languages this product owns, byte-identical to React's
/// `ARTIFACT_CREATION_PROGRESS_TEXT_V1` (`🏛️ShellHost/🌱️artifact-creation/🟦️.tsx`) where it has one.
pub fn hub_artifact_creation_label(key: HubArtifactCreationLabel, locale: Locale) -> &'static str {
    match (key, locale) {
        (HubArtifactCreationLabel::Title, Locale::En) => "Create an artifact",
        (HubArtifactCreationLabel::Title, Locale::De) => "Artefakt erstellen",
        (HubArtifactCreationLabel::Name, Locale::En) => "Artifact name",
        (HubArtifactCreationLabel::Name, Locale::De) => "Artefaktname",
        (HubArtifactCreationLabel::Create, Locale::En) => "Create artifact",
        (HubArtifactCreationLabel::Create, Locale::De) => "Artefakt erstellen",
        (HubArtifactCreationLabel::Heading, Locale::En) => "Creating artifact",
        (HubArtifactCreationLabel::Heading, Locale::De) => "Artefakt wird erstellt",
        (HubArtifactCreationLabel::Cancel, Locale::En) => "Cancel creation",
        (HubArtifactCreationLabel::Cancel, Locale::De) => "Erstellung abbrechen",
        (HubArtifactCreationLabel::Cancelling, Locale::En) => "Cancellation requested…",
        (HubArtifactCreationLabel::Cancelling, Locale::De) => "Abbruch angefordert…",
        (HubArtifactCreationLabel::Opening, Locale::En) => "Opening the artifact…",
        (HubArtifactCreationLabel::Opening, Locale::De) => "Artefakt wird geöffnet…",
        (HubArtifactCreationLabel::OpeningFailed, Locale::En) => "The artifact was created, but it could not be opened. You can safely try opening it again.",
        (HubArtifactCreationLabel::OpeningFailed, Locale::De) => "Das Artefakt wurde erstellt, konnte aber nicht geöffnet werden. Du kannst das Öffnen sicher erneut versuchen.",
        (HubArtifactCreationLabel::OpenRetry, Locale::En) => "Open artifact",
        (HubArtifactCreationLabel::OpenRetry, Locale::De) => "Artefakt öffnen",
        (HubArtifactCreationLabel::CatalogLoading, Locale::En) => "Loading the available artifact kinds…",
        (HubArtifactCreationLabel::CatalogLoading, Locale::De) => "Verfügbare Artefaktarten werden geladen…",
        (HubArtifactCreationLabel::CatalogReady, Locale::En) => "The available artifact kinds are current.",
        (HubArtifactCreationLabel::CatalogReady, Locale::De) => "Die verfügbaren Artefaktarten sind aktuell.",
        (HubArtifactCreationLabel::CatalogUnavailable, Locale::En) => "Artifact kinds are unavailable. Reopen the space before creating an artifact.",
        (HubArtifactCreationLabel::CatalogUnavailable, Locale::De) => "Artefaktarten sind nicht verfügbar. Öffne den Space erneut, bevor du ein Artefakt erstellst.",
    }
}

/// 🚦️ One creation phase's live text in both languages, byte-identical to React's.
pub fn hub_artifact_creation_phase_text(phase: SpaceArtifactCreationPhaseV1, locale: Locale) -> &'static str {
    match (phase, locale) {
        (SpaceArtifactCreationPhaseV1::Accepted, Locale::En) => "The creation request was accepted.",
        (SpaceArtifactCreationPhaseV1::Accepted, Locale::De) => "Die Erstellungsanfrage wurde angenommen.",
        (SpaceArtifactCreationPhaseV1::Preparing, Locale::En) => "The artifact is being created…",
        (SpaceArtifactCreationPhaseV1::Preparing, Locale::De) => "Das Artefakt wird erstellt…",
        (SpaceArtifactCreationPhaseV1::Ready, Locale::En) => "The artifact is ready.",
        (SpaceArtifactCreationPhaseV1::Ready, Locale::De) => "Das Artefakt ist bereit.",
        (SpaceArtifactCreationPhaseV1::Indeterminate, Locale::En) => "The creation outcome is unknown. Refresh the space before trying again.",
        (SpaceArtifactCreationPhaseV1::Indeterminate, Locale::De) => "Das Ergebnis der Erstellung ist unbekannt. Aktualisiere den Space vor einem neuen Versuch.",
        (SpaceArtifactCreationPhaseV1::Failed, Locale::En) => "Artifact creation failed.",
        (SpaceArtifactCreationPhaseV1::Failed, Locale::De) => "Die Artefakterstellung ist fehlgeschlagen.",
        (SpaceArtifactCreationPhaseV1::Cancelled, Locale::En) => "Artifact creation was cancelled.",
        (SpaceArtifactCreationPhaseV1::Cancelled, Locale::De) => "Die Artefakterstellung wurde abgebrochen.",
    }
}
//#endregion 🌱️ArtifactCreation

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
    UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: Some(id.into()), children, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None })
}

fn button(id: &str, icon_id: IconName, label: &str, action: ActionDescriptor, enabled: bool) -> UiNode {
    UiNode::Button(UiButtonNode { id: Some(id.into()), icon_id, label: Label::data(label), action, style: None, presence: UiPresence { state: if enabled { UiState::Normal } else { UiState::Disabled }, ..UiPresence::default() }, menu: None })
}

fn input(id: &str, value: &str, placeholder: &str, action: &str, submit: Option<&str>) -> UiNode {
    UiNode::Input(UiInputNode {
        id: id.into(),
        input_kind: if id == HUB_PASSWORD_INPUT_ID { "password".into() } else { "text".into() },
        value: value.to_string(),
        placeholder: Some(Label::data(placeholder)),
        accessibility_label: None,
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
///
/// 🗣️ The bootstrap entry's label is resolved at paint time, never stored: the book is built once,
/// before the shell knows its locale, so a persisted label would pin whichever language the process
/// happened to start in.
fn hub_sign_in_section(state: &HubWorkspaceState, locale: Locale) -> UiNode {
    let signed_in = state.session.phase == HubSessionPhase::SignedIn;
    let busy = matches!(state.session.phase, HubSessionPhase::SigningIn | HubSessionPhase::SigningOut);
    let mut children = vec![tagged_row(hub_sign_in_label(HubSignInLabel::Title, locale), &[("data-semio-hub-phase", state.session.phase.as_str()), ("data-semio-hub-connection", selected_hub_connection(&state.book).id.as_str())])];
    for connection in &state.book.connections {
        let selected = connection.id == state.book.selected_id;
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
        children.push(button(&format!("{HUB_SIGN_IN_FORM_ID}.forget"), IconName::Trash2, hub_sign_in_label(HubSignInLabel::ForgetHub, locale), descriptor(action::FORGET_CONNECTION, None), !busy));
    }
    children.push(input(HUB_ADDRESS_INPUT_ID, &state.address_draft, hub_sign_in_label(HubSignInLabel::HubAddress, locale), action::SET_ADDRESS, Some(action::ADD_CONNECTION)));
    children.push(button(&format!("{HUB_SIGN_IN_FORM_ID}.add"), IconName::Plus, hub_sign_in_label(HubSignInLabel::AddHub, locale), descriptor(action::ADD_CONNECTION, None), !state.address_draft.trim().is_empty()));
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
            space_browser_label(SpaceBrowserLabel::Refresh, locale),
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

/// 🌱️ The open space's creation door: the catalog line, one choice per creatable kind (its own en +
/// de label from the hub), the name, Create, then the one creation's live phase with its Cancel
/// while it runs and its opening once it is ready.
fn hub_artifact_creation_section(state: &HubWorkspaceState, locale: Locale) -> UiNode {
    let creation = &state.creation;
    let busy = creation.operation.as_ref().is_some_and(|operation| !hub_artifact_creation_terminal(operation.phase));
    let catalog_phase = hub_artifact_catalog_effective_phase(creation);
    let mut children = vec![tagged_row(hub_artifact_creation_label(HubArtifactCreationLabel::Title, locale), &[("data-semio-hub-artifact-catalog", catalog_phase.as_str())])];
    let catalog_line = match catalog_phase {
        HubArtifactCatalogPhase::Idle => None,
        HubArtifactCatalogPhase::Loading => Some((HubArtifactCreationLabel::CatalogLoading, "status")),
        HubArtifactCatalogPhase::Ready => Some((HubArtifactCreationLabel::CatalogReady, "status")),
        HubArtifactCatalogPhase::Unavailable => Some((HubArtifactCreationLabel::CatalogUnavailable, "alert")),
    };
    if let Some((label, role)) = catalog_line {
        children.push(tagged_row(hub_artifact_creation_label(label, locale), &[("data-semio-hub-artifact-catalog-role", role)]));
    }
    for kind in creation.catalog.iter().filter(|_| catalog_phase == HubArtifactCatalogPhase::Ready).flat_map(|catalog| catalog.kinds.iter()) {
        let selected = creation.kind_id.as_deref() == Some(kind.kind_id.as_str());
        let label = match locale {
            Locale::En => kind.label.en.as_str(),
            Locale::De => kind.label.de.as_str(),
        };
        children.push(button(&format!("{HUB_ARTIFACT_CREATION_ID}.kind.{}", kind.kind_id), if selected { IconName::Check } else { IconName::CircleDot }, label, descriptor(action::SELECT_ARTIFACT_KIND, one_arg("kindId", &kind.kind_id)), !busy));
    }
    children.push(input(HUB_ARTIFACT_NAME_INPUT_ID, &creation.name_draft, hub_artifact_creation_label(HubArtifactCreationLabel::Name, locale), action::SET_ARTIFACT_NAME, Some(action::CREATE_ARTIFACT)));
    children.push(button(
        &format!("{HUB_ARTIFACT_CREATION_ID}.create"),
        IconName::Plus,
        hub_artifact_creation_label(HubArtifactCreationLabel::Create, locale),
        descriptor(action::CREATE_ARTIFACT, None),
        hub_artifact_creation_intent(state).is_some(),
    ));
    if let Some(operation) = &creation.operation {
        let phase = hub_artifact_creation_phase_str(operation.phase);
        children.push(tagged_row(hub_artifact_creation_label(HubArtifactCreationLabel::Heading, locale), &[("data-semio-hub-artifact-creation", operation.intent.request_id.as_str()), ("data-semio-hub-artifact-creation-phase", phase)]));
        children.push(tagged_row(hub_artifact_creation_phase_text(operation.phase, locale), &[("data-semio-hub-artifact-creation-role", hub_artifact_creation_role(operation.phase, operation.opening))]));
        if !hub_artifact_creation_terminal(operation.phase) {
            if operation.cancel_requested {
                children.push(text_row(hub_artifact_creation_label(HubArtifactCreationLabel::Cancelling, locale)));
            } else {
                children.push(button(&format!("{HUB_ARTIFACT_CREATION_ID}.cancel"), IconName::X, hub_artifact_creation_label(HubArtifactCreationLabel::Cancel, locale), descriptor(action::CANCEL_ARTIFACT_CREATION, None), true));
            }
        }
        match (operation.ready.as_ref(), operation.opening) {
            (Some(_), HubArtifactOpening::Opening) => children.push(text_row(hub_artifact_creation_label(HubArtifactCreationLabel::Opening, locale))),
            (Some(ready), HubArtifactOpening::Failed) => {
                children.push(text_row(hub_artifact_creation_label(HubArtifactCreationLabel::OpeningFailed, locale)));
                children.push(button(&format!("{HUB_ARTIFACT_CREATION_ID}.open"), IconName::ArrowRight, hub_artifact_creation_label(HubArtifactCreationLabel::OpenRetry, locale), descriptor(action::OPEN_CREATED_ARTIFACT, None), true));
                children.push(tagged_row(&ready.artifact_id, &[("data-semio-hub-artifact-created", ready.artifact_id.as_str())]));
            }
            (Some(ready), _) => children.push(tagged_row(&ready.artifact_id, &[("data-semio-hub-artifact-created", ready.artifact_id.as_str())])),
            (None, _) => {}
        }
    }
    stack(HUB_ARTIFACT_CREATION_ID, children)
}

/// 🏛️ The whole hub workspace as one retained document — the wgpu twin of React's `HubWorkspace`
/// composition. Pure: it reads the state and names verbs, and owns no state of its own, so the same
/// tree can be asserted in a unit test with no shell, no hub and no GPU.
pub fn build_hub_workspace_ui(state: &HubWorkspaceState, locale: Locale) -> UiNode {
    let mut children = vec![button("framework.hub.close", IconName::X, hub_sign_in_label(HubSignInLabel::Cancel, locale), descriptor(action::CLOSE_WORKSPACE, None), true), hub_sign_in_section(state, locale)];
    if state.session.phase == HubSessionPhase::SignedIn || !state.rows.is_empty() {
        children.push(hub_spaces_section(state, locale));
    }
    if state.open_space_id.is_some() {
        children.push(hub_members_section(state, locale));
        children.push(hub_artifact_creation_section(state, locale));
    }
    stack(FRAMEWORK_HUB_PANEL_ID, children)
}
//#endregion 🌲️RetainedTree

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🧪️tests/🔬️wgpu-unit/🦀️.rs"]
mod tests;
