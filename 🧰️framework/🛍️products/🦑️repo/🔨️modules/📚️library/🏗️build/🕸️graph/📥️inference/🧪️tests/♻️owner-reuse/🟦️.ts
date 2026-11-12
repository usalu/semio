/** ♻️ Contract: a stored owner record is reloaded by a process that starts later whenever its inputs hold. A world of owners that share one command closure is built in a fresh process several times: with no edit every owner is reloaded and no closure entry is derived again; an edit re-infers exactly the owners that read the edited fact or whose answer to the closure question moves. The steps are language-agnostic data in `./🔣️.json`; the closure rosters are those esbuild finds. */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,rmSync,writeFileSync}from"node:fs";
import{createRequire}from"node:module";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import Ajv from"ajv";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";
import{OBSERVATION_LIMITS_V1}from"../../../../👁️observation/🟨️.mjs";
import{privateRepositoryWorkspaceV1}from"../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference",scratch:"checkout"});
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Record<string,string>>)=>{for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const esbuildClosure=async(root:string,entry:string)=>{const result=await createRequire(import.meta.url)("esbuild").build({entryPoints:[join(root,entry)],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent",outfile:"out.js"});return Object.keys(result.metafile.inputs).map((path:string)=>path.replace(/^\.\//u,"")).sort();};
const build=(root:string)=>{
 const run=Bun.spawnSync([process.execPath,"test",fileURLToPath(new URL("./🏭️build.ts",import.meta.url)),"--timeout","1800000"],{env:{...process.env,SEMIO_OWNER_REUSE_ROOT:root},stderr:"pipe",stdout:"pipe"}),text=run.stdout.toString(),line=text.split("\n").find(row=>row.startsWith("owner-reuse "));
 expect(line,run.stderr.toString()+text).toBeDefined();
 return JSON.parse(line!.slice("owner-reuse ".length)) as{inferred:string[];closures:{derived:number;loaded:number;stale:number};rosters:Record<string,string[]>};
};

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});

test("owners are reloaded from their stored records by a process that starts later, and an edit re-infers only the owners that depend on it",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("owner-reuse-");roots.push(root);write(root,fixture.files);
 for(const step of fixture.steps){
  write(root,step.edit);await Bun.sleep(OBSERVATION_LIMITS_V1.racyMilliseconds+250);
  const result=build(root);
  expect(result.inferred,step.name).toEqual(step.inferred);
  if(step.derivedClosures===0){expect(result.closures.derived,step.name).toBe(0);expect(result.closures.stale,step.name).toBe(0);}
  for(const name of fixture.owners)expect(result.rosters[name],step.name+" "+name).toEqual(await esbuildClosure(root,"owners/"+name+"/entry.mjs"));
 }
},{maximumElapsedMilliseconds:900000}),1800000);
