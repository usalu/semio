/** 🌐️ The UI i18n port on its own: the domain-neutral chrome bundles (English and German), the shared
 * i18next-backed {@link uiI18n}, product bundle registration, per-shell instances and locale resolution —
 * importable without the rest of the React target as `@semio-tech/ui-react/i18n`. i18next and react-i18next stay
 * behind the owned {@link UiI18nPort}; the React target barrel re-exports everything here unchanged.
 *
 * @see ../../../🧱️elements/📚️I18n/🟦️.tsx — the port interface, label shapes and key types
 * @see ../🟦️.tsx — the barrel that re-exports this module
 */

import i18next from "i18next";
import { initReactI18next } from "react-i18next";
import { createBrowserStoragePort, type StoragePort } from "../../../🔨️modules/🗄️storage-port/🟦️.ts";
import { ephemeralSet } from "@semio-tech/framework/transient";
import { type ShellLocale } from "@semio-tech/framework/manifest/generated/ui-axes";
import { resolveUiLabel, type DeepUiTranslationKeys, type UiI18nPort, type UiLabelPair, type UiLabelValue, type UiTranslationKey, type UiLocale, type UiRegisteredTranslationKey, type UiRibbonParentEntries, type UiTranslateFn, type UiTranslationSchema } from "../../../🧱️elements/📚️I18n/🟦️.tsx";
import { shellChromeDe, shellChromeEn } from "./🧭️shell-chrome/🟦️.ts";
import { configurationDe, configurationEn } from "./⚙️configuration/🟦️.ts";
import { contentHostsDe, contentHostsEn } from "./🧩️content-hosts/🟦️.ts";
import { collaborationDe, collaborationEn } from "./🤝️collaboration/🟦️.ts";
import { guidanceDe, guidanceEn } from "./🎓️guidance/🟦️.ts";

export { resolveUiLabel };
export type { DeepUiTranslationKeys, UiI18nPort, UiLabelPair, UiLabelValue, UiLocale, UiRegisteredTranslationKey, UiTranslateFn, UiTranslationKey, UiTranslationSchema };

// #region 🌐️LocaleStorage
/** @emoji 🌐️ Storage key for the active UI locale. */
export const UI_CHROME_LOCALE_STORAGE_KEY = "ui.chrome.locale";

/** @emoji 🌐️ Reads the persisted UI locale from the given shell's storage, if any. */
export function readStoredUiChromeLocale(storage: StoragePort): UiLocale | null {
  const raw = storage.get(UI_CHROME_LOCALE_STORAGE_KEY);
  return raw === "en" || raw === "de" ? raw : null;
}

/** @emoji 🌐️ Persists the active UI locale to the given shell's storage. */
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
// i18n "port"/wiring glue: registration functions, locale resolvers, the i18next module augmentation, and the shared port instance.

export type UiTranslationLocaleCode = UiLocale;

export type UiTranslationBundlesInput = {
  readonly [L in UiLocale]: { readonly translation: Record<string, unknown> };
};

declare module "i18next" {
  interface CustomTypeOptions {
    defaultNS: "translation";
    resources: {
      readonly en: { readonly translation: UiTranslationSchema };
      readonly de: { readonly translation: UiTranslationSchema };
    };
  }
}

// UiRegisteredTranslationKey imported from core I18n above/with schema import

/** @emoji 🪁️ Merges additional locale bundles into the shared UI i18n instance, requiring every
 * {@link UiLocale} to register the exact same schema `S` (a compile error otherwise — the same
 * both-locales-or-nothing guarantee the domain-neutral chrome bundle gets from `satisfies
 * UiTranslationSchema`). Returns a caster from `S`'s own dot-path key union to {@link UiRegisteredTranslationKey}
 * — the only way callers should obtain a key for their registered strings; passing an unregistered
 * string does not type-check. */
export function registerUiTranslationBundles<S extends Record<string, unknown>>(bundles: { readonly [L in UiLocale]: { readonly translation: S } }): <K extends DeepUiTranslationKeys<S>>(key: K) => UiRegisteredTranslationKey {
  applyUiTranslationBundleTo(i18next, bundles);
  registeredUiTranslationBundles.push(bundles);
  for (const instance of liveShellI18nInstances) applyUiTranslationBundleTo(instance, bundles);
  return (key) => key as UiRegisteredTranslationKey;
}

// #region 🐚️ShellI18n
/** 🐚️ Every bundle ever registered — the chrome's own domain-neutral one plus every product's, in
 * registration order — replayed onto each new per-shell i18next instance at creation time so an
 * embedded shell has the same translations as the page-owning singleton from its very first render.
 * Seeded with `uiChromeTranslationBundles` itself since that one is loaded directly via `.init({resources})`
 * rather than through `registerUiTranslationBundles`. */
const registeredUiTranslationBundles: { readonly [L in UiLocale]: { readonly translation: Record<string, unknown> } }[] = [uiChromeTranslationBundles];

/** 🐚️ Every currently-mounted shell's own i18next instance — {@link registerUiTranslationBundles}
 * replays a late-registering bundle (e.g. a lazily-loaded product module importing after some shells
 * already mounted) into each of these too, not just the shared singleton. */
const liveShellI18nInstances = ephemeralSet<typeof i18next>("framework.modules.ui.packages.typescript.targets.react.index.tsx.liveShellI18nInstances");

function applyUiTranslationBundleTo(instance: typeof i18next, bundle: { readonly [L in UiLocale]: { readonly translation: Record<string, unknown> } }): void {
  for (const [language, resource] of Object.entries(bundle)) {
    instance.addResourceBundle(language, "translation", resource.translation, true, true);
  }
}

