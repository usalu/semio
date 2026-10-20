import { registerUiTranslationBundles } from "@semio-tech/ui-react";
import { defaultShellState, type ReduceResult, type ShellCommand, type ShellState } from "@semio-tech/framework/shell";
import { type ApprovalDecision } from "@semio-tech/framework/mcp/bridge";
import { useDiscoveredState } from "./🔶️discovered/🟦️.tsx";
import { usePart2 } from "./🔸️part2/🟦️.tsx";
import { usePart3 } from "./🟠️part3/🟦️.tsx";
import type { AgentBridgePresence, AgentBridgeStatus, AgentBridgeVersionMismatch, AgentConversationEntry, PendingAgentApproval, UseAgentBridgeOptions } from "./🟢️shared/🟦️.tsx";

export { AGENT_BRIDGE_OFFER_ENDPOINT, AGENT_BRIDGE_OFFER_SCHEMA_V1, AGENT_BRIDGE_RENDEZVOUS_SCHEMA_VERSION, agentBridgeOfferAnswerV1, agentBridgeOfferPathV1, agentBridgeOfferScopeFromDelegationsV1, parseAgentBridgeOfferRecordV2, parseAgentBridgeOfferScopeV1, selectAgentBridgeOfferV1, type AgentBridgeOfferRecordV2, type AgentBridgeOfferScopeKindV2, type AgentBridgeOfferScopeV1, BRIDGE_DISCOVERY_MAX_INTERVAL_MS, BRIDGE_DISCOVERY_MIN_INTERVAL_MS, bridgeProtocols, fetchAgentBridgeConfig, isAdmissibleBridgeUrl, nextBridgeDiscoveryIntervalMs, parseAgentBridgeOffer, sameAgentBridgeOffer, type AgentBridgeConfig, type AgentBridgeOfferAnswerV1, type BridgeOfferFetch } from "./🛰️offer/🟦️.ts";

// #endregion 🔌️Adapters

