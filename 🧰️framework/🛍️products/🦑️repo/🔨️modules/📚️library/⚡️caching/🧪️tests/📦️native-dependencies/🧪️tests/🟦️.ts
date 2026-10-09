import {test} from "bun:test";
import {testNativeDependencies} from "../🟦️.ts";

test("complete original native dependency source and preparation receiving laws",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Original native dependency receiver requires its explicit artifact root");await testNativeDependencies(process.cwd(),output);console.log("[DEBUG] complete original native dependency source/preparation laws retained all actual Cargo/Nx/esbuild/Ajv/lock/closure/mutation assertions");
},120000);
