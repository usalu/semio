/** 🔑️ Session storage key holding the last recovery navigation and its original pathname. */
export const STALE_CHUNK_RELOAD_KEY = "staffage.stale-chunk-reload";

/** 🎬️ The hashed entry script in the generated release document. */
export const STALE_CHUNK_ENTRY_ID = "staffage-entry";

/** 🟢️ The archive entry dispatches this after it mounts successfully. */
export const STALE_CHUNK_MOUNTED_EVENT = "staffage:archiveMounted";

/** 🧭️ Reserved path prefix for fresh copies of the release document served through the static 404 alias. */
export const STALE_CHUNK_RECOVERY_PATH_PREFIX = "/__staffage_recovery-";

/** 🛟️ Inline release script that runs before the hashed entry. The static host serves the archive document for the unique recovery pathname, with a fresh cache key. Keep the query and fragment intact and restore the original pathname after mounting.
 * @see https://vite.dev/guide/build#load-error-handling */
export const STALE_CHUNK_BOOT_SCRIPT = `(() => {
  const key = "${STALE_CHUNK_RELOAD_KEY}";
  const prefix = "${STALE_CHUNK_RECOVERY_PATH_PREFIX}";
  function read(storage) {
    const raw = storage.getItem(key);
    try {
      const state = JSON.parse(raw ?? "null");
      if (Number.isFinite(state?.started) && typeof state.recoveryPath === "string" && state.recoveryPath.startsWith(prefix) && typeof state.returnPath === "string" && state.returnPath.startsWith("/") && !state.returnPath.startsWith(prefix)) return state;
    } catch {}
    return null;
  }
  function recover(event) {
    try {
      const storage = window.sessionStorage;
      const last = read(storage);
      const started = Date.now();
      if (last !== null && started - last.started < 60000) return;
      const url = new URL(window.location.href);
      const alreadyRecovering = url.pathname.startsWith(prefix);
      if (alreadyRecovering && last?.recoveryPath !== url.pathname) return;
      const returnPath = alreadyRecovering ? last.returnPath : url.pathname;
      const recoveryPath = prefix + started + "-" + window.crypto.randomUUID();
      url.pathname = recoveryPath;
      storage.setItem(key, JSON.stringify({ started, recoveryPath, returnPath }));
      window.location.replace(url.href);
      event.preventDefault();
    } catch {}
  }
  window.addEventListener("error", (event) => {
    const script = event.target;
    if (script?.id === "${STALE_CHUNK_ENTRY_ID}" && script.tagName === "SCRIPT" && script.type === "module") recover(event);
  }, true);
  window.addEventListener("vite:preloadError", recover);
  window.addEventListener("${STALE_CHUNK_MOUNTED_EVENT}", () => {
    try {
      const url = new URL(window.location.href);
      if (!url.pathname.startsWith(prefix)) return;
      const last = read(window.sessionStorage);
      if (last?.recoveryPath !== url.pathname) return;
      url.pathname = last.returnPath;
      window.history.replaceState(window.history.state, "", url.href);
    } catch {}
  }, { once: true });
})();`;
