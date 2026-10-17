/** 🔗️ Config discovery: 24 tests covering parseAgentBridgeOffer / isAdmissibleBridgeUrl; accepts exactly the typed offered answer the supervisor sends; refuses an offer that is not an object, has no proof, or has an empty proof; and… */
// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/AgentBridge/🔭️config-discovery.test.ts
/** 🔭️ AgentBridge discovery: the offer schema and answers, offer selection, the config fetch, the offer watch and the discovery hooks, each against a real loopback request or a fake transport. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import Ajv from "ajv";
import { type AgentBridgeConfig, type BridgeOfferFetch, AGENT_BRIDGE_OFFER_ENDPOINT, AGENT_BRIDGE_OFFER_SCHEMA_V1, agentBridgeOfferAnswerV1, BRIDGE_DISCOVERY_MIN_INTERVAL_MS, useAgentBridge, useDiscoveredAgentBridgeConfig, bridgeProtocols, fetchAgentBridgeConfig, isAdmissibleBridgeUrl, parseAgentBridgeOffer } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

const here = dirname(fileURLToPath(import.meta.url));

//#region 🔖️ConfigDiscovery
/** 🧪️ Ticket `26/09/18` slice M7: discovery is a real loopback request to the local supervisor
 * (`🔌️vite-plugins`' `semioAgentBridgeRendezvousVitePlugin`), which reads the owner-only offer file
 * the `semio-os-mcp` gateway published. These laws pin the two things that must never regress: an
 * inadmissible offer is refused rather than dialled, and an unchanged offer polled repeatedly keeps
 * one object identity so the socket effect never redials. */
describe("parseAgentBridgeOffer / isAdmissibleBridgeUrl", () => {
  it("accepts exactly the typed offered answer the supervisor sends", () => {
    expect(parseAgentBridgeOffer({ schema: AGENT_BRIDGE_OFFER_SCHEMA_V1, offered: true, url: "ws://127.0.0.1:6300/bridge", admissionProof: "deadbeef" })).toEqual({ url: "ws://127.0.0.1:6300/bridge", admissionProof: "deadbeef" });
    expect(parseAgentBridgeOffer({ url: "ws://127.0.0.1:6300/bridge", admissionProof: "deadbeef", principal: "agent:local", pid: 42 })).toBeNull();
  });

  it("refuses an offer that is not an object, has no proof, or has an empty proof", () => {
    expect(parseAgentBridgeOffer(null)).toBeNull();
    expect(parseAgentBridgeOffer("ws://127.0.0.1:6300/bridge")).toBeNull();
    expect(parseAgentBridgeOffer({ url: "ws://127.0.0.1:6300/bridge" })).toBeNull();
    expect(parseAgentBridgeOffer({ url: "ws://127.0.0.1:6300/bridge", admissionProof: "" })).toBeNull();
  });

  it("refuses a URL carrying its own credential, and every non-loopback host", () => {
    expect(isAdmissibleBridgeUrl("ws://127.0.0.1:6300/bridge?token=secret")).toBe(false);
    expect(isAdmissibleBridgeUrl("ws://user:pass@127.0.0.1:6300/bridge")).toBe(false);
    expect(isAdmissibleBridgeUrl("ws://evil.example.com:6300/bridge")).toBe(false);
    expect(isAdmissibleBridgeUrl("http://127.0.0.1:6300/bridge")).toBe(false);
    expect(isAdmissibleBridgeUrl("not a url")).toBe(false);
    expect(isAdmissibleBridgeUrl("ws://localhost:6300/bridge")).toBe(true);
  });
});

/** 🛰️ LAW over the language-neutral answer rows (`🧫️fixtures/🛰️offer-answers/🔣️.json`): the supervisor always answers
 * 200 with a typed answer — the encoder produces exactly the fixture's answers, the shell's parser dials exactly the
 * fixture's configs, and Ajv (third-party oracle) agrees with the fixture on which bodies are well-formed answers of
 * the canonical renderer contract. Ticket 26/09/23 U5: "no gateway" used to be a 404, a console error on every canonical session. */
