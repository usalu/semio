import {expect,test} from "bun:test";
import {createRequire} from "node:module";
import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {buildParserReceiptMatchesV1} from "../🟦️.ts";
import {ensureBuildEnvironmentV1} from "../../📜️script.ts";
import {createNativeBuildCallerCapabilitiesV1} from "../../../📥️caller/🟦️.ts";
import {createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🔣️.json";
const require=createRequire(import.meta.url),Ajv=require("ajv").default;

test("neutral bootstrap parser receipt requires exact binding and material",()=>{
 const independent=new Ajv({strict:true}).compile(schema),baseline=fixture.receipt;
 expect(independent(baseline)).toBe(true);expect(buildParserReceiptMatchesV1(baseline,structuredClone(baseline))).toBe(true);
 for(const field of fixture.changed){const changed={...baseline,[field]:"f".repeat(64)};expect(independent(changed)).toBe(true);expect(buildParserReceiptMatchesV1(baseline,changed)).toBe(false);}
 for(const changed of [{...baseline,ignored:true},{...baseline,parsers:[baseline.parsers[0],baseline.parsers[0]]}]){expect(buildParserReceiptMatchesV1(changed,changed)).toBe(false);}
 const changed=structuredClone(baseline);changed.parsers[0]!.digest=createHash("sha256").update("changed parser bytes").digest("hex");expect(buildParserReceiptMatchesV1(baseline,changed)).toBe(false);
});

test("actual issued bootstrap parser readiness avoids repeated whole workspace synchronization",()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bootstrap-parser-ready",maximumElapsedMilliseconds:120000},createNativeBuildCallerCapabilitiesV1(process.cwd(),["show","projects"],"nx",process.env),Date.now()),async original=>{
 const phaseControl=createIssuedResourceControlV1(original,"bootstrap:parser-fixture",0);let installs=0,published=0;const installer=async(root:string,invocation:Parameters<NonNullable<Parameters<typeof ensureBuildEnvironmentV1>[3]>>[1])=>{expect(root).toBe(process.cwd());expect(Object.is(original,invocation)).toBe(true);installs++;},publish=()=>{published++;};
 await ensureBuildEnvironmentV1(process.cwd(),original,phaseControl,installer,publish);const first=installs,started=performance.now();await ensureBuildEnvironmentV1(process.cwd(),original,phaseControl,installer,publish);
 expect(installs).toBe(first);expect(published).toBe(2);
 const oracle=createRequire(join(process.cwd(),"package.json")),typescript=oracle("typescript"),toml=oracle("@iarna/toml");expect(typescript.preProcessFile('import "./actual-source";').importedFiles.map((row:any)=>row.fileName)).toEqual(["./actual-source"]);expect(toml.parse('owned = "parser"')).toEqual({owned:"parser"});
}),130000);


test("actual pinned Nx resolves the declared workspace instead of inherited private launch roots",()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bootstrap-parser-workspace",maximumElapsedMilliseconds:30000},createNativeBuildCallerCapabilitiesV1(process.cwd(),["show","projects"],"nx",process.env),Date.now()),async original=>{
 const {nxBuildEnvironmentV1}=await import("../../../../⚡️caching/🚀️bootstrap/📜️script.ts"),{captureOwnedCommandV1}=await import("../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts"),{scriptInvocationEnvironment}=await import("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
 const {scriptCompilerAdmissionControlV1}=await import("../../../../../../../../🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts");const root=join(process.cwd(),fixture.workspace.root),environment=nxBuildEnvironmentV1(root,scriptInvocationEnvironment(original,{...process.env,NX_WORKSPACE_ROOT_PATH:join(process.cwd(),fixture.workspace.inherited)})),sdk=createRequire(join(process.env.SEMIO_NX_TOOLING_ROOT!,"package.json")),program='process.stdout.write(require(process.argv[1]).workspaceRoot)',actual=await captureOwnedCommandV1("node",["-e",program,sdk.resolve("nx/src/utils/workspace-root")],root,{control:scriptCompilerAdmissionControlV1(original,"bootstrap-parser-workspace",30000),env:environment});
 expect(actual).toBe(join(process.cwd(),fixture.workspace.expected));expect(environment.NX_WORKSPACE_ROOT_PATH).toBe(root);
}),35000);
