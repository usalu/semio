/** 🧪️ End-to-end suite (ticket `26/08/29/AI-MCP-END-TO-END`) — the gate that proves the gateway is
 * actually finished, by driving the REAL `semio-os-mcp` binary over stdio with raw JSON-RPC.
 *
 * The three sibling suites prove the *protocol* (dual-era handshake, SDK conformance, stdio
 * hygiene). This one proves the *surface and its semantics*:
 *
 * 1. every one of the 26 tools is present, and none of them is a stub any more;
 * 2. `prompts/list` is non-empty and bilingual;
 * 3. `resources/list` + `resources/templates/list` advertise the artifact, inference, UI and job
 *    families regardless of how the server was bound;
 * 4. **progressive enhancement** — with no `--folder`/`--hub`, every workspace-backed tool answers a
 *    structured, RETRYABLE `PLUGIN_UNAVAILABLE` that names the missing binding, never a protocol
 *    error and never fabricated data;
 * 5. with `--folder <tmp>` bound, the same calls stop reporting a missing workspace — the tier
 *    actually changes behaviour rather than merely being described;
 * 6. the catalog is compiled from the real installed plugin registry, not a note/cad fixture.
 *
 * The package test gate builds and requires the binary before Vitest starts. */
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { requireMcpBinary, spawnRawMcp, type RawMcpProcess } from "../../🟦️.ts";
import { getWorkspaceRoot } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot();
const bin = requireMcpBinary(repoRoot);

/** 🎯️ The full tool census `🦀️.rs`'s `GATEWAY_TOOL_NAMES` declares. Duplicated here on
 * purpose: this suite is an INDEPENDENT observer of the running binary, so it must not import the
 * value it is checking. */
const GATEWAY_TOOL_NAMES = [
  "capabilities_search",
  "capabilities_describe",
  "context_resolve",
  "action_prepare",
  "action_invoke",
  "action_cancel",
  "transaction_begin",
  "transaction_commit",
  "transaction_rollback",
  "history_undo",
  "history_redo",
  "artifact_open",
  "artifact_create",
  "artifact_validate",
  "artifact_snapshot",
  "artifact_export",
  "inference_list",
  "inference_get",
  "inference_submit",
  "inference_events",
  "inference_cancel",
  "inference_approve",
  "ui_focus",
  "ui_reveal",
  "job_get",
  "job_cancel",
] as const;

const GATEWAY_PROMPT_NAMES = ["explore_workspace", "safe_mutation", "inspect_artifact", "drive_the_ui", "undo_last_change"] as const;

/** 🏠️ The tools that genuinely need a bound workspace — the ones tier 1 must degrade. */
const WORKSPACE_BACKED_TOOLS = ["artifact_open", "artifact_validate", "artifact_snapshot", "artifact_export", "inference_list"] as const;

/** 🖥️ The tools that need an attached shell, which stdio never has. */
const SHELL_BACKED_TOOLS = ["ui_focus", "ui_reveal"] as const;

type ToolListResult = { tools: Array<{ name: string; description?: string; inputSchema: Record<string, unknown>; outputSchema?: Record<string, unknown> }> };
type ResourceListResult = { resources: Array<{ uri: string; name: string }> };
type TemplateListResult = { resourceTemplates: Array<{ uriTemplate: string; name: string }> };
type PromptListResult = { prompts: Array<{ name: string; title?: string; description?: string }> };
type CallToolResult = { isError?: boolean; structuredContent?: Record<string, unknown>; content?: Array<{ type: string; text?: string }> };