describe("agent-bridge offer answers", () => {
  it("encode and decode every fixture row, and agree with the JSON-schema oracle on the answer shape", () => {
    const fixture = JSON.parse(readFileSync(join(here, "../🧫️fixtures/🛰️offer-answers/🔣️.json"), "utf8")) as {
      readonly encode: readonly { readonly id: string; readonly offer: AgentBridgeConfig | null; readonly answer: unknown }[];
      readonly decode: readonly { readonly id: string; readonly body: unknown; readonly shape: boolean; readonly expected: AgentBridgeConfig | null }[];
    };
    const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(here, "../../../../../🧬️schema/🔣️.json"), "utf8")).$defs.AgentBridgeOfferAnswerV1);
    expect(fixture.encode.length).toBeGreaterThanOrEqual(2);
    for (const row of fixture.encode) {
      expect(agentBridgeOfferAnswerV1(row.offer), row.id).toEqual(row.answer);
      expect(validate(row.answer), `${row.id}: the encoded answer is a schema answer`).toBe(true);
    }
    expect(fixture.decode.length).toBeGreaterThanOrEqual(12);
    for (const row of fixture.decode) {
      expect(parseAgentBridgeOffer(row.body), row.id).toEqual(row.expected);
      expect(validate(row.body), `${row.id}: Ajv shape`).toBe(row.shape);
      if (row.expected !== null) expect(row.shape, `${row.id}: only a well-formed answer is ever dialled`).toBe(true);
    }
  });
});

/** 🔐️ LAW over the language-neutral `select` rows (scoped offers, ticket 26/09/23 G12 session 14c): a hub gateway's offer
 * reaches only the shell open on its hub and space whose own human's delegation list names its agent — a second human's
 * shell on the same machine never receives it — a local offer any shell, newest first. Every record is a schema record
 * (Ajv, third-party oracle), parses exactly, and the request a shell sends round-trips its scope. The Rust twin
 * (`🌉️mcp/🛰️rendezvous` quick law) decodes and re-encodes the same records. */
describe("agent-bridge offer selection", () => {
  it("serves every fixture request exactly the expected offer", async () => {
    const { agentBridgeOfferPathV1, agentBridgeOfferScopeFromDelegationsV1, parseAgentBridgeOfferRecordV2, parseAgentBridgeOfferScopeV1, selectAgentBridgeOfferV1 } = await import("../../🛰️offer/🟦️.ts");
    const fixture = JSON.parse(readFileSync(join(here, "../🧫️fixtures/🛰️offer-answers/🔣️.json"), "utf8")) as {
      readonly select: {
        readonly records: readonly unknown[];
        readonly cases: readonly { readonly id: string; readonly request: string; readonly live: readonly number[]; readonly expectedPid: number | null }[];
        readonly scopes: readonly { readonly id: string; readonly hubOrigin: string; readonly spaceId: string; readonly nowMs: number; readonly delegations: readonly { readonly agentPrincipalId: string; readonly revoked: boolean; readonly expiresAtMs: number }[]; readonly expectedAgents: readonly string[]; readonly expectedPid: number | null }[];
      };
    };
    const ajv = new Ajv({ strict: true });
    const validRecord = ajv.compile(JSON.parse(readFileSync(join(here, "../../../../../🧬️schema/🔣️.json"), "utf8")).$defs.AgentBridgeOfferRecordV2);
    const records = fixture.select.records.map((record) => {
      expect(validRecord(record), JSON.stringify(validRecord.errors)).toBe(true);
      const parsed = parseAgentBridgeOfferRecordV2(record);
      expect(parsed, JSON.stringify(record)).toEqual(record);
      return parsed!;
    });
    expect(parseAgentBridgeOfferRecordV2({ ...(fixture.select.records[0] as object), schemaVersion: 1 })).toBeNull();
    expect(parseAgentBridgeOfferRecordV2({ ...(fixture.select.records[0] as object), scope: { kind: "hub", hubOrigin: "http://127.0.0.1:7800" } })).toBeNull();
    expect(fixture.select.cases.length).toBeGreaterThanOrEqual(8);
    for (const row of fixture.select.cases) {
      const scope = parseAgentBridgeOfferScopeV1(row.request);
      if (scope !== null && scope.agentPrincipalIds.length > 0) expect(agentBridgeOfferPathV1(AGENT_BRIDGE_OFFER_ENDPOINT, scope), row.id).toBe(row.request);
      const served = selectAgentBridgeOfferV1(records.filter((record) => row.live.includes(record.pid)), scope);
      expect(served?.pid ?? null, row.id).toBe(row.expectedPid);
    }
    expect(fixture.select.scopes.length).toBeGreaterThanOrEqual(3);
    for (const row of fixture.select.scopes) {
      const scope = agentBridgeOfferScopeFromDelegationsV1(row.hubOrigin, row.spaceId, row.delegations, row.nowMs);
      expect(scope?.agentPrincipalIds, row.id).toEqual(row.expectedAgents);
      expect(selectAgentBridgeOfferV1(records, scope)?.pid ?? null, row.id).toBe(row.expectedPid);
    }
  });
});

