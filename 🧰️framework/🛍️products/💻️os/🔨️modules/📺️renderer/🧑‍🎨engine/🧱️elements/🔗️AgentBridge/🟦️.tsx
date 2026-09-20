// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/AgentBridge/component.tsx
/** @emoji 🌉️ `AgentBridge` — headless hook that dials the `semio-os-mcp` gateway's ShellBridge
 * WebSocket, publishes `ShellState` snapshots, receives inbound `ShellCommand` frames and applies
 * them via the `@semio-tech/framework-os-shell` reducer twin, and tracks agent presence + pending
 * capability approvals for `AgentPresence`/`🤖️AgentApprovals` to render. Ticket
 * `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY` packet P10 — see
 * `.🧬semio/…/📓️terra-P10-report.md` for the exact `ShellHost` lease diff that mounts this hook.
 * Discovery (§ DiscoverConfig below) is a loopback request to the local supervisor, never an
 * environment or build-time credential carrier: ticket `26/09/18` slice M7.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useCallback, useEffect, useRef, useState } from "react";
import { registerUiTranslationBundles } from "@semio-tech/ui-react";
import { defaultShellState, reduce, type ReduceResult, type ShellCommand, type ShellState } from "../../../../🖥️shell/🟦️.ts";
import {
  NO_BRIDGE_FLAGS,
  decodeGatewayToShell,
  encodeShellToGateway,
  type ApprovalDecision,
  type BridgeFlags,
  type BridgeInstanceRef,
  type GatewayToShell,
  type ShellKind,
  type ShellToGateway,
} from "../../../../🌉️mcp/🧵️bridge/🟦️.ts";
import { decodeShellAppCommand, encodeShellAppFrame, shellAppFault, type ShellAppCommandV1, type ShellAppFrameV1 } from "../../../../🌉️mcp/🐚️channel/🟦️.ts";
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
          chat: {
            panelTitle: { label: { normal: "Chat", beginner: "Chat" } },
            transcriptLabel: { label: { normal: "Agent conversation", beginner: "Agent conversation" } },
            empty: { label: { normal: "No agent activity yet. Messages you send appear here, along with every tool the agent runs.", beginner: "No agent activity yet. Messages you send appear here, along with every tool the agent runs." } },
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
          chat: {
            panelTitle: { label: { normal: "Chat", beginner: "Chat" } },
            transcriptLabel: { label: { normal: "Agent-Konversation", beginner: "Agent-Konversation" } },
            empty: { label: { normal: "Noch keine Agent-Aktivität. Gesendete Nachrichten erscheinen hier, ebenso jedes vom Agent ausgeführte Werkzeug.", beginner: "Noch keine Agent-Aktivität. Gesendete Nachrichten erscheinen hier, ebenso jedes vom Agent ausgeführte Werkzeug." } },
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
export type AgentBridgeConfig = { readonly url: string; readonly admissionProof: string };

/** 🛰️ The loopback endpoint the local supervisor (the dev server's
 * `semioAgentBridgeRendezvousVitePlugin`) serves the live gateway's own offer on. Admission never
 * travels through an environment variable or a build-time define: the supervisor reads the
 * owner-only `~/.semio/agent/bridge/offers/<pid>.json` the gateway wrote and hands it over loopback,
 * on request. `404` is the ordinary "no gateway is offering a bridge" answer, never an error. */
export const AGENT_BRIDGE_OFFER_ENDPOINT = "/__semio/agent-bridge";

/** 🔓️ A bridge URL is admissible only when it is a loopback websocket that carries **no** credential
 * of its own: the proof travels in the websocket subprotocol ({@link bridgeProtocols}), so a URL with
 * a query string, a userinfo component or a non-loopback host is a poisoned offer and is refused
 * rather than dialled. */
export function isAdmissibleBridgeUrl(url: string): boolean {
  let parsed: URL;
  try {
    parsed = new URL(url);
  } catch {
    return false;
  }
  if (parsed.protocol !== "ws:" && parsed.protocol !== "wss:") return false;
  if (parsed.username !== "" || parsed.password !== "") return false;
  if (parsed.search !== "" || parsed.hash !== "") return false;
  return parsed.hostname === "127.0.0.1" || parsed.hostname === "localhost" || parsed.hostname === "[::1]" || parsed.hostname === "::1";
}

/** 📨️ Reads one supervisor offer body into a config, or `null` when it is not one. Every field is
 * checked: a body that is not an object, a missing/empty proof and an inadmissible URL all answer
 * `null`, so a malformed or poisoned offer can never become a dialled socket. */
