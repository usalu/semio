/** 🧪️ Checks the published stdio binary's 2025-11-25 protocol with the MCP TypeScript SDK.
 * The suite covers discovery, JSON-RPC errors, and structured failures from registered tools. */
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";
import { LATEST_PROTOCOL_VERSION, McpError } from "@modelcontextprotocol/sdk/types.js";
import { afterAll, beforeAll, describe, expect, it } from "vitest";
import { spawnRawMcp, withMcpHostDriverPolicy } from "../../🟦️.ts";
import { requireMcpBinary } from "../../📦️binary/🟦️.ts";
import { isValidJsonSchema2020_12 } from "../../🧬️schema-validation/🟦️.ts";
import { findWorkspaceRoot } from "@semio-tech/process/routing";

const repoRoot = findWorkspaceRoot();
const bin = requireMcpBinary(repoRoot);
const TOOL_NAME_RE = /^[a-zA-Z0-9_-]{1,64}$/;

describe("semio-os-mcp — legacy era (@modelcontextprotocol/sdk 1.30.0)", () => {
  //#region 🔖️RawHandshake
  it("initialize succeeds and negotiates protocolVersion 2025-11-25", async () => {
    const raw = spawnRawMcp(bin);
    try {
      const response = await raw.request("initialize", { protocolVersion: LATEST_PROTOCOL_VERSION, capabilities: {}, clientInfo: { name: "semio-os-mcp-conformance-raw", version: "0.1.0" } });
      expect(response.error).toBeUndefined();
      const result = response.result as { protocolVersion: string; serverInfo: { name: string }; capabilities: Record<string, unknown> };
      expect(result.protocolVersion).toBe("2025-11-25");
      expect(result.serverInfo.name).toBe("semio");
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
    transport = new StdioClientTransport({ command: bin, args: withMcpHostDriverPolicy(["stdio", "--no-bridge"]), stderr: "inherit" });
    client = new Client({ name: "semio-os-mcp-conformance", version: "0.1.0" }, { capabilities: {} });
    await client.connect(transport);
  });

  afterAll(async () => {
    await client.close();
  });

  it("serverInfo.name and declared capabilities match after a real SDK handshake", () => {
    expect(client.getServerVersion()?.name).toBe("semio");
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

  it("a registered tool's typed error reaches the SDK client as isError:true, never as an output-schema exception", async () => {
    const tools = (await client.listTools()).tools;
    const withOutputSchema = tools.filter((tool) => tool.outputSchema !== undefined).map((tool) => tool.name);
    expect(withOutputSchema, "the tools that answer structured content declare it").toEqual(expect.arrayContaining(["action_invoke", "artifact_snapshot", "artifact_open"]));
    for (const name of ["action_invoke", "artifact_snapshot", "artifact_open"]) {
      const result = await client.callTool({ name, arguments: name === "action_invoke" ? { capabilityId: "no.such.capability", input: {} } : { artifactId: "no-such-artifact" } });
      expect(result.isError, `${name} answers its own failure as a tool result`).toBe(true);
      expect((result.structuredContent as { code?: string } | undefined)?.code, `${name} carries the typed GatewayError`).toMatch(/^[A-Z_]+$/);
    }
  });

  it("ping succeeds", async () => {
    await expect(client.ping()).resolves.toBeDefined();
  });
  //#endregion 🔖️SdkClient
});
