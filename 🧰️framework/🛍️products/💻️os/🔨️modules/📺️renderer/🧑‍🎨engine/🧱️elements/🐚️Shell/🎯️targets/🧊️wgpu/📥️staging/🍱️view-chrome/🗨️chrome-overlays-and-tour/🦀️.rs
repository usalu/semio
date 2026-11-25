//! 🍱️ `shell-view-chrome` staging of `chrome-overlays-and-tour`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour

//#region 🧯️TransientNoticeAndAgentOverlays

/// 🧾️ A wrapped status band: its rect, the wrapped lines and the line height they paint at.
pub struct StatusBandLayout {
    pub rect: Rect,
    pub lines: Vec<String>,
    pub line_h: f32,
}

/// 📐️ Word-wraps `text` to `max_chars` glyphs a line, paragraphs split on `\n`; a word longer than a line stays whole and is clipped by the painter.
pub fn wrap_status_text(text: &str, max_chars: usize) -> Vec<String> {
    let max_chars = max_chars.max(1);
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let mut line = String::new();
        for word in paragraph.split_whitespace() {
            if !line.is_empty() && line.chars().count() + 1 + word.chars().count() > max_chars {
                lines.push(std::mem::take(&mut line));
            }
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(word);
        }
        lines.push(line);
    }
    lines
}

/// 📐️ One `px-double py-single` band of `text-sm` copy beside an action label: shrink-to-fit up to `max_width`, never wider than the viewport, the copy wrapping onto more lines when it outgrows it. The width estimate is the same monospace approximation `render_chrome_tooltip` uses — this renderer measures glyphs only while painting them.
pub fn status_band_layout(text: &str, action_label: &str, width: f32, max_width: f32, top: f32, theme: &Theme) -> StatusBandLayout {
    let glyph_w = theme.font_size_small * 0.6;
    let line_h = theme.font_size_small * 1.6;
    let single_w = (text.chars().count() + action_label.chars().count() + 2) as f32 * glyph_w;
    let band_w = (single_w + theme.padding_standard * 4.0).min(max_width).min((width - theme.padding_standard * 2.0).max(1.0));
    let action_w = (action_label.chars().count() + 1) as f32 * glyph_w;
    let lines = wrap_status_text(text, ((band_w - theme.padding_standard * 4.0 - action_w) / glyph_w).floor().max(1.0) as usize);
    let band_h = line_h * lines.len() as f32 + theme.padding_standard * 2.0;
    StatusBandLayout { rect: Rect::new(((width - band_w) * 0.5).max(0.0), top, band_w, band_h), lines, line_h }
}

/// 📐️ The banner, top-centre at `top-workbench` (`navbar_height + 8px`), sized like React's `absolute left-1/2 -translate-x-1/2` box: it shrink-wraps its copy and the close control, up to the half of the workbench `left-1/2` leaves it (`toast.maxWidthFraction` of the shared schema).
pub fn transient_notice_layout(notice: &ShellTransientNotice, close_label: &str, width: f32, theme: &Theme) -> StatusBandLayout {
    status_band_layout(&notice.message, close_label, width, width * semio_renderer_wgpu_shell_chrome_state::chrome_parity_constants().toast.max_width_fraction, theme.navbar_height + TRANSIENT_NOTICE_TOP_GAP, theme)
}

/// 📐️ The banner's rect — [`transient_notice_layout`]'s box.
pub fn transient_notice_rect(notice: &ShellTransientNotice, close_label: &str, width: f32, theme: &Theme) -> Rect {
    transient_notice_layout(notice, close_label, width, theme).rect
}

//#region 🎬️PluginInstallBand

