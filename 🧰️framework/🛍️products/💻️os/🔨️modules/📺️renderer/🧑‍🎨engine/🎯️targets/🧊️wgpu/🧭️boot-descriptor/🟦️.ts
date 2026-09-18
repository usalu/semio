// #region 🧲️Header
/** @emoji 🧭️ The ONE boot-axis vocabulary every wgpu entry point speaks.
 *
 * React has a single boot door ({@link FrameworkOsBootOptions} in
 * `🧱️elements/🐚️Shell/🟦️.tsx`, fed by `🧑‍💻dev/🟦️.ts` + `🧑‍💻dev/🔗️boot-query/🟦️.ts`); wgpu has three
 * (the trunk page `../🚀️browser-boot/🟦️.ts`, the embeddable `../🎬️renderer-boot/🟦️.ts`, and the native
 * `../⌨️native-entrypoint/🦀️.rs`), and each used to carry a different subset of the same axes. This
 * module is the shared shape they all produce, so a url, a library call and a CLI invocation open the
 * same surface. Its Rust twin is `WgpuBootDescriptor` in `../🧊️renderer/🦀️.rs`, which is what the
 * renderer wasm receives (as JSON, through `semioWgpuSetBootDescriptor`) and what the native
 * entrypoint builds directly.
 *
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔗️boot-query/🟦️.ts */
// #endregion 🧲️Header

/** @emoji 📏️ Bound on ONE descriptor field, restated from the trunk page's own former local constant.
 * Every field the three doors accept passes through {@link boundedBootField}, so no door can hand the
 * renderer an unbounded string. */
export const WGPU_BOOT_FIELD_CAPACITY = 2048;

/** @emoji 📏️ Bound on the raw `location.search`/`location.hash` before it is parsed — the twin of
 * `🧑‍💻dev/🔗️boot-query/🟦️.ts`'s `BOOT_QUERY_CAPACITY`, so both renderers refuse the same oversized url
 * instead of handing it to `URLSearchParams`. */
export const WGPU_BOOT_LOCATION_CAPACITY = 8192;

/** @emoji 🔗️ The four per-navigation query axes, spelled exactly as `🔗️boot-query/🟦️.ts` spells them
 * plus the hub trio. A name here is a url parameter name, never a descriptor field name. */
export const WGPU_BOOT_QUERY_PARAMS = { plugin: "plugin", app: "app", role: "role", mode: "mode", example: "example", hub: "hub", user: "user", dataDir: "dataDir" } as const;

/** @emoji 🏷️ The `<meta name="…">` axes the wgpu page serves per SERVER, the twin of React's
 * `VITE_SEMIO_*` build-time env (`🧑‍💻dev/🟦️.ts`): the query is the per-navigation axis, these are the
 * per-server default. `semio-plugin` is injected by `../🌐️server/🟦️.ts`'s `wgpu-browser-selection`
 * plugin and predates this module. */
export const WGPU_BOOT_META_NAMES = {
  plugin: "semio-plugin",
  appId: "semio-app-id",
  appRole: "semio-app-role",
  brandId: "semio-brand",
  defaultExample: "semio-default-example",
  lockedExample: "semio-locked-example",
  lockedLocale: "semio-locked-locale",
  lockedTerminology: "semio-locked-terminology",
  lockedTheme: "semio-locked-theme",
  lockedAppearance: "semio-locked-appearance",
} as const;

/** @emoji #️⃣ The one `#` route both shells understand: a 64-hex local-hub broker proof handed over by
 * `🌎️hub/…/📜️script.ts` when it opens a browser. React reads it once at module scope and immediately
 * rewrites the url without it (`🏛️ShellHost/🟦️.tsx:209-214`); {@link readBootBrokerProof} is that
 * reader and {@link stripBootBrokerProof} is that rewrite. It is deliberately one-shot: a proof that
 * stayed in the address bar would be re-usable from history. */
export const WGPU_BOOT_BROKER_HASH_PATTERN = /^#semio-broker=([0-9a-f]{64})$/u;

/** @emoji 🔒️ Boot-time preference LOCKS — the wgpu twin of React's `FrameworkOsLocks`. A locked
 * preference removes its in-app switcher; an unknown value still stays locked (falling back to a safe
 * default), matching `resolveShellLocks`'s "the CLI asked for no switching, a typo must not restore
 * it" rule. `""` means unset, never "locked to the empty string". */
