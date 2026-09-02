// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/AgentBridge/component.tsx
/** @emoji 🌉️ `AgentBridge` — headless hook that dials the `semio-os-mcp` gateway's ShellBridge
 * WebSocket, publishes `ShellState` snapshots, receives inbound `ShellCommand` frames and applies
 * them via the `@semio-tech/framework-os-shell` reducer twin, and tracks agent presence + pending
 * capability approvals for `AgentPresence`/`AgentApprovals` to render. Ticket
 * `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` packet P10 — see
 * `.🧬semio/…/📓️terra-P10-report.md` for the port/token discovery compromise (§ DiscoverConfig
 * below) and the exact `ShellHost` lease diff that mounts this hook.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useCallback, useEffect, useRef, useState } from "react";
import { registerUiTranslationBundles } from "@semio-tech/ui-react";
import { reduce, type ReduceResult, type ShellCommand, type ShellState } from "../../../../🖥️shell/🟦️.ts";
import {
  NO_BRIDGE_FLAGS,
  decodeGatewayToShell,
  encodeShellToGateway,
  type ApprovalDecision,
  type BridgeFlags,
  type GatewayToShell,
  type ShellKind,
  type ShellToGateway,
} from "../../../../🌉️mcp/🧵️bridge/🟦️.ts";
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
            working: { label: { normal: "Agent working: {{label}}", beginner: "Agent working: {{label}}" } },
            idle: { label: { normal: "Agent idle", beginner: "Agent idle" } },
            statusLabel: { label: { normal: "Agent status", beginner: "Agent status" } },
          },
          approvals: {
            trigger: { label: { normal: "Open agent approvals", beginner: "Open agent approvals" } },
            title: { label: { normal: "Agent Approvals", beginner: "Agent Approvals" } },
            description: { label: { normal: "Review what the agent wants to do before it runs.", beginner: "Review what the agent wants to do before it runs." } },
            empty: { label: { normal: "No pending approvals", beginner: "No pending approvals" } },
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
            pendingCount: { label: { normal: "{{count}} pending", beginner: "{{count}} pending" } },
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
            working: { label: { normal: "Agent aktiv: {{label}}", beginner: "Agent aktiv: {{label}}" } },
            idle: { label: { normal: "Agent inaktiv", beginner: "Agent inaktiv" } },
            statusLabel: { label: { normal: "Agent-Status", beginner: "Agent-Status" } },
          },
          approvals: {
            trigger: { label: { normal: "Agent-Freigaben öffnen", beginner: "Agent-Freigaben öffnen" } },
            title: { label: { normal: "Agent-Freigaben", beginner: "Agent-Freigaben" } },
            description: { label: { normal: "Prüfe, was der Agent tun möchte, bevor es ausgeführt wird.", beginner: "Prüfe, was der Agent tun möchte, bevor es ausgeführt wird." } },
            empty: { label: { normal: "Keine ausstehenden Freigaben", beginner: "Keine ausstehenden Freigaben" } },
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
            pendingCount: { label: { normal: "{{count}} ausstehend", beginner: "{{count}} ausstehend" } },
          },
        },
      },
    },
  },
});
//#endregion 🌐️Labels

//#region 🔖️DiscoverConfig
export type AgentBridgeConfig = { readonly url: string; readonly token: string };

/** 🔎️ Discovers the bridge's `ws://host:port/bridge` URL + bearer token for a BROWSER shell.
 *
 * The gateway's CLI mints its bridge token loopback-only per `📋️master.md` §2.1; a browser tab
 * has no filesystem access to read it. Verified by reading `🌉️mcp/🦀️.rs` and
 * `🌉️mcp/📦️bin.rs` directly before writing this file: `run_http` does not yet mount
 * `bridge::server::bridge_router()` into the real HTTP process at all (only `/mcp` is served —
 * P1b's report §2.4/§5 calls the `/bridge` route a still-unmounted skeleton), and no
 * `~/.semio/agent/bridge-token`-style file is written anywhere in this crate today. There is
 * therefore no live gateway a browser can dial yet, dev-server-injected config or not — see the
 * P10 report's "port/token discovery" section. This function reads the one seam a dev server CAN
 * inject at build/launch time, mirroring `ShellHost/🟦️.tsx`'s `readViteSEnv` idiom
 * (guarded for non-Vite embeds where `import.meta.env` is absent): `VITE_SEMIO_BRIDGE_URL` /
 * `VITE_SEMIO_BRIDGE_TOKEN`. Returns `null` (never throws) when either is unset — `useAgentBridge`
 * treats `null` as "stay disabled, never attempt a connection". */
