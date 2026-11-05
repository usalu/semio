/** 🟢️ The spawn program phase of useShellUriAndDocuments. */
import { useCallback } from "../../../⚛️react-port/🟦️.ts";
import { spawnProgramRefusalCodeV1, spawnProgramRefusalNoticeTextV1, type SpawnProgramRefusalReasonV1 } from "../../🪟️spawned-program/🟦️.ts";
import { type SpaceProgramEntry } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { buildSpacePanelState, studioPanelFocusingSpawned } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { requiredHostPanelLeafId } from "../../🧱️built-node-stores/🟦️.ts";
import type { useScopeState } from "../🔶️scope/🟦️.ts";

/** 🔵 Builds spawnProgram of useShellUriAndDocuments. */
export function useSpawnProgram({ pluginOpInFlightRef, showTransientNoticeRef, uiLocaleRef, loadedPluginsRef, sessionRef, hostApp, updateSpacePanelFrom }: Pick<ReturnType<typeof useScopeState>, "showTransientNoticeRef" | "uiLocaleRef" | "loadedPluginsRef" | "sessionRef" | "hostApp" | "updateSpacePanelFrom"> & { readonly pluginOpInFlightRef: { current: ReadonlySet<string> } }) {
  const spawnProgram = useCallback(
    async (program: SpaceProgramEntry) => {
      const refuse = (reason: SpawnProgramRefusalReasonV1, ...detail: readonly unknown[]): void => {
        console.warn(`[os-shell] spawnProgram ${program.pluginId} ${program.appId}: ${spawnProgramRefusalCodeV1(reason)}`, ...detail);
        showTransientNoticeRef.current(spawnProgramRefusalNoticeTextV1(reason, program.breadcrumb.join(" · "), uiLocaleRef.current), "warning", spawnProgramRefusalCodeV1(reason));
      };
      const pluginEntry = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === program.pluginId);
      if (pluginOpInFlightRef.current.has(program.pluginId)) return refuse("session-not-ready");
      if (!pluginEntry) return refuse("program-not-installed");
      const owner = sessionRef.current;
      if (!owner) return refuse("session-not-ready");
      let instanceId: number;
      try {
        instanceId = await pluginEntry.handle.createApp(program.appId);
      } catch (openError) {
        return refuse("open-failed", openError);
      }
      const retire = async (): Promise<void> => {
        await pluginEntry.handle.destroyApp(instanceId).catch((error: unknown) => console.error(`[os-shell] unmounted program ${program.pluginId}#${instanceId} could not be retired`, error));
      };
      const current = sessionRef.current;
      if (pluginOpInFlightRef.current.has(program.pluginId) || current?.pluginId !== owner.pluginId || current?.instanceId !== owner.instanceId || current?.app.id !== owner.app.id || !loadedPluginsRef.current.some((entry) => entry.handle === pluginEntry.handle)) {
        await retire();
        return refuse("session-not-ready");
      }
      try {
        const mounted = updateSpacePanelFrom((latest) => studioPanelFocusingSpawned(latest ?? buildSpacePanelState([], requiredHostPanelLeafId(hostApp)), {
          id: `${program.pluginId}-${instanceId}`,
          pluginId: program.pluginId,
          instanceId,
          appId: program.appId,
          label: program.label,
          breadcrumb: program.breadcrumb,
        }));
        if (mounted) return;
        await retire();
        refuse("session-not-ready");
      } catch (error) {
        await retire();
        refuse("open-failed", error);
      }
    },
    [loadedPluginsRef, sessionRef, updateSpacePanelFrom, hostApp],
  );
  return { spawnProgram };
}
