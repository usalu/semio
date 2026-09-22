// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/AgentChatPanel/component.tsx
/** @emoji 💬️ `AgentChatPanel` — the OS shell's agent dock. It renders the LIVE MCP conversation:
 * every tool the connected agent invokes (with its real arguments), every result, every approval the
 * gateway parked, every turn the human typed back, and every free-text turn the agent itself
 * published through `conversation_reply` — all of it sourced from `🧵️bridge` frames the
 * `semio-os-mcp` gateway emits from its own `tools/call` dispatch (`AgentToolCall`/`AgentToolResult`)
 * and approval gate (`ApprovalRequested`/`ApprovalResolved`). Nothing on this surface is generated
 * locally: with no bridge attached the transcript is empty and the composer is disabled, which is the
 * honest state, not a simulated one.
 *
 * Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice M2 (audit §4 / P1.5) replaced the
 * previous body — `BasicChatPanel`, a client-side echo with local-only storage — which is deleted.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { type ReactElement, useEffect, useRef, useState } from "react";
import { Button, Textarea, useLabel } from "@semio-tech/ui-react";
import { AgentPresence, type AgentPresenceProps } from "../🚦️AgentPresence/🟦️.tsx";
import { agentUiLabel, type AgentConversationEntry } from "../🔗️AgentBridge/🟦️.tsx";
import { approvalSecondsRemaining, parseApprovalSummary } from "../🤖️AgentApprovals/🟦️.tsx";
import { type ApprovalDecision } from "../../../../🌉️mcp/🧵️bridge/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🔖️AgentChatPanel
export type AgentChatPanelProps = AgentPresenceProps & {
  /** 💬️ The live conversation, oldest first — `useAgentBridge().conversation`. */
  readonly conversation: readonly AgentConversationEntry[];
  /** 📤️ `useAgentBridge().sendAgentMessage`; `false` means no socket was open and nothing was sent. */
  readonly onSendMessage: (text: string) => boolean;
  /** 🛑️ `useAgentBridge().cancelToolCall` — asks the gateway to stop one still-running tool call
   * (audit `📓️g5-ux-completeness-audit.md` ranked item 2: the backend cancel existed, this surface
   * never offered it). Omitted means no cancel path is attached, and no row offers the control
   * rather than offering one that does nothing. */
  readonly onCancelToolCall?: (invocationId: string) => boolean;
  /** ⛩️ `useAgentBridge().resolveApproval` — decides one approval the gateway parked for a
   * destructive capability, inline in the transcript. The `🤖️AgentApprovals` dialog remains the
   * full-detail surface (risk, change summary, capability); this is the same decision offered where
   * the human is already reading, because the gateway BLOCKS on it (ticket 26/09/18 slice M4, audit
   * `📓️g7-mcp-agent-and-collaboration-audit.md` §6 P0.2). Omitted means no decision path is
   * attached, and no row offers a control that would do nothing. */
  readonly onResolveApproval?: (approvalId: string, decision: ApprovalDecision, note?: string) => void;
};

/** 🆔️ A stable DOM id per entry, so a test (and a screen reader's virtual cursor) can address one
 * row without depending on its position in the feed. */
function entryElementId(entry: AgentConversationEntry): string {
  return `framework.chat.entry.${entry.kind}.${entry.id}`;
}

/** ⏱️ Whole seconds left on a parked approval, re-read once a second while it is pending.
 * `null` means the producer named no budget, so nothing is counted rather than a number invented.
 * The ticker only runs while there IS something to count: a resolved row, and a summary with no
 * `timeoutMs`, both schedule nothing. */
function useApprovalCountdown(timeoutMs: number | null, requestedAtMs: number, pending: boolean): number | null {
  const [nowMs, setNowMs] = useState(() => Date.now());
  useEffect(() => {
    if (!pending || timeoutMs === null) return;
    const ticker = window.setInterval(() => setNowMs(Date.now()), 1000);
    return () => window.clearInterval(ticker);
  }, [pending, timeoutMs]);
  if (!pending) return null;
  return approvalSecondsRemaining(timeoutMs, requestedAtMs, nowMs);
}

/** 💬️ One conversation row. Split out so the feed's own markup stays readable and so every row gets
 * the identical role/labelling treatment. */