export function discoverAgentBridgeConfig(source?: Readonly<Record<string, string | undefined>>): AgentBridgeConfig | null {
  const env =
    source ??
    (() => {
      try {
        return (import.meta as unknown as { readonly env?: Readonly<Record<string, string | undefined>> }).env ?? {};
      } catch {
        return {};
      }
    })();
  const url = env.VITE_SEMIO_BRIDGE_URL;
  const token = env.VITE_SEMIO_BRIDGE_TOKEN;
  if (!url || !token) return null;
  return { url, token };
}

/** 🔗️ Appends `?token=…` to a bridge URL exactly once, matching `📋️master.md` §2.1's
 * `ws://127.0.0.1:<port>/bridge?token=…` shape — tolerant of a caller-supplied URL that already
 * carries other query params. */
export function bridgeUrlWithToken(config: AgentBridgeConfig): string {
  const separator = config.url.includes("?") ? "&" : "?";
  return `${config.url}${separator}token=${encodeURIComponent(config.token)}`;
}
//#endregion 🔖️DiscoverConfig

//#region 🔖️DefaultState
/** 🌱️ A fresh, empty `ShellState` — the mirror `AgentBridge` reduces `ShellCommand`s against
 * before a real `ShellHost`-derived snapshot is ever supplied via `initialState`. Field-for-field
 * identical to `🖥️shell/🟦️.ts`'s own in-source `defaultState()` test fixture (that
 * function is not exported — it lives inside an `import.meta.vitest`-gated block — so this is a
 * deliberate, checked-against-the-fixtures duplicate, not drift). */
export function createDefaultShellState(): ShellState {
  return {
    revision: 0,
    loadedPlugins: [],
    pluginStatusById: {},
    pluginSupervisorById: {},
    activeSession: null,
    sessionError: null,
    appLabelsOverlay: {},
    actionPaneFoldedByWindow: {},
    actionPaneExpandedByWindow: {},
    stagedActionArgs: {},
    activeUtilityByWindow: {},
    activeToolId: null,
    commandPanelExpanded: null,
    stagedCommandArgs: {},
    panelsVisible: { left: false, right: false, top: false, bottom: false },
    panelsSize: { left: 280, right: 280, top: 280, bottom: 280 },
    panelsPath: { left: [], right: [], top: [], bottom: [] },
    dockOverride: null,
    panelPathMemory: {},
    treeOpenStates: {},
    activeWindowId: null,
    shellLayout: null,
    activeExampleId: "",
    mobilePanelPath: [],
    mobilePanelVisible: false,
    extraWindows: [],
    windowTitlesById: {},
    windowIconsById: {},
    searchOpen: false,
    findOpen: false,
    introductionStepIndex: null,
    introductionAutoStartedKeys: [],
    introductionCompletedInteractions: [],
    dialogStack: [],
    transientNotice: null,
    openWithFocusRole: null,
    activeTutorialId: null,
    uiAppearance: "system",
    uiLayout: "default",
    uiDriverId: "",
    uiCustomDrivers: {},
    uiDriverDraft: null,
    uiLocale: "en",
    uiTerminology: "",
    uiThemeId: "",
    uiCustomThemes: {},
    uiThemeDraft: null,
    uiKeybindingOverrides: {},
    syncBackboneUri: null,
    syncCardKind: null,
    syncDraftPath: "",
    syncStatusByDocument: {},
    mergePolicy: "manual",
    conflicts: [],
    selectedConflictId: null,
    storageScope: "memory",
    openingPreferences: {},
  };
}
//#endregion 🔖️DefaultState

//#region 🔖️FramePayloads
/** 📦️ Wire payload codec for the `bytes` fields inside `ShellState`/`ShellStatePatch`/
 * `ShellCommand`/`AppCommand` bridge frames. `📋️master.md` §2.2 names these `pack`, but that
 * binary format lives in `semio-framework-actor` — a peer ticket's exclusive, mid-rewrite
 * territory (`📌️important.md`'s collision matrix, packet A4) this packet must not depend on
 * (mirrors P1b's own §7.5 deviation for the frame envelope itself). Plain JSON is what the
 * gateway's own dispatch already expects on the other end (P9 report §8.4:
 * `serde_json::from_value::<ShellCommand>`), so it is used symmetrically here for both
 * directions. */
export function encodeJsonPayload(value: unknown): Uint8Array {
  return new TextEncoder().encode(JSON.stringify(value));
}

export function decodeJsonPayload<T>(bytes: Uint8Array): T {
  return JSON.parse(new TextDecoder("utf-8").decode(bytes)) as T;
}

/** 📤️ Builds the `shellState` frame publishing a full snapshot — sent on `welcome` and whenever
 * a `shellStatePatch`'s `baseRevision` would not match (this packet always sends full snapshots;
 * P11's real adoption packet can add patch diffing without touching this frame shape). */
