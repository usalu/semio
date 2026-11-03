/** 🎯️ The shell-intercepted `focusWindow` action of the input funnel: opens the window kind when it is closed, raises it, moves the keyboard focus into it and announces it politely (the transient notice is a live `status` region) in the interface language.
 *
 * 🧱️ Child module of {@link ../../🟦️.ts | useShellActions}; the planning is the pure {@link ../../../🎯️focus-window/🟦️.ts | focus-window} module the language-agnostic fixture drives. */
import { type LocalizedLabel, type ShellLocale, windowLayoutWindowIdsV1 } from "@semio-tech/framework";
import { insertWindowAtDropZone, type WindowLayoutNode } from "@semio-tech/ui-react";
import { type InputCommitReceiptV1, type InputDiagnosticV1, type InputDraftDispositionV1, type InputOutcomeV1, type InputRefusalReasonV1, type ShellInputActionV1 } from "../../../🎯️input-ledger/🟦️.ts";
import { focusWindowArgsV1, focusWindowBodyV1, focusWindowNoticeV1, planFocusWindowV1, windowKindIdsOfLayoutV1, type FocusWindowDeviceV1 } from "../../../🎯️focus-window/🟦️.ts";
import { withLocalizedWindowKindLabels } from "../../../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterShellUriAndDocuments } from "../../../🗄️shell-uri-documents/🟦️.ts";
import { selectUiDevice } from "../../../../🐚️Shell/🕹️shell-store/🟦️.ts";
import { resolveFrameworkLayoutSeed } from "../../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { resolveManifestLabel } from "../../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";

/** 🎯️ Plans and applies one `focusWindow`: raise the open window, or open the closed one as a root split at the right, activate it, move the keyboard focus into it and announce the outcome; a kind the app never declared, or one the presented device layout lacks without a usable stand-in, is refused with a warning notice. */
export function handleFocusWindowAction({ action, session, core, refuse, applied }: {
  readonly action: ShellInputActionV1;
  readonly session: NonNullable<ShellCoreAfterShellUriAndDocuments["session"]>;
  readonly core: ShellCoreAfterShellUriAndDocuments;
  readonly refuse: (reason: InputRefusalReasonV1, detail?: string, alreadyNotified?: boolean, diagnostic?: InputDiagnosticV1, draftDisposition?: InputDraftDispositionV1) => InputOutcomeV1;
  readonly applied: (commit?: InputCommitReceiptV1) => InputOutcomeV1;
}): InputOutcomeV1 {
  const args = focusWindowArgsV1(action.args);
  if (args === null) return refuse("view-state-unresolved", "focusWindow needs the argument windowKindId");
  const state = core.shellStateRef.current;
  const locale = core.uiLocaleRef.current as ShellLocale;
  const terminology = core.uiTerminologyRef.current;
  const device = selectUiDevice(state, core.measuredDevice) as FocusWindowDeviceV1;
  const base: WindowLayoutNode = state.layout.shellLayout ?? resolveFrameworkLayoutSeed(session.app.defaultLayout, withLocalizedWindowKindLabels(session.app.windowKinds), state.windowUi.appLabelsOverlay, terminology, core.uiLocaleRef.current).modeLayout;
  const extraInstances = core.extraWindowInstancesRef.current;
  const declaredKindIds = session.app.windowKinds.map((kind) => kind.id);
  const mounted = [...windowLayoutWindowIdsV1(base)].map((windowId) => ({ windowId, windowKindId: extraInstances.find((instance) => instance.id === windowId)?.windowKindId ?? windowId }));
  const layouts = session.app.namedLayouts.map((layout) => ({ id: layout.id, windowKindIds: windowKindIdsOfLayoutV1(layout.layout) }));
  const plan = planFocusWindowV1({ declaredKindIds, layouts, device, requested: args.windowKindId, fallback: args.fallback, mounted, activeWindowId: core.activeWindowIdRef.current });
  const nameOf = (windowKindId: string, noticeLocale: ShellLocale): string => {
    const kind = session.app.windowKinds.find((candidate) => candidate.id === windowKindId);
    return kind === undefined ? windowKindId : resolveManifestLabel(kind.label as LocalizedLabel | string, terminology, noticeLocale);
  };
  const notice = focusWindowNoticeV1(plan, nameOf, locale);
  core.showTransientNoticeRef.current(notice.text, notice.severity, notice.code);
  if (plan.outcome === "refused") return refuse("not-applied", `window kind ${plan.windowKindId} ${plan.reason === "undeclared" ? "is not declared by the app" : `is not part of the ${plan.device} layout`}`, true);
  if (plan.outcome === "opened") core.dispatch({ type: "SET_SHELL_LAYOUT", value: (current) => insertWindowAtDropZone(current ?? base, plan.windowId, { kind: "root-split", side: "right" }) });
  core.dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: plan.windowId });
  void focusWindowBodyV1(plan.windowId);
  return applied();
}
