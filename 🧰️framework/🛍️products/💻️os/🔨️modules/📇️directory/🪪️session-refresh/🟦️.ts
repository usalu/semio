import { parseDirectorySessionAuthorityJsonV1, type DirectorySessionAuthorityV1 } from "../🧬️schema/🪪️session-authority-v1/🟦️.ts";

export const DIRECTORY_SESSION_REFRESH_INTERVAL_MS = 5_000;
export const DIRECTORY_SESSION_AUTHORITY_TEXT_V1 = {
  en: { pending: "Verifying shared access…", cancel: "Cancel shared access", unavailable: "Shared access is unavailable. Open a fresh authorized session from the secure launcher." },
  de: { pending: "Gemeinsamen Zugriff prüfen…", cancel: "Prüfung abbrechen", unavailable: "Der gemeinsame Zugriff ist nicht verfügbar. Öffne über den sicheren Starter eine neu autorisierte Sitzung." },
} as const;

export type DirectorySessionAuthorityLocaleV1 = keyof typeof DIRECTORY_SESSION_AUTHORITY_TEXT_V1;

/** 🔡️ Refuses unowned locales instead of silently selecting a default language. */
export function directorySessionAuthorityTextV1(locale: string): (typeof DIRECTORY_SESSION_AUTHORITY_TEXT_V1)[DirectorySessionAuthorityLocaleV1] {
  if (locale !== "en" && locale !== "de") throw new Error("directory.session-authority.locale-unsupported");
  return DIRECTORY_SESSION_AUTHORITY_TEXT_V1[locale];
}

export interface DirectorySessionRefreshV1 {
  refresh(): Promise<void>;
  close(): void;
}

/** 🧷️ Keeps an action bound to the same verified unexpired session across refreshes. */
export function directorySessionAuthorityIsCurrentV1(captured: DirectorySessionAuthorityV1 | null, current: DirectorySessionAuthorityV1 | null): boolean {
  return captured !== null && current !== null && captured.expiresAt > Date.now() && current.expiresAt > Date.now()
    && captured.sessionBindingSha256 === current.sessionBindingSha256 && captured.authorizationGeneration === current.authorizationGeneration;
}

/** 🪪️ Revalidates one broker owner serially until refusal, cancellation or replacement. */
export function startDirectorySessionRefreshV1(options: {
  readonly signal?: AbortSignal;
  readonly read: (signal: AbortSignal) => Promise<Readonly<{ status: number; body: string }>>;
  readonly onAuthority: (authority: DirectorySessionAuthorityV1) => void;
  readonly onUnavailable: () => void;
}): DirectorySessionRefreshV1 {
  const abort = new AbortController();
  let closed = false;
  let inFlight: Promise<void> | null = null;
  let timer: ReturnType<typeof setTimeout> | null = null;
  const close = (): void => {
    if (closed) return;
    closed = true;
    if (timer !== null) clearTimeout(timer);
    timer = null;
    options.signal?.removeEventListener("abort", close);
    abort.abort();
  };
  const refresh = (): Promise<void> => {
    if (closed) return Promise.resolve();
    if (inFlight !== null) return inFlight;
    if (timer !== null) clearTimeout(timer);
    timer = null;
    inFlight = (async () => {
      try {
        const response = await Promise.resolve().then(() => {
          if (closed) throw new Error("directory.session-authority.cancelled");
          return options.read(abort.signal);
        });
        if (closed) return;
        if (response.status !== 200) throw new Error("directory.session-authority.unavailable");
        const authority = parseDirectorySessionAuthorityJsonV1(response.body);
        if (authority.expiresAt <= Date.now()) throw new Error("directory.session-authority.expired");
        options.onAuthority(authority);
        if (!closed) timer = setTimeout(() => { void refresh(); }, DIRECTORY_SESSION_REFRESH_INTERVAL_MS);
      } catch {
        if (closed) return;
        close();
        options.onUnavailable();
      } finally {
        inFlight = null;
      }
    })();
    return inFlight;
  };
  options.signal?.addEventListener("abort", close, { once: true });
  if (options.signal?.aborted) close();
  else void refresh();
  return { refresh, close };
}
