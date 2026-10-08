import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

/** 🤖️ The agent and editor clients whose MCP configuration file is derived from the root dashboard declaration.
 * @see ../../🎛️dashboard/🧬️schema/🎮️registry/🔣️.json `#/$defs/AgentClientId` */
export const AGENT_CLIENTS = ["claude-code", "vscode", "cursor", "codex"] as const;
export type AgentClient = (typeof AGENT_CLIENTS)[number];

/** 📋️ The workspace-relative manifest whose `metadata.semio.dashboard.tools[].mcp` is the only declaration of every client file. */
export const AGENT_CLIENT_DECLARATION_MANIFEST = "📋️project.json";

/** 🔌️ One MCP stdio server as a client starts it. */
export type McpServer = Readonly<{ name: string; command: string; args: readonly string[] }>;

type ToolParameter = Readonly<{ id: string; kind: string; default?: unknown; required?: boolean; valuePositional?: boolean; valueFlag?: string }>;
type ToolMcp = Readonly<{ server: string; clients: Readonly<Record<string, Readonly<{ parameters?: Readonly<Record<string, string>> }>>> }>;

/** 🧰️ The members of a dashboard tool the client files are derived from. */
export type DeclaredTool = Readonly<{ id: string; command: readonly string[]; parameters?: readonly ToolParameter[]; mcp?: ToolMcp }>;

/** 📄️ One derived client file. */
export type AgentClientFile = Readonly<{ client: AgentClient; path: string; text: string }>;

/** 🚨️ A client file that differs from what the declaration derives. */
export type AgentClientDrift = Readonly<{ client: AgentClient; path: string; reason: "missing" | "drift" }>;

const jsonText = (value: unknown): string => `${JSON.stringify(value, null, 2)}\n`;
const tomlText = (value: string): string => JSON.stringify(value);
const tomlKey = (name: string): string => (/^[A-Za-z0-9_-]+$/.test(name) ? name : tomlText(name));

const stdioEntries = (servers: readonly McpServer[]) => Object.fromEntries(servers.map((server) => [server.name, { type: "stdio", command: server.command, args: [...server.args] }]));

const CLIENT_FILES: Readonly<Record<AgentClient, Readonly<{ path: string; render: (servers: readonly McpServer[]) => string }>>> = {
  "claude-code": { path: ".mcp.json", render: (servers) => jsonText({ mcpServers: stdioEntries(servers) }) },
  vscode: { path: ".vscode/mcp.json", render: (servers) => jsonText({ inputs: [], servers: stdioEntries(servers) }) },
  cursor: { path: ".cursor/mcp.json", render: (servers) => jsonText({ mcpServers: stdioEntries(servers) }) },
  codex: {
    path: ".codex/config.toml",
    render: (servers) =>
      `${servers.map((server) => `[mcp_servers.${tomlKey(server.name)}]\ncommand = ${tomlText(server.command)}\nargs = [${server.args.map(tomlText).join(", ")}]\nenabled = true\ncwd = "."\n`).join("\n")}`,
  },
};

/** 🧮️ The process words of `tool` for one client: the declared command followed by the value of every positional or flag parameter. */
function toolWords(tool: DeclaredTool, values: Readonly<Record<string, string>>): string[] {
  const unknown = Object.keys(values).filter((id) => !(tool.parameters ?? []).some((parameter) => parameter.id === id));
  if (unknown.length > 0) throw new Error(`[agents] tool ${tool.id}: no parameter ${unknown.join(", ")}`);
  const words = [...tool.command];
  for (const parameter of tool.parameters ?? []) {
    if (parameter.kind !== "text" || !(parameter.valuePositional || parameter.valueFlag)) throw new Error(`[agents] tool ${tool.id}: parameter ${parameter.id} must be a text parameter that is positional or a flag`);
    const value = values[parameter.id] ?? parameter.default;
    if (value === undefined) {
      if (parameter.required) throw new Error(`[agents] tool ${tool.id}: parameter ${parameter.id} needs a value`);
      continue;
    }
    if (typeof value !== "string") throw new Error(`[agents] tool ${tool.id}: parameter ${parameter.id} default must be text`);
    if (parameter.valuePositional) words.push(value);
    else if (parameter.valueFlag!.endsWith("=")) words.push(`${parameter.valueFlag}${value}`);
    else words.push(parameter.valueFlag!, value);
  }
  return words;
}

