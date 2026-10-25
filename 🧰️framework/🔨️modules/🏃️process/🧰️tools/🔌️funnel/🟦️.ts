import { runtimeCommandNameV1 } from "../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import { toolForCommandV1, TOOL_REGISTRY_V1, type ToolRegistryV1 } from "../📖️registry/🟦️.ts";
import { resolveToolV1, systemToolHostV1, toolRootV1 } from "../🔎️resolve/🟦️.ts";
import { toolEnvironmentV1 } from "../🌿️environment/🟦️.ts";

/** 🔌️ A command ready to spawn: the program, the arguments and the environment, rewritten when the command is a registered tool. */
export type RoutedCommandV1 = Readonly<{ command: string; arguments: readonly string[]; environment: Readonly<Record<string, string | undefined>>; tool: string | null }>;

const resolved = new Map<string, ReturnType<typeof resolveToolV1>>();

/** 🧭️ Routes a command through the tool registry when its name belongs to a registered tool (`bun`, `cargo`, `git`, or the running Bun's own path): the executable is the registry's resolution, the environment is the registry's sanitized one. Commands the registry does not know, explicit paths, tools the host lacks and `SEMIO_TOOL_GUARD=off` pass unchanged. The process funnels `runOwnedCommand`, `captureOwnedCommandV1` and `runCmd` call this, so every program they start follows the registry without each call site changing. */
export function routeRegisteredCommandV1(command: string, args: readonly string[], cwd: string, environment: Readonly<Record<string, string | undefined>> = process.env, registry: ToolRegistryV1 = TOOL_REGISTRY_V1): RoutedCommandV1 {
  const unchanged: RoutedCommandV1 = { command, arguments: args, environment, tool: null };
  if (environment.SEMIO_TOOL_GUARD === "off") return unchanged;
  const self = command === process.execPath, entry = self || !/[\\/]/u.test(command) ? toolForCommandV1(self ? runtimeCommandNameV1() : command, registry) : undefined;
  if (entry === undefined || entry.id === "repo") return unchanged;
  const name = self ? entry.commands[0]! : command, root = toolRootV1(cwd, environment), key = `${root}\0${entry.id}\0${name}`;
  let resolution = resolved.get(key);
  if (resolution === undefined) { resolution = resolveToolV1(entry, systemToolHostV1(root, environment), registry, 0, name); if (resolution.command !== null) resolved.set(key, resolution); }
  if (resolution.command === null) return unchanged;
  return { command: resolution.command, arguments: [...resolution.prefix, ...args], environment: toolEnvironmentV1(entry, environment, {}, { root }, registry), tool: entry.id };
}
