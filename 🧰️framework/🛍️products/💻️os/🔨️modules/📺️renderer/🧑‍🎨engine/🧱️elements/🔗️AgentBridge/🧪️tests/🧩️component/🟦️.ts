// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/AgentBridge/component.test.ts
/** 🧪️ Registered AgentBridge frame parity, protected configuration and exact Shell state laws. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi } from "vitest";
import { type AgentBridgeConfig, useAgentBridge, applyInboundShellCommand, bridgeProtocols, buildShellStateFrame, createDefaultShellState, decodeJsonPayload, discoverAgentBridgeConfig, encodeJsonPayload } from "../../🟦️.tsx";
import { bytesToHex, decodeShellToGateway, decodeGatewayToShell, encodeShellToGateway, encodeGatewayToShell, type GatewayToShell, type ShellToGateway } from "../../../../../../🌉️mcp/🧵️bridge/🟦️.ts";
// #endregion 🔌️Adapters

const here = dirname(fileURLToPath(import.meta.url));

//#region 🔖️ConfigDiscovery
describe("discoverAgentBridgeConfig", () => {
  it("returns null when neither env var is set", () => {
    expect(discoverAgentBridgeConfig({})).toBeNull();
  });

  it("rejects an environment URL without an in-memory admission proof", () => {
    expect(discoverAgentBridgeConfig({ VITE_SEMIO_BRIDGE_URL: "ws://127.0.0.1:6300/bridge" })).toBeNull();
  });

  it("rejects a poisoned Vite bridge credential", () => {
    expect(discoverAgentBridgeConfig({ VITE_SEMIO_BRIDGE_URL: "ws://127.0.0.1:6300/bridge?token=secret", VITE_SEMIO_BRIDGE_TOKEN: "secret" })).toBeNull();
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
  const clone: Record<string, unknown> = Array.isArray(value) ? [...(value as unknown[])] : { ...(value as Record<string, unknown>) };
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
    console.log("[DEBUG] AgentBridge inference default/command parity: neutral=1 equality=2");
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
