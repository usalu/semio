// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🤖️AgentApprovals/component.tsx
/** @emoji ✅️ `🤖️AgentApprovals` — the human-in-the-loop approval affordance: one live, keyboard-reachable
 * surface per approval request `AgentBridge`'s `approvalRequested` frames deliver (verb, target, change
 * summary, risk, countdown, three decisions), rendered in the agent conversation, plus the footer notice
 * that points at waiting requests. The decision goes back over the bridge through `resolveApproval`.
 * Ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` packet P10; unified (modal dialog retired)
 * in ticket 26/09/18 session 11 slice U5.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useEffect, useState, type ReactElement } from "react";
import { Button, useLabel } from "@semio-tech/ui-react";
import { agentUiLabel, type PendingAgentApproval } from "../🔗️AgentBridge/🟦️.tsx";
import { type ApprovalDecision, type ApprovalWithdrawal } from "../../../../🌉️mcp/🧵️bridge/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🔖️ParseSummary
export type ApprovalRisk = "low" | "medium" | "high";

export type ParsedApprovalSummary = {
  readonly capabilityId: string | null;
  /** 🏷️ The verb's own published label — WHAT is about to happen, in words the human reads. */
  readonly capabilityTitle: string | null;
  /** 📖️ The verb's published description, the sentence the agent found it by. */
  readonly description: string | null;
  /** 🗿️ The artifact kind the verb belongs to — WHAT it is about to happen to. */
  readonly artifactKind: string | null;
  readonly diffSummary: string;
  readonly risk: ApprovalRisk | null;
  readonly requestedBy: string | null;
  /** ⏱️ How long the gateway will wait, as a DURATION from this frame's arrival — never a wall
   * clock, so a browser whose clock disagrees with the gateway's still counts down truthfully.
   * `null` for a producer that sends none, and for a non-positive value, which cannot be counted. */
  readonly timeoutMs: number | null;
};

/** 🔍️ The bridge's `ApprovalRequested.summary` wire field is a single string
 * (`🌉️mcp/🧵️bridge/🦀️.rs` tag 3), and that string's JSON payload IS the schema: the producer is
 * `🌉️mcp/🛡️policy`'s `ApprovalRequest::shell_summary`, and the shape it writes
 * (`{capabilityId, capabilityTitle, description, artifactKind, diffSummary, risk, requestedBy,
 * timeoutMs}`) is pinned for BOTH banks by `🧫️fixtures/🛡️summary` — this parser and its wgpu twin
 * `parse_approval_summary` are asserted against the same rows, so the two hosts can never drift.
 *
 * A producer that sends only the older `{capabilityId, diffSummary, risk, requestedBy}` subset, and
 * one that sends plain text, both parse: the new fields answer `null` and the surfaces omit their
 * rows rather than going blank. */
export function parseApprovalSummary(summary: string): ParsedApprovalSummary {
  const plain: ParsedApprovalSummary = { capabilityId: null, capabilityTitle: null, description: null, artifactKind: null, diffSummary: summary, risk: null, requestedBy: null, timeoutMs: null };
  try {
    const value = JSON.parse(summary) as Record<string, unknown>;
    if (value && typeof value === "object" && !Array.isArray(value)) {
      const text = (field: unknown): string | null => (typeof field === "string" && field.length > 0 ? field : null);
      const capabilityId = text(value.capabilityId);
      const risk = value.risk === "low" || value.risk === "medium" || value.risk === "high" ? (value.risk as ApprovalRisk) : null;
      const requestedBy = text(value.requestedBy);
      const diffSummary = typeof value.diffSummary === "string" ? value.diffSummary : summary;
      const timeoutMs = typeof value.timeoutMs === "number" && Number.isFinite(value.timeoutMs) && value.timeoutMs > 0 ? value.timeoutMs : null;
      if (capabilityId !== null || risk !== null || requestedBy !== null) {
        return { capabilityId, capabilityTitle: text(value.capabilityTitle), description: text(value.description), artifactKind: text(value.artifactKind), diffSummary, risk, requestedBy, timeoutMs };
      }
    }
  } catch {
    // not JSON — plain-text summary, handled by the fallback below
  }
  return plain;
}