/// 🎨️ `(border, fill, text)` for one phase, reusing [`transient_notice_tone`]'s own severity map so
/// the band tints exactly like every other shell banner: a running install is neutral, a cancelled
/// one a warning, a failed one destructive.
pub fn plugin_install_tone(phase: &ShellPluginInstallPhase, theme: &Theme) -> (Rgba, Rgba, Rgba) {
    match phase {
        ShellPluginInstallPhase::Resolving | ShellPluginInstallPhase::Loading => transient_notice_tone(semio_framework::Severity::Info, theme),
        ShellPluginInstallPhase::Cancelled => transient_notice_tone(semio_framework::Severity::Warning, theme),
        ShellPluginInstallPhase::Failed(_) => transient_notice_tone(semio_framework::Severity::Error, theme),
    }
}

/// 📐️ The band's rect — same top-centre geometry as the notice, one band height further down.
pub fn plugin_install_rect(message: &str, action_label: &str, width: f32, theme: &Theme) -> Rect {
    let glyphs = message.chars().count() + action_label.chars().count() + 2;
    let text_w = glyphs as f32 * theme.font_size_small * 0.6;
    let band_w = (text_w + theme.padding_standard * 4.0).min(width * semio_renderer_wgpu_shell_chrome_state::chrome_parity_constants().status_stacks.install_max_width_fraction).min((width - theme.padding_standard * 2.0).max(1.0));
    let band_h = theme.font_size_small * 1.6 + theme.padding_standard * 2.0;
    Rect::new(((width - band_w) * 0.5).max(0.0), theme.navbar_height + TRANSIENT_NOTICE_TOP_GAP + band_h + PLUGIN_INSTALL_TOP_GAP, band_w, band_h)
}

//#endregion 🎬️PluginInstallBand
//#region 🚪️DocumentOpenBand

/// 🗣️ The band's message: phase, which app, step and elapsed seconds while it runs; a failure appends
/// its own reason. English first, German through [`shell_chrome_string`].
pub fn document_opening_banner_text(opening: &ShellDocumentOpening, now_ms: f64, is_de: bool) -> String {
    if opening.kept && opening.running() {
        return shell_chrome_string("local-files.opening", is_de).replace("{name}", &opening.label);
    }
    let (key, step) = match opening.phase {
        ShellDocumentOpenPhase::Resolving => ("document.open.resolving", Some(1)),
        ShellDocumentOpenPhase::Instantiating => ("document.open.instantiating", Some(2)),
        ShellDocumentOpenPhase::Seeding => ("document.open.seeding", Some(DOCUMENT_OPEN_STEPS)),
        ShellDocumentOpenPhase::Cancelled => ("document.open.cancelled", None),
        ShellDocumentOpenPhase::Failed(_) => ("document.open.failed", None),
    };
    let head = match opening.phase {
        ShellDocumentOpenPhase::Resolving => {
            let detail =
                ["document.open.resolving.lease", "document.open.resolving.component", "document.open.resolving.descriptor", "document.open.resolving.verified"][usize::from(opening.resolve_step.load(std::sync::atomic::Ordering::Acquire).min(3))];
            format!("{} {} ({})", shell_chrome_string(key, is_de), opening.label, shell_chrome_string(detail, is_de))
        }
        ShellDocumentOpenPhase::Instantiating if opening.observed_native_transfer_progress != 0 =>
            format!("{} {} · {}%", shell_chrome_string(key, is_de), opening.label, opening.observed_native_transfer_progress - 1),
        _ => format!("{} {}", shell_chrome_string(key, is_de), opening.label),
    };
    match (&opening.phase, step) {
        (ShellDocumentOpenPhase::Failed(reason), _) => format!("{head}: {}", semio_renderer_wgpu_shell_vocab::shell_fault_text(reason, is_de).unwrap_or(reason)),
        (_, Some(step)) => format!("{head} · {step}/{DOCUMENT_OPEN_STEPS} · {} s", ((now_ms - opening.started_at_ms).max(0.0) / 1000.0).floor() as u64),
        _ => head,
    }
}

