// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/AgentChatPanel/component.tsx
/** @emoji 💬️ `AgentChatPanel` — OS shell chat dock: {@link BasicChatPanel} plus {@link AgentPresence}
 * for the MCP ShellBridge session (no separate footer chrome). */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { type ReactElement } from "react";
import { BasicChatPanel, useLabel } from "@semio-tech/ui-react";
import { AgentPresence, type AgentPresenceProps } from "../🚦️AgentPresence/🟦️.tsx";
import { agentUiLabel } from "../🔗️AgentBridge/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️AgentChatPanel
export type AgentChatPanelProps = AgentPresenceProps;

/** 💬️ Chat side panel with MCP agent presence in the header (replaces ambient footer presence). */
export function AgentChatPanel({ status, presence }: AgentChatPanelProps): ReactElement {
  const title = useLabel(agentUiLabel("os.agent.chat.panelTitle"));
  return (
    <div data-semio-agent-chat-panel="" className="flex h-full min-h-0 flex-col gap-single">
      <div className="flex shrink-0 items-center justify-between gap-double border-b border-border px-single py-single">
        <span className="text-sm font-medium">{title}</span>
        <AgentPresence status={status} presence={presence} />
      </div>
      <div className="flex h-full min-h-0 flex-1 flex-col">
        <BasicChatPanel id="framework.chat" title={title} />
      </div>
    </div>
  );
}
//#endregion 🔖️AgentChatPanel