/** ⏱️ What the countdown shows: whole seconds left of `timeoutMs` counted from when the frame
 * arrived. `null` when the producer named no timeout (nothing to count), and `0` once the budget is
 * spent — the gateway refuses on its own clock, so the surface says "out of time" rather than
 * deciding anything itself. */
export function approvalSecondsRemaining(timeoutMs: number | null, requestedAtMs: number, nowMs: number): number | null {
  if (timeoutMs === null) return null;
  return Math.max(0, Math.ceil((requestedAtMs + timeoutMs - nowMs) / 1000));
}
//#endregion 🔖️ParseSummary

//#region 🔖️AgentApprovals
/** 🏷️ The DOM address of one approval's affordance — the target a reveal moves focus to. */
export function agentApprovalElementId(approvalId: string): string {
  return `framework.approvals.${approvalId}`;
}

/** 🎯️ Moves keyboard focus onto one approval's affordance (the group, not a decision: nothing is decided
 * by a stray Enter). `false` when that affordance is not rendered. */
export function focusAgentApprovalV1(approvalId: string, root: Pick<Document, "getElementById"> = document): boolean {
  const element = root.getElementById(agentApprovalElementId(approvalId));
  if (!(element instanceof HTMLElement)) return false;
  element.focus();
  return true;
}

export type AgentApprovalAffordanceProps = {
  readonly approvalId: string;
  /** 🧾️ The gateway's one wire string (`ApprovalRequested.summary`), parsed by {@link parseApprovalSummary}. */
  readonly summary: string;
  readonly requestedAtMs: number;
  readonly state: "pending" | "resolved" | "withdrawn";
  readonly decision: ApprovalDecision | null;
  /** 🪦️ Why the gateway withdrew the request; set exactly when `state` is `withdrawn`. */
  readonly withdrawal?: ApprovalWithdrawal | null;
  /** ⛩️ Absent means no decision path is attached, and no control is offered that would do nothing. */
  readonly onDecision?: (approvalId: string, decision: ApprovalDecision, note?: string) => void;
};

function RiskBadge({ risk }: { readonly risk: ApprovalRisk | null }): ReactElement | null {
  const lowLabel = useLabel(agentUiLabel("os.agent.approvals.riskLow"));
  const mediumLabel = useLabel(agentUiLabel("os.agent.approvals.riskMedium"));
  const highLabel = useLabel(agentUiLabel("os.agent.approvals.riskHigh"));
  if (!risk) return null;
  const text = risk === "low" ? lowLabel : risk === "medium" ? mediumLabel : highLabel;
  const tone = risk === "low" ? "bg-sky-400/20 text-sky-400" : risk === "medium" ? "bg-amber-400/20 text-amber-400" : "bg-red-400/20 text-red-400";
  return (
    <span className={`rounded-sm px-single py-0.5 text-xs ${tone}`} data-semio-agent-approval-risk={risk}>
      {text}
    </span>
  );
}

/** @emoji ⛩️ THE approval affordance — the one live, keyboard-reachable place a human decides what the agent
 * may do. It renders inside the agent conversation where the request arrived; the shell never opens a
 * second, modal copy of it (ticket 26/09/18 AP1 §6.5: the modal's veil made the inline decision unreachable
 * until dismissed). WHAT happens (verb, description), WHAT it applies to, WHO asked, the change summary, the
 * risk and a polite live countdown; then the three decisions, each a real button whose accessible name
 * carries the capability it decides. A resolved request keeps its record and says how it was decided. */