export function parseAgentBridgeOffer(body: unknown): AgentBridgeConfig | null {
  if (typeof body !== "object" || body === null) return null;
  const offer = body as { url?: unknown; admissionProof?: unknown };
  if (typeof offer.url !== "string" || typeof offer.admissionProof !== "string") return null;
  if (offer.admissionProof.length === 0) return null;
  if (!isAdmissibleBridgeUrl(offer.url)) return null;
  return { url: offer.url, admissionProof: offer.admissionProof };
}

export type BridgeOfferFetch = (input: string, init?: { readonly cache?: RequestCache; readonly signal?: AbortSignal }) => Promise<{ readonly ok: boolean; readonly status: number; json: () => Promise<unknown> }>;

/** 🔎️ Asks the local supervisor for the live gateway's offer. Never throws and never rejects: a
 * missing endpoint, a `404`, a non-JSON body and a refused offer are all the same ordinary `null`
 * ("no agent is offering a bridge right now"), because the shell must render identically whether or
 * not anybody ever launches an MCP gateway. */
export async function fetchAgentBridgeConfig(endpoint: string = AGENT_BRIDGE_OFFER_ENDPOINT, fetchImpl?: BridgeOfferFetch, signal?: AbortSignal): Promise<AgentBridgeConfig | null> {
  const request = fetchImpl ?? (globalThis.fetch as unknown as BridgeOfferFetch | undefined);
  if (!request) return null;
  try {
    const response = await request(endpoint, { cache: "no-store", signal });
    if (!response.ok) return null;
    return parseAgentBridgeOffer(await response.json());
  } catch {
    return null;
  }
}

/** ⏱️ How often the shell re-asks the supervisor. While no offer is standing the interval doubles
 * from {@link BRIDGE_DISCOVERY_MIN_INTERVAL_MS} up to the max, so a shell that will never see an
 * agent settles at one cheap loopback GET every 30 s instead of a poll storm. The poll continues at
 * the max interval once an offer IS standing, because that is the only way a shell learns that the
 * gateway restarted on a different port with a different proof. */
export const BRIDGE_DISCOVERY_MIN_INTERVAL_MS = 2000;
export const BRIDGE_DISCOVERY_MAX_INTERVAL_MS = 30000;

export type UseDiscoveredAgentBridgeConfigOptions = {
  readonly enabled?: boolean;
  readonly endpoint?: string;
  readonly fetchImpl?: BridgeOfferFetch;
};

/** 🛰️ The live supervisor offer, re-asked on a backoff. The returned object identity changes **only**
 * when `url` or `admissionProof` actually change, so an unchanged offer polled a hundred times never
 * re-runs {@link useAgentBridge}'s socket effect — the redial storm U1 already had to fix once. */
export function useDiscoveredAgentBridgeConfig(options: UseDiscoveredAgentBridgeConfigOptions = {}): AgentBridgeConfig | null {
  const { enabled = true, endpoint = AGENT_BRIDGE_OFFER_ENDPOINT, fetchImpl } = options;
  const [config, setConfig] = useState<AgentBridgeConfig | null>(null);
  const fetchImplRef = useRef(fetchImpl);
  fetchImplRef.current = fetchImpl;

  useEffect(() => {
    if (!enabled) {
      setConfig(null);
      return;
    }
    let disposed = false;
    let timer: ReturnType<typeof setTimeout> | null = null;
    let interval = BRIDGE_DISCOVERY_MIN_INTERVAL_MS;

    const poll = async (): Promise<void> => {
      const discovered = await fetchAgentBridgeConfig(endpoint, fetchImplRef.current);
      if (disposed) return;
      setConfig((current) => {
        if (discovered === null) return current === null ? current : null;
        if (current !== null && current.url === discovered.url && current.admissionProof === discovered.admissionProof) return current;
        return discovered;
      });
      interval = discovered === null ? Math.min(interval * 2, BRIDGE_DISCOVERY_MAX_INTERVAL_MS) : BRIDGE_DISCOVERY_MAX_INTERVAL_MS;
      if (!disposed) timer = setTimeout(() => void poll(), interval);
    };

    void poll();
    return () => {
      disposed = true;
      if (timer !== null) clearTimeout(timer);
    };
  }, [enabled, endpoint]);

  return config;
}

/** 🔗️ Exact ordered websocket subprotocols keep admission out of URLs, logs, and referrers. */
export function bridgeProtocols(config: AgentBridgeConfig): readonly ["semio.mcp.bridge.v1", string] {
  return ["semio.mcp.bridge.v1", config.admissionProof];
}
//#endregion 🔖️DiscoverConfig

