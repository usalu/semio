import { test } from "bun:test";
import { mkdirSync } from "node:fs";
import { testTypeScriptSourceInputs } from "./⚡️inputs/🟦️.ts";

test("declared source images preserve bundler closures, current facts and cancellation",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
 if(!artifacts)throw Error("Original caller artifacts required");
 mkdirSync(artifacts,{recursive:true});
 await testTypeScriptSourceInputs(artifacts);
 console.log("[DEBUG] declared source import images match esbuild and Bun runtime outputs");
},30000);
