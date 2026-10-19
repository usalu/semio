/** 🧰️ The shell-intercepted utility and tool activation of the input funnel.
 *
 * 🧱️ Child module of {@link ../../🟦️.ts | useShellActions}: each function is one stage of the shell input funnel (`onAction`), moved verbatim. */
import { type ActionDescriptor, resolveUiDirtyScope, panelViewContext, windowViewContext } from "@semio-tech/framework";
import { guestActiveUtilityByWindowIdV1 } from "../../../🪟️spawned-program/🟦️.ts";
import { causalOrderKeyV1, expectedGenerationFromArgsV1, resolveUtilityActivationV1, type InputCommitReceiptV1, type InputDiagnosticV1, type InputDraftDispositionV1, type InputOutcomeV1, type InputRefusalReasonV1, type ShellInputActionV1 } from "../../../🎯️input-ledger/🟦️.ts";
import { type ActiveSession } from "../../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import type { ViewModel } from "../../../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { leftoverWorldSelectionOverlayV1, publishLeftoverWorldSelectionV1 } from "../../../../🌐️World3dHost/🔗️instance-selection/🟦️.ts";
import { sessionWindowInstances } from "../../../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { encodeWindowActionInvocation } from "../../../📨️window-invocation/🟦️.ts";
import type { ShellCoreAfterShellUriAndDocuments } from "../../../🗄️shell-uri-documents/🟦️.ts";

/** 🔧️ Compare-and-sets a window's utility register and forwards the resolved utility to the owning program. */
export async function handleSetActiveUtilityAction({ activeWindowIdRef, activeUtilityByWindowIdRef, writeUtilityRegister, activeToolIdRef, writeToolRegister, completeIntroductionInteraction, loadedPlugins, uiLocaleRef, uiTerminologyRef, spawnedIdsRef, applyHistoryPatch, applyLeftoverInteractionView, isCurrentEffectOwner, applyHostEffects, action, refuse, programOfWindow, entry, primaryActionOwner, applied }: Pick<ShellCoreAfterShellUriAndDocuments, "activeWindowIdRef" | "activeUtilityByWindowIdRef" | "writeUtilityRegister" | "activeToolIdRef" | "writeToolRegister" | "completeIntroductionInteraction" | "loadedPlugins" | "uiLocaleRef" | "uiTerminologyRef" | "spawnedIdsRef" | "applyHistoryPatch" | "applyLeftoverInteractionView" | "isCurrentEffectOwner" | "applyHostEffects"> & {
  readonly action: ShellInputActionV1;
  readonly refuse: (reason: InputRefusalReasonV1, detail?: string, alreadyNotified?: boolean, diagnostic?: InputDiagnosticV1, draftDisposition?: InputDraftDispositionV1) => InputOutcomeV1;
  readonly programOfWindow: (windowId: string) => { readonly spawnedId: string | null; readonly session: ActiveSession; readonly extraInstances: readonly { readonly id: string; readonly windowKindId: string; }[]; readonly guestWindowId: string; };
  readonly entry: ReturnType<ShellCoreAfterShellUriAndDocuments["inputLedgerRef"]["current"]["issue"]>;
  readonly primaryActionOwner: ReturnType<ShellCoreAfterShellUriAndDocuments["captureEffectOwner"]>;
  readonly applied: (commit?: InputCommitReceiptV1) => InputOutcomeV1;
}) {
  const args = typeof action.args === "object" && action.args != null ? (action.args as { utilityId?: unknown; windowId?: unknown }) : {};
  const windowId = typeof args.windowId === "string" && args.windowId ? args.windowId : (activeWindowIdRef.current ?? "");
  if (!windowId) return refuse("view-state-unresolved", "no target window");
  const requestedUtility = typeof args.utilityId === "string" ? args.utilityId : "";
  const next = resolveUtilityActivationV1(activeUtilityByWindowIdRef.current[windowId], requestedUtility);
  const expected = expectedGenerationFromArgsV1(action.args);
  const write = writeUtilityRegister(windowId, next, expected);
  if (write.kind === "refused") return refuse("stale-generation", `window=${windowId} expected=${write.expected} current=${write.register.generation}`);
  if (next && activeToolIdRef.current) writeToolRegister(null, null);
  if (next) completeIntroductionInteraction((interaction) => interaction.on.kind === "utility" && interaction.on.id === next);
  const { spawnedId: utilitySpawnedId, session: utilitySession, extraInstances: utilityExtraInstances, guestWindowId: utilityGuestWindowId } = programOfWindow(windowId);
  const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === utilitySession.pluginId);
  const program = pluginEntry?.handle;
  if (program) {
    const viewState = windowViewContext(
      {
        ...utilitySession.viewState,
        locale: uiLocaleRef.current,
        terminology: uiTerminologyRef.current,
        activeToolId: next ? undefined : activeToolIdRef.current ?? undefined,
        activeUtilityByWindowId: guestActiveUtilityByWindowIdV1(activeUtilityByWindowIdRef.current, utilitySpawnedId, spawnedIdsRef.current),
        windowInstances: sessionWindowInstances(utilitySession.app, utilityExtraInstances).map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
      },
      utilityGuestWindowId,
    );
    if (!viewState) return refuse("view-state-unresolved", `window=${windowId}`);
    const forwarded: ActionDescriptor = { controllerId: action.controllerId, action: action.action, args: { utilityId: next } };
    await program
      .handleAction(utilitySession.instanceId, encodeWindowActionInvocation({ ...utilitySession, viewState }, forwarded, utilityExtraInstances, utilityGuestWindowId), viewState, { order: causalOrderKeyV1(entry.provenance) })
      .then((response) => {
        applyHistoryPatch(response.historyPatch, false, { pluginId: utilitySession.pluginId, instanceId: utilitySession.instanceId });
        applyLeftoverInteractionView(response.output, windowId);
        if (!isCurrentEffectOwner(primaryActionOwner)) return;
        return applyHostEffects(response.requestedEffects ?? [], { ...utilitySession, viewState }, resolveUiDirtyScope(response.uiScope), primaryActionOwner);
      })
      .catch((utilityError) => undefined);
  }
  return applied();
}