//#region 🔖️DefaultState
/** 🌱️ A fresh, empty `ShellState` — the mirror `AgentBridge` reduces `ShellCommand`s against before
 * a real `ShellHost`-derived snapshot is ever supplied via `initialState`. It is the SSOT twin's own
 * {@link defaultShellState}, re-exported under the name this module's callers already use: the
 * hand-written copy that used to live here drifted nine `ui*` rows wide (they belong to `🐚️Shell`'s
 * renderer preference state, not to the shell SSOT), which is exactly the failure a second spelling
 * of a default always produces. */
export { defaultShellState as createDefaultShellState };
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

//#region 🔖️ArtifactRoute
/** 🗿️ One inbound `appCommand` frame, decoded, with the correlation id its answer must carry.
 * `instanceId` is the shell's OWN plugin-instance id (the one this shell published in its
 * `instances` frame) — the gateway resolved it from the shell's census, never invented it. */
export type AgentAppCommandRequest = {
  readonly seq: bigint;
  readonly instanceId: string;
  readonly command: ShellAppCommandV1;
};

/** 🧑‍🔧️ What a host (`🏛️ShellHost`) supplies to make this shell the agent's document owner: it
 * executes `request` against the LIVE plugin instance — through the same action-dispatch lane the
 * UI uses, so the human sees the edit and can undo it — and answers the frames. Returning an
 * `error` frame is a first-class outcome; throwing is turned into one, never into silence. */
export type AgentAppCommandHandler = (request: AgentAppCommandRequest) => Promise<readonly ShellAppFrameV1[]> | readonly ShellAppFrameV1[];

/** 🚫️ The answer a shell with no artifact route mounted gives. It is a REFUSAL, not a timeout: the
 * agent learns immediately that this shell renders the agent chrome but does not own documents for
 * it, and its session can be re-resolved onto the headless channel instead of burning a wall
 * budget. */
/** 📇️ The stable empty census — a fresh `[]` per render would re-publish an `instances` frame on
 * every render, which is exactly the redial-storm class of defect one level down. */
const EMPTY_INSTANCES: readonly BridgeInstanceRef[] = [];

export const NO_ARTIFACT_ROUTE_MESSAGE = "this shell has no live artifact route mounted — it renders the agent surface but does not execute artifact commands; resolve a headless context (`--folder`/`--hub`) for mutations";

/** 📦️ Builds the `appFrames` answer for one request, never throwing: a handler that rejects or
 * throws becomes a single `channel.not-wired` error frame naming the failure. The correlation id
 * and the instance id are echoed verbatim — the gateway refuses a reply addressed elsewhere. */
export async function answerAgentAppCommand(request: AgentAppCommandRequest, handler: AgentAppCommandHandler | undefined): Promise<ShellToGateway> {
  let frames: readonly ShellAppFrameV1[];
  if (handler === undefined) {
    frames = [shellAppFault("plugin.unavailable", NO_ARTIFACT_ROUTE_MESSAGE)];
  } else {
    try {
      frames = await handler(request);
    } catch (error) {
      frames = [shellAppFault("channel.not-wired", error instanceof Error ? error.message : "the shell's artifact route failed without a message")];
    }
  }
  return { variant: "appFrames", inReplyTo: request.seq, instanceId: request.instanceId, frames: frames.map((frame) => encodeShellAppFrame(frame)) };
}
//#endregion 🔖️ArtifactRoute

//#region 🔖️Hook
export type AgentBridgeStatus = "disabled" | "connecting" | "open" | "reconnecting" | "closed";

export type AgentBridgePresence = { readonly active: boolean; readonly label: string; readonly invocationId: string | null };
const IDLE_PRESENCE: AgentBridgePresence = { active: false, label: "", invocationId: null };

export type PendingAgentApproval = { readonly approvalId: string; readonly summary: string; readonly requestedAtMs: number };

/** 💬️ One entry of the live agent conversation, exactly as the bridge reported it. `userMessage` is
 * a turn this shell itself sent (echoed locally the moment the frame leaves, so the panel is never
 * behind the human's own typing); every other kind is a real `GatewayToShell` frame the gateway
 * emitted from its own `tools/call` dispatch or approval gate. Nothing here is synthesised from a
 * guess: a tool call with no result yet simply has `state: "running"`. */
