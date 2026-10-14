#!/usr/bin/env bun
import {resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {receiveScriptProcessInvocation} from "@semio-tech/process/routing";
import {readRepositoryCargoExecutionInputV1,runRepositoryCargoExecutionV1} from "./🟦️.ts";

if(import.meta.main){
 if(process.argv.length!==3||process.argv[2]!=="execute"||resolve(process.argv[1]!)!==fileURLToPath(import.meta.url))throw Error("Private Cargo execution endpoint requires its exact original command");
 await receiveScriptProcessInvocation(process.env,async invocation=>{const stop=()=>process.stdin.destroy(invocation.control.signal.reason);invocation.control.signal.addEventListener("abort",stop,{once:true});try{const request=await readRepositoryCargoExecutionInputV1(process.cwd(),invocation,process.stdin);await runRepositoryCargoExecutionV1(process.cwd(),request,invocation,process.env);}finally{invocation.control.signal.removeEventListener("abort",stop);}});
}
