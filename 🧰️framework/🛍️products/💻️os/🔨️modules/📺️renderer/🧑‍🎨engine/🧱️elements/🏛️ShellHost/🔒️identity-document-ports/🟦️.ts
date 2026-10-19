/** 🔒️ Ui device, theme and driver reads, the backbone worker refs, the agent bridge, identity and hub session state, directory home refs, the document opening ports, archive pairing, effect owners, browser actor ui and the space directory.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useIdentityAndDocumentPorts` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import type { ShellCoreAfterShellCoreState } from "../🧩️shell-core-state/🟦️.ts";
import { useIdentityAgentBridgeAndDirectoryState } from "./🪪️identity-and-directory-state/🟦️.ts";
import { useEffectOwnersAndDialogs, useHubPortInferenceAndDocumentBackbone } from "./📎️hub-port-and-document-backbone/🟦️.ts";

//#region FrameworkOsShell
export type ShellCoreAfterIdentityAndDocumentPorts = ShellCoreAfterShellCoreState & ReturnType<typeof useIdentityAndDocumentPorts>;
export function useIdentityAndDocumentPorts(core: ShellCoreAfterShellCoreState) {
  const { hubSessionCapabilityRef, rememberHubSessionCapability, agentBridgeOfferScopeRef, hubEnv, openSpaceId, spaceAdministration, closeDocumentRef, backboneWorkerRef, applyRemoteMergeRef, documentPortsRef, documentAttachmentLane, spaceArtifactCreationCatalogRef, spaceArtifactCreationCatalogUiRef, sessionRef, historyStore, refreshHistorySnapshot, liveDialogRef, dispatch, dialogOpeningRef, identityRef, documentAccess, uiDevice, uiTheme, uiDriver, backboneWorkerStreamsDetachRef, shellSessionIdRef, shellActorIdRef, applyAgentShellCommandRef, agentArtifactRouteRef, agentDeferredPlansRef, setAgentBridgeCensus, agentBridgeInstances, agentBridge, identity, setIdentity, identityOffline, setIdentityOffline, hubSessionRefused, setHubSessionRefused, verifiedSessionAuthority, setVerifiedSessionAuthority, verifiedSessionAuthorityRef, sessionRefreshRef, hubSessionPortRef, hubSessionCapability, identityResolution, identityResolutionRef, directoryHomeOwnerRef, directoryHomeRetirementRef, directoryHomeOpeningRef, directoryBootstrapEpochRef, directoryBootstrapUi, setDirectoryBootstrapUi, refreshDirectoryHomeRef, handleDirectoryEventPageRef, handlePreferenceLaneMessageRef, handleDirectoryBootstrapFailureRef, uiLocaleRef, uiTerminologyRef, retireDirectoryHomeOwner, dispatchDirectoryEventsRef, showTransientNoticeRef, dispatchCheckpointRef, replayShellOwnedCommandRef, backgroundSpaceIndexSessionsRef, openDocumentRef, documentAttachmentLanesRef, retireDocumentAttachment, openArtifactWithAppRefRef, resolveArtifactOpeningRelayRef, resolveArtifactOpeningWithActivationRef, openReadySpaceArtifactCreationRef, spaceArtifactCreationOwnersRef, spaceArtifactCreationUi, setSpaceArtifactCreationUi, spaceArtifactCreationCatalog, setSpaceArtifactCreationCatalog, spaceArtifactCreationCatalogUi, setSpaceArtifactCreationCatalogUi, cancelSpaceArtifactCreationsForRuntime, cancelSpaceArtifactCreation, setDirectoryPendingCommands, directoryCommandResultsRef, setSpaceAdministration, hubWorkspaceOpen, setHubWorkspaceOpen } = useIdentityAgentBridgeAndDirectoryState({ core });
  const { captureDialogOrigin, loadDocumentArchive, isCurrentDialogOrigin, openDocumentSessionsRef, hubConnectionPort, spaceAdministrationRef, spaceAdministrationEpochRef, spaceAdministrationMessageRef, inferencePortEpochRef, inferencePortOwnerRef, inferencePortAuthorityRef, retiredSessionDocumentOwnersRef, inferencePortOpeningRef, setInferenceHistoryByRuntimeKey, inferenceHistoryByRuntimeKeyRef, identitySnapshotResolverRef, identityClientInstanceIdRef, identityBootstrapAbortRef, localDataDir, localCatalog, setLocalCatalog, localCatalogRef, localCatalogClientInstanceIdRef, localDocumentOwnersRef, rehydratedLocalDocumentsRef, presenceConnectedAtMsRef, presenceCursorRef, presenceBeatTriggersRef, extensionFetchAbortRef, segmentedDownloadAbortRef, failDocumentBackbone, receiveDocumentBackbone, bindDocumentBackbone, loadDocumentPair } = useHubPortInferenceAndDocumentBackbone({ hubSessionCapabilityRef, rememberHubSessionCapability, agentBridgeOfferScopeRef, hubEnv, openSpaceId, spaceAdministration, closeDocumentRef, backboneWorkerRef, applyRemoteMergeRef, documentPortsRef, documentAttachmentLane, core });
  const { captureEffectOwner, isCurrentEffectOwner, captureProgramEffectOwner, documentArchivePorts, closeOwnedDialog, makeOwnedDialog, browserActorUiByRuntimeKeyRef, browserActorUiVersion, setBrowserActorUiVersion, browserActorForSessionV1, publishUiRefreshSectionsV1, retireBrowserActorUi, directoryScopedOwnersRef, spaceDirectoryHistoriesRef, foldSpaceDirectoryRef, sharedSpaceAccessRef, sharedSpaceAccessVersion, adoptSharedSpaceAccess, collectSpaceDirectoryEvents } = useEffectOwnersAndDialogs({ captureDialogOrigin, core, spaceArtifactCreationCatalogRef, spaceArtifactCreationCatalogUiRef, sessionRef, historyStore, loadDocumentArchive, refreshHistorySnapshot, liveDialogRef, dispatch, isCurrentDialogOrigin, dialogOpeningRef, openDocumentSessionsRef, identityRef, documentAccess });
  return {
    uiDevice, uiTheme, uiDriver, backboneWorkerRef, backboneWorkerStreamsDetachRef, shellSessionIdRef, shellActorIdRef,
    applyAgentShellCommandRef, agentArtifactRouteRef, agentDeferredPlansRef, setAgentBridgeCensus, agentBridgeInstances,
    agentBridge, identity, setIdentity, identityRef, identityOffline, setIdentityOffline, hubSessionRefused,
    setHubSessionRefused, verifiedSessionAuthority, setVerifiedSessionAuthority, verifiedSessionAuthorityRef,
    sessionRefreshRef, hubSessionPortRef, hubEnv, hubSessionCapability, identityResolution, identityResolutionRef,
    directoryHomeOwnerRef, directoryHomeRetirementRef, directoryHomeOpeningRef, directoryBootstrapEpochRef,
    directoryBootstrapUi, setDirectoryBootstrapUi, refreshDirectoryHomeRef, handleDirectoryEventPageRef,
    handlePreferenceLaneMessageRef, handleDirectoryBootstrapFailureRef, uiLocaleRef, uiTerminologyRef,
    retireDirectoryHomeOwner, dispatchDirectoryEventsRef, applyRemoteMergeRef, showTransientNoticeRef,
    dispatchCheckpointRef, replayShellOwnedCommandRef, backgroundSpaceIndexSessionsRef, openDocumentRef,
    closeDocumentRef, documentAttachmentLanesRef, documentAttachmentLane, retireDocumentAttachment,
    openArtifactWithAppRefRef, resolveArtifactOpeningRelayRef, resolveArtifactOpeningWithActivationRef,
    openReadySpaceArtifactCreationRef, spaceArtifactCreationOwnersRef, spaceArtifactCreationUi,
    setSpaceArtifactCreationUi, spaceArtifactCreationCatalog, setSpaceArtifactCreationCatalog,
    spaceArtifactCreationCatalogUi, setSpaceArtifactCreationCatalogUi, spaceArtifactCreationCatalogRef,
    spaceArtifactCreationCatalogUiRef, cancelSpaceArtifactCreationsForRuntime, cancelSpaceArtifactCreation,
    setDirectoryPendingCommands, directoryCommandResultsRef, spaceAdministration, setSpaceAdministration,
    hubWorkspaceOpen, setHubWorkspaceOpen, hubConnectionPort, spaceAdministrationRef, spaceAdministrationEpochRef,
    spaceAdministrationMessageRef, inferencePortEpochRef, inferencePortOwnerRef, inferencePortAuthorityRef,
    retiredSessionDocumentOwnersRef, inferencePortOpeningRef, setInferenceHistoryByRuntimeKey,
    inferenceHistoryByRuntimeKeyRef, identitySnapshotResolverRef, identityClientInstanceIdRef, identityBootstrapAbortRef,
    localDataDir, localCatalog, setLocalCatalog, localCatalogRef, localCatalogClientInstanceIdRef,
    localDocumentOwnersRef, rehydratedLocalDocumentsRef, presenceConnectedAtMsRef, presenceCursorRef,
    presenceBeatTriggersRef, extensionFetchAbortRef, segmentedDownloadAbortRef, openDocumentSessionsRef,
    failDocumentBackbone, receiveDocumentBackbone, bindDocumentBackbone, loadDocumentPair, captureDialogOrigin,
    isCurrentDialogOrigin, captureEffectOwner, isCurrentEffectOwner, captureProgramEffectOwner, documentArchivePorts,
    closeOwnedDialog, makeOwnedDialog, browserActorUiByRuntimeKeyRef, browserActorUiVersion,
    setBrowserActorUiVersion, browserActorForSessionV1, publishUiRefreshSectionsV1, retireBrowserActorUi,
    directoryScopedOwnersRef, spaceDirectoryHistoriesRef, foldSpaceDirectoryRef, sharedSpaceAccessRef,
    sharedSpaceAccessVersion, adoptSharedSpaceAccess, collectSpaceDirectoryEvents
  };
}
//#endregion FrameworkOsShell
