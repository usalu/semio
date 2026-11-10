/** 🔏️ Contract: the backend door loads its registry validation lazily, and that lazy import is declared to the sealed implementation closure, so a service that runs from the captured closure never refuses it as an uncaptured implementation import. */
import {expect,test} from "bun:test";
import {relative} from "node:path";
import {fileURLToPath} from "node:url";
import {declareLibraryBuildTestEnvelopeV1} from "../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {createIssuedResourceControlV1} from "../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {captureCompilerBuildSourceClosureV1} from "../../../📥️source/🔏️closure/🏗️compiler/🟦️.ts";

declareLibraryBuildTestEnvelopeV1({suite:"build-tool-setup",maximumElapsedMilliseconds:300000});

test("the lazily imported registry validation is part of the captured implementation closure of the backend door",()=>withBuildTestInvocationV1(async original=>{
 const root=process.cwd(),door=relative(root,fileURLToPath(new URL("../../🟦️.ts",import.meta.url))).replaceAll("\\","/"),registry=relative(root,fileURLToPath(new URL("../../📜️registry/🟦️.ts",import.meta.url))).replaceAll("\\","/");
 const closure=await captureCompilerBuildSourceClosureV1(root,door,original,createIssuedResourceControlV1(original,"test:backend-door-closure",240000));
 expect(closure.files.map(file=>file.path)).toContain(registry);
 expect(Object.keys(closure.files.find(file=>file.path===door)!.imports),"the door declares the lazy import to the loader").toContain("./📜️registry/🟦️.ts");
}),290000);

test("modules that other doors load lazily with a synchronous require are captured too: the sealed store's manager data and the Bazel status leaf's validator and texts",()=>withBuildTestInvocationV1(async original=>{
 const root=process.cwd(),path=(relativePath:string)=>relative(root,fileURLToPath(new URL(relativePath,import.meta.url))).replaceAll("\\","/"),status=path("../../../🔧️tools/🏗️bazel/🧵️sessions/📖️status/🟦️.ts");
 const closure=await captureCompilerBuildSourceClosureV1(root,status,original,createIssuedResourceControlV1(original,"test:bazel-status-closure",240000)),paths=closure.files.map(file=>file.path);
 expect(paths).toContain(path("../../../📖️texts/🟦️.ts"));
 expect(paths).toContain("🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts");
}),290000);
