//! 🏛️ Host statics, boot descriptor, logging, plugin catalog, runtime assets, mesh assets, frame latency and renderer I/O of the wgpu renderer.
//!
//! 🧭️ Native and wasm32. The dissolved root of the renderer crate (host appearance, platform and locale, boot descriptor, logging, application task spawning) plus the leaf modules every other crate of the partition stands on.

#[cfg(target_arch = "wasm32")]
use semio_framework_async::browser::spawn_local;
use semio_framework_job::RetainedCloneGrant;
use std::cell::RefCell;
use std::future::Future;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use ui_wgpu::wgpu::Theme;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

//#region 🔖️ActorBudgetBridge
/// ⚖️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-kernel-loop, unblocking terra-shard-grants'
/// `ShardFrame::Grant` wire change): `semio_framework::kernel::Budget` (what this crate's own
/// `kernel_runtime::TURN_BUDGET`/`scale_bench::turn_budget_of` already speak) →
/// `semio_framework_actor::Budget` (what a `Grant` frame carries over `ShardTransport`, replacing
/// the deleted `ShardLoop::pump(|actor| ..)` budget closure). Shared by both native call sites
/// (`kernel_runtime::run_turn`, `scale_bench::Env::send_payload`) rather than duplicated — CLAUDE.md's
/// "if code is repeated, it MUST be close to each other" — so it lives here, the one file both
/// `#[cfg(not(target_arch = "wasm32"))]` modules already share. `memory_bytes`/`ui_nodes`/
/// `mailbox_len` have no source field on the kernel-`Budget` side; defaulted from `lane` via
/// `lane_defaults::budget_for` rather than invented — the same documented-gap shape
/// `🖥️host/🧵️shard/🦀️.rs`'s own `BudgetBridge` region already uses for the REVERSE
/// direction (`GRANT_BUDGET_DEFAULT_MAX_FRAMES`).
#[cfg(not(target_arch = "wasm32"))]
pub fn actor_budget_from_turn_budget(budget: semio_framework::kernel::Budget, original: semio_framework_plugin_host::shard::grant::ShardResourceBudget) -> semio_framework_plugin_host::shard::grant::ShardResourceBudget {
    semio_framework_plugin_host::shard::grant::ShardResourceBudget { fuel: budget.fuel, wall_ms: budget.deadline_ms, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes, max_frames: budget.max_frames, ..original }
}
//#endregion 🔖️ActorBudgetBridge

/// 🧑‍🏭️ Advances bounded native I/O turns on application and kernel workers before presentation exists.
#[cfg(not(target_arch = "wasm32"))]
pub async fn drive_renderer_worker_task<F: Future>(future: F) -> F::Output {
    let mut future = Box::pin(future);
    std::future::poll_fn(|cx| {
        let result = future.as_mut().poll(cx);
        let advanced = pump_renderer_io_sessions(16);
        if result.is_pending() && advanced != 0 {
            cx.waker().wake_by_ref();
        }
        result
    })
    .await
}

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_app_task<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    let _ = KernelPoolFuture::spawn(renderer_worker_pool(), semio_framework_async::Lane::Interactive, future);
}

#[cfg(target_arch = "wasm32")]
pub fn spawn_app_task<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    spawn_local(future);
}

/// 🩺️ Temporary transition-only tracing so a per-frame predicate can be observed without 60 lines a
/// second. Remove with the `[TRACE]` lines it serves.
pub fn log_debug_diagnostic_once_per_transition(site: &'static str, state: bool, message: &str) {
    if semio_framework_trace::runtime_diagnostics_enabled() {
        log_debug_once_per_transition(site, state, message);
    }
}

