/** 🔌️ The UI i18n port on its own: the domain-neutral chrome bundles (English and German), the shared
 * {@link uiI18n} over the owned translation engine, product bundle registration, per-shell instances and locale resolution —
 * importable without the rest of the React target as `@semio-tech/ui-react/i18n`. The engine
 * ({@link createUiI18nInstance}) has no runtime dependency; i18next is only the oracle of its tests. The React target barrel
 * re-exports everything here unchanged.
 *
 * @see ../../../🧱️elements/📚️I18n/🟦️.tsx — the port interface, label shapes and key types
 * @see ../🟦️.tsx — the barrel that re-exports this module
 */

import { createUiI18nInstance, type UiI18nCallOptions, type UiI18nInstance, type UiI18nTree } from "./🔡️instance/🟦️.ts";
import { createBrowserStoragePort, type StoragePort } from "../🔨️modules/🗄️storage-port/🟦️.ts";
import { ephemeralSet } from "@semio-tech/framework-transient";
import { type ShellLocale } from "@semio-tech/framework-manifest/generated/ui-axes";
import { resolveUiLabel, type DeepUiTranslationKeys, type UiI18nPort, type UiLabelPair, type UiLabelValue, type UiTranslationKey, type UiLocale, type UiRegisteredTranslationKey, type UiRibbonParentEntries, type UiTranslateFn, type UiTranslationSchema } from "../🧱️elements/📚️I18n/🟦️.tsx";
import { shellChromeDe, shellChromeEn } from "./🧭️shell-chrome/🟦️.ts";
import { configurationDe, configurationEn } from "./⚙️configuration/🟦️.ts";
import { contentHostsDe, contentHostsEn } from "./🧩️content-hosts/🟦️.ts";
import { collaborationDe, collaborationEn } from "./🤝️collaboration/🟦️.ts";
import { guidanceDe, guidanceEn } from "./🎓️guidance/🟦️.ts";

export { resolveUiLabel };
export type { DeepUiTranslationKeys, UiI18nPort, UiLabelPair, UiLabelValue, UiLocale, UiRegisteredTranslationKey, UiTranslateFn, UiTranslationKey, UiTranslationSchema };

// #region 🌐️LocaleStorage
/** 🌐️ Storage key for the active UI locale. */
export const UI_CHROME_LOCALE_STORAGE_KEY = "ui.chrome.locale";

/** 💾️ Reads the persisted UI locale from the given shell's storage, if any. */
export function readStoredUiChromeLocale(storage: StoragePort): UiLocale | null {
  const raw = storage.get(UI_CHROME_LOCALE_STORAGE_KEY);
  return raw === "en" || raw === "de" ? raw : null;
}

/** 📝️ Persists the active UI locale to the given shell's storage. */
export function writeStoredUiChromeLocale(storage: StoragePort, locale: UiLocale): void {
  storage.set(UI_CHROME_LOCALE_STORAGE_KEY, locale);
}

export const uiChromeTranslationBundles = {
  // #region 🇩️🇪️ German Bundle
  de: {
    translation: {
      ui: { ...shellChromeDe, ...configurationDe, ...contentHostsDe, ...collaborationDe },
      ...guidanceDe,
    } satisfies UiTranslationSchema,
  },
  // #endregion 🇩️🇪️ German Bundle

  // #region 🇬️🇧️ English Bundle
  en: {
    translation: {
      ui: { ...shellChromeEn, ...configurationEn, ...contentHostsEn, ...collaborationEn },
      ...guidanceEn,
    } satisfies UiTranslationSchema,
  },
  // #endregion 🇬️🇧️ English Bundle
} satisfies Record<UiLocale, { readonly translation: UiTranslationSchema }>;

// #region 🔌️I18n Port
// i18n "port"/wiring glue: registration functions, locale resolvers, and the shared port instance.

export type UiTranslationLocaleCode = UiLocale;

export type UiTranslationBundlesInput = {
  readonly [L in UiLocale]: { readonly translation: Record<string, unknown> };
};

// UiRegisteredTranslationKey imported from core I18n above/with schema import