export function buildShellStateFrame(state: ShellState): ShellToGateway {
  return { variant: "shellState", revision: BigInt(state.revision), state: encodeJsonPayload(state) };
}

export type ApplyInboundShellCommandOutcome = {
  /** `null` only when `commandBytes` failed to decode — no `ShellCommand` ever existed to reduce. */
  readonly command: ShellCommand | null;
  /** `null` only when `commandBytes` failed to decode; otherwise the real `reduce()` outcome
   * (which itself may be `{ok:false}` for a command the reducer rejects — that is NOT a decode
   * failure and is reported via `result.error`, not `null`). */
  readonly result: ReduceResult | null;
  readonly resultFrame: ShellToGateway;
};

/** 🧮️ Applies one inbound `shellCommand` frame's JSON-encoded payload to `state` via the
 * `@semio-tech/framework-os-shell` reducer twin, returning the decoded command, the reducer's own
 * {@link ReduceResult}, and the `shellCommandResult` frame to send back — the exact seam
 * `useAgentBridge`'s socket handler and this file's own tests both call. */
export function applyInboundShellCommand(state: ShellState, seq: bigint, commandBytes: Uint8Array, nowMs: number): ApplyInboundShellCommandOutcome {
  let command: ShellCommand;
  try {
    command = decodeJsonPayload<ShellCommand>(commandBytes);
  } catch (error) {
    const fault = error instanceof Error ? error.message : "malformed ShellCommand payload";
    return { command: null, result: null, resultFrame: { variant: "shellCommandResult", inReplyTo: seq, ok: false, fault } };
  }
  const result = reduce(state, command, nowMs);
  const resultFrame: ShellToGateway = { variant: "shellCommandResult", inReplyTo: seq, ok: result.ok, fault: result.ok ? null : result.error.kind };
  return { command, result, resultFrame };
}
//#endregion 🔖️FramePayloads

//#region 🔖️Hook
export type AgentBridgeStatus = "disabled" | "connecting" | "open" | "reconnecting" | "closed";

export type AgentBridgePresence = { readonly active: boolean; readonly label: string; readonly invocationId: string | null };
const IDLE_PRESENCE: AgentBridgePresence = { active: false, label: "", invocationId: null };

export type PendingAgentApproval = { readonly approvalId: string; readonly summary: string; readonly requestedAtMs: number };

export type UseAgentBridgeOptions = {
  readonly config?: AgentBridgeConfig | null;
  readonly shellKind?: ShellKind;
  readonly shellSessionId?: string;
  readonly principalActor?: string;
  readonly flags?: BridgeFlags;
  readonly initialState?: ShellState;
  readonly onCommandApplied?: (command: ShellCommand | null, result: ReduceResult | null) => void;
};

export type UseAgentBridgeResult = {
  readonly status: AgentBridgeStatus;
  readonly shellState: ShellState;
  readonly presence: AgentBridgePresence;
  readonly pendingApprovals: readonly PendingAgentApproval[];
  readonly lastError: string | null;
  readonly dispatch: (command: ShellCommand) => ReduceResult;
  readonly resolveApproval: (approvalId: string, decision: ApprovalDecision, note?: string) => void;
};

const RECONNECT_BASE_MS = 1000;
const RECONNECT_MAX_MS = 30000;
const PING_INTERVAL_MS = 20000;

/** 🌉️ Dials the ShellBridge WebSocket (when `config`/`discoverAgentBridgeConfig()` resolves one),
 * keeps a `ShellState` mirror in sync via `reduce()`, and surfaces connection status, agent
 * presence and pending approvals for `AgentPresence`/`AgentApprovals` to render. Never blocks the
 * UI thread: every socket call is fire-and-forget or scheduled on a timer, and a missing/invalid
 * config simply keeps `status: "disabled"` forever rather than throwing. */
