#!/usr/bin/env bun
/** 🌱️ Executes the shared authority's complete app and portable native laws. */
import {resolve} from "node:path";
import {receiveScriptProcessInvocation} from "../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runRepositoryCargoTests} from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
if(process.argv[2]!=="test"){
 const {runArtifactRustPackageMain}=await import("../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts");
 await runArtifactRustPackageMain(import.meta.dir,"semio-s-space-core");
}else{
if(process.argv.length!==3)throw Error("test accepts no overrides");
await receiveScriptProcessInvocation(process.env,invocation=>runRepositoryCargoTests(["semio-s-space-core"],resolve(import.meta.dir,"../.."),invocation,["--offline","--locked","--all-targets","--no-fail-fast","--","--nocapture"]));
}
