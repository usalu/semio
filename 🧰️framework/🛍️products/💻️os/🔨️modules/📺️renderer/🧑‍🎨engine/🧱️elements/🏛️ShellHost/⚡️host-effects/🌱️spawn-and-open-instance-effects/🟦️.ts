/** 🌱️ The host effects that create program instances: spawning and opening plugin instances.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | useApplyHostEffects}: each function is one effect arm of the `applyHostEffects` pass, moved verbatim. */
import { type Effect } from "@semio-tech/framework";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { viewStateWithSpacePanel } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import type { ShellCoreAfterSpawnedSession } from "../../🛫️spawned-session/🟦️.ts";

/** 🌿️ Spawns a plugin instance for the effect's program. */
export async function applySpawnPluginInstanceEffect({ spacePrograms, ensureSpawnedPlugin, isCurrentEffectOwner, effect, effectOwner, nextViewState }: Pick<ShellCoreAfterSpawnedSession, "spacePrograms" | "ensureSpawnedPlugin" | "isCurrentEffectOwner"> & {
  readonly effect: Extract<Effect, Readonly<Record<"spawnPluginInstance", unknown>>>;
  readonly effectOwner: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>;
  readonly nextViewState: ActiveSession["viewState"];
}) {
  const { pluginId, appId, osInstanceId, label, artifactJson } = effect.spawnPluginInstance;
  const program = spacePrograms.find((entry) => entry.pluginId === pluginId && entry.appId === appId) ?? spacePrograms.find((entry) => entry.pluginId === pluginId);
  if (program) {
    const nextPanel = await ensureSpawnedPlugin(program, () => isCurrentEffectOwner(effectOwner), label, osInstanceId, artifactJson, nextViewState);
    if (!isCurrentEffectOwner(effectOwner)) return null;
    if (nextPanel) nextViewState = viewStateWithSpacePanel(nextViewState, nextPanel);
  }
  return nextViewState;
}

/** 🚪️ Opens a plugin instance and mounts it into the shell. */
export async function applyOpenPluginInstanceEffect({ spacePrograms, ensureSpawnedPlugin, isCurrentEffectOwner, openSpaceIdRef, openInstanceIdRef, navigateShellUri, effect, effectOwner, nextViewState }: Pick<ShellCoreAfterSpawnedSession, "spacePrograms" | "ensureSpawnedPlugin" | "isCurrentEffectOwner" | "openSpaceIdRef" | "openInstanceIdRef" | "navigateShellUri"> & {
  readonly effect: Extract<Effect, Readonly<Record<"openPluginInstance", unknown>>>;
  readonly effectOwner: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>;
  readonly nextViewState: ActiveSession["viewState"];
}) {
  const { pluginId, appId, osInstanceId } = effect.openPluginInstance;
  const program = spacePrograms.find((entry) => entry.pluginId === pluginId && entry.appId === appId) ?? spacePrograms.find((entry) => entry.pluginId === pluginId);
  if (program) {
    const nextPanel = await ensureSpawnedPlugin(program, () => isCurrentEffectOwner(effectOwner), undefined, osInstanceId, undefined, nextViewState);
    if (!isCurrentEffectOwner(effectOwner)) return null;
    if (nextPanel) {
      nextViewState = viewStateWithSpacePanel(nextViewState, nextPanel);
    }
    if (osInstanceId && openSpaceIdRef.current) {
      openInstanceIdRef.current = osInstanceId;
      navigateShellUri(`/spaces/${openSpaceIdRef.current}/instances/${osInstanceId}`);
    }
  } else {
    console.warn(
      "[os-shell] openPluginInstance: no program matches",
      { pluginId, appId },
      "available:",
      spacePrograms.map((entry) => `${entry.pluginId}/${entry.appId}`),
    );
  }
  return nextViewState;
}
