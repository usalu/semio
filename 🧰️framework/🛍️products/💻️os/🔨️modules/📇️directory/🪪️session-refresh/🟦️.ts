import { parseDirectorySessionAuthorityJsonV1, type DirectorySessionAuthorityV1 } from "../🧬️schema/🪪️session-authority-v1/🟦️.ts";

export const DIRECTORY_SESSION_REFRESH_INTERVAL_MS = 5_000;
/** ⏱️ The first retry delay after a TRANSIENT revalidation failure, doubling per consecutive failure up
 * to `DIRECTORY_SESSION_REFRESH_RETRY_CEILING_MS`. Short enough that a link which returns within a
 * second is caught almost immediately, bounded so a hub that stays down is not hammered. */
export const DIRECTORY_SESSION_REFRESH_RETRY_BASE_MS = 500;
export const DIRECTORY_SESSION_REFRESH_RETRY_CEILING_MS = 5_000;
/** 🚫️ The only answers that mean THIS SESSION is not valid any more. Everything else a revalidation can
 * return — a transport throw, a gateway status, a proxy's HTML error page — says nothing about the
 * session and everything about the link, so it is retried rather than believed. */
export const DIRECTORY_SESSION_AUTHORITY_REFUSAL_STATUSES: readonly number[] = [401, 403];
export const DIRECTORY_SESSION_AUTHORITY_TEXT_V1 = {
  en: { pending: "Verifying shared access…", cancel: "Cancel shared access", unavailable: "Shared access is unavailable. Open a fresh authorized session from the secure launcher." },
  de: { pending: "Gemeinsamen Zugriff prüfen…", cancel: "Prüfung abbrechen", unavailable: "Der gemeinsame Zugriff ist nicht verfügbar. Öffne über den sicheren Starter eine neu autorisierte Sitzung." },
} as const;

export type DirectorySessionAuthorityLocaleV1 = keyof typeof DIRECTORY_SESSION_AUTHORITY_TEXT_V1;

/** 🔡️ Narrows an untyped locale tag to an owned one, refusing instead of selecting a default language. */
export function parseDirectorySessionAuthorityLocaleV1(locale: string): DirectorySessionAuthorityLocaleV1 {
  if (locale !== "en" && locale !== "de") throw new Error("directory.session-authority.locale-unsupported");
  return locale;
}

/** 🔡️ Refuses unowned locales instead of silently selecting a default language. */
export function directorySessionAuthorityTextV1(locale: string): (typeof DIRECTORY_SESSION_AUTHORITY_TEXT_V1)[DirectorySessionAuthorityLocaleV1] {
  return DIRECTORY_SESSION_AUTHORITY_TEXT_V1[parseDirectorySessionAuthorityLocaleV1(locale)];
}

export interface DirectorySessionRefreshV1 {
  refresh(): Promise<void>;
  close(): void;
}

/** ⏱️ The delay before the n-th consecutive transient retry (`attempt` counts from 1). */
export function directorySessionRefreshRetryDelayMsV1(attempt: number): number {
  if (!Number.isInteger(attempt) || attempt < 1) throw new Error("directory.session-authority.retry-attempt-invalid");
  return Math.min(DIRECTORY_SESSION_REFRESH_RETRY_CEILING_MS, DIRECTORY_SESSION_REFRESH_RETRY_BASE_MS * 2 ** (attempt - 1));
}

/** 🚫️ Marks a revalidation failure the hub itself pronounced on: the session is gone and retrying it
 * would only ask the same question again. Thrown internally, never crossing the module boundary. */
class DirectorySessionAuthorityRefusedV1 extends Error {}

/** 🧷️ Keeps an action bound to the same verified unexpired session across refreshes. */
export function directorySessionAuthorityIsCurrentV1(captured: DirectorySessionAuthorityV1 | null, current: DirectorySessionAuthorityV1 | null): boolean {
  return captured !== null && current !== null && captured.expiresAt > Date.now() && current.expiresAt > Date.now()
    && captured.sessionBindingSha256 === current.sessionBindingSha256 && captured.authorizationGeneration === current.authorizationGeneration;
}

