import mapping from "./🔣️.json";

export const PLAYGROUND_LOCKED_EXAMPLE_ENV = "PLAYGROUND_LOCKED_EXAMPLE_ID";
export const SEMIO_LOCKED_LOCALE_ENV = "SEMIO_LOCKED_LOCALE";
export const SEMIO_LOCKED_TERMINOLOGY_ENV = "SEMIO_LOCKED_TERMINOLOGY";
export const SEMIO_LOCKED_THEME_ENV = "SEMIO_LOCKED_THEME";
export const SEMIO_LOCKED_APPEARANCE_ENV = "SEMIO_LOCKED_APPEARANCE";
export const SEMIO_BRAND_ENV = "SEMIO_BRAND";
export const SEMIO_DEFAULT_EXAMPLE_ENV = "SEMIO_DEFAULT_EXAMPLE";

/** 🔒️ Projects explicitly set shell preferences into Vite's public environment. */
export function frameworkOsLockedPrefsEnv(env: Readonly<Record<string, string | undefined>> = process.env): Record<string, string> {
  return Object.fromEntries(Object.entries(mapping).flatMap(([source, target]) => {
    const value = env[source]?.trim();
    return value ? [[target, value]] : [];
  }));
}