pub fn log_debug_once_per_transition(site: &'static str, state: bool, message: &str) {
    use std::sync::Mutex;
    static LAST: Mutex<Option<Vec<(&'static str, bool)>>> = Mutex::new(None);
    let mut guard = LAST.lock().expect("debug transition ledger");
    let entries = guard.get_or_insert_with(Vec::new);
    match entries.iter_mut().find(|(key, _)| *key == site) {
        Some(entry) if entry.1 == state => return,
        Some(entry) => entry.1 = state,
        None => entries.push((site, state)),
    }
    drop(guard);
    log_debug(message);
}

/// 🩺️ A PER-FRAME `[TRACE]` dump. Printed only while runtime diagnostics are armed
/// (`SEMIO_RUNTIME_DIAGNOSTICS` — the process environment natively, the url stamp the UI isolate puts
/// on the frame Worker in a browser, `localStorage` in React's own shell), because sixty of these a
/// second per surface is not a trace, it is a mask: the boot fault this gate was added with
/// (`asset retained structure decoder rejected malformed input`) sat under ~2 000 lines of frame
/// census in the console. One-shot and transition-only sites keep using [`log_debug`] directly.
pub fn log_debug_diagnostic(message: &str) {
    if semio_framework_trace::runtime_diagnostics_enabled() {
        log_debug(message);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn log_debug(message: &str) {
    web_sys::console::log_1(&JsValue::from_str(message));
}

#[cfg(not(target_arch = "wasm32"))]
pub fn log_debug(message: &str) {
    eprintln!("{message}");
}

//#region 🌓️HostAppearance
/// 🌓️ The persisted `os.config.ui-preferences` appearance as the HOST read it — React's
/// `resolveUiPreferences(preferences, { appearance: "system", … }).appearance`, where
/// [`HostAppearancePreference::Unset`] is React's `preferences.appearance == null` (the store holds
/// no `setAppearance` event) and therefore falls through to the shell's own default.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HostAppearancePreference {
    #[default]
    Unset,
    System,
    Light,
    Dark,
}

impl HostAppearancePreference {
    /// 🌓️ The wire spelling — `""` for unset, so a host that read nothing says nothing.
    pub fn from_id(id: &str) -> Self {
        match id {
            "system" => Self::System,
            "light" => Self::Light,
            "dark" => Self::Dark,
            _ => Self::Unset,
        }
    }

    pub fn as_id(self) -> &'static str {
        match self {
            Self::Unset => "",
            Self::System => "system",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

/// 🌓️ The two appearance inputs this renderer cannot read for itself, published by whoever owns a
/// window: the persisted preference above and the host's live `prefers-color-scheme`.
///
/// 🩸️ What this replaces: a `web_sys::window().match_media("(prefers-color-scheme: dark)")` read
/// with an `unwrap_or(true)` fallback. The browser renderer runs inside the frame Worker
/// (`🎞️frame-worker/🟦️.ts`), whose realm owns NO `window` — so that read always missed and every
/// browser boot resolved DARK whatever the machine preferred, while React's own
/// `resolveElementsSurfaceChromeDark` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:1691`) resolved LIGHT on the
/// same host. `system_dark` therefore defaults to `false`, byte-for-byte that function's
/// `typeof window === "undefined"` arm, and is fed live by the page thread
/// (`🚀️browser-boot/🟦️.ts`), by the embeddable door (`🎬️renderer-boot/🟦️.ts`) and natively by
/// winit's own `Window::theme()`/`WindowEvent::ThemeChanged` (`🪟️winit-app/🦀️.rs`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HostAppearance {
    pub preference: HostAppearancePreference,
    pub system_dark: bool,
}

thread_local! {
    static HOST_APPEARANCE: std::cell::Cell<HostAppearance> = const { std::cell::Cell::new(HostAppearance { preference: HostAppearancePreference::Unset, system_dark: false }) };
}

/// 🌓️ Publishes the host's appearance inputs. This value store owns no scheduler: each live host
/// requests a frame after a change, and the next frame's `ThemeResolve` phase re-reads it.
pub fn set_host_appearance(appearance: HostAppearance) {
    HOST_APPEARANCE.with(|cell| cell.set(appearance));
}

/// 🌓️ Both published inputs, for a caller that changes one and must keep the other.
pub fn host_appearance() -> HostAppearance {
    HOST_APPEARANCE.with(|cell| cell.get())
}

/// 🌓️ The persisted preference the host read, or `None` when its store held none — the shell folds
/// this in exactly where React folds `preferences.appearance`.
pub fn host_appearance_preference() -> Option<&'static str> {
    match HOST_APPEARANCE.with(|cell| cell.get().preference) {
        HostAppearancePreference::Unset => None,
        other => Some(other.as_id()),
    }
}

pub fn prefers_dark_scheme() -> bool {
    HOST_APPEARANCE.with(|cell| cell.get().system_dark)
}
//#endregion 🌓️HostAppearance

//#region 🎓️HostIntroduction
thread_local! {
    static HOST_INTRODUCTION_SUPPRESSED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// 🎓️ Carries one host's introduction policy inside its independently owned renderer realm.
pub fn set_host_introduction_suppressed(suppressed: bool) {
    HOST_INTRODUCTION_SUPPRESSED.with(|cell| cell.set(suppressed));
}

/// 🪆️ Reads the current host policy before arming or reconciling its introduction.
pub fn host_introduction_suppressed() -> bool {
    HOST_INTRODUCTION_SUPPRESSED.with(|cell| cell.get())
}

//#endregion 🎓️HostIntroduction

//#region ⌨️HostPlatform
/// ⌨️ Byte-for-byte React's `keybindingPlatformUsesMetaV1`
/// (`🖱️ui/🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts`): the ONE rule deciding what `mod` means
/// on this machine. `userAgentData.platform` answers `"macOS"` exactly; the legacy `navigator.platform`
/// strings (`"MacIntel"`, `"iPhone"`, …) are matched case-insensitively, which is what React's
/// `/mac|iphone|ipad|ipod/i` does.
pub fn keybinding_platform_uses_meta(platform: &str) -> bool {
    let platform = platform.to_ascii_lowercase();
    ["mac", "iphone", "ipad", "ipod"].into_iter().any(|needle| platform.contains(needle))
}

/// ⌨️ The host OS as a MANIFEST platform — the axis every `PlatformKeybinding.platform` is declared
/// against. Derived from the SAME string `keybinding_platform_uses_meta` reads, so the door can never
/// answer "this is a Mac" for `mod` and "this is Linux" for a keybinding filter in the same breath.
/// React does not re-derive it per call site either: `ui.platform` is resolved once for the whole
/// shell.
pub fn keybinding_platform_kind(platform: &str) -> semio_framework::manifest::Platform {
    if keybinding_platform_uses_meta(platform) {
        return semio_framework::manifest::Platform::MacOs;
    }
    if platform.to_ascii_lowercase().contains("win") {
        return semio_framework::manifest::Platform::Windows;
    }
    semio_framework::manifest::Platform::Linux
}

/// ⌨️ The compile-time platform a NATIVE build runs on — the initializer both door cells start from.
///
/// 🌐️ `wasm32-unknown-unknown` lands here too, which is exactly the defect this door exists for:
/// the value is a PLACEHOLDER until the page publishes the real one, never an answer.
pub const fn compiled_host_platform() -> semio_framework::manifest::Platform {
    #[cfg(target_os = "macos")]
    {
        semio_framework::manifest::Platform::MacOs
    }
    #[cfg(target_os = "windows")]
    {
        semio_framework::manifest::Platform::Windows
    }
    #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
    {
        semio_framework::manifest::Platform::Linux
    }
}

thread_local! {
    static HOST_PLATFORM_USES_META: std::cell::Cell<bool> = const { std::cell::Cell::new(cfg!(target_os = "macos")) };
    static HOST_PLATFORM: std::cell::Cell<semio_framework::manifest::Platform> = const { std::cell::Cell::new(compiled_host_platform()) };
}

/// ⌨️ Publishes the host's platform reading. Natively nothing calls this: the initializer already IS
/// the OS answer (`cfg!(target_os = "macos")`), the same compile-time fact winit runs on. In the
/// browser the frame Worker realm owns no `navigator.platform` worth trusting — a dedicated Worker
/// reports the page's own user agent, but `userAgentData.platform` is page-thread-only — so the page
/// resolves it and hands it across with the boot message.
pub fn set_host_platform_uses_meta(uses_meta: bool) {
    HOST_PLATFORM_USES_META.with(|cell| cell.set(uses_meta));
}

/// ⌨️ Publishes the host's platform KIND. Always set together with [`set_host_platform_uses_meta`] by
/// [`set_host_platform`]; separate setters exist only because the `mod` rule and the manifest axis are
/// two different questions with two different answers on `iphone`/`ipad`.
pub fn set_host_platform_kind(platform: semio_framework::manifest::Platform) {
    HOST_PLATFORM.with(|cell| cell.set(platform));
}

/// ⌨️ Publishes BOTH platform readings from the one string the page resolved — the only setter any
/// door should call.
pub fn set_host_platform(platform: &str) {
    set_host_platform_uses_meta(keybinding_platform_uses_meta(platform));
    set_host_platform_kind(keybinding_platform_kind(platform));
}

/// ⌨️ Whether `mod` is Command on this host — read by every chord formatter in the shell.
pub fn host_platform_uses_meta() -> bool {
    HOST_PLATFORM_USES_META.with(std::cell::Cell::get)
}

/// ⌨️ The host platform every `PlatformKeybinding.platform` filter is answered against — read by the
/// shell's `command_host_platform()`.
pub fn host_platform() -> semio_framework::manifest::Platform {
    HOST_PLATFORM.with(std::cell::Cell::get)
}

//#endregion ⌨️HostPlatform

//#region 🗣️HostLocale
/// 🗣️ Byte-for-byte React's `normalizeUiLocale` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`): a BCP47 tag
/// (`navigator.language`, `"de-AT"`) folds onto the chrome's two tongues. `""` is "the host said
/// nothing", which leaves the door unset rather than pinning a tongue nobody chose.
pub fn host_locale_from_tag(tag: &str) -> Option<&'static str> {
    semio_framework_ui_locale::Locale::from_language_tag(tag).ok().map(|locale| locale.as_str())
}

thread_local! {
    static HOST_LOCALE: std::cell::Cell<Option<&'static str>> = const { std::cell::Cell::new(None) };
}

/// 🗣️ Publishes the host's own language read. The shell folds it in exactly where React's
/// `ShellHost` folds `detectShellLocale(navigator.language)`: after a lock and after the persisted
/// preference, never over either.
pub fn set_host_locale(tag: &str) {
    HOST_LOCALE.with(|cell| cell.set(host_locale_from_tag(tag)));
}

/// 🗣️ The host's published tongue, or `None` when no door has spoken.
pub fn host_locale() -> Option<&'static str> {
    HOST_LOCALE.with(std::cell::Cell::get)
}

//#endregion 🗣️HostLocale

/// 🌓️ Byte-for-byte React's `resolveElementsSurfaceChromeDark` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:1691`):
/// `"dark"` is dark, `"light"` is light, everything else (including `"system"`) asks the host.
pub fn resolve_theme(appearance_id: &str) -> Theme {
    match appearance_id {
        "light" => Theme::light(),
        "dark" => Theme::dark(),
        _ if prefers_dark_scheme() => Theme::dark(),
        _ => Theme::light(),
    }
}

pub fn appearance_is_dark(appearance_id: &str) -> bool {
    match appearance_id {
        "light" => false,
        "dark" => true,
        _ => prefers_dark_scheme(),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn app_now_ms() -> f64 {
    js_sys::Date::now()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn app_now_ms() -> f64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_secs_f64() * 1000.0).unwrap_or(0.0)
}

//#region 🧭️BootDescriptor
// 🧭️ ONE boot-axis vocabulary for all three wgpu entry points — the trunk page
// (`🚀️browser-boot/🟦️.ts`), the embeddable library door (`🎬️renderer-boot/🟦️.ts`) and the native CLI
// (`⌨️native-entrypoint/🦀️.rs`) — and the Rust twin of `🧭️boot-descriptor/🟦️.ts`. React has a single
// door (`FrameworkOsBootOptions`); wgpu had three, each carrying a different subset of the same axes,
// so `--plugin cad --example concrete-forest` could not open natively what
// `?plugin=cad&example=concrete-forest` opened in the browser. The standing law that keeps the four
// field sets equal is `🧪️tests/🧭️boot-axis-parity/🦀️.rs`.
//
// Still a `thread_local` a caller opts into reading rather than a parameter threaded through every
// mount signature: the shell reads it while it opens the session (`ShellState::boot`), which is why
// every door applies it BEFORE mounting.

/// 📏️ Bound on ONE descriptor field, the twin of `🧭️boot-descriptor/🟦️.ts`'s
/// `WGPU_BOOT_FIELD_CAPACITY` — the browser door refuses an oversized url before it ever reaches
/// here, and this is the same refusal for the native and library doors.
pub const WGPU_BOOT_FIELD_CAPACITY: usize = 2048;

/// 🔒️ Boot-time preference LOCKS — React's `FrameworkOsLocks`. `""` means unset, never "locked to the
/// empty string"; a locked preference loses its in-app switcher.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct WgpuBootLocks {
    pub example_id: String,
    pub locale: String,
    pub terminology: String,
    pub theme_id: String,
    pub appearance: String,
}

/// 🎛️ Boot-time preference DEFAULTS — React's `FrameworkOsDefaults`: seeds that KEEP their switcher.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct WgpuBootDefaults {
    pub example_id: String,
}