export type WgpuBootLocks = {
  readonly exampleId: string;
  readonly locale: string;
  readonly terminology: string;
  readonly themeId: string;
  readonly appearance: string;
};

/** @emoji 🎛️ Boot-time preference DEFAULTS — seeds that keep their in-app switcher visible, React's
 * `FrameworkOsDefaults`. */
export type WgpuBootDefaults = {
  readonly exampleId: string;
};

/** @emoji 🌐️ The hub trio (`?hub=&user=&dataDir=`), present only when a hub url was named. */
export type WgpuBootHub = {
  readonly hubUrl: string;
  readonly user: string;
  readonly dataDir: string;
};

/** @emoji 🧭️ Every boot axis, resolved. Field-for-field the Rust `WgpuBootDescriptor`'s serde shape:
 * absent axes are `""`/`{}` rather than optional, so the three doors cannot disagree about what
 * "unset" serializes to. */
export type WgpuBootDescriptor = {
  readonly pluginVariant: string;
  /** @emoji 📌️ Pinned app id — React's `appId`. Empty falls through to the variant's own default app,
   * never reaching the shell as a pin (`🧑‍💻dev/🟦️.ts`'s own note on `VITE_SEMIO_APP_ID`). */
  readonly appId: string;
  readonly appRole: "viewer" | "editor";
  readonly appMode: string;
  readonly appExample: string;
  readonly brandId: string;
  readonly brokerProof: string;
  readonly locks: WgpuBootLocks;
  readonly defaults: WgpuBootDefaults;
  readonly hub: WgpuBootHub | null;
};

/** @emoji 🧭️ What a caller may say instead of the page: every axis, all optional. The embeddable door
 * (`../🎬️renderer-boot/🟦️.ts`) passes exactly this; the trunk page passes nothing and reads the url. */
export type WgpuBootOverrides = {
  readonly plugin?: string;
  readonly appId?: string;
  readonly appRole?: "viewer" | "editor";
  readonly appMode?: string;
  readonly appExample?: string;
  readonly brandId?: string;
  readonly locks?: Partial<WgpuBootLocks>;
  readonly defaults?: Partial<WgpuBootDefaults>;
  readonly hub?: WgpuBootHub;
};

/** @emoji 📏️ Refuses an oversized field by NAME, so the fault text says which axis overflowed. */
export function boundedBootField(value: string, field: string): string {
  if (value.length > WGPU_BOOT_FIELD_CAPACITY) throw new Error(`boot-descriptor-overflow: ${field} exceeds ${WGPU_BOOT_FIELD_CAPACITY} code units`);
  return value;
}

function boundedLocation(value: string, field: string): string {
  if (value.length > WGPU_BOOT_LOCATION_CAPACITY) throw new Error(`boot-descriptor-overflow: ${field} exceeds ${WGPU_BOOT_LOCATION_CAPACITY} code units`);
  return value;
}

/** @emoji #️⃣ The 64-hex broker proof a `#semio-broker=…` hash carries, or `""`. */
export function readBootBrokerProof(hash: string): string {
  return WGPU_BOOT_BROKER_HASH_PATTERN.exec(boundedLocation(hash, "location.hash"))?.[1] ?? "";
}

/** @emoji #️⃣ Rewrites the address bar without the one-shot proof, exactly as React does — same
 * `replaceState` (no history entry), same `pathname + search` target. A page with no proof is left
 * alone, so this never touches a url it did not put there. */
export function stripBootBrokerProof(location: Location, history: History): void {
  if (!readBootBrokerProof(location.hash)) return;
  history.replaceState(history.state, "", `${location.pathname}${location.search}`);
}

/** @emoji 🏷️ Reads one `<meta name="…" content="…">` axis, or `""`. */
export type WgpuBootMetaReader = (name: string) => string;

/** @emoji 🏷️ The document-backed reader the trunk page uses. */
export function documentBootMetaReader(documentRef: Document): WgpuBootMetaReader {
  return (name) => documentRef.querySelector<HTMLMetaElement>(`meta[name="${name}"]`)?.content ?? "";
}

const NO_META: WgpuBootMetaReader = () => "";

