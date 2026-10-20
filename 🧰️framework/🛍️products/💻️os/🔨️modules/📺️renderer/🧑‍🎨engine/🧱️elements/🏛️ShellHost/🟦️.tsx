import { OwnedShellDialog } from "./🗨️dialog-origin/🌐️browser/🟦️.tsx";
import { isTerminalDocumentCheckInPhaseV1 } from "@semio-tech/framework/document/check-in";
import { SessionAuthorityNotice } from "./🔖️session-notice/🟦️.tsx";
import React, { useCallback, useEffect, useLayoutEffect, useState } from "react";
import { type Anchor, ANCHORS, CanvasSkeleton, cn, ContextMenuController, createShellScope, detectShellLocale, disposeShellI18nInstance, Footer, Layout, LevelProvider, Navbar, PanelDockProvider, type ShellScope, ShellScopeProvider, TextSelectionContextMenuHost, TutorialBar, UIIntroduction, UiKeybindingsProvider } from "@semio-tech/ui-react";
import { readUiPreferences } from "../🎚️UiPreferences/🟦️.ts";
import { PluginSurfaceActionsContext } from "../🗣️Interpreter/🧷️interpreter-context/🟦️.ts";
import { RowActionRefusalsContext, UiPresenceOverlayContext } from "../🗣️Interpreter/👥️presence-overlay/🟦️.ts";
import { SpaceAdministrationPane } from "../🛂️SpaceAdministration/🟦️.tsx";
import { HubWorkspace } from "../🔗️HubConnection/🏛️workspace/🟦️.tsx";
import { BoardSessionFactoryContext, registerWasmBindingsV1 } from "../🪪️WasmSessionLoader/🟦️.tsx";
import { isEphemeralShellBrand } from "../🐚️Shell/🔒️boot-and-locks/🟦️.ts";
import { ShellFaultBoundary } from "../🐚️Shell/🧯️fault-boundary/🟦️.tsx";
import { registerElementHostsV1 } from "../🪢️ElementHosts/🟦️.ts";
import { pluginInstallBandTextV1, pluginInstallCancelTextV1 } from "../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { checkinAbortText, checkinStatusText } from "../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { renderStagedArgControl, interactionSelectionIdsV1 } from "../🛠️ShellHelpers/🪟️window-panes/📝️staged-args/🟦️.tsx";
import { resolveDialogDefinition, resolveIntroductionDefinition, resolveManifestLabel, shellLabel } from "../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { HistoryReprojectionStatus, TimeTravelBand } from "../🛠️ShellHelpers/⏪️time-travel/🟦️.tsx";
import { localFolderNameV1, LocalFolderReconnectBand } from "./📎️local-folders/🟦️.tsx";
import { parseHubProgramIdV1 } from "@semio-tech/framework/plugin-module/hub-source/resolution";
import { BootstrapStatusNotice, ExecutionTargetStatusNotice, InstalledServicePanelV1 } from "./🪪️host-bootstrap/🟦️.tsx";
import { ArtifactCreationCatalogNotice, ArtifactCreationProgressNotice } from "./🌱️artifact-creation/🟦️.tsx";
import { DirectoryBootstrapStatusNotice } from "./📇️directory-bootstrap/🟦️.tsx";
import { commandStallWatch } from "../🔌️PluginRuntime/🟦️.tsx";
import { commandStallCancelTextV1 } from "../🔌️PluginRuntime/⏱️command-stall/🟦️.ts";
import { UIFind, UIFindProvider, UISearch } from "../🔎️ShellSearch/🟦️.tsx";
import { SetWindowIconContext, SetWindowTitleContext, TRANSIENT_NOTICE_TONE_CLASS } from "./🎚️shell-model/🟦️.tsx";
import { AppCatalogueContext } from "../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";
import { ShellContextMenuFallbackContext, AppKeybindingsContext } from "../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { TutorialCaptionsHost, TutorialGhostPointerHost, TutorialVideoOverlayHost } from "./🎬️tutorial-recorder/🟦️.tsx";
import { type FrameworkOsShellProps, resolveShellScopeStorage } from "./🔐️identity-binding/🟦️.ts";
import { commandStallLineV1 } from "./🔌️extension-runtime/🟦️.ts";
import { type FrameworkOsShellInnerProps } from "./🧩️shell-core-state/🟦️.ts";
import { useCoreState } from "./🔶️core/🟦️.tsx";

