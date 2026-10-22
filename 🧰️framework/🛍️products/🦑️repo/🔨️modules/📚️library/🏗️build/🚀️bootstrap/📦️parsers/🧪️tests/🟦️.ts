import {expect,test} from "bun:test";
import "./📥️resolution/🟦️.ts";
import {createRequire} from "node:module";
import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {buildParserReceiptMatchesV1,readBuildParserInspectionV1} from "../🟦️.ts";
import {ensureBuildEnvironmentV1} from "../../🟦️.ts";
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
 const {nxBuildEnvironmentV1}=await import("../../../../⚡️caching/🚀️bootstrap/📜️script.ts"),{captureOwnedCommandV1}=await import("@semio-tech/process/execution"),{scriptInvocationEnvironment}=await import("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
 const {scriptCompilerAdmissionControlV1}=await import("@semio-tech/process/compiler-capacity");const root=join(process.cwd(),fixture.workspace.root),environment=nxBuildEnvironmentV1(root,scriptInvocationEnvironment(original,{...process.env,NX_WORKSPACE_ROOT_PATH:join(process.cwd(),fixture.workspace.inherited)})),sdk=createRequire(join(process.env.SEMIO_NX_TOOLING_ROOT!,"package.json")),program='process.stdout.write(require(process.argv[1]).workspaceRoot)',actual=await captureOwnedCommandV1("node",["-e",program,sdk.resolve("nx/src/utils/workspace-root")],root,{control:scriptCompilerAdmissionControlV1(original,"bootstrap-parser-workspace",30000),env:environment});
 expect(actual).toBe(join(process.cwd(),fixture.workspace.expected));expect(environment.NX_WORKSPACE_ROOT_PATH).toBe(root);
}),35000);


test("Bootstrap publishes missing parser receipts from proven material without reinstalling",async()=>{
 const ts=require("typescript")as typeof import("typescript"),source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("bootstrap-inspection.ts",source,ts.ScriptTarget.Latest,true),owner=tree.statements.find((node):node is import("typescript").FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text==="ensureBuildEnvironmentV1");if(!owner?.body)throw Error("Actual bootstrap owner required");let readBranch:import("typescript").IfStatement|undefined;const missing:import("typescript").ReturnStatement[]=[];const visit=(node:import("typescript").Node)=>{if(ts.isIfStatement(node)&&node.expression.getText(tree)==="!publishReceipt")readBranch=node;if(ts.isCatchClause(node)){const returned=node.block.statements.find(ts.isReturnStatement);if(returned)missing.push(returned);}ts.forEachChild(node,visit);};visit(owner);if(!readBranch||missing.length!==2)throw Error("Actual parser material and receipt boundaries required");const start=owner.body.statements.findIndex(node=>node.getText(tree).includes("await inspect(false)"));if(start<0)throw Error("Actual parser action boundary required");const compile=(code:string)=>ts.transpileModule(code,{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,tail=owner.body.statements.slice(start).map(node=>node.getText(tree)).join("\n"),db=new(require("bun:sqlite").Database)(":memory:");
 try{for(const row of fixture.bootstrapCases){expect(new Ajv({strict:true}).validate(schema.definitions.BootstrapCase,row)).toBe(true);const inspectPorts={lstatSync:()=>row.receipt?{}:undefined,path:"/owned/receipt",bootstrap:{maximumReceiptBytes:16384},bounded:()=>Buffer.from("{}"),current:{},buildParserReceiptMatchesV1:()=>row.matches},read=new Function(...Object.keys(inspectPorts),compile("return function(publishReceipt){"+readBranch.getText(tree)+"};"))(...Object.values(inspectPorts)),material=new Function(compile("return function(){"+missing[0]!.getText(tree)+"};"))(),actual=row.material?read(false):material(),oracle=db.query("select case when ?=0 then 'material-required' when ?=1 and ?=1 then 'ready' else 'receipt-required' end state").get(Number(row.material),Number(row.receipt),Number(row.matches))as{state:string};expect(actual,row.id).toBe(row.state);expect(actual,row.id).toBe(oracle.state);let installs=0,writes=0,failed=false,observed:unknown;const marker=Object.freeze(Error("Actual writer recheck refused changed material")),ports={readBuildParserInspectionV1,inspect:async(write:boolean)=>{if(!write)return actual;writes++;if(row.writeFails)throw marker;return"ready";},root:"/workspace",phaseControl:{fixture:"original-phase"},check:()=>{},invocation:{policy:{owner:"fixture"},control:{publish:async()=>{}}},install:async()=>{installs++;},recheck:async()=>false},run=new Function(...Object.keys(ports),compile("return async function(){"+tail+"};"))(...Object.values(ports));try{await run();}catch(error){failed=true;observed=error;}expect(installs,row.id).toBe(row.installs);expect(writes,row.id).toBe(row.writes);expect(failed,row.id).toBe(row.failure);if(failed)expect(observed).toBe(marker);console.log(`[DEBUG] parser bootstrap ${row.id} state=${actual} installers=${installs} receiptWriters=${writes} failed=${failed}`);}
 for(const value of[undefined,null,false,{},[],"ready-ish"])expect(()=>readBuildParserInspectionV1(value)).toThrow("Closed parser bootstrap inspection");
 }finally{db.close();}
});