export type AgentConversationEntry =
  | { readonly kind: "userMessage"; readonly id: string; readonly text: string; readonly atMs: number }
  /** 🛑️ `cancelling` is this shell's own optimistic state between `cancelToolCall` leaving and the
   * gateway's real `agentToolResult` landing: cancellation is cooperative, so the call may still
   * succeed, fail, or settle as cancelled — the panel says "asked to stop", never "stopped". */
  | { readonly kind: "toolCall"; readonly id: string; readonly toolName: string; readonly args: string; readonly state: "running" | "cancelling" | "ok" | "failed"; readonly summary: string | null; readonly atMs: number }
  | { readonly kind: "approval"; readonly id: string; readonly summary: string; readonly state: "pending" | "resolved"; readonly decision: ApprovalDecision | null; readonly atMs: number };

/** ✂️ How many conversation entries the panel retains. The bridge is a live view, not an archive:
 * an agent running for hours must not grow this array without bound, and the oldest entries are the
 * least useful ones to keep. */
export const AGENT_CONVERSATION_MAX_ENTRIES = 200;

/** ➕️ Appends one entry and trims to {@link AGENT_CONVERSATION_MAX_ENTRIES}, oldest first. */
function appendConversationEntry(current: readonly AgentConversationEntry[], entry: AgentConversationEntry): readonly AgentConversationEntry[] {
  const next = [...current, entry];
  return next.length > AGENT_CONVERSATION_MAX_ENTRIES ? next.slice(next.length - AGENT_CONVERSATION_MAX_ENTRIES) : next;
}

/** 🔁️ Replaces the entry with `id`, leaving the rest untouched — how a `toolCall` becomes its own
 * result and a `pending` approval becomes a resolved one, in place, rather than as a second row. */
function updateConversationEntry(current: readonly AgentConversationEntry[], id: string, update: (entry: AgentConversationEntry) => AgentConversationEntry): readonly AgentConversationEntry[] {
  let found = false;
  const next = current.map((entry) => {
    if (entry.id !== id) return entry;
    found = true;
    return update(entry);
  });
  return found ? next : current;
}

export type UseAgentBridgeOptions = {
  /** 🛰️ Omit it (the product case) and the hook discovers the live gateway offer from the local
   * supervisor on {@link AGENT_BRIDGE_OFFER_ENDPOINT}; pass one (including `null`) and discovery is
   * off entirely, which is what every test and every embedder with its own supervisor does. */
  readonly config?: AgentBridgeConfig | null;
  /** 🔎️ Overrides for the discovery seam — the endpoint path and the `fetch` used to ask it. */
  readonly discoveryEndpoint?: string;
  readonly discoveryFetch?: BridgeOfferFetch;
  readonly shellKind?: ShellKind;
  readonly shellSessionId?: string;
  readonly principalActor?: string;
  readonly flags?: BridgeFlags;
  readonly initialState?: ShellState;
  readonly onCommandApplied?: (command: ShellCommand | null, result: ReduceResult | null) => void;
  /** 🗿️ Makes this shell the agent's document owner. Omit it and every inbound `appCommand` is
   * answered with {@link NO_ARTIFACT_ROUTE_MESSAGE} — the shell still renders the agent surface,
   * it just does not claim to execute mutations. */
  readonly onAppCommand?: AgentAppCommandHandler;
  /** 📇️ The live plugin instances this shell owns, published as `instances` frames whenever the
   * array's identity changes. The gateway resolves an `AppCommand`'s target from this census, so a
   * shell that publishes nothing is a shell no artifact verb can address. */
  readonly instances?: readonly BridgeInstanceRef[];
};