/**
 * 🧭️ Resolves the ONE descriptor from the three sources, in React's own precedence: an explicit
 * caller override beats the url, the url beats the per-server `<meta>`, and the `<meta>` beats the
 * built-in fallback. `appRole` follows `?role=`'s `"viewer"`-or-else rule verbatim
 * (`resolveBootQueryAppRole`), and `appExample` collapses `?example=` over the
 * `semio-default-example` seed the way `🧑‍💻dev/🟦️.ts` collapses the query over
 * `VITE_SEMIO_DEFAULT_EXAMPLE` into ONE `defaults.exampleId`.
 *
 * An id no plugin authors is NOT rejected here — a url is not a place to hard-fail a shell; the shell
 * drops it (`ShellState::apply_boot_example`/`apply_boot_mode`). Only overflow is fatal.
 */
export function resolveWgpuBootDescriptor(input: { readonly search?: string; readonly hash?: string; readonly meta?: WgpuBootMetaReader; readonly defaultVariant: string; readonly overrides?: WgpuBootOverrides }): WgpuBootDescriptor {
  const params = new URLSearchParams(boundedLocation(input.search ?? "", "location.search"));
  const meta = input.meta ?? NO_META;
  const overrides = input.overrides ?? {};
  const query = (name: string) => params.get(name) ?? "";
  const axis = (field: string, override: string | undefined, param: string, metaName?: string) => boundedBootField(override ?? (query(param) || (metaName ? meta(metaName) : "")), field);
  const locked = (field: string, override: string | undefined, metaName: string) => boundedBootField(override ?? meta(metaName), field);
  const roleRaw = overrides.appRole ?? query(WGPU_BOOT_QUERY_PARAMS.role) ?? "";
  const roleMeta = meta(WGPU_BOOT_META_NAMES.appRole);
  const hubUrl = boundedBootField(overrides.hub?.hubUrl ?? query(WGPU_BOOT_QUERY_PARAMS.hub), "hub");
  const defaultExample = locked("defaults.exampleId", overrides.defaults?.exampleId, WGPU_BOOT_META_NAMES.defaultExample);
  return {
    pluginVariant: boundedBootField(overrides.plugin ?? (query(WGPU_BOOT_QUERY_PARAMS.plugin) || meta(WGPU_BOOT_META_NAMES.plugin) || input.defaultVariant), "plugin"),
    appId: axis("app", overrides.appId, WGPU_BOOT_QUERY_PARAMS.app, WGPU_BOOT_META_NAMES.appId),
    appRole: roleRaw === "viewer" || (roleRaw === "" && roleMeta === "viewer") ? "viewer" : "editor",
    appMode: axis("mode", overrides.appMode, WGPU_BOOT_QUERY_PARAMS.mode),
    appExample: boundedBootField(overrides.appExample ?? (query(WGPU_BOOT_QUERY_PARAMS.example) || defaultExample), "example"),
    brandId: locked("brand", overrides.brandId, WGPU_BOOT_META_NAMES.brandId),
    brokerProof: readBootBrokerProof(input.hash ?? ""),
    locks: {
      exampleId: locked("locks.exampleId", overrides.locks?.exampleId, WGPU_BOOT_META_NAMES.lockedExample),
      locale: locked("locks.locale", overrides.locks?.locale, WGPU_BOOT_META_NAMES.lockedLocale),
      terminology: locked("locks.terminology", overrides.locks?.terminology, WGPU_BOOT_META_NAMES.lockedTerminology),
      themeId: locked("locks.themeId", overrides.locks?.themeId, WGPU_BOOT_META_NAMES.lockedTheme),
      appearance: locked("locks.appearance", overrides.locks?.appearance, WGPU_BOOT_META_NAMES.lockedAppearance),
    },
    defaults: { exampleId: defaultExample },
    hub: hubUrl ? { hubUrl, user: boundedBootField(overrides.hub?.user ?? query(WGPU_BOOT_QUERY_PARAMS.user), "user"), dataDir: boundedBootField(overrides.hub?.dataDir ?? query(WGPU_BOOT_QUERY_PARAMS.dataDir), "dataDir") } : null,
  };
}

// #region 🌓️HostAppearance
/** @emoji 🌓️ The one media query both renderers ask — React asks it in
 * `ensureElementsSurfaceChromeSystemListeners` and `resolveElementsSurfaceChromeDark`. */
export const WGPU_PREFERS_DARK_MEDIA_QUERY = "(prefers-color-scheme: dark)";

