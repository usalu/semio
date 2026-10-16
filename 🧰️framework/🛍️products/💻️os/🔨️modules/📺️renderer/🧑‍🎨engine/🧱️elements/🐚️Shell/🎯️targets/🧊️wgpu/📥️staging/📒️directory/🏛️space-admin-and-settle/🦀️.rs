//! 📒️ `shell-directory` staging of `space-admin-and-settle`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
//#region 🏛️SpaceAdministration
/// 🏛️ Closed lifecycle of the one shell-owned native space-administration operation — the WGPU twin
/// of the browser worker's `DirectoryAdministrationOperation`. Every terminal phase has already
/// erased the page, the receipt, and any invite capability.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellSpaceAdministrationPhaseV1 {
    Loading,
    Ready,
    Submitting,
    Receipt,
    Refreshing,
    Cancelled,
    Denied,
    Stale,
    Failed,
}

impl ShellSpaceAdministrationPhaseV1 {
    /// 🏁️ A terminal phase never advances again and holds no page, receipt, or capability.
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Cancelled | Self::Denied | Self::Stale | Self::Failed)
    }

    /// 🚦️ Dispatch is admitted only from a settled `Ready` page — never while a receipt is pending.
    pub fn is_dispatchable(self) -> bool {
        matches!(self, Self::Ready)
    }
}

/// 🎬️ The one bounded step the driver may take next. `Idle` means the operation is waiting on the
/// renderer; `Closed` means it has been retired and must be dropped.
#[derive(Clone, Debug, PartialEq)]
pub enum ShellSpaceAdministrationTurnV1 {
    FetchPage { cursor: Option<String> },
    Submit { request: DirectoryCommandRequestV1 },
    Idle,
    Closed,
}

/// 🏛️ The one retained native administration operation (fixed capacity: exactly one per shell).
/// It owns one canonical page, at most one command request/receipt, and at most one one-shot invite
/// capability. Administration mutations never auto-retry: an indeterminate transport is terminal
/// `Failed` ("unknown outcome / refresh required"), so a `create-invite` can never mint twice.
#[derive(Clone, Debug)]
pub struct ShellSpaceAdministrationOperationV1 {
    pub(crate) operation_epoch: u64,
    pub(crate) space_id: String,
    pub(crate) phase: ShellSpaceAdministrationPhaseV1,
    pub(crate) page: Option<DirectorySpaceAdministrationPageV1>,
    pub(crate) canonical_json: Option<String>,
    pub(crate) receipt_sha256: Option<String>,
    pub(crate) invite_capability: Option<String>,
    pending_request: Option<DirectoryCommandRequestV1>,
    pending_cursor: Option<String>,
    fetch_requested: bool,
    pub(crate) code: Option<DirectoryCommandErrorCodeV1>,
}

impl ShellSpaceAdministrationOperationV1 {
    /// 🆕️ Opens the one operation for exactly one space; the first turn fetches its page.
    pub fn open(operation_epoch: u64, space_id: &str) -> Self {
        Self {
            operation_epoch,
            space_id: space_id.to_string(),
            phase: ShellSpaceAdministrationPhaseV1::Loading,
            page: None,
            canonical_json: None,
            receipt_sha256: None,
            invite_capability: None,
            pending_request: None,
            pending_cursor: None,
            fetch_requested: true,
            code: None,
        }
    }

    pub fn operation_epoch(&self) -> u64 {
        self.operation_epoch
    }
    pub fn space_id(&self) -> &str {
        &self.space_id
    }
    pub fn phase(&self) -> ShellSpaceAdministrationPhaseV1 {
        self.phase
    }
    pub fn page(&self) -> Option<&DirectorySpaceAdministrationPageV1> {
        self.page.as_ref()
    }
    pub fn canonical_json(&self) -> Option<&str> {
        self.canonical_json.as_deref()
    }
    pub fn receipt_sha256(&self) -> Option<&str> {
        self.receipt_sha256.as_deref()
    }
    pub fn code(&self) -> Option<DirectoryCommandErrorCodeV1> {
        self.code
    }
    /// 🎁️ `true` while the worker still holds one unacknowledged one-shot invite capability.
    pub fn invite_capability_pending(&self) -> bool {
        self.invite_capability.is_some()
    }

    /// 🛂️ The ONLY authority a renderer may consult: the page's own server-filled capability flags.
    pub fn capabilities(&self) -> Option<DirectorySpaceAdministrationCapabilitiesV1> {
        self.page.as_ref().and_then(DirectorySpaceAdministrationPageV1::capabilities)
    }

