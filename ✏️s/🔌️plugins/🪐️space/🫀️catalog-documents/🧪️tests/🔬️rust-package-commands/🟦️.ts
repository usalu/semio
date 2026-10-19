import { resolve } from "node:path";
import { receiveScriptProcessInvocation } from "@semio-tech/framework/process/routing";
import { runAdmittedCargoPackageTestsV1 } from "@semio-tech/framework/process/cargo-test";
import { resolve as scriptDirectoryResolve } from "node:path";

const SCRIPT_DIR = scriptDirectoryResolve(import.meta.dir, "../../📦️packages/🦀️rust");


/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  if(process.argv[2]!=="test"){
   const {runArtifactRustPackageMain}=await import("@semio-tech/framework/process/cargo-test");
   await runArtifactRustPackageMain(SCRIPT_DIR,"semio-s-space-core");
  }else{
  if(process.argv.length!==3)throw new Error("test accepts no overrides");
  await receiveScriptProcessInvocation(process.env,invocation=>runAdmittedCargoPackageTestsV1(["semio-s-space-core"],resolve(SCRIPT_DIR,"../.."),invocation,["--offline","--locked","--all-targets","--no-fail-fast","--","--nocapture"]));
  }
}
