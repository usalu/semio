//#region 🛰️AgentBridgeOffer
/** 🛰️ The agent-bridge OFFER both shells discover — React's `useDiscoveredAgentBridgeConfig` and the wasm32 wgpu
 * page (`🚀️browser-boot`), which forwards it to its frame Worker — so the two cannot drift into two admission rules. A
 * zero-import leaf: the dev server's `semioAgentBridgeRendezvousVitePlugin` serves the offer at the same path, and the
 * wgpu page bundle may carry nothing React. */
export type AgentBridgeConfig = { readonly url: string; readonly admissionProof: string };

/** 🛰️ The loopback endpoint the local supervisor (the dev server's
 * `semioAgentBridgeRendezvousVitePlugin`) serves the live gateway's own offer on. Admission never
 * travels through an environment variable or a build-time define: the supervisor reads the
 * owner-only `~/.semio/agent/bridge/offers/<pid>.json` the gateway wrote and hands it over loopback,
 * on request, as an {@link AgentBridgeOfferAnswerV1}. */
export const AGENT_BRIDGE_OFFER_ENDPOINT = "/__semio/agent-bridge";

/** 🏷️ The schema every answer on {@link AGENT_BRIDGE_OFFER_ENDPOINT} carries. */
export const AGENT_BRIDGE_OFFER_SCHEMA_V1 = "semio.os.agent-bridge-offer/v1";

/** 📨️ The supervisor's answer, always `200`: the live gateway's offer, or the typed "not offered". Every shell polls
 * the endpoint from boot on, so "no gateway" is an ordinary answer — it used to be a `404`, which put a failed request
 * into the console of every canonical zero-touch session (ticket 26/09/23 U5). Language-neutral rows:
 * `🔗️AgentBridge/🧫️fixtures/🛰️offer-answers/🔣️.json`. */
export type AgentBridgeOfferAnswerV1 =
  | { readonly schema: typeof AGENT_BRIDGE_OFFER_SCHEMA_V1; readonly offered: true; readonly url: string; readonly admissionProof: string }
  | { readonly schema: typeof AGENT_BRIDGE_OFFER_SCHEMA_V1; readonly offered: false };

/** 📤️ Encodes what the supervisor found — the newest live offer, or none — as its answer. */
export function agentBridgeOfferAnswerV1(offer: AgentBridgeConfig | null): AgentBridgeOfferAnswerV1 {
  return offer === null ? { schema: AGENT_BRIDGE_OFFER_SCHEMA_V1, offered: false } : { schema: AGENT_BRIDGE_OFFER_SCHEMA_V1, offered: true, url: offer.url, admissionProof: offer.admissionProof };
}

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

/** 📨️ Reads one {@link AgentBridgeOfferAnswerV1} into a config, or `null` when it offers none. Every field is
 * checked: a body that is not exactly an answer of this schema, the typed "not offered", a missing/empty proof and an
 * inadmissible URL all answer `null`, so a malformed or poisoned offer can never become a dialled socket. */
export function parseAgentBridgeOffer(body: unknown): AgentBridgeConfig | null {
  if (typeof body !== "object" || body === null || Array.isArray(body)) return null;
  const offer = body as { schema?: unknown; offered?: unknown; url?: unknown; admissionProof?: unknown };
  if (offer.schema !== AGENT_BRIDGE_OFFER_SCHEMA_V1 || offer.offered !== true || Object.keys(offer).length !== 4) return null;
  if (typeof offer.url !== "string" || typeof offer.admissionProof !== "string") return null;
  if (offer.admissionProof.length === 0) return null;
  if (!isAdmissibleBridgeUrl(offer.url)) return null;
  return { url: offer.url, admissionProof: offer.admissionProof };
}

export type BridgeOfferFetch = (input: string, init?: { readonly cache?: RequestCache; readonly signal?: AbortSignal }) => Promise<{ readonly ok: boolean; readonly status: number; json: () => Promise<unknown> }>;

/** 🏷️ The rendezvous record version both halves write and read (`🌉️mcp/🛰️rendezvous::RENDEZVOUS_SCHEMA_VERSION`): the live
 * os session records and the gateway offer records — version 2 gave every offer its {@link AgentBridgeOfferRecordV2} scope. */
