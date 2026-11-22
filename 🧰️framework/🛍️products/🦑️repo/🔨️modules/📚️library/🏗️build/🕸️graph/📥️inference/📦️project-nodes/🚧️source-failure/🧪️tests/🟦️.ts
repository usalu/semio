/** 🚧️ Contract: a module that an owner's own source imports and that is not there (a replica moved it a moment ago) withholds that owner alone, as a recorded observation that holds until the module appears; a module of the Build tooling itself stays a stale implementation, and a failure that names no source stays what it was.
 * @see ../🟨️.mjs */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,rmSync,writeFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{staleImplementationOfV1}from"../../../../../🧷️format-stamps/🟨️.mjs";
import{privateRepositoryWorkspaceV1}from"../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import{requireTestToolV1}from"../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import{ownerSourceFailureV1}from"../🟨️.mjs";
const repository=await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference-projects",scratch:"checkout"});
const python=requireTestToolV1("python"),observation=await import("../../../../../👁️observation/🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Record<string,string>>)=>{for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const sha=(path:string)=>Bun.spawnSync([python,"-I","-c","import hashlib,sys;print(hashlib.sha256(open(sys.argv[1],'rb').read()).hexdigest())",path]).stdout.toString().trim();
const loadFailure=async(file:string)=>{try{await import(file+"?"+Math.random());}catch(error){return error as any;}throw Error("the import must fail");};
const files={"proj/project.json":"{}","proj/src/🟦️.ts":'import "../../peer/gone.ts";\n',"peer/other.ts":"export const x=1;\n"};

test("a missing import of an owner source is the failure of that owner: it names the importing file with its bytes and the real loader error is no stale implementation",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("source-failure-");roots.push(root);write(root,files);
 const importer=join(root,"proj/src/🟦️.ts"),raw=await loadFailure(importer);
 expect(staleImplementationOfV1(raw)?.origin,"the raw loader error alone is a half-edited module").toBe("load");
 const failure=ownerSourceFailureV1(raw,{workspaceRoot:root,fallback:join(root,"proj/project.json"),implementationRoot:join(root,"tooling")});
 expect(failure?.code).toBe("BUILD_SOURCE_UNPARSEABLE");
 expect(failure?.source).toBe(importer);
 expect(failure?.sha256).toBe(sha(importer));
 expect(failure?.message).toContain("gone.ts");expect(failure?.message).toContain("proj/src/");
 expect(staleImplementationOfV1(failure),"withheld owner never restarts the worker").toBeUndefined();
}),60_000);

test("a module of the Build tooling that does not load, a failure that names no workspace file and an ordinary error stay as they were",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("source-failure-tooling-");roots.push(root);write(root,files);
 const raw=await loadFailure(join(root,"proj/src/🟦️.ts")),scope={workspaceRoot:root,fallback:join(root,"proj/project.json")};
 expect(ownerSourceFailureV1(raw,{...scope,implementationRoot:root}),"importer inside the tooling").toBeUndefined();
 expect(ownerSourceFailureV1(raw,{...scope,workspaceRoot:join(root,"elsewhere"),implementationRoot:join(root,"tooling")}),"outside the workspace").toBeUndefined();
 const unnamed=Object.assign(Error("Cannot find module './x.ts'"),{name:"ResolveMessage"});
 expect(ownerSourceFailureV1(unnamed,{...scope,implementationRoot:join(root,"tooling")}),"no importer and no absolute module").toBeUndefined();
 expect(ownerSourceFailureV1(Error("ordinary"),scope)).toBeUndefined();
 const owned=Object.assign(Error("x"),{code:"BUILD_SOURCE_UNPARSEABLE"});expect(ownerSourceFailureV1(owned,scope)).toBe(owned);
}),60_000);

test("the missing module is an observation of the owner: the withheld owner holds until the module appears and is checked again then",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("source-failure-rows-");roots.push(root);write(root,files);
 const importer=join(root,"proj/src/🟦️.ts"),cachePath=join(root,"np","project.json"),raw=await loadFailure(importer);
 const value=await observation.observeInferenceSnapshotV1(()=>observation.observeProjectInferenceV1({cachePath,key:"project",owner:"project"},async()=>{
  const failure=ownerSourceFailureV1(raw,{workspaceRoot:root,fallback:join(root,"proj/project.json"),implementationRoot:join(root,"tooling")});
  return{source:failure?.source};
 }));
 expect(value.source).toBe(importer);
 const rows=(observation.projectInferenceObservationsV1(cachePath)??[]) as{kind:string;path:string;fact:unknown}[];
 expect(rows.some(row=>row.kind==="resolve"&&String(row.fact).startsWith("!")),"the miss is recorded").toBe(true);
 expect(await observation.observationsHold(rows)).toBe(true);
 write(root,{"peer/gone.ts":"export const y=2;\n"});
 expect(await observation.observationsHold(rows),"the module appears").toBe(false);
}),60_000);
