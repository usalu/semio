/** 🧪️ Legacy-era conformance suite (packet `P5-conformance-tests`, brief §3.2) — drives the REAL
 * compiled `semio-os-mcp stdio` binary with the REAL installed `@modelcontextprotocol/sdk` (1.30.0,
 * `LATEST_PROTOCOL_VERSION = '2025-11-25'`) `Client` + `StdioClientTransport`. `📓️terra-P1a-report.md`
 * proves the server agrees with itself via its own Rust unit tests; this suite is the independent
 * proof that a real MCP client library — the same one every IDE extension in this repo embeds —
 * can actually hold a conversation with it.
 *
 * Skips with a clear console message (never silently green) when the binary hasn't been built yet
 * — see `resolveMcpBinaryPath` in `../../🟦️component.ts`. Build it first:
 * `CARGO_TARGET_DIR=<ticket>/🎯️target cargo build -p semio-framework-os-mcp --bin semio-os-mcp`.
 *
 * ⚠️ `tools/call` on an unregistered tool: the shipped `run_stdio` boots `McpServer::with_defaults()`
 * — EMPTY tool/resource/prompt registries (`🦀️component.rs` `run_stdio`, confirmed live below and in
 * `🧪️p5-server-probe.txt`). `InMemoryToolRegistry::call` therefore ALWAYS takes the "unknown tool" =
 * protocol-error branch (`🧭️protocol/🦀️component.rs` `calling_an_unregistered_tool_is_a_protocol_error`)
 * — the OTHER half of the distinction brief §3.2 asks us to prove (`isError:true` for a REGISTERED
 * tool's own business failure) is only exercised by P1a's own Rust unit test
 * (`a_registered_tool_reporting_failure_is_a_successful_response_with_is_error_true`) today; it is
 * NOT independently observable through this black-box suite until a downstream packet (P2/P6)
 * registers at least one real tool. Flagged loudly here and in `📓️terra-P5-report.md`, per brief §2
 * — not weakened into a false positive. */
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";
import { LATEST_PROTOCOL_VERSION, McpError } from "@modelcontextprotocol/sdk/types.js";
import { existsSync } from "node:fs";
import { afterAll, beforeAll, describe, expect, it } from "vitest";
import { resolveMcpBinaryPath, spawnRawMcp } from "../../🟦️component.ts";
import { isValidJsonSchema2020_12 } from "./🧬️schema-validation.ts";
import { getWorkspaceRoot } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

const repoRoot = getWorkspaceRoot();
const bin = resolveMcpBinaryPath(repoRoot);
const BIN_PRESENT = existsSync(bin);
const TOOL_NAME_RE = /^[a-zA-Z0-9_-]{1,64}$/;

if (!BIN_PRESENT) {
  console.warn(`[@semio-tech/framework-os-mcp] legacy conformance suite SKIPPED — binary not found at ${bin}. Build it first: CARGO_TARGET_DIR=<ticket>/🎯️target cargo build -p semio-framework-os-mcp --bin semio-os-mcp`);
}