export {
  scheduleShellTransientNoticeDismissV1,
  SetWindowIconContext,
  SetWindowTitleContext,
  SHELL_TRANSIENT_NOTICE_AUTO_DISMISS_MS,
  type ShellCatalogProbe,
  shellCatalogProbePlugins
} from "./🎚️shell-model/🟦️.tsx";

export {
  startedTypedOperationV1,
  type StartedTypedOperationV1,
  typedOperationCancellationActionV1,
  typedOperationCancellationOwnerIsCurrentV1
} from "./🏁️operation-settle/🟦️.ts";

export {
  appCommandTakesPageRun,
  appOwnsCommand,
  invocationPluginIdV1,
  windowHostEventHandlersV1
} from "./📨️window-invocation/🟦️.ts";

export {
  tutorialInteractionSelectionActions,
  TutorialRecorder
} from "./🎬️tutorial-recorder/🟦️.tsx";

export {
  canonicalSurfaceId,
  type FrameworkOsShellProps,
  reloadRetainsActiveApp,
  shellActorId
} from "./🔐️identity-binding/🟦️.ts";

export {
  type ArtifactKindChoiceDraftDefinitionV1,
  artifactKindChoiceDraftRetirementsV1,
  captureSpaceArtifactCreationCatalogAuthorityV1,
  DIRECTORY_COMMAND_RESULT_SLOTS,
  directoryCommandFromAction,
  type DirectoryCommandResultSlotV1,
  mintDirectoryCommandRequestId,
  retainDirectoryCommandResult,
  selectedSpaceArtifactCreationCatalogV1,
  type SpaceArtifactCreationCatalogAuthorityV1,
  type SpaceArtifactCreationCatalogProjectionV1,
  spaceArtifactCreationCatalogRefreshRequestV1,
  spaceArtifactCreationOwnerAcceptsStatus,
  type SpaceArtifactCreationOwnerV1,
  spaceArtifactCreationReadyOpening,
  spaceArtifactCreationRequestFromAction,
  spaceIndexOpeningArgsV1,
  SURFACE_ROLE_ICON_IDS
} from "./🧾️space-artifact-creation/🟦️.ts";

export {
  copyDirectoryInviteCapabilityV1,
  type DirectoryInviteClipboardV1,
  reduceShellSpaceAdministrationState,
  SHELL_SPACE_ADMINISTRATION_INVITE_TTL_SECS,
  shellSpaceAdministrationCapabilityAllowed,
  shellSpaceAdministrationOpening,
  shellSpaceAdministrationRequest,
  type ShellSpaceAdministrationStateV1
} from "./🏠️space-administration/🟦️.ts";

export {
  dispatchInvokeExtensionEffect,
  EXTENSION_WORKER_LOST_FAULT,
  extensionProgramForV1,
  runInvokeExtensionEffect,
  RUNTIME_DIAGNOSTICS_KEY,
  runtimeDiagnosticsEnabled,
  setRuntimeDiagnostics
} from "./🔌️extension-runtime/🟦️.ts";

export {
  type BuiltNodeStoreCacheV1,
  createBuiltNodeStoreCacheV1,
  publishBuiltNodesV1,
  publishPanelBodiesV1
} from "./🧱️built-node-stores/🟦️.ts";

export type { DirectoryCommandErrorCodeV1, DirectoryCommandReceiptV1 } from "@semio-tech/framework/directory/schema";

/** 🪢️ The shell mounts windows through the Interpreter, so it is a composition root of the element hosts. */
registerElementHostsV1();