function AgentChatEntry({
  entry,
  onCancelToolCall,
  onResolveApproval,
}: {
  readonly entry: AgentConversationEntry;
  readonly onCancelToolCall?: (invocationId: string) => boolean;
  readonly onResolveApproval?: (approvalId: string, decision: ApprovalDecision, note?: string) => void;
}): ReactElement {
  // 🏷️ Every role noun is resolved unconditionally (hooks may not run behind a branch) and the one
  // this entry needs is picked afterwards — a translated noun, never the wire's own identifier.
  const youLabel = useLabel(agentUiLabel("os.agent.chat.youRole"));
  const toolCallLabel = useLabel(agentUiLabel("os.agent.chat.toolCallRole"));
  const approvalLabel = useLabel(agentUiLabel("os.agent.chat.approvalRole"));
  const agentLabel = useLabel(agentUiLabel("os.agent.chat.agentRole"));
  const roleLabel = entry.kind === "userMessage" ? youLabel : entry.kind === "toolCall" ? toolCallLabel : entry.kind === "agentMessage" ? agentLabel : approvalLabel;
  const runningLabel = useLabel(agentUiLabel("os.agent.chat.running"));
  const failedLabel = useLabel(agentUiLabel("os.agent.chat.failed"));
  const succeededLabel = useLabel(agentUiLabel("os.agent.chat.succeeded"));
  const approvalPendingLabel = useLabel(agentUiLabel("os.agent.chat.approvalPending"));
  const resultRoleLabel = useLabel(agentUiLabel("os.agent.chat.toolResultRole"));
  const cancellingLabel = useLabel(agentUiLabel("os.agent.chat.cancelling"));
  const cancelLabel = useLabel(agentUiLabel("os.agent.chat.cancel"));
  const cancelToolLabel = useLabel(agentUiLabel("os.agent.chat.cancelToolCall"), { tool: entry.kind === "toolCall" ? entry.toolName : "" });
  const approvalActionsLabel = useLabel(agentUiLabel("os.agent.chat.approvalActionsLabel"));
  const denyLabel = useLabel(agentUiLabel("os.agent.approvals.decisionDeny"));
  const approveOnceLabel = useLabel(agentUiLabel("os.agent.approvals.decisionOnce"));
  const approveSessionLabel = useLabel(agentUiLabel("os.agent.approvals.decisionSession"));
  const approvalVerbLabel = useLabel(agentUiLabel("os.agent.chat.approvalVerb"));
  const approvalTargetLabel = useLabel(agentUiLabel("os.agent.chat.approvalTarget"));
  const requestedByLabel = useLabel(agentUiLabel("os.agent.approvals.requestedBy"));
  const approvalExpiredLabel = useLabel(agentUiLabel("os.agent.chat.approvalExpired"));
  const replyStreamingLabel = useLabel(agentUiLabel("os.agent.chat.replyStreaming"));
  const replyToLabel = useLabel(agentUiLabel("os.agent.chat.replyTo"));
  // 🧾️ The parked approval, read out of the ONE wire string the gateway sends (`🛡️policy`'s
  // `ApprovalRequest::shell_summary`). Parsed unconditionally — hooks may not run behind a branch —
  // and only rendered on an approval row.
  const approval = parseApprovalSummary(entry.kind === "approval" ? entry.summary : "");
  const secondsLeft = useApprovalCountdown(entry.kind === "approval" ? approval.timeoutMs : null, entry.kind === "approval" ? entry.atMs : 0, entry.kind === "approval" && entry.state === "pending");
  const countdownLabel = useLabel(agentUiLabel("os.agent.chat.approvalCountdown"), { seconds: String(secondsLeft ?? 0) });

  const toolState = entry.kind === "toolCall" ? (entry.state === "running" ? runningLabel : entry.state === "cancelling" ? cancellingLabel : entry.state === "failed" ? failedLabel : succeededLabel) : "";
  const state = entry.kind === "toolCall" ? toolState : entry.kind === "approval" ? (entry.state === "pending" ? approvalPendingLabel : (entry.decision ?? "")) : entry.kind === "agentMessage" && entry.state === "streaming" ? replyStreamingLabel : "";
  // 🛑️ Only a call still reported as running can be cancelled: a `cancelling` row already sent its
  // frame, and a settled one has nothing left to stop.
  const cancellable = entry.kind === "toolCall" && entry.state === "running" && onCancelToolCall !== undefined;
  // ⛩️ Only a still-pending approval can be decided: a resolved row already carries its decision,
  // and with no decision path attached no control is offered at all.
  const decidable = entry.kind === "approval" && entry.state === "pending" && onResolveApproval !== undefined;

  return (
    <li id={entryElementId(entry)} data-semio-agent-chat-entry={entry.kind} data-agent-chat-state={entry.kind === "toolCall" ? entry.state : entry.kind === "approval" ? entry.state : entry.kind === "agentMessage" ? entry.state : "sent"} className="flex min-w-0 flex-col gap-single py-single">
      <div className="flex min-w-0 items-baseline justify-between gap-single">
        <span className="text-2xs font-semibold uppercase tracking-wide text-muted-foreground">{roleLabel}</span>
        <span className="flex items-center gap-single">
          {state ? <span className="text-2xs text-muted-foreground">{state}</span> : null}
          {cancellable && entry.kind === "toolCall" ? (
            <Button
              type="button"
              variant="ghost"
              icon="square"
              id={`framework.chat.cancel.${entry.id}`}
              data-semio-agent-chat-cancel={entry.id}
              aria-label={cancelToolLabel}
              title={cancelToolLabel}
              text={cancelLabel}
              onClick={() => onCancelToolCall?.(entry.id)}
            />
          ) : null}
        </span>
      </div>
      {entry.kind === "userMessage" ? <p className="whitespace-pre-wrap break-words text-xs text-foreground">{entry.text}</p> : null}
      {entry.kind === "agentMessage" ? (
        // 💬️ The agent's own prose. A live region because a streaming turn grows without the human
        // doing anything — announced politely, never interrupting — and `aria-busy` says the turn is
        // not finished yet, which is the same fact the visible state word carries.
        <p
          role="status"
          aria-live="polite"
          aria-busy={entry.state === "streaming"}
          data-semio-agent-chat-reply={entry.id}
          data-semio-agent-chat-reply-to={entry.inReplyTo ?? ""}
          title={entry.inReplyTo ? replyToLabel : undefined}
          className="whitespace-pre-wrap break-words text-xs text-foreground"
        >
          {entry.text}
        </p>
      ) : null}
      {entry.kind === "approval" ? (
        // 🧾️ WHO asked, WHAT it does, WHAT it touches — the three facts a human needs to decide,
        // each omitted rather than blanked when the producer did not send it. A row missing every
        // rich field still shows its `diffSummary`, which for a plain-text producer is the whole
        // wire string, so this surface never goes empty.
        <div className="flex min-w-0 flex-col gap-single text-xs text-foreground">
          {approval.capabilityTitle ? (
            <p className="min-w-0 break-words" data-semio-agent-chat-approval-verb={approval.capabilityId ?? ""}>
              <span className="text-2xs font-semibold uppercase tracking-wide text-muted-foreground">{approvalVerbLabel}: </span>
              <span className="font-medium">{approval.capabilityTitle}</span>
            </p>
          ) : null}
          {approval.description ? <p className="min-w-0 whitespace-pre-wrap break-words text-muted-foreground">{approval.description}</p> : null}
          {approval.artifactKind ? (
            <p className="min-w-0 break-words text-muted-foreground" data-semio-agent-chat-approval-target={approval.artifactKind}>
              <span className="text-2xs font-semibold uppercase tracking-wide">{approvalTargetLabel}: </span>
              {approval.artifactKind}
            </p>
          ) : null}
          <p className="min-w-0 whitespace-pre-wrap break-words">{approval.diffSummary}</p>
          {approval.requestedBy ? (
            <p className="min-w-0 break-words text-2xs text-muted-foreground">
              {requestedByLabel}: {approval.requestedBy}
            </p>
          ) : null}
          {secondsLeft !== null ? (
            // ⏳️ A live region, because the number changes without the human doing anything: a
            // screen reader is told politely, never interrupted mid-sentence.
            <p role="status" aria-live="polite" data-semio-agent-chat-approval-countdown={String(secondsLeft)} className="min-w-0 break-words text-2xs text-muted-foreground">
              {secondsLeft > 0 ? countdownLabel : approvalExpiredLabel}
            </p>
          ) : null}
        </div>
      ) : null}
      {decidable ? (
        <div role="group" aria-label={approvalActionsLabel} data-semio-agent-chat-approval={entry.id} className="flex flex-wrap items-center gap-single">
          <Button type="button" variant="ghost" icon="x" id={`framework.chat.approval.deny.${entry.id}`} text={denyLabel} aria-label={denyLabel} onClick={() => onResolveApproval?.(entry.id, "deny")} />
          <Button type="button" icon="check" id={`framework.chat.approval.once.${entry.id}`} text={approveOnceLabel} aria-label={approveOnceLabel} onClick={() => onResolveApproval?.(entry.id, "once")} />
          <Button type="button" variant="ghost" icon="check" id={`framework.chat.approval.session.${entry.id}`} text={approveSessionLabel} aria-label={approveSessionLabel} onClick={() => onResolveApproval?.(entry.id, "session")} />
        </div>
      ) : null}
      {entry.kind === "toolCall" ? (
        <>
          <p className="break-words font-mono text-xs text-foreground">{entry.toolName}</p>
          {entry.args ? <pre className="overflow-x-auto whitespace-pre-wrap break-words rounded-sm bg-muted p-single font-mono text-2xs text-muted-foreground">{entry.args}</pre> : null}
          {entry.summary !== null ? (
            <p className="whitespace-pre-wrap break-words text-xs text-foreground">
              <span className="text-2xs font-semibold uppercase tracking-wide text-muted-foreground">{resultRoleLabel}: </span>
              {entry.summary}
            </p>
          ) : null}
        </>
      ) : null}
    </li>
  );
}

