/** 🔲️ The shell-intercepted `fitGraph` action of the input funnel: fits the whole node graph the target window presents and persists the camera through the mounted canvas (the same `nodeGraphViewport` a wheel zoom publishes); a window with no live graph is refused with a polite localized notice.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | the input funnel}; the camera law is `dagFitCamera`, pinned by the language-agnostic `node-graph-fit-action` fixture. */
import { type ShellLocale } from "@semio-tech/framework";
import { runNodeGraphFitV1 } from "../../../../🕸️NodeGraph/🎯️fit-graph/🟦️.ts";
import { type InputCommitReceiptV1, type InputDiagnosticV1, type InputDraftDispositionV1, type InputOutcomeV1, type InputRefusalReasonV1, type ShellInputActionV1 } from "../../../🎯️input-ledger/🟦️.ts";
import type { ShellCoreAfterShellUriAndDocuments } from "../../../🗄️shell-uri-documents/🟦️.ts";

/** 🔊️ Notice code of a refused `fitGraph`; mirrors Rust `NODE_GRAPH_FIT_REFUSED_CODE`. */
export const NODE_GRAPH_FIT_REFUSED_CODE_V1 = "shell.node-graph-fit.refused";

/** 🗺️ The `data-surface-id` of every element in the body of window `windowId`, in document order. */
export function surfaceIdsInWindowV1(windowId: string, root: Document | undefined = typeof document === "undefined" ? undefined : document): readonly string[] {
  if (root === undefined) return [];
  const tab = [...root.querySelectorAll<HTMLElement>('[data-slot="mode-dock-tab"][data-window-id]')].find((element) => element.dataset.windowId === windowId && element.dataset.stackActive === "true");
  const panel = tab?.querySelector('[role="tab"]')?.getAttribute("aria-controls");
  const body = panel === null || panel === undefined ? null : root.getElementById(panel);
  return body === null ? [] : [...body.querySelectorAll<HTMLElement>("[data-surface-id]")].flatMap((element) => element.dataset.surfaceId ?? []);
}

/** 🗣️ The refusal notice in the interface language, no default language. */
export function fitGraphNoticeV1(locale: ShellLocale): { readonly text: string; readonly severity: "warning"; readonly code: string } {
  return { text: locale === "de" ? "In diesem Fenster gibt es keinen Knotengraphen zum Einpassen" : "No node graph to fit in this window", severity: "warning", code: NODE_GRAPH_FIT_REFUSED_CODE_V1 };
}

/** 🚀️ Fits the first graph surface of the action's window (else the active window) that a mounted canvas answers for. */
export function handleFitGraphAction({ action, core, refuse, applied }: {
  readonly action: ShellInputActionV1;
  readonly core: ShellCoreAfterShellUriAndDocuments;
  readonly refuse: (reason: InputRefusalReasonV1, detail?: string, alreadyNotified?: boolean, diagnostic?: InputDiagnosticV1, draftDisposition?: InputDraftDispositionV1) => InputOutcomeV1;
  readonly applied: (commit?: InputCommitReceiptV1) => InputOutcomeV1;
}): InputOutcomeV1 {
  const scoped = typeof action.args === "object" && action.args != null ? (action.args as { windowId?: unknown }).windowId : undefined;
  const windowId = typeof scoped === "string" ? scoped : core.activeWindowIdRef.current;
  if (windowId != null && surfaceIdsInWindowV1(windowId).some(runNodeGraphFitV1)) return applied();
  const notice = fitGraphNoticeV1(core.uiLocaleRef.current as ShellLocale);
  core.showTransientNoticeRef.current(notice.text, notice.severity, notice.code);
  return refuse("not-applied", `window ${windowId ?? ""} presents no node graph`, true);
}
