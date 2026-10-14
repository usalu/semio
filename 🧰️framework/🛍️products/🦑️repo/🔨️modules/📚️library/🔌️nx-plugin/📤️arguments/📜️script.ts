#!/usr/bin/env bun
import {receiveScriptProcessInvocation,scriptInvocationBudget} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runOwnedCommand} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import schema from "./🧬️schema/📥️source.json" with {type:"json"};
import {readFile} from "node:fs/promises";
import {resolve} from "node:path";
import {createHash} from "node:crypto";

/** 🚪️ Executes the private command source through the original invocation and portable shell. */
export async function runNxCommandSourceV1(args:readonly string[],environment:NodeJS.ProcessEnv=process.env):Promise<void>{
 if(args.length!==1||!/^[a-f0-9]{64}$/u.test(args[0]!)||typeof environment.SEMIO_NX_COMMAND_DIRECTORY!=="string"||typeof environment.SEMIO_SCRIPT_PROCESS_INVOCATION!=="string")throw Error("Private Nx command source required");
 const bytes=await readFile(resolve(environment.SEMIO_NX_COMMAND_DIRECTORY,args[0]+".json"));if(createHash("sha256").update(bytes).digest("hex")!==args[0])throw Error("Private Nx command source digest differs");
 const source=JSON.parse(bytes.toString("utf8"));
 if(validateJsonSchemaSubset(schema,source).length)throw Error("Private Nx command source refused");
 if(source.invocation!==createHash("sha256").update(environment.SEMIO_SCRIPT_PROCESS_INVOCATION).digest("hex"))throw Error("Original Nx command invocation differs");
 await receiveScriptProcessInvocation(environment,async invocation=>{
  const env={...environment};delete env.SEMIO_NX_COMMAND_RUNNER;delete env.SEMIO_NX_COMMAND_DIRECTORY;
  await runOwnedCommand(process.execPath,["exec",source.command],process.cwd(),"build:nx-command",scriptInvocationBudget(invocation,0),{env,signal:invocation.control.signal});
 });
}

if(import.meta.main)await runNxCommandSourceV1(process.argv.slice(2));
