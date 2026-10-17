/** 🔒️ Shell boot configuration: the app role environment, the option locks and their resolution and merging, the shell defaults, the brand and introduction rules and the durable storage clearing. */
import { builtinUiThemes, type ElementsSurfaceAppearance, type UiLocale } from "@semio-tech/ui-react";
import { createBrowserStoragePort, SHELL_LOCALES, type AppRole, type ShellBrand, type ShellLocale, type ShellTerminology } from "@semio-tech/framework";
import { PRESENCE_CLIENT_STORAGE_KEY } from "../../🛠️ShellHelpers/🔩️plumbing/👥️presence-identity/🟦️.ts";
import { readUiPreferences, resolveUiPreferences } from "../../🎚️UiPreferences/🟦️.ts";

//#region 🔖️types
//#region 👁️✏️BootAppRole
/** 👁️✏️ Reads `VITE_SEMIO_APP_ROLE` (contract freeze §5: `"viewer"`/`"editor"`, default `"editor"`) —
 * the React shell's half of "both shells read the role from `SEMIO_APP_ROLE`/`VITE_SEMIO_APP_ROLE`".
 * Guarded for non-Vite embeds (SSR/tests/other bundlers) where `import.meta.env` is absent. An
 * unrecognized value warns and falls back to `"editor"`, matching {@link resolveShellLocks}'s
 * fall-back-but-stay-safe idiom. */
function readViteAppRoleEnv(): string | undefined {
  try {
    const env = (import.meta as unknown as { readonly env?: Readonly<Record<string, string | undefined>> }).env;
    return env?.VITE_SEMIO_APP_ROLE;
  } catch {
    return undefined;
  }
}

/** 👁️✏️ Resolves the boot-time {@link AppRole}: an explicit `options.appRole` wins, otherwise
 * {@link readViteAppRoleEnv}, otherwise `"editor"`. */
export function resolveBootAppRole(explicit?: AppRole): AppRole {
  if (explicit === "viewer" || explicit === "editor") return explicit;
  const raw = readViteAppRoleEnv();
  if (raw === "viewer" || raw === "editor") return raw;
  if (raw !== undefined && raw !== "") console.warn(`[os] invalid VITE_SEMIO_APP_ROLE ${JSON.stringify(raw)}, falling back to "editor"`);
  return "editor";
}

//#endregion 👁️✏️BootAppRole
//#region 🔒️FrameworkOsLocks
/** 🔒️ Raw boot-time lock values from env, before validation (any of the five may be unset). */
export type FrameworkOsLocks = {
  readonly exampleId?: string;
  readonly locale?: string;
  readonly terminology?: string;
  readonly themeId?: string;
  readonly appearance?: string;
};

/** 🔒️ Validated locks: unknown values warn and fall back to a safe default while staying locked. */
export type ResolvedShellLocks = {
  readonly exampleId?: string;
  readonly locale?: UiLocale;
  readonly terminology?: ShellTerminology;
  readonly themeId?: string;
  readonly appearance?: ElementsSurfaceAppearance;
};

/**
 * 🔒️ Validates raw `FrameworkOsLocks` against what the shell can actually apply at boot. A locked
 * session stays locked even on an invalid value (falls back to a default) rather than silently
 * degrading to switchable — the CLI asked for no in-app switching, so a typo must not remove that.
 */
export function resolveShellLocks(locks: FrameworkOsLocks | undefined): ResolvedShellLocks {
  if (!locks) return {};
  const resolved: { -readonly [K in keyof ResolvedShellLocks]?: ResolvedShellLocks[K] } = {};
  if (locks.exampleId) resolved.exampleId = locks.exampleId;
  if (locks.locale !== undefined) {
    if ((SHELL_LOCALES as readonly string[]).includes(locks.locale)) {
      resolved.locale = locks.locale as ShellLocale;
    } else {
      console.warn(`[os] invalid SEMIO_LOCKED_LOCALE ${JSON.stringify(locks.locale)}, falling back to "en"`);
      resolved.locale = "en";
    }
  }
  if (locks.terminology !== undefined) {
    if (locks.terminology !== "") {
      resolved.terminology = locks.terminology as ShellTerminology;
    }
  }
  if (locks.themeId !== undefined) {
    const known = new Set([...builtinUiThemes().map((t) => t.id), ...Object.keys(resolveUiPreferences(readUiPreferences(createBrowserStoragePort()), { appearance: "system", layout: "desktop", driverId: "default", locale: "en", terminology: "native", themeId: "semio" }).customThemes)]);
    if (known.has(locks.themeId)) {
      resolved.themeId = locks.themeId;
    } else {
      console.warn(`[os] invalid SEMIO_LOCKED_THEME ${JSON.stringify(locks.themeId)}, falling back to "semio"`);
      resolved.themeId = "semio";
    }
  }
  if (locks.appearance !== undefined) {
    if (locks.appearance === "light" || locks.appearance === "dark") {
      resolved.appearance = locks.appearance;
    } else {
      console.warn(`[os] invalid SEMIO_LOCKED_APPEARANCE ${JSON.stringify(locks.appearance)}, falling back to "system"`);
      resolved.appearance = "system";
    }
  }
  return resolved;
}

