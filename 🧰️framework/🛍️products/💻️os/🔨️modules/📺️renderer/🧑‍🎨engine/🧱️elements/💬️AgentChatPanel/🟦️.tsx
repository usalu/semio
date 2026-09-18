// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/AgentChatPanel/component.tsx
/** @emoji 💬️ `AgentChatPanel` — the OS shell's agent dock. It renders the LIVE MCP conversation:
 * every tool the connected agent invokes (with its real arguments), every result, every approval the
 * gateway parked, and every turn the human typed back — all of it sourced from `🧵️bridge` frames the
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
// #endregion 🔌️Adapters

//#region 🔖️AgentChatPanel
export type AgentChatPanelProps = AgentPresenceProps & {
  /** 💬️ The live conversation, oldest first — `useAgentBridge().conversation`. */
  readonly conversation: readonly AgentConversationEntry[];
  /** 📤️ `useAgentBridge().sendAgentMessage`; `false` means no socket was open and nothing was sent. */
  readonly onSendMessage: (text: string) => boolean;
};

/** 🆔️ A stable DOM id per entry, so a test (and a screen reader's virtual cursor) can address one
 * row without depending on its position in the feed. */
function entryElementId(entry: AgentConversationEntry): string {
  return `framework.chat.entry.${entry.kind}.${entry.id}`;
}

/** 💬️ One conversation row. Split out so the feed's own markup stays readable and so every row gets
 * the identical role/labelling treatment. */
function AgentChatEntry({ entry }: { readonly entry: AgentConversationEntry }): ReactElement {
  // 🏷️ Every role noun is resolved unconditionally (hooks may not run behind a branch) and the one
  // this entry needs is picked afterwards — a translated noun, never the wire's own identifier.
  const youLabel = useLabel(agentUiLabel("os.agent.chat.youRole"));
  const toolCallLabel = useLabel(agentUiLabel("os.agent.chat.toolCallRole"));
  const approvalLabel = useLabel(agentUiLabel("os.agent.chat.approvalRole"));
  const roleLabel = entry.kind === "userMessage" ? youLabel : entry.kind === "toolCall" ? toolCallLabel : approvalLabel;
  const runningLabel = useLabel(agentUiLabel("os.agent.chat.running"));
  const failedLabel = useLabel(agentUiLabel("os.agent.chat.failed"));
  const succeededLabel = useLabel(agentUiLabel("os.agent.chat.succeeded"));
  const approvalPendingLabel = useLabel(agentUiLabel("os.agent.chat.approvalPending"));
  const resultRoleLabel = useLabel(agentUiLabel("os.agent.chat.toolResultRole"));

  const state =
    entry.kind === "toolCall" ? (entry.state === "running" ? runningLabel : entry.state === "failed" ? failedLabel : succeededLabel) : entry.kind === "approval" ? (entry.state === "pending" ? approvalPendingLabel : (entry.decision ?? "")) : "";

  return (
    <li id={entryElementId(entry)} data-semio-agent-chat-entry={entry.kind} data-agent-chat-state={entry.kind === "toolCall" ? entry.state : entry.kind === "approval" ? entry.state : "sent"} className="flex min-w-0 flex-col gap-single py-single">
      <div className="flex min-w-0 items-baseline justify-between gap-single">
        <span className="text-2xs font-semibold uppercase tracking-wide text-muted-foreground">{roleLabel}</span>
        {state ? <span className="text-2xs text-muted-foreground">{state}</span> : null}
      </div>
      {entry.kind === "userMessage" ? <p className="whitespace-pre-wrap break-words text-xs text-foreground">{entry.text}</p> : null}
      {entry.kind === "approval" ? <p className="whitespace-pre-wrap break-words text-xs text-foreground">{entry.summary}</p> : null}
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
export function AgentChatPanel({ status, presence, conversation, onSendMessage }: AgentChatPanelProps): ReactElement {
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
        {conversation.length === 0 ? <li className="py-single text-xs text-muted-foreground">{emptyLabel}</li> : conversation.map((entry) => <AgentChatEntry key={`${entry.kind}:${entry.id}`} entry={entry} />)}
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
