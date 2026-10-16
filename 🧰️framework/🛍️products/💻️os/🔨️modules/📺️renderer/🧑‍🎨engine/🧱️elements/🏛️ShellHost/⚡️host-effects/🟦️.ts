/** ⚡️ `applyHostEffects` and the effects that keep the refresh lane and the shell route in step with host effects.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useApplyHostEffects` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { shellDialogOriginIsCurrentV1, shellDialogSessionIsCurrentV1 } from "../🗨️dialog-origin/🟦️.ts";
import { useCallback, useEffect } from "react";
import { renderIconRequest } from "../../🖼️IconRenderHost/🚚️request/🟦️.ts";
import { type Effect, resolveUiDirtyScope, type UiDirtyScope } from "@semio-tech/framework";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { programHistoryKeyV1, spawnedProgramViewStateV1 } from "../🪟️spawned-program/🟦️.ts";
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
import { retireLocalDocument } from "@semio-tech/framework/config/mutations";
import { sharedDocumentOpeningRoleV1 } from "../🧭️opening/🟦️.ts";
import { type ActiveSession, resolveBootExampleId } from "../../🐚️Shell/🟦️.tsx";
import {
  SET_ACTIVE_EXAMPLE_ACTION_ID,
  isShellOwnedCommandId,
  documentTransferNoticeCodeV1,
  downloadMediaExportBytes,
  typedOperationCompletionRefreshV1,
  hostEffectRefreshScopeV1,
  hostEffectsRewriteGuestRenderInputsV1,
  documentLoadCancelledV1,
  drainSegmentedMediaExport,
  downloadDataUrl,
  downloadMediaExport,
  mediaExportEncodingText,
  shellSegmentedDownloadSinkFactory,
  makeEffectDispatchOne,
  operationProgressPartsV1,
  type OperationProgressV1,
  importOpenedFilesV1,
  programHistoryProjectionsWithLoadV1,
  parsePanelState,
  requestFileOpen,
  clipboardWriteFragmentFromEffect,
  resolveManifestLabel,
  requestMediaFramesSourceV1,
  runMediaFramesV1,
  scheduleDispatchAction,
  SEGMENTED_DOWNLOAD_MARKER_PREFIX,
  viewStateWithSpacePanel
} from "../../🛠️ShellHelpers/🟦️.tsx";
// #endregion 🔌️Adapters
import { isPluginInstanceRetiredV1 } from "../../🔌️PluginRuntime/🟦️.tsx";
import { reduceArtifactCreationProgressUiV1 } from "../🌱️artifact-creation/🟦️.tsx";
import { replayRefusalCodeV1, replayRefusalNoticeTextV1, type ReplayRefusalReasonV1 } from "../📣️replay-refusal/🟦️.ts";
import { browserVideoRenderEnvironmentV1, runVideoRenderExportV1 } from "../../🎥️VideoRenderHost/🟦️.ts";
import { coerceWireBytes } from "../../🔌️PluginRuntime/🟦️.tsx";
import {
  captureSpaceArtifactCreationCatalogAuthorityV1,
  directoryCommandFromAction,
  mintDirectoryCommandRequestId,
  SPACE_INDEX_DOCUMENT_ID,
  type SpaceArtifactCreationOwnerV1,
  spaceArtifactCreationRequestFromAction,
  spaceIndexOpeningArgsV1
} from "../🧾️space-artifact-creation/🟦️.ts";
import { shellSpaceAdministrationOpening } from "../🏠️space-administration/🟦️.ts";
import { dispatchInvokeExtensionEffect } from "../🔌️extension-runtime/🟦️.ts";
import { logUnlessRetiredV1 } from "../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterSpawnedSession } from "../🛫️spawned-session/🟦️.ts";

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
    applyHostEffectsRef, refreshUi, refreshSpawnedUi, ensureSpawnedPlugin, requestServiceOperation
  } = core;
  const applyHostEffects = useCallback(
    async (effects: readonly Effect[], baseSession: ActiveSession, uiScope: UiDirtyScope | undefined, effectOwner: ReturnType<typeof captureEffectOwner>) => {
      const effectOrigin = effectOwner.presentation;
      // 🪦️ An effect pass addressed to an instance a switch has SEALED is stale by construction — the
      // close ladder has already revoked its actor, so running the pass reaches `requireActorId` and
      // surfaces as `no actor for instance N`. One typed drop, no throw, no stack.
      if (effects.length === 0 && sealedInstancesRef.current.sealed(baseSession.pluginId, baseSession.instanceId)) return;
      if (dropForSealedInstance(baseSession, "host effects", `${effects.length} effect(s)`)) return;
      let nextViewState = baseSession.viewState;
      for (const effect of effects) {
        if (!isCurrentEffectOwner(effectOwner)) return;
        if (effect === "requestSync") continue;
        if ("notify" in effect) {
          // 🧯️ A plugin's own user-facing message (`kernel::Effect::Notify`) — already localized by the
          // plugin against the host's declared locale/terminology axes, so it is shown verbatim.
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
          // 🧰️ A program programmatically switched utility: mirror it into the host-owned store slice AND
          // the ref `refreshUi` reads (bare `dispatch` alone leaves the map stale until the next render —
          // which is after this same pass's refresh, so brush/suggestion ghosts and gumballs never appear).
          // 🔢️ Shell-authoritative (`expectedGeneration: null`): the guest's own switch advances the
          // window's register so the next widget press renders against the generation it produced.
          const { windowId, utilityId } = effect.setActiveUtility;
          writeUtilityRegister(windowId, utilityId || null, null);
          if (utilityId && activeToolIdRef.current) writeToolRegister(null, null);
          if (windowId === activeWindowIdRef.current) nextViewState = { ...nextViewState, activeUtilityId: utilityId || undefined, activeToolId: utilityId ? undefined : nextViewState.activeToolId };
          continue;
        }
        if ("setActiveTool" in effect) {
          // 🛠️ A program programmatically switched tools (e.g. puzzle3d fill via engagement text command):
          // mirror it into the host-owned store slice, clear every window's active utility (mutual
          // exclusion — a tool and a window utility never both claim the pointer), and fold it into the
          // view state fed to the follow-up refresh.
          const { toolId } = effect.setActiveTool;
          writeToolRegister(toolId || null, null);
          if (toolId) clearAllWindowUtilities();
          nextViewState = { ...nextViewState, activeToolId: toolId || undefined, activeUtilityId: toolId ? undefined : nextViewState.activeUtilityId };
          continue;
        }
        if ("openDialog" in effect) {
          const { dialogId, args } = effect.openDialog;
          if (shellDialogSessionIsCurrentV1(baseSession, shellStateRef.current.pluginRuntime.session)) {
            const value = makeOwnedDialog(dialogId, effectOrigin, args as Record<string, unknown> | undefined);
            if (value !== null) {
              liveDialogRef.current = value;
              dispatch({ type: "SET_DIALOG", value });
            }
          }
          continue;
        }
        if ("navigate" in effect) {
          navigateShellUri(effect.navigate.uri);
          continue;
        }
        if ("loadDocument" in effect) {
          const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
          const payload = effect.loadDocument;
          if (payload.pack && payload.spr && pluginEntry?.handle.loadAppDocumentArchive) {
            const packBytes = coerceWireBytes(payload.pack);
            const sprBytes = coerceWireBytes(payload.spr);
            const file = resolveManifestLabel(baseSession.app.label, uiTerminologyRef.current, uiLocaleRef.current) || baseSession.app.id;
            const task = core.documentTransferRef.current.trackDocumentTransfer(file, baseSession.pluginId);
            const showLoad = (load: { readonly completed: number; readonly total: number } | null) => historyStore.update((projections) => programHistoryProjectionsWithLoadV1(projections, programHistoryKeyV1(baseSession), load));
            try {
              if (await loadDocumentPair(pluginEntry.handle, baseSession.instanceId, packBytes, sprBytes, () => isCurrentEffectOwner(effectOwner), { signal: task.signal, progress: (status) => { task.progress(status.completed, status.total > 0 ? status.total : null); showLoad(status); } })) await refreshHistorySnapshot(baseSession);
            } catch (error) {
              const cancelled = documentLoadCancelledV1(error, task.signal);
              console.warn(`[os-shell] ${documentTransferNoticeCodeV1(cancelled ? "load-cancelled" : "load-failed")} ${baseSession.pluginId}#${baseSession.instanceId}`, error);
              core.documentTransferRef.current.notifyDocumentTransfer(cancelled ? "load-cancelled" : "load-failed", file, cancelled ? "info" : "error");
            } finally {
              showLoad(null);
              task.finish();
            }
          } else {
            // 🚧️ `Effect::LoadDocument` is pack+spr bytes only (no JSON-text fallback exists on the wire) and loads through the
            // stepped archive load — a program without `loadAppDocumentArchive` simply cannot receive this effect.
            console.error("[os-shell] loadDocument: program has no archive loader", baseSession.pluginId, Object.keys(payload));
          }
          continue;
        }
        if ("openExternalUrl" in effect) {
          window.open(effect.openExternalUrl.url, "_blank", "noopener,noreferrer");
          continue;
        }
        if ("downloadMediaExport" in effect) {
          const { filename, mimeType, data, encoding } = effect.downloadMediaExport;
          const encodingText = mediaExportEncodingText(encoding);
          if (encodingText?.startsWith(SEGMENTED_DOWNLOAD_MARKER_PREFIX)) {
            const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
            if (!pluginEntry) throw new Error(`segmented-download-plugin-missing:${baseSession.pluginId}`);
            // 🌊 The ASSEMBLED sink, not the File System Access stream: a segmented download must reach the
            // user as the same kind of file the inline lane produces, and `createSegmentedDownloadSink`
            // fails closed wherever `showSaveFilePicker` is absent — which turned every over-budget export
            // into silence. Bounded by the drain's own `SEGMENTED_DOWNLOAD_CONTRACT.maximumTotalBytes` cap.
            await drainSegmentedMediaExport(filename, mimeType, data, encodingText, (operationId) => pluginEntry.handle.takeSegmentedDownloadChunk(baseSession.instanceId, operationId), {
              signal: segmentedDownloadAbortRef.current.signal,
              sinkFactory: shellSegmentedDownloadSinkFactory,
            });
          } else {
            downloadMediaExport(filename, mimeType, data, encodingText);
          }
          continue;
        }
        if ("videoRenderExport" in effect) {
          const { filename, program } = effect.videoRenderExport;
          const capabilities = catalog.plugins.find((target) => target.pluginId === baseSession.pluginId)?.capabilities ?? [];
          void runVideoRenderExportV1(
            { filename, owner: baseSession.pluginId, program, capabilities },
            {
              environment: browserVideoRenderEnvironmentV1(),
              deliver: (name, bytes) => downloadMediaExportBytes(name, "video/mp4", bytes),
              locale: () => uiLocaleRef.current,
              announce: (text) => showTransientNoticeRef.current(text, "info", "video-render-export.started"),
            },
          ).then(({ outcome, text }) => showTransientNoticeRef.current(text, outcome.status === "done" || outcome.status === "cancelled" ? "info" : "warning", `video-render-export.${outcome.status}`));
          continue;
        }
        if ("iconRenderExport" in effect) {
          for (const item of effect.iconRenderExport.items) {
            try {
              const result = await renderIconRequest(item.request as Parameters<typeof renderIconRequest>[0]);
              downloadDataUrl(item.filename, result.dataUrl);
            } catch (error) {
              console.error(`icon render export failed for ${item.filename}`, error);
            }
          }
          continue;
        }
        if ("requestFileOpen" in effect) {
          const { accept, readAs, importAction, multiple, args } = effect.requestFileOpen;
          const resolvedImport = importAction || "importSnapshot";
          const opened = await requestFileOpen(accept || ".spk,.dsl,.ops,application/octet-stream", readAs, multiple);
          if (opened.length > 0) {
            const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
            if (pluginEntry) {
              const task = core.documentTransferRef.current.trackDocumentTransfer(opened.map((file) => file.name).join(", "), baseSession.pluginId);
              try {
                const dispatchOne = makeEffectDispatchOne(pluginEntry, baseSession, (effects, target, scope) => applyHostEffects(effects, target, scope, effectOwner), () => isCurrentEffectOwner(effectOwner), resolvedTargetViewState, { causedBy: effectOwner.inputSeq, windowId: baseSession.viewState.windowId ?? null });
                if ((await importOpenedFilesV1(baseSession.app, opened, resolvedImport, Boolean(multiple), dispatchOne, task.signal, task.progress, args as Readonly<Record<string, unknown>> | undefined)) === "cancelled") core.documentTransferRef.current.notifyDocumentTransfer("import-cancelled", opened[0]!.name, "info");
              } finally {
                task.finish();
              }
            }
          }
          continue;
        }
        if ("dispatchAction" in effect) {
          // 🔁️ Self re-dispatch (D2): re-invokes the same plugin instance with `action` after `delayMs`,
          // without blocking the current `applyHostEffects` pass — the host's one continuation scheduler
          // fires the follow-up call and feeds its own `requestedEffects` back through `applyHostEffects`
          // recursively, so a plugin can chain several ticks of staged/progressive work (e.g. a
          // multi-pass reconstruction) purely by re-emitting `dispatchAction` from its own handler.
          // 🪃️ `delayMs: 0` — what every `rearm()` in flow/generation2d/generation3d asks for — is an
          // unthrottled macrotask, NOT a nested `setTimeout`: this branch re-enters from inside the
          // previous dispatch's own callback, and a hidden/unfocused/headless renderer clamps exactly
          // that shape to ~1 tick/s (this ticket's measured ~24 s per extension hop).
          const { action: dispatchActionId, args: dispatchArgs, delayMs } = effect.dispatchAction;
          // 🪦️ `loadedPluginsRef`, never the render closure's `loadedPlugins`: a DEFERRED effect pass
          // (`publishContributions`'s `dispatchDeferredEffects`) runs against the `applyHostEffects`
          // captured when the publish STARTED — one render before the program it just installed reached
          // `loadedPlugins` state. Read from the closure, both of the install's own `flowEvalTick`
          // re-arms found an EMPTY list and were dropped without a word, so the whole evaluation chain
          // never started (measured on 6018, ticket 26/09/09/PROCEDURAL-3D-END-TO-END). A miss is now
          // loud: a re-arm the host silently swallows is indistinguishable from a guest that stopped.
          const pluginEntry = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === baseSession.pluginId);
          if (!pluginEntry) {
            console.error(`[os-shell] dispatchAction "${dispatchActionId}" dropped: no loaded program for "${baseSession.pluginId}" (loaded: ${loadedPluginsRef.current.map((entry) => entry.handle.pluginId).join(", ") || "none"})`);
            continue;
          }
          scheduleDispatchAction(dispatchActionId, dispatchArgs as Record<string, unknown> | undefined, delayMs, makeEffectDispatchOne(pluginEntry, baseSession, (effects, target, scope) => applyHostEffects(effects, target, scope, effectOwner), () => isCurrentEffectOwner(effectOwner), resolvedTargetViewState, { causedBy: effectOwner.inputSeq, windowId: baseSession.viewState.windowId ?? null }));
          continue;
        }
        if ("requestServiceOperation" in effect) {
          await requestServiceOperation(baseSession, effect.requestServiceOperation, () => isCurrentEffectOwner(effectOwner));
          continue;
        }
        if ("replayShellCommand" in effect) {
          const { actionId, args } = effect.replayShellCommand;
          const argsRecord = args as Record<string, unknown> | undefined;
          const refuseReplay = (reason: ReplayRefusalReasonV1, line: string, ...rest: readonly unknown[]): void => {
            console.warn(`[os-shell] replayShellCommand: ${line}`, ...rest);
            showTransientNoticeRef.current(replayRefusalNoticeTextV1(reason, uiLocaleRef.current), "warning", replayRefusalCodeV1(reason));
          };
          if (actionId === "os.directory.open-administration") {
            const opening = shellSpaceAdministrationOpening(actionId, argsRecord, spaceAdministrationEpochRef.current + 1);
            if (opening === null) {
              refuseReplay("space-required", "administration requires an exact space id");
            } else if (!identityRef.current) {
              refuseReplay("sign-in-required", "administration dropped, no signed-in identity");
            } else {
              const worker = ensureBackboneWorker();
              spaceAdministrationEpochRef.current = opening.state.operationEpoch;
              spaceAdministrationRef.current = opening.state;
              setSpaceAdministration(opening.state);
              worker.postMessage({ wire: encodeBackboneWorkerRequest(opening.request) });
            }
          } else if (actionId === "os.create-space-artifact") {
            const origins = [...openDocumentSessionsRef.current.entries()].filter(
              ([, entry]) => entry.session.pluginId === baseSession.pluginId
                && entry.session.instanceId === baseSession.instanceId
                && entry.scope?.documentId === SPACE_INDEX_DOCUMENT_ID,
            );
            const origin = origins.length === 1 ? origins[0] : undefined;
            if (origin === undefined || origin[1].scope === undefined || effectOrigin?.document?.runtimeKey !== origin[0]
              || effectOrigin.document.clientInstanceId !== origin[1].clientInstanceId
              || !shellDialogSessionIsCurrentV1(baseSession, shellStateRef.current.pluginRuntime.session)) {
              refuseReplay("space-index-required", "space artifact creation requires one mounted Space index");
            } else if (!identityRef.current) {
              refuseReplay("sign-in-required", "space artifact creation dropped, no signed-in identity");
            } else {
              const requestId = mintDirectoryCommandRequestId();
              const currentCatalog = captureSpaceArtifactCreationCatalogAuthorityV1(
                spaceArtifactCreationCatalogRef.current,
                spaceArtifactCreationCatalogUiRef.current,
                effectOrigin,
              );
              const request = spaceArtifactCreationRequestFromAction(actionId, argsRecord, origin[1].scope.spaceId, requestId, effectOwner.creationCatalog, currentCatalog);
              if (request === null) {
                refuseReplay("invalid-request", "invalid space artifact creation request");
              } else {
                const owner: SpaceArtifactCreationOwnerV1 = {
                  requestId,
                  spaceId: request.spaceId,
                  expectedCatalogGenerationId: request.expectedCatalogGenerationId,
                  kindId: request.kindId,
                  name: request.name,
                  runtimeKey: origin[0],
                  clientInstanceId: origin[1].clientInstanceId,
                  sessionInstanceId: origin[1].session.instanceId,
                  opening: false,
                  cancelRequested: false,
                  ready: null,
                };
                spaceArtifactCreationOwnersRef.current.set(requestId, owner);
                setSpaceArtifactCreationUi((current) => reduceArtifactCreationProgressUiV1(current, { kind: "issued", owner, atMs: Date.now() }));
                const worker = ensureBackboneWorker();
                worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
              }
            }
          } else if (actionId.startsWith("os.directory.")) {
            const indexScope = [...openDocumentSessionsRef.current.values()].find((entry) => entry.session.pluginId === baseSession.pluginId && entry.session.instanceId === baseSession.instanceId)?.scope;
            const command = directoryCommandFromAction(actionId, spaceIndexOpeningArgsV1(argsRecord, indexScope, SPACE_INDEX_DOCUMENT_ID));
            if (!command) {
              refuseReplay("invalid-request", "unrecognized directory action", actionId);
            } else if (!identityRef.current) {
              refuseReplay("sign-in-required", "directory command dropped, no signed-in identity", actionId);
            } else {
              const worker = ensureBackboneWorker();
              const requestId = mintDirectoryCommandRequestId();
              worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "directory-command", requestId, command }) });
            }
          } else if (actionId === "os.open-artifact" || actionId === "os.open-artifact-with") {
            try {
              const indexScope = [...openDocumentSessionsRef.current.values()].find((entry) => entry.session.pluginId === baseSession.pluginId && entry.session.instanceId === baseSession.instanceId)?.scope;
              const openingArgs = spaceIndexOpeningArgsV1(argsRecord, indexScope, SPACE_INDEX_DOCUMENT_ID);
              const sharedSpaceId = typeof openingArgs?.spaceId === "string" ? openingArgs.spaceId : null;
              const sharedHistory = sharedSpaceId === null ? undefined : [...spaceDirectoryHistoriesRef.current.values()].find((history) => history.spaceId === sharedSpaceId);
              const accessRole = sharedHistory === undefined || !identityRef.current ? null : sharedDocumentOpeningRoleV1(sharedHistory.events(), sharedHistory.spaceId, identityRef.current.userId);
              if (sharedSpaceId !== null) adoptSharedSpaceAccess(sharedSpaceId, accessRole);
              if (accessRole === "viewer" && actionId === "os.open-artifact-with" && (openingArgs?.role === "editor" || openingArgs?.role === 1)) {
                refuseReplay("view-only-access", "a read-only member asked to edit", args);
                continue;
              }
              const opening = await resolveArtifactOpeningWithActivationRef.current(actionId, accessRole === "viewer" && actionId === "os.open-artifact" ? { ...openingArgs, role: "viewer" } : openingArgs);
              if (!opening) {
                refuseReplay("router-not-ready", "artifact router is not ready", args);
                continue;
              }
              const target = await openArtifactWithAppRefRef.current(opening.app, opening.dialect, opening.role, () => isCurrentEffectOwner(effectOwner));
              if (target && opening.documentId && opening.schema) {
                await openDocumentRef.current({ documentId: opening.documentId, schema: opening.schema, ...(opening.spaceId ? { spaceId: opening.spaceId } : {}) }, undefined, target);
              }
            } catch (openingError) {
              refuseReplay("open-rejected", "artifact opening rejected", openingError, args);
            }
          } else if (actionId === LOCAL_CATALOG_ACTIONS_V1.admit) {
            const admission = localCatalogAdmissionV1(argsRecord, localDataDir, Date.now());
            if ("refusal" in admission) {
              showTransientNoticeRef.current(localCatalogNoticeTextV1(admission.refusal, uiLocaleRef.current), "warning", localCatalogNoticeCodeV1(admission.refusal));
            } else {
              const name = admission.document.name || admission.document.documentId;
              showTransientNoticeRef.current(localCatalogNoticeTextV1("keeping", uiLocaleRef.current, name), "info", localCatalogNoticeCodeV1("keeping"));
              const written = isCurrentEffectOwner(effectOwner) ? await writeLocalDocumentRef.current(admission) : null;
              if (written === null) {
                showTransientNoticeRef.current(localCatalogNoticeTextV1("cancelled", uiLocaleRef.current, name), "warning", localCatalogNoticeCodeV1("cancelled"));
              } else if (!written) {
                showTransientNoticeRef.current(localCatalogNoticeTextV1("write-failed", uiLocaleRef.current, name), "warning", localCatalogNoticeCodeV1("write-failed"));
              } else if (recordLocalCatalogRef.current(localCatalogAdmitMutationV1(admission)) === null) {
                showTransientNoticeRef.current(localCatalogNoticeTextV1("no-data-folder", uiLocaleRef.current, name), "warning", localCatalogNoticeCodeV1("no-data-folder"));
              } else {
                showTransientNoticeRef.current(localCatalogNoticeTextV1("kept", uiLocaleRef.current, name), "info", localCatalogNoticeCodeV1("kept"));
                const pluginEntry = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === baseSession.pluginId);
                const rehydration = localCatalogRehydrationArgumentsV1(admission);
                if (pluginEntry && rehydration !== null) await makeEffectDispatchOne(pluginEntry, baseSession, (effects, target, scope) => applyHostEffects(effects, target, scope, effectOwner), () => isCurrentEffectOwner(effectOwner), resolvedTargetViewState, { causedBy: effectOwner.inputSeq, windowId: baseSession.viewState.windowId ?? null })(LOCAL_CATALOG_ACTIONS_V1.rehydrate, rehydration);
                const seen = rehydratedLocalDocumentsRef.current.get(baseSession.instanceId);
                if (seen !== undefined) seen.add(`${admission.document.documentId}@${admission.document.admittedAtMs}`);
              }
            }
          } else if (actionId === LOCAL_CATALOG_ACTIONS_V1.retire) {
            const documentId = localCatalogDocumentIdV1(argsRecord);
            const kept = documentId === null ? undefined : localCatalogRef.current.documents.find((document) => document.documentId === documentId);
            if (kept === undefined) {
              showTransientNoticeRef.current(localCatalogNoticeTextV1("document-unknown", uiLocaleRef.current), "warning", localCatalogNoticeCodeV1("document-unknown"));
            } else if (recordLocalCatalogRef.current(retireLocalDocument(kept.documentId)) !== null) {
              showTransientNoticeRef.current(localCatalogNoticeTextV1("retired", uiLocaleRef.current, kept.name || kept.documentId), "info", localCatalogNoticeCodeV1("retired"));
            }
          } else if (isShellOwnedCommandId(actionId)) {
            // 🐚️ Chrome replays against shell-owned state. Routing one into the guest is what the
            // window-kind gate refused as `undeclared-action` after every undo (ticket 26/09/18 §3.2).
            if (!replayShellOwnedCommandRef.current(actionId, argsRecord)) {
              refuseReplay("unrouted-command", "no shell route for chrome command", actionId);
            }
          } else {
            if (actionId === SET_ACTIVE_EXAMPLE_ACTION_ID) {
              const raw = typeof argsRecord?.exampleId === "string" ? argsRecord.exampleId : "";
              const exampleId = raw || resolveBootExampleId("", exampleOptionsRef.current, defaults.exampleId);
              if (raw) lastDispatchedExampleIdRef.current = raw;
              dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: exampleId });
            }
            // 🔗️ A guest follow-up: `causedBy` is the input whose answer armed it, so the ledger sorts it
            // BEFORE any later input (L2) and a lost CAS race is logged, never toasted (§G). `inputSeq`
            // is re-minted by `issue`.
            core.onActionRef.current({
              controllerId: baseSession.app.controllerId,
              action: actionId,
              args: argsRecord,
              provenance: { origin: "guest", causedBy: effectOwner.inputSeq, windowId: typeof argsRecord?.windowId === "string" ? argsRecord.windowId : baseSession.viewState.windowId ?? null },
            });
          }
          continue;
        }
        if ("requestMediaFrames" in effect) {
          // 🎞️ D5: decodes a video (file picker, or `payload` bytes already in hand from a drop zone) as one Tasks-window
          // transfer with frame progress and Cancel, fanning sampled frames + a completion marker out through the same
          // `dispatchOne` path as every other effect branch — see `runMediaFramesV1` for the Tier 1 (WebCodecs)/Tier 2
          // (`<video>` seek-and-capture)/fallback decision tree and the `importAbort` a cancel after a frame sends.
          const { accept, payload, frameAction, doneAction, fallbackAction, sampleStride, maxFrames, maxLongEdgePx, fpsHint, args } = effect.requestMediaFrames;
          const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
          const source = pluginEntry ? await requestMediaFramesSourceV1(accept, payload) : null;
          if (pluginEntry && source) {
            const task = core.documentTransferRef.current.trackDocumentTransfer(source.name, baseSession.pluginId);
            try {
              const dispatchOne = makeEffectDispatchOne(pluginEntry, baseSession, (effects, target, scope) => applyHostEffects(effects, target, scope, effectOwner), () => isCurrentEffectOwner(effectOwner), resolvedTargetViewState, { causedBy: effectOwner.inputSeq, windowId: baseSession.viewState.windowId ?? null });
              const outcome = await runMediaFramesV1({ frameAction, doneAction, fallbackAction, sampleStride: sampleStride ?? 0, maxFrames: maxFrames ?? 0, maxLongEdgePx: maxLongEdgePx ?? 0, fpsHint: fpsHint ?? 0, args: args as Record<string, unknown> | undefined }, source, dispatchOne, baseSession.app, task.signal, task.progress);
              if (outcome === "cancelled") core.documentTransferRef.current.notifyDocumentTransfer("import-cancelled", source.name, "info");
            } finally {
              task.finish();
            }
          }
          continue;
        }
        if ("invokeExtension" in effect) {
          const pluginsNow = loadedPluginsRef.current;
          const requesterId = pluginsNow.some((entry) => entry.handle.pluginId === baseSession.pluginId) ? baseSession.pluginId : sessionRef.current?.pluginId;
          const requester = requesterId && pluginsNow.some((entry) => entry.handle.pluginId === requesterId) ? { pluginId: requesterId, instanceId: baseSession.instanceId } : baseSession;
          // ⏳️ An extension round trip holds the REQUESTER's captured activation from dispatch until its
          // `Completed` frame lands, so a switch that retires the requester in between is exactly the
          // `invokeExtension dispatch failed … no actor for instance N` measured on 6018. Registering
          // it here is what lets `switchToPluginApp`'s quiesce pass wait for it.
          const releaseExtensionWork = sessionWorkRef.current.begin(requester.pluginId, requester.instanceId, "extension-invocation");
          void dispatchInvokeExtensionEffect(pluginsNow, requester, effect.invokeExtension, async (requestingPlugin, response) => {
            if (dropForSealedInstance(requester, "extension answer", effect.invokeExtension.extensionId)) throw new Error("extension.requester-retired");
            if (!isCurrentEffectOwner(effectOwner)) throw new Error("extension.requester-retired");
            const current = sessionRef.current;
            const handle = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === requester.pluginId)?.handle;
            if (!current || handle !== requestingPlugin.handle) throw new Error("extension.requester-retired");
            const primary = current.pluginId === baseSession.pluginId && current.instanceId === baseSession.instanceId;
            const spawned = parsePanelState(current.viewState)?.spawnedApps.some((entry) => entry.pluginId === baseSession.pluginId && entry.instanceId === baseSession.instanceId);
            if (!primary && !spawned) throw new Error("extension.requester-retired");
            applyHistoryPatch(response.historyPatch, false, { pluginId: baseSession.pluginId, instanceId: baseSession.instanceId });
            applyLeftoverInteractionView(response.output);
            await applyHostEffects(response.requestedEffects ?? [], primary ? current : { ...baseSession, viewState: current.viewState }, resolveUiDirtyScope(response.uiScope), effectOwner);
          }).catch((error) => {
            const { extensionId, capability, req } = effect.invokeExtension;
            if (sealedInstancesRef.current.sealed(requester.pluginId, requester.instanceId)) return;
            // 🪦️ A hot swap RETIRES the requester rather than sealing it, and an extension round trip
            // outlives the swap by construction — the `evaluate` calls the guest had in flight land
            // after their instance is gone. The typed retirement is the drop, exactly as it is for the
            // typed-operation subscriptions and the local-interaction read above; the two
            // `no actor for instance 1` errors every measured hot swap left in the console were this
            // one path missing it (`📓️hot-swap-board-remount-2026-09-15.md` §1.4).
            if (isPluginInstanceRetiredV1(error)) {
              return;
            }
          }).finally(releaseExtensionWork);
          continue;
        }
        if ("spawnPluginInstance" in effect) {
          const { pluginId, appId, osInstanceId, label, artifactJson } = effect.spawnPluginInstance;
          const program = spacePrograms.find((entry) => entry.pluginId === pluginId && entry.appId === appId) ?? spacePrograms.find((entry) => entry.pluginId === pluginId);
          if (program) {
            // 🪟️ Fold spawn into `nextViewState` — a separate SET_SESSION would be clobbered by the
            // final write below and leave the shell stuck on the studio surface.
            const nextPanel = await ensureSpawnedPlugin(program, () => isCurrentEffectOwner(effectOwner), label, osInstanceId, artifactJson, nextViewState);
            if (!isCurrentEffectOwner(effectOwner)) return;
            if (nextPanel) nextViewState = viewStateWithSpacePanel(nextViewState, nextPanel);
          }
          continue;
        }
        if ("openPluginInstance" in effect) {
          const { pluginId, appId, osInstanceId } = effect.openPluginInstance;
          const program = spacePrograms.find((entry) => entry.pluginId === pluginId && entry.appId === appId) ?? spacePrograms.find((entry) => entry.pluginId === pluginId);
          if (program) {
            // 🪟️ Fold focus into `nextViewState` so the final SET_SESSION keeps `activeSpawnedId`
            // (opening a workflow node depends on this — otherwise nothing appears to happen).
            const nextPanel = await ensureSpawnedPlugin(program, () => isCurrentEffectOwner(effectOwner), undefined, osInstanceId, undefined, nextViewState);
            if (!isCurrentEffectOwner(effectOwner)) return;
            if (nextPanel) {
              nextViewState = viewStateWithSpacePanel(nextViewState, nextPanel);
            }
            if (osInstanceId && openSpaceIdRef.current) {
              openInstanceIdRef.current = osInstanceId;
              navigateShellUri(`/spaces/${openSpaceIdRef.current}/instances/${osInstanceId}`);
            }
          } else {
            console.warn(
              "[os-shell] openPluginInstance: no program matches",
              { pluginId, appId },
              "available:",
              spacePrograms.map((entry) => `${entry.pluginId}/${entry.appId}`),
            );
          }
          continue;
        }
      }
      const nextSession = { ...baseSession, viewState: nextViewState };
      if (!isCurrentEffectOwner(effectOwner)) return;
      // 🐢️ Did any EFFECT above actually rewrite the view state? Reference-comparing `nextViewState`
      // against the LIVE session's own `viewState` cannot answer that: every dispatch enters here with a
      // freshly built per-call projection (`onAction`'s `dispatchViewState` — locale/terminology/
      // `windowInstances`/`activeUtilityByWindowId` re-derived from refs, plus `windowId` pinned to the
      // dispatch's target window), so the identity check below was false on EVERY action and minted a new
      // `session` object each time. That churn re-ran the history-snapshot effect (one `readHistory` guest
      // round trip per dispatch) and re-subscribed the operation-completion effect — measured 2026-09-09
      // as 7 `readHistory` calls for the 7 boot mesh pages and 90 during a 35 s fill run. Comparing
      // against `baseSession.viewState` asks the question the comment below always meant to ask, and a
      // per-call projection is never written back into the session (`refreshUi` re-derives it anyway).
      const viewStateRewrittenByEffects = nextViewState !== baseSession.viewState;
      const isSpawnedPluginSession = hostMode && !shellDialogSessionIsCurrentV1(baseSession, shellStateRef.current.pluginRuntime.session);
      dispatch({
        type: "SET_SESSION",
        value: (current) => {
          if (!shellDialogOriginIsCurrentV1(effectOrigin, captureDialogOrigin(current))) return current;
          if (!current) return current;
          if (isSpawnedPluginSession) return viewStateRewrittenByEffects ? { ...current, viewState: nextViewState } : current;
          if (!shellDialogSessionIsCurrentV1(current, nextSession)) return current;
          // 🐢️ Preserve `current`'s identity when the viewState didn't actually change — otherwise every
          // action mints a new `session` object, which cascades into a new `onAction` identity, which
          // busts every memo keyed on it (windows, panels, the boot-refresh effect below) even when
          // nothing about the session changed.
          return viewStateRewrittenByEffects ? { ...current, viewState: nextViewState } : current;
        },
      });
      // 🧰️ What the GUEST dirtied (`uiScope`) unioned with what applying these effects dirtied
      // ({@link hostEffectRefreshScopeV1}) — an armed utility is a host-owned render input, and a
      // completion that re-took nothing declares `none`, so without this union the arm is applied and
      // never published (wave B37).
      const refreshScope = hostEffectRefreshScopeV1(effects, uiScope ?? { kind: "full" }, nextSession.app.windowKinds.map((kind) => kind.bodyKey));
      if (isSpawnedPluginSession) {
        const spawned = parsePanelState(nextViewState)?.spawnedApps.find((entry) => entry.pluginId === baseSession.pluginId && entry.instanceId === baseSession.instanceId);
        if (spawned) await refreshSpawnedUi(spawned, nextViewState, refreshScope);
      } else if (shellDialogSessionIsCurrentV1(shellStateRef.current.pluginRuntime.session, nextSession)) {
        await refreshUi(nextSession, refreshScope, undefined, leftoverReplaceRefreshBodiesV1(), hostEffectsRewriteGuestRenderInputsV1(effects) || nextViewState !== baseSession.viewState);
      }
    },
    [catalog,captureDialogOrigin, captureEffectOwner, dropForSealedInstance, isCurrentEffectOwner, loadDocumentPair, makeOwnedDialog, refreshHistorySnapshot, clearAllWindowUtilities, ensureSpawnedPlugin, loadedPlugins, navigateShellUri, refreshSpawnedUi, refreshUi, requestServiceOperation, resolvedTargetViewState, session, writeUtilityRegister, writeToolRegister, spacePrograms, hostMode],
  );
  core.applyHostEffects = applyHostEffects;
  // 🔁️ What the ui-refresh lane applies for a pass that asked for effects of its own, outside that pass.
  applyHostEffectsRef.current = applyHostEffects;

  /** 🏁️ Applies one retained typed operation's terminal publication. The command that started the
   * operation resolved on its FIRST reactor turn — its `InvocationResult` carries no outcome at all —
   * so a selection like "open the Nakagin example" used to leave the outliner, the inspection panel
   * and the History list showing the previous document forever. `PluginWasmHandle
   * .subscribeOperationCompletions` is the only carrier of that outcome: the operation's own history
   * delta, its final `UiDirtyScope`, and every host effect its continuation turns requested.
   *
   * 🫥️ What this pass is owed is decided by {@link typedOperationCompletionRefreshV1}, never by the
   * admitting reply: an admission reports `mutationCount: 0` for every typed operation, and a completion
   * that dirtied nothing, patched no history and requested no effect owes no pass at all — a retained
   * operation completes on every drain poll, so paying for those is a refresh storm (wave B27 §2). */
  useEffect(() => {
    if (!session) return;
    const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
    if (!plugin) return;
    const target = session;
    try {
      return plugin.subscribeOperationCompletions(target.instanceId, (completion) => {
        // 🏁️ Release every `onAction` promise waiting on this operation BEFORE the (async) effect pass —
        // the caller's contract is "the guest work is finished", and the host-effect pass that follows is
        // this same subscription's own work, not the guest's.
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

  /**
   * 🏁️🪟️ The SAME terminal-publication subscription, for every spawned program. The effect above
   * subscribes `session.pluginId`/`session.instanceId` and nothing else, so a spawned foreign editor's
   * typed operations completed into the void: no history delta, no final `UiDirtyScope`, none of the
   * continuation turns' effects — and `settleOperation` was never called, so every `onAction` on a
   * spawned program waited out the full {@link OPERATION_SETTLE_WATCHDOG_MS} watchdog.
   *
   * 🧯️ Measured live inside `s`: `addLayer` on a spawned `draw` DID add the layer, and the window kept
   * reading `1 layer · 0 selected` until the next unrelated interaction re-fetched it
   * (`🗑️generated/s5-action-{8,11}.txt`, ticket 26/09/18 S5 §4.3). `applyHostEffects` on a spawned base
   * session already routes to `refreshSpawnedUi`, so the completion pass needs no spawned-specific
   * branch of its own — only an owner.
   */
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
    // 🐢️ Keyed on the spawned ROSTER, not on `panel` identity: a panel object churns on every action.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [hostMode, session, spawnedRosterKey, applyHistoryPatch, applyHostEffects, captureProgramEffectOwner, settleOperation]);

  /** 🎞️ Renders a running operation's process while it runs: every mid-operation refresh a tool run asks for (its trace,
   * provisional pieces, progress) goes through the ui-refresh lane, whose coalescer merges what arrives while a pass is in
   * flight — the operation never waits on the renderer, and the renderer shows as much of the process as it keeps up with. */
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
      // 💼️ An Isolated spawned job (a mounted analysis, a fill plan) publishes no typed-operation
      // scope; its host driver reports coalesced progress instead, and every window adopting the
      // job's retained output re-renders through the same lane.
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

  /**
   * 🎞️🪟️ The SAME mid-operation progress lane, for every spawned program. The effect above
   * subscribes `session.instanceId` alone, so a spawned editor's running operation — a tool run's
   * trace, a mounted analysis's provisional pieces, a fill plan's progress — reached no window at
   * all: the human watched a frozen canvas until the terminal completion landed. Measured as
   * still-session-only by S6 (§2, `🏛️ShellHost` `subscribeOperationProgress`/
   * `subscribeSpawnedJobProgress`), which is the last of the six session-only decision sites S5
   * started on.
   */
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
    // 🐢️ Keyed on the spawned ROSTER, not on `panel` identity: a panel object churns on every action.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [hostMode, session, spawnedRosterKey, applyHistoryPatch, applyHostEffects, captureProgramEffectOwner]);

  return {};
}
//#endregion FrameworkOsShell