export type UseAgentBridgeResult = {
  readonly status: AgentBridgeStatus;
  readonly shellState: ShellState;
  readonly presence: AgentBridgePresence;
  readonly pendingApprovals: readonly PendingAgentApproval[];
  readonly conversation: readonly AgentConversationEntry[];
  readonly lastError: string | null;
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

const RECONNECT_BASE_MS = 1000;
const RECONNECT_MAX_MS = 30000;
const PING_INTERVAL_MS = 20000;

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
  const discovered = useDiscoveredAgentBridgeConfig({ enabled: options.config === undefined, endpoint: options.discoveryEndpoint, fetchImpl: options.discoveryFetch });
  const config = options.config === undefined ? discovered : options.config;
  const shellKind = options.shellKind ?? "react";
  // 🔒️ Minted ONCE per hook instance. It was `?? \`shell-${Math.random()…}\`` computed in the render
  // body while also sitting in the socket effect's dep array, so every state change this hook makes —
  // a tool call arriving, a message echoing, a cancel — tore the bridge socket down and redialled it.
  // Found by slice U1's `cancelToolCall` test, whose cancel frame was followed by an immediate `bye`.
  const generatedShellSessionIdRef = useRef(`shell-${Math.random().toString(36).slice(2)}`);
  const shellSessionId = options.shellSessionId ?? generatedShellSessionIdRef.current;
  const principalActor = options.principalActor ?? "agent:unknown";
  // 🚩️ `relayAppCommands` is a CLAIM, and the claim is what the gateway routes on: a shell that
  // declared it and then answers `plugin.unavailable` to every command is worse than one that never
  // claimed it (the gateway would have picked its own headless workspace instead). So the flag is
  // derived from the handler actually being mounted, not from a hand-set option — an explicit
  // `flags` option still wins for the tests and embedders that pin the whole mask.
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
  const nextMessageOrdinalRef = useRef(1);

  const socketRef = useRef<WebSocket | null>(null);
  const shellStateRef = useRef(shellState);
  shellStateRef.current = shellState;
  const reconnectAttemptRef = useRef(0);
  const reconnectTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const pingTimerRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const onCommandAppliedRef = useRef(options.onCommandApplied);
  onCommandAppliedRef.current = options.onCommandApplied;
  // 🗿️ A ref, for the same reason `onCommandApplied` is one: the socket effect must not re-dial
  // because a host re-created its handler closure on a render (`📓️m7` §2's no-redial-storm law).
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

  useEffect(() => {
    if (!config) {
      setStatus("disabled");
      return;
    }
    const admittedConfig = config;
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
          // 📇️ The instance census travels with the first snapshot: an `appCommand` that arrives
          // before the gateway knows what this shell has open cannot be addressed at all.
          send({ variant: "instances", entries: [...instancesRef.current] });
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
          setConversation((current) => appendConversationEntry(current, { kind: "approval", id: frame.approvalId, summary: frame.summary, state: "pending", decision: null, atMs: Date.now() }));
          break;
        }
        case "approvalResolved": {
          setPendingApprovals((current) => current.filter((approval) => approval.approvalId !== frame.approvalId));
          setConversation((current) => updateConversationEntry(current, frame.approvalId, (entry) => (entry.kind === "approval" ? { ...entry, state: "resolved", decision: frame.decision } : entry)));
          break;
        }
        case "agentToolCall": {
          setConversation((current) => appendConversationEntry(current, { kind: "toolCall", id: frame.invocationId, toolName: frame.toolName, args: frame.arguments, state: "running", summary: null, atMs: Date.now() }));
          break;
        }
        case "agentToolResult": {
          setConversation((current) => updateConversationEntry(current, frame.invocationId, (entry) => (entry.kind === "toolCall" ? { ...entry, state: frame.ok ? "ok" : "failed", summary: frame.summary } : entry)));
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
        case "appCommand": {
          // 🗿️ The live artifact route. Decoding happens here so a malformed payload answers a
          // named `channel.not-wired` error on the SAME correlation id instead of being dropped —
          // a dropped `appCommand` costs the agent its whole wall budget and tells it nothing.
          let request: AgentAppCommandRequest;
          try {
            request = { seq: frame.seq, instanceId: frame.instanceId, command: decodeShellAppCommand(frame.command) };
          } catch (error) {
            send({ variant: "appFrames", inReplyTo: frame.seq, instanceId: frame.instanceId, frames: [encodeShellAppFrame(shellAppFault("channel.not-wired", error instanceof Error ? error.message : "malformed AppCommand payload"))] });
            break;
          }
          void answerAgentAppCommand(request, onAppCommandRef.current).then(send);
          break;
        }
      }
    };

    function connect(): void {
      if (disposed) return;
      setStatus(reconnectAttemptRef.current > 0 ? "reconnecting" : "connecting");
      let socket: WebSocket;
      try {
        socket = new WebSocket(admittedConfig.url, [...bridgeProtocols(admittedConfig)]);
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
  }, [config?.url, config?.admissionProof, shellKind, shellSessionId, principalActor, send]);

  // 📇️ Re-publishes the census whenever the host's array identity changes — opening or closing an
  // artifact in the shell must change what an agent can address, without a reconnect.
  useEffect(() => {
    if (status !== "open") return;
    send({ variant: "instances", entries: [...instances] });
  }, [instances, status, send]);

  return { status, shellState, presence, pendingApprovals, conversation, lastError, dispatch, resolveApproval, sendAgentMessage, cancelToolCall };
}
//#endregion 🔖️Hook
