import { constants } from "node:os";
import { createToolAdmissionV1, findToolV1, resolveRegisteredToolV1, runToolV1, setDefaultToolAdmissionV1, toolLanguageV1, toolForCommandV1, toolMessageV1, toolRootV1, ToolErrorV1, TOOL_REGISTRY_V1, type ToolEntryV1, type ToolResolutionV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { BUILD_HOST_CAPACITY_POLICY_V1 } from "../../🌡️host-capacity/🟦️.ts";

/** 🔌️ What the tools command touches: output, environment, root and the cancellation signal; tests replace them. */
export type ToolsCommandContextV1 = Readonly<{ environment: NodeJS.ProcessEnv; write: (line: string) => void; root?: string; signal?: AbortSignal }>;

const flag = (head: readonly string[], name: string): string | undefined => {
  for (let index = 0; index < head.length; index++) { const argument = head[index]!; if (argument === `--${name}`) return head[index + 1]; if (argument.startsWith(`--${name}=`)) return argument.slice(name.length + 3); }
  return undefined;
};

const describe = (entry: ToolEntryV1, resolution: ToolResolutionV1): string => `${entry.id.padEnd(11)} ${(resolution.pin.version ?? resolution.pin.minimum ?? entry.pin.state).padEnd(26)} ${resolution.source.padEnd(15)} ${resolution.command === null ? "-" : [resolution.command, ...resolution.prefix].join(" ")}`;

/** 🧯️ Preserves a tool status or translates its terminating host signal into a shell exit status. */
export function toolCommandExitStatusV1(status: number | null, signal: string | null, signals: Readonly<Record<string, number>>): number {
  if (status !== null) return status;
  const signalNumber = signal === null ? undefined : Object.entries(signals).find(([name]) => name === signal)?.[1];
  return signalNumber === undefined ? 1 : 128 + signalNumber;
}

/** 📋️ Prints the registered tools with their pin, where each was found on this host and the command that runs it. */
function list(head: readonly string[], context: ToolsCommandContextV1, root: string): number {
  const language = toolLanguageV1(context.environment), rows = TOOL_REGISTRY_V1.tools.map(entry => ({ entry, resolution: resolveRegisteredToolV1(entry, root, { env: context.environment }) }));
  if (head.includes("--json")) { context.write(JSON.stringify(rows.map(({ entry, resolution }) => ({ id: entry.id, kind: entry.kind, summary: entry.summary, capabilities: entry.capabilities, adapter: entry.adapter, allowedCallers: entry.allowedCallers, pin: resolution.pin, source: resolution.source, command: resolution.command, prefix: resolution.prefix, suffix: resolution.suffix ?? [], tried: resolution.tried })), null, 1)); return 0; }
  context.write(toolMessageV1("listHeader", {}, language));
  for (const { entry, resolution } of rows) context.write(describe(entry, resolution));
  return 0;
}

/** ▶️ Runs a registered tool under machine capacity with the sanitized environment of the registry: `repo tools exec [--cwd=<path>] [--light] <tool|command> -- <arguments>` where the command is an executable a registered tool owns. */
async function exec(head: readonly string[], tail: readonly string[], context: ToolsCommandContextV1, root: string): Promise<number> {
  const language = toolLanguageV1(context.environment), id = head.find((word, index) => !word.startsWith("--") && head[index - 1] !== "--cwd"), entry = id === undefined ? undefined : findToolV1(id) ?? toolForCommandV1(id);
  if (id === undefined) throw new Error(toolMessageV1("execUsage", {}, language));
  if (entry === undefined) throw new ToolErrorV1("unknown-tool", id, toolMessageV1("unknownTool", { tool: id, known: TOOL_REGISTRY_V1.tools.map(tool => tool.id).join(", ") }, language));
  const controller = new AbortController(), stop = (): void => controller.abort(), cwd = flag(head, "cwd") ?? process.cwd();
  process.once("SIGINT", stop);
  process.once("SIGTERM", stop);
  context.signal?.addEventListener("abort", stop, { once: true });
  setDefaultToolAdmissionV1(createToolAdmissionV1(BUILD_HOST_CAPACITY_POLICY_V1, { environment: context.environment, write: line => process.stderr.write(line + "\n") }));
  try {
    const result = await runToolV1(entry.id, tail, { root, cwd, command: entry.id === id ? undefined : id, env: context.environment, signal: controller.signal, check: false, heavy: !head.includes("--light"), stdout: "inherit", trusted: true, onProgress: event => { if (event.phase === "admitting") process.stderr.write(toolMessageV1("admitting", { tool: event.tool }, language) + "\n"); } });
    return toolCommandExitStatusV1(result.status, result.signal, constants.signals);
  } finally {
    process.removeListener("SIGINT", stop);
    process.removeListener("SIGTERM", stop);
  }
}

/** 🚪️ Dispatches `repo tools list [--json]` and `repo tools exec [--cwd=<path>] <tool> -- <arguments>`; the exit code of `exec` is the tool's. */
export async function runToolsCommandV1(args: readonly string[], context: ToolsCommandContextV1): Promise<number> {
  const boundary = args.indexOf("--"), head = args.slice(1, boundary < 0 ? undefined : boundary), tail = boundary < 0 ? [] : args.slice(boundary + 1), root = context.root ?? toolRootV1(process.cwd(), context.environment);
  if (args[0] === "list") return list(head, context, root);
  if (args[0] === "exec") return exec(head, tail, context, root);
  throw new Error(toolMessageV1("execUsage", {}, toolLanguageV1(context.environment)));
}
