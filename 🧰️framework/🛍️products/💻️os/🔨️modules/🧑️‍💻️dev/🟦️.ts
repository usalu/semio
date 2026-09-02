// #region 🧲️Header
/** @emoji 🖥️ OS dev runner — boots the Rust program framework with a selectable renderer. */
// #endregion 🧲️Header

import "./🎨️.css";

// [DEBUG] temporary shard turn probe
{
  const proto = Worker.prototype as unknown as { postMessage: (m: unknown, t?: unknown) => void };
  const original = proto.postMessage;
  proto.postMessage = function (message: unknown, transfer?: unknown) {
    try {
      const record = message as { kind?: string; events?: readonly { kind?: string; payload?: object }[] };
      if (record && record.kind === "turn" && Array.isArray(record.events) && record.events.length > 0) {
        console.error("[DEBUG] outgoing turn", JSON.stringify(record.events.map((event) => ({ k: event.kind, keys: Object.keys(event.payload ?? {}).join("|") }))).slice(0, 500), "STACK", String(new Error().stack).split("\n").slice(1, 8).join(" <- "));
      }
    } catch {}
    return transfer === undefined ? original.call(this, message) : (original as (m: unknown, t: unknown) => void).call(this, message, transfer);
  };
}

export type { PluginBuildTarget } from "../🔌️plugin/📇️registry/🤖️generated/🟦️plugins.ts";
export { PLUGIN_BUILD_TARGETS, EXTENSION_TARGETS, PROGRAM_TARGETS, pluginModuleUrl, extensionModuleUrl } from "../🔌️plugin/📇️registry/🤖️generated/🟦️plugins.ts";
export { PLAYGROUND_SESSION } from "./🤖️generated/🟦️session.ts";

import { resolvePlaygroundBoot } from "@semio-tech/framework";
import { PUZZLE_BOARD_SESSION_FACTORIES } from "@semio-tech/puzzle-js";
import { PLUGIN_CATALOG } from "../🔌️plugin/📇️registry/🟦️.ts";
import { PLAYGROUND_SESSION } from "./🤖️generated/🟦️session.ts";
import { resolveShellBrandById } from "./🏷️brand/🟦️.ts";

const renderer = import.meta.env.VITE_SEMIO_RENDERER ?? import.meta.env.SEMIO_RENDERER ?? "react";
const boot = resolvePlaygroundBoot(PLUGIN_CATALOG, import.meta.env.VITE_SEMIO_PLUGIN || PLAYGROUND_SESSION.variant, PLAYGROUND_SESSION);
const pluginFilter = boot.variant;
const appId = import.meta.env.VITE_SEMIO_APP_ID ?? boot.defaultAppId;

/** @emoji 👁️✏️ Boot-time surface role (contract §5): `VITE_SEMIO_APP_ROLE` is `"viewer"`|`"editor"`,
 * default `"editor"` — dev boots the editor unless the env var explicitly asks for the viewer. Mirrors
 * `resolveBootAppRole`'s own validation so an unrecognized value falls back rather than throwing. */
const appRole: "viewer" | "editor" = import.meta.env.VITE_SEMIO_APP_ROLE === "viewer" ? "viewer" : "editor";

/** @emoji 🏷️ Baked-in shell brand for this artifact (registry `brand` column or `SEMIO_BRAND`); no `?query=` override. */
const brand = resolveShellBrandById(import.meta.env.VITE_SEMIO_BRAND || undefined);

/** @emoji 🔒️ Boot-time-only shell preference locks; unlike `program`, these have no `?query=` override. */
const locks = {
  exampleId: import.meta.env.VITE_SEMIO_LOCKED_EXAMPLE || undefined,
  locale: import.meta.env.VITE_SEMIO_LOCKED_LOCALE || undefined,
  terminology: import.meta.env.VITE_SEMIO_LOCKED_TERMINOLOGY || undefined,
  themeId: import.meta.env.VITE_SEMIO_LOCKED_THEME || undefined,
  appearance: import.meta.env.VITE_SEMIO_LOCKED_APPEARANCE || undefined,
};

/** @emoji 🎛️ Boot-time shell preference defaults — seed values that keep their in-app switcher visible. */
const defaults = {
  exampleId: import.meta.env.VITE_SEMIO_DEFAULT_EXAMPLE || undefined,
};

if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest) {
  const plugins = boot.plugins;
  if (renderer !== "wgpu") {
    const { bootFrameworkOs } = await import("@semio-tech/framework-renderer-react");
    void bootFrameworkOs({ plugin: pluginFilter, plugins, surfaceSessionFactories: PUZZLE_BOARD_SESSION_FACTORIES, appId, appRole, locks, defaults, brand }).catch((error) => {
      console.error("[DEBUG] os-dev react boot failed", error);
    });
  }
}
