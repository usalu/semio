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
import { AgentApprovalAffordance } from "../🤖️AgentApprovals/🟦️.tsx";
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
   * destructive capability. The transcript row IS the approval's one affordance
   * (`🤖️AgentApprovals`' `AgentApprovalAffordance`: verb, target, change summary, risk, countdown and the
   * three decisions); no modal copy exists beside it (ticket 26/09/18 session 11 slice U5). Omitted means
   * no decision path is attached, and no row offers a control that would do nothing. */
  readonly onResolveApproval?: (approvalId: string, decision: ApprovalDecision, note?: string) => void;
};

/** 🆔️ A stable DOM id per entry, so a test (and a screen reader's virtual cursor) can address one
 * row without depending on its position in the feed. */
function entryElementId(entry: AgentConversationEntry): string {
  return `framework.chat.entry.${entry.kind}.${entry.id}`;
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
  const resultRoleLabel = useLabel(agentUiLabel("os.agent.chat.toolResultRole"));
  const cancellingLabel = useLabel(agentUiLabel("os.agent.chat.cancelling"));
  const cancelLabel = useLabel(agentUiLabel("os.agent.chat.cancel"));
  const cancelToolLabel = useLabel(agentUiLabel("os.agent.chat.cancelToolCall"), { tool: entry.kind === "toolCall" ? entry.toolName : "" });
  const replyStreamingLabel = useLabel(agentUiLabel("os.agent.chat.replyStreaming"));
  const replyToLabel = useLabel(agentUiLabel("os.agent.chat.replyTo"));
  const toolState = entry.kind === "toolCall" ? (entry.state === "running" ? runningLabel : entry.state === "cancelling" ? cancellingLabel : entry.state === "failed" ? failedLabel : succeededLabel) : "";
  const state = entry.kind === "toolCall" ? toolState : entry.kind === "agentMessage" && entry.state === "streaming" ? replyStreamingLabel : "";
  // 🛑️ Only a call still reported as running can be cancelled: a `cancelling` row already sent its
  // frame, and a settled one has nothing left to stop.
  const cancellable = entry.kind === "toolCall" && entry.state === "running" && onCancelToolCall !== undefined;

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
              data-semio-agent-chat-tool={entry.toolName}
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
      {entry.kind === "approval" ? <AgentApprovalAffordance approvalId={entry.id} summary={entry.summary} requestedAtMs={entry.atMs} state={entry.state} decision={entry.decision} withdrawal={entry.withdrawal} onDecision={onResolveApproval} /> : null}
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
