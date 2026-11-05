/** 🧹️ Browser reload requires current storage proof for every open artifact owner. */
import type { ArtifactSyncStatus } from "@semio-tech/framework/backbone";
import type { PluginWasmHandle } from "../../../🔌️PluginRuntime/🟦️.tsx";
import type { ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { deriveSavedStateV1 } from "../../../🛠️ShellHelpers/💾️saved-state/🟦️.ts";
import { useBeforeUnloadGuardV1 } from "../../🚪️close-guard/🟦️.tsx";
import { captureProgramRetirementOwnersV1 } from "../🟦️.ts";

export function useShellProgramUnloadGuardV1(port: Readonly<{
  loaded: Parameters<typeof captureProgramRetirementOwnersV1>[0];
  session: ActiveSession | null;
  attachments: ReadonlyMap<string, Readonly<{ plugin: PluginWasmHandle; session: ActiveSession }>>;
  statuses: Readonly<Record<string, ArtifactSyncStatus>>;
}>): boolean {
  let blocked = [...port.attachments.keys()].some(key => deriveSavedStateV1(port.statuses[key] ?? null).kind !== "saved");
  try {
    blocked ||= captureProgramRetirementOwnersV1(port.loaded, port.session).some(owner => (owner.session.app.io?.artifactSchema ?? "").length > 0 && ![...port.attachments.values()].some(entry => entry.plugin === owner.handle && entry.session.instanceId === owner.session.instanceId && entry.session.app.id === owner.session.app.id));
  } catch {
    blocked = true;
  }
  useBeforeUnloadGuardV1(blocked);
  return blocked;
}