/** @emoji 🔑️ Byte-identical to `OsShellConfig`'s own storage key and to the wgpu shell's Rust mirror
 * `OS_SHELL_CONFIG_STORAGE_KEY` (`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`). Mirrored rather than
 * imported on purpose: this module is bundled into BOTH browser artifacts, and importing
 * `🎚️UiPreferences/🟦️.ts` would drag `@semio-tech/framework` and `@semio-tech/ui-react` into the trunk
 * page and the frame Worker for two string constants. */
export const WGPU_OS_SHELL_CONFIG_STORAGE_KEY = "semio.os.config";

/** @emoji 🔑️ Byte-identical to `UI_PREFERENCES_CONFIG_SCHEMA` (`🎚️UiPreferences/🟦️.ts`). */
export const WGPU_UI_PREFERENCES_CONFIG_SCHEMA = "os.config.ui-preferences";

/** @emoji 🌓️ The persisted appearance preference, `""` when the event log holds no `setAppearance` —
 * React's `preferences.appearance == null`, before `resolveUiPreferences`'s `?? "system"` seed. */
export type WgpuHostAppearancePreference = "" | "system" | "light" | "dark";

/** @emoji 🌓️ The two appearance inputs the renderer wasm cannot read for itself, resolved by whoever
 * owns a `window`. Handed to `semioWgpuSetHostAppearance` (`🧊️renderer/🦀️.rs`, region 🌓️HostAppearance)
 * at boot and again on every change.
 *
 * 🩸️ Why it exists: the browser renderer runs inside the frame Worker, whose realm owns neither
 * `window` nor `localStorage`. Its own `matchMedia` read therefore always missed (falling back to
 * DARK) and its own `localStorage` reads always answered nothing, so a host that React booted LIGHT
 * the wgpu shell booted DARK. Both reads move here, to the one thread that can make them. */
export type WgpuHostAppearance = { readonly preference: WgpuHostAppearancePreference; readonly systemDark: boolean };

/** @emoji 🌓️ Replays the persisted `os.config.ui-preferences` event log for its LAST `setAppearance`
 * — the same projection `replayUiPreferenceEvents` performs, narrowed to the one field a boot needs,
 * and the same read the React serve's own pre-paint inline script makes
 * (`🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`'s `PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT`). Every step
 * is guarded: a sandboxed page throws on `localStorage`, and a malformed log reads as "unset" rather
 * than pinning an appearance nobody chose. */
export function readPersistedAppearancePreference(storage: Pick<Storage, "getItem"> | undefined): WgpuHostAppearancePreference {
  try {
    const config = storage?.getItem(WGPU_OS_SHELL_CONFIG_STORAGE_KEY);
    if (!config) return "";
    const preferences = (JSON.parse(config) as { readonly preferences?: Record<string, unknown> }).preferences;
    const raw = preferences?.[WGPU_UI_PREFERENCES_CONFIG_SCHEMA];
    if (typeof raw !== "string") return "";
    const log = JSON.parse(raw) as { readonly version?: unknown; readonly events?: unknown };
    if (log.version !== 1 || !Array.isArray(log.events)) return "";
    let resolved: WgpuHostAppearancePreference = "";
    for (const event of log.events as readonly { readonly mutation?: unknown; readonly appearance?: unknown }[]) {
      if (event?.mutation !== "setAppearance") continue;
      resolved = event.appearance === "light" || event.appearance === "dark" || event.appearance === "system" ? event.appearance : "";
    }
    return resolved;
  } catch {
    return "";
  }
}

/** @emoji 🌓️ Both inputs from one page realm. `systemDark` falls back to `false` where `matchMedia`
 * is unavailable, byte-for-byte `resolveElementsSurfaceChromeDark`'s own `typeof window === "undefined"`
 * arm (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`) — never the `true` the Worker used to fall through to. */
export function resolveWgpuHostAppearance(view: { readonly localStorage?: Pick<Storage, "getItem">; readonly matchMedia?: (query: string) => MediaQueryList }): WgpuHostAppearance {
  let storage: Pick<Storage, "getItem"> | undefined;
  try {
    storage = view.localStorage;
  } catch {
    storage = undefined;
  }
  return { preference: readPersistedAppearancePreference(storage), systemDark: view.matchMedia?.(WGPU_PREFERS_DARK_MEDIA_QUERY).matches ?? false };
}
// #endregion 🌓️HostAppearance

