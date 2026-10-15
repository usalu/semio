//! 🏛️ Space administration, directory queue, hub tasks and the settle pump.
//!
//! Space administration operations, the directory command queue, hub workspace tasks, boot program selection,
//! the settle pump and its verdicts, pointer capture and the presented-input geometry.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

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
/// 🎮️ Bounded (contract §C6 "bounded, in-memory") native directory-command FIFO depth.
pub(crate) const MAX_PENDING_DIRECTORY_COMMANDS: usize = 64;
/// 🧾️ Bounded transient command-result slot depth.
pub(crate) const MAX_DIRECTORY_COMMAND_RESULTS: usize = 64;
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

//#region 🧯️BootProgramSelection
/// 🧯 One plugin whose boot activation faulted, kept as data so the shell can show WHICH plugin is
/// unavailable instead of dying with the plugin. `detail` is the guest-side cause verbatim (a
/// first-step trap message, a missing module, …) — never re-worded, so a browser status line and a
/// console trace read the same.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShellPluginFault {
    pub plugin_id: String,
    pub app_id: String,
    pub detail: String,
    #[cfg(not(target_arch = "wasm32"))]
    pub frame: Option<semio_framework_actor::pack::PackError>,
    #[cfg(not(target_arch="wasm32"))]
    pub kernel:Option<semio_framework_actor::KernelError>,
    #[cfg(not(target_arch="wasm32"))]
    pub actor_call:Option<semio_framework_plugin_host::original_actor_context::OriginalActorFixedCall>,
}

/// 🧯 One SURFACE whose `render_with_document` failed, kept as data for exactly the reason
/// [`ShellPluginFault`] is: a surface that cannot be read is a per-surface fault, never a boot failure.
/// Propagating it with `?` made `renderDocument promise failed: wgpu-ui.intake-budget-exhausted` become
/// `shell-boot: …`, then `worker-boot-failed`, which closed the Worker and took its transferred
/// `OffscreenCanvas` with it — one unreadable window body killed the whole renderer. `detail` is the
/// cause verbatim, so the fault card and the console trace read the same.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShellSurfaceFault {
    pub surface_id: String,
    pub body_key: String,
    pub detail: String,
    #[cfg(not(target_arch="wasm32"))]
    pub frame:Option<semio_framework_actor::pack::PackError>,
    #[cfg(not(target_arch="wasm32"))]
    pub kernel:Option<semio_framework_actor::KernelError>,
    #[cfg(not(target_arch="wasm32"))]
    pub actor_call:Option<semio_framework_plugin_host::original_actor_context::OriginalActorFixedCall>,
}

/// 🪟️ One bounded roster mutation whose first retained body is the release condition for its
/// optional React-equivalent transfer journal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WindowTopologyPublication {
    pub(crate) window_id: String,
    pub(crate) body_key: String,
    pub(crate) attempts: u8,
    pub(crate) journal_token: Option<u64>,
}

/// 🎯️ Which program + app a standalone (non-host) boot opens, resolved from the requested variant
/// alone. `resolvePlaygroundBoot` hands the renderer its plugins in DEPENDENCY order, so the first
/// entry is a dependency of the requested plugin (`flow` ahead of `procedural` for `generation3d`),
/// never the requested plugin itself: taking `plugins.first()` booted a foreign plugin's first app
/// and made that plugin's first-step trap the whole shell's boot failure. Falls back to the program
/// that actually declares the requested app id, and only then to the requested program's first app;
/// a variant whose plugin is absent resolves to `None` rather than to somebody else's app.
///
/// 📌️ `?app=`/`--app`/`appId` PINS the anchor, React's `appId` — a pin the manifest does not
/// declare is dropped back to the variant's own app, the same "a url is not a place to hard-fail a
/// shell" rule `?role=`/`?mode=`/`?example=` follow.
pub fn select_boot_program(programs: &[(&str, &semio_framework::PluginManifest)], request: BootSelectionRequest<'_>, app_role: semio_framework::manifest::AppRole, pinned_app_id: Option<&str>) -> Option<(usize, AppDefinition)> {
    let requested_app_id = pinned_app_id.filter(|app_id| !app_id.is_empty() && programs.iter().any(|(_, manifest)| manifest.apps.iter().any(|app| app.id == *app_id))).or(request.app_id);
    let index = programs.iter().position(|(plugin_id, _)| *plugin_id == request.plugin_id).or_else(|| requested_app_id.and_then(|app_id| programs.iter().position(|(_, manifest)| manifest.apps.iter().any(|app| app.id == app_id))))?;
    let manifest = programs[index].1;
    let anchor = requested_app_id.and_then(|app_id| manifest.apps.iter().find(|app| app.id == app_id)).or_else(|| manifest.apps.first())?;
    Some((index, project_boot_app_role(manifest, anchor, app_role).clone()))
}