describe("fetchAgentBridgeConfig", () => {
  const offerFetch = (status: number, body: unknown): BridgeOfferFetch => async () => ({ ok: status >= 200 && status < 300, status, json: async () => body });

  it("asks with the shell's scope and without one when the shell is in no hub space", async () => {
    const asked: string[] = [];
    const recording: BridgeOfferFetch = async (input) => {
      asked.push(input);
      return { ok: true, status: 200, json: async () => agentBridgeOfferAnswerV1(null) };
    };
    await fetchAgentBridgeConfig(AGENT_BRIDGE_OFFER_ENDPOINT, recording, undefined, { hubOrigin: "http://127.0.0.1:7800", spaceId: "space-a", agentPrincipalIds: ["agent:alice-agent"] });
    await fetchAgentBridgeConfig(AGENT_BRIDGE_OFFER_ENDPOINT, recording);
    expect(asked).toEqual(["/__semio/agent-bridge?hub=http%3A%2F%2F127.0.0.1%3A7800&space=space-a&agents=agent%3Aalice-agent", "/__semio/agent-bridge"]);
  });

  it("reads a live offer off the supervisor endpoint", async () => {
    await expect(fetchAgentBridgeConfig(AGENT_BRIDGE_OFFER_ENDPOINT, offerFetch(200, agentBridgeOfferAnswerV1({ url: "ws://127.0.0.1:6300/bridge", admissionProof: "proof" })))).resolves.toEqual({ url: "ws://127.0.0.1:6300/bridge", admissionProof: "proof" });
  });

  it("treats the typed not-offered answer — and a host without the endpoint — as the ordinary null, not an error", async () => {
    await expect(fetchAgentBridgeConfig(AGENT_BRIDGE_OFFER_ENDPOINT, offerFetch(200, agentBridgeOfferAnswerV1(null)))).resolves.toBeNull();
    await expect(fetchAgentBridgeConfig(AGENT_BRIDGE_OFFER_ENDPOINT, offerFetch(404, ""))).resolves.toBeNull();
  });

  it("never rejects when the endpoint is absent or the body is not JSON", async () => {
    const throwing: BridgeOfferFetch = async () => {
      throw new Error("fetch failed");
    };
    await expect(fetchAgentBridgeConfig(AGENT_BRIDGE_OFFER_ENDPOINT, throwing)).resolves.toBeNull();
    const badBody: BridgeOfferFetch = async () => ({ ok: true, status: 200, json: async () => { throw new Error("Unexpected token <"); } });
    await expect(fetchAgentBridgeConfig(AGENT_BRIDGE_OFFER_ENDPOINT, badBody)).resolves.toBeNull();
  });
});

/** 🛰️ LAW: the page-realm watcher both wgpu page entries run (`🚀️browser-boot` → frame Worker, `🎬️renderer-boot` →
 * page-mounted renderer) walks React's discovery schedule and publishes only CHANGES: an offer appearing, the gateway
 * restarting with another proof, and the offer going away — never an unchanged offer twice, so the wgpu bridge never
 * redials on a poll. */