/// 🏷️ The brand REGISTRY row this boot resolved — React's `resolveShellBrandById(brandId)` answer
/// (`🧑‍💻dev/🏷️brand/🟦️.ts`), reduced to the facts the shell itself reads. The table stays where it is
/// (a TypeScript catalogue over the demonstrator's own brand definitions); what crosses into the
/// renderer is the resolved ROW, exactly as `locks`/`defaults` already do — so a brand added there
/// needs no Rust edit and the two halves cannot drift into two catalogues.
///
/// `brand_id` alone was carried before this (`📓️w1d-boot-axis-parity.md` gap 1): the shell knew it was
/// branded and nothing else, which is why the tour's persisted-seen key dropped React's brand prefix
/// on every branded playground (`📓️w5a-browser-prefs-persistence.md` §7.1, demonstrated on
/// `aggregator`).
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct WgpuBootBrand {
    /// 🏷️ React's `ShellBrand.windowTitle` — the native window's title; `""` leaves the default.
    pub window_title: String,
    /// 🧊️ React's `ShellBrand.ephemeral`: the shell never reads or writes device-local state.
    pub ephemeral: bool,
    /// 🎓️ React's `ShellBrand.replayIntroductionOnLoad` — auto-start every load, persist no seen flag.
    /// `isEphemeralShellBrand` implies it there, and [`WgpuBootBrand::replays_introduction`] implies it
    /// here, so the two predicates cannot disagree.
    pub replay_introduction_on_load: bool,
}

