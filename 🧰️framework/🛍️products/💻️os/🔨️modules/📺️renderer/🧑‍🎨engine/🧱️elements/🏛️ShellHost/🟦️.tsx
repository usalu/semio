import { StagedActionDialog } from "./📝️staging/🟦️.tsx";
import { OwnedShellDialog } from "./🗨️dialog-origin/🌐️browser/🟦️.tsx";
import { DocumentCloseDialog, UnsavedMarkerContext } from "./🚪️close-guard/🟦️.tsx";
import { TransientNotice } from "./🍞️transient-notice/🟦️.tsx";
import { isTerminalDocumentCheckInPhaseV1 } from "@semio-tech/framework/document/check-in";
import { SessionAuthorityNotice } from "./🔖️session-notice/🟦️.tsx";
import * as React from "../⚛️react-port/🟦️.ts";
import { useCallback, useEffect, useLayoutEffect, useRef, useState } from "../⚛️react-port/🟦️.ts";
import { localeRequestToApply } from "./🌐️locale-request/🟦️.ts";
import { type Anchor, ANCHORS, Button, CanvasSkeleton, cn, ContextMenuController, ContextMenuTriggerButton, createShellScope, detectShellLocale, disposeShellI18nInstance, Footer, Layout, LevelProvider, Navbar, PanelDockProvider, type ShellScope, SharedBorderRow, SharedBorderRowCell, ShellScopeProvider, TextSelectionContextMenuHost, TutorialBar, UIIntroduction, UiKeybindingsProvider } from "@semio-tech/ui-react";
import { commitUiPreferencesConfigMutation, readUiPreferences } from "../🎚️UiPreferences/🟦️.ts";
import { setLocale } from "@semio-tech/framework/config/mutations";
import type { ShellLocale } from "@semio-tech/framework";
import { ShellLocalePicker } from "./🌐️locale-picker/🟦️.tsx";
import { useShellPlatformPrefs } from "./🎛️platform-prefs/🟦️.ts";
import { PluginSurfaceActionsContext } from "../🗣️Interpreter/🧷️interpreter-context/🟦️.ts";
import { RowActionRefusalsContext, UiPresenceOverlayContext } from "../🗣️Interpreter/👥️presence-overlay/🟦️.ts";
import { SpaceAdministrationPane } from "../🛂️SpaceAdministration/🟦️.tsx";
import { HubWorkspace } from "../🔗️HubConnection/🏛️workspace/🟦️.tsx";
import { BoardSessionFactoryContext, registerWasmBindingsV1 } from "../🪪️WasmSessionLoader/🟦️.tsx";
import { isEphemeralShellBrand } from "../🐚️Shell/🔒️boot-and-locks/🟦️.ts";
import { FloatingPane, NoticeActionRow, NoticeCell, NoticeStack } from "./🧱️notice-bands/🟦️.tsx";
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
  const { storageNamespace, brand, locks, initialLocale: requestedLocale } = props;
  const [resolvedLocale] = useState<ShellLocale | undefined>(() => locks?.locale ?? readUiPreferences(resolveShellScopeStorage(isEphemeralShellBrand(brand), storageNamespace)).locale ?? requestedLocale ?? detectShellLocale(typeof navigator !== "undefined" ? navigator.language : undefined));
  const [chosenLocale, setChosenLocale] = useState<ShellLocale | undefined>(undefined);
  const initialLocale = resolvedLocale ?? chosenLocale;
  const choose = useCallback((locale: ShellLocale) => {
    commitUiPreferencesConfigMutation(resolveShellScopeStorage(isEphemeralShellBrand(brand), storageNamespace), setLocale(locale));
    setChosenLocale(locale);
  }, [brand, storageNamespace]);
  return initialLocale === undefined ? <ShellLocalePicker onChoose={choose} /> : <FrameworkOsShellMounted {...props} initialLocale={initialLocale} requestedLocale={requestedLocale} />;
}

