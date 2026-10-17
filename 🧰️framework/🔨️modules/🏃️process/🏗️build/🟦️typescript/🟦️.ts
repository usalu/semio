import { createRequire } from "node:module";
import { join, relative, resolve } from "node:path";
import { buildModeEnvironment, type BuildMode } from "../🎚️mode/🟦️.ts";
import { runOwnedCommand, type OwnedCommandOptions } from "../../🎛️owned-execution/🟦️.ts";
import { advanceScriptInvocation, type ScriptInvocation } from "../../🧭️routing/📥️invocation/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { withCompilerState } from "../🗃️state/🟦️.ts";
import { compilerAdmissionBudgetV1,type CompilerAdmissionControl } from "../🎟️capacity/🟦️.ts";
import {requireIssuedResourceControlInvocationV1,requireIssuedResourceControlV1,requireIssuedResourceControlRefinementV1,withRequestedIssuedResourceControlV1}from"../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {readNativeStorageReservationPolicyV1,type NativeStorageReservationV1}from"../../📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import schema from "./🧬️schema/🔣️.json";

/** 🟦️ One caller-owned TypeScript check with explicit warm state and original process authority. */
export type TypeScriptCheckOptions = Readonly<{ workspace: string; stateRoot: string; storage:NativeStorageReservationV1;storageControl:CompilerAdmissionControl;control:CompilerAdmissionControl;cwd: string; project: string; owner: string; args?: readonly string[]; mode: BuildMode; timeoutMs: number; invocation: ScriptInvocation; execution?: Omit<OwnedCommandOptions, "signal"> }>;

/** 🧾️ Admits only finite diagnostic switches while keeping output, project and state configuration authored. */
export function readTypeScriptCheckArgumentsV1(value:readonly string[]):readonly string[]{if(!Array.isArray(value)||value.length>schema.definitions.Arguments.maxItems)throw Error("Closed TypeScript diagnostic arguments required");const captured=[...value];if(validateJsonSchemaSubset(schema.definitions.Arguments,captured,schema).length)throw Error("Closed TypeScript diagnostic arguments required");return Object.freeze(captured);}

/** 🧯️ Disables independent process bytecode stores before the compiler child is born. */
export function typeScriptCheckEnvironmentV1(mode:BuildMode,environment:Readonly<Record<string,string|undefined>>):Readonly<Record<string,string|undefined>>{const captured=buildModeEnvironment(mode,{...environment,...schema.definitions.CompilerEnvironment.const});delete captured.NODE_COMPILE_CACHE;return Object.freeze(captured);}

/** 🔍️ Checks one project while preserving compiler-state exclusion and original cancellation, deadline and progress. */
export async function typecheckProject(options: TypeScriptCheckOptions): Promise<void> {
  const incoming=options.control,invocation=options.invocation;requireIssuedResourceControlInvocationV1(incoming,invocation);
  const storageControl=options.storageControl;requireIssuedResourceControlV1(storageControl);requireIssuedResourceControlRefinementV1(storageControl,incoming);const storage=options.storage;readNativeStorageReservationPolicyV1(storage,storageControl);
  const failures = validateJsonSchemaSubset(schema, Object.fromEntries(Object.entries(options).filter(([key, value]) => !["invocation", "execution", "control", "storage", "storageControl"].includes(key) && value !== undefined)));
  if (failures.length) throw Error("Invalid TypeScript check: " + failures.join("; "));
  if (options.execution && Object.keys(options.execution).some(key => !["stdout", "output", "env", "onLine", "onProgress"].includes(key))) throw Error("TypeScript output options cannot replace original control");
  return withRequestedIssuedResourceControlV1(incoming,options.timeoutMs,async control=>{
  await advanceScriptInvocation(options.invocation, "typecheck", "running");
  const environment = options.execution?.env ?? process.env, mode = options.mode, project = resolve(options.cwd, options.project);
  const require = createRequire(join(options.cwd, "package.json")), compiler = require.resolve("typescript/bin/tsc"), version = require("typescript/package.json").version;
  const args = readTypeScriptCheckArgumentsV1(options.args??[]);
  await withCompilerState({ root: options.stateRoot, owner: options.owner, compiler: `typescript:${version}`, configuration: JSON.stringify([relative(options.workspace, project), args]), mode, control,storage,storageControl,files: ["state.tsbuildinfo"], environment, capacity:{kind:"exact",parallelism:1} }, async (directory, compilerEnvironment,actualControl) => {
    await advanceScriptInvocation(options.invocation, "typecheck", "running");
    await runOwnedCommand(process.execPath, [compiler, "--noEmit", "-p", project, ...args, "--incremental", "true", "--tsBuildInfoFile", join(directory, "state.tsbuildinfo")], options.cwd, options.owner, compilerAdmissionBudgetV1(actualControl), { ...options.execution, env: typeScriptCheckEnvironmentV1(mode, compilerEnvironment), signal:actualControl.signal });
  });
  await advanceScriptInvocation(options.invocation, "typecheck", "complete");
  });
}