    /// 🎬️ Returns the one bounded step the driver may take next.
    pub fn turn(&mut self) -> ShellSpaceAdministrationTurnV1 {
        if self.phase.is_terminal() {
            return ShellSpaceAdministrationTurnV1::Closed;
        }
        if let Some(request) = self.pending_request.clone() {
            self.phase = ShellSpaceAdministrationPhaseV1::Submitting;
            return ShellSpaceAdministrationTurnV1::Submit { request };
        }
        if self.fetch_requested {
            self.fetch_requested = false;
            self.phase = if self.page.is_some() { ShellSpaceAdministrationPhaseV1::Refreshing } else { ShellSpaceAdministrationPhaseV1::Loading };
            return ShellSpaceAdministrationTurnV1::FetchPage { cursor: self.pending_cursor.take() };
        }
        ShellSpaceAdministrationTurnV1::Idle
    }

    /// 📄️ Requests exactly one page read; `cursor` advances precisely the window it was issued for.
    pub fn request_page(&mut self, cursor: Option<String>) -> bool {
        if self.phase.is_terminal() || self.pending_request.is_some() {
            return false;
        }
        self.pending_cursor = cursor;
        self.fetch_requested = true;
        true
    }

    /// 📮️ Admits exactly one command request; a second one before the receipt is refused.
    pub fn request_command(&mut self, request: DirectoryCommandRequestV1) -> bool {
        if !self.phase.is_dispatchable() || self.pending_request.is_some() || self.fetch_requested {
            return false;
        }
        self.pending_request = Some(request);
        true
    }

    /// 📥️ Applies exactly one canonical page whose space matches this operation.
    pub fn apply_page(&mut self, canonical_json: String, page: DirectorySpaceAdministrationPageV1) -> bool {
        if self.phase.is_terminal() || page.space_id() != self.space_id {
            return false;
        }
        self.page = Some(page);
        self.canonical_json = Some(canonical_json);
        self.phase = ShellSpaceAdministrationPhaseV1::Ready;
        true
    }

    /// 🧾️ Applies exactly one accepted server receipt, then schedules the mandatory page refresh.
    pub fn apply_receipt(&mut self, receipt: &DirectoryCommandReceiptV1) -> bool {
        let Some(request) = self.pending_request.take() else { return false };
        if request.request_id != receipt.request_id || self.phase.is_terminal() {
            return false;
        }
        self.receipt_sha256 = Some(receipt.receipt_sha256.clone());
        self.invite_capability = match &receipt.result {
            DirectoryCommandResultV1::Invite { invite_token } => Some(invite_token.clone()),
            DirectoryCommandResultV1::None => None,
        };
        self.phase = ShellSpaceAdministrationPhaseV1::Receipt;
        self.fetch_requested = true;
        self.pending_cursor = None;
        true
    }

    /// 🎁️ Hands the one-shot invite capability over exactly once and erases it in the same turn.
    pub fn acknowledge_capability(&mut self) -> Option<String> {
        self.invite_capability.take()
    }

    /// 🧯️ Erases page, receipt, and capability, then settles one terminal phase exactly once.
    pub fn terminate(&mut self, phase: ShellSpaceAdministrationPhaseV1, code: Option<DirectoryCommandErrorCodeV1>) {
        if self.phase.is_terminal() {
            return;
        }
        self.page = None;
        self.canonical_json = None;
        self.receipt_sha256 = None;
        self.invite_capability = None;
        self.pending_request = None;
        self.pending_cursor = None;
        self.fetch_requested = false;
        self.phase = if phase.is_terminal() { phase } else { ShellSpaceAdministrationPhaseV1::Failed };
        self.code = code;
    }

    /// 🚦️ Maps one page-read rejection onto the closed terminal vocabulary; 401/403 clears the pane.
    pub fn fail_page(&mut self, error: &DirectoryClientError) {
        let (phase, code) = match error {
            DirectoryClientError::Unauthorized => (ShellSpaceAdministrationPhaseV1::Denied, DirectoryCommandErrorCodeV1::Unauthorized),
            DirectoryClientError::Cancelled => (ShellSpaceAdministrationPhaseV1::Cancelled, DirectoryCommandErrorCodeV1::Cancelled),
            DirectoryClientError::Http { status: 403 | 404, .. } => (ShellSpaceAdministrationPhaseV1::Denied, DirectoryCommandErrorCodeV1::Forbidden),
            DirectoryClientError::Http { status: 409 | 410, .. } => (ShellSpaceAdministrationPhaseV1::Stale, DirectoryCommandErrorCodeV1::StaleSession),
            _ => (ShellSpaceAdministrationPhaseV1::Failed, DirectoryCommandErrorCodeV1::Transport),
        };
        self.terminate(phase, Some(code));
    }

