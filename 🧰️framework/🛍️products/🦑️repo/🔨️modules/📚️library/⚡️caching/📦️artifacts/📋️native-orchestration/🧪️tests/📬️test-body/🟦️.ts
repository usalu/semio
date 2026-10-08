import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve,dirname} from "node:path";
import ts from "typescript";
import {nativeOwnerTestManifestRequestV1} from "../../🗺️owner-test-manifests/🟦️.ts";
const owner=resolve(import.meta.dir,"../.."),fixture=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/📬️test-body/🔣️.json"),"utf8"));
const source=ts.createSourceFile("native.ts",readFileSync(resolve(owner,"🟦️.ts"),"utf8"),ts.ScriptTarget.Latest,true);
const native=source.statements.find((node:any)=>node.name?.text==="NativeScript")!.getText(source).replace(/^export /,"");
const library=resolve(owner,"../../.."),bodySource=ts.createSourceFile("library.ts",readFileSync(resolve(library,"🟦️.ts"),"utf8"),ts.ScriptTarget.Latest,true);
const consume=bodySource.statements.find((node:any)=>node.name?.text==="runRepositoryCargoTests")!.getText(bodySource).replace(/^export /,"");
test("typed owner route prepares exactly at current Cargo consumption and preserves external preparation",async()=>{
 const oracle=Bun.spawnSync(["node","--eval",`const cases=${JSON.stringify(fixture.cases)};console.log(JSON.stringify(cases.map(row=>{const valid=row.route==='owner-command'||row.command==='bun'&&row.args[0]==='./📜️script.ts'&&row.args[1]==='test';return valid?(row.route==='owner-command'?['prepare:1','owned','prepare:2','cargo:2']:['owned','prepare:2','cargo:2']):['refused']})))`]);expect(oracle.exitCode).toBe(0);expect(JSON.parse(new TextDecoder().decode(oracle.stdout))).toEqual(fixture.cases.map((row:any)=>row.expected));
 for(const compile of [(text:string)=>new Bun.Transpiler({loader:"ts"}).transformSync(text),(text:string)=>ts.transpileModule(text,{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.ESNext}}).outputText]){
  for(const row of fixture.cases){
   const records:string[]=[];let version=1;
   const prepare=()=>records.push(`prepare:${version}`),policy=()=>({manifestPath:"/repo/owner/Cargo.toml",version});
   const consumeBody=new Function("getWorkspaceRoot","cargoRepositoryPackageSelections","cargoWorkspaceForManifest","resolve","relative","prepareCargoWorkspaceInvocation","repositoryCargoTestPolicyV1","runCargoTestsV1","TEST_LEVELS","packageTestBudgetMs",compile(consume)+";return runRepositoryCargoTests;")(()=>"/repo",()=>[{manifest:"owner/Cargo.toml",name:"owner"}],()=>({directory:"owner"}),resolve,()=>"owner/Cargo.toml",prepare,policy,async(_request:any,current:any)=>records.push(`cargo:${current.version}`),["quick"],()=>100);
   const owned=async()=>{records.push("owned");version=2;await consumeBody(["owner"],"/repo/owner",[],{});};
   const Native=new Function("BundleScript","nativeOwnerTestManifestRequestV1","resolve","dirname","readFileSync","Bun","prepareCargoWorkspaceInvocation","repositoryVitestPolicyV1","repositoryProcessOwnerContextV1","repositoryCargoArtifactBuildPolicyV1","repositoryCargoTestPolicyV1","runOwnedCommand","process",compile(native)+";return NativeScript;")(class{repoRoot="/repo"},nativeOwnerTestManifestRequestV1,resolve,dirname,()=>"[package]\nname=\"owner\"",{TOML:Bun.TOML},prepare,()=>({}),()=>({}),()=>({}),policy,owned,{env:{},execPath:"bun",stderr:{write(){}}});
   try{await new Native().run([row.route,"--manifest","owner/Cargo.toml","--cwd","owner","--",row.command,...row.args]);}catch{records.push("refused");}
   expect(records,row.id).toEqual(row.expected);
  }
 }
 console.log("[DEBUG] actual native wrapper and actual Cargo consumer executed through Bun/TypeScript; independent Node argv oracle preserves external preparation and fresh current body preparation");
});

test("canonical Nx target route requires explicit current Cargo consumer ownership",()=>{
 const syntax=ts.createSourceFile("nx.mjs",readFileSync(resolve(library,"🟨️.mjs"),"utf8"),ts.ScriptTarget.Latest,true);
 const declaration=syntax.statements.find((node:any)=>node.name?.text==="nativeOwnerExecutionRoute")!.getText(syntax).replace(/^export /,"");
 const classify=new Function(declaration+";return nativeOwnerExecutionRoute;")();
 for(const row of fixture.targets){let result;try{result=classify(row.target);}catch{result="refused";}expect(result,row.id).toBe(row.expected);}
 const roots=["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust","✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📦️packages/🦀️rust","✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust","✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust","✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/📦️packages/🦀️rust"];
 const repository=resolve(library,"../../../../..");
 for(const root of roots){const project=JSON.parse(readFileSync(resolve(repository,root,"📋️project.json"),"utf8"));expect(classify(project.targets.test),project.name).toBe("repository-test-body");}
 for(const root of ["🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust","🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust"]){const project=JSON.parse(readFileSync(resolve(repository,root,"📋️project.json"),"utf8"));expect(classify(project.targets.test),project.name).toBe("owner-command");}
 console.log("[DEBUG] Nx5 verified Cargo-consuming test owners/level siblings selected;2 direct neutral driver owners preserve eager preparation; custom declarations refused");
});