impl WgpuBootBrand {
    /// 🎓️ React's `shouldReplayIntroductionOnLoad(brand)`.
    pub fn replays_introduction(&self) -> bool {
        self.ephemeral || self.replay_introduction_on_load
    }

    /// 🎓️ React's `shouldPersistIntroductionSeen(brand)`.
    pub fn persists_introduction_seen(&self) -> bool {
        !self.replays_introduction()
    }
}

/// 🌐️ The hub trio (`?hub=&user=&dataDir=`, `--hub/--user/--data-dir`).
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct WgpuBootHub {
    pub hub_url: String,
    pub user: String,
    pub data_dir: String,
}

/// 🧭️ Every boot axis, resolved. Absent axes are `""`/`Default` rather than `Option`, so the three
/// doors cannot disagree about what "unset" serializes to.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct WgpuBootDescriptor {
    pub plugin_variant: String,
    pub app_id: String,
    pub app_role: String,
    pub app_mode: String,
    pub app_example: String,
    pub brand_id: String,
    pub brand: WgpuBootBrand,
    pub broker_proof: String,
    pub locks: WgpuBootLocks,
    pub defaults: WgpuBootDefaults,
    pub hub: Option<WgpuBootHub>,
}

impl WgpuBootDescriptor {
    /// 📏️ Refuses an oversized field by NAME, so the fault says which axis overflowed — the Rust half
    /// of `boundedBootField`. Applied before any axis is stored, never after.
    pub fn validated(self) -> Result<Self, String> {
        let hub = self.hub.as_ref();
        let fields: [(&str, &str); 14] = [
            ("plugin", self.plugin_variant.as_str()),
            ("app", self.app_id.as_str()),
            ("role", self.app_role.as_str()),
            ("mode", self.app_mode.as_str()),
            ("example", self.app_example.as_str()),
            ("brand", self.brand_id.as_str()),
            ("brand.windowTitle", self.brand.window_title.as_str()),
            ("brokerProof", self.broker_proof.as_str()),
            ("locks.exampleId", self.locks.example_id.as_str()),
            ("locks.locale", self.locks.locale.as_str()),
            ("locks.terminology", self.locks.terminology.as_str()),
            ("locks.themeId", self.locks.theme_id.as_str()),
            ("locks.appearance", self.locks.appearance.as_str()),
            ("defaults.exampleId", self.defaults.example_id.as_str()),
        ];
        let hub_fields: [(&str, &str); 3] = [("hub", hub.map_or("", |hub| hub.hub_url.as_str())), ("user", hub.map_or("", |hub| hub.user.as_str())), ("dataDir", hub.map_or("", |hub| hub.data_dir.as_str()))];
        for (field, value) in fields.into_iter().chain(hub_fields) {
            if value.chars().count() > WGPU_BOOT_FIELD_CAPACITY {
                return Err(format!("boot-descriptor-overflow: {field} exceeds {WGPU_BOOT_FIELD_CAPACITY} code units"));
            }
        }
        Ok(self)
    }
}