describe("watchAgentBridgeOffer", () => {
  it("publishes appear/change/vanish once each and backs off exactly like React's hook", async () => {
    const { watchAgentBridgeOffer, nextBridgeDiscoveryIntervalMs, BRIDGE_DISCOVERY_MAX_INTERVAL_MS } = await import("../../🛰️offer/🟦️.ts");
    const served: (AgentBridgeConfig | null)[] = [null, null, { url: "ws://127.0.0.1:6300/bridge", admissionProof: "first" }, { url: "ws://127.0.0.1:6300/bridge", admissionProof: "first" }, { url: "ws://127.0.0.1:6301/bridge", admissionProof: "second" }, null];
    let call = 0;
    const fetchImpl: BridgeOfferFetch = async () => {
      const offer = served[Math.min(call, served.length - 1)] ?? null;
      call += 1;
      return { ok: true, status: 200, json: async () => agentBridgeOfferAnswerV1(offer) };
    };
    const published: (AgentBridgeConfig | null)[] = [];
    const delays: number[] = [];
    const pending: (() => void)[] = [];
    const stop = watchAgentBridgeOffer((offer) => published.push(offer), { fetchImpl, setTimer: (run, delayMs) => { delays.push(delayMs); pending.push(run); return pending.length; }, clearTimer: () => {} });
    for (let step = 0; step < served.length; step += 1) {
      await vi.waitFor(() => expect(delays.length).toBe(step + 1));
      if (step + 1 < served.length) pending[step]!();
    }
    stop();
    expect(published).toEqual([served[2], served[4], null]);
    expect(delays).toEqual([4000, 8000, BRIDGE_DISCOVERY_MAX_INTERVAL_MS, BRIDGE_DISCOVERY_MAX_INTERVAL_MS, BRIDGE_DISCOVERY_MAX_INTERVAL_MS, BRIDGE_DISCOVERY_MAX_INTERVAL_MS]);
    expect(nextBridgeDiscoveryIntervalMs(BRIDGE_DISCOVERY_MIN_INTERVAL_MS, null)).toBe(2 * BRIDGE_DISCOVERY_MIN_INTERVAL_MS);
  });

  it("asks with the shell's scope, re-read before every poll, and reads the wgpu shell's published scope exactly", async () => {
    const { watchAgentBridgeOffer, agentBridgeOfferScopeFromJsonV1, agentBridgeOfferPathV1 } = await import("../../🛰️offer/🟦️.ts");
    const scopes = [
      { hubOrigin: "http://127.0.0.1:7800", spaceId: "space-a", agentPrincipalIds: ["agent:d-1"] },
      null,
      { hubOrigin: "http://127.0.0.1:7800", spaceId: "space-b", agentPrincipalIds: [] },
    ] as const;
    const asked: string[] = [];
    let reads = 0;
    const pending: (() => void)[] = [];
    const stop = watchAgentBridgeOffer(() => {}, {
      fetchImpl: async (input) => {
        asked.push(input);
        return { ok: true, status: 200, json: async () => agentBridgeOfferAnswerV1(null) };
      },
      offerScope: async () => scopes[Math.min(reads++, scopes.length - 1)] ?? null,
      setTimer: (run) => {
        pending.push(run);
        return pending.length;
      },
      clearTimer: () => {},
    });
    for (let poll = 0; poll < scopes.length; poll += 1) {
      await vi.waitFor(() => expect(asked.length).toBe(poll + 1));
      if (poll + 1 < scopes.length) pending[poll]!();
    }
    stop();
    expect(asked).toEqual(scopes.map((scope) => agentBridgeOfferPathV1(AGENT_BRIDGE_OFFER_ENDPOINT, scope)));
    expect(agentBridgeOfferScopeFromJsonV1(JSON.stringify(scopes[0]))).toEqual(scopes[0]);
    for (const refused of [null, "null", "not json", "[]", JSON.stringify({ hubOrigin: "http://127.0.0.1:7800", spaceId: "space-a" }), JSON.stringify({ ...scopes[0], token: "secret" }), JSON.stringify({ ...scopes[0], agentPrincipalIds: [7] }), JSON.stringify({ ...scopes[0], spaceId: "" })]) {
      expect(agentBridgeOfferScopeFromJsonV1(refused)).toBeNull();
    }
  });

  it("refuses a poisoned offer as no offer at all", async () => {
    const { watchAgentBridgeOffer } = await import("../../🛰️offer/🟦️.ts");
    const published: (AgentBridgeConfig | null)[] = [];
    let armed = 0;
    const stop = watchAgentBridgeOffer((offer) => published.push(offer), { fetchImpl: async () => ({ ok: true, status: 200, json: async () => agentBridgeOfferAnswerV1({ url: "ws://evil.example.com:6300/bridge", admissionProof: "x" }) }), setTimer: () => { armed += 1; return armed; }, clearTimer: () => {} });
    await vi.waitFor(() => expect(armed).toBe(1));
    stop();
    expect(published).toEqual([]);
  });
});