//#region 🌐️Labels
export const agentUiLabel = registerUiTranslationBundles({
  en: {
    translation: {
      os: {
        agent: {
          presence: {
            connected: { label: { normal: "Agent connected", beginner: "Agent connected" } },
            connecting: { label: { normal: "Connecting to agent…", beginner: "Connecting to agent…" } },
            reconnecting: { label: { normal: "Reconnecting to agent…", beginner: "Reconnecting to agent…" } },
            disconnected: { label: { normal: "Agent disconnected", beginner: "Agent disconnected" } },
            unavailable: { label: { normal: "The AI client's bridge does not answer; restart the AI client to connect again", beginner: "Your AI client does not answer. Restart it to connect again." } },
            incompatible: { label: { normal: "The AI client uses bridge version {{gateway}}, this shell version {{shell}}; update the older one", beginner: "Your AI client and this app are different versions. Update the older one." } },
            working: { label: { normal: "Agent working: {{label}}", beginner: "Agent working: {{label}}" } },
            idle: { label: { normal: "Agent idle", beginner: "Agent idle" } },
            statusLabel: { label: { normal: "Agent status", beginner: "Agent status" } },
          },
          chat: {
            panelTitle: { label: { normal: "Chat", beginner: "Chat" } },
            transcriptLabel: { label: { normal: "Agent conversation", beginner: "Agent conversation" } },
            empty: { label: { normal: "No agent activity yet. Messages you send appear here, along with the agent's replies and every tool it runs.", beginner: "No agent activity yet. Messages you send appear here, along with the agent's replies and every tool it runs." } },
            youRole: { label: { normal: "You", beginner: "You" } },
            toolCallRole: { label: { normal: "Tool call", beginner: "Tool call" } },
            toolResultRole: { label: { normal: "Result", beginner: "Result" } },
            approvalRole: { label: { normal: "Approval", beginner: "Approval" } },
            running: { label: { normal: "Running…", beginner: "Running…" } },
            failed: { label: { normal: "Failed", beginner: "Failed" } },
            succeeded: { label: { normal: "Done", beginner: "Done" } },
            approvalPending: { label: { normal: "Waiting for your decision", beginner: "Waiting for your decision" } },
            approvalActionsLabel: { label: { normal: "Decide this approval", beginner: "Decide this approval" } },
            approvalCountdown: { label: { normal: "{{seconds}}s left to decide", beginner: "{{seconds}}s left to decide" } },
            approvalExpired: { label: { normal: "Out of time — the agent was refused", beginner: "Out of time — the agent was refused" } },
            approvalVerb: { label: { normal: "Action", beginner: "Action" } },
            approvalTarget: { label: { normal: "Applies to", beginner: "Applies to" } },
            draftLabel: { label: { normal: "Message to the agent", beginner: "Message to the agent" } },
            placeholder: { label: { normal: "Tell the agent what to do…", beginner: "Tell the agent what to do…" } },
            send: { label: { normal: "Send", beginner: "Send" } },
            disconnected: { label: { normal: "Not connected to an agent — messages cannot be sent.", beginner: "Not connected to an agent — messages cannot be sent." } },
            cancel: { label: { normal: "Cancel", beginner: "Stop" } },
            cancelToolCall: { label: { normal: "Cancel {{tool}}", beginner: "Stop {{tool}}" } },
            cancelling: { label: { normal: "Cancelling…", beginner: "Stopping…" } },
            agentRole: { label: { normal: "Agent", beginner: "Agent" } },
            replyStreaming: { label: { normal: "Still writing…", beginner: "Still writing…" } },
            replyTo: { label: { normal: "Answering your message", beginner: "Answering your message" } },
          },
          approvals: {
            title: { label: { normal: "Agent Approvals", beginner: "Agent Approvals" } },
            capability: { label: { normal: "Capability", beginner: "Capability" } },
            diffSummary: { label: { normal: "Change summary", beginner: "Change summary" } },
            requestedBy: { label: { normal: "Requested by", beginner: "Requested by" } },
            riskLabel: { label: { normal: "Risk", beginner: "Risk" } },
            riskLow: { label: { normal: "Low", beginner: "Low" } },
            riskMedium: { label: { normal: "Medium", beginner: "Medium" } },
            riskHigh: { label: { normal: "High", beginner: "High" } },
            decisionDeny: { label: { normal: "Deny", beginner: "Deny" } },
            decisionOnce: { label: { normal: "Approve Once", beginner: "Approve Once" } },
            decisionSession: { label: { normal: "Approve for Session", beginner: "Approve for Session" } },
            decidedDeny: { label: { normal: "Denied", beginner: "You said no" } },
            decidedOnce: { label: { normal: "Approved once", beginner: "You allowed it once" } },
            decidedSession: { label: { normal: "Approved for this session", beginner: "You allowed it for this session" } },
            withdrawnCancelled: { label: { normal: "Withdrawn — the agent's request was cancelled", beginner: "No longer needed — the request was stopped" } },
            withdrawnTimedOut: { label: { normal: "Withdrawn — nobody decided in time", beginner: "No longer needed — the time ran out" } },
            withdrawnSuperseded: { label: { normal: "Withdrawn — moved to your newer window", beginner: "Moved to your newer window" } },
            waitingOne: { label: { normal: "1 agent approval is waiting", beginner: "The agent is waiting for your OK" } },
            waitingMany: { label: { normal: "{{count}} agent approvals are waiting", beginner: "The agent is waiting for {{count}} OKs" } },
            review: { label: { normal: "Review", beginner: "Look at it" } },
            requested: { label: { normal: "Approval requested: {{title}}", beginner: "The agent asks for your OK: {{title}}" } },
          },
        },
      },
    },
  },
  de: {
    translation: {
      os: {
        agent: {
          presence: {
            connected: { label: { normal: "Agent verbunden", beginner: "Agent verbunden" } },
            connecting: { label: { normal: "Verbinde mit Agent…", beginner: "Verbinde mit Agent…" } },
            reconnecting: { label: { normal: "Verbindung zum Agent wird wiederhergestellt…", beginner: "Verbindung zum Agent wird wiederhergestellt…" } },
            disconnected: { label: { normal: "Agent getrennt", beginner: "Agent getrennt" } },
            unavailable: { label: { normal: "Die Brücke des KI-Clients antwortet nicht; starte den KI-Client neu, um erneut zu verbinden", beginner: "Dein KI-Client antwortet nicht. Starte ihn neu, um erneut zu verbinden." } },
            incompatible: { label: { normal: "Der KI-Client nutzt Brückenversion {{gateway}}, diese Oberfläche Version {{shell}}; aktualisiere die ältere", beginner: "Dein KI-Client und diese App haben verschiedene Versionen. Aktualisiere die ältere." } },
            working: { label: { normal: "Agent aktiv: {{label}}", beginner: "Agent aktiv: {{label}}" } },
            idle: { label: { normal: "Agent inaktiv", beginner: "Agent inaktiv" } },
            statusLabel: { label: { normal: "Agent-Status", beginner: "Agent-Status" } },
          },
          chat: {
            panelTitle: { label: { normal: "Chat", beginner: "Chat" } },
            transcriptLabel: { label: { normal: "Agent-Konversation", beginner: "Agent-Konversation" } },
            empty: { label: { normal: "Noch keine Agent-Aktivität. Gesendete Nachrichten erscheinen hier, ebenso die Antworten des Agents und jedes von ihm ausgeführte Werkzeug.", beginner: "Noch keine Agent-Aktivität. Gesendete Nachrichten erscheinen hier, ebenso die Antworten des Agents und jedes von ihm ausgeführte Werkzeug." } },
            youRole: { label: { normal: "Du", beginner: "Du" } },
            toolCallRole: { label: { normal: "Werkzeugaufruf", beginner: "Werkzeugaufruf" } },
            toolResultRole: { label: { normal: "Ergebnis", beginner: "Ergebnis" } },
            approvalRole: { label: { normal: "Freigabe", beginner: "Freigabe" } },
            running: { label: { normal: "Läuft…", beginner: "Läuft…" } },
            failed: { label: { normal: "Fehlgeschlagen", beginner: "Fehlgeschlagen" } },
            succeeded: { label: { normal: "Fertig", beginner: "Fertig" } },
            approvalPending: { label: { normal: "Wartet auf deine Entscheidung", beginner: "Wartet auf deine Entscheidung" } },
            approvalActionsLabel: { label: { normal: "Diese Freigabe entscheiden", beginner: "Diese Freigabe entscheiden" } },
            approvalCountdown: { label: { normal: "Noch {{seconds}} s für die Entscheidung", beginner: "Noch {{seconds}} s für die Entscheidung" } },
            approvalExpired: { label: { normal: "Zeit abgelaufen — der Agent wurde abgelehnt", beginner: "Zeit abgelaufen — der Agent wurde abgelehnt" } },
            approvalVerb: { label: { normal: "Aktion", beginner: "Aktion" } },
            approvalTarget: { label: { normal: "Betrifft", beginner: "Betrifft" } },
            draftLabel: { label: { normal: "Nachricht an den Agent", beginner: "Nachricht an den Agent" } },
            placeholder: { label: { normal: "Sag dem Agent, was zu tun ist…", beginner: "Sag dem Agent, was zu tun ist…" } },
            send: { label: { normal: "Senden", beginner: "Senden" } },
            disconnected: { label: { normal: "Nicht mit einem Agent verbunden — Nachrichten können nicht gesendet werden.", beginner: "Nicht mit einem Agent verbunden — Nachrichten können nicht gesendet werden." } },
            cancel: { label: { normal: "Abbrechen", beginner: "Stopp" } },
            cancelToolCall: { label: { normal: "{{tool}} abbrechen", beginner: "{{tool}} stoppen" } },
            cancelling: { label: { normal: "Wird abgebrochen…", beginner: "Wird gestoppt…" } },
            agentRole: { label: { normal: "Agent", beginner: "Agent" } },
            replyStreaming: { label: { normal: "Schreibt noch…", beginner: "Schreibt noch…" } },
            replyTo: { label: { normal: "Antwortet auf deine Nachricht", beginner: "Antwortet auf deine Nachricht" } },
          },
          approvals: {
            title: { label: { normal: "Agent-Freigaben", beginner: "Agent-Freigaben" } },
            capability: { label: { normal: "Fähigkeit", beginner: "Fähigkeit" } },
            diffSummary: { label: { normal: "Änderungszusammenfassung", beginner: "Änderungszusammenfassung" } },
            requestedBy: { label: { normal: "Angefragt von", beginner: "Angefragt von" } },
            riskLabel: { label: { normal: "Risiko", beginner: "Risiko" } },
            riskLow: { label: { normal: "Niedrig", beginner: "Niedrig" } },
            riskMedium: { label: { normal: "Mittel", beginner: "Mittel" } },
            riskHigh: { label: { normal: "Hoch", beginner: "Hoch" } },
            decisionDeny: { label: { normal: "Ablehnen", beginner: "Ablehnen" } },
            decisionOnce: { label: { normal: "Einmal genehmigen", beginner: "Einmal genehmigen" } },
            decisionSession: { label: { normal: "Für Sitzung genehmigen", beginner: "Für Sitzung genehmigen" } },
            decidedDeny: { label: { normal: "Abgelehnt", beginner: "Du hast abgelehnt" } },
            decidedOnce: { label: { normal: "Einmal genehmigt", beginner: "Du hast es einmal erlaubt" } },
            decidedSession: { label: { normal: "Für diese Sitzung genehmigt", beginner: "Du hast es für diese Sitzung erlaubt" } },
            withdrawnCancelled: { label: { normal: "Zurückgezogen — die Anfrage des Agenten wurde abgebrochen", beginner: "Nicht mehr nötig — die Anfrage wurde gestoppt" } },
            withdrawnTimedOut: { label: { normal: "Zurückgezogen — niemand hat rechtzeitig entschieden", beginner: "Nicht mehr nötig — die Zeit ist abgelaufen" } },
            withdrawnSuperseded: { label: { normal: "Zurückgezogen — in dein neueres Fenster verschoben", beginner: "In dein neueres Fenster verschoben" } },
            waitingOne: { label: { normal: "1 Agent-Freigabe wartet", beginner: "Der Agent wartet auf dein OK" } },
            waitingMany: { label: { normal: "{{count}} Agent-Freigaben warten", beginner: "Der Agent wartet auf {{count}} OKs" } },
            review: { label: { normal: "Prüfen", beginner: "Ansehen" } },
            requested: { label: { normal: "Freigabe angefragt: {{title}}", beginner: "Der Agent bittet um dein OK: {{title}}" } },
          },
        },
      },
    },
  },
});

