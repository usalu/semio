import type { ShellCoreAfterDirectoryLaneAndLayout } from "../📐️layout-hotkeys/🟦️.ts";
import { useCatalogState } from "./🔶️catalog/🟦️.ts";
import { useOpenArtifactAppState } from "./🔸️open-artifact-app/🟦️.ts";
import { useOpenEntriesState } from "./🟠️open-entries/🟦️.ts";

export type ShellCoreAfterArtifactOpening = ShellCoreAfterDirectoryLaneAndLayout & ReturnType<typeof useArtifactOpening>;
/**
 * 📂️ Opens artifacts with an app and keeps the session aware of the app router and its conflicts.
 *
 * 📖️ Seeds the Conflicts panel with this session's authoritative roster on session start/switch
 * (`AppCommand::ReadConflicts`, contract freeze §C8/§C9) — otherwise the panel would only ever
 * show conflicts a later `setMergePolicy`/`resolveConflict` reply happened to carry, staying empty
 * across a reload even when the guest already holds `Open` conflicts. Reads
 * `loadedPluginsRef.current` rather than depending on `loadedPlugins` so a plugin hot-swap that
 * leaves the session's pluginId in place doesn't re-fire this on every unrelated roster change.
 */
export function useArtifactOpening(core: ShellCoreAfterDirectoryLaneAndLayout) {
  const { shellStateRef, captureEffectOwner, captureDialogOrigin, isCurrentEffectOwner, loadedPluginsRef, installPlugin, publishPreparedArtifactOpening, loadedPlugins, openArtifactWithAppRefRef, spaceArtifactCreationOwnersRef, openDocumentSessionsRef, setSpaceArtifactCreationUi, resolveArtifactOpeningWithActivationRef, openDocument, closeDocument, retireDocumentAttachment, openReadySpaceArtifactCreationRef, appRouter, knownDialects, pinnedAppFor, pluginLabelById, uiLocale, dispatchSetDefaultApp, dispatchClearDefaultApp, shellState, selectedConflictId, dispatch, dispatchResolveConflict, session, pluginSupervisorByIdRef, showTransientNoticeRef, applyRemoteMergeRef, routerFaultByPluginId, dispatchSetMergePolicy } = useCatalogState({ core });
  const { openArtifactWithAppRef, openReadySpaceArtifactCreation, defaultAppsHost, defaultAppsHostRef, conflictsHost, conflictsHostRef } = useOpenArtifactAppState({ shellStateRef, captureEffectOwner, captureDialogOrigin, isCurrentEffectOwner, loadedPluginsRef, installPlugin, publishPreparedArtifactOpening, loadedPlugins, openArtifactWithAppRefRef, spaceArtifactCreationOwnersRef, openDocumentSessionsRef, setSpaceArtifactCreationUi, resolveArtifactOpeningWithActivationRef, openDocument, closeDocument, retireDocumentAttachment, openReadySpaceArtifactCreationRef, appRouter, knownDialects, pinnedAppFor, pluginLabelById, uiLocale, dispatchSetDefaultApp, dispatchClearDefaultApp, shellState, selectedConflictId, dispatch, dispatchResolveConflict });
  const { openWithEntries, hasOpenArtifactSurfaces } = useOpenEntriesState({ session, loadedPluginsRef, dispatch, pluginSupervisorByIdRef, appRouter, loadedPlugins, pinnedAppFor, pluginLabelById, core, showTransientNoticeRef, applyRemoteMergeRef });
  return {
    appRouter, routerFaultByPluginId, dispatchSetDefaultApp, dispatchClearDefaultApp, dispatchSetMergePolicy,
    openArtifactWithAppRef, openReadySpaceArtifactCreation, defaultAppsHost, defaultAppsHostRef, conflictsHost,
    conflictsHostRef, openWithEntries, hasOpenArtifactSurfaces
  };
}
