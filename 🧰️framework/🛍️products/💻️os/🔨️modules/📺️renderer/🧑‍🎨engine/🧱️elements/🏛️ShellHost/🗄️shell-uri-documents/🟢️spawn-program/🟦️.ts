/** 🟢️ The spawn program phase of useShellUriAndDocuments. */
import { useCallback } from "react";
import { spawnProgramRefusalCodeV1, spawnProgramRefusalNoticeTextV1, type SpawnProgramRefusalReasonV1 } from "../../🪟️spawned-program/🟦️.ts";
import { type SpaceProgramEntry } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { buildSpacePanelState, parsePanelState, studioPanelFocusingSpawned } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { requiredHostPanelLeafId } from "../../🧱️built-node-stores/🟦️.ts";
import type { useScopeState } from "../🔶️scope/🟦️.ts";

/** 🔵 Builds spawnProgram of useShellUriAndDocuments. */
export function useSpawnProgram({ showTransientNoticeRef, uiLocaleRef, loadedPlugins, session, hostApp, updateSpacePanel }: Pick<ReturnType<typeof useScopeState>, "showTransientNoticeRef" | "uiLocaleRef" | "loadedPlugins" | "session" | "hostApp" | "updateSpacePanel">) {
  const spawnProgram = useCallback(
    async (program: SpaceProgramEntry) => {
      const refuse = (reason: SpawnProgramRefusalReasonV1, ...detail: readonly unknown[]): void => {
        console.warn(`[os-shell] spawnProgram ${program.pluginId} ${program.appId}: ${spawnProgramRefusalCodeV1(reason)}`, ...detail);
        showTransientNoticeRef.current(spawnProgramRefusalNoticeTextV1(reason, program.breadcrumb.join(" · "), uiLocaleRef.current), "warning", spawnProgramRefusalCodeV1(reason));
      };
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === program.pluginId);
      if (!pluginEntry) return refuse("program-not-installed");
      if (!session) return refuse("session-not-ready");
      let instanceId: number;
      try {
        instanceId = await pluginEntry.handle.createApp(program.appId);
      } catch (openError) {
        return refuse("open-failed", openError);
      }
      const currentPanel = parsePanelState(session.viewState) ?? buildSpacePanelState([], requiredHostPanelLeafId(hostApp));
      const spawnedId = `${program.pluginId}-${instanceId}`;
      updateSpacePanel(
        studioPanelFocusingSpawned(currentPanel, {
          id: spawnedId,
          pluginId: program.pluginId,
          instanceId,
          appId: program.appId,
          label: program.label,
          breadcrumb: program.breadcrumb,
        }),
      );
    },
    [loadedPlugins, session, updateSpacePanel, hostApp],
  );
  return { spawnProgram };
}