    /// 🚦️ Maps one command rejection onto the closed terminal vocabulary. Nothing is ever retried:
    /// an indeterminate outcome must be resolved by an operator refresh, never by a silent reissue.
    pub fn fail_command(&mut self, code: DirectoryCommandErrorCodeV1) {
        let phase = match code {
            DirectoryCommandErrorCodeV1::Unauthorized | DirectoryCommandErrorCodeV1::Forbidden => ShellSpaceAdministrationPhaseV1::Denied,
            DirectoryCommandErrorCodeV1::StaleSession => ShellSpaceAdministrationPhaseV1::Stale,
            DirectoryCommandErrorCodeV1::Cancelled => ShellSpaceAdministrationPhaseV1::Cancelled,
            _ => ShellSpaceAdministrationPhaseV1::Failed,
        };
        self.terminate(phase, Some(code));
    }
}

/// 🛂️ An owner row can never be removed: the server rejects it, so the control is disabled before
/// dispatch rather than offered and then refused.
pub fn shell_space_administration_member_removable(row: &DirectorySpaceAdministrationMemberRowV1, capabilities: Option<DirectorySpaceAdministrationCapabilitiesV1>) -> bool {
    capabilities.is_some_and(|capabilities| capabilities.remove_member) && !row.owner
}

/// 🎟️ A revoked or accepted invitation is terminal; only a live one may be revoked.
pub fn shell_space_administration_invite_revocable(row: &DirectorySpaceAdministrationInviteRowV1, capabilities: Option<DirectorySpaceAdministrationCapabilitiesV1>) -> bool {
    capabilities.is_some_and(|capabilities| capabilities.revoke_invite) && !row.revoked && !row.accepted
}

/// 🗣️ Every visible administration string, in both languages, with no default: the caller passes the
/// active locale and gets the exact pair member. Mirrors the React pane's own bundle one-for-one.
pub fn shell_space_administration_label(key: &str, german: bool) -> &'static str {
    match (key, german) {
        ("title", false) => "Space administration",
        ("title", true) => "Space-Verwaltung",
        ("members", false) => "Members",
        ("members", true) => "Mitglieder",
        ("invites", false) => "Invitations",
        ("invites", true) => "Einladungen",
        ("role", false) => "Role",
        ("role", true) => "Rolle",
        ("author", false) => "Author",
        ("author", true) => "Autor",
        ("spectator", false) => "Spectator",
        ("spectator", true) => "Betrachter",
        ("owner", false) => "Owner",
        ("owner", true) => "Eigentümer",
        ("remove", false) => "Remove member",
        ("remove", true) => "Mitglied entfernen",
        ("issue", false) => "Issue invitation",
        ("issue", true) => "Einladung ausstellen",
        ("revoke", false) => "Revoke invitation",
        ("revoke", true) => "Einladung widerrufen",
        ("copy", false) => "Copy invitation link",
        ("copy", true) => "Einladungslink kopieren",
        ("more", false) => "Show more",
        ("more", true) => "Mehr anzeigen",
        ("loading", false) => "Loading the administration page…",
        ("loading", true) => "Verwaltungsseite wird geladen…",
        ("ready", false) => "Administration page is current.",
        ("ready", true) => "Verwaltungsseite ist aktuell.",
        ("submitting", false) => "Waiting for the server receipt…",
        ("submitting", true) => "Warte auf die Serverquittung…",
        ("receipt", false) => "Server receipt accepted.",
        ("receipt", true) => "Serverquittung angenommen.",
        ("refreshing", false) => "Refreshing after the receipt…",
        ("refreshing", true) => "Aktualisierung nach der Quittung…",
        ("cancelled", false) => "Administration was cancelled.",
        ("cancelled", true) => "Verwaltung wurde abgebrochen.",
        ("denied", false) => "Access to this space was withdrawn.",
        ("denied", true) => "Der Zugriff auf diesen Space wurde entzogen.",
        ("stale", false) => "This session changed; reopen administration.",
        ("stale", true) => "Diese Sitzung hat sich geändert; Verwaltung erneut öffnen.",
        ("failed", false) => "Unknown outcome — refresh required before retrying.",
        ("failed", true) => "Unbekanntes Ergebnis — vor einem erneuten Versuch aktualisieren.",
        _ => "",
    }
}