/** 🪁️ Merges additional locale bundles into the shared UI i18n instance, requiring every
 * {@link UiLocale} to register the exact same schema `S` (a compile error otherwise — the same
 * both-locales-or-nothing guarantee the domain-neutral chrome bundle gets from `satisfies
 * UiTranslationSchema`). Returns a caster from `S`'s own dot-path key union to {@link UiRegisteredTranslationKey}
 * — the only way callers should obtain a key for their registered strings; passing an unregistered
 * string does not type-check. */
export function registerUiTranslationBundles<S extends Record<string, unknown>>(bundles: { readonly [L in UiLocale]: { readonly translation: S } }): <K extends DeepUiTranslationKeys<S>>(key: K) => UiRegisteredTranslationKey {
  applyUiTranslationBundleTo(uiI18nInstance, bundles);
  registeredUiTranslationBundles.push(bundles);
  for (const instance of liveShellI18nInstances) applyUiTranslationBundleTo(instance, bundles);
  return (key) => key as UiRegisteredTranslationKey;
}

/** 📦️ Adds the bundle of one locale to the shared UI i18n instance and to every live shell instance — for a product that
 * fetches its languages one at a time, each as a chunk of its own, and registers a language once its texts have arrived. */
export function registerUiTranslationBundle(locale: UiLocale, translation: Record<string, unknown>): void {
  const bundle = { [locale]: { translation } };
  applyUiTranslationBundleTo(uiI18nInstance, bundle);
  registeredUiTranslationBundles.push(bundle);
  for (const instance of liveShellI18nInstances) applyUiTranslationBundleTo(instance, bundle);
}

// #region 🐚️ShellI18n
/** 🐚️ Every bundle ever registered — the chrome's own domain-neutral one plus every product's, in
 * registration order — replayed onto each new per-shell instance at creation time so an
 * embedded shell has the same translations as the page-owning singleton from its very first render.
 * Seeded with `uiChromeTranslationBundles` itself since that one is loaded directly into the shared instance
 * rather than through `registerUiTranslationBundles`. */
const registeredUiTranslationBundles: Partial<{ readonly [L in UiLocale]: { readonly translation: Record<string, unknown> } }>[] = [uiChromeTranslationBundles];

/** 👥️ Every currently-mounted shell's own translation instance — {@link registerUiTranslationBundles}
 * replays a late-registering bundle (e.g. a lazily-loaded product module importing after some shells
 * already mounted) into each of these too, not just the shared singleton. */
const liveShellI18nInstances = ephemeralSet<UiI18nInstance>("framework.modules.ui.packages.typescript.targets.react.index.tsx.liveShellI18nInstances");

function applyUiTranslationBundleTo(instance: UiI18nInstance, bundle: Partial<{ readonly [L in UiLocale]: { readonly translation: Record<string, unknown> } }>): void {
  for (const [language, resource] of Object.entries(bundle)) {
    if (resource !== undefined) instance.addBundle(language, resource.translation as UiI18nTree);
  }
}

/** 🆕️ Creates a fresh translation instance for one shell, pre-loaded with every bundle registered so far, so an embedded
 * shell never flashes untranslated chrome on its first paint either. {@link UiI18nProvider} resolves the *nearest*
 * provider ancestor via context, so wrapping a shell's subtree in one (see `ShellScopeProvider`) is the only wiring
 * `useUiTranslation`/`useLabel` call sites need. */
export function createShellI18nInstance(initialLocale: UiLocale): UiI18nInstance {
  if (initialLocale !== "en" && initialLocale !== "de") throw new Error("Shell locale requires an explicit supported language source");
  const instance = createUiI18nInstance({ language: initialLocale, ...UI_I18N_STATIC_OPTIONS });
  for (const bundle of registeredUiTranslationBundles) applyUiTranslationBundleTo(instance, bundle);
  liveShellI18nInstances.add(instance);
  return instance;
}

/** 🔓️ Releases a shell's translation instance on unmount — stops it receiving future
 * {@link registerUiTranslationBundles} replays. */
export function disposeShellI18nInstance(instance: UiI18nInstance): void {
  liveShellI18nInstances.delete(instance);
}
// #endregion 🐚️ShellI18n

function normalizeUiLocale(language?: string): UiTranslationLocaleCode {
  return language?.toLowerCase().startsWith("de") ? "de" : "en";
}

/** 🧭️ Maps a BCP47 tag (e.g. `navigator.language`, `"de-AT"`) onto a {@link ShellLocale};
 * defaults to `"en"`. Same rule the chrome's own locale detector uses — exposed so boot code
 * (renderer/demonstrator) can resolve a default before any brand lock is known. */