/** 🛠️ Compare-and-sets the mode-level tool register, clears every window utility and forwards the tool to the program. */
export async function handleSetActiveToolAction({ activeToolIdRef, writeToolRegister, clearAllWindowUtilities, completeIntroductionInteraction, activeWindowIdRef, loadedPlugins, uiLocaleRef, uiTerminologyRef, activeUtilityByWindowIdRef, spawnedIdsRef, applyHistoryPatch, applyLeftoverInteractionView, isCurrentEffectOwner, applyHostEffects, action, refuse, programOfWindow, entry, primaryActionOwner, applied }: Pick<ShellCoreAfterShellUriAndDocuments, "activeToolIdRef" | "writeToolRegister" | "clearAllWindowUtilities" | "completeIntroductionInteraction" | "activeWindowIdRef" | "loadedPlugins" | "uiLocaleRef" | "uiTerminologyRef" | "activeUtilityByWindowIdRef" | "spawnedIdsRef" | "applyHistoryPatch" | "applyLeftoverInteractionView" | "isCurrentEffectOwner" | "applyHostEffects"> & {
  readonly action: ShellInputActionV1;
  readonly refuse: (reason: InputRefusalReasonV1, detail?: string, alreadyNotified?: boolean, diagnostic?: InputDiagnosticV1, draftDisposition?: InputDraftDispositionV1) => InputOutcomeV1;
  readonly programOfWindow: (windowId: string) => { readonly spawnedId: string | null; readonly session: ActiveSession; readonly extraInstances: readonly { readonly id: string; readonly windowKindId: string; }[]; readonly guestWindowId: string; };
  readonly entry: ReturnType<ShellCoreAfterShellUriAndDocuments["inputLedgerRef"]["current"]["issue"]>;
  readonly primaryActionOwner: ReturnType<ShellCoreAfterShellUriAndDocuments["captureEffectOwner"]>;
  readonly applied: (commit?: InputCommitReceiptV1) => InputOutcomeV1;
}) {
  const args = typeof action.args === "object" && action.args != null ? (action.args as { toolId?: unknown }) : {};
  const requestedTool = typeof args.toolId === "string" ? args.toolId : "";
  const next = resolveUtilityActivationV1(activeToolIdRef.current, requestedTool);
  const expected = expectedGenerationFromArgsV1(action.args);
  const write = writeToolRegister(next, expected);
  if (write.kind === "refused") return refuse("stale-generation", `tool expected=${write.expected} current=${write.register.generation}`);
  if (next) clearAllWindowUtilities();
  const priorToolLeftover = leftoverWorldSelectionOverlayV1();
  publishLeftoverWorldSelectionV1(
    {
      ids: priorToolLeftover?.ids ?? [],
      hoveredId: priorToolLeftover?.hoveredId ?? null,
      hoveredDomain: priorToolLeftover?.hoveredDomain,
      gumballActive: priorToolLeftover?.gumballActive ?? false,
      gumballAnchorId: priorToolLeftover?.gumballAnchorId ?? null,
      activeUtility: next === "fill" ? "fill" : "select",
      activeToolId: next,
    },
    { kind: "allWindows" },
  );
  if (next) completeIntroductionInteraction((interaction) => interaction.on.kind === "tool" && interaction.on.id === next);
  const { spawnedId: toolSpawnedId, session: toolSession, extraInstances: toolExtraInstances, guestWindowId: toolGuestWindowId } = programOfWindow(activeWindowIdRef.current ?? "");
  const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === toolSession.pluginId);
  const program = pluginEntry?.handle;
  if (program) {
    const baseToolViewState: ViewModel = {
      ...toolSession.viewState,
      locale: uiLocaleRef.current,
      terminology: uiTerminologyRef.current,
      activeToolId: next ?? undefined,
      activeUtilityId: next ? undefined : toolSession.viewState.activeUtilityId,
      windowInstances: sessionWindowInstances(toolSession.app, toolExtraInstances).map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
      activeUtilityByWindowId: guestActiveUtilityByWindowIdV1(activeUtilityByWindowIdRef.current, toolSpawnedId, spawnedIdsRef.current),
    };
    const viewState = toolGuestWindowId ? windowViewContext(baseToolViewState, toolGuestWindowId) : panelViewContext(baseToolViewState);
    if (!viewState) return refuse("view-state-unresolved", `tool window=${toolGuestWindowId}`);
    const forwarded: ActionDescriptor = { controllerId: toolSession.app.controllerId, action: action.action, args: { toolId: next } };
    await program
      .handleAction(toolSession.instanceId, encodeWindowActionInvocation({ ...toolSession, viewState }, forwarded, toolExtraInstances, toolGuestWindowId || undefined), viewState, { order: causalOrderKeyV1(entry.provenance) })
      .then((response) => {
        applyHistoryPatch(response.historyPatch, false, { pluginId: toolSession.pluginId, instanceId: toolSession.instanceId });
        applyLeftoverInteractionView(response.output);
        if (!isCurrentEffectOwner(primaryActionOwner)) return;
        return applyHostEffects(response.requestedEffects ?? [], { ...toolSession, viewState }, { kind: "full" }, primaryActionOwner);
      })
      .catch((toolError) => undefined);
  }
  return applied();
}