export function useAgentBridge(options: UseAgentBridgeOptions = {}): UseAgentBridgeResult {
  const config = options.config === undefined ? discoverAgentBridgeConfig() : options.config;
  const shellKind = options.shellKind ?? "react";
  const shellSessionId = options.shellSessionId ?? `shell-${Math.random().toString(36).slice(2)}`;
  const principalActor = options.principalActor ?? "agent:unknown";
  const flags = options.flags ?? NO_BRIDGE_FLAGS;

  const [status, setStatus] = useState<AgentBridgeStatus>(config ? "connecting" : "disabled");
  const [shellState, setShellState] = useState<ShellState>(() => options.initialState ?? createDefaultShellState());
  const [presence, setPresence] = useState<AgentBridgePresence>(IDLE_PRESENCE);
  const [pendingApprovals, setPendingApprovals] = useState<readonly PendingAgentApproval[]>([]);
  const [lastError, setLastError] = useState<string | null>(null);

  const socketRef = useRef<WebSocket | null>(null);
  const shellStateRef = useRef(shellState);
  shellStateRef.current = shellState;
  const reconnectAttemptRef = useRef(0);
  const reconnectTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const pingTimerRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const onCommandAppliedRef = useRef(options.onCommandApplied);
  onCommandAppliedRef.current = options.onCommandApplied;

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
    },
    [send],
  );

  useEffect(() => {
    if (!config) {
      setStatus("disabled");
      return;
    }
    let disposed = false;

    const clearReconnectTimer = () => {
      if (reconnectTimerRef.current !== null) {
        clearTimeout(reconnectTimerRef.current);
        reconnectTimerRef.current = null;
      }
    };
    const clearPingTimer = () => {
      if (pingTimerRef.current !== null) {
        clearInterval(pingTimerRef.current);
        pingTimerRef.current = null;
      }
    };

    const scheduleReconnect = () => {
      if (disposed) return;
      clearPingTimer();
      const attempt = reconnectAttemptRef.current + 1;
      reconnectAttemptRef.current = attempt;
      const delay = Math.min(RECONNECT_BASE_MS * 2 ** (attempt - 1), RECONNECT_MAX_MS);
      setStatus("reconnecting");
      reconnectTimerRef.current = setTimeout(connect, delay);
    };

    const handleFrame = (frame: GatewayToShell) => {
      switch (frame.variant) {
        case "welcome": {
          reconnectAttemptRef.current = 0;
          setStatus("open");
          setLastError(null);
          send(buildShellStateFrame(shellStateRef.current));
          break;
        }
        case "shellCommand": {
          const { command, result, resultFrame } = applyInboundShellCommand(shellStateRef.current, frame.seq, frame.command, Date.now());
          if (result?.ok) {
            shellStateRef.current = result.state;
            setShellState(result.state);
          }
          onCommandAppliedRef.current?.(command, result);
          send(resultFrame);
          break;
        }
        case "approvalRequested": {
          setPendingApprovals((current) => [...current.filter((approval) => approval.approvalId !== frame.approvalId), { approvalId: frame.approvalId, summary: frame.summary, requestedAtMs: Date.now() }]);
          break;
        }
        case "approvalResolved": {
          setPendingApprovals((current) => current.filter((approval) => approval.approvalId !== frame.approvalId));
          break;
        }
        case "agentPresence": {
          setPresence({ active: frame.active, label: frame.label, invocationId: frame.invocationId });
          break;
        }
        case "pong":
          break;
        case "bye": {
          setLastError(frame.reason || null);
          break;
        }
        case "appCommand":
          break;
      }
    };

    function connect(): void {
      if (disposed) return;
      setStatus(reconnectAttemptRef.current > 0 ? "reconnecting" : "connecting");
      let socket: WebSocket;
      try {
        socket = new WebSocket(bridgeUrlWithToken(config as AgentBridgeConfig));
      } catch (error) {
        setLastError(error instanceof Error ? error.message : "failed to open bridge socket");
        scheduleReconnect();
        return;
      }
      socket.binaryType = "arraybuffer";
      socketRef.current = socket;

      socket.onopen = () => {
        if (disposed) return;
        send({ variant: "hello", bridgeVersion: 1, shellKind, shellSessionId, principalActor, flags });
        pingTimerRef.current = setInterval(() => send({ variant: "ping" }), PING_INTERVAL_MS);
      };
      socket.onmessage = (event) => {
        if (disposed) return;
        try {
          const bytes = event.data instanceof ArrayBuffer ? new Uint8Array(event.data) : new Uint8Array(0);
          handleFrame(decodeGatewayToShell(bytes));
        } catch (error) {
          setLastError(error instanceof Error ? error.message : "failed to decode bridge frame");
        }
      };
      socket.onerror = () => {
        if (disposed) return;
        setLastError("bridge socket error");
      };
      socket.onclose = () => {
        socketRef.current = null;
        clearPingTimer();
        if (!disposed) scheduleReconnect();
      };
    }

    connect();

    return () => {
      disposed = true;
      clearReconnectTimer();
      clearPingTimer();
      const socket = socketRef.current;
      socketRef.current = null;
      if (socket) {
        socket.onopen = null;
        socket.onmessage = null;
        socket.onerror = null;
        socket.onclose = null;
        try {
          socket.send(encodeShellToGateway({ variant: "bye" }));
        } catch {
          // best-effort — socket may already be closing
        }
        socket.close();
      }
      setStatus("disabled");
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [config?.url, config?.token, shellKind, shellSessionId, principalActor, send]);

  return { status, shellState, presence, pendingApprovals, lastError, dispatch, resolveApproval };
}
//#endregion 🔖️Hook