export const AGENT_BRIDGE_RENDEZVOUS_SCHEMA_VERSION = 2;

/** 🎯️ Which shells one gateway offer is for: a hub-bound gateway's offer is for the shell of the human whose delegation it
 * runs, open on that hub and space; a local (folder) gateway's offer is for any shell of this machine's user. */
export type AgentBridgeOfferScopeKindV2 = { readonly kind: "hub"; readonly hubOrigin: string; readonly spaceId: string } | { readonly kind: "local" };

/** 📨️ One live gateway's offer record as the gateway publishes it, owner-only, in `<rendezvous>/offers/<pid>.json`
 * (`🌉️mcp/🛰️rendezvous::BridgeOffer`). `principal` is the principal the gateway acts as — for a delegated agent,
 * `agent:<delegation id>`. Language-neutral rows: `🧫️fixtures/🛰️offer-answers/🔣️.json` `select`. */
export type AgentBridgeOfferRecordV2 = {
  readonly schemaVersion: typeof AGENT_BRIDGE_RENDEZVOUS_SCHEMA_VERSION;
  readonly url: string;
  readonly admissionProof: string;
  readonly principal: string;
  readonly pid: number;
  readonly publishedAtMs: number;
  readonly scope: AgentBridgeOfferScopeKindV2;
};

/** 🎯️ Who asks for an offer: the shell's hub origin, the space it has open, and the agent principals the hub lists as
 * delegated BY THIS HUMAN in that space (`GET /auth/agent-delegations` answers only the asking human's own). `null` — a
 * shell in no hub space. */
export type AgentBridgeOfferScopeV1 = { readonly hubOrigin: string; readonly spaceId: string; readonly agentPrincipalIds: readonly string[] } | null;

/** 📨️ Reads one offer record file, exactly: a record of another version, with a missing or extra field or an unknown scope
 * is no offer at all. */
export function parseAgentBridgeOfferRecordV2(value: unknown): AgentBridgeOfferRecordV2 | null {
  if (typeof value !== "object" || value === null || Array.isArray(value)) return null;
  const record = value as Record<string, unknown>;
  if (Object.keys(record).sort().join(",") !== "admissionProof,pid,principal,publishedAtMs,schemaVersion,scope,url") return null;
  if (record.schemaVersion !== AGENT_BRIDGE_RENDEZVOUS_SCHEMA_VERSION || typeof record.url !== "string" || typeof record.admissionProof !== "string" || typeof record.principal !== "string") return null;
  if (typeof record.pid !== "number" || typeof record.publishedAtMs !== "number") return null;
  const scope = record.scope as Record<string, unknown> | null;
  if (typeof scope !== "object" || scope === null) return null;
  const keys = Object.keys(scope).sort().join(",");
  const parsed: AgentBridgeOfferScopeKindV2 | null =
    scope.kind === "local" && keys === "kind"
      ? { kind: "local" }
      : scope.kind === "hub" && keys === "hubOrigin,kind,spaceId" && typeof scope.hubOrigin === "string" && typeof scope.spaceId === "string"
        ? { kind: "hub", hubOrigin: scope.hubOrigin, spaceId: scope.spaceId }
        : null;
  return parsed === null ? null : { schemaVersion: AGENT_BRIDGE_RENDEZVOUS_SCHEMA_VERSION, url: record.url, admissionProof: record.admissionProof, principal: record.principal, pid: record.pid, publishedAtMs: record.publishedAtMs, scope: parsed };
}

/** 🔐️ The offer a shell of `scope` may dial, newest first (ticket 26/09/23, G12 session 14c: a gateway's offer used to go to
 * every live os session of the machine, and another person's shell attached to it). A hub gateway's offer is served only
 * to the shell open on its hub and space whose human delegated its agent; a local gateway's offer to any shell of this
 * machine's user. A shell with a matching hub offer never takes a local one instead. */