export function AgentApprovalAffordance({ approvalId, summary, requestedAtMs, state, decision, withdrawal = null, onDecision }: AgentApprovalAffordanceProps): ReactElement {
  const roleLabel = useLabel(agentUiLabel("os.agent.chat.approvalRole"));
  const verbLabel = useLabel(agentUiLabel("os.agent.chat.approvalVerb"));
  const capabilityLabel = useLabel(agentUiLabel("os.agent.approvals.capability"));
  const diffLabel = useLabel(agentUiLabel("os.agent.approvals.diffSummary"));
  const requestedByLabel = useLabel(agentUiLabel("os.agent.approvals.requestedBy"));
  const riskLabel = useLabel(agentUiLabel("os.agent.approvals.riskLabel"));
  const targetLabel = useLabel(agentUiLabel("os.agent.chat.approvalTarget"));
  const expiredLabel = useLabel(agentUiLabel("os.agent.chat.approvalExpired"));
  const pendingLabel = useLabel(agentUiLabel("os.agent.chat.approvalPending"));
  const actionsLabel = useLabel(agentUiLabel("os.agent.chat.approvalActionsLabel"));
  const denyLabel = useLabel(agentUiLabel("os.agent.approvals.decisionDeny"));
  const onceLabel = useLabel(agentUiLabel("os.agent.approvals.decisionOnce"));
  const sessionLabel = useLabel(agentUiLabel("os.agent.approvals.decisionSession"));
  const decidedLabels: Readonly<Record<ApprovalDecision, string>> = {
    deny: useLabel(agentUiLabel("os.agent.approvals.decidedDeny")),
    once: useLabel(agentUiLabel("os.agent.approvals.decidedOnce")),
    session: useLabel(agentUiLabel("os.agent.approvals.decidedSession")),
  };
  const withdrawnLabels: Readonly<Record<ApprovalWithdrawal, string>> = {
    cancelled: useLabel(agentUiLabel("os.agent.approvals.withdrawnCancelled")),
    timed_out: useLabel(agentUiLabel("os.agent.approvals.withdrawnTimedOut")),
    superseded: useLabel(agentUiLabel("os.agent.approvals.withdrawnSuperseded")),
  };
  const parsed = parseApprovalSummary(summary);
  const pending = state === "pending";
  const [nowMs, setNowMs] = useState(() => Date.now());
  useEffect(() => {
    if (!pending || parsed.timeoutMs === null) return;
    const ticker = window.setInterval(() => setNowMs(Date.now()), 1000);
    return () => window.clearInterval(ticker);
  }, [pending, parsed.timeoutMs]);
  const secondsLeft = pending ? approvalSecondsRemaining(parsed.timeoutMs, requestedAtMs, nowMs) : null;
  const countdownLabel = useLabel(agentUiLabel("os.agent.chat.approvalCountdown"), { seconds: String(secondsLeft ?? 0) });
  const subject = parsed.capabilityTitle ?? parsed.capabilityId ?? parsed.diffSummary;
  const titleId = `${agentApprovalElementId(approvalId)}.title`;
  const decidable = pending && onDecision !== undefined;

  return (
    <section id={agentApprovalElementId(approvalId)} tabIndex={-1} aria-labelledby={titleId} data-semio-agent-approval-id={approvalId} data-semio-agent-approval-state={pending ? "pending" : state === "withdrawn" ? "withdrawn" : (decision ?? "resolved")} data-semio-agent-approval-withdrawal={withdrawal ?? ""} className="flex min-w-0 flex-col gap-single rounded-sm text-xs text-foreground outline-none focus-visible:ring-2 focus-visible:ring-emphasized">
      <p id={titleId} className="min-w-0 break-words" data-semio-agent-approval-verb={parsed.capabilityId ?? ""}>
        <span className="text-2xs font-semibold uppercase tracking-wide text-muted-foreground">{parsed.capabilityTitle ? `${verbLabel}: ` : `${roleLabel}: `}</span>
        <span className="font-medium">{subject}</span>
      </p>
      {parsed.capabilityId && parsed.capabilityTitle ? (
        <p className="min-w-0 break-words text-muted-foreground">
          {capabilityLabel}: <span className="font-mono">{parsed.capabilityId}</span>
        </p>
      ) : null}
      {parsed.description ? <p className="min-w-0 whitespace-pre-wrap break-words text-muted-foreground">{parsed.description}</p> : null}
      {parsed.artifactKind ? (
        <p className="min-w-0 break-words text-muted-foreground" data-semio-agent-approval-target={parsed.artifactKind}>
          {targetLabel}: {parsed.artifactKind}
        </p>
      ) : null}
      <p className="min-w-0 whitespace-pre-wrap break-words">
        <span className="text-muted-foreground">{diffLabel}: </span>
        {parsed.diffSummary}
      </p>
      {parsed.requestedBy ? (
        <p className="min-w-0 break-words text-2xs text-muted-foreground">
          {requestedByLabel}: {parsed.requestedBy}
        </p>
      ) : null}
      {parsed.risk ? (
        <p className="flex items-center gap-single text-muted-foreground">
          <span>{riskLabel}: </span>
          <RiskBadge risk={parsed.risk} />
        </p>
      ) : null}
      {pending ? (
        <p role="status" aria-live="polite" data-semio-agent-approval-countdown={secondsLeft === null ? "" : String(secondsLeft)} className="min-w-0 break-words text-2xs text-muted-foreground">
          {secondsLeft === null ? pendingLabel : secondsLeft > 0 ? countdownLabel : expiredLabel}
        </p>
      ) : (
        <p role={state === "withdrawn" ? "status" : undefined} data-semio-agent-approval-decision={decision ?? ""} className="text-2xs font-medium text-muted-foreground">
          {state === "withdrawn" && withdrawal ? withdrawnLabels[withdrawal] : decision ? decidedLabels[decision] : ""}
        </p>
      )}
      {decidable ? (
        <div role="group" aria-label={actionsLabel} className="flex flex-wrap items-center gap-single">
          <Button type="button" variant="ghost" icon="x" id={`framework.approvals.deny.${approvalId}`} text={denyLabel} aria-label={`${denyLabel}: ${subject}`} onClick={() => onDecision?.(approvalId, "deny")} />
          <Button type="button" icon="check" id={`framework.approvals.once.${approvalId}`} text={onceLabel} aria-label={`${onceLabel}: ${subject}`} onClick={() => onDecision?.(approvalId, "once")} />
          <Button type="button" variant="ghost" icon="check" id={`framework.approvals.session.${approvalId}`} text={sessionLabel} aria-label={`${sessionLabel}: ${subject}`} onClick={() => onDecision?.(approvalId, "session")} />
        </div>
      ) : null}
    </section>
  );
}

