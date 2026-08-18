// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/AgentBridge/component.test.ts
/** @emoji 🧪️ Pure-logic tests for `AgentBridge`: bridge-frame codec round-trip against P1b's own
 * Rust↔TS parity fixtures, `reduce()` application to an inbound `ShellCommand` frame, and
 * config/URL discovery. Not wired into `@semio-tech/framework-renderer-react`'s nx `test` target
 * (its `vitest.config.ts` `root` is the `⚛️react` package dir, which does not reach into
 * `🧱️elements/**` — see `.🧬semio/…/📓️terra-P10-report.md` §"Acceptance" for the direct foreground
 * `vitest run` invocation that verifies this file instead of leasing that config).
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { applyInboundShellCommand, bridgeUrlWithToken, buildShellStateFrame, createDefaultShellState, decodeJsonPayload, discoverAgentBridgeConfig, encodeJsonPayload } from "./🟦️component.tsx";
import { bytesToHex, decodeShellToGateway, decodeGatewayToShell, encodeShellToGateway, encodeGatewayToShell, type GatewayToShell, type ShellToGateway } from "../../../../🌉️mcp/🧵️bridge/🟦️component.ts";
// #endregion 🔌️Adapters

const here = dirname(fileURLToPath(import.meta.url));

//#region 🔖️ConfigDiscovery
describe("discoverAgentBridgeConfig", () => {
  it("returns null when neither env var is set", () => {
    expect(discoverAgentBridgeConfig({})).toBeNull();
  });

  it("returns null when only one env var is set", () => {
    expect(discoverAgentBridgeConfig({ VITE_SEMIO_BRIDGE_URL: "ws://127.0.0.1:6300/bridge" })).toBeNull();
  });

  it("returns a config when both env vars are set", () => {
    const config = discoverAgentBridgeConfig({ VITE_SEMIO_BRIDGE_URL: "ws://127.0.0.1:6300/bridge", VITE_SEMIO_BRIDGE_TOKEN: "secret" });
    expect(config).toEqual({ url: "ws://127.0.0.1:6300/bridge", token: "secret" });
  });
});

describe("bridgeUrlWithToken", () => {
  it("appends ?token= when the url has no query", () => {
    expect(bridgeUrlWithToken({ url: "ws://127.0.0.1:6300/bridge", token: "abc" })).toBe("ws://127.0.0.1:6300/bridge?token=abc");
  });

  it("appends &token= when the url already has a query", () => {
    expect(bridgeUrlWithToken({ url: "ws://127.0.0.1:6300/bridge?shell=react", token: "abc" })).toBe("ws://127.0.0.1:6300/bridge?shell=react&token=abc");
  });

  it("percent-encodes the token", () => {
    expect(bridgeUrlWithToken({ url: "ws://127.0.0.1:6300/bridge", token: "a b/c" })).toBe("ws://127.0.0.1:6300/bridge?token=a%20b%2Fc");
  });
});
//#endregion 🔖️ConfigDiscovery

//#region 🔖️CodecParity
type FixtureRow = { readonly direction: "shell_to_gateway" | "gateway_to_shell"; readonly variant: string; readonly frame: unknown; readonly hex: string };

function loadFixtures(): readonly FixtureRow[] {
  const path = join(here, "..", "..", "..", "..", "🌉️mcp", "🧵️bridge", "🧫️fixtures", "frames.json");
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