// #region ⌨️HostPlatform
/** @emoji ⌨️ The host's own platform string, as React's `keybindingPlatformUsesMetaV1`
 * (`🖱️ui/🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts`) reads it: `userAgentData.platform`
 * (`"macOS"`) when the browser publishes it, else the legacy `navigator.platform` (`"MacIntel"`, …).
 * `""` means "nothing to say", which the renderer reads as its own compile-time default.
 *
 * 🩸️ Why it travels: the renderer resolved `mod` with `cfg!(target_os = "macos")`, which is FALSE on
 * `wasm32-unknown-unknown`, so a macOS browser painted `Ctrl+Alt+E` where React painted `⌘️⌥️E`.
 * `userAgentData` exists only on the page thread, so the page makes the read and forwards it — an
 * environment field beside `locale`/`dpr`/`appearance`, never a boot axis: React's
 * `FrameworkOsBootOptions` has no twin for it (it reads the navigator directly), and
 * `🧪️tests/🧭️boot-axis-parity/🦀️.rs` holds both lists to the same shape. */
export type WgpuHostPlatform = string;

/** @emoji ⌨️ The page realm's platform read. Every step is guarded: a realm with no `navigator`, or
 * one that throws on `userAgentData`, answers `""` rather than pinning a platform nobody is on. */
export function resolveWgpuHostPlatform(view: { readonly navigator?: Navigator }): WgpuHostPlatform {
  try {
    const agent = view.navigator;
    if (!agent) return "";
    if ("userAgentData" in agent && agent.userAgentData && typeof agent.userAgentData === "object" && "platform" in agent.userAgentData) {
      const platform = (agent.userAgentData as { readonly platform?: string }).platform;
      if (typeof platform === "string" && platform.length > 0) return platform;
    }
    return typeof agent.platform === "string" ? agent.platform : "";
  } catch {
    return "";
  }
}
// #endregion ⌨️HostPlatform

// #region 🗄️HostStorage
/** @emoji 🔑️ Every durable browser key the wgpu shell's OWN persistence lane reads or writes, in the
 * spelling React's `StoragePort` uses (`🖥️platform/🟦️.ts`'s `createBrowserStoragePort` — flat
 * `localStorage`, one key per entry). Byte-identical spellings are the whole point: a host that chose
 * Light, dismissed a tour or rearranged its dock in React must find all three again after switching to
 * the wgpu renderer, and the other way round.
 *
 * 🚫️ What is deliberately NOT here: `ui.chrome.*`, `ui.themes.custom`, `ui.drivers.custom` and
 * `ui.keybindings.overrides`. Those belong to the standalone `🖱️ui` surface library (`useUiTerminology`,
 * `UiDriver`, the locale detector); React's OWN OS shell never reads them either — it persists through
 * `semio.os.config`'s `os.config.ui-preferences` event log, and so does this renderer. Carrying them
 * would be dead payload in the frame Worker. `semio.presence.client` is `sessionStorage` and
 * credential-bearing (a presence identity pack), so it stays off the snapshot too and is reachable only
 * through an explicit `scope: "session"` door call. The Rust census
 * (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, region 🚪️StorageDoor) classifies all of them and a law pins the
 * two lists against React's own constants. */
export const WGPU_HOST_STORAGE_KEYS = ["SEMIO_RUNTIME_DIAGNOSTICS", "semio.os.config", "ui.compute.workerCount"] as const;

/** @emoji 🔑️ Key FAMILIES the snapshot scans for, because their tail is a runtime app id —
 * `UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`). */
export const WGPU_HOST_STORAGE_KEY_PREFIXES = ["ui.introduction.seen."] as const;

/** @emoji 📏️ Per-value ceiling, the same 64 KiB `OS_SHELL_CONFIG_MAX_BYTES` the shell already refuses a
 * larger `semio.os.config` document at, so the door cannot become the wider hole. */
export const WGPU_HOST_STORAGE_VALUE_MAX_BYTES = 64 * 1024;

/** @emoji 📏️ Ceiling on the whole boot snapshot. A snapshot is structured-cloned into the frame Worker
 * before its first frame, so it is priced like any other boot payload rather than left unbounded. */
export const WGPU_HOST_STORAGE_SNAPSHOT_MAX_BYTES = 128 * 1024;

/** @emoji 🗄️ Which browser store one door call addresses — React's two, no more. */
export type WgpuHostStorageScope = "local" | "session";

/** @emoji 🗄️ The page realm's read of every carried key, handed to the renderer wasm's
 * `semioWgpuSetHostStorage` with the boot descriptor so the shell's FIRST frame can answer
 * appearance, tour-seen and dock-skeleton reads synchronously — no flash, no second boot pass. */
