import { createRequire } from "node:module";
import { join, relative, resolve } from "node:path";
import { buildModeEnvironment, readBuildMode, type BuildMode } from "../🎚️mode/🟦️.ts";
import { runOwnedCommand, type OwnedCommandOptions } from "../../🎛️owned-execution/🟦️.ts";
import { advanceScriptInvocation, type ScriptInvocation } from "../../🧭️routing/📥️invocation/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { withCompilerState } from "../🗃️state/🟦️.ts";
import { scriptCompilerAdmissionControlV1, compilerAdmissionBudgetV1 } from "../🎟️capacity/🟦️.ts";
import schema from "./🧬️schema/🔣️.json";

/** 🟦️ One caller-owned TypeScript check with explicit warm state and original process authority. */
export type TypeScriptCheckOptions = Readonly<{ workspace: string; stateRoot: string; leaseDirectory: string; cwd: string; project: string; owner: string; args?: readonly string[]; mode?: BuildMode; timeoutMs: number; invocation: ScriptInvocation; execution?: Omit<OwnedCommandOptions, "signal"> }>;

/** 🔍️ Checks one project while preserving compiler-state exclusion and original cancellation, deadline and progress. */
export async function typecheckProject(options: TypeScriptCheckOptions): Promise<void> {
  const failures = validateJsonSchemaSubset(schema, Object.fromEntries(Object.entries(options).filter(([key, value]) => !["invocation", "execution"].includes(key) && value !== undefined)));
  if (failures.length) throw Error("Invalid TypeScript check: " + failures.join("; "));
  if (options.execution && Object.keys(options.execution).some(key => !["stdout", "output", "env", "onLine", "onProgress"].includes(key))) throw Error("TypeScript output options cannot replace original control");
  const control = scriptCompilerAdmissionControlV1(options.invocation, "typecheck", options.timeoutMs);
  await advanceScriptInvocation(options.invocation, "typecheck", "running");
  const environment = options.execution?.env ?? process.env, mode = options.mode ?? readBuildMode(environment.SEMIO_BUILD_MODE), project = resolve(options.cwd, options.project);
  const require = createRequire(join(options.cwd, "package.json")), compiler = require.resolve("typescript/bin/tsc"), version = require("typescript/package.json").version;
  const args = [...(options.args ?? [])];
  if (args.some(value => /^(?:-p|--project|--incremental|--tsBuildInfoFile)(?:=|$)/u.test(value))) throw Error("TypeScript project and incremental state are owned by the typecheck command");
  const signal = options.invocation.control.signal;
  await withCompilerState({ root: options.stateRoot, owner: options.owner, compiler: `typescript:${version}`, configuration: JSON.stringify([relative(options.workspace, project), args]), mode, control, leaseDirectory: options.leaseDirectory, files: ["state.tsbuildinfo"], environment, requestedParallelism: 1 }, async (directory, compilerEnvironment) => {
    await advanceScriptInvocation(options.invocation, "typecheck", "running");
    await runOwnedCommand(process.execPath, [compiler, "--noEmit", "-p", project, ...args, "--incremental", "true", "--tsBuildInfoFile", join(directory, "state.tsbuildinfo")], options.cwd, options.owner, compilerAdmissionBudgetV1(control), { ...options.execution, env: buildModeEnvironment(mode, compilerEnvironment), signal });
  });
  await advanceScriptInvocation(options.invocation, "typecheck", "complete");
}
