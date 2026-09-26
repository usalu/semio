// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/AgentBridge/component.test.ts
/** 🧪️ Registered AgentBridge frame parity, protected configuration and exact Shell state laws. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import Ajv from "ajv";
import { type AgentBridgeConfig, type BridgeOfferFetch, AGENT_BRIDGE_OFFER_ENDPOINT, AGENT_BRIDGE_OFFER_SCHEMA_V1, agentBridgeOfferAnswerV1, BRIDGE_DISCOVERY_MIN_INTERVAL_MS, useAgentBridge, useDiscoveredAgentBridgeConfig, applyInboundShellCommand, bridgeProtocols, buildShellStateFrame, createDefaultShellState, decodeJsonPayload, fetchAgentBridgeConfig, isAdmissibleBridgeUrl, parseAgentBridgeOffer, encodeJsonPayload } from "../../🟦️.tsx";
import { bytesToHex, decodeShellToGateway, decodeGatewayToShell, encodeShellToGateway, encodeGatewayToShell, type GatewayToShell, type ShellToGateway } from "../../../../../../🌉️mcp/🧵️bridge/🟦️.ts";
import { answerAgentAppCommand, NO_ARTIFACT_ROUTE_MESSAGE } from "../../🟦️.tsx";
import { BRIDGE_HANDSHAKE_DEADLINE_MS, BRIDGE_UNANSWERED_ATTEMPTS, type AgentBridgeStatus, type AgentBridgeVersionMismatch } from "../../🟦️.tsx";
import { BRIDGE_VERSION } from "../../../../../../🌉️mcp/🧵️bridge/🟦️.ts";
import { shellAppFault } from "../../../../../../🌉️mcp/🐚️channel/🟦️.ts";
import { decodeChannelBase64, decodeShellAppCommand, encodeChannelBase64, shellAppFrameToJson, type ShellAppFrameV1 } from "../../../../../../🌉️mcp/🐚️channel/🟦️.ts";
// #endregion 🔌️Adapters

const here = dirname(fileURLToPath(import.meta.url));
const cancellationFixture = JSON.parse(readFileSync(join(here, "../../🧫️fixtures/🛑️cancellation/🔣️.json"), "utf8")) as {
  readonly invocation: { readonly id: string; readonly toolName: string; readonly arguments: string };
  readonly openCancellation: { readonly accepted: boolean; readonly outbound: ShellToGateway; readonly state: string };
  readonly closedCancellation: { readonly accepted: boolean; readonly outboundCount: number; readonly state: string };
  readonly terminalResult: { readonly ok: boolean; readonly summary: string; readonly state: string };
};

type HandshakeDial = "silence" | "close" | { readonly frame: GatewayToShell; readonly thenClose?: boolean };
const handshakeFixture = JSON.parse(readFileSync(join(here, "../../🧫️fixtures/🤝️handshake/🔣️.json"), "utf8")) as {
  readonly handshakeDeadlineMs: number;
  readonly unansweredAttempts: number;
  readonly shellVersion: number;
  readonly scenarios: readonly { readonly name: string; readonly dials: readonly HandshakeDial[]; readonly status: AgentBridgeStatus; readonly versionMismatch: AgentBridgeVersionMismatch | null }[];
};

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
 * `🧬️schema.json`. Ticket 26/09/23 U5: "no gateway" used to be a 404, a console error on every canonical session. */
