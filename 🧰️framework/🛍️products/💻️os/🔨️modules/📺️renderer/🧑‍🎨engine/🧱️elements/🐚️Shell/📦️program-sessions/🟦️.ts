/** 📦️ Shell program sessions: the mixed-value inspector placeholder, the catalog program module and loaded program state, the local and session program readers, the plugin panel status and canvas status, and the active session and space program entries. */
import type { PluginModuleSourceV1 } from "@semio-tech/framework/plugin-module/hub-source/resolution";
import { type AppDefinition } from "@semio-tech/framework";
import { shellLabel } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import type { PluginManifest, PluginWasmHandle, ViewModel } from "../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";

//#region 🔖️types
/** 🌐️ Locale-resolved mixed-value placeholder for this renderer layer; framework/core/js/index.ts keeps its own non-reactive low-level default.
 *
 * 🔁️ Resolved on CALL, never at module load. `shellLabel` lives in `🛠️ShellHelpers`, which imports this
 * module, so a module-top-level call ran Shell's body from inside ShellHelpers' own initialization and
 * read a binding ShellHelpers had not reached yet — `Cannot access '__vite_ssr_import_18__' before
 * initialization`, thrown for every entry point that reaches ShellHelpers before Shell (importing
 * `🌐️World3dHost` first, which is what the mesh-residency lane does). A function has no load-time
 * order to get wrong, and the label is locale-resolved when it is read, which is what a relabelled
 * shell needs anyway (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function uiInspectorMixedPlaceholder(): ReturnType<typeof shellLabel> {
  return shellLabel("ui.common.mixedValues");
}

/** 🧩️ The catalog module a hub document's program runs: its plugin, catalog generation and module bundle, where its
 * verified bytes came from, and the program of every module of its closure (plugin id → program id), so the requests it
 * makes to its dependencies and extensions stay inside that generation. */
export type CatalogProgramModuleV1 = Readonly<{
  pluginId: string;
  generationId: string;
  bundleSha256: string;
  source: PluginModuleSourceV1;
  peers: Readonly<Record<string, string>>;
}>;

export type LoadedProgramState = {
  readonly handle: PluginWasmHandle;
  readonly manifest: PluginManifest;
  /** 🧩️ Present only on a hub document's catalog-resolved program (`handle.pluginId` is then its hub program id): it runs
   * beside the device's own program of the same plugin and is never one of the device's own programs. */
  readonly catalogModule?: CatalogProgramModuleV1;
};

/** 🔌️ The device's own programs — every loaded program except hub documents' catalog-resolved ones: what routing, the
 * program launcher, contributions and the dialect index read. The same array while no hub program is loaded. */
export function localProgramsV1(loaded: readonly LoadedProgramState[]): readonly LoadedProgramState[] {
  return loaded.some((entry) => entry.catalogModule !== undefined) ? loaded.filter((entry) => entry.catalogModule === undefined) : loaded;
}

/** 🧩️ The programs one session's program runs with: a hub program's own catalog closure (same generation), the device's
 * own programs for every other session. */
export function sessionProgramsV1(loaded: readonly LoadedProgramState[], sessionProgramId: string): readonly LoadedProgramState[] {
  const own = loaded.find((entry) => entry.handle.pluginId === sessionProgramId)?.catalogModule;
  if (own === undefined) return localProgramsV1(loaded);
  const peers = new Set(Object.values(own.peers));
  return loaded.filter((entry) => peers.has(entry.handle.pluginId));
}

/** 🪪️ The plugin id a program's own descriptor names: its catalog module's plugin for a hub program, else its id. */
export function programPluginIdV1(entry: LoadedProgramState): string {
  return entry.catalogModule?.pluginId ?? entry.handle.pluginId;
}

/** 🔌️ Lifecycle status of one registry entry for the plugin panel (bottom-right dock): "available" —
 * registered but not (yet) loaded, including a plugin whose first build hasn't landed. Driven by
 * `installPlugin`/`reloadPlugin`/`uninstallPlugin` and the `PluginSource` subscription, not by
 * `loadedPlugins` membership alone — a `"reloading"` entry is still present in `loadedPlugins` (the old
 * handle keeps serving until the swap completes). */
export type PluginPanelStatus = "available" | "installing" | "loaded" | "failed" | "reloading";

export type PluginSupervisorState = "loaded" | "running" | "restarting" | "crashed" | "quarantined";

/** 🧯️ Keeps terminal boot and recovery content visible through the layout's loading boundary. */
export function resolvePluginCanvasStatus(hasSession: boolean, error: string | null, pluginStatus?: PluginPanelStatus, supervisor?: PluginSupervisorState): "loading" | undefined {
  if (error || supervisor === "crashed" || supervisor === "quarantined") return undefined;
  if (!hasSession) return "loading";
  if (pluginStatus === "installing" || pluginStatus === "reloading") return "loading";
  return undefined;
}

export type ActiveSession = {
  readonly documentInitialized?: boolean;
  readonly pluginId: string;
  readonly instanceId: number;
  readonly app: AppDefinition;
  readonly viewState: ViewModel;
};

export type SpaceProgramEntry = {
  readonly pluginId: string;
  readonly workflowStepId: string;
  readonly appId: string;
  readonly label: string;
  readonly breadcrumb: readonly string[];
  readonly yields: string;
};
//#endregion 🔖️types
