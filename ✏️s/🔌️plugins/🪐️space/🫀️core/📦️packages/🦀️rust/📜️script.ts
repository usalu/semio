#!/usr/bin/env bun
/** 🌱️ Executes the shared authority's complete app and portable native laws. */
import {resolve} from "node:path";
import {readMutationInventoryExecutionPolicy} from "../../../../../../🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts";
import {acquireCargoBuildLeaseV1} from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import {readCargoTestPolicyV1,runCargoTestsV1} from "../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
if(process.argv[2]!=="test"){
 const {runArtifactRustPackageMain}=await import("../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts");
 await runArtifactRustPackageMain(import.meta.dir,"semio-s-space-core");
}else{
if(process.argv.length!==3)throw Error("test accepts no overrides");
const manifestPath=resolve(import.meta.dir,"Cargo.toml"),cwd=resolve(import.meta.dir,"../.."),controller=new AbortController(),cancel=()=>controller.abort(),policy=readMutationInventoryExecutionPolicy(process.env);
process.once("SIGINT",cancel);process.once("SIGTERM",cancel);
if(process.env.CARGO_TARGET_DIR!==policy.compilerStorage.targetDirectory||process.env.CARGO_BUILD_BUILD_DIR!==policy.compilerStorage.buildDirectory)throw Error("Compiler custody differs from the authored shared policy");
const lease=await acquireCargoBuildLeaseV1({directory:policy.compilerStorage.leaseDirectory,buildDirectory:policy.compilerStorage.buildDirectory,args:process.argv.slice(2),signal:controller.signal,onWait:()=>console.error("[DEBUG] Space core waiting for compiler lease")});
try{await runCargoTestsV1({manifestPath,packages:["semio-s-space-core"],cwd,extraArgs:["--offline","--locked","--all-targets","--features","component-app-assembly","--no-fail-fast","--","--nocapture"],environment:process.env,signal:controller.signal},readCargoTestPolicyV1(process.env));}
finally{lease.release();process.off("SIGINT",cancel);process.off("SIGTERM",cancel);}

}