/// 🚀️ Resolves a declared plugin and optional app at the boot composition boundary.
#[derive(Clone, Copy, Debug)]
pub struct BootSelectionRequest<'a> {
    pub plugin_id: &'a str,
    pub app_id: Option<&'a str>,
}

/// 👁️✏️ Projects the variant's anchor app onto the boot ROLE (`?role=viewer`, `SEMIO_APP_ROLE`,
/// `VITE_SEMIO_APP_ROLE`). A surface app id is `<dialect>#<role>` by construction
/// (`semio_framework::surface_app_id`), so the registry's `generation3d → …#editor` row names the
/// right DIALECT and the wrong role the moment a viewer boot is requested. This is the Rust twin of
/// React's `resolveBootPrimaryAppV1` (`🏛️ShellHost/🔀️surface-switch/🟦️.ts`) and follows its rule
/// exactly: a manifest that declares the sibling surface for the anchor's dialect wins, and a
/// manifest that does not keeps the anchor — a viewer request on an editor-only artifact is a
/// downgrade to the surface that exists, never a dead boot.
fn project_boot_app_role<'a>(manifest: &'a semio_framework::PluginManifest, anchor: &'a AppDefinition, app_role: semio_framework::manifest::AppRole) -> &'a AppDefinition {
    if anchor.role == app_role {
        return anchor;
    }
    manifest.apps.iter().find(|app| app.role == app_role && app.dialect == anchor.dialect).unwrap_or(anchor)
}
//#endregion 🧯️BootProgramSelection

/// 📏️ Rounds one `flush_deferred_actions` may spend chasing the chain an extension answer
/// re-arms. The native preview chain converges in five ticks and six extension round trips per
/// preview window; this is generous enough for several windows and finite enough that a guest that
/// never settles is reported instead of spinning forever.
pub(crate) const SHELL_DEFERRED_CHAIN_ROUNDS: usize = 512;

/// 📏️ Bounded steps one `drain_retained_document_arenas` spends returning the previous refresh's
/// retained documents and their `UiValue` pages. Finite so a registry that never settles is left
/// behind rather than spinning the shell.
pub(crate) const SHELL_DOCUMENT_RETIREMENT_STEPS: usize = 65_536;

/// 📏️ Items one retirement step may return. A page, never one item — the unit the rest of this
/// shell's close ladder is already priced in (`📓️close-ladder-budget-2026-09-12.md` §3).
pub(crate) const SHELL_DOCUMENT_RETIREMENT_ITEMS: usize = 1_024;

/// 📏️ Bytes one retirement step may return. Kept under the guest contiguous request ceiling so a
/// grant never asks an arena for a block `dlmalloc` cannot serve once fragmented.
pub(crate) const SHELL_DOCUMENT_RETIREMENT_BYTES: usize = 32 * 1_024;

/// 📏️ Convergence rounds one `flush_deferred_actions` may spend re-entering the window bodies after
/// its actions and extension answers have quiesced.
///
/// ⚖️ A guest's preview job is driven BY its own window render: rendering `procedural-preview` is
/// what arms the next `flowEvalTick`, and each tick's extension answer arms exactly one more. One
/// refresh plus one flush therefore stops the moment the chain THAT render armed quiesces — measured
/// on 6118 as `meshingFaces 6/8` with the solid never published. React has no such hole because a
/// mutation re-renders the shell, which re-enters every window body and re-arms
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(crate) const SHELL_SETTLE_ROUNDS: usize = 64;

