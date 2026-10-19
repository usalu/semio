import {receiveScriptProcessInvocation,scriptInvocationBudget} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runOwnedCommand} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import {admitReadinessDeclarationV1,readinessMarkerV1,resolveReadinessV1,watchReadinessV1} from "../../../../🏗️build/🟢️readiness/🟦️.ts";
import schema from "./🧬️schema/📥️source.json" with {type:"json"};
import {readFile} from "node:fs/promises";
import {resolve} from "node:path";
import {createHash} from "node:crypto";


/** 🚪️ Executes the private command source through the original invocation and portable shell. */
export async function runNxCommandSourceV1(args:readonly string[],environment:NodeJS.ProcessEnv=process.env):Promise<void>{
 if(args.length!==1||!/^[a-f0-9]{64}$/u.test(args[0]!)||typeof environment.SEMIO_NX_COMMAND_DIRECTORY!=="string"||typeof environment.SEMIO_SCRIPT_PROCESS_INVOCATION!=="string")throw new Error("Private Nx command source required");
 const bytes=await readFile(resolve(environment.SEMIO_NX_COMMAND_DIRECTORY,args[0]+".json"));if(createHash("sha256").update(bytes).digest("hex")!==args[0])throw new Error("Private Nx command source digest differs");
 const source=JSON.parse(bytes.toString("utf8"));
 if(validateJsonSchemaSubset(schema,source).length)throw new Error("Private Nx command source refused");
 if(source.invocation!==createHash("sha256").update(environment.SEMIO_SCRIPT_PROCESS_INVOCATION).digest("hex"))throw new Error("Original Nx command invocation differs");
 await receiveScriptProcessInvocation(environment,async invocation=>{
  const env={...environment};delete env.SEMIO_NX_COMMAND_RUNNER;delete env.SEMIO_NX_COMMAND_DIRECTORY;
  const watch=source.ready===undefined?undefined:watchReadinessV1(resolveReadinessV1(admitReadinessDeclarationV1(source.ready,"Nx command"),env,"Nx command"),invocation.control.signal);
  if(watch)void watch.ready.then(address=>console.log(`${readinessMarkerV1} ${address}`));
  try{await runOwnedCommand(process.execPath,["exec",source.command],process.cwd(),"build:nx-command",scriptInvocationBudget(invocation,0),{env,signal:invocation.control.signal,...(watch?{onLine:line=>watch.observe(line+"\n")}:{})});}finally{watch?.stop();}
 });
}
/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  
  if(true)await runNxCommandSourceV1(process.argv.slice(2));
}