thread_local! {
    static BOOT_DESCRIPTOR: RefCell<WgpuBootDescriptor> = RefCell::new(resolve_environment_boot_descriptor());
}

/// 🖥️ The native per-server seeds, read straight off the process env under the SAME names React's
/// serve projects into `VITE_SEMIO_*` (`🧑‍💻dev/🟦️.ts`). The CLI's own per-navigation flags overwrite
/// these in `apply_boot_descriptor`; a browser build has no process env and starts empty.
///
/// 🏷️ The resolved brand ROW, under the same `SEMIO_*` names the serve injects as
/// `<meta semio-brand-*>` for the browser door — one vocabulary, three doors.
#[cfg(not(target_arch = "wasm32"))]
pub fn resolve_environment_boot_descriptor() -> WgpuBootDescriptor {
    let read = |name: &str| std::env::var(name).ok().filter(|value| !value.is_empty()).unwrap_or_default();
    WgpuBootDescriptor {
        app_id: read("SEMIO_APP_ID"),
        app_role: read("SEMIO_APP_ROLE"),
        brand_id: read("SEMIO_BRAND"),
        brand: WgpuBootBrand { window_title: read("SEMIO_BRAND_WINDOW_TITLE"), ephemeral: read("SEMIO_BRAND_EPHEMERAL") == "true", replay_introduction_on_load: read("SEMIO_BRAND_REPLAY_INTRODUCTION") == "true" },
        app_example: read("SEMIO_DEFAULT_EXAMPLE"),
        defaults: WgpuBootDefaults { example_id: read("SEMIO_DEFAULT_EXAMPLE") },
        locks: WgpuBootLocks { example_id: read("SEMIO_LOCKED_EXAMPLE"), locale: read("SEMIO_LOCKED_LOCALE"), terminology: read("SEMIO_LOCKED_TERMINOLOGY"), theme_id: read("SEMIO_LOCKED_THEME"), appearance: read("SEMIO_LOCKED_APPEARANCE") },
        ..WgpuBootDescriptor::default()
    }
}

