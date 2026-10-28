/** 🚀️ The browser actor route and the plugin route that run a dispatched action and settle its ledger entry.
 *
 * 🧱️ Child module of {@link ../../🟦️.ts | useShellActions}: each function is one stage of the shell input funnel (`onAction`), moved verbatim. */
import { type ShellDialogOriginV1 } from "../../../🗨️dialog-origin/🟦️.ts";
import { type ActionDispatchLifecycleV1, SemioFaultError, SURFACE_FAULT_CODES, resolveUiDirtyScope, hostArmedViewContext } from "@semio-tech/framework";
import { programHistoryKeyV1, programHistoryProjectionV1 } from "../../../🪟️spawned-program/🟦️.ts";
import { causalOrderKeyV1, inputCommitReceiptV1, inputDiagnosticV1, type InputCommitReceiptV1, type InputDiagnosticV1, type InputDraftDispositionV1, type InputOutcomeV1, type InputRefusalReasonV1, type ShellInputActionV1 } from "../../../🎯️input-ledger/🟦️.ts";
import { type ActiveSession } from "../../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { resolveBootExampleId } from "../../../../🐚️Shell/🔒️boot-and-locks/🟦️.ts";
import { navbarExampleIdFromHistoryUpserts, historyRefreshNeededV1 } from "../../../../🛠️ShellHelpers/🔩️plumbing/🔀️example-switching/🟦️.ts";
import { appFaultNoticeV1, historyFaultNoticeV1, historyInputDraftDispositionV1, historyOutputNoticeV1, historySilentRefusalOfFaultV1, historySilentRefusalOfOutputV1 } from "../../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { EMPTY_SHELL_HISTORY_PROJECTION_V1 } from "../../../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { viewerReadOnlyNoticeText } from "../../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { browserActorActionRefusalReasonV1 } from "@semio-tech/framework/browser-bundle/action-handoff/requests";
import { startedTypedOperationV1, typedOperationCancellationActionV1 } from "../../../🏁️operation-settle/🟦️.ts";
import { encodeWindowActionInvocation, windowActionInvocation } from "../../../📨️window-invocation/🟦️.ts";
import { OBSERVED_INTERACTION_ACTION_IDS } from "../../../🎬️tutorial-recorder/🟦️.tsx";
import type { ShellCoreAfterShellUriAndDocuments } from "../../../🗄️shell-uri-documents/🟦️.ts";
import type { PluginWasmHandle } from "../../../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { beginSourceRevisionWorkV1, type SourceRevisionWorkV1 } from "../../../../🛠️ShellHelpers/📝️source-revision/work.ts";

/** 🪪️ Retires the active admitted hash while this exact document has source work in flight. */
function holdSourceRevision(core: ShellCoreAfterShellUriAndDocuments, session: ActiveSession, plugin: PluginWasmHandle): SourceRevisionWorkV1 | null {
  const owner = core.localSourceOwnerRef.current;
  if (owner?.identity.pluginId !== session.pluginId || owner.identity.appId !== session.app.id || owner.identity.sessionInstanceId !== session.instanceId) return null;
  const source = core.localSourceRevisions.acquire(owner, plugin);
  if (source === null) return null;
  return beginSourceRevisionWorkV1({
    source, owner, plugin,
    subscribe: listener => plugin.subscribeOperationCompletions(session.instanceId, listener),
    refresh: () => core.observeLocalInteraction({ plugin, instanceId: session.instanceId }),
  });
}

