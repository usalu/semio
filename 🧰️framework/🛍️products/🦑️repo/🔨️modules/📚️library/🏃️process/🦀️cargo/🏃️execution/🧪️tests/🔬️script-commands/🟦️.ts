import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { receiveScriptProcessInvocation, createIssuedResourceControlV1 } from "@semio-tech/process/routing";
import { readRepositoryCargoExecutionInputV1, runRepositoryCargoExecutionV1 } from "../../🟦️.ts";
import { dirname as scriptModuleDirname, resolve as scriptModuleResolve } from "node:path";
import { pathToFileURL as scriptModulePathToFileUrl } from "node:url";

/** 📍️ Location of the former `📜️script.ts` of this module, kept for code that resolves siblings of its entry. */
const scriptModulePath = scriptModuleResolve(import.meta.dir, "../../📜️script.ts");
const scriptModuleUrl = scriptModulePathToFileUrl(scriptModulePath).href;


/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  
  if(true){
   if(process.argv.length!==3||process.argv[2]!=="execute"||resolve(process.argv[1]!)!==fileURLToPath(scriptModuleUrl))throw new Error("Private Cargo execution endpoint requires its exact original command");
   await receiveScriptProcessInvocation(process.env,async invocation=>{const control=createIssuedResourceControlV1(invocation,"cargo:execution",0),stop=()=>process.stdin.destroy(invocation.control.signal.reason);invocation.control.signal.addEventListener("abort",stop,{once:true});try{const request=await readRepositoryCargoExecutionInputV1(process.cwd(),invocation,process.stdin);await runRepositoryCargoExecutionV1(process.cwd(),request,invocation,control,process.env);}finally{invocation.control.signal.removeEventListener("abort",stop);}});
  }
}