export function selectAgentBridgeOfferV1(records: readonly AgentBridgeOfferRecordV2[], scope: AgentBridgeOfferScopeV1): AgentBridgeOfferRecordV2 | null {
  const newest = (candidates: readonly AgentBridgeOfferRecordV2[]): AgentBridgeOfferRecordV2 | null => candidates.reduce<AgentBridgeOfferRecordV2 | null>((best, record) => (best === null || record.publishedAtMs > best.publishedAtMs ? record : best), null);
  const origin = (value: string): string => value.replace(/\/+$/u, "");
  const hub = scope === null ? [] : records.filter((record) => record.scope.kind === "hub" && origin(record.scope.hubOrigin) === origin(scope.hubOrigin) && record.scope.spaceId === scope.spaceId && scope.agentPrincipalIds.includes(record.principal));
  return newest(hub) ?? newest(records.filter((record) => record.scope.kind === "local"));
}

/** 🎯️ A shell's offer scope from the hub's list of THIS human's delegations in the open space: only a live delegation (not
 * revoked, not expired) names an agent whose offer the shell may dial — a withdrawn delegation's gateway is never offered
 * again, while the gateway still runs. */
export function agentBridgeOfferScopeFromDelegationsV1(hubOrigin: string, spaceId: string, delegations: readonly { readonly agentPrincipalId: string; readonly revoked: boolean; readonly expiresAtMs: number }[], nowMs: number): AgentBridgeOfferScopeV1 {
  return { hubOrigin, spaceId, agentPrincipalIds: delegations.filter((delegation) => !delegation.revoked && delegation.expiresAtMs > nowMs).map((delegation) => delegation.agentPrincipalId) };
}

/** 🔗️ The offer request of a shell of `scope` — the endpoint itself for a shell in no hub space. */
export function agentBridgeOfferPathV1(endpoint: string, scope: AgentBridgeOfferScopeV1): string {
  if (scope === null) return endpoint;
  return `${endpoint}?${new URLSearchParams({ hub: scope.hubOrigin, space: scope.spaceId, agents: scope.agentPrincipalIds.join(",") }).toString()}`;
}

/** 🔎️ The scope an offer request names — `null` when it names no hub space. */
export function parseAgentBridgeOfferScopeV1(requestUrl: string): AgentBridgeOfferScopeV1 {
  const at = requestUrl.indexOf("?");
  const query = new URLSearchParams(at < 0 ? "" : requestUrl.slice(at + 1));
  const hubOrigin = query.get("hub");
  const spaceId = query.get("space");
  if (!hubOrigin || !spaceId) return null;
  return { hubOrigin, spaceId, agentPrincipalIds: (query.get("agents") ?? "").split(",").filter((agent) => agent.length > 0) };
}

/** 🎯️ Reads the scope a wgpu shell publishes for its page (`dumpAgentBridgeOfferScope`): exactly `{hubOrigin, spaceId,
 * agentPrincipalIds}` of non-empty strings, else `null` (the shell is in no hub space and dials only local offers). */
export function agentBridgeOfferScopeFromJsonV1(json: string | null): AgentBridgeOfferScopeV1 {
  if (json === null) return null;
  let value: unknown;
  try {
    value = JSON.parse(json);
  } catch {
    return null;
  }
  if (typeof value !== "object" || value === null || Array.isArray(value)) return null;
  const record = value as Record<string, unknown>;
  if (Object.keys(record).sort().join(",") !== "agentPrincipalIds,hubOrigin,spaceId") return null;
  const { hubOrigin, spaceId, agentPrincipalIds } = record;
  if (typeof hubOrigin !== "string" || hubOrigin.length === 0 || typeof spaceId !== "string" || spaceId.length === 0) return null;
  if (!Array.isArray(agentPrincipalIds) || !agentPrincipalIds.every((agent) => typeof agent === "string" && agent.length > 0)) return null;
  return { hubOrigin, spaceId, agentPrincipalIds: agentPrincipalIds as string[] };
}

/** 🔎️ Asks the local supervisor for the offer a shell of `scope` may dial. Never throws and never rejects: the typed "not
 * offered", a host without the endpoint, a non-JSON body and a refused offer are all the same ordinary `null`
 * ("no agent is offering a bridge right now"), because the shell must render identically whether or
 * not anybody ever launches an MCP gateway. */