//#endregion 🔖️DiscoverConfig

//#region 🔖️DefaultState
/** 🌱️ A fresh, empty `ShellState` — the mirror `AgentBridge` reduces `ShellCommand`s against before
 * a real `ShellHost`-derived snapshot is ever supplied via `initialState`. It is the SSOT twin's own
 * {@link defaultShellState}, re-exported under the name this module's callers already use: the
 * hand-written copy that used to live here drifted nine `ui*` rows wide (they belong to `🐚️Shell`'s
 * renderer preference state, not to the shell SSOT), which is exactly the failure a second spelling
 * of a default always produces. */
export { defaultShellState as createDefaultShellState };

export type UseAgentBridgeResult = {
  readonly status: AgentBridgeStatus;
  readonly shellState: ShellState;
  readonly presence: AgentBridgePresence;
  readonly pendingApprovals: readonly PendingAgentApproval[];
  readonly conversation: readonly AgentConversationEntry[];
  readonly lastError: string | null;
  /** 🤝️ Both bridge versions while `status` is `incompatible`, otherwise `null`. */
  readonly versionMismatch: AgentBridgeVersionMismatch | null;
  readonly dispatch: (command: ShellCommand) => ReduceResult;
  readonly resolveApproval: (approvalId: string, decision: ApprovalDecision, note?: string) => void;
  /** 💬️ Sends one human turn to the connected agent as a `ShellToGateway.agentMessage` frame and
   * echoes it into {@link UseAgentBridgeResult.conversation}. Returns `false` (and records nothing)
   * when no socket is open, so the panel can tell the human their message did not go anywhere
   * instead of showing it as if it had. */
  readonly sendAgentMessage: (text: string) => boolean;
  /** 🛑️ Asks the gateway to cancel the still-running tool call `invocationId`. Returns `false` (and
   * changes nothing) when no socket is open, so the panel can tell the human their cancel did not
   * leave rather than showing it as if it had — the exact contract `sendAgentMessage` uses. */
  readonly cancelToolCall: (invocationId: string) => boolean;
};
/** 🌉️ Dials the ShellBridge WebSocket (the `config` given, or the live gateway offer
 * {@link useDiscoveredAgentBridgeConfig} keeps asking the local supervisor for), keeps a `ShellState`
 * mirror in sync via `reduce()`, and surfaces connection status, agent presence, the live
 * conversation and pending approvals for `AgentPresence`/`💬️AgentChatPanel`/`🤖️AgentApprovals` to
 * render. Never blocks the UI thread: every socket call is fire-and-forget or scheduled on a timer,
 * and no standing offer simply keeps `status: "disabled"` rather than throwing. A gateway that
 * restarts publishes a new offer on a new port with a new proof; discovery swaps the config and this
 * effect redials it, while an unchanged offer keeps the exact same object identity and redials
 * nothing. */
export function useAgentBridge(options: UseAgentBridgeOptions = {}): UseAgentBridgeResult {
  const { setVersionMismatch, reconnectAttemptRef, config, setStatus, socketRef, reconnectTimerRef, pingTimerRef, setLastError, send, shellStateRef, instancesRef, setShellState, onCommandAppliedRef, setPendingApprovals, setConversation, setPresence, onAppCommandRef, shellKind, shellSessionId, principalActor, flags, status, instances, shellState, presence, pendingApprovals, conversation, lastError, versionMismatch, dispatch, resolveApproval, sendAgentMessage, cancelToolCall } = useDiscoveredState({ options });
  usePart2({ setVersionMismatch, reconnectAttemptRef, config, setStatus, socketRef, reconnectTimerRef, pingTimerRef, setLastError, send, shellStateRef, instancesRef, setShellState, onCommandAppliedRef, setPendingApprovals, setConversation, setPresence, onAppCommandRef, shellKind, shellSessionId, principalActor, flags });
  usePart3({ status, send, instances });
  return { status, shellState, presence, pendingApprovals, conversation, lastError, versionMismatch, dispatch, resolveApproval, sendAgentMessage, cancelToolCall };
}
