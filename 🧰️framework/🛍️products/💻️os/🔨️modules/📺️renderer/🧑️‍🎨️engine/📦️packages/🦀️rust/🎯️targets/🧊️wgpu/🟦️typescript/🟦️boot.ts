// #region 🧲️Header
/** @emoji 🧊️ Trunk boot glue — loads wasm programs and starts the wgpu renderer. */
// #endregion 🧲️Header

import { pluginGraphErrorMessage, type PluginRegistryEntry, type ShellLocale } from "@semio-tech/framework";
import { resolvePlaygroundBoot } from "@semio-tech/framework";
import { PLUGIN_CATALOG } from "../../../../../../../🔌️plugin/📇️registry/🟦️catalog.ts";
import { loadPluginModule, pluginHandleForBridge } from "./🐚️plugin-bridge.ts";

await new Promise<void>((resolve) => {
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", () => resolve(), { once: true });
  } else {
    resolve();
  }
});

// 🛠️ Named (not `typeof handles`) so `semioWgpuMount`'s cast below doesn't reference the very
// variable it types — TS flags that self-reference (TS2502) once the `handles` array's element type
// itself involves a `ReturnType<...>` (see `pluginHandleForBridge`'s exported `WgpuJsBridge` return
// type in `🐚️plugin-bridge.ts`).
type WgpuBootPluginEntry = { readonly pluginId: string; readonly handle: ReturnType<typeof pluginHandleForBridge> };

const bootVariant = new URLSearchParams(window.location.search).get("plugin") ?? "s";
const boot = resolvePlaygroundBoot(PLUGIN_CATALOG, bootVariant);
const pluginTargets: PluginRegistryEntry[] = boot.plugins.map((entry) => ({
  pluginId: entry.pluginId,
  moduleUrl: entry.moduleUrl,
  contributes: entry.contributes,
  consumes: entry.consumes,
}));
const pluginFilter = boot.variant;

async function pluginModuleAvailable(moduleUrl: string): Promise<boolean> {
  try {
    const response = await fetch(moduleUrl, { method: "HEAD" });
    return response.ok;
  } catch {
    return false;
  }
}

function renderBootErrorBanner(message: string): void {
  console.error(`[DEBUG] wgpu boot failed: ${message}`);
  const root = document.getElementById("root");
  if (!root) return;
  const banner = document.createElement("div");
  banner.style.cssText = "position:fixed;inset:0;padding:24px;background:#2a0a0a;color:#ffb4b4;font-family:monospace;font-size:14px;white-space:pre-wrap;overflow:auto;z-index:9999;";
  banner.textContent = `wgpu renderer boot failed:\n\n${message}`;
  root.appendChild(banner);
}

/** 🌐️ No shell locale selector exists this early in boot (before any app/config has loaded) —
 * `navigator.language` is the best signal available, English/German only per the repo's
 * `ShellLocale` axis. */
function resolveBootLocale(): ShellLocale {
  return typeof navigator !== "undefined" && navigator.language.toLowerCase().startsWith("de") ? "de" : "en";
}

/** 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §5: `VITE_SEMIO_APP_ROLE`,
 * values `"viewer"`/`"editor"`, default `"editor"`. This target is Trunk-served (`Trunk.toml`), not
 * Vite-bundled, so `import.meta.env.VITE_SEMIO_APP_ROLE` is read defensively (a harmless `undefined`
 * unless a deployment wraps this boot module through a Vite dev server) — a `?plugin=`-style URL
 * param is the always-available fallback for this shell, mirroring `bootVariant`'s own
 * `URLSearchParams` idiom a few lines below. */
function resolveBootAppRole(): string {
  const viteEnv = (import.meta as unknown as { env?: Record<string, string | undefined> }).env?.VITE_SEMIO_APP_ROLE;
  const urlRole = new URLSearchParams(window.location.search).get("role") ?? undefined;
  return viteEnv === "viewer" || urlRole === "viewer" ? "viewer" : "editor";
}

/** 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0 — `S_HUB_URL`/
 * `S_USER`/`S_DATA_DIR` for the browser wgpu build. Same defensive-read posture as
 * `resolveBootAppRole` right above (this target is Trunk-served, not Vite-bundled, so
 * `import.meta.env.VITE_S_*` only resolves when a deployment wraps this boot module through a Vite
 * dev server) with `?hub=`/`?user=`/`?dataDir=` URL-param fallbacks mirroring `resolveBootAppRole`'s
 * own `?role=` idiom. `undefined` hub url ⇒ no hub env at all ⇒ `semioWgpuSetHubEnv` is never called
 * ⇒ the Rust side's `resolve_identity_env` stays `None` ⇒ unchanged local-only behaviour. */