function FrameworkOsShellMounted(props: FrameworkOsShellProps & { readonly initialLocale: ShellLocale; readonly requestedLocale: ShellLocale | undefined }): React.ReactElement {
  const { shellId, storageNamespace, ownsPage = false, brand, locks, initialLocale, requestedLocale, ...innerProps } = props;
  const ephemeral = isEphemeralShellBrand(brand);
  const wasmBindings = props.hostPorts?.wasmBindings;
  useLayoutEffect(() => (wasmBindings === undefined ? undefined : registerWasmBindingsV1(wasmBindings)), [wasmBindings]);
  const [scope] = useState<ShellScope>(() => {
    const storage = resolveShellScopeStorage(ephemeral, storageNamespace);
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
  const [rootAttached, setRootAttached] = useState<Element | null>(null);
  useLayoutEffect(() => setRootAttached(scope.rootRef.current));
  useShellPlatformPrefs(scope.rootRef, rootAttached);
  const appliedLocale = useRef<ShellLocale>(requestedLocale ?? initialLocale);
  useEffect(() => {
    const next = localeRequestToApply(appliedLocale.current, requestedLocale, locks?.locale);
    if (next === undefined) return;
    appliedLocale.current = next;
    commitUiPreferencesConfigMutation(scope.storage, setLocale(next));
  }, [requestedLocale, locks?.locale, scope]);
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
  const { boardSessionFactory, setWindowTitle, setWindowIcon, keysByActionId, appCatalogue, controlKeybindings, timeTravelPeers, rowActionRefusals, requestContextMenu, buildShellContextMenuItems, session, initialExampleReady, error, agentBridgeInstances, historyDomRef, hubEnv, hubSessionCapability, verifiedSessionAuthority, identityOffline, uiLocale, cancelSessionAuthorityBootstrap, bootstrapUiByDocument, closeDocument, closePrompt, currentSavedState, executionTargetUiByDocument, spaceArtifactCreationUi, spaceArtifactCreationCatalogUi, cancelSpaceArtifactCreation, openReadySpaceArtifactCreation, inferencePortRuntimeKey, inferencePort, documentServices, dispatchInferencePortIntent, directoryBootstrapUi, cancelDirectoryBootstrap, hubWorkspaceOpen, hubConnectionPort, openSpaceId, hubOnlineUserIds, navigateShellUri, setHubWorkspaceOpen, hostMode, navigateHistory, sessionRouteRef, spaceAdministration, dispatchSpaceAdministrationIntent, routeAdmissionNotice, routeAdmission, openHubWorkspace, transientNotice, dispatch, installingPluginIds, pluginInstallProgress, cancelPluginInstalls, commandStalls, loadedPlugins, panel, uiTerminology, commandStallClockMs, dock, handleTabDockDrop, handleTreeUnitDockDrop, mobile, mobilePanel, navbarItems, onCommand, activeTutorial, tutorialPlaying, tutorialRate, tutorialMuted, tutorialCaptionsOn, tutorialRecording, tutorialRecorderAvailable, tutorialChapterMarkers, tutorialClock, playPauseTutorial, stopTutorial, seekTutorial, toggleTutorialRecording, addTutorialChapter, canvasChips, footerItems, focusedTimeTravel, focusedReprojection, currentCheckIn, folderReconnectOffer, folderReconnectBusy, reconnectLocalFolder, forgetLocalFolder, focusedApp, onActionStable, currentDocumentRuntimeKey, dismissHubCheckIn, cancelHubCheckIn, buildPanelProps, shellPluginCanvasStatus, canvas, searchItems, searchOpen, findOpen, shellContextMenuTitleLabel, shellContextMenu, setShellContextMenu, activeIntroduction, introductionStepIndex, brand, appLabelsOverlay, introductionCompletedInteractions, dismissIntroduction, overlayDialog, isCurrentDialogOrigin, localPrograms, selectedSpaceArtifactKinds, selectedSpaceArtifactCreationCatalog, localInteraction, stagedActions, closeOwnedDialog, onAction } = useCoreState({ props });
  const unsavedMarkerValue = React.useMemo(() => ({ saved: currentSavedState, locale: uiLocale }), [currentSavedState, uiLocale]);
  return (
    <BoardSessionFactoryContext.Provider value={boardSessionFactory}>
    <SetWindowTitleContext.Provider value={setWindowTitle}>
    <SetWindowIconContext.Provider value={setWindowIcon}>
    <AppKeybindingsContext.Provider value={keysByActionId}>
    <UnsavedMarkerContext.Provider value={unsavedMarkerValue}>
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
            <NoticeStack data-semio-notice-stack="bootstrap" className="top-workbench left-1/2 -translate-x-1/2">
              {Object.entries(bootstrapUiByDocument).map(([runtimeKey, status]) => (
                <NoticeCell key={runtimeKey}><BootstrapStatusNotice status={status} locale={uiLocale} onCancel={() => closeDocument(runtimeKey)} /></NoticeCell>
              ))}
            </NoticeStack>
          ) : null}
          {Object.values(executionTargetUiByDocument).length > 0 ? (
            <NoticeStack data-semio-notice-stack="execution-target" className="top-workbench left-1/2 mt-double -translate-x-1/2">
              {Object.entries(executionTargetUiByDocument).map(([runtimeKey, status]) => (
                <NoticeCell key={runtimeKey}><ExecutionTargetStatusNotice status={status} locale={uiLocale} onCancel={() => closeDocument(runtimeKey)} /></NoticeCell>
              ))}
            </NoticeStack>
          ) : null}
          {Object.values(spaceArtifactCreationUi).length > 0 || spaceArtifactCreationCatalogUi !== null ? (
            <NoticeStack data-semio-notice-stack="artifact-creation" className="top-workbench left-double max-w-[28rem]">
              {spaceArtifactCreationCatalogUi === null ? null : <NoticeCell><ArtifactCreationCatalogNotice status={spaceArtifactCreationCatalogUi} locale={uiLocale} /></NoticeCell>}
              {Object.values(spaceArtifactCreationUi).map((status) => (
                <NoticeCell key={status.requestId}><ArtifactCreationProgressNotice state={status} locale={uiLocale} onCancel={cancelSpaceArtifactCreation} onOpen={openReadySpaceArtifactCreation} /></NoticeCell>
              ))}
            </NoticeStack>
          ) : null}
          {inferencePortRuntimeKey !== null && inferencePort !== undefined && documentServices.some((entry) => entry.owner === inferencePort.owner && entry.serviceId === inferencePort.serviceId) ? (
            <FloatingPane data-semio-floating-pane="installed-service" className="top-workbench left-1/2 w-[28rem] -translate-x-1/2 text-sm" scrollClassName="px-double py-single">
              <InstalledServicePanelV1 presentation={documentServices.find((entry) => entry.owner === inferencePort.owner && entry.serviceId === inferencePort.serviceId)!} status={inferencePort} locale={uiLocale === "de" ? "de" : "en"} onAction={(action) => dispatchInferencePortIntent(inferencePortRuntimeKey, action)} />
            </FloatingPane>
          ) : null}
          {directoryBootstrapUi.kind !== "idle" ? (
            <NoticeStack className="top-workbench right-double">
              <NoticeCell><DirectoryBootstrapStatusNotice state={directoryBootstrapUi} locale={uiLocale} onCancel={cancelDirectoryBootstrap} /></NoticeCell>
            </NoticeStack>
          ) : null}
          {hubWorkspaceOpen ? (
            <FloatingPane data-elevation-root="" data-semio-floating-pane="hub-workspace" className="top-workbench left-1/2 max-h-[80vh] w-full max-w-[44rem] -translate-x-1/2">
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
            </FloatingPane>
          ) : null}
          {spaceAdministration !== null ? (
            <FloatingPane data-semio-floating-pane="space-administration" className="top-workbench left-1/2 max-h-[70vh] w-[36rem] -translate-x-1/2">
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
            </FloatingPane>
          ) : null}
          {routeAdmissionNotice === null ? null : (
            <NoticeStack orientation="horizontal" role="status" aria-live="polite" data-semio-route-admission={routeAdmission} className="bottom-[var(--footer-band-height)] left-1/2 z-40 max-w-[90vw] -translate-x-1/2">
              <NoticeCell>{routeAdmissionNotice.text}</NoticeCell>
              {routeAdmissionNotice.action === null ? null : (
                <Button icon="arrow-right" type="button" variant="outline" data-semio-route-admission-action="" onClick={openHubWorkspace}>
                  {routeAdmissionNotice.action}
                </Button>
              )}
            </NoticeStack>
          )}
          {closePrompt === null ? null : <DocumentCloseDialog name={closePrompt.name} locale={uiLocale} failure={closePrompt.failure} saving={closePrompt.phase === "saving"} onKeep={closePrompt.onKeep} onDiscard={closePrompt.onDiscard} />}
          {transientNotice ? <TransientNotice notice={transientNotice} closeLabel={shellLabel("ui.common.close")} onClose={() => dispatch({ type: "SET_TRANSIENT_NOTICE", value: null })} /> : null}
          {installingPluginIds.length > 0 || commandStalls.length > 0 ? (
            <NoticeStack data-semio-notice-stack="install" className="top-[var(--navbar-band-height)] left-1/2 max-w-[90vw] -translate-x-1/2">
              {installingPluginIds.length === 0 ? null : (
                <NoticeActionRow
                  role="status"
                  aria-live="polite"
                  data-semio-plugin-install=""
                  data-plugin-install-ids={installingPluginIds.join(",")}
                  text={pluginInstallBandTextV1(installingPluginIds.map((programId) => parseHubProgramIdV1(programId)?.pluginId ?? programId), uiLocale, pluginInstallProgress)}
                  action={{ label: pluginInstallCancelTextV1(uiLocale), onPress: cancelPluginInstalls }}
                  actionProps={{ "data-semio-plugin-install-cancel": "" }}
                >
                  {pluginInstallProgress ? <progress className="ml-single align-middle" data-semio-plugin-install-progress="" value={pluginInstallProgress.completedBytes} max={pluginInstallProgress.totalBytes} aria-label={pluginInstallBandTextV1(installingPluginIds, uiLocale, pluginInstallProgress)} /> : null}
                </NoticeActionRow>
              )}
              {commandStalls.length === 0 ? null : (
                <SharedBorderRow orientation="vertical" role="status" aria-live="polite" data-semio-command-stall="">
                  {commandStalls.map((stall) => (
                    <NoticeActionRow
                      key={stall.id}
                      data-semio-command-stall-id={stall.id}
                      data-command-id={stall.commandId ?? ""}
                      text={commandStallLineV1(stall, loadedPlugins, panel?.spawnedApps ?? [], session ?? null, uiTerminology, uiLocale, commandStallClockMs)}
                      action={{ label: commandStallCancelTextV1(uiLocale), onPress: () => commandStallWatch.cancel(stall.id) }}
                      actionProps={{ "data-semio-command-stall-cancel": "" }}
                    />
                  ))}
                </SharedBorderRow>
              )}
            </NoticeStack>
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
                  <div data-semio-bottom-bands="" className="flex w-full flex-col items-stretch">
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
                      <SharedBorderRow role="status" aria-live="polite" data-semio-checkin-band={currentCheckIn.status.phase} className="text-sm">
                        <NoticeCell className="flex-row items-center"><span id="s-checkin-status">{checkinStatusText(currentCheckIn.status, uiLocale)}</span></NoticeCell>
                        {isTerminalDocumentCheckInPhaseV1(currentCheckIn.status.phase) ? (
                          <Button icon="x" type="button" variant="outline" id="s-checkin-dismiss" onClick={() => dismissHubCheckIn(currentDocumentRuntimeKey)}>
                            {shellLabel("ui.common.close")}
                          </Button>
                        ) : (
                          <>
                            {currentCheckIn.status.progress.totalUnits > 0 ? (
                              <SharedBorderRowCell>
                                <progress value={currentCheckIn.status.progress.completedUnits} max={currentCheckIn.status.progress.totalUnits} aria-label={checkinStatusText(currentCheckIn.status, uiLocale)} />
                              </SharedBorderRowCell>
                            ) : null}
                            <Button icon="x" type="button" variant="outline" id="s-checkin-abort" onClick={() => cancelHubCheckIn(currentDocumentRuntimeKey)}>
                              {checkinAbortText(uiLocale)}
                            </Button>
                          </>
                        )}
                      </SharedBorderRow>
                    )}
                  </div>
                ) : undefined
              }
              panels={Object.fromEntries(ANCHORS.map((anchor) => [anchor, buildPanelProps(anchor)])) as Record<Anchor, ReturnType<typeof buildPanelProps>>}
              canvasChips={canvasChips}
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
        {session ? (
          <ContextMenuTriggerButton
            data-glass-chip=""
            className="ui-glass fixed bottom-[var(--footer-band-height)] right-0 z-50"
            resolveTarget={(button) => {
              const selection = typeof window === "undefined" ? null : window.getSelection();
              return selection !== null && selection.toString() !== "" ? (selection.anchorNode?.parentElement ?? button) : button;
            }}
          />
        ) : null}
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
        <StagedActionDialog host={stagedActions} />
      </LevelProvider>
    </UIFindProvider>
    </ShellFaultBoundary>
    </ShellContextMenuFallbackContext.Provider>
    </PluginSurfaceActionsContext.Provider>
    </RowActionRefusalsContext.Provider>
    </UiPresenceOverlayContext.Provider>
    </UiKeybindingsProvider>
    </AppCatalogueContext.Provider>
    </UnsavedMarkerContext.Provider>
    </AppKeybindingsContext.Provider>
    </SetWindowIconContext.Provider>
    </SetWindowTitleContext.Provider>
    </BoardSessionFactoryContext.Provider>
  );
}
