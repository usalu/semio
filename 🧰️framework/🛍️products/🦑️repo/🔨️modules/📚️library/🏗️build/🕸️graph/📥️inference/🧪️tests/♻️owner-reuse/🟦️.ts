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
const write=(root:string,files:Readonly<Record<string,string|undefined>>)=>{for(const[path,text]of Object.entries(files)){if(text===undefined)continue;mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const esbuildClosure=async(root:string,entry:string)=>{const result=await createRequire(import.meta.url)("esbuild").build({entryPoints:[join(root,entry)],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent",outfile:"out.js"});return Object.keys(result.metafile.inputs).map((path:string)=>path.replace(/^\.\//u,"")).sort();};
const build=(root:string,killAfter?:number)=>{
 const run=Bun.spawnSync([process.execPath,"test",fileURLToPath(new URL("./🏭️build.ts",import.meta.url)),"--timeout","1800000"],{cwd:fileURLToPath(new URL(".",import.meta.url)),env:{...process.env,SEMIO_OWNER_REUSE_ROOT:root,...killAfter===undefined?{}:{SEMIO_OWNER_REUSE_KILL_AFTER:String(killAfter)}},stderr:"pipe",stdout:"pipe"}),text=run.stdout.toString(),line=text.split("\n").find(row=>row.startsWith("owner-reuse "));
 if(killAfter!==undefined){
  expect(run.signalCode,run.stderr.toString()+text).toBe("SIGKILL");
  const settled=text.split("\n").filter(row=>row.startsWith("owner-reuse-settled ")).map(row=>row.split(" "));
  return{inferred:settled.filter(row=>row[2]==="inferred").map(row=>row[1]!),closures:{derived:0,loaded:0,stale:0},rosters:{}} as{inferred:string[];closures:{derived:number;loaded:number;stale:number};rosters:Record<string,string[]>};
 }
 expect(line,run.stderr.toString()+text).toBeDefined();
 return JSON.parse(line!.slice("owner-reuse ".length)) as{inferred:string[];closures:{derived:number;loaded:number;stale:number};rosters:Record<string,string[]>};
};

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});

test("owners are reloaded from their stored records by a process that starts later, and an edit re-infers only the owners that depend on it",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("owner-reuse-");roots.push(root);write(root,fixture.files);
 for(const step of fixture.steps){
  write(root,step.edit);await Bun.sleep(OBSERVATION_LIMITS_V1.racyMilliseconds+250);
  const result=build(root,(step as{killAfter?:number}).killAfter);
  expect(result.inferred,step.name).toEqual(step.inferred);
  if((step as{killAfter?:number}).killAfter!==undefined)continue;
  if(step.derivedClosures===0){expect(result.closures.derived,step.name).toBe(0);expect(result.closures.stale,step.name).toBe(0);}
  for(const name of fixture.owners)expect(result.rosters[name],step.name+" "+name).toEqual(await esbuildClosure(root,"owners/"+name+"/entry.mjs"));
 }
},{maximumElapsedMilliseconds:900000}),1800000);