/// 🪟️ Full-refresh attempts a newly admitted window may spend acquiring its required first
/// retained body before the host exposes one terminal publication refusal and retires its journal.
pub(crate) const WINDOW_TOPOLOGY_PUBLICATION_ATTEMPTS: u8 = 3;

/// 📏️ Opportunities one window body's retained paint may spend inside a single chrome walk before
/// the walk gives up on it and moves to the next window. Generous enough for the largest document
/// this shell renders and finite so one stalled body cannot hold the frame — and with it the navbar,
/// the panels and the GPU present — forever.
pub(crate) const SHELL_WINDOW_PAINT_OPPORTUNITIES: usize = 1 << 20;

/// ⏱️ Spends one of a retained document's paint opportunities — only when the step was the document's OWN work
/// (`UiDocumentFrameCursor::last_step_was_own_work`: waiting on the layout worker pool or on another surface's layout costs
/// nothing) — and answers whether the document may keep stepping under `budget`.
pub(crate) fn document_opportunity_remains(cursor: &mut ShellChromeChildCursor, budget: usize) -> bool {
    if cursor.document.last_step_was_own_work() {
        cursor.scalar = cursor.scalar.saturating_add(1);
    }
    !cursor.document.terminal_is_fault() && cursor.scalar < budget
}