describe("agent-bridge offer answers", () => {
  it("encode and decode every fixture row, and agree with the JSON-schema oracle on the answer shape", () => {
    const fixture = JSON.parse(readFileSync(join(here, "../../🧫️fixtures/🛰️offer-answers/🔣️.json"), "utf8")) as {
      readonly encode: readonly { readonly id: string; readonly offer: AgentBridgeConfig | null; readonly answer: unknown }[];
      readonly decode: readonly { readonly id: string; readonly body: unknown; readonly shape: boolean; readonly expected: AgentBridgeConfig | null }[];
    };
    const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(here, "../../🧫️fixtures/🛰️offer-answers/🧬️schema.json"), "utf8")) as object);
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

describe("fetchAgentBridgeConfig", () => {
  const offerFetch = (status: number, body: unknown): BridgeOfferFetch => async () => ({ ok: status >= 200 && status < 300, status, json: async () => body });

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

//#region 🔖️CodecParity
type FixtureRow = { readonly direction: "shell_to_gateway" | "gateway_to_shell"; readonly variant: string; readonly frame: unknown; readonly hex: string };

function loadFixtures(): readonly FixtureRow[] {
  const path = join(here, "..", "..", "..", "..", "..", "..", "🌉️mcp", "🧵️bridge", "🧫️fixtures", "📨️frames.json");
  return JSON.parse(readFileSync(path, "utf8")) as FixtureRow[];
}

describe("bridge frame codec (imported, not reimplemented) round-trips through every P1b fixture", () => {
  const fixtures = loadFixtures();

  it("has fixture rows in both directions", () => {
    expect(fixtures.length).toBeGreaterThan(0);
    expect(fixtures.some((row) => row.direction === "shell_to_gateway")).toBe(true);
    expect(fixtures.some((row) => row.direction === "gateway_to_shell")).toBe(true);
  });

  for (const row of loadFixtures()) {
    it(`${row.direction} ${row.variant} encodes to the fixture hex and decodes back to the fixture frame`, () => {
      if (row.direction === "shell_to_gateway") {
        const frame = row.frame as ShellToGateway;
        const bytes = encodeShellToGateway(frame);
        expect(bytesToHex(bytes)).toBe(row.hex);
        expect(decodeShellToGateway(bytes)).toEqual(normalizeBigints(frame));
      } else {
        const frame = row.frame as GatewayToShell;
        const bytes = encodeGatewayToShell(frame);
        expect(bytesToHex(bytes)).toBe(row.hex);
        expect(decodeGatewayToShell(bytes)).toEqual(normalizeBigints(frame));
      }
    });
  }
});

/** 🔢️ Fixture JSON stores `u64` fields as plain numbers and `bytes` fields as plain number
 * arrays; the codec's decoded frames carry them as `bigint` and `Uint8Array` respectively.
 * Mirrors what a real caller does with fixture data — compare against the same shape `decode*`
 * actually returns. */
function normalizeBigints<T>(value: T): T {
  if (typeof value !== "object" || value === null) return value;
  // 🧭️ An array is not a `Record<string, unknown>`; it is normalized element-wise and returned before
  // the keyed rewrites below, which only ever apply to an object.
  if (Array.isArray(value)) return (value as unknown[]).map((entry) => normalizeBigints(entry)) as T;
  const clone: Record<string, unknown> = { ...(value as Record<string, unknown>) };
  for (const key of ["revision", "baseRevision", "inReplyTo", "seq"]) {
    if (key in clone && typeof clone[key] === "number") clone[key] = BigInt(clone[key] as number);
  }
  for (const key of ["state", "patch", "command"]) {
    if (key in clone && Array.isArray(clone[key])) clone[key] = new Uint8Array(clone[key] as number[]);
  }
  if ("frames" in clone && Array.isArray(clone.frames)) clone.frames = (clone.frames as number[][]).map((frame) => new Uint8Array(frame));
  return clone as T;
}
//#endregion 🔖️CodecParity

//#region 🔖️ApplyInboundShellCommand
describe("applyInboundShellCommand", () => {
  it("applies a valid ShellCommand and emits an ok shellCommandResult frame with the bumped revision", () => {
    const state = createDefaultShellState();
    const commandBytes = encodeJsonPayload({ type: "setSearchOpen", open: true });
    const { command, result, resultFrame } = applyInboundShellCommand(state, 42n, commandBytes, 1_700_000_000_000);

    expect(command).toEqual({ type: "setSearchOpen", open: true });
    expect(result).not.toBeNull();
    if (!result?.ok) throw new Error("unreachable");
    expect(result.state.searchOpen).toBe(true);
    expect(result.state.revision).toBe(state.revision + 1);
    expect(state.searchOpen).toBe(false);
    expect(resultFrame).toEqual({ variant: "shellCommandResult", inReplyTo: 42n, ok: true, fault: null });
  });

  it("rejects a ShellCommand the reducer itself rejects and emits an ok:false frame carrying the error kind", () => {
    const state = createDefaultShellState();
    const commandBytes = encodeJsonPayload({ type: "selectConflict", conflictId: "missing" });
    const { result, resultFrame } = applyInboundShellCommand(state, 7n, commandBytes, 1_700_000_000_000);

    expect(result).not.toBeNull();
    if (!result || result.ok) throw new Error("unreachable");
    expect(result.error).toEqual({ kind: "unknownConflict", conflictId: "missing" });
    expect(resultFrame).toEqual({ variant: "shellCommandResult", inReplyTo: 7n, ok: false, fault: "unknownConflict" });
  });

  it("rejects a malformed payload without throwing, and never invents a command or reduce result", () => {
    const state = createDefaultShellState();
    const commandBytes = new TextEncoder().encode("not json");
    const { command, result, resultFrame } = applyInboundShellCommand(state, 1n, commandBytes, 1_700_000_000_000);

    expect(command).toBeNull();
    expect(result).toBeNull();
    expect(resultFrame.variant).toBe("shellCommandResult");
    if (resultFrame.variant === "shellCommandResult") {
      expect(resultFrame.ok).toBe(false);
      expect(resultFrame.fault).toBeTruthy();
    }
  });
});

describe("buildShellStateFrame", () => {
  it("round-trips a ShellState through JSON bytes", () => {
    const state = { ...createDefaultShellState(), searchOpen: true, revision: 3 };
    const frame = buildShellStateFrame(state);
    expect(frame.variant).toBe("shellState");
    if (frame.variant !== "shellState") throw new Error("unreachable");
    expect(frame.revision).toBe(3n);
    expect(decodeJsonPayload(frame.state)).toEqual(state);
  });
});
//#endregion 🔖️ApplyInboundShellCommand

describe("AgentBridge inference state parity", () => {
  it("starts from the neutral Shell state and applies the exact inference port command", async () => {
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(join(here, "../../../../../../🖥️shell/🧫️fixtures/💡️set-document-inference-port.json"), "utf8"));
    const state = createDefaultShellState();
    expect(equal(state, fixture.state)).toBe(true);
    expect(state).toEqual(fixture.state);
    const applied = applyInboundShellCommand(state, 8n, encodeJsonPayload(fixture.command), 1000);
    if (!applied.result?.ok) throw new Error("inference port command was rejected");
    expect(equal(applied.result.state, fixture.expected.state)).toBe(true);
    expect(applied.result.state).toEqual(fixture.expected.state);
    expect(state.inferencePortByDocument).toEqual({});
  });

  it("spells the neutral state exactly once, in the shell SSOT twin", async () => {
    const { defaultShellState } = await import("../../../../../../🖥️shell/🟦️.ts");
    expect(createDefaultShellState).toBe(defaultShellState);
    expect(Object.keys(createDefaultShellState()).sort()).toEqual(Object.keys(defaultShellState()).sort());
  });
});

describe("AgentBridge protected connection ownership", () => {
  it("retires the old socket before a replacement or disabled configuration", async () => {
    const { renderHook } = await import("@testing-library/react");
    const opened: { url: string; protocols: readonly string[]; closed: boolean }[] = [];
    class Socket {
      static OPEN = 1;
      readonly readyState = 1;
      readonly record: (typeof opened)[number];
      constructor(url: string, protocols: readonly string[]) {
        this.record = { url, protocols, closed: false };
        opened.push(this.record);
      }
      send(_bytes: Uint8Array): void {}
      close(): void { this.record.closed = true; }
    }
    vi.stubGlobal("WebSocket", Socket);
    const first = { url: "ws://127.0.0.1:6300/bridge", admissionProof: "session.v1.first.proof" };
    const second = { url: "ws://127.0.0.1:6301/bridge", admissionProof: "session.v1.second.proof" };
    const initialProps: { config: AgentBridgeConfig | null } = { config: first };
    const hook = renderHook(({ config }) => useAgentBridge({ config }), { initialProps });
    try {
      expect(opened).toEqual([{ url: first.url, protocols: bridgeProtocols(first), closed: false }]);
      hook.rerender({ config: second });
      expect(opened).toEqual([
        { url: first.url, protocols: bridgeProtocols(first), closed: true },
        { url: second.url, protocols: bridgeProtocols(second), closed: false },
      ]);
      hook.rerender({ config: null });
      expect(opened.every((socket) => socket.closed)).toBe(true);
      expect(hook.result.current.status).toBe("disabled");
      console.log("[DEBUG] AgentBridge protected config lifetime: opened=2 retired=2 disabled=1");
    } finally {
      hook.unmount();
      vi.unstubAllGlobals();
    }
  });
});

//#region 🔖️CancelToolCall
/** 🧪️ Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice U1 (audit ranked item 2): the shell
 * can now ask the gateway to stop a tool call it is showing as running. Drives the REAL hook over a
 * fake `WebSocket` (the one seam a jsdom test can inject), feeds a real `agentToolCall` frame in
 * through `socket.onmessage`, and asserts the exact `agentCancel` bytes that leave. */
describe("useAgentBridge cancelToolCall", () => {
  it("matches the shared cancellation lifecycle through the real React hook", async () => {
    const { renderHook, act } = await import("@testing-library/react");
    const sent: ShellToGateway[] = [];
    let live: { onmessage: ((event: { data: ArrayBuffer }) => void) | null } | null = null;
    class Socket {
      static OPEN = 1;
      readyState = 1;
      onopen: (() => void) | null = null;
      onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
      onerror: (() => void) | null = null;
      onclose: (() => void) | null = null;
      binaryType = "arraybuffer";
      constructor() {
        live = this as unknown as { onmessage: ((event: { data: ArrayBuffer }) => void) | null };
      }
      send(bytes: Uint8Array): void {
        sent.push(decodeShellToGateway(bytes));
      }
      close(): void {}
    }
    vi.stubGlobal("WebSocket", Socket);
    const config: AgentBridgeConfig = { url: "ws://127.0.0.1:6300/bridge", admissionProof: "session.v1.cancel.proof" };
    const hook = renderHook(() => useAgentBridge({ config }));
    try {
      const toolCall: GatewayToShell = { variant: "agentToolCall", invocationId: cancellationFixture.invocation.id, toolName: cancellationFixture.invocation.toolName, arguments: cancellationFixture.invocation.arguments };
      const wire = encodeGatewayToShell(toolCall);
      act(() => live?.onmessage?.({ data: wire.buffer.slice(wire.byteOffset, wire.byteOffset + wire.byteLength) as ArrayBuffer }));
      expect(hook.result.current.conversation.map((entry) => entry.id)).toEqual([cancellationFixture.invocation.id]);

      let accepted = false;
      act(() => {
        accepted = hook.result.current.cancelToolCall(cancellationFixture.invocation.id);
      });
      expect(accepted).toBe(cancellationFixture.openCancellation.accepted);
      expect(sent.at(-1)).toEqual(cancellationFixture.openCancellation.outbound);
      const entry = hook.result.current.conversation[0]!;
      expect(entry.kind === "toolCall" && entry.state).toBe(cancellationFixture.openCancellation.state);

      // 🛑️ A cancel with nothing open must report that it did not leave, never pretend.
      (live as unknown as { readyState: number }).readyState = 3;
      const beforeClosedCancel = sent.length;
      let refused = true;
      act(() => {
        refused = hook.result.current.cancelToolCall(cancellationFixture.invocation.id);
      });
      expect(refused).toBe(cancellationFixture.closedCancellation.accepted);
      expect(sent.length - beforeClosedCancel).toBe(cancellationFixture.closedCancellation.outboundCount);
      const cancelling = hook.result.current.conversation[0]!;
      expect(cancelling.kind === "toolCall" && cancelling.state).toBe(cancellationFixture.closedCancellation.state);

      const terminal: GatewayToShell = {
        variant: "agentToolResult",
        invocationId: cancellationFixture.invocation.id,
        toolName: cancellationFixture.invocation.toolName,
        ok: cancellationFixture.terminalResult.ok,
        summary: cancellationFixture.terminalResult.summary,
      };
      const terminalWire = encodeGatewayToShell(terminal);
      act(() => live?.onmessage?.({ data: terminalWire.buffer.slice(terminalWire.byteOffset, terminalWire.byteOffset + terminalWire.byteLength) as ArrayBuffer }));
      const settled = hook.result.current.conversation[0]!;
      expect(settled.kind === "toolCall" && settled.state).toBe(cancellationFixture.terminalResult.state);
      expect(settled.kind === "toolCall" && settled.summary).toBe(cancellationFixture.terminalResult.summary);
    } finally {
      hook.unmount();
      vi.unstubAllGlobals();
    }
  });
});
//#endregion 🔖️CancelToolCall

//#region 💬️AgentReply
/** 🧪️ Ticket `26/09/18` slice AC1: the agent's own free-text turn, through the real hook. One turn
 * is ONE row however many chunks it arrives in, the row says it is still arriving until the chunk
 * marked `complete` lands, and a second turn is a second row — nothing here is synthesised. */
describe("useAgentBridge agentReply", () => {
  it("folds every chunk of one turn into one row and settles it on the complete chunk", async () => {
    const { renderHook, act } = await import("@testing-library/react");
    let live: { onmessage: ((event: { data: ArrayBuffer }) => void) | null } | null = null;
    class Socket {
      static OPEN = 1;
      readyState = 1;
      onopen: (() => void) | null = null;
      onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
      onerror: (() => void) | null = null;
      onclose: (() => void) | null = null;
      binaryType = "arraybuffer";
      constructor() {
        live = this as unknown as { onmessage: ((event: { data: ArrayBuffer }) => void) | null };
      }
      send(): void {}
      close(): void {}
    }
    vi.stubGlobal("WebSocket", Socket);
    const config: AgentBridgeConfig = { url: "ws://127.0.0.1:6300/bridge", admissionProof: "session.v1.reply.proof" };
    const hook = renderHook(() => useAgentBridge({ config }));
    const deliver = (frame: GatewayToShell): void => {
      const wire = encodeGatewayToShell(frame);
      act(() => live?.onmessage?.({ data: wire.buffer.slice(wire.byteOffset, wire.byteOffset + wire.byteLength) as ArrayBuffer }));
    };
    try {
      deliver({ variant: "agentReply", replyId: "rep_1", inReplyTo: "msg_1", text: "Widening that wall means", complete: false });
      let entry = hook.result.current.conversation[0]!;
      expect(hook.result.current.conversation).toHaveLength(1);
      expect(entry.kind === "agentMessage" && entry.state).toBe("streaming");
      expect(entry.kind === "agentMessage" && entry.inReplyTo).toBe("msg_1");

      deliver({ variant: "agentReply", replyId: "rep_1", inReplyTo: null, text: " the 300 mm variant.", complete: true });
      expect(hook.result.current.conversation).toHaveLength(1);
      entry = hook.result.current.conversation[0]!;
      expect(entry.kind === "agentMessage" && entry.text).toBe("Widening that wall means the 300 mm variant.");
      expect(entry.kind === "agentMessage" && entry.state).toBe("complete");

      deliver({ variant: "agentReply", replyId: "rep_2", inReplyTo: null, text: "Anything else?", complete: true });
      expect(hook.result.current.conversation.map((row) => row.id)).toEqual(["rep_1", "rep_2"]);
    } finally {
      hook.unmount();
      vi.unstubAllGlobals();
    }
  });
});
//#endregion 💬️AgentReply

//#region 🪦️ApprovalWithdrawal
/** 🪦️ Replays `🌉️mcp/🛡️policy/🧫️fixtures/🪦️approval-withdrawal.json`'s `retiredAffordances` through the
 * real hook over a fake socket: the same frames the gateway sends end every approval as pending,
 * resolved or withdrawn-with-its-reason, and a withdrawn one leaves nothing waiting. */
const withdrawalLaw = JSON.parse(readFileSync(join(here, "../../../../../../🌉️mcp/🛡️policy/🧫️fixtures/🪦️approval-withdrawal.json"), "utf8")) as {
  readonly retiredAffordances: readonly { readonly name: string; readonly frames: readonly Record<string, string>[]; readonly state: string; readonly withdrawal: string | null; readonly pending: number }[];
};

describe("useAgentBridge approval withdrawal", () => {
  for (const row of withdrawalLaw.retiredAffordances) {
    it(`${row.name}: the approval ends ${row.state}${row.withdrawal ? ` (${row.withdrawal})` : ""}`, async () => {
      const { renderHook, act } = await import("@testing-library/react");
      let live: { onmessage: ((event: { data: ArrayBuffer }) => void) | null } | null = null;
      class Socket {
        static OPEN = 1;
        readyState = 1;
        onopen: (() => void) | null = null;
        onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
        onerror: (() => void) | null = null;
        onclose: (() => void) | null = null;
        binaryType = "arraybuffer";
        constructor() {
          live = this as unknown as { onmessage: ((event: { data: ArrayBuffer }) => void) | null };
        }
        send(): void {}
        close(): void {}
      }
      vi.stubGlobal("WebSocket", Socket);
      const config: AgentBridgeConfig = { url: "ws://127.0.0.1:6300/bridge", admissionProof: "session.v1.withdrawal.proof" };
      const hook = renderHook(() => useAgentBridge({ config }));
      try {
        for (const frame of row.frames) {
          const full = { approvalId: "appr_law", ...(frame.variant === "approvalRequested" ? { summary: "{}" } : {}), ...frame } as GatewayToShell;
          const wire = encodeGatewayToShell(full);
          act(() => live?.onmessage?.({ data: wire.buffer.slice(wire.byteOffset, wire.byteOffset + wire.byteLength) as ArrayBuffer }));
        }
        const entry = hook.result.current.conversation.find((candidate) => candidate.id === "appr_law");
        expect(entry?.kind).toBe("approval");
        expect(entry?.kind === "approval" && entry.state).toBe(row.state);
        expect(entry?.kind === "approval" && entry.withdrawal).toBe(row.withdrawal);
        expect(hook.result.current.pendingApprovals).toHaveLength(row.pending);
      } finally {
        hook.unmount();
        vi.unstubAllGlobals();
      }
    });
  }
});
//#endregion 🪦️ApprovalWithdrawal

//#region 🔖️LiveArtifactRoute
/** 🧪️ Ticket `26/09/18` slice LB1: the LIVE artifact route — the seam that makes an MCP client's
 * `action_invoke`/`history_undo`/`transaction_*` execute in THIS shell instead of in the gateway's
 * own headless interpreter. Two banks, one fixture file: these laws read the same
 * `🐚️channel/🧫️fixtures/🗿️app-payloads.json` the Rust side's own tests assert against, so a drift in
 * either codec is red here AND there, never a dropped mutation at runtime. */
const appPayloadFixtures = JSON.parse(readFileSync(join(here, "../../../../../../🌉️mcp/🐚️channel/🧫️fixtures/🗿️app-payloads.json"), "utf8")) as readonly {
  readonly direction: "gateway_to_shell" | "shell_to_gateway";
  readonly kind: string;
  readonly payload: Record<string, unknown>;
}[];

const appPayload = (kind: string): Record<string, unknown> => {
  const row = appPayloadFixtures.find((entry) => entry.kind === kind);
  if (!row) throw new Error(`fixture \`${kind}\` exists`);
  return row.payload;
};

const appPayloadBytes = (kind: string): Uint8Array => new TextEncoder().encode(JSON.stringify(appPayload(kind)));

describe("shell channel payload codec", () => {
  it("round-trips base64 at every length class and refuses malformed text", () => {
    for (let length = 0; length <= 64; length += 1) {
      const bytes = Uint8Array.from({ length }, (_unused, index) => (index * 7) % 251);
      expect(Array.from(decodeChannelBase64(encodeChannelBase64(bytes)) ?? [])).toEqual(Array.from(bytes));
    }
    expect(decodeChannelBase64("AQI")).toBeNull();
    expect(decodeChannelBase64("A===")).toBeNull();
    expect(decodeChannelBase64("AQ=D")).toBeNull();
    expect(decodeChannelBase64("AQ!D")).toBeNull();
  });

  it("decodes every gateway→shell command in the shared fixture", () => {
    expect(decodeShellAppCommand(appPayloadBytes("readHistory"))).toEqual({ kind: "readHistory" });
    expect(decodeShellAppCommand(appPayloadBytes("readArtifact"))).toEqual({ kind: "readArtifact" });
    expect(decodeShellAppCommand(appPayloadBytes("pureCommand"))).toEqual({ kind: "pureCommand", capabilityId: "note.note.appendParagraph", input: { text: "hello" } });
    const prepare = decodeShellAppCommand(appPayloadBytes("transactionPrepare"));
    expect(prepare.kind).toBe("transactionPrepare");
    expect(prepare.kind === "transactionPrepare" && Array.from(prepare.ops.document[0]!)).toEqual([1, 2, 3]);
    expect(prepare.kind === "transactionPrepare" && prepare.origin).toEqual({ kind: "agent", principal: "agent:local", invocationId: "inv_1" });
    expect(decodeShellAppCommand(appPayloadBytes("transactionCommit"))).toEqual({ kind: "transactionCommit", txnId: "txn_1" });
    expect(decodeShellAppCommand(appPayloadBytes("transactionRollback"))).toEqual({ kind: "transactionRollback", txnId: "txn_1" });
    expect(decodeShellAppCommand(appPayloadBytes("transactionUndo"))).toEqual({ kind: "transactionUndo", groupId: "edit_7" });
    expect(decodeShellAppCommand(appPayloadBytes("transactionRedo"))).toEqual({ kind: "transactionRedo", groupId: "edit_7" });
    const exported = decodeShellAppCommand(appPayloadBytes("exportMedia"));
    expect(exported.kind === "exportMedia" && Array.from(exported.document)).toEqual([1, 2, 3, 4]);
  });

  it("encodes every shell→gateway frame back to the shared fixture, byte for byte", () => {
    const frames: readonly ShellAppFrameV1[] = [
      { kind: "historySnapshot", artifactId: "note-1", headEditId: "edit_7", cursor: "7" },
      { kind: "emit", ops: { document: [Uint8Array.from([1, 2, 3])], config: [], draft: [] }, warnings: [] },
      { kind: "transactionPrepared", txnId: "txn_1" },
      { kind: "transactionCommitted", txnId: "txn_1", editId: "edit_8" },
      { kind: "transactionRolledBack", txnId: "txn_1" },
      { kind: "transactionUndone", groupId: "edit_8" },
      { kind: "transactionRedone", groupId: "edit_8" },
      { kind: "artifact", pack: Uint8Array.from([1, 2, 3, 4]), spr: Uint8Array.from([5, 6]) },
      { kind: "exported", port: "pdf", descriptor: Uint8Array.from([1]), data: Uint8Array.from([2, 3]) },
      { kind: "error", code: "mutation.rejected", message: "the document is read-only for this actor" },
    ];
    for (const frame of frames) expect(shellAppFrameToJson(frame)).toEqual(appPayload(frame.kind));
    expect(frames.length).toBe(appPayloadFixtures.filter((row) => row.direction === "shell_to_gateway").length);
  });

  it("refuses a payload from another codec version and an unknown kind, by name", () => {
    const future = new TextEncoder().encode(JSON.stringify({ ...appPayload("transactionCommit"), version: 99 }));
    expect(() => decodeShellAppCommand(future)).toThrow(/99/);
    const unknown = new TextEncoder().encode(JSON.stringify({ ...appPayload("transactionCommit"), kind: "teleport" }));
    expect(() => decodeShellAppCommand(unknown)).toThrow(/teleport/);
  });
});

describe("answerAgentAppCommand", () => {
  it("refuses immediately when no host mounted an artifact route, instead of going silent", async () => {
    const answer = await answerAgentAppCommand({ seq: 7n, instanceId: "inst-1", command: { kind: "readHistory" } }, undefined);
    expect(answer.variant).toBe("appFrames");
    expect(answer.variant === "appFrames" && answer.inReplyTo).toBe(7n);
    expect(answer.variant === "appFrames" && answer.instanceId).toBe("inst-1");
    const decoded = JSON.parse(new TextDecoder().decode((answer as { frames: Uint8Array[] }).frames[0]!)) as Record<string, unknown>;
    expect(decoded.kind).toBe("error");
    expect(decoded.code).toBe("plugin.unavailable");
    expect(String(decoded.message)).toBe(NO_ARTIFACT_ROUTE_MESSAGE);
  });

  it("turns a throwing host handler into a named error frame on the same correlation id", async () => {
    const answer = await answerAgentAppCommand({ seq: 9n, instanceId: "inst-2", command: { kind: "transactionCommit", txnId: "txn_1" } }, () => {
      throw new Error("the active window rejected the dispatch");
    });
    expect(answer.variant === "appFrames" && answer.inReplyTo).toBe(9n);
    const decoded = JSON.parse(new TextDecoder().decode((answer as { frames: Uint8Array[] }).frames[0]!)) as Record<string, unknown>;
    expect(decoded.code).toBe("channel.not-wired");
    expect(decoded.message).toBe("the active window rejected the dispatch");
  });
});

describe("useAgentBridge live artifact route", () => {
  it("declares relayAppCommands only with a handler, publishes its instance census, and answers an inbound appCommand", async () => {
    const { renderHook, act } = await import("@testing-library/react");
    const sent: ShellToGateway[] = [];
    let live: { onopen: (() => void) | null; onmessage: ((event: { data: ArrayBuffer }) => void) | null } | null = null;
    class Socket {
      static OPEN = 1;
      readyState = 1;
      onopen: (() => void) | null = null;
      onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
      onerror: (() => void) | null = null;
      onclose: (() => void) | null = null;
      binaryType = "arraybuffer";
      constructor() {
        live = this as unknown as { onopen: (() => void) | null; onmessage: ((event: { data: ArrayBuffer }) => void) | null };
      }
      send(bytes: Uint8Array): void {
        sent.push(decodeShellToGateway(bytes));
      }
      close(): void {}
    }
    vi.stubGlobal("WebSocket", Socket);
    const config: AgentBridgeConfig = { url: "ws://127.0.0.1:6300/bridge", admissionProof: "session.v1.route.proof" };
    const instances = [{ pluginId: "note", appId: "note", instanceId: "inst-1", artifactRef: "note-1", windowIds: ["note-composite"] }];
    const seen: string[] = [];
    const hook = renderHook(() =>
      useAgentBridge({
        config,
        instances,
        onAppCommand: (request) => {
          seen.push(`${request.instanceId}:${request.command.kind}`);
          return [{ kind: "transactionCommitted", txnId: "txn_1", editId: "edit_8" }];
        },
      }),
    );
    try {
      act(() => live?.onopen?.());
      const hello = sent.find((frame) => frame.variant === "hello");
      expect(hello?.variant === "hello" && hello.flags.relayAppCommands).toBe(true);

      const welcome = encodeGatewayToShell({ variant: "welcome", bridgeVersion: 1, connection: "conn_1", principal: "agent:local" });
      act(() => live?.onmessage?.({ data: welcome.buffer.slice(welcome.byteOffset, welcome.byteOffset + welcome.byteLength) as ArrayBuffer }));
      const census = sent.find((frame) => frame.variant === "instances");
      expect(census?.variant === "instances" && census.entries).toEqual(instances);

      const command = encodeGatewayToShell({ variant: "appCommand", seq: 11n, instanceId: "inst-1", command: appPayloadBytes("transactionCommit") });
      await act(async () => {
        live?.onmessage?.({ data: command.buffer.slice(command.byteOffset, command.byteOffset + command.byteLength) as ArrayBuffer });
        await Promise.resolve();
        await Promise.resolve();
      });
      expect(seen).toEqual(["inst-1:transactionCommit"]);
      const reply = sent.find((frame) => frame.variant === "appFrames");
      expect(reply?.variant === "appFrames" && reply.inReplyTo).toBe(11n);
      const decoded = JSON.parse(new TextDecoder().decode((reply as { frames: Uint8Array[] }).frames[0]!)) as Record<string, unknown>;
      expect(decoded).toEqual(appPayload("transactionCommitted"));
    } finally {
      hook.unmount();
      vi.unstubAllGlobals();
    }
  });

  it("does not claim the relay when no handler is mounted, so the gateway keeps its headless workspace", async () => {
    const { renderHook, act } = await import("@testing-library/react");
    const sent: ShellToGateway[] = [];
    let live: { onopen: (() => void) | null } | null = null;
    class Socket {
      static OPEN = 1;
      readyState = 1;
      onopen: (() => void) | null = null;
      onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
      onerror: (() => void) | null = null;
      onclose: (() => void) | null = null;
      binaryType = "arraybuffer";
      constructor() {
        live = this as unknown as { onopen: (() => void) | null };
      }
      send(bytes: Uint8Array): void {
        sent.push(decodeShellToGateway(bytes));
      }
      close(): void {}
    }
    vi.stubGlobal("WebSocket", Socket);
    const hook = renderHook(() => useAgentBridge({ config: { url: "ws://127.0.0.1:6300/bridge", admissionProof: "session.v1.norelay.proof" } }));
    try {
      act(() => live?.onopen?.());
      const hello = sent.find((frame) => frame.variant === "hello");
      expect(hello?.variant === "hello" && hello.flags.relayAppCommands).toBe(false);
    } finally {
      hook.unmount();
      vi.unstubAllGlobals();
    }
  });
});
//#endregion 🔖️LiveArtifactRoute

//#region 🔖️ArtifactBytesRoute
/** 🧪️ Slice LB1 §11: an agent's `ReadArtifact`/`ExportMedia` against a live editor instance. The
 * host handler is the seam `🏛️ShellHost` fills from the plugin handle's own `readAppDocumentPack`;
 * these laws pin BOTH outcomes the route promises — real bytes for a plugin with a document port,
 * and a refusal that NAMES the missing port for one without. Never silence: a dropped answer costs
 * the agent its whole wall budget and tells it nothing. */
describe("agent artifact-bytes route", () => {
  const decodeFrame = (answer: ShellToGateway): Record<string, unknown> => JSON.parse(new TextDecoder().decode((answer as { frames: Uint8Array[] }).frames[0]!)) as Record<string, unknown>;

  it("answers a ReadArtifact for a live instance with the document's own pack and spr bytes", async () => {
    const pack = Uint8Array.from([1, 2, 3, 4]);
    const spr = Uint8Array.from([5, 6]);
    const answer = await answerAgentAppCommand({ seq: 21n, instanceId: "inst-1", command: { kind: "readArtifact" } }, async () => {
      const document: { readonly pack: Uint8Array; readonly spr: Uint8Array } | null = { pack, spr };
      if (!document) return [shellAppFault("plugin.unavailable", "no document")];
      return [{ kind: "artifact", pack: document.pack, spr: document.spr }];
    });
    expect(answer.variant === "appFrames" && answer.inReplyTo).toBe(21n);
    const decoded = decodeFrame(answer);
    expect(decoded.kind).toBe("artifact");
    expect(decoded.pack).toBe(encodeChannelBase64(pack));
    expect(decoded.spr).toBe(encodeChannelBase64(spr));
    expect(decoded).toEqual(appPayload("artifact"));
  });

  it("refuses by name when the plugin exposes no document port, and when it holds no document", async () => {
    const noPort = await answerAgentAppCommand({ seq: 22n, instanceId: "inst-1", command: { kind: "readArtifact" } }, () => [
      shellAppFault("plugin.unavailable", "plugin `space` exposes no document port in this shell — an agent needing a genesis read must resolve a headless context (`--folder`/`--hub`)"),
    ]);
    const refused = decodeFrame(noPort);
    expect(refused.kind).toBe("error");
    expect(refused.code).toBe("plugin.unavailable");
    expect(String(refused.message)).toContain("no document port");
    expect(String(refused.message)).toContain("space");

    const noDocument = await answerAgentAppCommand({ seq: 23n, instanceId: "inst-1", command: { kind: "readArtifact" } }, () => [shellAppFault("plugin.unavailable", "plugin `note` instance 3 holds no document to read")]);
    expect(String(decodeFrame(noDocument).message)).toContain("holds no document");
  });

  it("refuses an ExportMedia by naming the missing OUT port, never a silent or empty export", async () => {
    const answer = await answerAgentAppCommand({ seq: 24n, instanceId: "inst-1", command: { kind: "exportMedia", port: "pdf", document: new Uint8Array(), documentSpr: new Uint8Array() } }, () => [
      shellAppFault("plugin.unavailable", "this shell has no media OUT port for `pdf` — `PluginWasmHandle` exposes none, and the UI's export path hands the human a download rather than returning bytes; `artifact_export` needs a headless context (`--folder`/`--hub`)"),
    ]);
    const decoded = decodeFrame(answer);
    expect(decoded.kind).toBe("error");
    expect(String(decoded.message)).toContain("media OUT port for `pdf`");
    expect(answer.variant === "appFrames" && answer.instanceId).toBe("inst-1");
  });

  it("encodes a real exported frame back to the shared fixture, so the day a port exists the wire is already pinned", () => {
    expect(shellAppFrameToJson({ kind: "exported", port: "pdf", descriptor: Uint8Array.from([1]), data: Uint8Array.from([2, 3]) })).toEqual(appPayload("exported"));
  });
});
//#endregion 🔖️ArtifactBytesRoute

//#region 🤝️Handshake
/** 🤝️ Ticket `26/09/23` G10 bridge item: every dialled gateway either answers the handshake or is
 * given up on. The scenarios in `🧫️fixtures/🤝️handshake/🔣️.json` (replayed by the wgpu twin's dialer
 * too) pin the bounded ladder: a silent or refusing gateway ends `unavailable` after
 * {@link BRIDGE_UNANSWERED_ATTEMPTS} dials, a typed refusal or a foreign welcome ends the offer at
 * once, and nothing is dialled again nor logged to the console afterwards. */
describe("useAgentBridge handshake", () => {
  it("dials under the fixture's own constants", () => {
    expect([BRIDGE_HANDSHAKE_DEADLINE_MS, BRIDGE_UNANSWERED_ATTEMPTS, BRIDGE_VERSION]).toEqual([handshakeFixture.handshakeDeadlineMs, handshakeFixture.unansweredAttempts, handshakeFixture.shellVersion]);
  });

  for (const scenario of handshakeFixture.scenarios) {
    it(scenario.name, async () => {
      const { renderHook, act } = await import("@testing-library/react");
      vi.useFakeTimers();
      const consoleError = vi.spyOn(console, "error");
      const consoleWarn = vi.spyOn(console, "warn");
      const sockets: Socket[] = [];
      class Socket {
        static CONNECTING = 0;
        static OPEN = 1;
        static CLOSED = 3;
        readyState = 0;
        onopen: (() => void) | null = null;
        onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
        onerror: (() => void) | null = null;
        onclose: (() => void) | null = null;
        binaryType = "arraybuffer";
        constructor() {
          sockets.push(this);
        }
        send(): void {}
        close(): void {
          if (this.readyState === 3) return;
          this.readyState = 3;
          setTimeout(() => this.onclose?.(), 0);
        }
        drop(): void {
          this.readyState = 3;
          this.onerror?.();
          this.onclose?.();
        }
      }
      vi.stubGlobal("WebSocket", Socket);
      const config: AgentBridgeConfig = { url: "ws://127.0.0.1:6300/bridge", admissionProof: "session.v1.handshake.proof" };
      const hook = renderHook(() => useAgentBridge({ config }));
      try {
        for (const [index, dial] of scenario.dials.entries()) {
          while (sockets.length <= index) await act(async () => void (await vi.advanceTimersToNextTimerAsync()));
          const socket = sockets[index]!;
          if (dial === "close") {
            act(() => socket.drop());
            continue;
          }
          act(() => {
            socket.readyState = 1;
            socket.onopen?.();
          });
          if (dial === "silence") {
            await act(async () => void (await vi.advanceTimersByTimeAsync(handshakeFixture.handshakeDeadlineMs + 1)));
            continue;
          }
          const wire = encodeGatewayToShell(dial.frame);
          act(() => socket.onmessage?.({ data: wire.buffer.slice(wire.byteOffset, wire.byteOffset + wire.byteLength) as ArrayBuffer }));
          if (dial.thenClose) act(() => socket.drop());
        }
        await act(async () => void (await vi.advanceTimersByTimeAsync(10 * 30_000)));
        expect(sockets.length).toBe(scenario.dials.length);
        expect(hook.result.current.status).toBe(scenario.status);
        expect(hook.result.current.versionMismatch).toEqual(scenario.versionMismatch);
        expect(consoleError).not.toHaveBeenCalled();
        expect(consoleWarn).not.toHaveBeenCalled();
      } finally {
        hook.unmount();
        consoleError.mockRestore();
        consoleWarn.mockRestore();
        vi.useRealTimers();
        vi.unstubAllGlobals();
      }
    });
  }

  it("a different offer restarts the ladder after a terminal state", async () => {
    const { renderHook, act } = await import("@testing-library/react");
    const sockets: { onopen: (() => void) | null; onmessage: ((event: { data: ArrayBuffer }) => void) | null; readyState: number }[] = [];
    class Socket {
      static OPEN = 1;
      readyState = 0;
      onopen: (() => void) | null = null;
      onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
      onerror: (() => void) | null = null;
      onclose: (() => void) | null = null;
      binaryType = "arraybuffer";
      constructor() {
        sockets.push(this);
      }
      send(): void {}
      close(): void {}
    }
    vi.stubGlobal("WebSocket", Socket);
    const refused = encodeGatewayToShell({ variant: "refused", reason: "version", gatewayVersion: 2 });
    const initialProps = { config: { url: "ws://127.0.0.1:6300/bridge", admissionProof: "session.v1.old" } as AgentBridgeConfig };
    const hook = renderHook(({ config }) => useAgentBridge({ config }), { initialProps });
    try {
      act(() => sockets[0]!.onmessage?.({ data: refused.buffer.slice(refused.byteOffset, refused.byteOffset + refused.byteLength) as ArrayBuffer }));
      expect(hook.result.current.status).toBe("incompatible");
      hook.rerender({ config: { url: "ws://127.0.0.1:6301/bridge", admissionProof: "session.v1.new" } });
      expect(sockets.length).toBe(2);
      expect(hook.result.current.status).toBe("connecting");
      expect(hook.result.current.versionMismatch).toBeNull();
    } finally {
      hook.unmount();
      vi.unstubAllGlobals();
    }
  });
});
//#endregion 🤝️Handshake
