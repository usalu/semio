#!/usr/bin/env bun
/** 🌱️ Executes the shared authority's complete app and portable native laws. */
import {resolve} from "node:path";
import { receiveScriptProcessInvocation } from "../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runAdmittedCargoPackageTestsV1 } from "../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
if(process.argv[2]!=="test"){
 const {runArtifactRustPackageMain}=await import("../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts");
 await runArtifactRustPackageMain(import.meta.dir,"semio-s-space-core");
}else{
if(process.argv.length!==3)throw Error("test accepts no overrides");
await receiveScriptProcessInvocation(process.env,invocation=>runAdmittedCargoPackageTestsV1(["semio-s-space-core"],resolve(import.meta.dir,"../.."),invocation,["--offline","--locked","--all-targets","--no-fail-fast","--","--nocapture"]));
}