/// 🗣️ The status line one phase renders, in the active language.
pub fn shell_space_administration_status(phase: ShellSpaceAdministrationPhaseV1, german: bool) -> &'static str {
    let key = match phase {
        ShellSpaceAdministrationPhaseV1::Loading => "loading",
        ShellSpaceAdministrationPhaseV1::Ready => "ready",
        ShellSpaceAdministrationPhaseV1::Submitting => "submitting",
        ShellSpaceAdministrationPhaseV1::Receipt => "receipt",
        ShellSpaceAdministrationPhaseV1::Refreshing => "refreshing",
        ShellSpaceAdministrationPhaseV1::Cancelled => "cancelled",
        ShellSpaceAdministrationPhaseV1::Denied => "denied",
        ShellSpaceAdministrationPhaseV1::Stale => "stale",
        ShellSpaceAdministrationPhaseV1::Failed => "failed",
    };
    shell_space_administration_label(key, german)
}

/// 🎛️ One keyboard-reachable administration control the WGPU pane renders. `enabled` is derived
/// solely from the server's own capability flags and the operation's settled phase, so the native
/// renderer offers exactly the affordances the React pane does — and neither derives authority from
/// a locally stored role.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShellSpaceAdministrationControlV1 {
    pub control_id: String,
    pub label: &'static str,
    pub value: String,
    pub enabled: bool,
}

/// 🖼️ Builds the pane's complete control set from the canonical page alone. A `member`/`public`
/// page yields no administration control at all, because it structurally carries no capabilities.
pub fn shell_space_administration_controls(operation: &ShellSpaceAdministrationOperationV1, german: bool) -> Vec<ShellSpaceAdministrationControlV1> {
    let mut controls = Vec::new();
    controls.push(ShellSpaceAdministrationControlV1 {
        control_id: "os.space-administration.status".into(),
        label: shell_space_administration_status(operation.phase(), german),
        value: operation.receipt_sha256().unwrap_or_default().to_string(),
        enabled: false,
    });
    let Some(page) = operation.page() else { return controls };
    let capabilities = operation.capabilities();
    let dispatchable = operation.phase().is_dispatchable();
    if let DirectorySpaceAdministrationPageV1::Member { members, .. } | DirectorySpaceAdministrationPageV1::Author { members, .. } = page {
        for row in &members.rows {
            controls.push(ShellSpaceAdministrationControlV1 {
                control_id: format!("os.space-administration.role.{}", row.user_id),
                label: shell_space_administration_label("role", german),
                value: shell_space_administration_label(if row.role == DirectorySpaceRole::Author { "author" } else { "spectator" }, german).to_string(),
                enabled: dispatchable && capabilities.is_some_and(|capabilities| capabilities.upsert_member),
            });
            controls.push(ShellSpaceAdministrationControlV1 {
                control_id: format!("os.space-administration.remove.{}", row.user_id),
                label: shell_space_administration_label("remove", german),
                value: if row.owner { shell_space_administration_label("owner", german).to_string() } else { String::new() },
                enabled: dispatchable && shell_space_administration_member_removable(row, capabilities),
            });
        }
        if let Some(cursor) = &members.next_cursor {
            controls.push(ShellSpaceAdministrationControlV1 { control_id: "os.space-administration.members.more".into(), label: shell_space_administration_label("more", german), value: cursor.clone(), enabled: dispatchable });
        }
    }
    if let DirectorySpaceAdministrationPageV1::Author { invites, .. } = page {
        controls.push(ShellSpaceAdministrationControlV1 {
            control_id: "os.space-administration.invite.issue".into(),
            label: shell_space_administration_label("issue", german),
            value: String::new(),
            enabled: dispatchable && capabilities.is_some_and(|capabilities| capabilities.create_invite),
        });
        if operation.invite_capability_pending() {
            controls.push(ShellSpaceAdministrationControlV1 { control_id: "os.space-administration.invite.copy".into(), label: shell_space_administration_label("copy", german), value: String::new(), enabled: true });
        }
        for row in &invites.rows {
            controls.push(ShellSpaceAdministrationControlV1 {
                control_id: format!("os.space-administration.invite.revoke.{}", row.invite_id),
                label: shell_space_administration_label("revoke", german),
                value: row.invite_id.clone(),
                enabled: dispatchable && shell_space_administration_invite_revocable(row, capabilities),
            });
        }
        if let Some(cursor) = &invites.next_cursor {
            controls.push(ShellSpaceAdministrationControlV1 { control_id: "os.space-administration.invites.more".into(), label: shell_space_administration_label("more", german), value: cursor.clone(), enabled: dispatchable });
        }
    }
    controls
}