/** 📮️ Runs an action through the hub-mounted browser actor and maps a rejection to a typed refusal. */
export async function dispatchDirectBrowserActorAction({ dispatchDirectBrowserActorCommand, uiTerminologyRef, directBrowserActor, targetSession, dispatchViewState, action, dispatchExtraInstances, dispatchWindowId, applied, refusalReasonForError, core, refuse, propagateFailure }: Pick<ShellCoreAfterShellUriAndDocuments, "dispatchDirectBrowserActorCommand" | "uiTerminologyRef"> & {
  readonly directBrowserActor: ReturnType<ShellCoreAfterShellUriAndDocuments["directBrowserActorForSession"]>;
  readonly targetSession: ActiveSession;
  readonly dispatchViewState: NonNullable<ReturnType<typeof hostArmedViewContext>>;
  readonly action: ShellInputActionV1;
  readonly dispatchExtraInstances: readonly { readonly id: string; readonly windowKindId: string; }[];
  readonly dispatchWindowId: string | undefined;
  readonly applied: (commit?: InputCommitReceiptV1) => InputOutcomeV1;
  readonly refusalReasonForError: (error: unknown) => { readonly reason: InputRefusalReasonV1; readonly detail?: string; readonly diagnostic?: InputDiagnosticV1; readonly draftDisposition?: InputDraftDispositionV1; };
  readonly core: ShellCoreAfterShellUriAndDocuments;
  readonly refuse: (reason: InputRefusalReasonV1, detail?: string, alreadyNotified?: boolean, diagnostic?: InputDiagnosticV1, draftDisposition?: InputDraftDispositionV1) => InputOutcomeV1;
  readonly propagateFailure: boolean;
}) {
  if (directBrowserActor !== null) {
    const sourcePlugin = core.loadedPluginsRef.current.find(entry => entry.handle.pluginId === targetSession.pluginId)?.handle;
    const releaseSourceWork = sourcePlugin ? holdSourceRevision(core, targetSession, sourcePlugin) : null;
    try {
      const result = await dispatchDirectBrowserActorCommand(
        directBrowserActor,
        windowActionInvocation({ ...targetSession, viewState: dispatchViewState }, action, dispatchExtraInstances, dispatchWindowId),
        dispatchViewState,
      );
      releaseSourceWork?.admit(undefined);
      return applied(result.commit ?? undefined);
    } catch (actionError) {
      const mailboxReason = browserActorActionRefusalReasonV1(actionError);
      const mapped = refusalReasonForError(actionError);
      const { reason, detail, diagnostic, draftDisposition } = mailboxReason === null ? mapped : { ...mapped, reason: mailboxReason, detail: String(actionError instanceof Error ? actionError.message : actionError) };
      if (reason === "mutation-rejected") core.showMutationRejectedNotice((actionError as SemioFaultError).fault);
      const appNotice = reason === "dispatch-failed" && actionError instanceof SemioFaultError ? appFaultNoticeV1(actionError.fault, targetSession.app, uiTerminologyRef.current) : null;
      if (appNotice !== null) core.showTransientNotice(appNotice.text, appNotice.kind, appNotice.code);
      const outcome = refuse(reason, appNotice?.code ?? detail, reason === "mutation-rejected" || appNotice !== null, diagnostic, draftDisposition);
      if (propagateFailure) throw actionError;
      return outcome;
    } finally {
      releaseSourceWork?.finish();
    }
  }
}