#[cfg(target_arch = "wasm32")]
pub fn resolve_environment_boot_descriptor() -> WgpuBootDescriptor {
    WgpuBootDescriptor::default()
}

/// 🧭️ Installs the resolved axes, replacing the environment seeds this thread started from. Every
/// door calls it once, before the mount that opens the session.
pub fn apply_boot_descriptor(descriptor: WgpuBootDescriptor) -> Result<(), String> {
    let descriptor = descriptor.validated()?;
    BOOT_DESCRIPTOR.with(|cell| *cell.borrow_mut() = descriptor);
    Ok(())
}

/// 🧭️ The whole resolved descriptor, for a reader that needs more than one axis.
pub fn boot_descriptor() -> WgpuBootDescriptor {
    BOOT_DESCRIPTOR.with(|cell| cell.borrow().clone())
}

pub fn boot_axis(read: impl Fn(&WgpuBootDescriptor) -> &str) -> Option<String> {
    BOOT_DESCRIPTOR.with(|cell| {
        let descriptor = cell.borrow();
        let value = read(&descriptor);
        (!value.is_empty()).then(|| value.to_string())
    })
}

/// 📌️ The boot-pinned app id, or `None`. React's `appId`: an empty value is NOT a pin, it falls
/// through to the variant's own default app (`🧑‍💻dev/🟦️.ts`'s own note on `VITE_SEMIO_APP_ID`).
pub fn boot_app_id() -> Option<String> {
    boot_axis(|descriptor| &descriptor.app_id)
}

/// 🎭️ The boot-requested mode id, or `None` when nothing named one. A mode the open app does not
/// declare is ignored by the reader (`ShellState::apply_boot_mode`), never a boot failure.
pub fn boot_app_mode() -> Option<String> {
    boot_axis(|descriptor| &descriptor.app_mode)
}

/// 📚️ The boot-requested example id, or `None`. It already collapses `?example=`/`--example` over the
/// `SEMIO_DEFAULT_EXAMPLE` seed the way React collapses the query over `VITE_SEMIO_DEFAULT_EXAMPLE`
/// into ONE `defaults.exampleId`. An id the open DIALECT does not author is ignored by the reader
/// (`shell::resolve_boot_example_id`), never a boot failure.
pub fn boot_app_example() -> Option<String> {
    boot_axis(|descriptor| &descriptor.app_example)
}