/** 🪪️ Revalidates one session owner serially until refusal, expiry, cancellation or replacement.
 *
 * A failed revalidation is NOT a refusal. Before ticket `26/09/18`'s C1c slice any single non-200 —
 * including a transport throw and a dev proxy's own error page — closed this loop for ever and retired
 * the shell's authority, so the shell was signed out by a hiccup it should have ridden out. It was
 * observed live: a second browser context signing in made the shared dev server queue the first shell's
 * `GET /auth/sessions/me` past its deadline, and that shell never held hub authority again. Only
 * `DIRECTORY_SESSION_AUTHORITY_REFUSAL_STATUSES` and an expired authority are answers ABOUT the
 * session; everything else is answered by retrying on a bounded backoff while the last authority this
 * loop observed is still unexpired — which is what lets a short connection loss leave the app working
 * and catch up when the link returns. `onDegraded` reports the transient window's edges so a shell can
 * say it is offline without tearing down a session that is still valid. */
export function startDirectorySessionRefreshV1(options: {
  readonly signal?: AbortSignal;
  readonly read: (signal: AbortSignal) => Promise<Readonly<{ status: number; body: string }>>;
  readonly onAuthority: (authority: DirectorySessionAuthorityV1) => void;
  readonly onUnavailable: () => void;
  readonly onDegraded?: (degraded: boolean) => void;
}): DirectorySessionRefreshV1 {
  const abort = new AbortController();
  let closed = false;
  let inFlight: Promise<void> | null = null;
  let timer: ReturnType<typeof setTimeout> | null = null;
  let consecutiveTransientFailures = 0;
  let degraded = false;
  let observedExpiresAt: number | null = null;
  const close = (): void => {
    if (closed) return;
    closed = true;
    if (timer !== null) clearTimeout(timer);
    timer = null;
    options.signal?.removeEventListener("abort", close);
    abort.abort();
  };
  const reportDegraded = (next: boolean): void => {
    if (degraded === next) return;
    degraded = next;
    options.onDegraded?.(next);
  };
  const refresh = (): Promise<void> => {
    if (closed) return Promise.resolve();
    if (inFlight !== null) return inFlight;
    if (timer !== null) clearTimeout(timer);
    timer = null;
    inFlight = (async () => {
      try {
        const response = await Promise.resolve().then(() => {
          if (closed) throw new DirectorySessionAuthorityRefusedV1("directory.session-authority.cancelled");
          return options.read(abort.signal);
        });
        if (closed) return;
        if (DIRECTORY_SESSION_AUTHORITY_REFUSAL_STATUSES.includes(response.status)) throw new DirectorySessionAuthorityRefusedV1("directory.session-authority.refused");
        if (response.status !== 200) throw new Error("directory.session-authority.unreachable");
        const authority = parseDirectorySessionAuthorityJsonV1(response.body);
        if (authority.expiresAt <= Date.now()) throw new DirectorySessionAuthorityRefusedV1("directory.session-authority.expired");
        consecutiveTransientFailures = 0;
        observedExpiresAt = authority.expiresAt;
        reportDegraded(false);
        options.onAuthority(authority);
        if (!closed) timer = setTimeout(() => { void refresh(); }, DIRECTORY_SESSION_REFRESH_INTERVAL_MS);
      } catch (error) {
        if (closed) return;
        if (error instanceof DirectorySessionAuthorityRefusedV1 || (observedExpiresAt !== null && observedExpiresAt <= Date.now())) {
          close();
          options.onUnavailable();
          return;
        }
        consecutiveTransientFailures += 1;
        reportDegraded(true);
        timer = setTimeout(() => { void refresh(); }, directorySessionRefreshRetryDelayMsV1(consecutiveTransientFailures));
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
