/** ✏️ The shell-intercepted `focusRowControl` action of the input funnel: moves the keyboard focus into the first editable cell of the named tree row, or refuses politely (the transient notice is a live `status` region) in the interface language when the row holds none.
 *
 * 🧱️ Child module of {@link ../../🟦️.ts | useShellActions}; the rule is the pure {@link ../../../🎯️focus-row-control/🟦️.ts | focus-row-control} module the language-agnostic fixture drives. */
import { type ShellLocale } from "@semio-tech/framework";
import { type InputCommitReceiptV1, type InputDiagnosticV1, type InputDraftDispositionV1, type InputOutcomeV1, type InputRefusalReasonV1, type ShellInputActionV1 } from "../../../🎯️input-ledger/🟦️.ts";
import { focusRowControlArgsV1, focusRowControlRefusalV1, focusRowControlV1 } from "../../../🎯️focus-row-control/🟦️.ts";
import type { ShellCoreAfterShellUriAndDocuments } from "../../../🗄️shell-uri-documents/🟦️.ts";

/** ✏️ Runs one `focusRowControl`: focuses the cell, or announces the refusal and refuses the action. */
export function handleFocusRowControlAction({ action, core, refuse, applied }: {
  readonly action: ShellInputActionV1;
  readonly core: ShellCoreAfterShellUriAndDocuments;
  readonly refuse: (reason: InputRefusalReasonV1, detail?: string, alreadyNotified?: boolean, diagnostic?: InputDiagnosticV1, draftDisposition?: InputDraftDispositionV1) => InputOutcomeV1;
  readonly applied: (commit?: InputCommitReceiptV1) => InputOutcomeV1;
}): InputOutcomeV1 {
  const rowKey = focusRowControlArgsV1(action.args);
  if (rowKey === null) return refuse("view-state-unresolved", "focusRowControl needs the argument rowId");
  if (typeof document !== "undefined" && focusRowControlV1(document, rowKey) !== null) return applied();
  const notice = focusRowControlRefusalV1(core.uiLocaleRef.current as ShellLocale);
  core.showTransientNoticeRef.current(notice.text, notice.severity, notice.code);
  return refuse("not-applied", `row ${rowKey} holds no editable control`, true);
}
