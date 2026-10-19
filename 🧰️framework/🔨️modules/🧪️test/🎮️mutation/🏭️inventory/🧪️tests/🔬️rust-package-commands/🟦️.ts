import { createIssuedResourceControlV1 } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { runOwnedCommand } from "../../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { readCargoTestPolicyV1, runAdmittedCargoTestsV1 as runCargoTestsV1 } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { resolve as scriptDirectoryResolve } from "node:path";

const SCRIPT_DIR = scriptDirectoryResolve(import.meta.dir, "../../📦️packages/🦀️rust");


/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  const [command,...args]=process.argv.slice(2),root=resolve(SCRIPT_DIR,"../../../../../../.."),manifestPath=resolve(SCRIPT_DIR,"Cargo.toml");
  if(command==="conformance"){if(args.length)throw new Error("conformance accepts no arguments");await runOwnedCommand(process.execPath,["test",resolve(SCRIPT_DIR,"../../🧪️tests/🟦️.ts")],root,"mutation-inventory-conformance",45000,{env:process.env});}
  else if(command==="conformance-process"){if(args.length)throw new Error("conformance-process accepts no arguments");const {readMutationInventoryExecutionPolicy}=await import("../../🏃️execution/🟦️.ts");const policy=readMutationInventoryExecutionPolicy(process.env);await runOwnedCommand(process.execPath,["test",resolve(SCRIPT_DIR,"../../🏃️execution/🧪️tests/🟦️.ts")],root,"mutation-inventory-process-conformance",policy.budgetMs,{env:process.env});}
  else if(command==="conformance-command"){if(args.length)throw new Error("conformance-command accepts no arguments");const {readMutationInventoryExecutionPolicy}=await import("../../🏃️execution/🟦️.ts");const policy=readMutationInventoryExecutionPolicy(process.env);await runOwnedCommand(process.execPath,["test",resolve(SCRIPT_DIR,"../../🏃️execution/🚪️command/🧪️tests/🟦️.ts")],root,"mutation-inventory-command-conformance",policy.budgetMs,{env:process.env});}
  else if(command==="test")await receiveScriptProcessInvocation(process.env,original=>{const cargoPhase=createIssuedResourceControlV1(original,"cargo:script-entry",0);return runCargoTestsV1({manifestPath,packages:["semio-framework-test-mutation-inventory"],cwd:root,extraArgs:["--lib",...args],environment:process.env},readCargoTestPolicyV1(process.env),root,original,cargoPhase);});
  else throw new Error("conformance|conformance-process|conformance-command|test");
}