describe.skipIf(!BIN_PRESENT)("semio-os-mcp — legacy era (@modelcontextprotocol/sdk 1.30.0)", () => {
  //#region 🔖️RawHandshake
  // The `Client` class never exposes the negotiated `protocolVersion` string it received (only
  // `getServerCapabilities()`/`getServerVersion()`/`getInstructions()` survive `connect()` —
  // confirmed against `dist/esm/client/index.d.ts`), so this one field is asserted directly off the
  // wire, sending the EXACT body `Client.connect()` sends (same method, same
  // `LATEST_PROTOCOL_VERSION` constant imported from the SDK itself, not re-typed by hand).
  it("initialize succeeds and negotiates protocolVersion 2025-11-25", async () => {
    const raw = spawnRawMcp(bin);
    try {
      const response = await raw.request("initialize", { protocolVersion: LATEST_PROTOCOL_VERSION, capabilities: {}, clientInfo: { name: "semio-os-mcp-conformance-raw", version: "0.1.0" } });
      expect(response.error).toBeUndefined();
      const result = response.result as { protocolVersion: string; serverInfo: { name: string }; capabilities: Record<string, unknown> };
      expect(result.protocolVersion).toBe("2025-11-25");
      expect(result.serverInfo.name).toBe("semio-os-mcp");
      expect(result.capabilities).toMatchObject({ tools: { listChanged: true }, resources: { listChanged: true, subscribe: true }, prompts: { listChanged: true } });
    } finally {
      await raw.close();
    }
  });
  //#endregion 🔖️RawHandshake

  //#region 🔖️SdkClient
  let client: Client;
  let transport: StdioClientTransport;

  beforeAll(async () => {
    transport = new StdioClientTransport({ command: bin, args: ["stdio"], stderr: "pipe" });
    client = new Client({ name: "semio-os-mcp-conformance", version: "0.1.0" }, { capabilities: {} });
    await client.connect(transport);
  });

  afterAll(async () => {
    await client.close();
  });

  it("serverInfo.name and declared capabilities match after a real SDK handshake", () => {
    expect(client.getServerVersion()?.name).toBe("semio-os-mcp");
    expect(client.getServerCapabilities()).toMatchObject({ tools: { listChanged: true }, resources: { listChanged: true, subscribe: true }, prompts: { listChanged: true } });
  });

  it("tools/list is schema-valid, every name matches ^[a-zA-Z0-9_-]{1,64}$, byte-identical across two calls", async () => {
    const first = await client.listTools();
    const second = await client.listTools();
    expect(JSON.stringify(first.tools)).toBe(JSON.stringify(second.tools));
    for (const tool of first.tools) {
      expect(tool.name).toMatch(TOOL_NAME_RE);
      const inputCheck = isValidJsonSchema2020_12(tool.inputSchema);
      expect(inputCheck.valid, `tool ${tool.name} inputSchema: ${!inputCheck.valid && inputCheck.error}`).toBe(true);
      if (tool.outputSchema !== undefined) {
        const outputCheck = isValidJsonSchema2020_12(tool.outputSchema);
        expect(outputCheck.valid, `tool ${tool.name} outputSchema: ${!outputCheck.valid && outputCheck.error}`).toBe(true);
      }
    }
    // With `McpServer::with_defaults()` (zero registered tools today) this loop is vacuously true —
    // it activates for real once P2/P6 register the catalog; kept unconditional so it starts
    // asserting automatically rather than needing a follow-up edit.
  });

  it("resources/list and resources/templates/list return schema-shaped (possibly empty) arrays", async () => {
    const resources = await client.listResources();
    const templates = await client.listResourceTemplates();
    expect(Array.isArray(resources.resources)).toBe(true);
    expect(Array.isArray(templates.resourceTemplates)).toBe(true);
  });

  it("resources/read on an unresolvable URI returns a well-formed MCP error", async () => {
    let threw = false;
    try {
      await client.readResource({ uri: "semio://capability/does-not-exist" });
    } catch (error) {
      threw = true;
      expect(error).toBeInstanceOf(McpError);
      const mcpError = error as InstanceType<typeof McpError>;
      expect(typeof mcpError.code).toBe("number");
      expect(mcpError.code).toBeLessThan(0);
      expect(typeof mcpError.message).toBe("string");
      expect(mcpError.data).toMatchObject({ gatewayCode: "NOT_FOUND" });
    }
    expect(threw, "an unresolvable resource URI must reject, not resolve").toBe(true);
  });

  it("prompts/list serves the real registered prompts and prompts/get on an unknown name is a well-formed MCP error", async () => {
    const prompts = await client.listPrompts();
    expect(Array.isArray(prompts.prompts)).toBe(true);
    // 🎫️ 26/08/29/AI-MCP-END-TO-END registered the first real prompts; this assertion used to read
    // "empty (no registrations yet)" in its NAME while only ever checking the array shape, so it
    // would have kept passing either way. It now actually pins the registered set.
    expect(prompts.prompts.length).toBeGreaterThan(0);
    for (const prompt of prompts.prompts) expect(prompt.name).toMatch(/^[a-z][a-z0-9_]*$/);
    await expect(client.getPrompt({ name: "does-not-exist" })).rejects.toBeInstanceOf(McpError);
  });

  it("tools/call on an unregistered tool is a genuine JSON-RPC protocol error, never a resolved isError:true result", async () => {
    let threw = false;
    try {
      await client.callTool({ name: "definitely_not_a_registered_tool", arguments: {} });
    } catch (error) {
      threw = true;
      expect(error).toBeInstanceOf(McpError);
      const mcpError = error as InstanceType<typeof McpError>;
      expect(typeof mcpError.code).toBe("number");
      expect(mcpError.data).toMatchObject({ gatewayCode: "NOT_FOUND" });
    }
    expect(threw, "an unregistered tool name must reject the request, not resolve with isError:true").toBe(true);
  });

  it("ping succeeds", async () => {
    await expect(client.ping()).resolves.toBeDefined();
  });
  //#endregion 🔖️SdkClient
});
