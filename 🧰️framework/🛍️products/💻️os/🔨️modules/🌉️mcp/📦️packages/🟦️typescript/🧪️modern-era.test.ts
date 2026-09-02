/** 🧪️ Modern-era suite (packet `P5-conformance-tests`, brief §3.3) — the installed
 * `@modelcontextprotocol/sdk` is legacy-only (`LATEST_PROTOCOL_VERSION = '2025-11-25'`,
 * `📓️design-decisions.md` D1's own SDK survey), so it cannot send a per-request `_meta` modern
 * request or open a connection with no `initialize` at all. This suite drives the real binary with
 * the hand-rolled raw newline-delimited JSON-RPC client (`spawnRawMcp` in `../../🟦️.ts`)
 * to independently prove the `2026-07-28` half of D1's dual-era contract — the spec's own words,
 * fetched 2026-08-17: *"There is no negotiation handshake. Every request carries its protocol
 * version."*
 *
 * `server/discover`'s success response carries only the ONE negotiated `protocolVersion` plus
 * `capabilities`/`serverInfo` (`🧭️protocol/🦀️.rs` `handle_server_discover`) — matching
 * `📓️luna-mcpspec-audit.md`'s own audited response shape (`{resultType, protocolVersion,
 * capabilities, serverInfo, _meta?}`) exactly, NOT a bug. The full supported-version SET is exposed
 * authoritatively via the `-32022` error's `data.supported` array instead — asserted below. */
import { existsSync } from "node:fs";
import { beforeEach, describe, expect, it } from "vitest";
import { resolveMcpBinaryPath, spawnRawMcp, type RawMcpProcess } from "../../🟦️.ts";
import { getWorkspaceRoot } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();
const bin = resolveMcpBinaryPath(repoRoot);
const BIN_PRESENT = existsSync(bin);
const META_KEY = "io.modelcontextprotocol/protocolVersion";
const MODERN_VERSION = "2026-07-28";
const SUPPORTED = [MODERN_VERSION, "2025-11-25", "2025-06-18"];

if (!BIN_PRESENT) {
  console.warn(`[@semio-tech/framework-os-mcp] modern-era suite SKIPPED — binary not found at ${bin}. Build it first: CARGO_TARGET_DIR=<ticket>/🎯️target cargo build -p semio-framework-os-mcp --bin semio-os-mcp`);
}

describe.skipIf(!BIN_PRESENT)("semio-os-mcp — modern era (2026-07-28, raw JSON-RPC, no SDK)", () => {
  let procs: RawMcpProcess[] = [];

  beforeEach(() => {
    procs = [];
  });

  const freshProcess = (): RawMcpProcess => {
    const proc = spawnRawMcp(bin);
    procs.push(proc);
    return proc;
  };

  const teardown = async (): Promise<void> => {
    await Promise.all(procs.map((proc) => proc.close()));
  };

  it("server/discover with no _meta negotiates the newest supported version and returns capabilities + serverInfo", async () => {
    const proc = freshProcess();
    try {
      const response = await proc.request("server/discover", {});
      expect(response.error).toBeUndefined();
      const result = response.result as { protocolVersion: string; capabilities: Record<string, unknown>; serverInfo: { name: string } };
      expect(result.protocolVersion).toBe(MODERN_VERSION);
      expect(result.serverInfo.name).toBe("semio-os-mcp");
      expect(result.capabilities).toMatchObject({ tools: { listChanged: true }, resources: { listChanged: true, subscribe: true }, prompts: { listChanged: true } });
    } finally {
      await teardown();
    }
  });

  it("server/discover with an explicit supported _meta version negotiates exactly that version", async () => {
    const proc = freshProcess();
    try {
      const response = await proc.request("server/discover", { _meta: { [META_KEY]: "2025-11-25" } });
      const result = response.result as { protocolVersion: string };
      expect(result.protocolVersion).toBe("2025-11-25");
    } finally {
      await teardown();
    }
  });

  it("an unsupported _meta version on server/discover yields -32022 with data.supported", async () => {
    const proc = freshProcess();
    try {
      const response = await proc.request("server/discover", { _meta: { [META_KEY]: "1999-01-01" } });
      expect(response.result).toBeUndefined();
      expect(response.error?.code).toBe(-32022);
      expect(response.error?.data).toMatchObject({ requested: "1999-01-01", supported: SUPPORTED });
    } finally {
      await teardown();
    }
  });

  it("a _meta-tagged request is served statelessly with NO initialize, on a completely fresh process with no prior handshake", async () => {
    const proc = freshProcess();
    try {
      // The FIRST line this process ever receives is a plain capability call, not server/discover
      // and not initialize — proving the spec's "no negotiation handshake" contract for real.
      const response = await proc.request("tools/list", { _meta: { [META_KEY]: MODERN_VERSION } });
      expect(response.error).toBeUndefined();
      const result = response.result as { tools: unknown[] };
      expect(Array.isArray(result.tools)).toBe(true);
    } finally {
      await teardown();
    }
  });

  it("an unsupported _meta version on an ordinary method (not just server/discover) yields -32022 with data.supported", async () => {
    const proc = freshProcess();
    try {
      const response = await proc.request("tools/list", { _meta: { [META_KEY]: "1999-01-01" } });
      expect(response.result).toBeUndefined();
      expect(response.error?.code).toBe(-32022);
      expect(response.error?.data).toMatchObject({ requested: "1999-01-01", supported: SUPPORTED });
    } finally {
      await teardown();
    }
  });

  it("two independent fresh processes each serve a modern request with no shared state (truly stateless)", async () => {
    const procA = freshProcess();
    const procB = freshProcess();
    try {
      const [responseA, responseB] = await Promise.all([procA.request("ping", { _meta: { [META_KEY]: MODERN_VERSION } }), procB.request("resources/list", { _meta: { [META_KEY]: MODERN_VERSION } })]);
      expect(responseA.error).toBeUndefined();
      expect(responseB.error).toBeUndefined();
    } finally {
      await teardown();
    }
  });
});
