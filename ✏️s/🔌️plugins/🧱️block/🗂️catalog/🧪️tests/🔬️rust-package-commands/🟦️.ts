import { resolve } from "node:path";
import { runArtifactRustPackageMain } from "@semio-tech/framework/process/cargo-test";
import { runOwnedCommand } from "@semio-tech/framework/process/execution";
import { resolve as scriptDirectoryResolve } from "node:path";

const SCRIPT_DIR = scriptDirectoryResolve(import.meta.dir, "../../📦️packages/🦀️rust");


/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  const [command,...args]=process.argv.slice(2);
  if(command==="conformance"){if(args.length)throw new Error("conformance accepts no arguments");await runOwnedCommand(process.execPath,["test",resolve(SCRIPT_DIR,"../../🧪️tests/🟦️.ts")],resolve(SCRIPT_DIR,"../../../../../.."),"block-catalog-conformance",45000,{env:process.env});}else await runArtifactRustPackageMain(SCRIPT_DIR,"semio-s-plugin-block-catalog");
}
