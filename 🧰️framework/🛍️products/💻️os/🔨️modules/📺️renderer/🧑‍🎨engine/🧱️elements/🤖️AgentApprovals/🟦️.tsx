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
  readonly diffSummary: string;
  readonly risk: ApprovalRisk | null;
  readonly requestedBy: string | null;
};

/** 🔍️ The bridge's `ApprovalRequested.summary` wire field is a single string
 * (`🌉️mcp/🧵️bridge/🦀️.rs` tag 3 — `📓️terra-P10-report.md` documents this as a real gap
 * between the required "capability + diff + risk" richness and today's shipped frame shape, not
 * something this element's own `path_scope` may fix by editing that Rust facet). A future
 * approval-producing packet can JSON-encode `{capabilityId, diffSummary, risk, requestedBy}` into
 * that same string field with zero wire-format change; this parses that shape when present and
 * falls back to treating the whole string as the diff summary otherwise, so the dialog never goes
 * blank for a plain-text producer. */
export function parseApprovalSummary(summary: string): ParsedApprovalSummary {
  try {
    const value = JSON.parse(summary) as Record<string, unknown>;
    if (value && typeof value === "object" && !Array.isArray(value)) {
      const capabilityId = typeof value.capabilityId === "string" ? value.capabilityId : null;
      const risk = value.risk === "low" || value.risk === "medium" || value.risk === "high" ? (value.risk as ApprovalRisk) : null;
      const requestedBy = typeof value.requestedBy === "string" ? value.requestedBy : null;
      const diffSummary = typeof value.diffSummary === "string" ? value.diffSummary : summary;
      if (capabilityId !== null || risk !== null || requestedBy !== null) return { capabilityId, diffSummary, risk, requestedBy };
    }
  } catch {
    // not JSON — plain-text summary, handled by the fallback below
  }
  return { capabilityId: null, diffSummary: summary, risk: null, requestedBy: null };
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
  const parsed = parseApprovalSummary(approval.summary);

  return (
    <li className="space-y-2 border-b py-3 last:border-b-0" data-semio-agent-approval-id={approval.approvalId}>
      <div className="space-y-1 text-sm">
        {parsed.capabilityId ? (
          <p>
            <span className="text-muted-foreground">{capabilityLabel}: </span>
            <span className="font-medium">{parsed.capabilityId}</span>
          </p>
        ) : null}
        <p>
          <span className="text-muted-foreground">{diffLabel}: </span>
          {parsed.diffSummary}
        </p>
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
      <div className="flex gap-2">
        <button type="button" className="rounded-sm border px-double py-1 text-sm text-red-400" aria-label={`${denyLabel}: ${parsed.capabilityId ?? parsed.diffSummary}`} onClick={() => onDecision(approval.approvalId, "deny")}>
          {denyLabel}
        </button>
        <button type="button" className="rounded-sm border px-double py-1 text-sm" aria-label={`${onceLabel}: ${parsed.capabilityId ?? parsed.diffSummary}`} onClick={() => onDecision(approval.approvalId, "once")}>
          {onceLabel}
        </button>
        <button type="button" className="rounded-sm border px-double py-1 text-sm font-medium" aria-label={`${sessionLabel}: ${parsed.capabilityId ?? parsed.diffSummary}`} onClick={() => onDecision(approval.approvalId, "session")}>
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
