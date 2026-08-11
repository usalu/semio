// #region 🧲️Header
/** @emoji 🖥️ OS dev runner — boots the Rust program framework with a selectable renderer. */
// #endregion 🧲️Header

import "./🎨️globals.css";

export type { PluginBuildTarget } from "../🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🟦️plugins.ts";
export { PLUGIN_BUILD_TARGETS, EXTENSION_TARGETS, PROGRAM_TARGETS, pluginModuleUrl, extensionModuleUrl } from "../🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🟦️plugins.ts";
export { PLAYGROUND_SESSION } from "./🤖️generated/🟦️session.ts";

import { resolvePlaygroundBoot } from "@semio-tech/framework";
import { PLUGIN_CATALOG } from "../🔌️plugin/📦️packages/🟦️typescript/📇️registry/🟦️catalog.ts";
import { PLAYGROUND_SESSION } from "./🤖️generated/🟦️session.ts";
import { resolveShellBrandById } from "./🏷️brand/📦️index.ts";

const renderer = import.meta.env.VITE_SEMIO_RENDERER ?? import.meta.env.SEMIO_RENDERER ?? "react";
const boot = resolvePlaygroundBoot(PLUGIN_CATALOG, import.meta.env.VITE_SEMIO_PLUGIN || PLAYGROUND_SESSION.variant, PLAYGROUND_SESSION);
const pluginFilter = boot.variant;
const appId = import.meta.env.VITE_SEMIO_APP_ID ?? boot.defaultAppId;

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
    void bootFrameworkOs({ plugin: pluginFilter, plugins, appId, locks, defaults, brand }).catch((error) => {
      console.error("[DEBUG] os-dev react boot failed", error);
    });
  }
}