/** 💬️ Chat side panel with MCP agent presence in the header, the live agent conversation as its body,
 * and a composer that sends one human turn per submit to the connected agent. */
export function AgentChatPanel({ status, presence, conversation, onSendMessage, onCancelToolCall, onResolveApproval }: AgentChatPanelProps): ReactElement {
  const title = useLabel(agentUiLabel("os.agent.chat.panelTitle"));
  const transcriptLabel = useLabel(agentUiLabel("os.agent.chat.transcriptLabel"));
  const emptyLabel = useLabel(agentUiLabel("os.agent.chat.empty"));
  const draftLabel = useLabel(agentUiLabel("os.agent.chat.draftLabel"));
  const placeholderLabel = useLabel(agentUiLabel("os.agent.chat.placeholder"));
  const sendLabel = useLabel(agentUiLabel("os.agent.chat.send"));
  const disconnectedLabel = useLabel(agentUiLabel("os.agent.chat.disconnected"));

  const [draft, setDraft] = useState("");
  const feedRef = useRef<HTMLOListElement | null>(null);
  const connected = status === "open";

  useEffect(() => {
    const feed = feedRef.current;
    if (feed) feed.scrollTop = feed.scrollHeight;
  }, [conversation.length]);

  const submit = (): void => {
    const trimmed = draft.trim();
    if (!trimmed || !connected) return;
    if (onSendMessage(trimmed)) setDraft("");
  };

  return (
    <div data-semio-agent-chat-panel="" className="flex w-full min-w-0 flex-col gap-single">
      <div className="flex shrink-0 items-center justify-between gap-double border-b border-border px-single py-single">
        <span className="text-sm font-medium">{title}</span>
        <AgentPresence status={status} presence={presence} />
      </div>
      <ol ref={feedRef} id="framework.chat.feed" data-semio-agent-chat-feed="" aria-label={transcriptLabel} aria-live="polite" className="min-h-huge flex min-w-0 flex-col divide-y divide-border overflow-y-auto px-single">
        {conversation.length === 0 ? <li className="py-single text-xs text-muted-foreground">{emptyLabel}</li> : conversation.map((entry) => <AgentChatEntry key={`${entry.kind}:${entry.id}`} entry={entry} onCancelToolCall={onCancelToolCall} onResolveApproval={onResolveApproval} />)}
      </ol>
      <div className="flex shrink-0 flex-col gap-single px-single pb-single">
        {connected ? null : <p className="text-2xs text-muted-foreground">{disconnectedLabel}</p>}
        <Textarea
          id="framework.chat.draft"
          aria-label={draftLabel}
          value={draft}
          disabled={!connected}
          onChange={(event) => setDraft(event.target.value)}
          onKeyDown={(event) => {
            if (event.key !== "Enter" || event.shiftKey) return;
            event.preventDefault();
            submit();
          }}
          rows={3}
          placeholder={placeholderLabel}
        />
        <div className="flex items-center justify-end gap-single">
          <Button type="button" id="framework.chat.send" text={sendLabel} icon="arrow-right" onClick={submit} disabled={!connected || !draft.trim()} />
        </div>
      </div>
    </div>
  );
}
//#endregion 🔖️AgentChatPanel
