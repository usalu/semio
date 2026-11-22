import{test,expect}from"bun:test";
import{mkdtempSync,mkdirSync,writeFileSync,realpathSync,rmSync,lstatSync,opendirSync,readdirSync,utimesSync}from"node:fs";
import{tmpdir}from"node:os";
import{join,basename}from"node:path";
import{createHash}from"node:crypto";
import{JavascriptDependencyRowMismatchV1,javascriptDependencyUnreadyRetentionIntentV1}from"../../../../../../../../../🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts";
import{readJavascriptDependencyRetainedGenerationDataV1}from"../../../../../../../../../🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/📄️generation/🟦️.ts";
import{javascriptInstallSchemaV1 as dependencyInstallSchema}from"../../../../../../../../../🚀️bootstrap/📦️dependencies/💾️storage/🧱️schemas/🟦️.ts";
import{dependencyNameKindV1,settleDependencyForeignNamesV1}from"../../../../🧹️foreign-names/🟦️.ts";
import retirementSchema from"../../🧬️schema/🔣️.json";
import texts from"../../../../../../../../../📖️texts/🔣️.json";

const sha=(char:string)=>char.repeat(64),dependencySelectedPlanProtocol=Object.freeze({...dependencyInstallSchema.$defs.SelectedPlanProtocol.const});
const transpiler=new Bun.Transpiler({loader:"ts",target:"bun"});
function sliceFunction(source:string,name:string):string{
 const match=new RegExp("^(?:export )?(?:async )?function "+name+"\\b","mu").exec(source);expect(match).not.toBeNull();
 const start=match!.index,lineEnd=source.indexOf("\n",start),header=source.slice(start,lineEnd),end=header.trimEnd().endsWith("{")?source.indexOf("\n}",start)+2:lineEnd;
 return source.slice(start,end).replace(/^export\s/u,"");
}
async function load(url:URL,names:string[],ports:Record<string,unknown>):Promise<Record<string,(...args:any[])=>any>>{
 const source=await Bun.file(url).text(),code=transpiler.transformSync(names.map(name=>sliceFunction(source,name)).join("\n"));
 return new Function(...Object.keys(ports),code+";return{"+names.join(",")+"};")(...Object.values(ports));
}

function receipt(workspace:string,root:string,generation:string,odd:boolean){
 const row:Record<string,unknown>={version:4,purpose:"current-dependency-source-owner-ready",scope:".",generation,ownerRelationSha256:sha("1"),groupSha256:sha("2"),recipeSha256:sha("3"),members:[],bindings:[{sourceDirectory:workspace+"/node_modules",targetDirectory:join(root,"j",generation.slice(0,20),"node_modules")}],ownerReferences:[],singletons:[]};
 if(odd){delete row.singletons;row.platform="linux";}
 return JSON.stringify(row);
}