export async function fetchAgentBridgeConfig(endpoint: string = AGENT_BRIDGE_OFFER_ENDPOINT, fetchImpl?: BridgeOfferFetch, signal?: AbortSignal, scope: AgentBridgeOfferScopeV1 = null): Promise<AgentBridgeConfig | null> {
  const request = fetchImpl ?? (globalThis.fetch as unknown as BridgeOfferFetch | undefined);
  if (!request) return null;
  try {
    const response = await request(agentBridgeOfferPathV1(endpoint, scope), { cache: "no-store", signal });
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

/** 🔗️ Exact ordered websocket subprotocols keep admission out of URLs, logs, and referrers. */
export function bridgeProtocols(config: AgentBridgeConfig): readonly ["semio.mcp.bridge.v1", string] {
  return ["semio.mcp.bridge.v1", config.admissionProof];
}
/** ⏱️ The next discovery delay: doubling from {@link BRIDGE_DISCOVERY_MIN_INTERVAL_MS} while no offer stands, the max
 * once one does (the only way a shell learns the gateway restarted with another port and proof). */
export function nextBridgeDiscoveryIntervalMs(currentMs: number, discovered: AgentBridgeConfig | null): number {
  return discovered === null ? Math.min(currentMs * 2, BRIDGE_DISCOVERY_MAX_INTERVAL_MS) : BRIDGE_DISCOVERY_MAX_INTERVAL_MS;
}

/** 🟰️ Whether two offers name the same gateway and proof — a shell re-dials only when this is false. */
export function sameAgentBridgeOffer(left: AgentBridgeConfig | null, right: AgentBridgeConfig | null): boolean {
  return left === right || (left !== null && right !== null && left.url === right.url && left.admissionProof === right.admissionProof);
}

/** 🛰️ Polls the supervisor's offer on the shared schedule and publishes every CHANGE — an offer appearing, changing or
 * going away — the page-realm loop both wgpu page entries run (`🚀️browser-boot` forwards it to its frame Worker,
 * `🎬️renderer-boot` hands it to the page-mounted renderer). `offerScope` is re-read before every poll, exactly like React's
 * hook: the shell's hub, space and its human's agents (a failed read is a shell in no hub space). Returns the stop. */
export function watchAgentBridgeOffer(
  publish: (offer: AgentBridgeConfig | null) => void,
  options: {
    readonly endpoint?: string;
    readonly fetchImpl?: BridgeOfferFetch;
    readonly setTimer?: (run: () => void, delayMs: number) => unknown;
    readonly clearTimer?: (handle: unknown) => void;
    readonly offerScope?: (signal: AbortSignal) => Promise<AgentBridgeOfferScopeV1>;
  } = {},
): () => void {
  const setTimer = options.setTimer ?? ((run: () => void, delayMs: number) => setTimeout(run, delayMs));
  const clearTimer = options.clearTimer ?? ((handle: unknown) => clearTimeout(handle as ReturnType<typeof setTimeout>));
  const abort = new AbortController();
  let current: AgentBridgeConfig | null = null;
  let interval = BRIDGE_DISCOVERY_MIN_INTERVAL_MS;
  let timer: unknown = null;
  let stopped = false;
  const poll = async (): Promise<void> => {
    const scope = await (options.offerScope?.(abort.signal) ?? Promise.resolve(null)).catch(() => null);
    if (stopped) return;
    const discovered = await fetchAgentBridgeConfig(options.endpoint ?? AGENT_BRIDGE_OFFER_ENDPOINT, options.fetchImpl, abort.signal, scope);
    if (stopped) return;
    if (!sameAgentBridgeOffer(current, discovered)) {
      current = discovered;
      publish(discovered);
    }
    interval = nextBridgeDiscoveryIntervalMs(interval, discovered);
    timer = setTimer(() => void poll(), interval);
  };
  void poll();
  return () => {
    stopped = true;
    abort.abort();
    if (timer !== null) clearTimer(timer);
  };
}
//#endregion 🛰️AgentBridgeOffer
