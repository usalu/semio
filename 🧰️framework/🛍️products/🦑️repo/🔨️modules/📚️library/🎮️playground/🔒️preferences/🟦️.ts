import mapping from "./🔣️.json";

export const PLAYGROUND_LOCKED_EXAMPLE_ENV = "PLAYGROUND_LOCKED_EXAMPLE_ID";
export const SEMIO_LOCKED_LOCALE_ENV = "SEMIO_LOCKED_LOCALE";
export const SEMIO_LOCKED_TERMINOLOGY_ENV = "SEMIO_LOCKED_TERMINOLOGY";
export const SEMIO_LOCKED_THEME_ENV = "SEMIO_LOCKED_THEME";
export const SEMIO_LOCKED_APPEARANCE_ENV = "SEMIO_LOCKED_APPEARANCE";
export const SEMIO_BRAND_ENV = "SEMIO_BRAND";
export const SEMIO_DEFAULT_EXAMPLE_ENV = "SEMIO_DEFAULT_EXAMPLE";
/** 👁️✏️ Boot-time surface role. The shells read `VITE_SEMIO_APP_ROLE` (`🐚️Shell/🟦️.tsx`'s
 * `resolveBootAppRole`, `🧑‍💻dev/🟦️.ts`'s `appRole`), so without this projection the ONLY way to open a
 * plugin's viewer surface was to hand-export the `VITE_`-prefixed name before starting the dev server —
 * a launch row could not reach it, and no artifact's read-only surface was reachable from any launch
 * entry (`📓️audit-window-inventory-2026-09-12.md` §4 P0 item 2). */
export const SEMIO_APP_ROLE_ENV = "SEMIO_APP_ROLE";

/** 🔒️ Projects explicitly set shell preferences into Vite's public environment. */
export function frameworkOsLockedPrefsEnv(env: Readonly<Record<string, string | undefined>> = process.env): Record<string, string> {
  return Object.fromEntries(Object.entries(mapping).flatMap(([source, target]) => {
    const value = env[source]?.trim();
    return value ? [[target, value]] : [];
  }));
}
