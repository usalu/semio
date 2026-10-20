import { join, resolve } from "node:path";
import { advanceScriptInvocation, SCRIPT_PROCESS_INVOCATION_ENV, type ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { resolveBuildHelpLanguageV1 } from "../../../🏗️build/⚙️operations/📖️help/🟦️.ts";
import type { RootCommandGateV1, RootCommandRegistryV1, RootCommandV1 } from "../🟦️.ts";

/** 🔌️ The effects of provisioning, replaceable by a law that must not install anything. */
export type DependencyProvisionPortsV1 = Readonly<{
  run(command: string, args: readonly string[], cwd: string, label: string, environment: Readonly<Record<string, string | undefined>>, signal: AbortSignal): Promise<void>;
  write(line: string): void;
  now(): number;
}>;

/** 🏃️ Runs the owner of provisioning as an owned child: its output is prefixed, its progress repeats every 10 s, and cancellation ends its process tree. */
export const dependencyProvisionPortsV1: DependencyProvisionPortsV1 = {
  run: (command, args, cwd, label, environment, signal) => runOwnedCommand(command, [...args], cwd, label, 0, { env: environment, signal, outputStyle: "stream" }),
  write: (line) => console.log(line),
  now: () => Date.now(),
};

const ENVELOPE_VARIABLES: readonly string[] = [SCRIPT_PROCESS_INVOCATION_ENV, "SEMIO_SCRIPT_POLICY", "SEMIO_SCRIPT_CAPABILITIES"];

/** 🪪️ Whether the build CLI issued this invocation: it ensures the dependencies itself before it starts any task. */
export function buildOwnedInvocationV1(root: string, invocation: ScriptInvocation): boolean {
  const owned = invocation.capabilities as Readonly<{ workspaceRoot?: unknown; repositoryRoot?: unknown }>;
  return [owned.workspaceRoot, owned.repositoryRoot].some((value) => typeof value === "string" && resolve(value) === resolve(root));
}

/** 🧹️ The environment of the provisioning child: a fresh build CLI birth, never the envelope of this process. */
export function provisionEnvironmentV1(environment: Readonly<Record<string, string | undefined>>): Record<string, string | undefined> {
  return Object.fromEntries(Object.entries(environment).filter(([key]) => !ENVELOPE_VARIABLES.includes(key)));
}

/** 🥾️ Builds the gate that runs `repo dependencies ensure` before a command that requires dependencies is loaded; it reports progress in the help language and never swallows a failure. */
export function dependencyGateV1(root: string, registry: RootCommandRegistryV1, environment: Readonly<Record<string, string | undefined>> = process.env, ports: DependencyProvisionPortsV1 = dependencyProvisionPortsV1, osLanguages?: () => readonly string[]): RootCommandGateV1 {
  const owner = registry.commands.find((command) => command.id === "repo")!;
  const say = (name: string, values: Readonly<Record<string, string>>): string => Object.entries(values).reduce((text, [key, value]) => text.replaceAll(`{${key}}`, value), registry.labels[name]![resolveBuildHelpLanguageV1(registry.languages, [], environment, osLanguages)]!);
  return async (command: RootCommandV1, invocation: ScriptInvocation): Promise<void> => {
    if (command.requires === "none" || buildOwnedInvocationV1(root, invocation)) return;
    ports.write(say("provisioning", { command: command.id }));
    await advanceScriptInvocation(invocation, `dependencies:${command.id}`, "running");
    const started = ports.now();
    try {
      await ports.run(process.execPath, [join(root, owner.module), "repo", "dependencies", "ensure"], root, "dependencies", provisionEnvironmentV1(environment), invocation.control.signal);
    } catch (error) {
      if (invocation.control.signal.aborted) throw error;
      throw new Error(say("provisionFailed", { command: command.id, reason: error instanceof Error ? error.message : String(error) }), { cause: error });
    }
    ports.write(say("provisioned", { command: command.id, seconds: ((ports.now() - started) / 1000).toFixed(1) }));
  };
}