/** 🐚️ Creates and synchronously initializes a fresh i18next instance for one shell, pre-loaded with
 * every bundle registered so far — mirrors {@link initializeUiI18n}'s own synchronous init (`initImmediate:
 * false`) so an embedded shell never flashes untranslated chrome on its first paint either. `react-i18next`
 * resolves the *nearest* `I18nextProvider` ancestor via context, so wrapping a shell's subtree in one
 * (see `FrameworkOsShell`) is the only wiring `useUiTranslation`/`useLabel` call sites need — none of
 * their many call sites throughout this file change. */
export function createShellI18nInstance(initialLocale: UiLocale): typeof i18next {
  if (initialLocale !== "en" && initialLocale !== "de") throw new Error("Shell locale requires an explicit supported language source");
  const instance = i18next.createInstance();
  instance.use(initReactI18next);
  void instance.init({
    resources: {},
    fallbackLng: "en",
    supportedLngs: ["en", "de"],
    nonExplicitSupportedLngs: true,
    lng: initialLocale,
    showSupportNotice: false,
    returnObjects: true,
    initImmediate: false,
    interpolation: { escapeValue: false },
    react: { useSuspense: false, bindI18n: "languageChanged", bindI18nStore: "added removed" },
  });
  for (const bundle of registeredUiTranslationBundles) applyUiTranslationBundleTo(instance, bundle);
  liveShellI18nInstances.add(instance);
  return instance;
}

/** 🐚️ Releases a shell's i18next instance on unmount — stops it receiving future
 * {@link registerUiTranslationBundles} replays. */
export function disposeShellI18nInstance(instance: typeof i18next): void {
  liveShellI18nInstances.delete(instance);
}
// #endregion 🐚️ShellI18n

function normalizeUiLocale(language?: string): UiTranslationLocaleCode {
  return language?.toLowerCase().startsWith("de") ? "de" : "en";
}

/** @emoji 🧭️ Maps a BCP47 tag (e.g. `navigator.language`, `"de-AT"`) onto a {@link ShellLocale};
 * defaults to `"en"`. Same rule the chrome's own locale detector uses — exposed so boot code
 * (renderer/demonstrator) can resolve a default before any brand lock is known. */
export const detectShellLocale = normalizeUiLocale as (language?: string) => ShellLocale;

function resolveRequestedUiLocale(): UiTranslationLocaleCode {
  const storedLocale = readStoredUiChromeLocale(createBrowserStoragePort());
  if (storedLocale) return storedLocale;
  return normalizeUiLocale(i18next.resolvedLanguage || i18next.language || (typeof navigator !== "undefined" ? navigator.language : undefined));
}

function registerUiChromeTranslationBundles() {
  registerUiTranslationBundles(uiChromeTranslationBundles);
}

function createUiI18nPort(instance: typeof i18next): UiI18nPort {
  return {
    t: ((key, options) => instance.t(key as never, options as never)) as UiTranslateFn,
    tIn: (locale, key, options) => instance.getFixedT(locale)(key as never, options as never),
    exists: (key) => instance.exists(key),
    changeLanguage: (locale) => instance.changeLanguage(locale),
    get language() {
      return instance.language;
    },
    get resolvedLanguage() {
      return instance.resolvedLanguage;
    },
    get isInitialized() {
      return instance.isInitialized;
    },
  };
}

function initializeUiI18n(): UiI18nPort {
  const requestedLocale = resolveRequestedUiLocale();

  if (i18next.isInitialized) {
    registerUiChromeTranslationBundles();
    if (i18next.language !== requestedLocale) {
      void i18next.changeLanguage(requestedLocale);
    }
    return createUiI18nPort(i18next);
  }

  i18next.use(initReactI18next);

  void i18next.init({
    resources: uiChromeTranslationBundles,
    fallbackLng: "en",
    supportedLngs: ["en", "de"],
    nonExplicitSupportedLngs: true,
    lng: requestedLocale,
    showSupportNotice: false,
    returnObjects: true,
    initImmediate: false,
    interpolation: {
      escapeValue: false,
    },
    react: {
      useSuspense: false,
      bindI18n: "languageChanged",
      bindI18nStore: "added removed",
    },
  });

  return createUiI18nPort(i18next);
}

/** @emoji 🪁️ Shared UI i18n port (domain-neutral bundles; extend via {@link registerUiTranslationBundles}). */
export const uiI18n = initializeUiI18n();

/** @emoji 🪁️ Sets the active UI locale on the shared i18n port (user-initiated, in-app switch —
 * for boot-time/brand-locked locale resolution, use {@link initUiLocaleSync} instead, which runs
 * before the first render rather than in a post-paint effect). */
export function setUiLocale(locale: UiLocale): Promise<unknown> {
  if (typeof document !== "undefined") document.documentElement.lang = locale;
  return uiI18n.changeLanguage(locale);
}

/** @emoji 🚀️ Resolves the shell's locale synchronously, before the first React render — call this
 * at renderer/demonstrator boot (module scope or before `ReactDOM.createRoot(...).render(...)`),
 * never from a `useEffect`. A `useEffect`-based call runs after the first paint has already
 * committed, which is exactly how a German-locked brand could still flash English chrome
 * ("Skip"/"Back"/"Next"/"Done") on first load. Persists the locale (so a reload's
 * `resolveRequestedUiLocale` agrees) and sets `documentElement.lang` synchronously; also nudges the
 * already-initialized i18next instance in case this runs after `uiI18n`'s own module-load default
 * resolved differently. */
export function initUiLocaleSync(locale: ShellLocale): void {
  writeStoredUiChromeLocale(createBrowserStoragePort(), locale);
  if (typeof document !== "undefined") document.documentElement.lang = locale;
  if (i18next.language !== locale) void i18next.changeLanguage(locale);
}

// #endregion 🔌️I18n Port