/** 🎬️ Runs an action through the program handler, applies its history patch and host effects and settles the ledger entry. */
export async function dispatchProgramAction({ sessionWorkRef, isCurrentEffectOwner, applyHistoryPatch, applyLeftoverInteractionView, historyStore, lastDispatchedExampleIdRef, exampleOptionsRef, defaults, dispatch, refreshHistorySnapshot, applyHostEffects, observeLocalInteraction, awaitOperationSettle, uiTerminologyRef, uiLocale, dropForSealedInstance, dropForRetiredInstance, targetSession, plugin, dispatchViewState, action, dispatchExtraInstances, dispatchWindowId, entry, actionOwner, lifecycle, onAction, actionOrigin, hostWindowId, core, applied, refuse, refusalReasonForError, propagateFailure }: Pick<ShellCoreAfterShellUriAndDocuments, "sessionWorkRef" | "isCurrentEffectOwner" | "applyHistoryPatch" | "applyLeftoverInteractionView" | "historyStore" | "lastDispatchedExampleIdRef" | "exampleOptionsRef" | "defaults" | "dispatch" | "refreshHistorySnapshot" | "applyHostEffects" | "observeLocalInteraction" | "awaitOperationSettle" | "uiTerminologyRef" | "uiLocale" | "dropForSealedInstance" | "dropForRetiredInstance"> & {
  readonly targetSession: ActiveSession;
  readonly plugin: ShellCoreAfterShellUriAndDocuments["loadedPlugins"][number]["handle"];
  readonly dispatchViewState: NonNullable<ReturnType<typeof hostArmedViewContext>>;
  readonly action: ShellInputActionV1;
  readonly dispatchExtraInstances: readonly { readonly id: string; readonly windowKindId: string; }[];
  readonly dispatchWindowId: string | undefined;
  readonly entry: ReturnType<ShellCoreAfterShellUriAndDocuments["inputLedgerRef"]["current"]["issue"]>;
  readonly actionOwner: ReturnType<ShellCoreAfterShellUriAndDocuments["captureEffectOwner"]>;
  readonly lifecycle: ActionDispatchLifecycleV1 | undefined;
  readonly onAction: (requested: ShellInputActionV1, submittedOrigin?: ShellDialogOriginV1, propagateFailure?: boolean, lifecycle?: ActionDispatchLifecycleV1, requiredOperationOwner?: ReturnType<ShellCoreAfterShellUriAndDocuments["captureEffectOwner"]>) => Promise<InputOutcomeV1>;
  readonly actionOrigin: Readonly<{ pluginId: string; appId: string; controllerId: string; sessionInstanceId: number; document: Readonly<{ runtimeKey: string; clientInstanceId: string; scope: Readonly<{ spaceId: string; documentId: string; }> | null; }> | null; }> | null;
  readonly hostWindowId: string | undefined;
  readonly core: ShellCoreAfterShellUriAndDocuments;
  readonly applied: (commit?: InputCommitReceiptV1) => InputOutcomeV1;
  readonly refuse: (reason: InputRefusalReasonV1, detail?: string, alreadyNotified?: boolean, diagnostic?: InputDiagnosticV1, draftDisposition?: InputDraftDispositionV1) => InputOutcomeV1;
  readonly refusalReasonForError: (error: unknown) => { readonly reason: InputRefusalReasonV1; readonly detail?: string; readonly diagnostic?: InputDiagnosticV1; readonly draftDisposition?: InputDraftDispositionV1; };
  readonly propagateFailure: boolean;
}) {
  const releaseActionWork = sessionWorkRef.current.begin(targetSession.pluginId, targetSession.instanceId, "typed-operation");
  const releaseSourceWork = holdSourceRevision(core, targetSession, plugin);
  try {
    const response = await plugin.handleAction(targetSession.instanceId, encodeWindowActionInvocation({ ...targetSession, viewState: dispatchViewState }, action, dispatchExtraInstances, dispatchWindowId), dispatchViewState, { order: causalOrderKeyV1(entry.provenance) });
    const startedOperation = startedTypedOperationV1(response.output);
    releaseSourceWork?.admit(startedOperation?.operation);
    if (startedOperation !== undefined && isCurrentEffectOwner(actionOwner)) lifecycle?.started({
      operationId: startedOperation.operationId,
      generation: startedOperation.generation,
      cancel: () => onAction(typedOperationCancellationActionV1(action, startedOperation), actionOrigin!, false, undefined, actionOwner),
    });
    applyHistoryPatch(response.historyPatch, false, { pluginId: targetSession.pluginId, instanceId: targetSession.instanceId });
    applyLeftoverInteractionView(response.output, hostWindowId);
    const rejectedNotice = historyOutputNoticeV1(response.output, programHistoryProjectionV1(historyStore.get(), programHistoryKeyV1(targetSession), EMPTY_SHELL_HISTORY_PROJECTION_V1).editCount);
    if (rejectedNotice !== null) core.showTransientNotice(rejectedNotice.text, rejectedNotice.kind, rejectedNotice.code);
    const ignoredRefusal = historySilentRefusalOfOutputV1(response.output);
    if (ignoredRefusal !== null) console.debug(`[os-shell] ${action.action} was ignored: ${ignoredRefusal}`);
    const navbarExample = navbarExampleIdFromHistoryUpserts(response.historyPatch?.upserts, lastDispatchedExampleIdRef.current, resolveBootExampleId("", exampleOptionsRef.current, defaults.exampleId));
    if (navbarExample !== undefined) dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: navbarExample });
    const needsHistoryRefresh = historyRefreshNeededV1(action.action, response.historyPatch);
    if (!isCurrentEffectOwner(actionOwner)) {
      await awaitOperationSettle(targetSession.instanceId, startedOperation?.operation);
      if (needsHistoryRefresh) refreshHistorySnapshot({ pluginId: targetSession.pluginId, instanceId: targetSession.instanceId });
      return applied();
    }
    await applyHostEffects(response.requestedEffects ?? [], { ...targetSession, viewState: dispatchViewState }, resolveUiDirtyScope(response.uiScope), actionOwner);
    const completion = await awaitOperationSettle(targetSession.instanceId, startedOperation?.operation);
    if (needsHistoryRefresh) refreshHistorySnapshot({ pluginId: targetSession.pluginId, instanceId: targetSession.instanceId });
    return applied(completion === null ? undefined : inputCommitReceiptV1(completion.operation, completion.revision));
  } catch (actionError) {
    let outcome: InputOutcomeV1;
    const diagnostic = actionError instanceof SemioFaultError ? inputDiagnosticV1(actionError.fault) : undefined;
    const historyNotice = actionError instanceof SemioFaultError ? historyFaultNoticeV1(actionError.fault, programHistoryProjectionV1(historyStore.get(), programHistoryKeyV1(targetSession), EMPTY_SHELL_HISTORY_PROJECTION_V1).editCount) : null;
    const silentRefusal = actionError instanceof SemioFaultError ? historySilentRefusalOfFaultV1(actionError.fault) : null;
    const draftDisposition = actionError instanceof SemioFaultError ? historyInputDraftDispositionV1(actionError.fault) : undefined;
    const appNotice = actionError instanceof SemioFaultError ? appFaultNoticeV1(actionError.fault, targetSession.app, uiTerminologyRef.current) : null;
    if (core.isViewerReadOnlyFault(actionError)) {
      core.showTransientNotice(viewerReadOnlyNoticeText(uiLocale), "info", SURFACE_FAULT_CODES.ViewerReadOnly);
      outcome = refuse("viewer-read-only", undefined, true, diagnostic, draftDisposition);
    } else if (core.isMutationRejectedFault(actionError)) {
      core.showMutationRejectedNotice((actionError as SemioFaultError).fault);
      outcome = refuse("mutation-rejected", undefined, true, diagnostic, draftDisposition);
    } else if (silentRefusal !== null) {
      outcome = refuse("dispatch-failed", silentRefusal, true, diagnostic, draftDisposition);
    } else if (historyNotice !== null) {
      core.showTransientNotice(historyNotice.text, historyNotice.kind, historyNotice.code);
      outcome = refuse("dispatch-failed", historyNotice.code, true, diagnostic, draftDisposition);
    } else if (appNotice !== null) {
      core.showTransientNotice(appNotice.text, appNotice.kind, appNotice.code);
      outcome = refuse("dispatch-failed", appNotice.code, true, diagnostic);
    } else if (dropForSealedInstance(targetSession, "action failure", action.action)) {
      outcome = refuse("instance-sealed", `${targetSession.pluginId}#${targetSession.instanceId}`);
    } else if (dropForRetiredInstance(targetSession, `action ${action.action}`, actionError)) {
      outcome = refuse("instance-retired", `${targetSession.pluginId}#${targetSession.instanceId}`);
    } else {
      const { reason, detail, diagnostic, draftDisposition } = refusalReasonForError(actionError);
      outcome = refuse(reason, detail, false, diagnostic, draftDisposition);
    }
    if (propagateFailure) throw actionError;
    return outcome;
  } finally {
    releaseSourceWork?.finish();
    releaseActionWork();
    if (releaseSourceWork === null && OBSERVED_INTERACTION_ACTION_IDS.has(action.action)) observeLocalInteraction({ plugin, instanceId: targetSession.instanceId });
  }
}