/// 📏️ Pump steps ONE live producer may publish the same progress reading for before the pump stops
/// believing the guest will arm another hop and drives that run to a terminal state.
///
/// ⚖️ A step is one frame, so this is a wall-clock patience measured in frames rather than in
/// milliseconds: a solve that is genuinely advancing moves its `unitsDone`/`facesDone`/`inFlight`
/// witness and resets the count, and only a producer whose witness is frozen — `meshingFaces 36/56`
/// for 200 s on 6118, deaf to `setActiveExample` — ever reaches it
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-progress-visibility-2026-09-14.md` §8.2).
pub(crate) const SHELL_SETTLE_STALL_STEPS: u32 = 240;

/// 📏️ Terminal drives the pump may aim at ONE wedged surface before it leaves the run alone. The
/// abort is the producer's OWN published `cancelAction`, and the run it retires is commonly restarted
/// by the producer's own restart policy — so this is bounded to keep a producer that answers an abort
/// with a fresh wedge from becoming an abort storm.
pub(crate) const SHELL_SETTLE_TERMINAL_DRIVES: u32 = 3;

/// 📏️ Steps between two settle-pump heartbeat traces. A converging edit alternates `Drained` and
/// `Crossed` every frame, so the trace reports class TRANSITIONS plus one heartbeat per this many
/// steps — enough for a probe to prove the lane is live and running, few enough to read.
pub(crate) const SHELL_SETTLE_HEARTBEAT_STEPS: u64 = 64;

/// ⏳️ What one [`ShellState::settle_pump_step`] did — the frame loop traces it so a step that cost a
/// guest crossing can be told from one that only drained an already-armed action.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellSettleStep {
    /// ♻️ The step executed armed work (deferred actions, extension answers, file opens) and took the
    /// refresh pass that work owed.
    Drained,
    /// 🫀️ Nothing was armed and a producer is still working, so the step spent ONE guest crossing on
    /// exactly the window bodies whose producers report `computing`.
    Crossed,
    /// 🧯 A producer published the same witness for [`SHELL_SETTLE_STALL_STEPS`] steps, so the step
    /// drove its run to a terminal state through the producer's own published `cancelAction`.
    Wedged,
    /// 🏁️ Nothing is armed and nothing is working — the chain has settled and the pump stands down.
    Quiescent,
}

/// ⏳️ One live producer's progress witness, as the pump watches it across frames.
#[derive(Default)]
pub(crate) struct ShellSettleWatch {
    pub(crate) signature: String,
    stalled_steps: u32,
    terminal_drives: u32,
    pub(crate) standing: bool,
}

/// 🫀️ What the watchdog decided about ONE live producer this step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShellSettleVerdict {
    /// 🫀️ Spend this step's crossing on the producer's own window body.
    Fund,
    /// 🧯 The witness has been frozen for [`SHELL_SETTLE_STALL_STEPS`] steps: drive the run to a
    /// terminal state through the producer's own published `cancelAction`.
    Terminal,
    /// 🏁️ The witness is frozen and there is nothing left to try — the producer published no way to
    /// end its own run, or the terminal-drive budget is spent. Crossings are not helping it, so the
    /// pump stops paying for them until the witness moves again.
    ///
    /// ⚖️ This is the term that keeps a producer which reports `computing` FOREVER from pinning the
    /// host at one guest crossing per frame. The generation3d preview does exactly that on the served
    /// guest — 560 consecutive samples of `computing:true phase:"idle" inFlight:0 ratio:1.0`, over a
    /// window in which nothing was evaluating at all (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `📓️wgpu-progress-visibility-2026-09-14.md` §3.1) — and a pump that believed it would never
    /// stand down.
    Stand,
}

/// 🫀️ Whether the frame loop owes the shell a settle step — the whole predicate, over the parts a
/// frame can see, so it is drivable without a live guest.
///
/// ⚖️ Derived from five independent terms, never from the arm alone: a step that forgot to re-arm
/// cannot park a live chain, and a chain nobody armed cannot pin the pump awake.
pub(crate) fn settle_pump_owes(settling: bool, has_session: bool, armed_work: bool, owed_scope: &UiDirtyScope, armed: bool, computing: bool) -> bool {
    if settling || !has_session {
        return false;
    }
    armed_work || !owed_scope.asks_for_nothing() || armed || computing
}

/// 🧯 The watchdog's verdict for ONE live producer, and the only place the stall is counted.
///
/// A witness that MOVED resets everything — including the terminal-drive count, so a producer that
/// recovered after an abort is watched from scratch. A witness that is frozen counts up, and the step
/// that crosses [`SHELL_SETTLE_STALL_STEPS`] asks for a terminal drive at most
/// [`SHELL_SETTLE_TERMINAL_DRIVES`] times — a producer that answers an abort with a fresh wedge is a
/// producer defect, not a reason for the host to abort forever.
///
/// ⚠️ A producer that offers no `cancelAction` is never driven: this shell learns no domain verb from
/// code, so a run it has no published way to end is left to the user.
pub(crate) fn settle_watch_verdict(watch: &mut ShellSettleWatch, status: &World3dComputeStatus) -> ShellSettleVerdict {
    let signature = settle_progress_signature(status);
    if watch.signature != signature {
        watch.signature = signature;
        watch.stalled_steps = 0;
        watch.terminal_drives = 0;
        watch.standing = false;
        return ShellSettleVerdict::Fund;
    }
    watch.stalled_steps = watch.stalled_steps.saturating_add(1);
    if watch.stalled_steps < SHELL_SETTLE_STALL_STEPS {
        return ShellSettleVerdict::Fund;
    }
    if watch.terminal_drives >= SHELL_SETTLE_TERMINAL_DRIVES || !status.cancellable {
        watch.stalled_steps = SHELL_SETTLE_STALL_STEPS;
        watch.standing = true;
        return ShellSettleVerdict::Stand;
    }
    watch.stalled_steps = 0;
    watch.terminal_drives = watch.terminal_drives.saturating_add(1);
    ShellSettleVerdict::Terminal
}

/// 🫀️ The shell's half of the runtime-owned settle pump — the wgpu twin of React's
/// `createUiRefreshCoalescerV1` drain loop (`🛠️ShellHelpers/🟦️.tsx`).
///
/// React's shell never converges a chain inside one call either: a pass DECLARES what it still owes
/// and the lane's own loop runs the next pass, so the browser paints between them. This is that lane,
/// with the loop owned by the frame runtime instead of by a promise chain — `settle_boot` arms it and
/// returns, `🧊️renderer/🦀️.rs`'s `FrameDeferredWork::Settle` takes exactly one step per frame, and the
/// step's own verdict says whether another is owed.
///
/// Two things only this layer can own live here:
///
/// * **the guest crossing that funds a background solve.** A window body's render is also the turn
///   that pumps the guest's worker pool, which is why narrowing `refresh_ui` on `UiDirtyScope` alone
///   froze 14 of 16 examples mid-solve (`📓️wgpu-dirty-scope-refresh-2026-09-14.md` §4). The pump makes
///   that crossing per FRAME, on exactly the surfaces whose producers report `computing`, so the
///   refresh may narrow without withdrawing compute from a solve the UI has nothing to do with.
/// * **the wedge watchdog.** A producer that stops arming hops leaves a non-terminal run standing, and
///   a non-terminal run refuses every restart — the preview then ignores `setActiveExample` forever
///   (`📓️wgpu-progress-visibility-2026-09-14.md` §8.2). The pump watches each producer's own witness
///   and, when it freezes, dispatches the producer's own published `cancelAction`, which is the one
///   gesture measured to unblock it (§7.1 of the same report).
#[derive(Default)]
pub struct ShellSettlePump {
    pub(crate) owed: bool,
    pub(crate) steps: u64,
    pub(crate) crossings: u64,
    pub(crate) traced: Option<ShellSettleStep>,
    pub(crate) watches: HashMap<String, ShellSettleWatch>,
    pub(crate) rendering: Option<ShellDetached<ShellRenderedRefresh>>,
}

/// ⏳️ The progress witness the wedge watchdog compares across frames — every counter a producer moves
/// while it is genuinely advancing, and nothing that changes on its own.
fn settle_progress_signature(status: &World3dComputeStatus) -> String {
    format!("{}|{}|{}|{}|{}|{}", status.phase, status.units_done, status.units_total, status.faces_done, status.faces_total, status.in_flight)
}

/// 📥️ One parked `Effect::InvokeExtension`, awaiting the flush that can publish its answer.
#[cfg(target_arch = "wasm32")]
pub(crate) struct PendingExtensionInvocation {
    pub(crate) req: u64,
    pub(crate) extension_id: String,
    pub(crate) capability: String,
    pub(crate) request_json: String,
}

/// 📤️ One parked `Effect::RequestFileOpen`, awaiting the flush that can open a real picker.
///
/// ⚖️ Parked rather than opened inline for the same reason an extension invocation is: opening a
/// picker is asynchronous (it ends on a user gesture) and `queue_host_effects` is deliberately a
/// plain `fn`. `drain_deferred_actions` owns `&mut self`, so it can await the pick AND dispatch the
/// import chunks it produces through the normal action path, folding each one's own effects back in.
#[cfg(target_arch = "wasm32")]
pub(crate) struct PendingFileOpen {
    pub(crate) controller_id: String,
    pub(crate) accept: String,
    pub(crate) read_as: Option<String>,
    pub(crate) import_action: String,
    pub(crate) args: Option<semio_framework::DslValue>,
    pub(crate) multiple: bool,
}

/// 🎯️ The two layers a pointer can belong to. Chrome is everything the SHELL paints — the navbar,
/// the dock's structure, a pane's own overlay chips, an anchored panel, a menu, a dropdown, a dialog
/// and the tour veil; Surface is the engine canvas under it. React gets this partition for free from
/// the DOM (its chrome is elements ABOVE the `<canvas>`), and the wgpu target has to state it, which
/// is what [`ShellState::pointer_hit_owner`] does. One owner per point, and — through
/// `AppInteractionState`'s capture — one owner per pointer SEQUENCE.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointerHitOwner {
    Chrome,
    Surface,
}

/// 🎯️🖱️ The layer that owns the pointer SEQUENCE in flight — the DOM's implicit pointer capture,
/// which the wgpu target has to keep itself because it has one canvas and no event targets.
///
/// ⚖️ A press captures the layer it landed on and every move and the release belong to it until the
/// release ends the sequence. Both directions matter: a press on chrome keeps the whole gesture away
/// from the engine surface under it (React's `pointerdown` on a `<button>` means the canvas never
/// sees that `pointerup`), and a press on the surface keeps an orbit drag on the surface even where
/// it sweeps over a panel (three's `OrbitControls` calls `setPointerCapture` on the canvas).
///
/// 🩸️ Without it every chrome click ALSO reached the world pane painted under it — the reference
/// journals one `noteShellCommand` for a navbar panel tab where wgpu journalled
/// `noteShellCommand` + `interactionHover` + `interactionSelect`, and the stray
/// `interactionSelect` carried empty `targets`, i.e. pressing a panel tab CLEARED the world
/// selection (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
/// `📓️w11a-prepared-world-mesh-missing.md` §6 family A).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointerCapture {
    pub(crate) slots: [Option<PointerCaptureSlot>; crate::element_contracts::SCENE_POINTER_OWNER_CAPACITY],
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PointerCaptureSlot {
    pub(crate) pointer_id: ui_render::PointerId,
    pub(crate) owner: PointerHitOwner,
    pub(crate) position: [f32; 2],
}

impl PointerCapture {
    pub fn press(&mut self, pointer_id: ui_render::PointerId, owner: PointerHitOwner, x: f32, y: f32) -> Option<PointerHitOwner> {
        let index = self.slots.iter().position(|slot| slot.is_some_and(|slot| slot.pointer_id == pointer_id)).or_else(|| self.slots.iter().position(Option::is_none))?;
        self.slots[index] = Some(PointerCaptureSlot { pointer_id, owner, position: [x, y] });
        Some(owner)
    }

    /// 🖱️⬆️ Ends the sequence and answers the layer that owned it. A release with no press before it
    /// — the pointer entered the canvas already down — is the surface's, as it is in the DOM.
    pub fn release(&mut self, pointer_id: ui_render::PointerId) -> PointerHitOwner {
        self.slots.iter_mut().find(|slot| slot.is_some_and(|slot| slot.pointer_id == pointer_id)).and_then(Option::take).map_or(PointerHitOwner::Surface, |slot| slot.owner)
    }

    pub fn owner_of_move(&self, pointer_id: ui_render::PointerId, under_pointer: PointerHitOwner) -> PointerHitOwner {
        self.holder(pointer_id).unwrap_or(under_pointer)
    }

    pub fn holder(&self, pointer_id: ui_render::PointerId) -> Option<PointerHitOwner> {
        self.slots.iter().flatten().find(|slot| slot.pointer_id == pointer_id).map(|slot| slot.owner)
    }

    pub fn position(&self, pointer_id: ui_render::PointerId) -> Option<[f32; 2]> {
        self.slots.iter().flatten().find(|slot| slot.pointer_id == pointer_id).map(|slot| slot.position)
    }

    pub fn advance(&mut self, pointer_id: ui_render::PointerId, x: f32, y: f32) -> Option<(f32, f32)> {
        let slot = self.slots.iter_mut().flatten().find(|slot| slot.pointer_id == pointer_id)?;
        let delta = (x - slot.position[0], y - slot.position[1]);
        slot.position = [x, y];
        Some(delta)
    }

    pub fn any_active(&self) -> bool {
        self.slots.iter().any(Option::is_some)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PresentedInputCandidateWitness(pub(crate) u64);

pub(crate) struct PresentedInputGeometry {
    pub(crate) retained_body_rects: Vec<(String, Rect)>,
    pub(crate) media_owners: Vec<(String, Option<crate::media_slots::PresentedMediaOwner>)>,
    pub(crate) media_blocked: bool,
    pub(crate) retained_surfaces: Vec<(String, ui_wgpu::wgpu::UiSurfaceToken)>,
    pub(crate) dock_canvas_bounds: Rect,
    pub(crate) dock_axis_separator: f32,
    pub(crate) dock_drop_tab_bars: Vec<(Vec<usize>, WindowStackCorner, Rect, Vec<f32>)>,
    pub(crate) dock_drop_bodies: Vec<(Vec<usize>, Rect, String)>,
    pub(crate) dock_window_plan: Vec<(String, Rect)>,
    pub(crate) window_content_rects: HashMap<String, Rect>,
    pub(crate) window_silhouettes: HashMap<String, WindowSilhouette>,
    pub(crate) modal: bool,
    pub(crate) panel_rects: Vec<Rect>,
}

impl Default for PresentedInputGeometry {
    fn default() -> Self {
        Self {
            retained_body_rects: Vec::new(),
            media_owners: Vec::new(),
            media_blocked: false,
            retained_surfaces: Vec::new(),
            dock_canvas_bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
            dock_axis_separator: 0.0,
            dock_drop_tab_bars: Vec::new(),
            dock_drop_bodies: Vec::new(),
            dock_window_plan: Vec::new(),
            window_content_rects: HashMap::new(),
            window_silhouettes: HashMap::new(),
            modal: false,
            panel_rects: Vec::new(),
        }
    }
}
//#endregion ShellTypes
