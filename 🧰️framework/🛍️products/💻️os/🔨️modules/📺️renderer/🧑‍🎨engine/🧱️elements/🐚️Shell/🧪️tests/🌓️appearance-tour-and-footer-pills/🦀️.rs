//! 🌓️ Packet W4a's three standing laws: the appearance-resolution precedence, the boot tour's
//! mounting condition, and the footer's sync/presence pills on BOTH targets.
//!
//! Each one pins a defect that made the wgpu shell's boot differ from React's on the same host:
//! a frame Worker with no `window` resolved DARK where React resolved LIGHT; `start_introduction`
//! was `#[cfg(test)]`, so no boot ever painted a tour; and the whole footer sync phase was
//! `cfg(not(wasm32))`, so the browser build painted neither `Remote: detached` nor the presence pill.
//!
//! Two of the laws read SOURCE rather than running code, for the reason `🧭️boot-axis-parity/🦀️.rs`
//! does: what they assert is that a browser-only code path EXISTS, and `cfg(target_arch = "wasm32")`
//! never compiles into a native test binary, so running the native build can prove nothing about it.

use super::*;

/// 🌳️ `…/🧑‍🎨engine`, the root the source-reading laws resolve their files against — the same
/// derivation `🧪️tests/🧭️boot-axis-parity/🦀️.rs` uses.
fn engine_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..").canonicalize().expect("engine root")
}

