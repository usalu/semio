import { createRequire } from "node:module";
import { join, relative, resolve } from "node:path";
import { buildModeEnvironment, readBuildMode, type BuildMode } from "../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";
import { runOwnedCommand, type OwnedCommandOptions } from "../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { repoCacheDirectory } from "../🟦️.ts";
import { withCompilerState } from "../🏗️state/🟦️.ts";

/** 🟦️ Checks one TypeScript project under its exclusive warm-state owner and original cancellation. */
export async function typecheckProject(options: Readonly<{ workspace: string; cwd: string; project: string; owner: string; args?: readonly string[]; mode?: BuildMode; timeoutMs: number; execution?: OwnedCommandOptions }>): Promise<void> {
  const environment = options.execution?.env ?? process.env, mode = options.mode ?? readBuildMode(environment.SEMIO_BUILD_MODE), project = resolve(options.cwd, options.project);
  const require = createRequire(join(options.cwd, "package.json")), compiler = require.resolve("typescript/bin/tsc"), version = require("typescript/package.json").version;
  const args = [...(options.args ?? [])];
  if (args.some(value => /^(?:-p|--project|--incremental|--tsBuildInfoFile)(?:=|$)/u.test(value))) throw Error("TypeScript project and incremental state are owned by the typecheck command");
  const signal = options.execution?.signal ?? new AbortController().signal;
  await withCompilerState({ root: repoCacheDirectory(options.workspace), owner: options.owner, compiler: `typescript:${version}`, configuration: JSON.stringify([relative(options.workspace, project), args]), mode, signal }, async directory => {
    await runOwnedCommand(process.execPath, [compiler, "--noEmit", "-p", project, ...args, "--incremental", "true", "--tsBuildInfoFile", join(directory, "state.tsbuildinfo")], options.cwd, options.owner, options.timeoutMs, { ...options.execution, env: buildModeEnvironment(mode, environment), signal });
  });
}
