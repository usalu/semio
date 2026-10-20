import type { ShellCoreAfterBackboneWorker } from "../🧵️backbone-worker/🟦️.ts";
import { useCatalogState } from "./🔶️catalog/🟦️.ts";
import { useRetireAuthenticatedInferenceState } from "./🔸️retire-authenticated-inference/🟦️.ts";
import { useShellUriState } from "./🟠️shell-uri/🟦️.ts";

export type ShellCoreAfterDirectoryAndRoute = ShellCoreAfterBackboneWorker & ReturnType<typeof useDirectoryAndRoute>;
/**
 * 🗺️ Routes directory events and space administration through the backbone worker.
 *
 * 🧯️ Unmount and identity change both retire the operation: the worker cancels its transport and
 * erases page, receipt, and capability before the pane can be remounted under a new identity.
 */
export function useDirectoryAndRoute(core: ShellCoreAfterBackboneWorker) {
  const { inferencePortOpeningRef, backboneWorkerRef, inferencePortEpochRef, inferencePortOwnerRef, inferencePortAuthorityRef, dispatch, openDocumentSessionsRef, retiredSessionDocumentOwnersRef, inferenceHistoryByRuntimeKeyRef, setInferenceHistoryByRuntimeKey, shellActorIdRef, identityBootstrapAbortRef, hubEnv, sessionRefreshRef, hubSessionPortRef, verifiedSessionAuthorityRef, setVerifiedSessionAuthority, setIdentityOffline, setHubSessionRefused, shellSessionIdRef, identityClientInstanceIdRef, hubSessionCapability, ensureBackboneWorker, identitySnapshotResolverRef, setIdentity, identity, verifiedSessionAuthority, hostPlugin, landingApp, session, directoryBootstrapEpochRef, directoryHomeOpeningRef, sessionRef, uiLocaleRef, uiTerminologyRef, refreshDirectoryHomeRef, retireDirectoryHomeOwner, directoryHomeOwnerRef, setDirectoryBootstrapUi, presenceCursorRef, hostMode, scope, identityResolutionRef, shellSessionLaneRef, shellRouteLedgerRef, plugins, pluginFilter, catalog, writeLocalDocumentRef, readLocalDocumentRef, recordLocalCatalogRef, cancelDirectoryBootstrap, dispatchSpaceAdministrationIntent, dispatchInferencePortIntent } = useCatalogState({ core });
  const { cancelSessionAuthorityBootstrap } = useRetireAuthenticatedInferenceState({ inferencePortOpeningRef, backboneWorkerRef, inferencePortEpochRef, inferencePortOwnerRef, inferencePortAuthorityRef, dispatch, openDocumentSessionsRef, retiredSessionDocumentOwnersRef, inferenceHistoryByRuntimeKeyRef, setInferenceHistoryByRuntimeKey, shellActorIdRef, identityBootstrapAbortRef, hubEnv, sessionRefreshRef, hubSessionPortRef, verifiedSessionAuthorityRef, setVerifiedSessionAuthority, setIdentityOffline, setHubSessionRefused, shellSessionIdRef, identityClientInstanceIdRef, hubSessionCapability, ensureBackboneWorker, identitySnapshotResolverRef, setIdentity });
  const { shellUri, canGoBack, canGoForward, canGoUp, goBack, goForward, goUp, navigateHistory, notFoundPath, sessionRouteRef, sessionRoute, setSessionRoute, requestSessionRoute, navigateShellUri, dockLayoutStore, dockUiStateStore, registry } = useShellUriState({ identity, verifiedSessionAuthority, hubEnv, hostPlugin, landingApp, session, ensureBackboneWorker, directoryBootstrapEpochRef, directoryHomeOpeningRef, sessionRef, uiLocaleRef, uiTerminologyRef, refreshDirectoryHomeRef, retireDirectoryHomeOwner, directoryHomeOwnerRef, setDirectoryBootstrapUi, presenceCursorRef, hostMode, scope, identityResolutionRef, shellSessionLaneRef, shellRouteLedgerRef, plugins, pluginFilter, catalog });
  return {
    writeLocalDocumentRef, readLocalDocumentRef, recordLocalCatalogRef, cancelDirectoryBootstrap,
    dispatchSpaceAdministrationIntent, dispatchInferencePortIntent, cancelSessionAuthorityBootstrap, shellUri, canGoBack,
    canGoForward, canGoUp, goBack, goForward, goUp, navigateHistory, notFoundPath, sessionRouteRef, sessionRoute,
    setSessionRoute, requestSessionRoute, navigateShellUri, dockLayoutStore, dockUiStateStore, registry
  };
}
