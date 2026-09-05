/** 🧪️ MCP ↔ hub GIS Map inference bridge conformance (ticket `26/09/02/COMPLETE-SEMIO-END-TO-END`,
 * lane `fable-mcp-inference-bridge`).
 *
 * Two halves, deliberately separable so the gate can run the cheap one alone:
 *
 * 1. the **source oracle** — `proveMcpInferenceBridgeFixture` (`💡️inference-bridge.ts`), a real
 *    third-party AJV 2020-12 pass over the shared neutral fixture
 *    `🌎️hub/🧪️fixtures/🗳️gis-map-proposal-approval-v1`, the four closed wire shapes with their
 *    hostile mutations, and a cross-check of the client's four path builders against the routes the
 *    hub binary actually registers;
 * 2. the **process laws** — the REAL `semio-os-mcp` binary driven over raw stdio JSON-RPC.
 *
 * Explicit nonclaims, restated because they are easy to over-read: no external model provider is
 * involved anywhere; no WGPU or browser rendering is implicated; no two-user process journey is run
 * here; and nothing here asserts that a live hub inference job ran — with no trusted GIS Map
 * binding a hub answers `503 inference.unavailable`, and that is the honest end of the chain. */
import { afterEach, describe, expect, it } from "vitest";
import { INFERENCE_JOB_TOOLS, proveMcpInferenceBridgeFixture } from "./💡️inference-bridge.ts";
import { isValidJsonSchema2020_12 } from "./🧬️schema-validation.ts";
import { requireMcpBinary, spawnRawMcp, type RawMcpProcess } from "../../🟦️.ts";
import { getWorkspaceRoot } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();
const bin = requireMcpBinary(repoRoot);

type CallToolResult = { isError?: boolean; structuredContent?: Record<string, unknown>; content?: Array<{ type: string; text?: string }> };
type ToolListResult = { tools: Array<{ name: string; description?: string; inputSchema: Record<string, unknown>; outputSchema?: Record<string, unknown> }> };

describe("gis map inference bridge — neutral fixture and closed wire shapes", () => {
  it("the independent AJV oracle passes every fixture, wire-shape, hostile and route law", () => {
    const report = proveMcpInferenceBridgeFixture(repoRoot);
    expect(report.ajv).toBe(8);
    expect(report.errors).toBe(11);
    expect(report.visibility).toBe(7);
    expect(report.lifecycle).toBe(9);
    expect(report.routes).toBe(4);
    expect(report.limits).toBe(4);
    expect(report.hostile).toBeGreaterThanOrEqual(26);
  });
});

describe("gis map inference bridge — the real semio-os-mcp binary", () => {
  const procs: RawMcpProcess[] = [];

  /** 🚀️ A freshly spawned, already-discovered server — `server/discover` is the modern era's opener,
   * exactly as the sibling end-to-end suite drives it. */
  const spawn = async (args: readonly string[]): Promise<RawMcpProcess> => {
    const proc = spawnRawMcp(bin, args);
    procs.push(proc);
    const discovered = await proc.request("server/discover", {});
    expect(discovered.error, `server/discover failed: ${JSON.stringify(discovered.error)}`).toBeUndefined();
    return proc;
  };

  afterEach(async () => {
    await Promise.all(procs.splice(0).map((proc) => proc.close()));
  });

  it("tools/list carries the four inference job tools with valid 2020-12 schemas and bilingual descriptions", async () => {
    const proc = await spawn(["stdio"]);
    const listed = (await proc.request("tools/list", {})).result as ToolListResult;
    for (const name of INFERENCE_JOB_TOOLS) {
      const tool = listed.tools.find((candidate) => candidate.name === name);
      expect(tool, `${name} is missing from tools/list`).toBeDefined();
      expect(isValidJsonSchema2020_12(tool!.inputSchema), `${name} input schema`).toEqual({ valid: true });
      expect(tool!.inputSchema["type"]).toBe("object");
      const description = tool!.description ?? "";
      const halves = description.split(" — ");
      expect(halves.length, `${name} is not bilingual: ${description}`).toBe(2);
      expect(halves[0].length).toBeGreaterThan(40);
      expect(halves[1].length).toBeGreaterThan(40);
      expect(description.toLowerCase()).not.toMatch(/not yet implemented|not implemented yet/);
    }
  });

  it("without the granted scope every inference job tool is a local PERMISSION_DENIED before any hub call", async () => {
    const proc = await spawn(["stdio"]);
    for (const name of INFERENCE_JOB_TOOLS) {
      const result = (await proc.request("tools/call", { name, arguments: {} })).result as CallToolResult;
      expect(result.isError, `${name} must not fabricate a success`).toBe(true);
      expect(result.structuredContent?.["code"], `${name}: ${JSON.stringify(result.structuredContent)}`).toBe("PERMISSION_DENIED");
    }
  });

  it("with the scope granted but no hub binding, every inference job tool answers a retryable PLUGIN_UNAVAILABLE naming --hub", async () => {
    const proc = await spawn(["stdio", "--scopes", "inference.execute"]);
    for (const name of INFERENCE_JOB_TOOLS) {
      const args: Record<string, unknown> = name === "inference_submit" ? { documentId: "doc-alpha" } : name === "inference_approve" ? { jobHandle: "job_none", proposalHash: "a".repeat(64) } : { jobHandle: "job_none" };
      const result = (await proc.request("tools/call", { name, arguments: args })).result as CallToolResult;
      expect(result.isError, `${name} must not fabricate a success`).toBe(true);
      expect(result.structuredContent?.["code"], `${name}: ${JSON.stringify(result.structuredContent)}`).toBe("PLUGIN_UNAVAILABLE");
      expect(result.structuredContent?.["retryable"]).toBe(true);
      expect(String(result.structuredContent?.["message"] ?? "")).toContain("--hub");
    }
  });

  it("a malformed inference job call is INPUT_INVALID, never a protocol error and never a fabricated job", async () => {
    const proc = await spawn(["stdio", "--scopes", "inference.execute"]);
    const missingDocument = (await proc.request("tools/call", { name: "inference_submit", arguments: {} })).result as CallToolResult;
    expect(missingDocument.isError).toBe(true);
    expect(missingDocument.structuredContent?.["code"]).toBe("INPUT_INVALID");
    const missingHandle = (await proc.request("tools/call", { name: "inference_events", arguments: {} })).result as CallToolResult;
    expect(missingHandle.isError).toBe(true);
    expect(missingHandle.structuredContent?.["code"]).toBe("INPUT_INVALID");
  });

  it("a folder-bound workspace still has no inference authority and says so instead of inventing one", async () => {
    const proc = await spawn(["stdio", "--scopes", "inference.execute", "--folder", repoRoot]);
    const result = (await proc.request("tools/call", { name: "inference_submit", arguments: { documentId: "doc-alpha" } })).result as CallToolResult;
    expect(result.isError).toBe(true);
    expect(result.structuredContent?.["code"]).toBe("PLUGIN_UNAVAILABLE");
    expect(String(result.structuredContent?.["message"] ?? "")).toContain("--hub");
  });
});