/** 🔌️ The MCP servers every client starts, in declaration order. */
export function mcpServersOf(tools: readonly DeclaredTool[]): Readonly<Record<AgentClient, readonly McpServer[]>> {
  const servers: Record<AgentClient, McpServer[]> = { "claude-code": [], vscode: [], cursor: [], codex: [] };
  const names = new Set<string>();
  for (const tool of tools) {
    if (!tool.mcp) continue;
    if (names.has(tool.mcp.server)) throw new Error(`[agents] MCP server ${tool.mcp.server} is declared twice`);
    names.add(tool.mcp.server);
    for (const [client, options] of Object.entries(tool.mcp.clients)) {
      if (!(AGENT_CLIENTS as readonly string[]).includes(client)) throw new Error(`[agents] tool ${tool.id}: unknown client ${client}`);
      const [command, ...args] = toolWords(tool, options.parameters ?? {});
      if (!command) throw new Error(`[agents] tool ${tool.id}: empty command`);
      servers[client as AgentClient].push({ name: tool.mcp.server, command, args });
    }
  }
  return servers;
}

/** 📄️ The client files derived from `tools`, one per client in {@link AGENT_CLIENTS} order. */
export function deriveAgentClientFiles(tools: readonly DeclaredTool[]): readonly AgentClientFile[] {
  const servers = mcpServersOf(tools);
  return AGENT_CLIENTS.map((client) => ({ client, path: CLIENT_FILES[client].path, text: CLIENT_FILES[client].render(servers[client]) }));
}

/** 📖️ The dashboard tools declared in the root manifest of `root`. */
export function readDeclaredTools(root: string): readonly DeclaredTool[] {
  const manifest = JSON.parse(readFileSync(join(root, AGENT_CLIENT_DECLARATION_MANIFEST), "utf8")) as { metadata?: { semio?: { dashboard?: { tools?: readonly DeclaredTool[] } } } };
  return manifest.metadata?.semio?.dashboard?.tools ?? [];
}

const lineFeeds = (text: string): string => text.replace(/\r\n/g, "\n");

/** 🔎️ The client files of `root` that differ from the root declaration; line-ending style is not drift. */
export function checkAgentClients(root: string): readonly AgentClientDrift[] {
  const drift: AgentClientDrift[] = [];
  for (const file of deriveAgentClientFiles(readDeclaredTools(root))) {
    const path = join(root, file.path);
    if (!existsSync(path)) drift.push({ client: file.client, path: file.path, reason: "missing" });
    else if (lineFeeds(readFileSync(path, "utf8")) !== file.text) drift.push({ client: file.client, path: file.path, reason: "drift" });
  }
  return drift;
}

/** ✍️ Writes every client file that differs from the root declaration and returns the paths it changed. */
export function writeAgentClients(root: string): readonly string[] {
  const changed: string[] = [];
  for (const file of deriveAgentClientFiles(readDeclaredTools(root))) {
    const path = join(root, file.path);
    if (existsSync(path) && lineFeeds(readFileSync(path, "utf8")) === file.text) continue;
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, file.text);
    changed.push(file.path);
  }
  return changed;
}

/** 🚀️ The launcher of one MCP server declared by `tools` for a credential flow that completes the command line itself (`--hub`, `--space`, `--credential-file`): the declared words up to the first flag, with a relative entry script resolved against `root` and the first word replaced by `runtime`. */
export function agentClientLauncher(tools: readonly DeclaredTool[], root: string, server: string, runtime: string): Readonly<{ command: string; args: readonly string[] }> {
  const declared = Object.values(mcpServersOf(tools)).flat().find((candidate) => candidate.name === server);
  if (!declared) throw new Error(`[agents] no MCP server ${server} is declared`);
  const flag = declared.args.findIndex((word) => word.startsWith("--"));
  const words = flag < 0 ? declared.args : declared.args.slice(0, flag);
  return { command: runtime, args: words.map((word, index) => (index === 0 && word.startsWith("./") ? join(root, word) : word)) };
}