/// 🎨️ `(border, fill, text)` for one phase, the same severity map every shell banner uses.
pub fn document_opening_tone(phase: &ShellDocumentOpenPhase, theme: &Theme) -> (Rgba, Rgba, Rgba) {
    match phase {
        ShellDocumentOpenPhase::Resolving | ShellDocumentOpenPhase::Instantiating | ShellDocumentOpenPhase::Seeding => transient_notice_tone(semio_framework::Severity::Info, theme),
        ShellDocumentOpenPhase::Cancelled => transient_notice_tone(semio_framework::Severity::Warning, theme),
        ShellDocumentOpenPhase::Failed(_) => transient_notice_tone(semio_framework::Severity::Error, theme),
    }
}

/// 📐️ The band's rect: the plugin-install band's geometry, one band height and gap further down, so
/// an open that installs its plugin shows both.
pub fn document_opening_rect(message: &str, action_label: &str, width: f32, theme: &Theme) -> Rect {
    let band = plugin_install_rect(message, action_label, width, theme);
    Rect::new(band.x, band.y + band.h + PLUGIN_INSTALL_TOP_GAP, band.w, band.h)
}

//#endregion 🚪️DocumentOpenBand
//#endregion 🧯️TransientNoticeAndAgentOverlays
/// 🫧 Closes an overlay sheet's glass-content layer, if this walk opened one — the tour card and the
/// confirm dialog both. `cursor.depth` carries `region + 1` so `0` is "never opened": these ladders
/// are stepped ACROSS host opportunities and can be abandoned at a fault, a closed session, a
/// vanished step or a popped dialog, and an unbalanced `begin_glass_content` would classify every
/// LATER overlay child as this sheet's foreground.
pub fn open_chrome_overlay_glass_content(cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, rect: Rect, style: semio_ui_theme::GlassStyle) {
    close_chrome_overlay_glass_content(cursor, overlay);
    let region = overlay.push_glass([rect.x, rect.y, rect.w, rect.h], 0.0, style);
    cursor.depth = region.saturating_add(1);
    overlay.begin_glass_content(region);
}

pub fn close_chrome_overlay_glass_content(cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList) {
    if cursor.depth == 0 {
        return;
    }
    overlay.end_glass_content();
    cursor.depth = 0;
}

//#region 🧱️StatusStacks
/// 🆔️ The execution-target stack's cancel — `cancel_document_opening`, as the band it replaces had.
pub const STATUS_STACK_EXECUTION_TARGET_CANCEL_CONTROL_ID: &str = "shell.status.execution-target.cancel";

/// 🆔️ The bootstrap stack's cancel — closes the document whose restore is in flight, as React's `closeDocument`.
pub const STATUS_STACK_BOOTSTRAP_CANCEL_CONTROL_ID: &str = "shell.status.bootstrap.cancel";

/// 🆔️ The route-admission stack's sign-in button, which opens the hub workspace.
pub const STATUS_STACK_ROUTE_ADMISSION_ACTION_CONTROL_ID: &str = "shell.status.route-admission.action";

/// 🆔️ The session-authority notice's cancel, which retires the private-session handshake (`cancelSessionAuthorityBootstrap`).
pub const STATUS_STACK_SESSION_AUTHORITY_CANCEL_CONTROL_ID: &str = "shell.status.session-authority.cancel";

/// 🆔️ The prefix of the artifact-creation stack's action ids; the hub action name follows.
pub const STATUS_STACK_ARTIFACT_CREATION_ACTION_CONTROL_ID_PREFIX: &str = "shell.status.artifact-creation.";

/// 🧱️ The five floating status notice stacks React's `ShellHost` mounts as `NoticeStack`s, each a shared-hairline frame whose notices are cells divided by single hairlines.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusStackKind {
    Bootstrap,
    ExecutionTarget,
    ArtifactCreation,
    RouteAdmission,
    SessionAuthority,
}

impl StatusStackKind {
    pub const ALL: [Self; 5] = [Self::Bootstrap, Self::ExecutionTarget, Self::ArtifactCreation, Self::RouteAdmission, Self::SessionAuthority];
}

/// 🖱️ The one button of a status cell: the hit id it registers, its label and, when the press is a framework action, the action it dispatches (otherwise [`ChromeViewCtx::apply_status_action`] answers the id).
#[derive(Clone)]
pub struct StatusAction {
    pub control_id: String,
    pub label: String,
    pub event: Option<ActionDescriptor>,
}

