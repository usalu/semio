import { statSync } from "node:fs";
import { join, resolve } from "node:path";
import { advanceScriptInvocation, SCRIPT_PROCESS_INVOCATION_ENV, type ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { isRepoErrorV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { pythonEnvironmentMissingV1 } from "../../../../../../../🔨️modules/🏃️process/🏗️build/🧰️toolchains/🟦️.ts";
import { resolveBuildHelpLanguageV1 } from "../../../🏗️build/⚙️operations/📖️help/🟦️.ts";
import { rootCommandRequirementV1, type RootCommandGateV1, type RootCommandRegistryV1, type RootCommandV1 } from "../🟦️.ts";

/** 🔌️ The effects of provisioning, replaceable by a law that must not install anything. */
export type DependencyProvisionPortsV1 = Readonly<{
  available(root: string): boolean;
  run(command: string, args: readonly string[], cwd: string, label: string, environment: Readonly<Record<string, string | undefined>>, signal: AbortSignal): Promise<void>;
  write(line: string): void;
  now(): number;
}>;

/** 🏃️ Runs the owner of provisioning as an owned child: its output is prefixed and goes to stderr like every line of the gate, because stdout belongs to the command that follows (an MCP server speaks JSON-RPC on it); its progress repeats every 10 s, and cancellation ends its process tree. */
export const dependencyProvisionPortsV1: DependencyProvisionPortsV1 = {
  available: (root) => (statSync(join(root, "node_modules"), { throwIfNoEntry: false })?.isDirectory() ?? false) && !pythonEnvironmentMissingV1(root),
  run: (command, args, cwd, label, environment, signal) => runOwnedCommand(command, [...args], cwd, label, 0, { env: environment, signal, outputStyle: "stream", stdout: "stderr" }),
  write: (line) => console.error(line),
  now: () => Date.now(),
};

const ENVELOPE_VARIABLES: readonly string[] = [SCRIPT_PROCESS_INVOCATION_ENV, "SEMIO_SCRIPT_POLICY", "SEMIO_SCRIPT_CAPABILITIES"];

/** 🪪️ Whether the build CLI issued this invocation for the selected workspace. */
export function buildOwnedInvocationV1(root: string, invocation: ScriptInvocation): boolean {
  const owned = invocation.capabilities as Readonly<{ workspaceRoot?: unknown; repositoryRoot?: unknown }>;
  return [owned.workspaceRoot, owned.repositoryRoot].some((value) => typeof value === "string" && resolve(value) === resolve(root));
}

/** 🧹️ The environment of the provisioning child: a fresh build CLI birth, never the envelope of this process. */
export function provisionEnvironmentV1(environment: Readonly<Record<string, string | undefined>>): Record<string, string | undefined> {
  return Object.fromEntries(Object.entries(environment).filter(([key]) => !ENVELOPE_VARIABLES.includes(key)));
}

const ensured = new WeakMap<DependencyProvisionPortsV1, Map<string, Promise<void>>>();

/** 🧰️ Ensures the dependencies of `root` once per process and provisioning ports: the gate and a staged route share one `repo dependencies ensure`, and a failed or cancelled run is retried by the next caller. */
export function ensureRootDependenciesV1(root: string, registry: RootCommandRegistryV1, commandId: string, signal: AbortSignal, environment: Readonly<Record<string, string | undefined>> = process.env, ports: DependencyProvisionPortsV1 = dependencyProvisionPortsV1, osLanguages?: () => readonly string[]): Promise<void> {
  const runs = ensured.get(ports) ?? new Map<string, Promise<void>>(), key = resolve(root), running = runs.get(key);
  ensured.set(ports, runs);
  if (running !== undefined) return running;
  const run = provisionRootDependenciesV1(root, registry, commandId, signal, environment, ports, osLanguages).catch((error: unknown) => { runs.delete(key); throw error; });
  runs.set(key, run);
  return run;
}

/** 🏃️ Runs `repo dependencies ensure` for one command with progress in the help language and never swallows a failure; a cancellation rethrows the cause. */
async function provisionRootDependenciesV1(root: string, registry: RootCommandRegistryV1, commandId: string, signal: AbortSignal, environment: Readonly<Record<string, string | undefined>> = process.env, ports: DependencyProvisionPortsV1 = dependencyProvisionPortsV1, osLanguages?: () => readonly string[]): Promise<void> {
  const owner = registry.commands.find((command) => command.id === "repo")!;
  const say = (name: string, values: Readonly<Record<string, string>>): string => Object.entries(values).reduce((text, [key, value]) => text.replaceAll(`{${key}}`, value), registry.labels[name]![resolveBuildHelpLanguageV1(registry.languages, [], environment, osLanguages)]!);
  ports.write(say("provisioning", { command: commandId }));
  const started = ports.now();
  try {
    await ports.run(process.execPath, [join(root, owner.module), "repo", "dependencies", "ensure"], root, "dependencies", provisionEnvironmentV1(environment), signal);
  } catch (error) {
    if (signal.aborted) {
      if (isRepoErrorV1(error) && error.code === "cancelled" && error.key === "process.execution.cancelled") throw signal.reason;
      throw error;
    }
    throw new Error(say("provisionFailed", { command: commandId, reason: error instanceof Error ? error.message : String(error) }), { cause: error });
  }
  ports.write(say("provisioned", { command: commandId, seconds: ((ports.now() - started) / 1000).toFixed(1) }));
}

/** 🥾️ Builds the gate that runs `repo dependencies ensure` before the module of a route that requires dependencies is loaded; a `staged` route with installed dependencies loads at once because it ensures them during its own staging. */
export function dependencyGateV1(root: string, registry: RootCommandRegistryV1, environment: Readonly<Record<string, string | undefined>> = process.env, ports: DependencyProvisionPortsV1 = dependencyProvisionPortsV1, osLanguages?: () => readonly string[]): RootCommandGateV1 {
  return async (command: RootCommandV1, segments: readonly string[], invocation: ScriptInvocation): Promise<void> => {
    const requires = rootCommandRequirementV1(command, segments);
    if (requires === "none" || ((requires === "staged" || buildOwnedInvocationV1(root, invocation)) && ports.available(root))) return;
    await advanceScriptInvocation(invocation, `dependencies:${command.id}`, "running");
    await ensureRootDependenciesV1(root, registry, command.id, invocation.control.signal, environment, ports, osLanguages);
  };
}