//#endregion 🏛️SpaceAdministration
//#region 🎮️DirectoryCommandQueue
/// ⏳️ Finite per-command deadline; a hung hub can never retain a command turn forever.
pub(crate) const DIRECTORY_COMMAND_DEADLINE_MS: u64 = 5_000;

/// ⏳️ Browser identity retry floor: a hub that refuses `/auth/sessions/me` must not be re-asked on
/// every 100 ms frame pump. Native needs no twin — its bootstrap is a one-shot pool future.
#[cfg(target_arch = "wasm32")]
pub(crate) const BROWSER_IDENTITY_RETRY_MS: u64 = 5_000;

/// 🧾️ One transient native command completion: either the authoritative receipt or a closed
/// terminal code. Never a raw server body, and never a logged capability.
#[derive(Clone, Debug, PartialEq)]
pub enum ShellDirectoryCommandResultV1 {
    Receipt(DirectoryCommandReceiptV1),
    Failed(DirectoryCommandErrorCodeV1),
}

/// 🎮️ The shell's owned, bounded FIFO of sealed directory-command operations plus their transient
/// request-id-keyed result slot. Every policy this transport owes — fixed capacity, byte-identical
/// retry of the head, stop-at-the-first-transient-fault, and proceed past a terminal auth/conflict
/// failure — lives here so it is provable without a renderer, a surface, or a live hub.
#[derive(Debug, Default)]
pub struct ShellDirectoryCommandQueueV1 {
    pub(crate) pending: Vec<DirectoryCommandRequestV1>,
    results: Vec<(String, ShellDirectoryCommandResultV1)>,
}

impl ShellDirectoryCommandQueueV1 {
    /// 📥️ Admits one sealed operation, answering the NEWEST intent with a terminal `capacity`
    /// result when full rather than silently discarding an older one the caller believes it issued.
    pub fn admit(&mut self, request: DirectoryCommandRequestV1) {
        let rejection = if request.validate().is_err() {
            Some(DirectoryCommandErrorCodeV1::Invalid)
        } else if self.pending.len() >= MAX_PENDING_DIRECTORY_COMMANDS {
            Some(DirectoryCommandErrorCodeV1::Capacity)
        } else {
            None
        };
        match rejection {
            Some(code) => self.retain(&request.request_id, ShellDirectoryCommandResultV1::Failed(code)),
            None => self.pending.push(request),
        }
    }

    /// 🚀️ Puts one freshly issued operation at the head so an interactive command is not stuck
    /// behind an offline backlog it never chose.
    pub fn admit_first(&mut self, request: DirectoryCommandRequestV1) {
        let request_id = request.request_id.clone();
        self.admit(request);
        if let Some(index) = self.pending.iter().position(|entry| entry.request_id == request_id) {
            let entry = self.pending.remove(index);
            self.pending.insert(0, entry);
        }
    }

    /// 👀️ The exact sealed request the next turn must re-send byte-identically.
    pub fn head(&self) -> Option<&DirectoryCommandRequestV1> {
        self.pending.first()
    }

    /// 🏁️ Applies one turn's outcome. Returns `true` while the FIFO may proceed: a transient fault
    /// retains the head and stops it; every terminal code produces a result and advances.
    pub fn settle(&mut self, outcome: Result<DirectoryCommandReceiptV1, DirectoryCommandErrorCodeV1>) -> bool {
        let Some(request) = self.pending.first().map(|request| request.request_id.clone()) else {
            return false;
        };
        match outcome {
            Err(code) if code.is_transient() => false,
            Err(code) => {
                self.pending.remove(0);
                self.retain(&request, ShellDirectoryCommandResultV1::Failed(code));
                true
            }
            Ok(receipt) => {
                self.pending.remove(0);
                self.retain(&request, ShellDirectoryCommandResultV1::Receipt(receipt));
                true
            }
        }
    }