/// 🧱️ One notice of a stack: paragraphs split on `\n` and at most one action.
#[derive(Clone)]
pub struct StatusCell {
    pub text: String,
    pub progress: Option<f32>,
    pub action: Option<StatusAction>,
}

/// 🧱️ One laid out cell: its box, its wrapped lines and its action's box.
#[derive(Clone)]
pub struct StatusCellLayout {
    pub rect: Rect,
    pub lines: Vec<String>,
    pub bar: Option<(Rect, f32)>,
    pub action: Option<(Rect, StatusAction)>,
}

/// 🧱️ One laid out stack: the frame and its cells, divided by hairlines.
#[derive(Clone)]
pub struct StatusStackLayout {
    pub kind: StatusStackKind,
    pub frame: Rect,
    pub cells: Vec<StatusCellLayout>,
}

/// 🧱️ One paint opportunity of a laid out stack.
#[derive(Clone)]
pub enum StatusOp {
    Fill(Rect, Rgba),
    Text { text: String, x: f32, baseline: f32, max_w: f32, color: Rgba },
    Hit { rect: Rect, action: StatusAction },
}

/// 📐️ Lays one stack out at React's position and size: bootstrap `top-workbench left-1/2`, execution-target the same one `mt-double` lower, artifact-creation `top-workbench left-double max-w-[28rem]`, session-authority `top-workbench right-double max-w-[28rem]`, route-admission a horizontal row `max-w-[90vw]` centred above the footer band. A stack is a column of cells, or a row for route-admission, shrink-wrapped to its widest copy.
pub fn status_stack_layout(kind: StatusStackKind, cells: &[StatusCell], viewport: (f32, f32), theme: &Theme) -> Option<StatusStackLayout> {
    if cells.is_empty() {
        return None;
    }
    let constants = semio_renderer_wgpu_shell_chrome_state::chrome_parity_constants();
    let (width, height) = viewport;
    let hair = theme.stroke_hairline;
    let glyph_w = theme.font_size_small * 0.6;
    let line_h = theme.font_size_small * 1.6;
    let pad_x = theme.padding_standard * 2.0;
    let pad_y = theme.padding_standard;
    let bar_h = line_h * 0.5;
    let top = theme.navbar_height + TRANSIENT_NOTICE_TOP_GAP;
    let left_double = pad_x;
    let max_w = match kind {
        StatusStackKind::Bootstrap | StatusStackKind::ExecutionTarget => width * constants.toast.max_width_fraction,
        StatusStackKind::ArtifactCreation | StatusStackKind::SessionAuthority => (constants.status_stacks.artifact_creation_max_width_rem * constants.status_stacks.rem_px).min((width - left_double).max(1.0)),
        StatusStackKind::RouteAdmission => width * constants.status_stacks.route_admission_max_width_fraction,
    };
    let widest_chars = |cell: &StatusCell| cell.text.split('\n').map(|line| line.chars().count()).chain(cell.action.iter().map(|action| action.label.chars().count())).max().unwrap_or(0);
    let horizontal = kind == StatusStackKind::RouteAdmission;
    let cell_content_w = |cell: &StatusCell| widest_chars(cell) as f32 * glyph_w;
    let inner_w = (if horizontal {
        max_w - hair * 2.0 - hair * cells.len().saturating_sub(1) as f32 - pad_x * 2.0 * cells.len() as f32
    } else {
        (cells.iter().map(cell_content_w).fold(0.0_f32, f32::max) + pad_x * 2.0).min(max_w) - pad_x * 2.0 - hair * 2.0
    })
    .max(glyph_w);
    let mut rows: Vec<(Vec<String>, f32, f32)> = Vec::new();
    let mut budget = inner_w;
    for (index, cell) in cells.iter().enumerate() {
        let reserve = if horizontal { glyph_w * (cells.len() - index - 1) as f32 } else { 0.0 };
        let own_w = cell_content_w(cell).min((budget - reserve).max(glyph_w));
        let lines = wrap_status_text(&cell.text, ((if horizontal { own_w } else { inner_w }) / glyph_w).floor().max(1.0) as usize);
        let action_h = if cell.action.is_some() && !horizontal { line_h } else { 0.0 } + if cell.progress.is_some() { bar_h } else { 0.0 };
        let content_w = if horizontal { own_w } else { inner_w };
        if horizontal {
            budget = (budget - own_w).max(glyph_w);
        }
        rows.push((lines.clone(), content_w + pad_x * 2.0, lines.len() as f32 * line_h + action_h + pad_y * 2.0));
    }
    let frame_w = if horizontal { rows.iter().map(|row| row.1).sum::<f32>() + hair * (cells.len() + 1) as f32 } else { inner_w + pad_x * 2.0 + hair * 2.0 };
    let frame_h = if horizontal { rows.iter().map(|row| row.2).fold(0.0_f32, f32::max) + hair * 2.0 } else { rows.iter().map(|row| row.2).sum::<f32>() + hair * (cells.len() + 1) as f32 };
    let (x, y) = match kind {
        StatusStackKind::Bootstrap => ((width - frame_w) * 0.5, top),
        StatusStackKind::ExecutionTarget => ((width - frame_w) * 0.5, top + pad_x),
        StatusStackKind::ArtifactCreation => (left_double, top),
        StatusStackKind::RouteAdmission => ((width - frame_w) * 0.5, height - theme.footer_height - frame_h),
        StatusStackKind::SessionAuthority => (width - frame_w - left_double, top),
    };
    let frame = Rect::new(x.max(0.0), y.max(0.0), frame_w, frame_h);
    let mut cursor_x = frame.x + hair;
    let mut cursor_y = frame.y + hair;
    let mut laid = Vec::new();
    for (cell, (lines, cell_w, cell_h)) in cells.iter().zip(rows) {
        let rect = if horizontal { Rect::new(cursor_x, cursor_y, cell_w, frame_h - hair * 2.0) } else { Rect::new(cursor_x, cursor_y, frame_w - hair * 2.0, cell_h) };
        let bar = cell.progress.map(|fraction| (Rect::new(rect.x + pad_x, rect.y + pad_y + lines.len() as f32 * line_h + bar_h * 0.25, (rect.w - pad_x * 2.0).max(1.0), bar_h * 0.5), fraction.clamp(0.0, 1.0)));
        let bar_offset = if bar.is_some() { bar_h } else { 0.0 };
        let action = cell.action.as_ref().map(|action| {
            let label_w = action.label.chars().count() as f32 * glyph_w;
            let action_rect = if horizontal { Rect::new(rect.x + pad_x, rect.y + pad_y, label_w.min(rect.w - pad_x * 2.0).max(1.0), line_h) } else { Rect::new(rect.x + pad_x, rect.y + pad_y + lines.len() as f32 * line_h + bar_offset, label_w.min(rect.w - pad_x * 2.0).max(1.0), line_h) };
            (action_rect, action.clone())
        });
        if horizontal {
            cursor_x += cell_w + hair;
        } else {
            cursor_y += cell_h + hair;
        }
        laid.push(StatusCellLayout { rect, lines, bar, action });
    }
    Some(StatusStackLayout { kind, frame, cells: laid })
}

