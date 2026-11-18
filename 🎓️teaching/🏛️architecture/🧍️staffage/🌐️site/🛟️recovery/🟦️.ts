/** 🛟️ Recovery of a tab opened before a redeploy: the CDN no longer serves the hashed chunks its document names, so the first failed chunk load reloads the page once for the current document. A second failure within {@link STALE_CHUNK_RELOAD_WINDOW_MS} of that reload, or a page without session storage, keeps the failure for the page's own retry, so the page never loops.
 * @see https://vite.dev/guide/build#load-error-handling — `vite:preloadError` */

/** 🔑️ Session storage key holding the time of the last recovery reload. */
export const STALE_CHUNK_RELOAD_KEY = "staffage.stale-chunk-reload";

/** ⏲️ How long after a recovery reload a further failure is taken as a real outage, not a stale document. */
export const STALE_CHUNK_RELOAD_WINDOW_MS = 60_000;

/** 🧩️ What the recovery needs from the page. */
export type StaleChunkHost = { readonly target: EventTarget; readonly storage: () => Pick<Storage, "getItem" | "setItem">; readonly reload: () => void; readonly now: () => number };

/** 🔁️ Listens for `vite:preloadError` on `host.target` and reloads at most once per window. */
export function recoverStaleChunks(host: StaleChunkHost): void {
  host.target.addEventListener("vite:preloadError", (event) => {
    try {
      const storage = host.storage();
      const last = Number(storage.getItem(STALE_CHUNK_RELOAD_KEY) ?? Number.NEGATIVE_INFINITY);
      if (host.now() - last < STALE_CHUNK_RELOAD_WINDOW_MS) return;
      storage.setItem(STALE_CHUNK_RELOAD_KEY, String(host.now()));
    } catch {
      return;
    }
    event.preventDefault();
    host.reload();
  });
}