    /// 🧾️ Reads one retained transient result for the issuing surface.
    pub fn result(&self, request_id: &str) -> Option<&ShellDirectoryCommandResultV1> {
        self.results.iter().find(|(id, _)| id == request_id).map(|(_, result)| result)
    }

    /// 🔢️ Operations still awaiting a live turn.
    pub fn pending(&self) -> usize {
        self.pending.len()
    }

    pub(crate) fn retain(&mut self, request_id: &str, result: ShellDirectoryCommandResultV1) {
        self.results.retain(|(id, _)| id != request_id);
        if self.results.len() >= MAX_DIRECTORY_COMMAND_RESULTS {
            self.results.remove(0);
        }
        self.results.push((request_id.to_string(), result));
    }
}

//#endregion 🎮️DirectoryCommandQueue
//#region 🔐️HubWorkspaceTask
/// ⏳️ The credential sign-in's own deadline (the mint and the `me` read that follows it). A mint spends a
/// password hash on the hub, which the directory command deadline does not fit: measured 6.8 s and 13.6 s on
/// hub 7800 under load ~40, so every native sign-in answered `Unreachable` while React's (no request
/// deadline, cancellable) signed in (ticket 26/09/23 slice WG8, session 12). Still finite.
pub(crate) const HUB_SIGN_IN_DEADLINE_MS: u64 = 30_000;

/// 🔐️ What one spawned sign-in answers: the mint's refusal, a mint the hub's own `me` read would not confirm, or the
/// verified session.
pub(crate) enum ShellHubSignInAnswer {
    Failed { code: HubSignInErrorCode, retry_after_seconds: Option<u64> },
    Unverified,
    Verified { origin: String, user_id: String, credential: std::sync::Arc<LocalHubCredential>, client: std::sync::Arc<ShellDirectoryClient>, authority: DirectorySessionAuthorityV1 },
}

/// 🏘️ What one spawned spaces read answers: the rows, or `Err` when the list could not be read (the rows on screen stay).
pub(crate) type ShellHubSpacesAnswer = Result<Vec<crate::space_browser::SpaceRow>, ()>;

/// 🔐️ One hub workspace request whose network legs run on a spawned task and never on the frame's interaction state:
/// the frame keeps taking input while the hub answers, the chrome shows the phase, and `cancel` ends the request. A
/// slow hub froze every input of the browser shell for 60–106 s while these legs were awaited inside the action
/// (ticket 26/09/23 session 12, runs s12g/s12h on hub 7800).
pub(crate) struct ShellHubTask<T> {
    pub(crate) receiver: std::sync::mpsc::Receiver<T>,
    pub(crate) cancel: CancelToken,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) task: Option<std::sync::Arc<ShellPoolFuture>>,
}

impl<T: 'static> ShellHubTask<T> {
    /// 🚀️ Runs `leg` on the shared pool's I/O lane; the pump reads its answer.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn spawn(cancel: CancelToken, leg: impl std::future::Future<Output = T> + Send + 'static) -> Self
    where
        T: Send,
    {
        let (sender, receiver) = std::sync::mpsc::channel();
        let task = ShellPoolFuture::spawn(crate::renderer_worker_pool(), Lane::Io, async move {
            let _ = sender.send(leg.await);
        });
        Self { receiver, cancel, task: Some(task) }
    }

    /// 🚀️ Runs `leg` on the page's own microtask queue; the pump reads its answer.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn spawn(cancel: CancelToken, leg: impl std::future::Future<Output = T> + 'static) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        crate::spawn_app_task(async move {
            let _ = sender.send(leg.await);
        });
        Self { receiver, cancel }
    }

    /// 📬️ `Some(Ok)` once answered, `Some(Err)` when the task ended without an answer, `None` while it runs.
    pub(crate) fn answer(&self) -> Option<Result<T, ()>> {
        match self.receiver.try_recv() {
            Ok(answer) => Some(Ok(answer)),
            Err(std::sync::mpsc::TryRecvError::Empty) => None,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => Some(Err(())),
        }
    }

    /// 🛑️ Ends the request: its transport sees the cancellation, and no answer is read any more.
    pub(crate) fn cancel(self) {
        self.cancel.cancel_now();
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(task) = self.task {
            task.cancel();
        }
    }
}

//#endregion 🔐️HubWorkspaceTask
//#endregion ShellTypes

