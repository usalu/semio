/** 🎯️ The `replayShellCommand` host effect: directory administration, space artifact creation and shell command replay.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | useApplyHostEffects}: the effect arm of the `applyHostEffects` pass, moved verbatim. */
import { shellDialogSessionIsCurrentV1 } from "../../🗨️dialog-origin/🟦️.ts";
import { dialectCoordinate, type LocalizedLabel, type Effect, type UiDirtyScope } from "@semio-tech/framework";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { LOCAL_CATALOG_ACTIONS_V1, localCatalogAdmissionV1, localCatalogAdmitMutationV1, localCatalogDocumentIdV1, localCatalogNoticeCodeV1, localCatalogNoticeTextV1, localCatalogRehydrationArgumentsV1 } from "../../🗂️local-catalog/🟦️.ts";
import { retireLocalDocument } from "@semio-tech/framework/config/mutations";
import { sharedDocumentOpeningRoleV1 } from "../../🧭️opening/🟦️.ts";
import { resolveManifestLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { isShellOwnedCommandId } from "../../../🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
import { makeEffectDispatchOne } from "../../../🛠️ShellHelpers/🔩️plumbing/🚀️effect-dispatch/🟦️.ts";
import { reduceArtifactCreationProgressUiV1 } from "../../🌱️artifact-creation/🟦️.tsx";
import { replayRefusalCodeV1, replayRefusalNoticeTextV1, type ReplayRefusalReasonV1 } from "../../📣️replay-refusal/🟦️.ts";
import { captureSpaceArtifactCreationCatalogAuthorityV1, directoryCommandFromAction, mintDirectoryCommandRequestId, SPACE_INDEX_DOCUMENT_ID, type SpaceArtifactCreationOwnerV1, spaceArtifactCreationRequestFromAction, spaceIndexOpeningArgsV1 } from "../../🧾️space-artifact-creation/🟦️.ts";
import { readWorkflowArtifactCreationCatalogV1, workflowArtifactCreationRequestV1, workflowArtifactDropV1 } from "../../🧾️space-artifact-creation/🔁️workflow/🟦️.ts";
import { replayOpenArtifactV1 } from "./🚪️open-artifact/🟦️.ts";
import { shellSpaceAdministrationOpening } from "../../🏠️space-administration/🟦️.ts";
import type { ShellCoreAfterSpawnedSession } from "../../🛫️spawned-session/🟦️.ts";

/** 🎭️ Replays a shell-owned command against the directory, the space index and the artifact creation lane. */
export async function applyReplayShellCommandEffect({ showTransientNoticeRef, uiLocaleRef, spaceAdministrationEpochRef, identityRef, ensureBackboneWorker, spaceAdministrationRef, setSpaceAdministration, openDocumentSessionsRef, shellStateRef, spaceArtifactCreationCatalogRef, spaceArtifactCreationCatalogUiRef, spaceArtifactCreationOwnersRef, setSpaceArtifactCreationUi, spaceDirectoryHistoriesRef, hostPorts, adoptSharedSpaceAccess, resolveArtifactOpeningWithActivationRef, openResolvedArtifactRef, isCurrentEffectOwner, openDocumentRef, localDataDir, writeLocalDocumentRef, recordLocalCatalogRef, loadedPluginsRef, resolvedTargetViewState, rehydratedLocalDocumentsRef, localCatalogRef, replayShellOwnedCommandRef, effect, baseSession, effectOrigin, effectOwner, applyHostEffects, core }: Pick<ShellCoreAfterSpawnedSession, "showTransientNoticeRef" | "uiLocaleRef" | "spaceAdministrationEpochRef" | "identityRef" | "ensureBackboneWorker" | "spaceAdministrationRef" | "setSpaceAdministration" | "openDocumentSessionsRef" | "shellStateRef" | "spaceArtifactCreationCatalogRef" | "spaceArtifactCreationCatalogUiRef" | "spaceArtifactCreationOwnersRef" | "setSpaceArtifactCreationUi" | "spaceDirectoryHistoriesRef" | "hostPorts" | "adoptSharedSpaceAccess" | "resolveArtifactOpeningWithActivationRef" | "openResolvedArtifactRef" | "isCurrentEffectOwner" | "openDocumentRef" | "localDataDir" | "writeLocalDocumentRef" | "recordLocalCatalogRef" | "loadedPluginsRef" | "resolvedTargetViewState" | "rehydratedLocalDocumentsRef" | "localCatalogRef" | "replayShellOwnedCommandRef"> & {
  readonly effect: Extract<Effect, Readonly<Record<"replayShellCommand", unknown>>>;
  readonly baseSession: ActiveSession;
  readonly effectOrigin: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>["presentation"];
  readonly effectOwner: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>;
  readonly applyHostEffects: (effects: readonly Effect[], baseSession: ActiveSession, uiScope: UiDirtyScope | undefined, effectOwner: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>) => Promise<void>;
  readonly core: ShellCoreAfterSpawnedSession;
}) {
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
  } else if (actionId === "os.create-workflow-artifact") {
    const spaceId = core.openSpaceIdRef.current;
    const spaceEpoch = core.openSpaceEpochRef.current;
    const identity = identityRef.current?.userId;
    const drop = workflowArtifactDropV1(argsRecord);
    const app = drop === null ? undefined : loadedPluginsRef.current.find((plugin) => plugin.handle.pluginId === drop.pluginId)
      ?.manifest.apps.find((app) => app.id === drop.appId);
    if (!spaceId || spaceId === "demo" || baseSession.pluginId !== core.hostConfig?.pluginId || baseSession.app.id !== core.hostAppId
      || !shellDialogSessionIsCurrentV1(baseSession, shellStateRef.current.pluginRuntime.session)) {
      refuseReplay("space-required", "workflow artifact creation requires an open Studio");
    } else if (identity === undefined) {
      refuseReplay("sign-in-required", "workflow artifact creation requires a signed-in identity");
    } else if (drop === null || !app?.dialect || !app.io?.artifactSchema) {
      refuseReplay("invalid-request", "workflow artifact creation requires a registered document app");
    } else {
      const worker = ensureBackboneWorker();
      const requestId = mintDirectoryCommandRequestId();
      const current = (): boolean => core.openSpaceEpochRef.current === spaceEpoch && core.openSpaceIdRef.current === spaceId
        && identityRef.current?.userId === identity && core.backboneWorkerRef.current === worker && isCurrentEffectOwner(effectOwner);
      const name = resolveManifestLabel(app.label as LocalizedLabel | string, core.uiTerminologyRef.current, uiLocaleRef.current);
      const expectedSchema = app.io.artifactSchema;
      const expectedDialect = dialectCoordinate(app.dialect);
      const source = effectOwner.source?.document;
      void readWorkflowArtifactCreationCatalogV1(worker, spaceId).then((catalog) => {
        if (!current()) return;
        const request = catalog === null ? null : workflowArtifactCreationRequestV1(catalog, app, spaceId, requestId, name);
        if (request === null) {
          refuseReplay("invalid-request", "the Studio app has no exact kind in the trusted Space catalog");
          return;
        }
        const progressOwner = {
          requestId, spaceId, name, kindId: request.kindId, expectedCatalogGenerationId: request.expectedCatalogGenerationId,
          runtimeKey: source?.runtimeKey ?? `workflow:${spaceId}:${baseSession.instanceId}`,
          clientInstanceId: source?.clientInstanceId ?? requestId,
          sessionInstanceId: baseSession.instanceId,
        };
        const owner: SpaceArtifactCreationOwnerV1 = {
          ...progressOwner, opening: false, cancelRequested: false, ready: null,
          workflow: {
            current, expectedSchema, expectedDialect,
            finish: async ({ artifactId, artifactSchema }) => {
              const outcome = await core.onActionRef.current({
                controllerId: baseSession.app.controllerId, action: "finishSpawnApp",
                args: { ...drop, artifactId, artifactSchema, spaceId, name },
                provenance: { origin: "guest", causedBy: effectOwner.inputSeq, windowId: baseSession.viewState.windowId ?? baseSession.app.windowKinds[0]?.id ?? null },
              }, undefined, false, undefined, effectOwner);
              return outcome.kind === "applied";
            },
          },
        };
        spaceArtifactCreationOwnersRef.current.set(requestId, owner);
        setSpaceArtifactCreationUi((state) => reduceArtifactCreationProgressUiV1(state, { kind: "issued", owner: progressOwner, atMs: Date.now() }));
        worker.postMessage({ wire: encodeBackboneWorkerRequest(request) });
      }).catch((error) => {
        if (current()) refuseReplay("invalid-request", "the trusted Space catalog could not be read", error);
      });
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
      const accessRole = sharedHistory === undefined || !identityRef.current ? null : sharedDocumentOpeningRoleV1(sharedHistory.events(), sharedHistory.spaceId, identityRef.current.userId, hostPorts?.documentAccess);
      if (sharedSpaceId !== null) adoptSharedSpaceAccess(sharedSpaceId, accessRole);
      if (accessRole === "viewer" && actionId === "os.open-artifact-with" && (openingArgs?.role === "editor" || openingArgs?.role === 1)) {
        refuseReplay("view-only-access", "a read-only member asked to edit", args);
        return;
      }
      await replayOpenArtifactV1({
        resolve: () => resolveArtifactOpeningWithActivationRef.current(actionId, accessRole === "viewer" && actionId === "os.open-artifact" ? { ...openingArgs, role: "viewer" } : openingArgs),
        open: (opening) => openResolvedArtifactRef.current(opening, () => isCurrentEffectOwner(effectOwner)),
        refuse: (reason, cause) => refuseReplay(reason, reason === "router-not-ready" ? "artifact router is not ready" : "artifact opening rejected", ...(cause === undefined ? [args] : [cause, args])),
      });
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
      } else if ((await recordLocalCatalogRef.current(localCatalogAdmitMutationV1(admission))) === null) {
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
    } else if ((await recordLocalCatalogRef.current(retireLocalDocument(kept.documentId))) !== null) {
      showTransientNoticeRef.current(localCatalogNoticeTextV1("retired", uiLocaleRef.current, kept.name || kept.documentId), "info", localCatalogNoticeCodeV1("retired"));
    }
  } else if (isShellOwnedCommandId(actionId)) {
    if (!replayShellOwnedCommandRef.current(actionId, argsRecord)) {
      refuseReplay("unrouted-command", "no shell route for chrome command", actionId);
    }
  } else {
    core.onActionRef.current({
      controllerId: baseSession.app.controllerId,
      action: actionId,
      args: argsRecord,
      provenance: { origin: "guest", causedBy: effectOwner.inputSeq, windowId: typeof argsRecord?.windowId === "string" ? argsRecord.windowId : baseSession.viewState.windowId ?? null },
    });
  }
}