export const detectShellLocale = normalizeUiLocale as (language?: string) => ShellLocale;

const UI_I18N_STATIC_OPTIONS = { fallbackLanguage: "en", supportedLanguages: ["en", "de"] } as const;

const UI_I18N_SHARED_KEY = Symbol.for("semio.ui.i18n.shared");

function pageSharedUiI18nInstance(): UiI18nInstance | undefined {
  return (globalThis as unknown as Record<symbol, UiI18nInstance | undefined>)[UI_I18N_SHARED_KEY];
}

function resolveRequestedUiLocale(): UiTranslationLocaleCode {
  const storedLocale = readStoredUiChromeLocale(createBrowserStoragePort());
  if (storedLocale) return storedLocale;
  const shared = pageSharedUiI18nInstance();
  return normalizeUiLocale(shared?.resolvedLanguage || shared?.language || (typeof navigator !== "undefined" ? navigator.language : undefined));
}

function registerUiChromeTranslationBundles() {
  registerUiTranslationBundles(uiChromeTranslationBundles);
}

function createUiI18nPort(instance: UiI18nInstance): UiI18nPort {
  return {
    t: ((key, options) => instance.t(key, options as UiI18nCallOptions | undefined)) as UiTranslateFn,
    tIn: (locale, key, options) => instance.getFixedT(locale)(key, options as UiI18nCallOptions | undefined),
    exists: (key) => instance.exists(key),
    changeLanguage: (locale) => instance.changeLanguage(locale),
    get language() {
      return instance.language;
    },
    get resolvedLanguage() {
      return instance.resolvedLanguage;
    },
    get isInitialized() {
      return true;
    },
  };
}

/** 📃️ The page's one shared translation instance, held in the global symbol registry so every evaluation of this module
 * shares it — a second evaluation (the module served again under another URL while the page keeps its mounted views) must
 * not strand those views on an instance nobody feeds any more. */
export const uiI18nInstance: UiI18nInstance = (() => {
  const registry = globalThis as unknown as Record<symbol, UiI18nInstance | undefined>;
  return (registry[UI_I18N_SHARED_KEY] ??= createUiI18nInstance({ language: resolveRequestedUiLocale(), ...UI_I18N_STATIC_OPTIONS }));
})();

function initializeUiI18n(): UiI18nPort {
  const requestedLocale = resolveRequestedUiLocale();
  registerUiChromeTranslationBundles();
  if (uiI18nInstance.language !== requestedLocale) void uiI18nInstance.changeLanguage(requestedLocale);
  return createUiI18nPort(uiI18nInstance);
}

/** 🧩️ Shared UI i18n port (domain-neutral bundles; extend via {@link registerUiTranslationBundles}). */
export const uiI18n = initializeUiI18n();

/** 🔤️ Sets the active UI locale on the shared i18n port (user-initiated, in-app switch —
 * for boot-time/brand-locked locale resolution, use {@link initUiLocaleSync} instead, which runs
 * before the first render rather than in a post-paint effect). */
export function setUiLocale(locale: UiLocale): Promise<unknown> {
  if (typeof document !== "undefined") document.documentElement.lang = locale;
  return uiI18n.changeLanguage(locale);
}

/** 🚀️ Resolves the shell's locale synchronously, before the first React render — call this
 * at renderer/demonstrator boot (module scope or before `ReactDOM.createRoot(...).render(...)`),
 * never from a `useEffect`. A `useEffect`-based call runs after the first paint has already
 * committed, which is exactly how a German-locked brand could still flash English chrome
 * ("Skip"/"Back"/"Next"/"Done") on first load. Persists the locale (so a reload's
 * `resolveRequestedUiLocale` agrees) and sets `documentElement.lang` synchronously; also nudges the
 * already-initialized shared instance in case this runs after `uiI18n`'s own module-load default
 * resolved differently. */
export function initUiLocaleSync(locale: ShellLocale): void {
  writeStoredUiChromeLocale(createBrowserStoragePort(), locale);
  if (typeof document !== "undefined") document.documentElement.lang = locale;
  if (uiI18nInstance.language !== locale) void uiI18nInstance.changeLanguage(locale);
}

// #endregion 🔌️I18n Port