test("a retained generation whose receipt no longer has the current field set is retired and named, while the plan and the current generation are untouched",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"stale-gen-"))),workspace="/ws",current=sha("a"),odd=sha("b"),said:Array<{key:string;values:Record<string,unknown>;level?:string}>=[];
 try{
  mkdirSync(join(root,"j"));writeFileSync(join(root,"dependencies.json"),"{}");writeFileSync(join(root,"storage-events.jsonl"),"");
  for(const[generation,isOdd]of[[current,false],[odd,true]]as const){const directory=join(root,"j",generation.slice(0,20));mkdirSync(directory);writeFileSync(join(directory,"bindings.json"),receipt(workspace,root,generation,isOdd));}
  const control={publish:async()=>{},yieldContinuation:async()=>{}},context={root,check:()=>{},recheck:async()=>{}};
  const names=["describeRepositoryDependencyCandidateV1","repositoryDependencySelectedNamesV1","requireRepositoryDependencyImageRolesV1","captureRepositoryDependencySelectedPlanV1"];
  const ports={dependencyForeignPolicy:dependencySelectedPlanProtocol.foreignNames,dependencyNameKindV1,settleDependencyForeignNamesV1,lstatSync,opendirSync,join,basename,createHash,JavascriptDependencyRowMismatchV1,readJavascriptDependencyRetainedGenerationDataV1,javascriptDependencyUnreadyRetentionIntentV1,dependencySelectedPlanProtocol,dependencyInstallSchema,retainNativeStorageFailureV1:(list:unknown[],error:unknown)=>{list.push(error);},repositorySelectedToolingStampV1:(row:{dev:bigint;ino:bigint;mtimeNs:bigint})=>[row.dev,row.ino,row.mtimeNs].join(":"),observeSelectedToolingPlanServiceV1:async()=>{},readJavascriptDependencyUnreadyMetadataV1:()=>{throw Error("no capsule here");},readJavascriptDependencyBindingReceiptReadV1:async(_context:unknown,path:string)=>{const text=await Bun.file(path).text();return{value:JSON.parse(text),fingerprint:"fp:"+createHash("sha256").update(text).digest("hex")};},sayCachingV1:(key:string,values:Record<string,unknown>,level?:string)=>{said.push({key,values,level});}};
  const api=await load(new URL("../../../../🟦️.ts",import.meta.url),names,ports);
  const plan=await api.captureRepositoryDependencySelectedPlanV1(workspace,context,control,{},{scopes:[{generation:current}]},null);
  const rows=Object.fromEntries(plan.candidates.map((row:{prefix:string})=>[row.prefix,row]));
  expect(plan.candidates.length).toBe(2);
  expect(rows[current.slice(0,20)].status).toBe("active");
  expect(rows[odd.slice(0,20)]).toMatchObject({status:"unready",generation:null,receiptFingerprint:null,roles:["bindings.json"]});
  expect(said.length).toBe(1);
  expect(said[0]!.key).toBe("dependency-store.stale-generation");
  expect(said[0]!.values).toMatchObject({prefix:odd.slice(0,20),file:"bindings.json",missing:'["singletons"]',unexpected:'["platform"]'});
  expect(said[0]!.level).toBe("warn");
  for(const language of["en","de"])expect((texts as any).messages["dependency-store.stale-generation"][language]).toContain("{prefix}");
  const driver=await load(new URL("../../🟦️.ts",import.meta.url),["selectRepositoryDependencyRetirementCandidateV1"],{retirementProtocol:retirementSchema.$defs.Protocol.const});
  const chosen=driver.selectRepositoryDependencyRetirementCandidateV1({current:{sha256:sha("c"),scopes:[{generation:current}]},generations:{names:plan.generations?.names??Object.keys(rows)},candidates:plan.candidates});
  expect(chosen.prefix).toBe(odd.slice(0,20));
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the same unreadable receipt of the generation currently selected stays a hard refusal naming prefix, file and fields",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"stale-gen-"))),workspace="/ws",current=sha("a");
 try{
  mkdirSync(join(root,"j"));writeFileSync(join(root,"dependencies.json"),"{}");writeFileSync(join(root,"storage-events.jsonl"),"");
  const directory=join(root,"j",current.slice(0,20));mkdirSync(directory);writeFileSync(join(directory,"bindings.json"),receipt(workspace,root,current,true));
  const control={publish:async()=>{},yieldContinuation:async()=>{}},context={root,check:()=>{},recheck:async()=>{}};
  const names=["describeRepositoryDependencyCandidateV1","repositoryDependencySelectedNamesV1","requireRepositoryDependencyImageRolesV1","captureRepositoryDependencySelectedPlanV1"];
  const ports={dependencyForeignPolicy:dependencySelectedPlanProtocol.foreignNames,dependencyNameKindV1,settleDependencyForeignNamesV1,lstatSync,opendirSync,join,basename,createHash,JavascriptDependencyRowMismatchV1,readJavascriptDependencyRetainedGenerationDataV1,javascriptDependencyUnreadyRetentionIntentV1,dependencySelectedPlanProtocol,dependencyInstallSchema,retainNativeStorageFailureV1:(list:unknown[],error:unknown)=>{list.push(error);},repositorySelectedToolingStampV1:(row:{dev:bigint;ino:bigint;mtimeNs:bigint})=>[row.dev,row.ino,row.mtimeNs].join(":"),observeSelectedToolingPlanServiceV1:async()=>{},readJavascriptDependencyUnreadyMetadataV1:()=>{throw Error("no capsule here");},readJavascriptDependencyBindingReceiptReadV1:async(_context:unknown,path:string)=>({value:JSON.parse(await Bun.file(path).text()),fingerprint:"fp"}),sayCachingV1:()=>{}};
  const api=await load(new URL("../../../../🟦️.ts",import.meta.url),names,ports);
  let message="";try{await api.captureRepositoryDependencySelectedPlanV1(workspace,context,control,{},{scopes:[{generation:current}]},null);}catch(error){message=String((error as Error).message);}
  expect(message.startsWith("Retained dependency generation "+current.slice(0,20)+" file bindings.json: Exact selected dependency fields required")).toBe(true);
  expect(message).toContain('missing ["singletons"]');
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a retirement tombstone and OS metadata never refuse the roster, an old foreign name is quarantined by itself and a young one is named in the refusal",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"stale-gen-"))),workspace="/ws",current=sha("a"),said:Array<{key:string;values:Record<string,unknown>;level?:string}>=[];
 try{
  mkdirSync(join(root,"j"));writeFileSync(join(root,"dependencies.json"),"{}");writeFileSync(join(root,"storage-events.jsonl"),"");
  const directory=join(root,"j",current.slice(0,20));mkdirSync(directory);writeFileSync(join(directory,"bindings.json"),receipt(workspace,root,current,false));
  mkdirSync(join(root,"j",".x"+sha("c").slice(0,20)));writeFileSync(join(root,"j",".DS_Store"),"x");writeFileSync(join(root,"j","._"+current.slice(0,20)),"x");
  writeFileSync(join(root,"j","stray-old"),"old");const old=(Date.now()-3_600_000)/1000;utimesSync(join(root,"j","stray-old"),old,old);
  const control={publish:async()=>{},yieldContinuation:async()=>{}},context={root,check:()=>{},recheck:async()=>{}};
  const names=["describeRepositoryDependencyCandidateV1","repositoryDependencySelectedNamesV1","requireRepositoryDependencyImageRolesV1","captureRepositoryDependencySelectedPlanV1"];
  const ports={dependencyForeignPolicy:dependencySelectedPlanProtocol.foreignNames,dependencyNameKindV1,settleDependencyForeignNamesV1,lstatSync,opendirSync,join,basename,createHash,JavascriptDependencyRowMismatchV1,readJavascriptDependencyRetainedGenerationDataV1,javascriptDependencyUnreadyRetentionIntentV1,dependencySelectedPlanProtocol,dependencyInstallSchema,retainNativeStorageFailureV1:(list:unknown[],error:unknown)=>{list.push(error);},repositorySelectedToolingStampV1:(row:{dev:bigint;ino:bigint;mtimeNs:bigint})=>[row.dev,row.ino,row.mtimeNs].join(":"),observeSelectedToolingPlanServiceV1:async()=>{},readJavascriptDependencyUnreadyMetadataV1:()=>{throw Error("no capsule here");},readJavascriptDependencyBindingReceiptReadV1:async(_context:unknown,path:string)=>{const text=await Bun.file(path).text();return{value:JSON.parse(text),fingerprint:"fp:"+createHash("sha256").update(text).digest("hex")};},sayCachingV1:(key:string,values:Record<string,unknown>,level?:string)=>{said.push({key,values,level});}};
  const api=await load(new URL("../../../../🟦️.ts",import.meta.url),names,ports);
  const plan=await api.captureRepositoryDependencySelectedPlanV1(workspace,context,control,{},{scopes:[{generation:current}]},null);
  expect(plan.candidates.map((row:{prefix:string})=>row.prefix)).toEqual([current.slice(0,20)]);
  expect(plan.generations.names).toEqual([current.slice(0,20)]);
  expect(readdirSync(join(root,"j")).sort()).toEqual([".q",".x"+sha("c").slice(0,20),current.slice(0,20)].sort());
  expect(readdirSync(join(root,"j",".q")).length).toBe(1);
  writeFileSync(join(root,"j","stray-young"),"young");
  let message="";try{await api.captureRepositoryDependencySelectedPlanV1(workspace,context,control,{},{scopes:[{generation:current}]},null);}catch(error){message=String((error as Error).message);}
  expect(message).toContain("stray-young");expect(message).toContain("roster required");
 }finally{rmSync(root,{recursive:true,force:true});}
});
