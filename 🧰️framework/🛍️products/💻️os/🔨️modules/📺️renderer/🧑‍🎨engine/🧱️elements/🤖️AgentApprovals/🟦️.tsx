// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🤖️AgentApprovals/component.tsx
/** @emoji ✅️ `🤖️AgentApprovals` — the `os.agent.approvals` human-in-the-loop dialog: lists parked
 * approval requests (capability, change summary, risk) delivered by `AgentBridge`'s
 * `approvalRequested` frames and sends the human's `Approval{decision}` back over the bridge via
 * `resolveApproval`. Ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` packet P10.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useEffect, useState, type ReactElement } from "react";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, useLabel } from "@semio-tech/ui-react";
import { agentUiLabel, type PendingAgentApproval } from "../🔗️AgentBridge/🟦️.tsx";
import { type ApprovalDecision } from "../../../../🌉️mcp/🧵️bridge/🟦️.ts";
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
export type AgentApprovalsProps = {
  readonly approvals: readonly PendingAgentApproval[];
  readonly onDecision: (approvalId: string, decision: ApprovalDecision, note?: string) => void;
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

function ApprovalRow({ approval, onDecision }: { readonly approval: PendingAgentApproval; readonly onDecision: AgentApprovalsProps["onDecision"] }): ReactElement {
  const capabilityLabel = useLabel(agentUiLabel("os.agent.approvals.capability"));
  const diffLabel = useLabel(agentUiLabel("os.agent.approvals.diffSummary"));
  const requestedByLabel = useLabel(agentUiLabel("os.agent.approvals.requestedBy"));
  const riskLabel = useLabel(agentUiLabel("os.agent.approvals.riskLabel"));
  const denyLabel = useLabel(agentUiLabel("os.agent.approvals.decisionDeny"));
  const onceLabel = useLabel(agentUiLabel("os.agent.approvals.decisionOnce"));
  const sessionLabel = useLabel(agentUiLabel("os.agent.approvals.decisionSession"));
  const targetLabel = useLabel(agentUiLabel("os.agent.chat.approvalTarget"));
  const expiredLabel = useLabel(agentUiLabel("os.agent.chat.approvalExpired"));
  const parsed = parseApprovalSummary(approval.summary);
  const [nowMs, setNowMs] = useState(() => Date.now());
  useEffect(() => {
    if (parsed.timeoutMs === null) return;
    const ticker = window.setInterval(() => setNowMs(Date.now()), 1000);
    return () => window.clearInterval(ticker);
  }, [parsed.timeoutMs]);
  const secondsLeft = approvalSecondsRemaining(parsed.timeoutMs, approval.requestedAtMs, nowMs);
  const countdownLabel = useLabel(agentUiLabel("os.agent.chat.approvalCountdown"), { seconds: String(secondsLeft ?? 0) });

  return (
    <li className="space-y-2 border-b py-3 last:border-b-0" data-semio-agent-approval-id={approval.approvalId}>
      <div className="space-y-1 text-sm">
        {parsed.capabilityTitle ? <p className="font-medium">{parsed.capabilityTitle}</p> : null}
        {parsed.capabilityId ? (
          <p>
            <span className="text-muted-foreground">{capabilityLabel}: </span>
            <span className="font-medium">{parsed.capabilityId}</span>
          </p>
        ) : null}
        {parsed.description ? <p className="text-muted-foreground">{parsed.description}</p> : null}
        {parsed.artifactKind ? (
          <p className="text-muted-foreground" data-semio-agent-approval-target={parsed.artifactKind}>
            <span>{targetLabel}: </span>
            {parsed.artifactKind}
          </p>
        ) : null}
        <p>
          <span className="text-muted-foreground">{diffLabel}: </span>
          {parsed.diffSummary}
        </p>
        {secondsLeft !== null ? (
          <p role="status" aria-live="polite" data-semio-agent-approval-countdown={String(secondsLeft)} className="text-muted-foreground">
            {secondsLeft > 0 ? countdownLabel : expiredLabel}
          </p>
        ) : null}
        {parsed.requestedBy ? (
          <p className="text-muted-foreground">
            <span>{requestedByLabel}: </span>
            <span>{parsed.requestedBy}</span>
          </p>
        ) : null}
        {parsed.risk ? (
          <p className="flex items-center gap-1.5 text-muted-foreground">
            <span>{riskLabel}: </span>
            <RiskBadge risk={parsed.risk} />
          </p>
        ) : null}
      </div>
      {/* ⌨️ Wraps at phone width, and every control carries the same `framework.approvals.<decision>.<id>`
          address its React and wgpu twins use, so a keyboard, a screen reader and a gate all reach the
          same three decisions by name rather than by position. */}
      <div className="flex flex-wrap gap-2">
        <button type="button" id={`framework.approvals.deny.${approval.approvalId}`} className="rounded-sm border px-double py-1 text-sm text-red-400" aria-label={`${denyLabel}: ${parsed.capabilityId ?? parsed.diffSummary}`} onClick={() => onDecision(approval.approvalId, "deny")}>
          {denyLabel}
        </button>
        <button type="button" id={`framework.approvals.once.${approval.approvalId}`} className="rounded-sm border px-double py-1 text-sm" aria-label={`${onceLabel}: ${parsed.capabilityId ?? parsed.diffSummary}`} onClick={() => onDecision(approval.approvalId, "once")}>
          {onceLabel}
        </button>
        <button type="button" id={`framework.approvals.session.${approval.approvalId}`} className="rounded-sm border px-double py-1 text-sm font-medium" aria-label={`${sessionLabel}: ${parsed.capabilityId ?? parsed.diffSummary}`} onClick={() => onDecision(approval.approvalId, "session")}>
          {sessionLabel}
        </button>
      </div>
    </li>
  );
}

/** ✅️ Self-contained dialog: pops open whenever `approvals` is non-empty (and pops back open on a
 * newly arrived approval even if the human dismissed a prior, now-empty state), listing every
 * pending request with its capability/diff/risk and three decision buttons wired straight to
 * `onDecision` (the caller passes `useAgentBridge().resolveApproval`). */
export function AgentApprovals({ approvals, onDecision }: AgentApprovalsProps): ReactElement {
  const [dismissed, setDismissed] = useState(false);
  useEffect(() => {
    if (approvals.length > 0) setDismissed(false);
  }, [approvals.length]);

  const titleLabel = useLabel(agentUiLabel("os.agent.approvals.title"));
  const descriptionLabel = useLabel(agentUiLabel("os.agent.approvals.description"));
  const emptyLabel = useLabel(agentUiLabel("os.agent.approvals.empty"));

  const open = approvals.length > 0 && !dismissed;

  return (
    <Dialog open={open} onOpenChange={(next) => setDismissed(!next)}>
      <DialogContent showCloseButton className="max-w-lg" aria-label={titleLabel}>
        <DialogHeader>
          <DialogTitle>{titleLabel}</DialogTitle>
          <DialogDescription>{descriptionLabel}</DialogDescription>
        </DialogHeader>
        {approvals.length === 0 ? (
          <p className="py-4 text-sm text-muted-foreground">{emptyLabel}</p>
        ) : (
          <ul className="max-h-96 overflow-y-auto">
            {approvals.map((approval) => (
              <ApprovalRow key={approval.approvalId} approval={approval} onDecision={onDecision} />
            ))}
          </ul>
        )}
        <DialogFooter />
      </DialogContent>
    </Dialog>
  );
}
//#endregion 🔖️AgentApprovals
