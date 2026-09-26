//#region 🛰️AgentBridgeOffer
/** @emoji 🛰️ The agent-bridge OFFER both shells discover — React's `useDiscoveredAgentBridgeConfig` and the wasm32 wgpu
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

/** 🔎️ Asks the local supervisor for the live gateway's offer. Never throws and never rejects: the typed "not
 * offered", a host without the endpoint, a non-JSON body and a refused offer are all the same ordinary `null`
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
 * `🎬️renderer-boot` hands it to the page-mounted renderer). Returns the stop. */
export function watchAgentBridgeOffer(
  publish: (offer: AgentBridgeConfig | null) => void,
  options: { readonly endpoint?: string; readonly fetchImpl?: BridgeOfferFetch; readonly setTimer?: (run: () => void, delayMs: number) => unknown; readonly clearTimer?: (handle: unknown) => void } = {},
): () => void {
  const setTimer = options.setTimer ?? ((run: () => void, delayMs: number) => setTimeout(run, delayMs));
  const clearTimer = options.clearTimer ?? ((handle: unknown) => clearTimeout(handle as ReturnType<typeof setTimeout>));
  const abort = new AbortController();
  let current: AgentBridgeConfig | null = null;
  let interval = BRIDGE_DISCOVERY_MIN_INTERVAL_MS;
  let timer: unknown = null;
  let stopped = false;
  const poll = async (): Promise<void> => {
    const discovered = await fetchAgentBridgeConfig(options.endpoint ?? AGENT_BRIDGE_OFFER_ENDPOINT, options.fetchImpl, abort.signal);
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