fn wgpu_shell_source() -> String {
    let path = engine_root().join("🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs");
    std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn preferences_with_appearance(appearance: Option<OsUiAppearance>) -> UiPreferences {
    UiPreferences { appearance, ..UiPreferences::default() }
}

fn lock_appearance(locked: &str) {
    let mut descriptor = crate::boot_descriptor();
    descriptor.locks.appearance = locked.to_string();
    crate::apply_boot_descriptor(descriptor).expect("appearance lock is a bounded field");
}

//#region 🌓️Appearance

/// 🧪️ The appearance-resolution law, term for term with React's two functions:
/// `🏛️ShellHost/🟦️.tsx:2174`'s `locks.appearance ?? resolveUiPreferences(preferences, { appearance:
/// "system", … }).appearance` decides the ID, and `resolveElementsSurfaceChromeDark`
/// (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:1691`) decides whether that id is dark — `"dark"` yes, `"light"`
/// no, anything else asks the host, and a realm with no host reads LIGHT.
///
/// 🩸️ The last row is the packet's defect: `prefers_dark_scheme` used to `unwrap_or(true)` in a
/// realm with no `window`, which is every frame Worker, so `"system"` meant DARK in the browser and
/// DARK unconditionally natively while React read the machine's real preference.
#[test]
fn appearance_resolves_lock_then_store_then_host_then_system() {
    lock_appearance("");
    crate::set_host_appearance(crate::HostAppearance::default());

    assert_eq!(resolve_appearance_id(&preferences_with_appearance(None)), "system", "no lock, no stored event, no host read is React's own `\"system\"` seed");
    assert_eq!(resolve_appearance_id(&preferences_with_appearance(Some(OsUiAppearance::Light))), "light");
    assert_eq!(resolve_appearance_id(&preferences_with_appearance(Some(OsUiAppearance::Dark))), "dark");

    crate::set_host_appearance(crate::HostAppearance { preference: crate::HostAppearancePreference::Dark, system_dark: false });
    assert_eq!(resolve_appearance_id(&preferences_with_appearance(None)), "dark", "the host's read of the SAME store answers where this realm's own store cannot");
    assert_eq!(resolve_appearance_id(&preferences_with_appearance(Some(OsUiAppearance::Light))), "light", "the host read is a fallback, never an override");

    lock_appearance("light");
    assert_eq!(resolve_appearance_id(&preferences_with_appearance(Some(OsUiAppearance::Dark))), "light", "a lock outranks every store");
    lock_appearance("");

    for (system_dark, system_is_dark) in [(false, false), (true, true)] {
        crate::set_host_appearance(crate::HostAppearance { preference: crate::HostAppearancePreference::Unset, system_dark });
        assert!(!crate::appearance_is_dark("light"));
        assert!(crate::appearance_is_dark("dark"));
        assert_eq!(crate::appearance_is_dark("system"), system_is_dark);
        assert_eq!(crate::appearance_is_dark(""), system_is_dark, "an unspelled appearance follows `system`, exactly as React's `else` arm does");
        assert_eq!(crate::resolve_theme("system").background, if system_is_dark { Theme::dark().background } else { Theme::light().background });
    }
    crate::set_host_appearance(crate::HostAppearance::default());
}

/// 🧪️ The default is LIGHT, not dark — `resolveElementsSurfaceChromeDark`'s `typeof window ===
/// "undefined"` arm returns `false`. A renderer that is booted and never told anything therefore
/// agrees with React on a host whose preference nobody could read, instead of disagreeing with it.
#[test]
fn an_unpublished_host_appearance_reads_light() {
    crate::set_host_appearance(crate::HostAppearance::default());
    assert!(!crate::appearance_is_dark("system"));
    assert_eq!(crate::host_appearance_preference(), None);
    assert_eq!(crate::HostAppearancePreference::from_id("nonsense"), crate::HostAppearancePreference::Unset);
    for id in ["system", "light", "dark"] {
        assert_eq!(crate::HostAppearancePreference::from_id(id).as_id(), id, "the wire spelling round-trips");
    }
}

/// 🧪️ The browser half of the appearance door has to EXIST — `cfg(target_arch = "wasm32")` code is
/// invisible to this binary, so the law reads the source: one `#[wasm_bindgen]` hook, fed by the
/// frame Worker at boot and by a live host message, and no `match_media` read left in the renderer
/// (the realm that runs it has no `window` to ask).
#[test]
fn the_browser_appearance_door_is_wired_end_to_end() {
    let renderer = std::fs::read_to_string(engine_root().join("🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")).expect("renderer source");
    assert!(renderer.contains("js_name = semioWgpuSetHostAppearance"), "the renderer exports the appearance hook");
    let code_asks_the_window = renderer.lines().any(|line| line.contains("match_media") && !line.trim_start().starts_with("//"));
    assert!(!code_asks_the_window, "the renderer no longer asks a `window` it does not have — the phrase survives only in the docstring that explains why");
    let worker = std::fs::read_to_string(engine_root().join("🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts")).expect("frame worker source");
    assert!(worker.contains("semioWgpuSetHostAppearance?.(message.appearance.preference, message.appearance.systemDark)"), "the frame Worker forwards the boot value AND live changes");
    assert!(worker.contains("host-appearance"), "the live message kind is handled");
    let page = std::fs::read_to_string(engine_root().join("🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts")).expect("browser boot source");
    for needle in ["resolveWgpuHostAppearance(window)", "WGPU_PREFERS_DARK_MEDIA_QUERY", "\"storage\", republishAppearance"] {
        assert!(page.contains(needle), "the page thread makes the reads and keeps them live: {needle}");
    }
    let descriptor = std::fs::read_to_string(engine_root().join("🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts")).expect("boot descriptor source");
    assert!(descriptor.contains("os.config.ui-preferences"), "the page replays React's own persisted event log, not a private key");
}
//#endregion 🌓️Appearance

//#region 🎓️BootTour

pub(crate) fn tour_app(introduction: Option<semio_framework::IntroductionDefinition>) -> AppDefinition {
    AppDefinition {
        id: "tour-app".into(),
        role: semio_framework::manifest::AppRole::Editor,
        dialect: semio_framework::ArtifactDialect { artifact_kind: "s.test.tour".into(), standard: "1".into(), subset: "*".into() },
        label: LocalizedLabel::data("Tour App"),
        breadcrumb: vec!["semio".into(), "tour".into()],
        icon_id: None,
        controller_id: "tour".into(),
        modes: semio_framework::Modes::one(semio_framework::ModeDefinition { id: "default".into(), label: LocalizedLabel::data("Default"), icon_id: "pencil".into(), tools: vec![], layout_id: None, commands: vec![] }),
        default_mode_id: "default".into(),
        window_kinds: semio_framework::WindowKinds::try_from(vec![semio_framework::WindowKindDefinition {
            id: "main".into(),
            label: LocalizedLabel::data("Main"),
            body_key: "main.body".into(),
            surface_kind: ui_wgpu::wgpu::SurfaceKind::Canvas2d,
            icon_id: "app-window".into(),
            options: Default::default(),
            actions: vec![],
            utilities: vec![],
            interactions: vec![],
            params_schema: None,
            artifact_snapshot_schema: None,
            input_event_schema: None,
            output_schema: None,
            capabilities: vec![],
        }])
        .expect("non-empty"),
        panel_tabs: vec![],
        keybindings: vec![],
        actions: vec![],
        utilities: vec![],
        tools: vec![],
        commands: vec![],
        interactions: vec![],
        named_layouts: vec![],
        default_layout: None,
        terminologies: vec!["de".into()],
        terminology_breadcrumbs: HashMap::new(),
        introduction,
        tutorials: Vec::new(),
        dialogs: Vec::new(),
        media_inputs: Vec::new(),
        media_outputs: Vec::new(),
        artifact_kinds: Vec::new(),
        config: semio_framework_async::block_on(semio_framework::ConfigSpec::empty()),
        command_grammar: semio_framework_async::block_on(semio_framework::CommandGrammar::empty()),
        io: semio_framework::AppIo::default(),
    }
}

pub(crate) fn tour_introduction() -> semio_framework::IntroductionDefinition {
    semio_framework::IntroductionDefinition {
        title: LocalizedLabel::data("Welcome"),
        steps: vec![
            semio_framework::IntroductionStepDefinition::new("one", LocalizedLabel::data("One"), LocalizedLabel::data("First")),
            semio_framework::IntroductionStepDefinition::new("two", LocalizedLabel::data("Two"), LocalizedLabel::data("Second")),
        ],
    }
}

pub(crate) fn tour_shell(introduction: Option<semio_framework::IntroductionDefinition>) -> ShellState {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.session = Some(ActiveSession { plugin_id: "tour".into(), instance_id: 1, app: tour_app(introduction), view_state: ViewModel::default() });
    shell
}

#[test]
fn tutorial_dialog_restoration_uses_declared_dialog_construction() {
    let mut shell = tour_shell(None);
    shell.session.as_mut().unwrap().app.dialogs.push(semio_framework::DialogDefinition::new("confirm.reset", LocalizedLabel::native("Reset?", "Zurücksetzen?"), semio_framework::ActionRef::new("resetTheme")));
    let snapshot = semio_framework::TutorialUiSnapshot { open_dialog_id: Some("confirm.reset".into()), ..Default::default() };
    tutorial_apply_ui_snapshot(&mut shell, &snapshot);
    let restored = shell.chrome_build.dialog_stack.last().expect("known dialog is restored");
    assert_eq!(restored.id, "confirm.reset");
    assert_eq!(restored.confirm_action.action, "resetTheme");
    tutorial_apply_ui_snapshot(&mut shell, &semio_framework::TutorialUiSnapshot::default());
    assert!(shell.chrome_build.dialog_stack.is_empty(), "an absent dialog closes the restored request");
    shell.queue_host_effects("tour-controller", vec![semio_framework::kernel::Effect::OpenDialog { req: semio_framework::kernel::RequestId(1), dialog_id: "confirm.reset".into(), args: None }]);
    assert_eq!(shell.chrome_build.dialog_stack.last().map(|dialog| dialog.id.as_str()), Some("confirm.reset"), "normal effects and tutorial restoration share the constructor");
}

/// 🧪️ The mounting condition, mirroring `🧱️elements/🐚️Shell/🟦️.tsx`'s `shouldAutoStartIntroduction`:
/// a fresh profile arms the tour once, a seen flag does not, an app with no introduction never does,
/// and a running tutorial owns the surface instead (Design Decision 8).
#[test]
fn a_fresh_profile_arms_the_app_introduction_exactly_once() {
    assert!(should_auto_start_introduction("tour-app", true, false, false, false));
    assert!(!should_auto_start_introduction("tour-app", true, false, true, false), "a device that has seen it is not offered it again");
    assert!(!should_auto_start_introduction("tour-app", false, false, false, false), "an app with no introduction has nothing to show");
    assert!(!should_auto_start_introduction("tour-app", true, true, false, false), "a tutorial and an introduction are mutually exclusive");
    assert!(!should_auto_start_introduction("", true, false, false, false), "no app id is no session");
    // 🏷️ React's `replayIntroductionOnLoad` brand flag (packet W15f): the seen flag is ignored, but the
    // other three terms still hold — a replaying brand does not fight a running tutorial either.
    assert!(should_auto_start_introduction("brand:tour-app", true, false, true, true), "a replaying brand plays its tour on a device that already saw it");
    assert!(!should_auto_start_introduction("brand:tour-app", true, true, false, true), "and still yields to a running tutorial");

    let mut shell = tour_shell(Some(tour_introduction()));
    assert!(shell.chrome_build.tour_state.is_none());
    shell.auto_start_introduction("tour-app", false);
    assert_eq!(shell.chrome_build.tour_state.as_ref().map(|tour| tour.step_index), Some(0), "a fresh profile boots into step 1");

    let mut seen = tour_shell(Some(tour_introduction()));
    seen.auto_start_introduction("tour-app", true);
    assert!(seen.chrome_build.tour_state.is_none());

    let mut bare = tour_shell(None);
    bare.auto_start_introduction("tour-app", false);
    assert!(bare.chrome_build.tour_state.is_none());
}

/// 🧪️ Answering the tour ends it for this device AND for this process: `dismiss_introduction` is
/// React's `dismissIntroduction`, which persists the seen flag on Skip exactly as it does on Done.
/// The in-memory flag is what stops the auto-start re-arming, because the storage read that feeds it
/// is only ever requested while the flag is absent.
#[test]
fn dismissing_the_tour_records_the_answer_and_stops_the_read_that_would_re_arm_it() {
    let mut shell = tour_shell(Some(tour_introduction()));
    shell.auto_start_introduction("tour-app", false);
    shell.chrome_build.dismiss_introduction("tour-app");
    assert!(shell.chrome_build.tour_state.is_none(), "Skip and Done both end the tour");
    assert!(shell.chrome_build.introduction_was_seen("tour-app"));
    assert_eq!(shell.chrome_build.introduction_seen_writes, vec!["tour-app".to_string()], "the device-local flag is queued for the store");

    shell.request_introduction_read();
    assert_eq!(shell.chrome_present.maintenance.introduction_read, None, "an answered app is never re-read, so it is never re-armed");

    shell.auto_start_introduction("tour-app", true);
    assert!(shell.chrome_build.tour_state.is_none());
}

/// 🧪️ The wiring itself — the defect was a correct tour that nothing started. `start_introduction`
/// must be production code, and the one place the seen flag becomes known must call the arming law.
#[test]
fn the_introduction_read_arms_the_tour() {
    let source = wgpu_shell_source();
    assert!(!source.contains("#[cfg(test)]\n    fn start_introduction"), "`start_introduction` is production code, not a test-only helper");
    let arm = source.split("introduction_read.take()").nth(1).expect("the introduction-seen read arm exists");
    let arm = &arm[..arm.find("introduction_write").unwrap_or(arm.len())];
    assert!(arm.contains("self.auto_start_introduction(&seen_key, seen)"), "the landing read arms the tour");
}

/// 🧪️ The card's own chrome, against `UIIntroduction` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`): Skip in the
/// cap row, and a footer carrying Back (from step 2 on), the `i / n` counter and Next/Done. The
/// counter and Back had no twin on this side at all.
#[test]
fn the_tour_card_carries_reacts_own_controls() {
    let source = wgpu_shell_source();
    let card = source.split("fn render_chrome_tour_step").nth(1).expect("the tour card renderer exists");
    let card = &card[..card.find("//#region SilhouetteContent").unwrap_or(card.len())];
    let layout = source.split("fn chrome_tour_layout").nth(1).expect("the tour card measurer exists");
    let layout = &layout[..layout.find("//#endregion").unwrap_or(layout.len())];
    for needle in ["introduction.skip", "introduction.back", "introduction.next", "introduction.done", "let counter_text = format!(\"{} / {step_count}\", step_index + 1)"] {
        assert!(layout.contains(needle), "the card paints {needle}");
    }
    for needle in ["UI_INTRODUCTION_SKIP_CONTROL_ID", "UI_INTRODUCTION_NEXT_CONTROL_ID", "UI_INTRODUCTION_BACK_CONTROL_ID"] {
        assert!(card.contains(needle), "the card registers a hit for {needle}");
    }
}
//#endregion 🎓️BootTour

//#region 🚦️FooterPills

/// 🧪️ `syncPillText`'s truth table (`🛠️ShellHelpers/🟦️.tsx`), both locales — this side used to be
/// English-only, so a `de` shell read `Remote: detached` where React reads `Remote: getrennt`.
#[test]
fn the_sync_pill_speaks_reacts_own_vocabulary() {
    assert_eq!(shell_sync_pill_text(ShellSyncPill::Persisted, false), "Persisted");
    assert_eq!(shell_sync_pill_text(ShellSyncPill::Persisted, true), "Gespeichert");
    assert_eq!(shell_sync_pill_text(ShellSyncPill::Pending(3), false), "Pending (3)");
    assert_eq!(shell_sync_pill_text(ShellSyncPill::Remote(ShellSyncRemote::Connected), false), "Remote: connected");
    assert_eq!(shell_sync_pill_text(ShellSyncPill::Remote(ShellSyncRemote::Connecting), false), "Remote: connecting");
    assert_eq!(shell_sync_pill_text(ShellSyncPill::Remote(ShellSyncRemote::Connecting), true), "Remote: verbindet");
    assert_eq!(shell_sync_pill_text(ShellSyncPill::Remote(ShellSyncRemote::Backoff), false), "Remote: backoff");
    assert_eq!(shell_sync_pill_text(ShellSyncPill::Remote(ShellSyncRemote::Detached), false), "Remote: detached");
    assert_eq!(shell_sync_pill_text(ShellSyncPill::Remote(ShellSyncRemote::Detached), true), "Remote: getrennt");
}

/// 🧪️ `computeSyncPillState(null)` is `{ remote: "detached" }`, and a shell with no backbone
/// attached is exactly that `null` — on both targets. React's browser footer shows the pill for the
/// same reason.
#[test]
fn a_shell_with_no_backbone_reads_remote_detached() {
    let shell = ShellState::new(Vec::new(), String::new());
    assert_eq!(shell.sync_pill(), ShellSyncPill::Remote(ShellSyncRemote::Detached));
    assert_eq!(shell_sync_pill_text(shell.sync_pill(), false), "Remote: detached");
    assert!(shell.footer_presence_rows().is_empty());
}

/// 🧪️ The presence pill's empty state is the ELEMENT's own copy, so the chip and the `UiNode` roster
/// can never drift — React's footer keeps `#s-presence-peers` mounted with `No one else is here`
/// rather than hiding it (`🏛️ShellHost/🟦️.tsx`'s `footerItems`).
#[test]
fn the_presence_pill_shares_the_elements_own_copy() {
    assert_eq!(ui_wgpu::wgpu::presence_bar_chip_text(&[], None, Locale::En), "No one else is here");
    assert_eq!(ui_wgpu::wgpu::presence_bar_chip_text(&[], None, Locale::De), "Niemand sonst ist hier");
    let peers: Vec<ui_wgpu::wgpu::PresencePeerRow> = (0..7).map(|index| ui_wgpu::wgpu::PresencePeerRow { actor: format!("a{index}"), user_id: None, label: format!("Peer {index}"), role: None, connected_at_ms: None, color: None, is_agent: false }).collect();
    let chip = ui_wgpu::wgpu::presence_bar_chip_text(&peers, None, Locale::En);
    assert!(chip.starts_with("Peer 0 · "), "visible peers keep their order");
    assert!(chip.ends_with("+2 more"), "the same cap and overflow suffix the roster uses");
}

/// 🧪️ Both pills reach the BROWSER build. `cfg(target_arch = "wasm32")` is invisible to this binary,
/// so the law reads the source: nothing between the footer phase and the pill renderer may be gated
/// on a native target any more, and the wasm arm of `sync_pill` must answer `Remote(Detached)`.
///
/// 🩸️ The renderer was `render_sync_status_and_checkin` until packet W6a: React paints NO footer
/// check-in chip (`#s-checkin` is a row of the History panel's own tree, gated by
/// `canCheckIn(session.app.role)`), so this law now pins the two pills that DO belong there and pins
/// the third one's absence — the footer's x-order itself is
/// `🧭️wgpu-navbar-footer-parity/🦀️.rs`'s own law.
#[test]
fn the_footer_pills_are_not_gated_off_the_browser_build() {
    let source = wgpu_shell_source();
    let dock = source.split("pub fn default_dock(&self)").nth(1).expect("the default dock exists");
    let dock = &dock[..dock.find("\n    pub ").unwrap_or(dock.len())];
    assert!(dock.contains("FRAMEWORK_SYNC_PANEL_TAB_ID"), "the footer's bottom-left sync leaf is always declared");
    assert!(!dock.contains("cfg(not(target_arch"), "the footer's sync leaf is no longer native-only");
    let pill = source.split("fn sync_pill(&self)").nth(1).expect("the pill projection exists");
    let pill = &pill[..pill.find("\n    /// ").unwrap_or(pill.len())];
    assert!(!pill.contains("cfg(target_arch") && !pill.contains("cfg(not(target_arch"), "one pill body on both builds: the browser document actor reports its remote exactly as the native one");
    assert!(pill.contains("let Some(status) = self.sync_status.as_ref() else { return ShellSyncPill::Remote(ShellSyncRemote::Detached) };"), "no status observed yet resolves the state React's footer resolves with no backbone");
    let renderer = source.split("fn render_footer_step").nth(1).expect("the footer renderer exists");
    let renderer = &renderer[..renderer.find("\n    fn render_overlay_step").unwrap_or(renderer.len())];
    assert!(renderer.contains("\"s-presence-peers\""), "the footer paints the ambient presence badge");
    assert!(!renderer.contains("\"s-checkin\""), "and paints no check-in chip, because React's footer has none");
}
//#endregion 🚦️FooterPills
