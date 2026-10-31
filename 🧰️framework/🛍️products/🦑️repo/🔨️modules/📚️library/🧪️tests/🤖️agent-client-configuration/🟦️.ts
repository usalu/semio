/** 🤖️ Agent client configuration canon: every MCP client file is derived from the one root dashboard declaration. The rendered
 * text is compared with a language-agnostic fixture; its meaning is read back independently by jsonc-parser, smol-toml
 * and Bun TOML and the declaration is validated by Ajv against the registry schema. @see ../🧫️fixtures/🤖️agent-client-configuration/🔣️.json */
import { describe, expect, test } from "bun:test";
import { cpSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";

import { dirname, join, resolve } from "node:path";
import Ajv2020 from "ajv";
import { parse as parseJsonc } from "jsonc-parser";
import * as SmolToml from "smol-toml";
import { AGENT_CLIENTS, AGENT_CLIENT_DECLARATION_MANIFEST, agentClientLauncher, checkAgentClients, deriveAgentClientFiles, mcpServersOf, readDeclaredTools, writeAgentClients, type DeclaredTool, type McpServer } from "../../🤖️agent-clients/🟦️.ts";

type Fixture = Readonly<{
  declaration: Readonly<{ tools: readonly DeclaredTool[] }>;
  servers: Readonly<Record<string, readonly McpServer[]>>;
  files: Readonly<Record<string, string>>;
  launchers: readonly Readonly<{ server: string; expected: { command: string; args: string[] } }>[];
  refusals: readonly Readonly<{ id: string; tool: DeclaredTool; message: string }>[];
}>;

const libraryRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(libraryRoot, "../../../../..");
const fixture = JSON.parse(readFileSync(join(libraryRoot, "🧪️tests/🧫️fixtures/🤖️agent-client-configuration/🔣️.json"), "utf8")) as Fixture;
const registrySchema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json"), "utf8")) as { $id: string; $defs: { AgentClientId: { enum: string[] } } };

type ServerTable = Record<string, { type?: string; command: string; args: string[]; env?: Record<string, string>; enabled?: boolean; cwd?: string }>;

function parsedServers(path: string, text: string): McpServer[] {
  const table: ServerTable = path.endsWith(".toml")
    ? ((SmolToml.parse(text) as { mcp_servers: ServerTable }).mcp_servers)
    : (() => {
        const document = parseJsonc(text) as { mcpServers?: ServerTable; servers?: ServerTable };
        return document.mcpServers ?? document.servers ?? {};
      })();
  return Object.entries(table).map(([name, entry]) => ({ name, command: entry.command, args: entry.args, ...(entry.env ? { env: entry.env } : {}) }));
}

describe("derivation from a fixture declaration", () => {
  const files = deriveAgentClientFiles(fixture.declaration.tools);

  test("renders exactly the fixture text for every client", () => {
    expect(files.map((file) => file.path)).toEqual(Object.keys(fixture.files));
    for (const file of files) expect(file.text).toBe(fixture.files[file.path]!);
  });

  test("the declared servers are what each client file parses to (jsonc-parser, smol-toml)", () => {
    for (const file of files) expect(parsedServers(file.path, file.text)).toEqual([...fixture.servers[file.client]!]);
  });

  test("a second TOML parser reads the Codex file the same way", () => {
    const codex = files.find((file) => file.client === "codex")!;
    expect(Bun.TOML.parse(codex.text)).toEqual(SmolToml.parse(codex.text) as object);
  });

  test("server resolution is the fixture's independent statement", () => {
    expect(mcpServersOf(fixture.declaration.tools)).toEqual(fixture.servers as ReturnType<typeof mcpServersOf>);
  });

  for (const refusal of fixture.refusals) {
    test(`refuses ${refusal.id}`, () => {
      expect(() => mcpServersOf([refusal.tool])).toThrow(refusal.message);
    });
  }

  test("refuses one server name declared by two tools", () => {
    const tool = fixture.declaration.tools[0]!;
    expect(() => mcpServersOf([tool, { ...tool, id: "again" }])).toThrow("declared twice");
  });
});

describe("the launcher a credential flow completes", () => {
  for (const launcher of fixture.launchers) {
    test(`is the declared words of ${launcher.server} up to the first flag`, () => {
      const answer = agentClientLauncher(fixture.declaration.tools, launcher.server);
      expect({ command: answer.command, args: answer.args }).toEqual(launcher.expected);
    });
  }

  test("the real semio server starts through the repo command without the catalog, folder and scopes of its client files", () => {
    expect(agentClientLauncher(readDeclaredTools(repoRoot), "semio")).toEqual({ command: "repo", args: ["dev", "mcp", "stdio", "os"] });
  });

  test("refuses a server nobody declares", () => {
    expect(() => agentClientLauncher(fixture.declaration.tools, "missing")).toThrow("no MCP server missing");
  });
});

describe("the registry schema", () => {
  const ajv = new Ajv2020({ strict: false, allErrors: true });
  ajv.addSchema(registrySchema);
  const validateDeclaration = ajv.getSchema(`${registrySchema.$id}#/$defs/ProjectDeclaration`)!;

  test("names exactly the clients the writer knows", () => {
    expect([...AGENT_CLIENTS] as string[]).toEqual(registrySchema.$defs.AgentClientId.enum);
  });

  test("accepts the fixture declaration and the root declaration", () => {
    expect(validateDeclaration({ tools: fixture.declaration.tools })).toBe(true);
    expect(validateDeclaration({ tools: readDeclaredTools(repoRoot) })).toBe(true);
  });

  test("rejects an unknown client and an empty client table", () => {
    const tool = fixture.declaration.tools[0]!;
    expect(validateDeclaration({ tools: [{ ...tool, mcp: { server: "alpha", clients: { windsurf: {} } } }] })).toBe(false);
    expect(validateDeclaration({ tools: [{ ...tool, mcp: { server: "alpha", clients: {} } }] })).toBe(false);
  });
});

describe("the checked-in client files", () => {
  test("match the root declaration", () => {
    expect(checkAgentClients(repoRoot)).toEqual([]);
  });

  test("parse to the servers the root declaration states (jsonc-parser, smol-toml)", () => {
    const declared = mcpServersOf(readDeclaredTools(repoRoot));
    expect(declared["claude-code"].map((server) => server.name)).toEqual(["repo", "semio"]);
    for (const file of deriveAgentClientFiles(readDeclaredTools(repoRoot))) {
      expect(parsedServers(file.path, readFileSync(join(repoRoot, file.path), "utf8"))).toEqual([...declared[file.client]]);
    }
  });

  test("every server starts through the repo command and binds a folder that is not the repository root", () => {
    for (const server of Object.values(mcpServersOf(readDeclaredTools(repoRoot))).flat()) {
      expect(server.command).toBe("repo");
      expect(server.args).not.toContain("./📜️script.ts");
      if (server.args[3] === "os") expect(server.args[server.args.indexOf("--folder") + 1]).not.toBe(".");
    }
  });
});

describe("check and write on a scratch workspace", () => {
  function scratch(): string {
    const temporary = process.env.SEMIO_TEST_OUTPUT_DIR ?? join(repoRoot, ".🧬semio/🦑️repo/⚡️cache/tests/agent-clients");
    mkdirSync(temporary, { recursive: true });
    const root = mkdtempSync(join(temporary, "agent-clients-"));
    cpSync(join(repoRoot, AGENT_CLIENT_DECLARATION_MANIFEST), join(root, AGENT_CLIENT_DECLARATION_MANIFEST));
    for (const file of deriveAgentClientFiles(readDeclaredTools(repoRoot))) {
      mkdirSync(dirname(join(root, file.path)), { recursive: true });
      cpSync(join(repoRoot, file.path), join(root, file.path));
    }
    return root;
  }

  test("a copy is current, a mutated copy drifts, write repairs it", () => {
    const root = scratch();
    try {
      expect(checkAgentClients(root)).toEqual([]);
      writeFileSync(join(root, ".mcp.json"), readFileSync(join(root, ".mcp.json"), "utf8").replace("stdio", "http"));
      writeFileSync(join(root, ".codex/config.toml"), readFileSync(join(root, ".codex/config.toml"), "utf8").replace('"."', '"os"'));
      rmSync(join(root, ".codex/config.toml"));
      expect(checkAgentClients(root).map((drift) => [drift.path, drift.reason])).toEqual([
        [".mcp.json", "drift"],
        [".codex/config.toml", "missing"],
      ]);
      expect([...writeAgentClients(root)].sort()).toEqual([".codex/config.toml", ".mcp.json"]);
      expect(checkAgentClients(root)).toEqual([]);
      expect(writeAgentClients(root)).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("a changed declaration drifts every client that exposes the tool", () => {
    const root = scratch();
    try {
      const manifestPath = join(root, AGENT_CLIENT_DECLARATION_MANIFEST);
      writeFileSync(manifestPath, readFileSync(manifestPath, "utf8").replace("conversation.write", "conversation.read"));
      expect(checkAgentClients(root).map((drift) => drift.path)).toEqual([".mcp.json", ".codex/config.toml"]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("line-ending style is not drift", () => {
    const root = scratch();
    try {
      writeFileSync(join(root, ".mcp.json"), readFileSync(join(root, ".mcp.json"), "utf8").replace(/\n/g, "\r\n"));
      expect(checkAgentClients(root)).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
