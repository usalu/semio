/** 📡️ The operation completion, diagnostic and progress subscriptions of the host effects hook.
 *
 * 🧱️ Child hook module of {@link ../🟦️.ts | useApplyHostEffects}: runs the original `useEffect` statements in order. */
import { useEffect } from "react";
import { type Effect, type UiDirtyScope } from "@semio-tech/framework";
import { spawnedProgramViewStateV1 } from "../../🪟️spawned-program/🟦️.ts";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { typedOperationCompletionRefreshV1 } from "../../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { operationProgressPartsV1, type OperationProgressV1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { logUnlessRetiredV1 } from "../../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterSpawnedSession } from "../../🛫️spawned-session/🟦️.ts";

/** 🛰️ Applies terminal operation publications and shows the operation diagnostics for the active and spawned receivers. */
export function useOperationCompletionSubscriptions({ session, loadedPluginsRef, settleOperation, applyHistoryPatch, captureProgramEffectOwner, applyLeftoverInteractionView, dropForRetiredInstance, isCurrentEffectOwner, hostMode, spawnedAppsRef, spawnedRosterKey, showTransientNoticeRef, applyHostEffects }: Pick<ShellCoreAfterSpawnedSession, "session" | "loadedPluginsRef" | "settleOperation" | "applyHistoryPatch" | "captureProgramEffectOwner" | "applyLeftoverInteractionView" | "dropForRetiredInstance" | "isCurrentEffectOwner" | "hostMode" | "spawnedAppsRef" | "spawnedRosterKey" | "showTransientNoticeRef"> & {
  readonly applyHostEffects: (effects: readonly Effect[], baseSession: ActiveSession, uiScope: UiDirtyScope | undefined, effectOwner: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>) => Promise<void>;
}) {
  useEffect(() => {
    if (!session) return;
    const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
    if (!plugin) return;
    const target = session;
    try {
      return plugin.subscribeOperationCompletions(target.instanceId, (completion) => {
        settleOperation(completion);
        applyHistoryPatch(completion.historyPatch, false, { pluginId: target.pluginId, instanceId: target.instanceId });
        const refresh = typedOperationCompletionRefreshV1(completion);
        const owner = captureProgramEffectOwner(target);
        if (refresh === null) return;
        void applyHostEffects(completion.requestedEffects, target, refresh, owner).catch(logUnlessRetiredV1("typed-operation completion effects failed"));
      });
    } catch (error) {
      return;
    }
  }, [applyHistoryPatch, applyLeftoverInteractionView, applyHostEffects, captureProgramEffectOwner, dropForRetiredInstance, isCurrentEffectOwner, session, settleOperation]);

  useEffect(() => {
    if (!hostMode || !session) return;
    const disposers: (() => void)[] = [];
    for (const spawned of spawnedAppsRef.current) {
      const entry = loadedPluginsRef.current.find((candidate) => candidate.handle.pluginId === spawned.pluginId);
      const app = entry?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
      if (!entry || !app) continue;
      const target: ActiveSession = { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: spawnedProgramViewStateV1(session.viewState, app) };
      try {
        const dispose = entry.handle.subscribeOperationCompletions(spawned.instanceId, (completion) => {
          settleOperation(completion);
          applyHistoryPatch(completion.historyPatch, false, { pluginId: spawned.pluginId, instanceId: spawned.instanceId });
          const refresh = typedOperationCompletionRefreshV1(completion);
          if (refresh === null) return;
          void applyHostEffects(completion.requestedEffects, target, refresh, captureProgramEffectOwner(target)).catch(logUnlessRetiredV1("spawned typed-operation completion effects failed"));
        });
        if (typeof dispose === "function") disposers.push(dispose);
      } catch (error) {
        console.warn(`[os-shell] spawned operation-completion subscription refused for ${spawned.pluginId}#${spawned.instanceId}`, error);
      }
    }
    return () => {
      for (const dispose of disposers) dispose();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [hostMode, session, spawnedRosterKey, applyHistoryPatch, applyHostEffects, captureProgramEffectOwner, settleOperation]);

  useEffect(() => {
    if (!session) return;
    const instances = [{ pluginId: session.pluginId, instanceId: session.instanceId }, ...(hostMode ? spawnedAppsRef.current : [])];
    const disposers: (() => void)[] = [];
    for (const instance of instances) {
      const plugin = loadedPluginsRef.current.find(entry => entry.handle.pluginId === instance.pluginId)?.handle;
      if (!plugin) continue;
      disposers.push(plugin.subscribeOperationDiagnostics(instance.instanceId, ({ token, diagnostic }) => {
        if (token.receiver !== instance.instanceId) return;
        const severity = diagnostic.severity === "fatal" ? "error" : diagnostic.severity;
        showTransientNoticeRef.current(diagnostic.message, severity, diagnostic.code);
      }));
    }
    return () => { for (const dispose of disposers) dispose(); };
  }, [hostMode, session, spawnedRosterKey]);
}

/** 📈️ Renders a running operation's process through the ui-refresh lane for the session and every spawned program. */
export function useOperationProgressSubscriptions({ session, loadedPluginsRef, applyHistoryPatch, captureProgramEffectOwner, dropForRetiredInstance, hostMode, spawnedAppsRef, spawnedRosterKey, applyHostEffects }: Pick<ShellCoreAfterSpawnedSession, "session" | "loadedPluginsRef" | "applyHistoryPatch" | "captureProgramEffectOwner" | "dropForRetiredInstance" | "hostMode" | "spawnedAppsRef" | "spawnedRosterKey"> & {
  readonly applyHostEffects: (effects: readonly Effect[], baseSession: ActiveSession, uiScope: UiDirtyScope | undefined, effectOwner: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>) => Promise<void>;
}) {
  useEffect(() => {
    if (!session) return;
    const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
    if (!plugin) return;
    const target = session;
    try {
      const unsubscribeOperations = plugin.subscribeOperationProgress(target.instanceId, (progress: OperationProgressV1) => {
        const { scope, historyPatch } = operationProgressPartsV1(progress);
        applyHistoryPatch(historyPatch, false, { pluginId: target.pluginId, instanceId: target.instanceId });
        if (scope.kind === "none") return;
        void applyHostEffects([], target, scope, captureProgramEffectOwner(target)).catch(logUnlessRetiredV1("typed-operation progress refresh failed"));
      });
      const unsubscribeJobs = plugin.subscribeSpawnedJobProgress?.(target.instanceId, () => {
        void applyHostEffects([], target, { kind: "full" }, captureProgramEffectOwner(target)).catch(logUnlessRetiredV1("spawned job progress refresh failed"));
      });
      return () => {
        unsubscribeOperations();
        unsubscribeJobs?.();
      };
    } catch (error) {
      return;
    }
  }, [applyHistoryPatch, applyHostEffects, captureProgramEffectOwner, dropForRetiredInstance, session]);

  useEffect(() => {
    if (!hostMode || !session) return;
    const disposers: (() => void)[] = [];
    for (const spawned of spawnedAppsRef.current) {
      const entry = loadedPluginsRef.current.find((candidate) => candidate.handle.pluginId === spawned.pluginId);
      const app = entry?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
      if (!entry || !app) continue;
      const target: ActiveSession = { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: spawnedProgramViewStateV1(session.viewState, app) };
      try {
        const unsubscribeOperations = entry.handle.subscribeOperationProgress(spawned.instanceId, (progress: OperationProgressV1) => {
          const { scope, historyPatch } = operationProgressPartsV1(progress);
          applyHistoryPatch(historyPatch, false, { pluginId: spawned.pluginId, instanceId: spawned.instanceId });
          if (scope.kind === "none") return;
          void applyHostEffects([], target, scope, captureProgramEffectOwner(target)).catch(logUnlessRetiredV1("spawned typed-operation progress refresh failed"));
        });
        const unsubscribeJobs = entry.handle.subscribeSpawnedJobProgress?.(spawned.instanceId, () => {
          void applyHostEffects([], target, { kind: "full" }, captureProgramEffectOwner(target)).catch(logUnlessRetiredV1("spawned job progress refresh failed"));
        });
        disposers.push(() => {
          unsubscribeOperations();
          unsubscribeJobs?.();
        });
      } catch (error) {
        console.warn(`[os-shell] spawned operation-progress subscription refused for ${spawned.pluginId}#${spawned.instanceId}`, error);
      }
    }
    return () => {
      for (const dispose of disposers) dispose();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [hostMode, session, spawnedRosterKey, applyHistoryPatch, applyHostEffects, captureProgramEffectOwner]);
}