/** @emoji 📣️ The always-visible pointer to waiting approvals, in the shell footer: it decides nothing itself —
 * it says how many requests wait and takes the human (and keyboard focus) to the first one. Announces a new
 * request politely through its live region. Renders nothing while nothing waits. */
export function AgentApprovalsNotice({ approvals, onReview }: { readonly approvals: readonly PendingAgentApproval[]; readonly onReview: (approvalId: string) => void }): ReactElement | null {
  const title = useLabel(agentUiLabel("os.agent.approvals.title"));
  const one = useLabel(agentUiLabel("os.agent.approvals.waitingOne"));
  const many = useLabel(agentUiLabel("os.agent.approvals.waitingMany"), { count: String(approvals.length) });
  const reviewLabel = useLabel(agentUiLabel("os.agent.approvals.review"));
  const newest = approvals.at(-1);
  const announcement = useLabel(agentUiLabel("os.agent.approvals.requested"), { title: newest ? (parseApprovalSummary(newest.summary).capabilityTitle ?? parseApprovalSummary(newest.summary).capabilityId ?? "") : "" });
  if (!newest) return null;
  const text = approvals.length === 1 ? one : many;
  return (
    <div role="status" aria-live="polite" aria-label={`${title}: ${text}`} data-semio-agent-approvals-waiting={approvals.length} className="flex items-center gap-single px-single text-2xs text-amber-400">
      <span className="sr-only">{announcement}</span>
      <span aria-hidden="true">{text}</span>
      <Button type="button" variant="ghost" icon="hand" id="framework.approvals.review" data-semio-agent-approvals-review={approvals[0]!.approvalId} text={reviewLabel} aria-label={`${reviewLabel}: ${text}`} onClick={() => onReview(approvals[0]!.approvalId)} />
    </div>
  );
}
//#endregion 🔖️AgentApprovals