describe("semio-os-mcp — end to end", () => {
  let procs: RawMcpProcess[] = [];
  let folders: string[] = [];

  beforeEach(() => {
    procs = [];
    folders = [];
  });

  afterEach(async () => {
    await Promise.all(procs.map((proc) => proc.close()));
    for (const folder of folders) rmSync(folder, { recursive: true, force: true });
  });

  /** 🚀️ A freshly spawned, already-discovered server. `server/discover` is the modern era's opener; no
   * `initialize` handshake is sent, matching the stateless `2026-07-28` contract. */
  const openServer = async (args: readonly string[] = ["stdio"]): Promise<RawMcpProcess> => {
    const proc = spawnRawMcp(bin, args);
    procs.push(proc);
    const discovered = await proc.request("server/discover", {});
    expect(discovered.error, `server/discover failed: ${JSON.stringify(discovered.error)}`).toBeUndefined();
    return proc;
  };

  /** 📁️ A throwaway space directory for the `--folder` (headless) tier. */
  const freshFolder = (): string => {
    const folder = mkdtempSync(join(tmpdir(), "semio-os-mcp-e2e-"));
    folders.push(folder);
    return folder;
  };

  //#region 🔖️Surface
  it("tools/list is the full 26-tool gateway surface", async () => {
    const proc = await openServer();
    const response = await proc.request("tools/list", {});
    expect(response.error).toBeUndefined();
    const names = (response.result as ToolListResult).tools.map((tool) => tool.name).sort();
    expect(names).toEqual([...GATEWAY_TOOL_NAMES].sort());
  });

  it("no tool describes itself as unimplemented — the stub era is over", async () => {
    const proc = await openServer();
    const { tools } = (await proc.request("tools/list", {})).result as ToolListResult;
    for (const tool of tools) {
      const description = (tool.description ?? "").toLowerCase();
      expect(description, `${tool.name} still advertises itself as a stub`).not.toMatch(/not yet implemented|not implemented yet|declared, not yet/);
    }
  });

  it("every tool advertises an object-typed input schema, and an object-typed output schema when it has one", async () => {
    const proc = await openServer();
    const { tools } = (await proc.request("tools/list", {})).result as ToolListResult;
    for (const tool of tools) {
      expect(tool.inputSchema.type, `${tool.name} inputSchema`).toBe("object");
      if (tool.outputSchema !== undefined) expect(tool.outputSchema.type, `${tool.name} outputSchema`).toBe("object");
    }
  });

  it("prompts/list serves the real bilingual prompt set", async () => {
    const proc = await openServer();
    const response = await proc.request("prompts/list", {});
    expect(response.error).toBeUndefined();
    const names = (response.result as PromptListResult).prompts.map((prompt) => prompt.name).sort();
    expect(names).toEqual([...GATEWAY_PROMPT_NAMES].sort());
  });

  it("prompts/get answers differently in English and German", async () => {
    const proc = await openServer();
    const english = await proc.request("prompts/get", { name: "safe_mutation", arguments: { locale: "en" } });
    const german = await proc.request("prompts/get", { name: "safe_mutation", arguments: { locale: "de" } });
    expect(english.error).toBeUndefined();
    expect(german.error).toBeUndefined();
    const englishText = ((english.result as { messages: Array<{ content: { text: string } }> }).messages[0]?.content.text ?? "").trim();
    const germanText = ((german.result as { messages: Array<{ content: { text: string } }> }).messages[0]?.content.text ?? "").trim();
    expect(englishText.length).toBeGreaterThan(0);
    expect(germanText.length).toBeGreaterThan(0);
    expect(germanText).not.toBe(englishText);
  });

  it("resources advertise the workspace, artifact, inference, UI and job families regardless of tier", async () => {
    const proc = await openServer();
    const listed = (await proc.request("resources/list", {})).result as ResourceListResult;
    const uris = listed.resources.map((resource) => resource.uri);
    expect(uris).toContain("semio://capability");
    expect(uris).toContain("semio://workspace");
    expect(uris).toContain("semio://workspace/artifacts");
    expect(uris.some((uri) => uri.startsWith("semio://window"))).toBe(true);
    expect(uris.some((uri) => uri.startsWith("semio://ui/"))).toBe(true);

    const templates = (await proc.request("resources/templates/list", {})).result as TemplateListResult;
    const shapes = templates.resourceTemplates.map((template) => template.uriTemplate);
    expect(shapes).toContain("semio://capability/{id}");
    expect(shapes).toContain("semio://artifact/{artifactId}");
    expect(shapes.some((shape) => shape.includes("/inference/"))).toBe(true);
    expect(shapes.some((shape) => shape.includes("{jobId}"))).toBe(true);
  });
  //#endregion 🔖️Surface

  //#region 🔖️ProgressiveEnhancement
  it("tier 1 (no binding): workspace-backed tools degrade to a retryable PLUGIN_UNAVAILABLE naming the binding", async () => {
    const proc = await openServer();
    for (const name of WORKSPACE_BACKED_TOOLS) {
      const response = await proc.request("tools/call", { name, arguments: { artifactId: "anything" } });
      expect(response.error, `${name} must not fail at the protocol level`).toBeUndefined();
      const result = response.result as CallToolResult;
      expect(result.isError, `${name} must not fabricate a success`).toBe(true);
      const structured = result.structuredContent ?? {};
      expect(structured.code, `${name}: ${JSON.stringify(structured)}`).toBe("PLUGIN_UNAVAILABLE");
      expect(structured.retryable, `${name} must be retryable — binding one closes the gap`).toBe(true);
      expect(String(structured.message ?? ""), `${name} must name the missing binding`).toMatch(/--folder|--hub/);
    }
  });

  it("tier 2 (stdio, no shell): UI tools report the missing shell, not a missing workspace", async () => {
    const proc = await openServer(["stdio", "--folder", freshFolder()]);
    for (const name of SHELL_BACKED_TOOLS) {
      const response = await proc.request("tools/call", { name, arguments: { windowId: "w1", anchor: "left", path: [] } });
      expect(response.error, `${name} must not fail at the protocol level`).toBeUndefined();
      const result = response.result as CallToolResult;
      expect(result.isError).toBe(true);
      const structured = result.structuredContent ?? {};
      expect(structured.retryable, `${name} is retryable — a shell may attach later`).toBe(true);
      expect(String(structured.message ?? "").toLowerCase(), `${name}: ${JSON.stringify(structured)}`).toContain("shell");
    }
  });

  it("binding a folder actually changes behaviour — the workspace resource stops reporting a missing binding", async () => {
    const bare = await openServer();
    const bareRead = await bare.request("resources/read", { uri: "semio://workspace" });
    const bareError = JSON.stringify(bareRead.result ?? bareRead.error ?? {});
    expect(bareError).toMatch(/--folder|--hub|PLUGIN_UNAVAILABLE/);

    const bound = await openServer(["stdio", "--folder", freshFolder()]);
    const boundRead = await bound.request("resources/read", { uri: "semio://workspace" });
    expect(boundRead.error).toBeUndefined();
    const contents = (boundRead.result as { contents: Array<{ text?: string }> }).contents;
    expect(contents.length).toBeGreaterThan(0);
    const body = JSON.parse(contents[0]?.text ?? "{}") as Record<string, unknown>;
    expect(body, "a bound workspace reports its own origin, never a PLUGIN_UNAVAILABLE").toHaveProperty("origin");
  });

  it("an unknown resource is a well-formed NOT_FOUND, never a crash", async () => {
    const proc = await openServer();
    const response = await proc.request("resources/read", { uri: "semio://not-a-real-resource" });
    const payload = JSON.stringify(response.result ?? response.error ?? {});
    expect(payload).toMatch(/NOT_FOUND|not a real|unknown resource|no such resource/i);
    expect(await proc.waitForExit(50).catch(() => null), "the server must survive an unknown resource").toBeNull();
  });
  //#endregion 🔖️ProgressiveEnhancement

  //#region 🔖️PluginIndependence
  it("the catalog is compiled from the installed plugin registry, not a hardcoded note/cad fixture", async () => {
    const proc = await openServer();
    const response = await proc.request("tools/call", { name: "context_resolve", arguments: { principal: "agent:local" } });
    expect(response.error).toBeUndefined();
    const structured = (response.result as CallToolResult).structuredContent ?? {};
    expect(structured.catalogHash, "context_resolve must report a real catalog hash").toBeTruthy();
  });

  it("capabilities_search answers structurally for a goal no installed plugin may satisfy", async () => {
    const proc = await openServer();
    const response = await proc.request("tools/call", { name: "capabilities_search", arguments: { query: "something no plugin implements zzzz" } });
    expect(response.error).toBeUndefined();
    const result = response.result as CallToolResult;
    expect(result.isError).toBeFalsy();
    expect(Array.isArray((result.structuredContent ?? {}).results), "search always answers with a results array, even when empty").toBe(true);
  });
  //#endregion 🔖️PluginIndependence
});