/// 🏷️ The boot brand id, or `None` — React's `brand`, resolved there by `resolveShellBrandById`.
pub fn boot_brand_id() -> Option<String> {
    boot_axis(|descriptor| &descriptor.brand_id)
}

/// 🏷️ The resolved brand ROW — React's `resolveShellBrandById(brandId)` answer. An UNBRANDED boot
/// answers `WgpuBootBrand::default()`, whose predicates read exactly as React's do for `undefined`
/// (`shouldReplayIntroductionOnLoad(undefined) === false`), so no caller needs an `Option` arm.
pub fn boot_brand() -> WgpuBootBrand {
    BOOT_DESCRIPTOR.with(|cell| cell.borrow().brand.clone())
}

/// 🏷️ The shell window's title — React's `ShellBrand.windowTitle`, `"Semio"` when this boot resolved no brand — which the
/// native window shows and its platform accessibility root announces.
pub fn boot_window_title() -> String {
    let title = boot_brand().window_title;
    if title.is_empty() {
        "Semio".to_string()
    } else {
        title
    }
}

/// #️⃣ The one-shot `#semio-broker=` proof the page carried, or `None`. React reads the same hash at
/// `🏛️ShellHost/🟦️.tsx:209` and hands it to its browser broker client.
pub fn boot_broker_proof() -> Option<String> {
    boot_axis(|descriptor| &descriptor.broker_proof)
}

/// 🔒️ The boot preference locks — read by the shell's `env_lock`, so a browser build locks exactly as
/// a native `SEMIO_LOCKED_*` deployment does instead of being unlockable for want of a process env.
pub fn boot_locks() -> WgpuBootLocks {
    BOOT_DESCRIPTOR.with(|cell| cell.borrow().locks.clone())
}

/// 👁️✏️ The boot-resolved role, contract freeze §5 — `"viewer"`/`"editor"`, anything else is
/// `ChromeRole::from_boot_env`'s own `editor` fallback.
pub fn boot_app_role() -> ui_wgpu::wgpu::component::role_chrome::ChromeRole {
    BOOT_DESCRIPTOR.with(|cell| ui_wgpu::wgpu::component::role_chrome::ChromeRole::from_boot_env(Some(cell.borrow().app_role.as_str())))
}
//#endregion 🧭️BootDescriptor

#[path = "../⏱️frame-latency/🦀️.rs"]
pub mod frame_latency;

// 🥽️ Target-neutral: the catalog is `include_str!`-embedded, and `scenes::render_icon_render` needs
// `mesh_asset_transport_url` on the browser build too — the same public-id → transport-path rewrite
// React's `IconRenderHost` applies before handing a `/mesh/…` id to its loader.
#[path = "../../../../../../../../🔨️modules/🖼️assets/🥽️mesh/🦀️.rs"]
pub mod mesh_assets;

// 🔤️ Canvas's runtime asset port: the one place each renderer host hands the fonts and the icon pack
// to `semio_framework_canvas`, before the first icon or label resolves.
#[path = "../🔤️runtime-assets/🦀️.rs"]
pub mod runtime_assets;

#[path = "../📇️catalog/🦀️.rs"]
pub mod plugin_catalog;
pub use plugin_catalog::{install_plugin_catalog_json_v1, install_plugin_catalog_v1};

#[cfg(not(target_os = "wasi"))]
#[path = "../🧊️renderer/📇️registry/🦀️.rs"]
mod registration;

#[path = "📥️lifted/🔁️asset-decode-jobs/🦀️.rs"]
mod renderer_io;
pub use renderer_io::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "📥️lifted/🏊️kernel-pool/🦀️.rs"]
mod kernel_pool_future;
#[cfg(not(target_arch = "wasm32"))]
pub use kernel_pool_future::KernelPoolFuture;

#[path = "📥️lifted/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod host_io;
pub use host_io::*;

#[cfg(target_arch = "wasm32")]
#[path = "📥️lifted/🌐️browser-worker/🦀️.rs"]
mod boot_subphase;
#[cfg(target_arch = "wasm32")]
pub use boot_subphase::*;

#[path = "📥️lifted/🎨️presentation-raster/🦀️.rs"]
mod host_waker;
pub use host_waker::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