/// 🎨️ The paint opportunities of one laid out stack, in paint order: the `bg-base` frame, its outer border and the single hairlines between cells, every line of copy and every action's label with its hit box.
pub fn status_stack_ops(layout: &StatusStackLayout, theme: &Theme) -> Vec<StatusOp> {
    let hair = theme.stroke_hairline;
    let frame = layout.frame;
    let horizontal = layout.kind == StatusStackKind::RouteAdmission;
    let pad_x = theme.padding_standard * 2.0;
    let line_h = theme.font_size_small * 1.6;
    let baseline = |line_top: f32| line_top + line_h * 0.5 + theme.font_size_small * 0.5 - 1.0;
    let mut ops = vec![
        StatusOp::Fill(frame, theme.input_bg),
        StatusOp::Fill(Rect::new(frame.x, frame.y, frame.w, hair), theme.border_normal),
        StatusOp::Fill(Rect::new(frame.x, frame.y + frame.h - hair, frame.w, hair), theme.border_normal),
        StatusOp::Fill(Rect::new(frame.x, frame.y, hair, frame.h), theme.border_normal),
        StatusOp::Fill(Rect::new(frame.x + frame.w - hair, frame.y, hair, frame.h), theme.border_normal),
    ];
    for (index, cell) in layout.cells.iter().enumerate() {
        if index > 0 {
            ops.push(StatusOp::Fill(if horizontal { Rect::new(cell.rect.x - hair, cell.rect.y, hair, cell.rect.h) } else { Rect::new(cell.rect.x, cell.rect.y - hair, cell.rect.w, hair) }, theme.border_normal));
        }
        for (line_index, line) in cell.lines.iter().enumerate().filter(|(_, line)| !line.is_empty()) {
            ops.push(StatusOp::Text { text: line.clone(), x: cell.rect.x + pad_x, baseline: baseline(cell.rect.y + theme.padding_standard + line_index as f32 * line_h), max_w: (cell.rect.w - pad_x * 2.0).max(1.0), color: theme.text });
        }
        if let Some((track, fraction)) = cell.bar {
            ops.push(StatusOp::Fill(track, theme.border_normal));
            ops.push(StatusOp::Fill(Rect::new(track.x, track.y, track.w * fraction, track.h), theme.accent));
        }
        if let Some((rect, action)) = &cell.action {
            ops.push(StatusOp::Text { text: action.label.clone(), x: rect.x, baseline: baseline(rect.y), max_w: rect.w, color: theme.text });
            ops.push(StatusOp::Hit { rect: *rect, action: action.clone() });
        }
    }
    ops
}