function resolveBootHubEnv(): { hubUrl: string; user: string; dataDir: string } | undefined {
  const viteEnv = (import.meta as unknown as { env?: Record<string, string | undefined> }).env;
  const params = new URLSearchParams(window.location.search);
  const hubUrl = viteEnv?.VITE_S_HUB_URL ?? params.get("hub") ?? undefined;
  if (!hubUrl) return undefined;
  const user = viteEnv?.VITE_S_USER ?? params.get("user") ?? "";
  const dataDir = viteEnv?.VITE_S_DATA_DIR ?? params.get("dataDir") ?? "";
  return { hubUrl, user, dataDir };
}

/** 🌐️ Surfaces a missing/incompatible plugin dependency (ticket
 * 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS) as a real, localized
 * banner instead of only a console error — non-fatal, since `boot.plugins` already excludes the
 * blocked entries and every OTHER plugin still boots (contract freeze §4 rule 5's fail-soft posture). */
function renderDependencyFaultBanner(messages: readonly string[]): void {
  const root = document.getElementById("root");
  if (!root) return;
  const banner = document.createElement("div");
  banner.style.cssText = "position:fixed;top:0;left:0;right:0;padding:12px 24px;background:#4a2a00;color:#ffd8a8;font-family:monospace;font-size:13px;white-space:pre-wrap;z-index:9998;";
  banner.textContent = messages.join("\n");
  root.appendChild(banner);
}

if (boot.dependencyErrors.length > 0) {
  const locale = resolveBootLocale();
  const messages = boot.dependencyErrors.map((error) => pluginGraphErrorMessage(error, locale));
  for (const message of messages) console.error(`[DEBUG] plugin dependency fault: ${message}`);
  renderDependencyFaultBanner(messages);
}

try {
  const availableTargets: PluginRegistryEntry[] = [];
  for (const entry of pluginTargets) {
    if (await pluginModuleAvailable(entry.moduleUrl)) {
      availableTargets.push(entry);
    }
  }
  if (availableTargets.length === 0) {
    throw new Error(`[DEBUG] no wasm plugin modules found for filter ${pluginFilter}`);
  }

  // 🎯️ Loaded SEQUENTIALLY, in `boot.plugins`'s already dependency-ordered sequence (scout-2 §4:
  // "boot must walk the dependency order... instead of relying on array order") — a concurrent
  // `Promise.all` gives no guarantee a dependency finishes loading before its dependent starts.
  const handles: WgpuBootPluginEntry[] = [];
  for (const entry of availableTargets) {
    handles.push({ pluginId: entry.pluginId, handle: pluginHandleForBridge(await loadPluginModule(entry.pluginId, entry.moduleUrl)) });
  }

  const bindings = await new Promise<Record<string, unknown>>((resolve, reject) => {
    const host = window as { wasmBindings?: Record<string, unknown> };
    const finish = () => {
      if (!host.wasmBindings) {
        reject(new Error("[DEBUG] trunk wasm bindings missing"));
        return;
      }
      resolve(host.wasmBindings);
    };
    if (host.wasmBindings) {
      finish();
      return;
    }
    const timeout = window.setTimeout(() => reject(new Error("[DEBUG] trunk wasm bindings timeout")), 30000);
    const done = () => {
      window.clearTimeout(timeout);
      window.clearInterval(poll);
      finish();
    };
    window.addEventListener("TrunkApplicationStarted", done, { once: true });
    const poll = window.setInterval(() => {
      if (host.wasmBindings) done();
    }, 50);
  });

  if (!bindings.semioWgpuMount) throw new Error("[DEBUG] missing semioWgpuMount");
  const root = document.getElementById("root");
  if (!root) throw new Error("[DEBUG] missing #root");
  const canvas = document.createElement("canvas");
  canvas.style.display = "block";
  canvas.style.width = "100%";
  canvas.style.height = "100%";
  canvas.style.touchAction = "none";
  canvas.style.outline = "none";
  root.replaceChildren(canvas);
  // 👁️✏️ Contract freeze §5: boot role, applied before mount so the very first `Shell::set_window_layout`
  // already carries it. Guarded — `semioWgpuSetAppRole` is new (this ticket) and a stale wasm build
  // predating it simply skips role chrome rather than throwing, same fail-soft posture as every other
  // optional binding this file checks.
  if (bindings.semioWgpuSetAppRole) {
    (bindings.semioWgpuSetAppRole as (role: string) => void)(resolveBootAppRole());
  }
  // 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0/§1 — same guarded,
  // fail-soft posture as `semioWgpuSetAppRole` right above: a stale wasm build predating this ticket
  // simply skips identity/directory wiring rather than throwing.
  const hubEnv = resolveBootHubEnv();
  if (hubEnv && bindings.semioWgpuSetHubEnv) {
    (bindings.semioWgpuSetHubEnv as (hubUrl: string, user: string, dataDir: string) => void)(hubEnv.hubUrl, hubEnv.user, hubEnv.dataDir);
  }
  (bindings.semioWgpuMount as (canvas: HTMLCanvasElement, handles: readonly WgpuBootPluginEntry[], pluginFilter: string) => void)(canvas, handles, pluginFilter);
} catch (error) {
  renderBootErrorBanner(error instanceof Error ? error.message : String(error));
  throw error;
}