export type WgpuHostStorageSnapshot = Readonly<Record<string, string>>;

/** @emoji 🔑️ Whether the door may address `key` at all. The door is an allowlist, not a general
 * `localStorage` proxy: the shell is the only caller and its census is closed, so an unknown key is a
 * defect worth refusing loudly rather than a value worth serving. */
export function wgpuHostStorageCarriesKey(key: string): boolean {
  return (WGPU_HOST_STORAGE_KEYS as readonly string[]).includes(key) || WGPU_HOST_STORAGE_KEY_PREFIXES.some((prefix) => key.startsWith(prefix) && key.length > prefix.length);
}

/** @emoji 🗄️ Reads every carried key from the page's `localStorage`, in sorted key order so two reads of
 * one store produce one snapshot. Each step is guarded: a sandboxed page throws on `localStorage`, an
 * oversized value is dropped rather than truncated (a half document parses as garbage), and the scan
 * stops at {@link WGPU_HOST_STORAGE_SNAPSHOT_MAX_BYTES} rather than growing with the store. */
export function readWgpuHostStorageSnapshot(view: { readonly localStorage?: Storage }): WgpuHostStorageSnapshot {
  let storage: Storage | undefined;
  try {
    storage = view.localStorage;
  } catch {
    return {};
  }
  if (!storage) return {};
  const names = new Set<string>(WGPU_HOST_STORAGE_KEYS);
  try {
    for (let index = 0; index < storage.length; index += 1) {
      const name = storage.key(index);
      if (typeof name === "string" && wgpuHostStorageCarriesKey(name)) names.add(name);
    }
  } catch {
    /* an enumeration a sandboxed store refuses still leaves the fixed keys readable */
  }
  const snapshot: Record<string, string> = {};
  let bytes = 0;
  for (const name of [...names].sort()) {
    let value: string | null = null;
    try {
      value = storage.getItem(name);
    } catch {
      value = null;
    }
    if (value === null || value.length > WGPU_HOST_STORAGE_VALUE_MAX_BYTES) continue;
    bytes += name.length + value.length;
    if (bytes > WGPU_HOST_STORAGE_SNAPSHOT_MAX_BYTES) break;
    snapshot[name] = value;
  }
  return snapshot;
}
// #endregion 🗄️HostStorage

// #region 🔖️ReadinessBeacon
/** @emoji 🪪️ React's own beacon value when no plugin filter resolved (`pluginFilter ?? "unknown"`,
 * `🏛️ShellHost/🟦️.tsx`'s 🔖️ReadinessBeacon effect) — reached on this target only when the boot failed
 * before a descriptor could be built. */
export const WGPU_READINESS_BEACON_UNKNOWN_PLUGIN = "unknown";

/** @emoji 🚦️ The two-attribute readiness beacon React writes on `document.documentElement`:
 * `data-semio-os-ready` and `data-semio-os-error`, each carrying the plugin id and each CLEARING the
 * other, so a reader never sees both and never has to guess which is newer.
 *
 * 🩸️ The wgpu page set NEITHER, so `data-semio-os-ready` stayed `null` through a 45 s clean boot
 * (`📓️audit-visual-parity-puzzle3d.md` §7 item 8) and every probe and e2e runner that waits on the
 * attribute — `🌎️hub/📦️packages/🦀️rust/📜️script.ts`'s `[data-semio-os-ready]` locator included — had to
 * fall back to a fixed sleep on this target while working unchanged on React's.
 *
 * 🪟️ It lives HERE, beside the page realm's other reads, for the same reason they do: the frame Worker
 * owns no `document` at all, so only the page can write it — and a pure module is the only shape a unit
 * suite can import (`🚀️browser-boot/🟦️.ts` mounts the shell on import). `root` is the element to stamp,
 * which is always `document.documentElement` in production and a fake in the laws. */
export function wgpuReadinessBeacon(root: HTMLElement, beaconId: string) {
  return {
    ready: () => {
      root.dataset.semioOsReady = beaconId;
      delete root.dataset.semioOsError;
    },
    error: () => {
      root.dataset.semioOsError = beaconId;
      delete root.dataset.semioOsReady;
    },
    clear: () => {
      delete root.dataset.semioOsReady;
      delete root.dataset.semioOsError;
    },
  };
}
// #endregion 🔖️ReadinessBeacon