/// 🚪️ Where the shell's hub identity stands — `ShellIdentityResolutionV1` (`🏛️ShellHost/🔀️surface-switch`), rows of `🧫️fixtures/🧭️session-lane` `identities`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellIdentityResolution {
    LocalOnly,
    SignedOut,
    Pending,
    SignedIn,
    HubUnavailable,
}

/// 🚪️ Port of `shellIdentityResolutionV1`.
pub fn shell_identity_resolution(hub_configured: bool, session_held: bool, refused: bool, confirmed: bool, offline: bool) -> ShellIdentityResolution {
    if !hub_configured {
        ShellIdentityResolution::LocalOnly
    } else if !session_held || refused {
        ShellIdentityResolution::SignedOut
    } else if confirmed {
        ShellIdentityResolution::SignedIn
    } else if offline {
        ShellIdentityResolution::HubUnavailable
    } else {
        ShellIdentityResolution::Pending
    }
}

/// 🚪️ What a space route does while the hub identity resolves — `ShellRouteAdmissionV1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellRouteAdmission {
    Apply,
    ApplyOffline,
    AwaitIdentity,
    AwaitSignIn,
}

/// 🚪️ Port of `shellRouteAdmissionV1`: only a `/spaces/<id>` route waits, and only while a hub is configured.
pub fn shell_route_admission(route: &str, identity: ShellIdentityResolution) -> ShellRouteAdmission {
    let is_space_route = route.strip_prefix("/spaces/").is_some_and(|rest| rest.split(['/', '?', '#']).next().is_some_and(|id| !id.is_empty()));
    if !is_space_route || matches!(identity, ShellIdentityResolution::LocalOnly | ShellIdentityResolution::SignedIn) {
        return ShellRouteAdmission::Apply;
    }
    match identity {
        ShellIdentityResolution::HubUnavailable => ShellRouteAdmission::ApplyOffline,
        ShellIdentityResolution::Pending => ShellRouteAdmission::AwaitIdentity,
        _ => ShellRouteAdmission::AwaitSignIn,
    }
}
//#endregion 🧱️StatusStacks
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