describe("useDiscoveredAgentBridgeConfig", () => {
  it("returns the live offer, keeps ONE object identity while it is unchanged, and swaps when the gateway restarts", async () => {
    const { renderHook, waitFor } = await import("@testing-library/react");
    let served: { url: string; admissionProof: string } | null = { url: "ws://127.0.0.1:6300/bridge", admissionProof: "first" };
    let calls = 0;
    const fetchImpl: BridgeOfferFetch = async () => {
      calls += 1;
      return { ok: true, status: 200, json: async () => agentBridgeOfferAnswerV1(served) };
    };
    vi.useFakeTimers({ shouldAdvanceTime: true });
    const hook = renderHook(() => useDiscoveredAgentBridgeConfig({ fetchImpl }));
    try {
      await waitFor(() => expect(hook.result.current).toEqual({ url: "ws://127.0.0.1:6300/bridge", admissionProof: "first" }));
      const firstIdentity = hook.result.current;
      const callsAfterFirst = calls;

      await vi.advanceTimersByTimeAsync(BRIDGE_DISCOVERY_MIN_INTERVAL_MS * 40);
      expect(calls).toBeGreaterThan(callsAfterFirst);
      expect(hook.result.current).toBe(firstIdentity);

      served = { url: "ws://127.0.0.1:6399/bridge", admissionProof: "second" };
      await vi.advanceTimersByTimeAsync(BRIDGE_DISCOVERY_MIN_INTERVAL_MS * 40);
      await waitFor(() => expect(hook.result.current).toEqual({ url: "ws://127.0.0.1:6399/bridge", admissionProof: "second" }));

      served = null;
      await vi.advanceTimersByTimeAsync(BRIDGE_DISCOVERY_MIN_INTERVAL_MS * 40);
      await waitFor(() => expect(hook.result.current).toBeNull());
    } finally {
      hook.unmount();
      vi.useRealTimers();
    }
  });

  it("asks nothing at all when discovery is disabled", async () => {
    const { renderHook } = await import("@testing-library/react");
    let calls = 0;
    const fetchImpl: BridgeOfferFetch = async () => {
      calls += 1;
      return { ok: true, status: 200, json: async () => agentBridgeOfferAnswerV1(null) };
    };
    const hook = renderHook(() => useDiscoveredAgentBridgeConfig({ enabled: false, fetchImpl }));
    try {
      expect(hook.result.current).toBeNull();
      expect(calls).toBe(0);
    } finally {
      hook.unmount();
    }
  });
});

describe("useAgentBridge discovery", () => {
  it("dials the discovered offer with its proof in the subprotocols, and redials exactly once when the gateway restarts", async () => {
    const { renderHook, waitFor } = await import("@testing-library/react");
    const opened: { url: string; protocols: readonly string[]; closed: boolean }[] = [];
    class Socket {
      static OPEN = 1;
      readyState = 1;
      onopen: (() => void) | null = null;
      onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
      onerror: (() => void) | null = null;
      onclose: (() => void) | null = null;
      binaryType = "arraybuffer";
      readonly record: (typeof opened)[number];
      constructor(url: string, protocols: readonly string[]) {
        this.record = { url, protocols, closed: false };
        opened.push(this.record);
      }
      send(_bytes: Uint8Array): void {}
      close(): void {
        this.record.closed = true;
      }
    }
    vi.stubGlobal("WebSocket", Socket);
    let served: { url: string; admissionProof: string } | null = null;
    const fetchImpl: BridgeOfferFetch = async () => ({ ok: true, status: 200, json: async () => agentBridgeOfferAnswerV1(served) });
    vi.useFakeTimers({ shouldAdvanceTime: true });
    const hook = renderHook(() => useAgentBridge({ discoveryFetch: fetchImpl }));
    try {
      await vi.advanceTimersByTimeAsync(BRIDGE_DISCOVERY_MIN_INTERVAL_MS * 4);
      expect(opened).toEqual([]);
      expect(hook.result.current.status).toBe("disabled");

      served = { url: "ws://127.0.0.1:6300/bridge", admissionProof: "live.proof" };
      await vi.advanceTimersByTimeAsync(BRIDGE_DISCOVERY_MIN_INTERVAL_MS * 40);
      await waitFor(() => expect(opened.length).toBe(1));
      expect(opened[0]!.url).toBe("ws://127.0.0.1:6300/bridge");
      expect(opened[0]!.protocols).toEqual(["semio.mcp.bridge.v1", "live.proof"]);

      // 🔁️ Many more polls of the SAME offer must not open a second socket.
      await vi.advanceTimersByTimeAsync(BRIDGE_DISCOVERY_MIN_INTERVAL_MS * 200);
      expect(opened.length).toBe(1);

      served = { url: "ws://127.0.0.1:6399/bridge", admissionProof: "restarted.proof" };
      await vi.advanceTimersByTimeAsync(BRIDGE_DISCOVERY_MIN_INTERVAL_MS * 40);
      await waitFor(() => expect(opened.length).toBe(2));
      expect(opened[0]!.closed).toBe(true);
      expect(opened[1]!.protocols).toEqual(["semio.mcp.bridge.v1", "restarted.proof"]);
    } finally {
      hook.unmount();
      vi.useRealTimers();
      vi.unstubAllGlobals();
    }
  });
});

describe("bridgeProtocols", () => {
  it("keeps the proof in exact ordered websocket subprotocols and leaves the URL credential-free", () => {
    const config = { url: "ws://127.0.0.1:6300/bridge", admissionProof: "session.v1.selector.proof" };
    expect(bridgeProtocols(config)).toEqual(["semio.mcp.bridge.v1", "session.v1.selector.proof"]);
    expect(config.url).toBe("ws://127.0.0.1:6300/bridge");
  });
});
//#endregion 🔖️ConfigDiscovery
