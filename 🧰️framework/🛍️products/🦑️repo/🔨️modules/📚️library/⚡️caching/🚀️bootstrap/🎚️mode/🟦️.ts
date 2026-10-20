import { buildModeEnvironment, readBuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import type {SelectedBuildTargetModeV1} from "../../../🏗️build/🧭️invocation/🟦️.ts";
import {fixedTargetBuildMode} from "../../../🏗️build/🎚️mode/🟨️.mjs";

type Selection = { args: string[]; env: NodeJS.ProcessEnv; watch?: string };

/** 🎯️ Resolves mode options before the task graph and daemon receive the environment. */
export function resolveNxBuildMode(selection: Selection, environment: NodeJS.ProcessEnv, declarations:readonly SelectedBuildTargetModeV1[]=[]): Selection {
  const delimiter = selection.args.indexOf("--"), head = selection.args.slice(0, delimiter < 0 ? undefined : delimiter), tail = delimiter < 0 ? [] : selection.args.slice(delimiter);
  if (!["run", "run-many", "affected"].includes(head[0]??"")) return selection;
  const configurations: string[] = [], args: string[] = [];
  for (let index = 0; index < head.length; index++) {
    const argument = head[index]!;
    if (argument === "--configuration" || argument === "-c") {const value=head[++index];if(!value||value.startsWith("-"))throw Error("Explicit compiler configuration value required");configurations.push(value);}
    else if (/^(?:--configuration|-c)=/.test(argument)) configurations.push(argument.slice(argument.indexOf("=") + 1));
    else args.push(argument);
  }
  if (args[0] === "run" && args[1]?.split(":").length === 3) {
    const parts = args[1].split(":"); configurations.push(parts.pop()!); args[1] = parts.join(":");
  }
  const targets = args[0] === "run" ? [args[1]?.split(":")[1] ?? ""] : args.flatMap((argument, index) => /^(?:--targets?|-t)=/.test(argument) ? argument.slice(argument.indexOf("=") + 1).split(",") : ["--target", "--targets", "-t"].includes(argument) ? (args[index + 1] ?? "").split(",") : []);
  const fixed = [...targets.map(fixedTargetBuildMode),...declarations.map(row=>row.fixed)].filter((value): value is "prototype" | "release" => value !== undefined);
  const authored = selection.env.SEMIO_BUILD_MODE;
  const inherited = environment.SEMIO_BUILD_MODE;
  const explicit = configurations.map(value => readBuildMode(value));
  const values = [...explicit, ...fixed, ...(authored === undefined ? [] : [readBuildMode(authored)]), ...(inherited === undefined ? [] : [readBuildMode(inherited)])];
  if(!values.length)values.push(...declarations.flatMap(row=>row.default===undefined?[]:[row.default]));
  const mode = values[0] ?? "prototype";
  if (values.some(value => value !== mode)) throw new Error(`Conflicting Nx build modes: ${values.join(", ")}`);
  const compilerArguments = tail.slice(1);
  if (targets.some(target => /^(?:build|wasm|native|component|cpp|package|extension-package|bundle)(?:-|$)/.test(target)) && compilerArguments.some(argument => ["--release", "--dist", "--dev", "--prototype", "--mode", "--build-mode"].includes(argument) || /^(?:--mode|--build-mode|--profile)=/.test(argument))) throw new Error("Select the Nx prototype or release configuration before --");
  return { ...selection, args: selection.args, env: { ...selection.env, ...buildModeEnvironment(mode, {}) } };
}
