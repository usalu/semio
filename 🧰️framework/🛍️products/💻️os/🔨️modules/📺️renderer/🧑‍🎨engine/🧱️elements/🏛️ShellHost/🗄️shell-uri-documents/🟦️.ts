import type { ShellCoreAfterApplyHostEffects } from "../⚡️host-effects/🟦️.ts";
import { useScopeState } from "./🔶️scope/🟦️.ts";
import { useOpenDocument } from "./🔸️open-document/🟦️.ts";
import { useCloseDocumentState } from "./🟠️close-document/🟦️.ts";
import { useSpawnProgram } from "./🟢️spawn-program/🟦️.ts";

export type ShellCoreAfterShellUriAndDocuments = ShellCoreAfterApplyHostEffects & ReturnType<typeof useShellUriAndDocuments>;

//#region FrameworkOsShell
/**
 * 📑️ Applies shell uris, opens and closes documents and rehydrates the local catalog documents into the landing app.
 *
 * 🗃️ Hands the landing app every document this device keeps that its current instance was not handed yet: each one's
 * archive is read back from its own lane and applied as `applyLocalCatalogDocument` (a retained, idempotent catalog
 * job), so a studio kept before a reload is listed — and opens — again. A lane that holds no archive is skipped.
 */
export function useShellUriAndDocuments(core: ShellCoreAfterApplyHostEffects) {
  const { resolveSyncTargetSession, loadedPlugins, ensureBackboneWorker, identityRef, hubEnv, openDocumentSessionsRef, browserActorUiByRuntimeKeyRef, retireBrowserActorUi, shellActorIdRef, socketActorReadyRef, postBrowserActorViewState, retireDocumentAttachment, spaceDirectoryHistoriesRef, directoryScopedOwnersRef, setSpaceArtifactCreationCatalog, setSpaceArtifactCreationCatalogUi, documentAttachmentLane, bindDocumentBackbone, dispatch, backboneWorkerRef, closeDocumentRef, openDocumentRef, sessionRef, activeToolId, activeUtilityByWindowId, extraWindowInstances, session, cancelSpaceArtifactCreationsForRuntime, backgroundSpaceIndexSessionsRef, inferencePortOpeningRef, inferencePortOwnerRef, inferencePortAuthorityRef, inferencePortEpochRef, localDocumentOwnerRegistry, liveDialogRef, closeOwnedDialog, tutorialDrivenRef, rebootstrapDiscardedSessionsRef, setInferenceHistoryByRuntimeKey, setPresencePeersByRuntimeKey, setBootstrapUiByDocument, setExecutionTargetUiByDocument, scope, showTransientNoticeRef, syncBackboneUri, uiLocaleRef, hostApp, updateSpacePanel } = useScopeState({ core });
  const { openDocument } = useOpenDocument({ resolveSyncTargetSession, loadedPlugins, ensureBackboneWorker, identityRef, hubEnv, openDocumentSessionsRef, browserActorUiByRuntimeKeyRef, retireBrowserActorUi, shellActorIdRef, socketActorReadyRef, postBrowserActorViewState, retireDocumentAttachment, spaceDirectoryHistoriesRef, directoryScopedOwnersRef, setSpaceArtifactCreationCatalog, setSpaceArtifactCreationCatalogUi, documentAttachmentLane, bindDocumentBackbone, dispatch, backboneWorkerRef, closeDocumentRef, openDocumentRef, sessionRef, activeToolId, activeUtilityByWindowId, extraWindowInstances, session });
  const { closeDocument, forgetLocalFolder, attachSyncTarget, detachSyncBackbone, folderReconnectOffer, folderReconnectBusy, reconnectLocalFolder, browseSyncBackbonePath } = useCloseDocumentState({ openDocumentSessionsRef, cancelSpaceArtifactCreationsForRuntime, backboneWorkerRef, backgroundSpaceIndexSessionsRef, setSpaceArtifactCreationCatalog, setSpaceArtifactCreationCatalogUi, socketActorReadyRef, inferencePortOpeningRef, inferencePortOwnerRef, inferencePortAuthorityRef, inferencePortEpochRef, dispatch, retireDocumentAttachment, localDocumentOwnerRegistry, liveDialogRef, closeOwnedDialog, core, tutorialDrivenRef, retireBrowserActorUi, rebootstrapDiscardedSessionsRef, setInferenceHistoryByRuntimeKey, setPresencePeersByRuntimeKey, setBootstrapUiByDocument, setExecutionTargetUiByDocument, directoryScopedOwnersRef, spaceDirectoryHistoriesRef, closeDocumentRef, scope, resolveSyncTargetSession, loadedPlugins, showTransientNoticeRef, openDocument, syncBackboneUri });
  const { spawnProgram } = useSpawnProgram({ showTransientNoticeRef, uiLocaleRef, loadedPlugins, session, hostApp, updateSpacePanel });
  return {
    resolveSyncTargetSession, openDocument, closeDocument, forgetLocalFolder, attachSyncTarget, detachSyncBackbone,
    folderReconnectOffer, folderReconnectBusy, reconnectLocalFolder, browseSyncBackbonePath, spawnProgram
  };
}

//#endregion FrameworkOsShell
