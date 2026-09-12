// #region 🧲️Header
/** @emoji 🖥️ OS dev runner — boots the Rust program framework with a selectable renderer. */
// #endregion 🧲️Header

import "./🎨️.css";

export type { PluginBuildTarget } from "../🔌️plugin/📇️registry/🤖️generated/🧩️plugins.ts";
export { PLUGIN_BUILD_TARGETS, EXTENSION_TARGETS, PROGRAM_TARGETS, pluginModuleUrl, extensionModuleUrl } from "../🔌️plugin/📇️registry/🤖️generated/🧩️plugins.ts";
export { PLAYGROUND_SESSION } from "virtual:semio-playground-session";

import type { AppRole } from "@semio-tech/framework";
import { resolvePlaygroundBoot } from "@semio-tech/framework";
import { PUZZLE_BOARD_SESSION_FACTORIES } from "@semio-tech/puzzle-js";
import { PLUGIN_CATALOG } from "../🔌️plugin/📇️registry/🟦️.ts";
import { PLAYGROUND_SESSION } from "virtual:semio-playground-session";
import { resolveShellBrandById } from "./🏷️brand/🟦️.ts";
import { resolveBootQueryAppRole } from "./🔗️boot-query/🟦️.ts";

const renderer = import.meta.env.VITE_SEMIO_RENDERER ?? import.meta.env.SEMIO_RENDERER ?? "react";
const boot = resolvePlaygroundBoot(PLUGIN_CATALOG, import.meta.env.VITE_SEMIO_PLUGIN || PLAYGROUND_SESSION.variant, PLAYGROUND_SESSION);
const pluginFilter = boot.variant;
const appId = import.meta.env.VITE_SEMIO_APP_ID ?? boot.defaultAppId;

/** @emoji 👁️✏️ Per-server default surface role (contract §5): `VITE_SEMIO_APP_ROLE` is
 * `"viewer"`|`"editor"`, default `"editor"`. Mirrors `resolveBootAppRole`'s own validation so an
 * unrecognized value falls back rather than throwing. */
const envAppRole: AppRole = import.meta.env.VITE_SEMIO_APP_ROLE === "viewer" ? "viewer" : "editor";

/** @emoji 👁️✏️ Per-navigation surface role: `?role=viewer`/`?role=editor` wins over the env default, so
 * `http://127.0.0.1:6018/?plugin=<variant>&role=viewer` opens the viewer without restarting the server —
 * the React half of the `?role=` axis the wgpu browser boot already reads. */
const appRole: AppRole = typeof window === "undefined" ? envAppRole : resolveBootQueryAppRole(window.location.search, envAppRole);

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
