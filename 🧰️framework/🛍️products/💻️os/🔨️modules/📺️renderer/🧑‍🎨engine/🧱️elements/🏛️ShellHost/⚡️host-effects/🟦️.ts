/** ⚡️ `applyHostEffects` and the effects that keep the refresh lane and the shell route in step with host effects.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useApplyHostEffects` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { shellDialogOriginIsCurrentV1, shellDialogSessionIsCurrentV1 } from "../🗨️dialog-origin/🟦️.ts";
import { useCallback } from "react";
import { type Effect, type UiDirtyScope } from "@semio-tech/framework";
// #region 🔌️Adapters
import {
  LOCAL_CATALOG_ACTIONS_V1,
  localCatalogAdmissionV1,
  localCatalogAdmitMutationV1,
  localCatalogDocumentIdV1,
  localCatalogNoticeCodeV1,
  localCatalogNoticeTextV1,
  localCatalogRehydrationArgumentsV1
} from "../🗂️local-catalog/🟦️.ts";
import { type ActiveSession } from "../../🐚️Shell/📦️program-sessions/🟦️.ts";
import {
  downloadMediaExportBytes,
  downloadDataUrl,
  downloadMediaExport,
  mediaExportEncodingText,
  shellSegmentedDownloadSinkFactory,
  requestFileOpen
} from "../../🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import {
  typedOperationCompletionRefreshV1,
  hostEffectRefreshScopeV1,
  hostEffectsRewriteGuestRenderInputsV1
} from "../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import {
  operationProgressPartsV1,
  type OperationProgressV1,
  programHistoryProjectionsWithLoadV1
} from "../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import {
  importOpenedFilesV1,
  requestMediaFramesSourceV1,
  runMediaFramesV1,
  scheduleDispatchAction
} from "../../🛠️ShellHelpers/🎞️media-import/🟦️.ts";
import { parsePanelState } from "../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { clipboardWriteFragmentFromEffect } from "../../🛠️ShellHelpers/🪟️window-panes/📋️clipboard-activation/🟦️.ts";
// #endregion 🔌️Adapters
import {
  captureSpaceArtifactCreationCatalogAuthorityV1,
  directoryCommandFromAction,
  mintDirectoryCommandRequestId,
  SPACE_INDEX_DOCUMENT_ID,
  type SpaceArtifactCreationOwnerV1,
  spaceArtifactCreationRequestFromAction,
  spaceIndexOpeningArgsV1
} from "../🧾️space-artifact-creation/🟦️.ts";
import type { ShellCoreAfterSpawnedSession } from "../🛫️spawned-session/🟦️.ts";
import { applyOpenDialogEffect, applySetActiveToolEffect, applySetActiveUtilityEffect } from "./🎛️view-state-effects/🟦️.ts";
import { applyLoadDocumentEffect, applyRequestFileOpenEffect } from "./📥️document-load-effects/🟦️.ts";
import { applyDownloadMediaExportEffect, applyIconRenderExportEffect, applyVideoRenderExportEffect } from "./📤️export-effects/🟦️.ts";
import { applyDispatchActionEffect, applyInvokeExtensionEffect, applyRequestMediaFramesEffect } from "./🔁️dispatch-media-and-extension-effects/🟦️.ts";
import { applyReplayShellCommandEffect } from "./🎯️replay-shell-command-effect/🟦️.ts";
import { applyOpenPluginInstanceEffect, applySpawnPluginInstanceEffect } from "./🌱️spawn-and-open-instance-effects/🟦️.ts";
import { useOperationCompletionSubscriptions, useOperationProgressSubscriptions } from "./📡️operation-subscriptions/🟦️.ts";

export type ShellCoreAfterApplyHostEffects = ShellCoreAfterSpawnedSession & ReturnType<typeof useApplyHostEffects>;

//#region FrameworkOsShell
export function useApplyHostEffects(core: ShellCoreAfterSpawnedSession) {
  const {
    catalog, hostMode, defaults, dispatch, historyStore, exampleOptionsRef, lastDispatchedExampleIdRef, loadedPlugins,
    session, applyHistoryPatch, leftoverReplaceRefreshBodiesV1, applyLeftoverInteractionView, refreshHistorySnapshot,
    spacePrograms, liveDialogRef, clipboardFragmentRef, settleOperation, sessionWorkRef, sealedInstancesRef,
    dropForSealedInstance, dropForRetiredInstance, openSpaceIdRef, openInstanceIdRef, sessionRef, identityRef,
    uiLocaleRef, uiTerminologyRef, showTransientNoticeRef, replayShellOwnedCommandRef, openDocumentRef,
    openArtifactWithAppRefRef, resolveArtifactOpeningWithActivationRef, spaceArtifactCreationOwnersRef,
    setSpaceArtifactCreationUi, spaceArtifactCreationCatalogRef, spaceArtifactCreationCatalogUiRef,
    setSpaceAdministration, spaceAdministrationRef, spaceAdministrationEpochRef, localDataDir, localCatalogRef,
    rehydratedLocalDocumentsRef, segmentedDownloadAbortRef, openDocumentSessionsRef, loadDocumentPair,
    captureDialogOrigin, captureEffectOwner, isCurrentEffectOwner, captureProgramEffectOwner, makeOwnedDialog,
    spaceDirectoryHistoriesRef, adoptSharedSpaceAccess, loadedPluginsRef, ensureBackboneWorker, writeLocalDocumentRef,
    recordLocalCatalogRef, navigateShellUri, spawnedAppsRef, spawnedRosterKey, activeToolIdRef, writeUtilityRegister,
    clearAllWindowUtilities, writeToolRegister, activeWindowIdRef, shellStateRef, resolvedTargetViewState,
    applyHostEffectsRef, refreshUi, refreshSpawnedUi, ensureSpawnedPlugin, requestServiceOperation, hostPorts
  } = core;
  const applyHostEffects = useCallback(
    async (effects: readonly Effect[], baseSession: ActiveSession, uiScope: UiDirtyScope | undefined, effectOwner: ReturnType<typeof captureEffectOwner>) => {
      const effectOrigin = effectOwner.presentation;
      if (effects.length === 0 && sealedInstancesRef.current.sealed(baseSession.pluginId, baseSession.instanceId)) return;
      if (dropForSealedInstance(baseSession, "host effects", `${effects.length} effect(s)`)) return;
      let nextViewState = baseSession.viewState;
      for (const effect of effects) {
        if (!isCurrentEffectOwner(effectOwner)) return;
        if (effect === "requestSync") continue;
        if ("notify" in effect) {
          if (effect.notify.message) showTransientNoticeRef.current(effect.notify.message, "warning");
          continue;
        }
        if ("clipboardWrite" in effect) {
          clipboardFragmentRef.current = clipboardWriteFragmentFromEffect(effect);
          continue;
        }
        if ("setPanel" in effect) {
          nextViewState = { ...nextViewState, panelJson: effect.setPanel.panelJson };
          continue;
        }
        if ("setActiveUtility" in effect) {
          nextViewState = applySetActiveUtilityEffect({ writeUtilityRegister, activeToolIdRef, writeToolRegister, activeWindowIdRef, effect, nextViewState });
          continue;
        }
        if ("setActiveTool" in effect) {
          nextViewState = applySetActiveToolEffect({ writeToolRegister, clearAllWindowUtilities, effect, nextViewState });
          continue;
        }
        if ("openDialog" in effect) {
          applyOpenDialogEffect({ shellStateRef, makeOwnedDialog, liveDialogRef, dispatch, effect, baseSession, effectOrigin });
          continue;
        }
        if ("navigate" in effect) {
          navigateShellUri(effect.navigate.uri);
          continue;
        }
        if ("loadDocument" in effect) {
          await applyLoadDocumentEffect({ loadedPlugins, uiTerminologyRef, uiLocaleRef, historyStore, loadDocumentPair, isCurrentEffectOwner, refreshHistorySnapshot, baseSession, effect, core, effectOwner });
          continue;
        }
        if ("openExternalUrl" in effect) {
          window.open(effect.openExternalUrl.url, "_blank", "noopener,noreferrer");
          continue;
        }
        if ("downloadMediaExport" in effect) {
          await applyDownloadMediaExportEffect({ loadedPlugins, segmentedDownloadAbortRef, effect, baseSession });
          continue;
        }
        if ("videoRenderExport" in effect) {
          applyVideoRenderExportEffect({ catalog, uiLocaleRef, showTransientNoticeRef, effect, baseSession });
          continue;
        }
        if ("iconRenderExport" in effect) {
          await applyIconRenderExportEffect({ effect });
          continue;
        }
        if ("requestFileOpen" in effect) {
          await applyRequestFileOpenEffect({ loadedPlugins, isCurrentEffectOwner, resolvedTargetViewState, effect, baseSession, core, applyHostEffects, effectOwner });
          continue;
        }
        if ("dispatchAction" in effect) {
          applyDispatchActionEffect({ loadedPluginsRef, isCurrentEffectOwner, resolvedTargetViewState, effect, baseSession, applyHostEffects, effectOwner });
          continue;
        }
        if ("requestServiceOperation" in effect) {
          await requestServiceOperation(baseSession, effect.requestServiceOperation, () => isCurrentEffectOwner(effectOwner));
          continue;
        }
        if ("replayShellCommand" in effect) {
          await applyReplayShellCommandEffect({ showTransientNoticeRef, uiLocaleRef, spaceAdministrationEpochRef, identityRef, ensureBackboneWorker, spaceAdministrationRef, setSpaceAdministration, openDocumentSessionsRef, shellStateRef, spaceArtifactCreationCatalogRef, spaceArtifactCreationCatalogUiRef, spaceArtifactCreationOwnersRef, setSpaceArtifactCreationUi, spaceDirectoryHistoriesRef, hostPorts, adoptSharedSpaceAccess, resolveArtifactOpeningWithActivationRef, openArtifactWithAppRefRef, isCurrentEffectOwner, openDocumentRef, localDataDir, writeLocalDocumentRef, recordLocalCatalogRef, loadedPluginsRef, resolvedTargetViewState, rehydratedLocalDocumentsRef, localCatalogRef, replayShellOwnedCommandRef, exampleOptionsRef, defaults, lastDispatchedExampleIdRef, dispatch, effect, baseSession, effectOrigin, effectOwner, applyHostEffects, core });
          continue;
        }
        if ("requestMediaFrames" in effect) {
          await applyRequestMediaFramesEffect({ loadedPlugins, isCurrentEffectOwner, resolvedTargetViewState, effect, baseSession, core, applyHostEffects, effectOwner });
          continue;
        }
        if ("invokeExtension" in effect) {
          applyInvokeExtensionEffect({ loadedPluginsRef, sessionRef, sessionWorkRef, dropForSealedInstance, isCurrentEffectOwner, applyHistoryPatch, applyLeftoverInteractionView, sealedInstancesRef, baseSession, effect, effectOwner, applyHostEffects });
          continue;
        }
        if ("spawnPluginInstance" in effect) {
          const nextViewStateNext = await applySpawnPluginInstanceEffect({ spacePrograms, ensureSpawnedPlugin, isCurrentEffectOwner, effect, effectOwner, nextViewState });
          if (nextViewStateNext === null) return;
          nextViewState = nextViewStateNext;
          continue;
        }
        if ("openPluginInstance" in effect) {
          const nextViewStateNext = await applyOpenPluginInstanceEffect({ spacePrograms, ensureSpawnedPlugin, isCurrentEffectOwner, openSpaceIdRef, openInstanceIdRef, navigateShellUri, effect, effectOwner, nextViewState });
          if (nextViewStateNext === null) return;
          nextViewState = nextViewStateNext;
          continue;
        }
      }
      const nextSession = { ...baseSession, viewState: nextViewState };
      if (!isCurrentEffectOwner(effectOwner)) return;
      const viewStateRewrittenByEffects = nextViewState !== baseSession.viewState;
      const isSpawnedPluginSession = hostMode && !shellDialogSessionIsCurrentV1(baseSession, shellStateRef.current.pluginRuntime.session);
      dispatch({
        type: "SET_SESSION",
        value: (current) => {
          if (!shellDialogOriginIsCurrentV1(effectOrigin, captureDialogOrigin(current))) return current;
          if (!current) return current;
          if (isSpawnedPluginSession) return viewStateRewrittenByEffects ? { ...current, viewState: nextViewState } : current;
          if (!shellDialogSessionIsCurrentV1(current, nextSession)) return current;
          return viewStateRewrittenByEffects ? { ...current, viewState: nextViewState } : current;
        },
      });
      const refreshScope = hostEffectRefreshScopeV1(effects, uiScope ?? { kind: "full" }, nextSession.app.windowKinds.map((kind) => kind.bodyKey));
      if (isSpawnedPluginSession) {
        const spawned = parsePanelState(nextViewState)?.spawnedApps.find((entry) => entry.pluginId === baseSession.pluginId && entry.instanceId === baseSession.instanceId);
        if (spawned) await refreshSpawnedUi(spawned, nextViewState, refreshScope);
      } else if (shellDialogSessionIsCurrentV1(shellStateRef.current.pluginRuntime.session, nextSession)) {
        await refreshUi(nextSession, refreshScope, undefined, leftoverReplaceRefreshBodiesV1(), hostEffectsRewriteGuestRenderInputsV1(effects) || nextViewState !== baseSession.viewState);
      }
    },
    [catalog,captureDialogOrigin, captureEffectOwner, dropForSealedInstance, isCurrentEffectOwner, loadDocumentPair, makeOwnedDialog, refreshHistorySnapshot, clearAllWindowUtilities, ensureSpawnedPlugin, loadedPlugins, navigateShellUri, refreshSpawnedUi, refreshUi, requestServiceOperation, resolvedTargetViewState, session, writeUtilityRegister, writeToolRegister, spacePrograms, hostMode, hostPorts?.documentAccess],
  );
  core.applyHostEffects = applyHostEffects;
  applyHostEffectsRef.current = applyHostEffects;

  useOperationCompletionSubscriptions({ session, loadedPluginsRef, settleOperation, applyHistoryPatch, captureProgramEffectOwner, applyLeftoverInteractionView, dropForRetiredInstance, isCurrentEffectOwner, hostMode, spawnedAppsRef, spawnedRosterKey, showTransientNoticeRef, applyHostEffects });

  useOperationProgressSubscriptions({ session, loadedPluginsRef, applyHistoryPatch, captureProgramEffectOwner, dropForRetiredInstance, hostMode, spawnedAppsRef, spawnedRosterKey, applyHostEffects });

  return {};
}
//#endregion FrameworkOsShell
