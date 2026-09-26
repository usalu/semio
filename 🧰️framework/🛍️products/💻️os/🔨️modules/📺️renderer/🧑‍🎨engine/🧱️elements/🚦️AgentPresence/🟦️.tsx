// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/AgentPresence/component.tsx
/** @emoji 🟢️ `AgentPresence` — small status indicator for the connected agent: connected/working/
 * idle/disconnected plus the current invocation label, driven by `AgentBridge`'s `agentPresence`
 * frames. Ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` packet P10.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { type ReactElement } from "react";
import { useLabel } from "@semio-tech/ui-react";
import { agentUiLabel, type AgentBridgePresence, type AgentBridgeStatus, type AgentBridgeVersionMismatch } from "../🔗️AgentBridge/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️AgentPresence
export type AgentPresenceProps = {
  readonly status: AgentBridgeStatus;
  readonly presence: AgentBridgePresence;
  /** 🤝️ `useAgentBridge().versionMismatch` — names both versions while `status` is `incompatible`. */
  readonly versionMismatch?: AgentBridgeVersionMismatch | null;
};

/** 🚦️ Dot color + status-label key for the current bridge status/presence combination — pure so
 * it is directly unit-testable without rendering. */
export function agentPresenceTone(status: AgentBridgeStatus, presence: AgentBridgePresence): "connected" | "working" | "connecting" | "disconnected" | "blocked" {
  if (status === "unavailable" || status === "incompatible") return "blocked";
  if (status === "disabled" || status === "closed") return "disconnected";
  if (status === "connecting" || status === "reconnecting") return "connecting";
  if (presence.active) return "working";
  return "connected";
}

export function AgentPresence({ status, presence, versionMismatch = null }: AgentPresenceProps): ReactElement {
  const connectingLabel = useLabel(agentUiLabel("os.agent.presence.connecting"));
  const reconnectingLabel = useLabel(agentUiLabel("os.agent.presence.reconnecting"));
  const disconnectedLabel = useLabel(agentUiLabel("os.agent.presence.disconnected"));
  const idleLabel = useLabel(agentUiLabel("os.agent.presence.idle"));
  const workingLabel = useLabel(agentUiLabel("os.agent.presence.working"), { label: presence.label });
  const unavailableLabel = useLabel(agentUiLabel("os.agent.presence.unavailable"));
  const incompatibleLabel = useLabel(agentUiLabel("os.agent.presence.incompatible"), { gateway: String(versionMismatch?.gateway ?? "?"), shell: String(versionMismatch?.shell ?? "?") });
  const accessibleName = useLabel(agentUiLabel("os.agent.presence.statusLabel"));

  const tone = agentPresenceTone(status, presence);
  const text = status === "unavailable" ? unavailableLabel : status === "incompatible" ? incompatibleLabel : status === "reconnecting" ? reconnectingLabel : status === "connecting" ? connectingLabel : tone === "disconnected" ? disconnectedLabel : tone === "working" ? workingLabel : idleLabel;

  return (
    <div role="status" aria-label={accessibleName} data-semio-agent-presence-tone={tone} data-semio-agent-bridge-status={status} className="flex items-center gap-1.5 text-xs">
      <span aria-hidden="true" className={`inline-block h-2 w-2 rounded-full ${tone === "working" ? "bg-emerald-400" : tone === "connected" ? "bg-sky-400" : tone === "connecting" ? "bg-amber-400" : tone === "blocked" ? "bg-destructive" : "bg-muted-foreground"}`} />
      <span className="text-muted-foreground">{text}</span>
    </div>
  );
}
//#endregion 🔖️AgentPresence