/** 🔒️ Stable empty-locks reference so an omitted `locks` prop never busts memo dependency arrays. */
export const EMPTY_SHELL_LOCKS: ResolvedShellLocks = {};

/** 🔒️ Overlays env locks onto a brand's locks per key — a lock set by either source stays locked, an env value wins over the brand value. */
export function mergeShellLockSources(brandLocks: FrameworkOsLocks | undefined, envLocks: FrameworkOsLocks | undefined): FrameworkOsLocks | undefined {
  if (!brandLocks || !envLocks) return envLocks ?? brandLocks;
  return { ...brandLocks, ...Object.fromEntries(Object.entries(envLocks).filter(([, value]) => value !== undefined)) };
}

/** 🎛️ Boot-time default values that seed shell state without locking it — the matching in-app switcher stays visible, unlike locks. */
export type FrameworkOsDefaults = {
  readonly exampleId?: string;
};

/** 🎛️ Resolves boot defaults: an env-provided default wins over the brand's. */
export function resolveShellDefaults(brand: ShellBrand | undefined, defaults: FrameworkOsDefaults | undefined): FrameworkOsDefaults {
  return { exampleId: defaults?.exampleId ?? brand?.defaults?.exampleId };
}

/**
 * 🧪️ Picks the example id announced on a fresh session: keep a still-valid active/default id,
 * otherwise the first registered example (matches wgpu `sync_session_chrome`) so the navbar never
 * boots on “No example” while the plugin’s default document is already that first fixture.
 */
export function resolveBootExampleId(
  activeExampleId: string,
  exampleOptions: readonly { readonly id: string }[],
  defaultsExampleId?: string,
): string {
  if (activeExampleId && exampleOptions.some((option) => option.id === activeExampleId)) return activeExampleId;
  if (defaultsExampleId && exampleOptions.some((option) => option.id === defaultsExampleId)) return defaultsExampleId;
  return exampleOptions[0]?.id ?? "";
}

/** 🎓️ Whether a brand's introduction should auto-start on every window load, ignoring any device-local seen flag. */
export function shouldReplayIntroductionOnLoad(brand: ShellBrand | undefined): boolean {
  return isEphemeralShellBrand(brand) || brand?.replayIntroductionOnLoad === true;
}

/** 🎓️ Whether completing or dismissing an introduction should persist a device-local seen flag for this brand. */
export function shouldPersistIntroductionSeen(brand: ShellBrand | undefined): boolean {
  return !shouldReplayIntroductionOnLoad(brand);
}

/** 🧊️ Whether a brand boots with no durable shell state — every refresh starts from locks/defaults only. */
export function isEphemeralShellBrand(brand: ShellBrand | undefined): boolean {
  return brand?.ephemeral === true;
}

/** 🧊️ Removes known durable shell keys so an ephemeral brand leaves no localStorage/sessionStorage residue across refresh. */
export function clearDurableShellStorage(): void {
  if (typeof window === "undefined") return;
  const prefixes = ["ui.chrome.", "ui.introduction.seen.", "ui.themes.", "ui.compute.", "semio.os.", "compose.display.layouts."];
  try {
    const keys = Array.from({ length: localStorage.length }, (_, index) => localStorage.key(index)).filter((key): key is string => typeof key === "string");
    for (const key of keys) {
      if (prefixes.some((prefix) => key === prefix || key.startsWith(prefix))) localStorage.removeItem(key);
    }
  } catch {
    /* ignore */
  }
  try {
    window.sessionStorage.removeItem(PRESENCE_CLIENT_STORAGE_KEY);
  } catch {
    /* ignore */
  }
}

/** 🎛️ Stable empty-defaults reference so an omitted `defaults` prop never busts memo dependency arrays. */
export const EMPTY_SHELL_DEFAULTS: FrameworkOsDefaults = {};
//#endregion 🔒️FrameworkOsLocks
//#endregion 🔖️types