//#region FrameworkOsShell
//#region 🐚️ShellMount
/** 🐚️ Mounts a `.semio-scope` root (theme/appearance/id scoping lands with later waves) carrying a
 * {@link ShellScope} — the seam that lets several of these coexist on one page — around the actual shell
 * implementation in {@link FrameworkOsShellInner}. */
export function FrameworkOsShell(props: FrameworkOsShellProps): React.ReactElement {
  const { shellId, storageNamespace, ownsPage = false, brand, locks, initialLocale: requestedLocale, ...innerProps } = props;
  const ephemeral = isEphemeralShellBrand(brand);
  const wasmBindings = props.hostPorts?.wasmBindings;
  useLayoutEffect(() => (wasmBindings === undefined ? undefined : registerWasmBindingsV1(wasmBindings)), [wasmBindings]);
  const [scope] = useState<ShellScope>(() => {
    const storage = resolveShellScopeStorage(ephemeral, storageNamespace);
    const initialLocale = locks?.locale ?? readUiPreferences(storage).locale ?? requestedLocale ?? detectShellLocale(typeof navigator !== "undefined" ? navigator.language : undefined);
    return createShellScope({ shellId, ownsPage, storage, initialLocale });
  });
  const [, bumpAfterRootAttach] = useState(0);
  const setRoot = useCallback((node: HTMLDivElement | null) => {
    scope.rootRef.current = node;
    bumpAfterRootAttach((n) => n + 1);
  }, [scope]);
  const setPortalLayer = useCallback((node: HTMLDivElement | null) => {
    scope.portalLayerRef.current = node;
    bumpAfterRootAttach((n) => n + 1);
  }, [scope]);
  useEffect(() => () => disposeShellI18nInstance(scope.i18n), [scope]);
  return (
    <div ref={setRoot} className="semio-scope" data-shell-id={scope.shellId} style={{ position: "relative", height: "100%", width: "100%", isolation: "isolate" }}>
      <ShellScopeProvider scope={scope}>
        <FrameworkOsShellInner {...innerProps} locks={locks} brand={brand} />
        <div data-semio-portal-layer ref={setPortalLayer} className="pointer-events-none absolute inset-0" />
      </ShellScopeProvider>
    </div>
  );
}
function FrameworkOsShellInner(props: FrameworkOsShellInnerProps) {
  const { boardSessionFactory, setWindowTitle, setWindowIcon, keysByActionId, appCatalogue, controlKeybindings, timeTravelPeers, rowActionRefusals, requestContextMenu, buildShellContextMenuItems, session, initialExampleReady, error, agentBridgeInstances, historyDomRef, hubEnv, hubSessionCapability, verifiedSessionAuthority, identityOffline, uiLocale, cancelSessionAuthorityBootstrap, bootstrapUiByDocument, closeDocument, executionTargetUiByDocument, spaceArtifactCreationUi, spaceArtifactCreationCatalogUi, cancelSpaceArtifactCreation, openReadySpaceArtifactCreation, inferencePortRuntimeKey, inferencePort, documentServices, dispatchInferencePortIntent, directoryBootstrapUi, cancelDirectoryBootstrap, hubWorkspaceOpen, hubConnectionPort, openSpaceId, hubOnlineUserIds, navigateShellUri, setHubWorkspaceOpen, hostMode, navigateHistory, sessionRouteRef, spaceAdministration, dispatchSpaceAdministrationIntent, routeAdmissionNotice, routeAdmission, openHubWorkspace, transientNotice, dispatch, installingPluginIds, pluginInstallProgress, cancelPluginInstalls, commandStalls, loadedPlugins, panel, uiTerminology, commandStallClockMs, dock, handleTabDockDrop, handleTreeUnitDockDrop, mobile, mobilePanel, navbarItems, onCommand, activeTutorial, tutorialPlaying, tutorialRate, tutorialMuted, tutorialCaptionsOn, tutorialRecording, tutorialRecorderAvailable, tutorialChapterMarkers, tutorialClock, playPauseTutorial, stopTutorial, seekTutorial, toggleTutorialRecording, addTutorialChapter, footerItems, focusedTimeTravel, focusedReprojection, currentCheckIn, folderReconnectOffer, folderReconnectBusy, reconnectLocalFolder, forgetLocalFolder, focusedApp, onActionStable, currentDocumentRuntimeKey, dismissHubCheckIn, cancelHubCheckIn, buildPanelProps, shellPluginCanvasStatus, canvas, searchItems, searchOpen, findOpen, shellContextMenuTitleLabel, shellContextMenu, setShellContextMenu, activeIntroduction, introductionStepIndex, brand, appLabelsOverlay, introductionCompletedInteractions, dismissIntroduction, overlayDialog, isCurrentDialogOrigin, localPrograms, selectedSpaceArtifactKinds, selectedSpaceArtifactCreationCatalog, localInteraction, closeOwnedDialog, onAction } = useCoreState({ props });
  return (
    <BoardSessionFactoryContext.Provider value={boardSessionFactory}>
    <SetWindowTitleContext.Provider value={setWindowTitle}>
    <SetWindowIconContext.Provider value={setWindowIcon}>
    <AppKeybindingsContext.Provider value={keysByActionId}>
    <AppCatalogueContext.Provider value={appCatalogue}>
    <UiKeybindingsProvider bindings={controlKeybindings}>
    <UiPresenceOverlayContext.Provider value={timeTravelPeers.overlay}>
    <RowActionRefusalsContext.Provider value={rowActionRefusals}>
    <PluginSurfaceActionsContext.Provider value={requestContextMenu}>
    <ShellContextMenuFallbackContext.Provider value={buildShellContextMenuItems}>
    <ShellFaultBoundary boundaryId="shell-root" fallbackLabel={shellLabel("ui.common.renderError")}>
    <UIFindProvider>
      <LevelProvider level="base">
        <div
          className="flex h-full min-h-0 w-full flex-col bg-transparent"
          data-level="base"
          data-semio-os-ready={session && initialExampleReady && !error ? "" : undefined}
          data-semio-artifact-id={agentBridgeInstances[0]?.artifactRef}
          ref={historyDomRef}
        >
          {hubEnv && hubSessionCapability !== null && verifiedSessionAuthority === null ? <SessionAuthorityNotice state={identityOffline ? "unavailable" : "pending"} locale={uiLocale} onCancel={cancelSessionAuthorityBootstrap} /> : null}
          {Object.values(bootstrapUiByDocument).length > 0 ? (
            <div className="pointer-events-auto absolute top-workbench left-1/2 z-50 flex -translate-x-1/2 flex-col gap-single rounded-sm border bg-base px-double py-single text-sm shadow-sm">
              {Object.entries(bootstrapUiByDocument).map(([runtimeKey, status]) => (
                <BootstrapStatusNotice key={runtimeKey} status={status} locale={uiLocale} onCancel={() => closeDocument(runtimeKey)} />
              ))}
            </div>
          ) : null}
          {Object.values(executionTargetUiByDocument).length > 0 ? (
            <div className="pointer-events-auto absolute top-workbench left-1/2 z-50 mt-double flex -translate-x-1/2 flex-col gap-single rounded-sm border bg-base px-double py-single text-sm shadow-sm">
              {Object.entries(executionTargetUiByDocument).map(([runtimeKey, status]) => (
                <ExecutionTargetStatusNotice key={runtimeKey} status={status} locale={uiLocale} onCancel={() => closeDocument(runtimeKey)} />
              ))}
            </div>
          ) : null}
          {Object.values(spaceArtifactCreationUi).length > 0 || spaceArtifactCreationCatalogUi !== null ? (
            <div className="pointer-events-auto absolute top-workbench left-double z-50 flex max-w-[28rem] flex-col gap-single rounded-sm border bg-base px-double py-single text-sm shadow-sm">
              {spaceArtifactCreationCatalogUi === null ? null : <ArtifactCreationCatalogNotice status={spaceArtifactCreationCatalogUi} locale={uiLocale} />}
              {Object.values(spaceArtifactCreationUi).map((status) => (
                <ArtifactCreationProgressNotice key={status.requestId} state={status} locale={uiLocale} onCancel={cancelSpaceArtifactCreation} onOpen={openReadySpaceArtifactCreation} />
              ))}
            </div>
          ) : null}
          {inferencePortRuntimeKey !== null && inferencePort !== undefined && documentServices.some((entry) => entry.owner === inferencePort.owner && entry.serviceId === inferencePort.serviceId) ? (
            <div className="pointer-events-auto absolute top-workbench left-1/2 z-50 w-[28rem] -translate-x-1/2 rounded-sm border bg-base px-double py-single text-sm shadow-sm">
              <InstalledServicePanelV1 presentation={documentServices.find((entry) => entry.owner === inferencePort.owner && entry.serviceId === inferencePort.serviceId)!} status={inferencePort} locale={uiLocale === "de" ? "de" : "en"} onAction={(action) => dispatchInferencePortIntent(inferencePortRuntimeKey, action)} />
            </div>
          ) : null}
          {directoryBootstrapUi.kind !== "idle" ? (
            <div className="pointer-events-auto absolute top-workbench right-double z-50 rounded-sm border bg-base px-double py-single text-sm shadow-sm">
              <DirectoryBootstrapStatusNotice state={directoryBootstrapUi} locale={uiLocale} onCancel={cancelDirectoryBootstrap} />
            </div>
          ) : null}
          {hubWorkspaceOpen ? (
            <div data-elevation-root="" className="pointer-events-auto absolute top-workbench left-1/2 z-50 max-h-[80vh] w-full max-w-[44rem] -translate-x-1/2 overflow-auto rounded-sm border bg-base shadow-sm">
              <HubWorkspace
                port={hubConnectionPort}
                locale={uiLocale === "de" ? "de" : "en"}
                activeSpaceId={openSpaceId}
                onlineUserIds={hubOnlineUserIds}
                onOpenSpace={(spaceId) => navigateShellUri(`/spaces/${spaceId}`)}
                onClose={() => {
                  setHubWorkspaceOpen(false);
                  if (hostMode) navigateHistory(sessionRouteRef.current ?? "/");
                }}
              />
            </div>
          ) : null}
          {spaceAdministration !== null ? (
            <div className="pointer-events-auto absolute top-workbench left-1/2 z-50 max-h-[70vh] w-[36rem] -translate-x-1/2 overflow-auto rounded-sm border bg-base shadow-sm">
              <SpaceAdministrationPane
                spaceId={spaceAdministration.spaceId}
                phase={spaceAdministration.phase}
                page={spaceAdministration.page}
                receiptSha256={spaceAdministration.receiptSha256}
                code={spaceAdministration.code}
                inviteCapabilityPending={spaceAdministration.inviteCapabilityPending}
                inviteCapabilityStatus={spaceAdministration.inviteCapabilityStatus}
                onIntent={dispatchSpaceAdministrationIntent}
              />
            </div>
          ) : null}
          {routeAdmissionNotice === null ? null : (
            <div role="status" aria-live="polite" data-semio-route-admission={routeAdmission} className="pointer-events-auto absolute bottom-double left-1/2 z-40 flex max-w-[90vw] -translate-x-1/2 flex-wrap items-center gap-single rounded-sm border bg-base px-double py-single text-sm shadow-sm">
              <span>{routeAdmissionNotice.text}</span>
              {routeAdmissionNotice.action === null ? null : (
                <button type="button" data-semio-route-admission-action="" className="underline" onClick={openHubWorkspace}>
                  {routeAdmissionNotice.action}
                </button>
              )}
            </div>
          )}
          {transientNotice ? (
            <div
              role="status"
              aria-live="polite"
              data-semio-transient-notice=""
              data-notice-code={transientNotice.code}
              className={cn("pointer-events-auto absolute top-workbench left-1/2 z-50 -translate-x-1/2 rounded-sm border px-double py-single text-sm shadow-sm", TRANSIENT_NOTICE_TONE_CLASS[transientNotice.kind])}
            >
              {transientNotice.message}
              <button type="button" className="ml-single underline" onClick={() => dispatch({ type: "SET_TRANSIENT_NOTICE", value: null })}>
                {shellLabel("ui.common.close")}
              </button>
            </div>
          ) : null}
          {installingPluginIds.length > 0 ? (
            <div
              role="status"
              aria-live="polite"
              data-semio-plugin-install=""
              data-plugin-install-ids={installingPluginIds.join(",")}
              className="pointer-events-auto absolute top-workbench left-1/2 z-50 mt-double -translate-x-1/2 rounded-sm border border-normal bg-menu px-double py-single text-sm shadow-sm"
            >
              {pluginInstallBandTextV1(installingPluginIds.map((programId) => parseHubProgramIdV1(programId)?.pluginId ?? programId), uiLocale, pluginInstallProgress)}
              {pluginInstallProgress ? (
                <progress
                  className="ml-single align-middle"
                  data-semio-plugin-install-progress=""
                  value={pluginInstallProgress.completedBytes}
                  max={pluginInstallProgress.totalBytes}
                  aria-label={pluginInstallBandTextV1(installingPluginIds, uiLocale, pluginInstallProgress)}
                />
              ) : null}
              <button type="button" className="ml-single underline" data-semio-plugin-install-cancel="" onClick={cancelPluginInstalls}>
                {pluginInstallCancelTextV1(uiLocale)}
              </button>
            </div>
          ) : null}
          {commandStalls.length > 0 ? (
            <div
              role="status"
              aria-live="polite"
              data-semio-command-stall=""
              className={cn("pointer-events-auto absolute top-workbench left-1/2 z-50 mt-double -translate-x-1/2 rounded-sm border border-normal bg-menu px-double py-single text-sm shadow-sm", installingPluginIds.length > 0 && "translate-y-full")}
            >
              {commandStalls.map((stall) => (
                <div key={stall.id} data-semio-command-stall-id={stall.id} data-command-id={stall.commandId ?? ""}>
                  {commandStallLineV1(stall, loadedPlugins, panel?.spawnedApps ?? [], uiTerminology, uiLocale, commandStallClockMs)}
                  <button type="button" className="ml-single underline" data-semio-command-stall-cancel="" onClick={() => commandStallWatch.cancel(stall.id)}>
                    {commandStallCancelTextV1(uiLocale)}
                  </button>
                </div>
              ))}
            </div>
          ) : null}
          <PanelDockProvider dock={dock} onTabDockDrop={handleTabDockDrop} onTreeUnitDockDrop={handleTreeUnitDockDrop}>
            <Layout
              mobile={mobile}
              mobilePanel={mobilePanel}
              navbar={
                <Navbar
                  items={navbarItems}
                  showFullscreenToggle={!mobile}
                  onFullscreenToggle={() => onCommand({ owner: "os", commandId: "os.toggleFullscreen" })}
                />
              }
              subnavbar={
                activeTutorial ? (
                  <TutorialBar
                    title={resolveManifestLabel(activeTutorial.title, uiTerminology, uiLocale)}
                    durationMs={activeTutorial.durationMs}
                    playing={tutorialPlaying}
                    rate={tutorialRate}
                    muted={tutorialMuted}
                    captionsOn={tutorialCaptionsOn}
                    recording={tutorialRecording}
                    recordAvailable={tutorialRecorderAvailable}
                    chapters={tutorialChapterMarkers}
                    clock={tutorialClock}
                    onPlayPause={playPauseTutorial}
                    onStop={stopTutorial}
                    onSeek={seekTutorial}
                    onRateChange={(value) => dispatch({ type: "SET_TUTORIAL_RATE", value })}
                    onMutedChange={(value) => dispatch({ type: "SET_TUTORIAL_MUTED", value })}
                    onCaptionsChange={(value) => dispatch({ type: "SET_TUTORIAL_CAPTIONS", value })}
                    onRecordToggle={toggleTutorialRecording}
                    onAddChapter={addTutorialChapter}
                  />
                ) : undefined
              }
              footer={<Footer items={footerItems} />}
              subfooter={
                focusedTimeTravel !== null || focusedReprojection !== null || currentCheckIn !== undefined || folderReconnectOffer !== null ? (
                  <div data-semio-bottom-bands="" className="flex flex-col items-center gap-single px-double py-single">
                    {folderReconnectOffer === null ? null : (
                      <LocalFolderReconnectBand
                        label={String(shellLabel("ui.sync.reconnect.label"))}
                        message={String(shellLabel("ui.sync.reconnect.message", { folder: localFolderNameV1(folderReconnectOffer.folder.path) }))}
                        reconnect={String(shellLabel("ui.sync.reconnect.attach"))}
                        forget={String(shellLabel("ui.sync.reconnect.forget"))}
                        busy={folderReconnectBusy}
                        onReconnect={() => reconnectLocalFolder(folderReconnectOffer)}
                        onForget={() => forgetLocalFolder(folderReconnectOffer.documentId)}
                      />
                    )}
                    {focusedTimeTravel === null || focusedApp === null ? null : <TimeTravelBand session={focusedTimeTravel} terminology={uiTerminology} locale={uiLocale} controllerId={focusedApp.controllerId} onAction={onActionStable} />}
                    {focusedReprojection === null || focusedApp === null ? null : <HistoryReprojectionStatus reprojection={focusedReprojection} locale={uiLocale} sessionOpen={focusedTimeTravel !== null} controllerId={focusedApp.controllerId} onAction={onActionStable} />}
                    {currentCheckIn === undefined || currentDocumentRuntimeKey === null ? null : (
                      <div role="status" aria-live="polite" data-semio-checkin-band={currentCheckIn.status.phase} className="pointer-events-auto flex flex-wrap items-center gap-single rounded-sm border border-normal bg-menu px-double py-single text-sm shadow-sm">
                        <span id="s-checkin-status">{checkinStatusText(currentCheckIn.status, uiLocale)}</span>
                        {isTerminalDocumentCheckInPhaseV1(currentCheckIn.status.phase) ? (
                          <button type="button" id="s-checkin-dismiss" className="underline" onClick={() => dismissHubCheckIn(currentDocumentRuntimeKey)}>
                            {shellLabel("ui.common.close")}
                          </button>
                        ) : (
                          <>
                            {currentCheckIn.status.progress.totalUnits > 0 ? <progress value={currentCheckIn.status.progress.completedUnits} max={currentCheckIn.status.progress.totalUnits} aria-label={checkinStatusText(currentCheckIn.status, uiLocale)} /> : null}
                            <button type="button" id="s-checkin-abort" className="underline" onClick={() => cancelHubCheckIn(currentDocumentRuntimeKey)}>
                              {checkinAbortText(uiLocale)}
                            </button>
                          </>
                        )}
                      </div>
                    )}
                  </div>
                ) : undefined
              }
              panels={Object.fromEntries(ANCHORS.map((anchor) => [anchor, buildPanelProps(anchor)])) as Record<Anchor, ReturnType<typeof buildPanelProps>>}
              canvasStatus={shellPluginCanvasStatus}
              canvasSkeleton={<CanvasSkeleton label={shellLabel("ui.common.loadingPlugins")} />}
              canvas={
                <ShellFaultBoundary boundaryId="route-canvas" fallbackLabel={shellLabel("ui.common.renderError")}>
                  {canvas}
                </ShellFaultBoundary>
              }
            />
          </PanelDockProvider>
        </div>
        <UISearch items={searchItems} open={searchOpen} onOpenChange={(value) => dispatch({ type: "SET_SEARCH_OPEN", value })} />
        <UIFind open={findOpen} onOpenChange={(value) => dispatch({ type: "SET_FIND_OPEN", value })} />
        <TextSelectionContextMenuHost />
        <ContextMenuController
          title={shellContextMenuTitleLabel}
          open={shellContextMenu != null}
          position={shellContextMenu}
          items={shellContextMenu?.items ?? []}
          onOpenChange={(open) => {
            if (!open) setShellContextMenu(null);
          }}
        />
        {session && activeIntroduction && introductionStepIndex != null && (
          <UIIntroduction
            introduction={brand?.introduction ?? resolveIntroductionDefinition(activeIntroduction, appLabelsOverlay, uiTerminology, uiLocale)}
            stepIndex={introductionStepIndex}
            completedInteractionIndices={introductionCompletedInteractions}
            onStepIndexChange={(value) => dispatch({ type: "SET_INTRODUCTION_STEP", value })}
            onDismiss={dismissIntroduction}
          />
        )}
        {activeTutorial && (
          <>
            <TutorialCaptionsHost tutorial={activeTutorial} clock={tutorialClock} captionsOn={tutorialCaptionsOn} terminology={uiTerminology} locale={uiLocale} />
            <TutorialVideoOverlayHost tutorial={activeTutorial} clock={tutorialClock} muted={tutorialMuted} playing={tutorialPlaying} rate={tutorialRate} />
            <TutorialGhostPointerHost tutorial={activeTutorial} clock={tutorialClock} />
          </>
        )}
        {session &&
          overlayDialog &&
          (() => {
            if (!isCurrentDialogOrigin(overlayDialog.origin)) return null;
            const dialog = session.app.dialogs?.find((entry) => entry.id === overlayDialog.dialogId);
            if (!dialog) return null;
            const resolved = resolveDialogDefinition(dialog, appLabelsOverlay, uiTerminology, uiLocale, localPrograms.map((entry) => entry.manifest), selectedSpaceArtifactKinds);
            const artifactKindArgIds = resolved.args.filter((def) => def.schema.kind === "string" && def.schema.format?.kind === "artifactKind").map((def) => def.id);
            const creationCatalog = artifactKindArgIds.length > 0 ? selectedSpaceArtifactCreationCatalog : null;
            const choiceRevisions = creationCatalog === null ? undefined : Object.fromEntries(artifactKindArgIds.map((id) => [id, creationCatalog.choiceRevision]));
            return (
              <OwnedShellDialog
                owner={overlayDialog}
                dialog={resolved}
                renderField={(def, value, onChange, field) => renderStagedArgControl(def, value, onChange, false, field, { selection: (domain) => interactionSelectionIdsV1(localInteraction.get(), domain) })}
                notice={creationCatalog === null ? undefined : <ArtifactCreationCatalogNotice status={creationCatalog} locale={uiLocale} hasChoices={creationCatalog.kinds.length > 0} />}
                choiceRevisions={choiceRevisions}
                isCurrent={isCurrentDialogOrigin}
                close={closeOwnedDialog}
                dispatch={(action, origin, args) => onAction({ controllerId: origin.controllerId, action, args }, origin)}
              />
            );
          })()}
      </LevelProvider>
    </UIFindProvider>
    </ShellFaultBoundary>
    </ShellContextMenuFallbackContext.Provider>
    </PluginSurfaceActionsContext.Provider>
    </RowActionRefusalsContext.Provider>
    </UiPresenceOverlayContext.Provider>
    </UiKeybindingsProvider>
    </AppCatalogueContext.Provider>
    </AppKeybindingsContext.Provider>
    </SetWindowIconContext.Provider>
    </SetWindowTitleContext.Provider>
    </BoardSessionFactoryContext.Provider>
  );
}
