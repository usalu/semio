/** 🔶️ The discovered phase of useAgentBridge. */
import { useCallback, useRef, useState } from "../../⚛️react-port/🟦️.ts";
import { defaultShellState, reduce, type ReduceResult, type ShellCommand, type ShellState } from "@semio-tech/framework/shell";
import { NO_BRIDGE_FLAGS, encodeShellToGateway, type ApprovalDecision, type BridgeFlags, type ShellToGateway } from "@semio-tech/framework/mcp/bridge";
import { EMPTY_INSTANCES, IDLE_PRESENCE, appendConversationEntry, buildShellStateFrame, updateConversationEntry, useDiscoveredAgentBridgeConfig } from "../🟢️shared/🟦️.tsx";
import type { AgentBridgePresence, AgentBridgeStatus, AgentBridgeVersionMismatch, AgentConversationEntry, PendingAgentApproval, UseAgentBridgeOptions } from "../🟢️shared/🟦️.tsx";

/** 🔷 Builds discovered, config, shellKind, generatedShellSessionIdRef of useAgentBridge. */
export function useDiscoveredState({ options }: { readonly options: UseAgentBridgeOptions }) {
  const discovered = useDiscoveredAgentBridgeConfig({ enabled: options.config === undefined, endpoint: options.discoveryEndpoint, fetchImpl: options.discoveryFetch, offerScope: options.offerScope });
  const config = options.config === undefined ? discovered : options.config;
  const shellKind = options.shellKind ?? "react";
  const generatedShellSessionIdRef = useRef(`shell-${Math.random().toString(36).slice(2)}`);
  const shellSessionId = options.shellSessionId ?? generatedShellSessionIdRef.current;
  const principalActor = options.principalActor ?? "agent:unknown";
  const declaredFlags = options.flags ?? NO_BRIDGE_FLAGS;
  const relayAppCommands = options.flags ? declaredFlags.relayAppCommands : options.onAppCommand !== undefined;
  const flagsIdentityRef = useRef<BridgeFlags>({ ...declaredFlags, relayAppCommands });
  if (flagsIdentityRef.current.relayAppCommands !== relayAppCommands || flagsIdentityRef.current.sharedBackbone !== declaredFlags.sharedBackbone || flagsIdentityRef.current.elicit !== declaredFlags.elicit) {
    flagsIdentityRef.current = { ...declaredFlags, relayAppCommands };
  }
  const flags = flagsIdentityRef.current;

  const [status, setStatus] = useState<AgentBridgeStatus>(config ? "connecting" : "disabled");
  const [shellState, setShellState] = useState<ShellState>(() => options.initialState ?? defaultShellState());
  const [presence, setPresence] = useState<AgentBridgePresence>(IDLE_PRESENCE);
  const [pendingApprovals, setPendingApprovals] = useState<readonly PendingAgentApproval[]>([]);
  const [conversation, setConversation] = useState<readonly AgentConversationEntry[]>([]);
  const [lastError, setLastError] = useState<string | null>(null);
  const [versionMismatch, setVersionMismatch] = useState<AgentBridgeVersionMismatch | null>(null);
  const nextMessageOrdinalRef = useRef(1);

  const socketRef = useRef<WebSocket | null>(null);
  const shellStateRef = useRef(shellState);
  shellStateRef.current = shellState;
  const reconnectAttemptRef = useRef(0);
  const reconnectTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const pingTimerRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const onCommandAppliedRef = useRef(options.onCommandApplied);
  onCommandAppliedRef.current = options.onCommandApplied;
  const onAppCommandRef = useRef(options.onAppCommand);
  onAppCommandRef.current = options.onAppCommand;
  const instances = options.instances ?? EMPTY_INSTANCES;
  const instancesRef = useRef(instances);
  instancesRef.current = instances;

  const send = useCallback((frame: ShellToGateway) => {
    const socket = socketRef.current;
    if (!socket || socket.readyState !== WebSocket.OPEN) return;
    try {
      socket.send(encodeShellToGateway(frame));
    } catch (error) {
      setLastError(error instanceof Error ? error.message : "failed to send bridge frame");
    }
  }, []);

  const dispatch = useCallback(
    (command: ShellCommand): ReduceResult => {
      const result = reduce(shellStateRef.current, command, Date.now());
      if (result.ok) {
        shellStateRef.current = result.state;
        setShellState(result.state);
        send(buildShellStateFrame(result.state));
      }
      onCommandAppliedRef.current?.(command, result);
      return result;
    },
    [send],
  );

  const resolveApproval = useCallback(
    (approvalId: string, decision: ApprovalDecision, note?: string) => {
      send({ variant: "approval", approvalId, decision, note: note ?? null });
      setPendingApprovals((current) => current.filter((approval) => approval.approvalId !== approvalId));
      setConversation((current) => updateConversationEntry(current, approvalId, (entry) => (entry.kind === "approval" ? { ...entry, state: "resolved", decision } : entry)));
    },
    [send],
  );

  const sendAgentMessage = useCallback(
    (text: string): boolean => {
      const trimmed = text.trim();
      const socket = socketRef.current;
      if (!trimmed || !socket || socket.readyState !== WebSocket.OPEN) return false;
      const messageId = `msg_${shellSessionId}_${nextMessageOrdinalRef.current}`;
      nextMessageOrdinalRef.current += 1;
      send({ variant: "agentMessage", messageId, text: trimmed });
      setConversation((current) => appendConversationEntry(current, { kind: "userMessage", id: messageId, text: trimmed, atMs: Date.now() }));
      return true;
    },
    [send, shellSessionId],
  );

  const cancelToolCall = useCallback(
    (invocationId: string): boolean => {
      const socket = socketRef.current;
      if (!invocationId || !socket || socket.readyState !== WebSocket.OPEN) return false;
      send({ variant: "agentCancel", invocationId });
      setConversation((current) => updateConversationEntry(current, invocationId, (entry) => (entry.kind === "toolCall" && entry.state === "running" ? { ...entry, state: "cancelling" } : entry)));
      return true;
    },
    [send],
  );
  return { setVersionMismatch, reconnectAttemptRef, config, setStatus, socketRef, reconnectTimerRef, pingTimerRef, setLastError, send, shellStateRef, instancesRef, setShellState, onCommandAppliedRef, setPendingApprovals, setConversation, setPresence, onAppCommandRef, shellKind, shellSessionId, principalActor, flags, status, instances, shellState, presence, pendingApprovals, conversation, lastError, versionMismatch, dispatch, resolveApproval, sendAgentMessage, cancelToolCall };
}
