import {test,expect} from "bun:test";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {spawnSync} from "node:child_process";
import {dirname,join} from "node:path";
import {type BigIntStats,lstatSync,lutimesSync,mkdirSync,mkdtempSync,readFileSync,readlinkSync,renameSync,rmSync,symlinkSync,unlinkSync,utimesSync,writeFileSync} from "node:fs";
import {withBuildTestInvocationV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import * as issued from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import * as leases from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {javascriptInstallSchemaV1 as installSchema} from "../../🧱️schemas/🟦️.ts";
import {publishJavascriptDependencyPreservingSourceDescriptorV1,readJavascriptDependencyPreservationSourceV1,requireJavascriptDependencyEndpointFenceV1,requireJavascriptDependencyPreservationSourceUnchangedV1,javascriptDependencyPreservationSourceMismatchV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";

type Row={sourceDirectory:string;kind:string;stamp:string|null;target:string|null};
type Observation={rows:Row[];parents:{path:string;identity:string}[];ancestors:{path:string;identity:string}[]};

const stamp=(row:BigIntStats)=>JSON.stringify([row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].map(String));
const anchor=(row:BigIntStats)=>JSON.stringify([row.dev,row.ino,row.mode].map(String));

const layout=(root:string)=>{
 const paths={root,g:join(root,"g"),p0:join(root,"g","p0"),p1:join(root,"g","p1"),p2:join(root,"g","p2"),store:join(root,"store")};
 for(const path of[paths.g,paths.p0,paths.p1,paths.p2,paths.store,join(paths.store,"t0"),join(paths.store,"t1"),join(paths.store,"t2"),join(paths.store,"other")])mkdirSync(path);
 const endpoints=[0,1,2].map(index=>join(root,"g","p"+index,"node_modules"));for(const[index,endpoint]of endpoints.entries())symlinkSync(join(paths.store,"t"+index),endpoint);
 return{...paths,e0:endpoints[0]!,e1:endpoints[1]!,e2:endpoints[2]!,endpoints};
};

const observe=(root:string,endpoints:readonly string[]):Observation=>{
 const rows=endpoints.map(sourceDirectory=>{const facts=lstatSync(sourceDirectory,{bigint:true,throwIfNoEntry:false});return{sourceDirectory,kind:facts?facts.isSymbolicLink()?"link":facts.isDirectory()?"directory":"file":"missing",stamp:facts?stamp(facts):null,target:facts?.isSymbolicLink()?readlinkSync(sourceDirectory):null};}),parents=new Map<string,string>(),ancestors=new Map<string,string>();
 for(const endpoint of endpoints){const parent=dirname(endpoint),chain:string[]=[];for(let path=parent;path.length>=root.length;path=dirname(path)){chain.push(path);if(path===root)break;}for(const path of chain.reverse()){const facts=lstatSync(path,{bigint:true,throwIfNoEntry:false});if(!ancestors.has(path))ancestors.set(path,facts?anchor(facts):"missing");}const facts=lstatSync(parent,{bigint:true,throwIfNoEntry:false});parents.set(parent,facts?stamp(facts):"missing");}
 return{rows,parents:[...parents].map(([path,identity])=>({path,identity})),ancestors:[...ancestors].map(([path,identity])=>({path,identity}))};
};

const maps=(observation:Observation)=>({expected:new Map(observation.rows.map(row=>[row.sourceDirectory,row])),parents:new Map(observation.parents.map(row=>[row.path,row.identity])),ancestors:new Map(observation.ancestors.map(row=>[row.path,row.identity]))});

const accepts=(body:()=>void)=>{try{body();return true;}catch{return false;}};

/** 🚧️ Proves the per-binding endpoint fence refuses every change of the endpoint just published and leaves each other row to the full comparison that follows. */
test("dependency endpoint fence is local while the full comparison stays complete",()=>{
 const validate=new Ajv({strict:true}).compile(corpus.schema),artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original dependency fence artifact root required");
 for(const row of corpus.cases){
  expect(validate(row),row.id).toBe(true);const root=mkdtempSync(join(artifacts,"ef-"));
  try{
   const fixture=layout(root),before=maps(observe(root,fixture.endpoints)),future=new Date(Date.now()+5000),target=row.target==="none"?"":(fixture as unknown as Record<string,string>)[row.target]!;
   switch(row.effect){
    case"retarget":unlinkSync(target);symlinkSync(join(fixture.store,"other"),target);break;
    case"remove":unlinkSync(target);break;
    case"replace-with-directory":unlinkSync(target);mkdirSync(target);break;
    case"touch-link":lutimesSync(target,future,future);break;
    case"replace-directory":{const endpoint=join(target,"node_modules"),destination=readlinkSync(endpoint);renameSync(target,target+".moved");mkdirSync(target);symlinkSync(destination,endpoint);break;}
    case"add-sibling":writeFileSync(join(target,"sibling"),"x");break;
    case"swap-for-link":renameSync(target,target+".moved");symlinkSync(target+".moved",target);break;
    case"touch-directory":utimesSync(target,future,future);break;
    case"none":break;
   }
   const after=observe(root,fixture.endpoints);
   expect(accepts(()=>requireJavascriptDependencyEndpointFenceV1(fixture.e0,before.expected,before.parents,before.ancestors)),row.id+" endpoint fence").toBe(row.endpoint);
   expect(accepts(()=>requireJavascriptDependencyPreservationSourceUnchangedV1(after,before.expected,before.parents,before.ancestors)),row.id+" full comparison").toBe(row.full);
   expect(row.full||!row.endpoint||row.target!=="e0",row.id).toBe(true);
  }finally{rmSync(root,{recursive:true,force:true});}
 }
});

/** 🐍 Reproduces every stamp, anchor and link target of the observer with an independent Python lstat walk. */
test("dependency endpoint fence stamps agree with an independent Python lstat",()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original dependency fence artifact root required");const root=mkdtempSync(join(artifacts,"ep-"));
 try{
  const fixture=layout(root),observation=observe(root,fixture.endpoints),script="import os,sys,json\nout=[]\nfor p in json.loads(sys.argv[1]):\n s=os.lstat(p)\n out.append([json.dumps([str(getattr(s,f)) for f in ['st_dev','st_ino','st_mode','st_nlink','st_size','st_mtime_ns','st_ctime_ns']],separators=(',',':')),os.readlink(p) if os.path.islink(p) else None])\nprint(json.dumps(out))",child=spawnSync(testPythonV1()??"python3",["-c",script,JSON.stringify(fixture.endpoints)],{encoding:"utf8"});
  expect(child.status).toBe(0);const oracle=JSON.parse(child.stdout)as [string,string|null][];
  expect(observation.rows.map(row=>[row.stamp,row.target])).toEqual(oracle);
  const before=maps(observation);for(const endpoint of fixture.endpoints)requireJavascriptDependencyEndpointFenceV1(endpoint,before.expected,before.parents,before.ancestors);
 }finally{rmSync(root,{recursive:true,force:true});}
});

/** 🧮 Checks the pure full comparison against SQLite JSON equality and a first-three-fields identity oracle. */
test("dependency preservation source comparison agrees with SQLite JSON oracles",()=>{
 const validate=new Ajv({strict:true}).compile(corpus.compareSchema),artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original dependency fence artifact root required");const root=mkdtempSync(join(artifacts,"ec-")),database=new Database(":memory:");
 try{
  const fixture=layout(root),baseline=observe(root,fixture.endpoints),before=maps(baseline);
  for(const row of corpus.compare){
   expect(validate(row),row.id).toBe(true);const current:Observation=structuredClone(baseline);
   if(row.field==="rows"){if(row.edit==="reverse")current.rows.reverse();else current.rows.pop();}
   if(row.field==="parents"){if(row.edit==="add")current.parents.push({path:join(root,"extra"),identity:current.parents[0]!.identity});else{const identity=JSON.parse(current.parents[0]!.identity)as string[];identity[row.edit==="inode"?1:5]="9999999999999999999";current.parents[0]={path:current.parents[0]!.path,identity:JSON.stringify(identity)};}}
   if(row.field==="ancestors"){if(row.edit==="drop-last")current.ancestors.pop();else{const identity=JSON.parse(current.ancestors[0]!.identity)as string[];identity[2]="1";current.ancestors[0]={path:current.ancestors[0]!.path,identity:JSON.stringify(identity)};}}
   const actual=accepts(()=>requireJavascriptDependencyPreservationSourceUnchangedV1(current,before.expected,before.parents,before.ancestors));
   const oracle=row.field==="parents"&&row.edit!=="add"?Boolean((database.query("select json_extract(?1,'$[0]')=json_extract(?2,'$[0]') and json_extract(?1,'$[1]')=json_extract(?2,'$[1]') and json_extract(?1,'$[2]')=json_extract(?2,'$[2]') ok").get(current.parents[0]!.identity,baseline.parents[0]!.identity)as{ok:number}).ok):Boolean((database.query("select json(?1)=json(?2) ok").get(JSON.stringify(row.field==="rows"?current.rows:row.field==="parents"?current.parents:current.ancestors),JSON.stringify(row.field==="rows"?baseline.rows:row.field==="parents"?baseline.parents:baseline.ancestors))as{ok:number}).ok);
   expect(actual,row.id).toBe(row.accepted);expect(oracle,row.id).toBe(row.accepted);
  }
 }finally{database.close();rmSync(root,{recursive:true,force:true});}
});

/** 🔒️ Keeps the complete fence at every interval boundary and after the last binding, and removes the partial fence mode. */
test("dependency producer keeps the full fence at every interval and after the final verification",async()=>{
 const ts=await import("typescript"),source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),file=ts.createSourceFile("storage.ts",source,ts.ScriptTarget.Latest,true),producer=file.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="runJavascriptDependencyStoreProducerV1")!.getText(file);
 const interval=installSchema.$defs.InstalledPathProtocol.const.fullFenceInterval;expect(Number.isSafeInteger(interval)&&interval>=1&&interval<=64).toBe(true);
 expect(producer).toContain("if(++published%installSchema.$defs.InstalledPathProtocol.const.fullFenceInterval===0)await fence();else{checkpoint();requireJavascriptDependencyEndpointFenceV1(binding.sourceDirectory,expected,parents,ancestors);checkpoint();}");
 expect(producer).not.toContain("fence=async(full");expect(producer).not.toContain("fence(false");
 const loop=producer.indexOf("requireJavascriptDependencyEndpointFenceV1("),tail=producer.slice(loop);
 expect(tail.indexOf("await verifyReady();")).toBeGreaterThan(0);expect(tail.indexOf("await verifyReady();")).toBeLessThan(tail.indexOf("for(const scope of requested)"));expect(tail.slice(tail.indexOf("for(const scope of requested)"))).toContain("await fence();");
 const verify=producer.slice(producer.indexOf("verifyReady=async()=>{"));expect(verify.slice(0,verify.indexOf("};"))).toContain("await fence();");
 const definition=producer.slice(producer.indexOf("fence=async()=>{"),producer.indexOf("material=("));
 for(const needed of["await store.recheck()","await custody.recheck()","recheckJavascriptDependencyInputsV1(row.plan.sources,control)","requireJavascriptDependencyPreservationSourceUnchangedV1(await readJavascriptDependencyPreservationSourceV1(","verifyJavascriptDependencyDescriptorGroupV1(","recheckJavascriptDependencyPreservationsDescriptorV1(held"])expect(definition).toContain(needed);
});

/** 🛤️ Narrows publication ancestry to the lineage of the published endpoint and leaves every other ancestor to the full comparison that follows. */
test("dependency publication checks the endpoint lineage and the full comparison catches every other ancestor",async()=>{
 const validate=new Ajv({strict:true}).compile(corpus.publishSchema),artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original dependency fence artifact root required");
 await withBuildTestInvocationV1(async original=>{
  const control=issued.createIssuedResourceControlV1(original,"dependencies:publication-lineage",0),failures:string[]=[];
  for(const row of corpus.publish){
   expect(validate(row),row.id).toBe(true);const root=mkdtempSync(join(artifacts,"pl-")),workspace=join(root,"s"),cache=join(root,"n"),first=join(workspace,"a","node_modules"),second=join(workspace,"a","m","node_modules"),targets=[join(cache,"t1"),join(cache,"t2"),join(cache,"t3")];
   try{
    for(const path of[join(workspace,"a","m"),...targets])mkdirSync(path,{recursive:true});symlinkSync(targets[0]!,first,"dir");symlinkSync(targets[1]!,second,"dir");
    const check=()=>{issued.requireIssuedResourceControlInvocationV1(control,original);issued.requireIssuedResourceControlRefinementV1(control,control);leases.checkResourceLeaseControl(control);},context=Object.freeze({root:cache,invocation:original,control,check,recheck:async()=>{check();}}),ownership={version:1,scopes:[{manifest:"package.json",packages:["a","a/m"]}]};
    const source=await readJavascriptDependencyPreservationSourceV1(workspace,context,control,ownership),previous=source.rows.find(entry=>entry.sourceDirectory===first)!,facts={sourceSha256:source.sha256,generation:"a".repeat(64),readySha256:"b".repeat(64),inputSha256:"c".repeat(64),recipeSha256:"d".repeat(64),fundingSha256:"e".repeat(64)};
    let calls=0,fired=false;const mutate=()=>{switch(row.mutation){
      case"swap-lineage":renameSync(workspace,workspace+".moved");symlinkSync(workspace+".moved",workspace,"dir");break;
      case"swap-unrelated":{const folder=join(workspace,"a","m");renameSync(folder,folder+".moved");mkdirSync(folder);symlinkSync(targets[1]!,second,"dir");break;}
      case"retarget-endpoint":unlinkSync(first);symlinkSync(targets[2]!,first,"dir");break;
      case"none":break;}};
    const checkpoint=()=>{check();if(!fired&&row.mutation!=="none"&&++calls>=row.at){fired=true;mutate();}};
    let published=true;try{await publishJavascriptDependencyPreservingSourceDescriptorV1({sourceDirectory:first,targetDirectory:targets[0]!},previous,source,facts,checkpoint);}catch{published=false;}
    if(row.mutation!=="none"&&!fired&&row.publish)failures.push(row.id+": mutation never fired");
    if(row.mutation!=="none"&&!fired&&!row.publish)failures.push(row.id+": mutation never fired");
    if(published!==row.publish)failures.push(row.id+": publication expected "+(row.publish?"accepted":"refused"));
    const expected=new Map(source.rows.map(entry=>[entry.sourceDirectory,entry])),parents=new Map(source.parents.map(entry=>[entry.path,entry.identity])),ancestors=new Map(source.ancestors.map(entry=>[entry.path,entry.identity]));
    let full=true;try{requireJavascriptDependencyPreservationSourceUnchangedV1(await readJavascriptDependencyPreservationSourceV1(workspace,context,control,ownership),expected,parents,ancestors);}catch{full=false;}
    if(full!==row.full)failures.push(row.id+": full comparison expected "+(row.full?"accepted":"refused"));
   }finally{rmSync(root,{recursive:true,force:true});}
  }
  expect(failures).toEqual([]);
 });
},120000);

/** 🔎️ Reports the first exact retained Source mismatch using portable authored DATA and SQLite JSON field oracles. */
test("dependency source mismatch diagnostics preserve exact fence semantics",()=>{
 const validate=new Ajv({strict:true}).compile(installSchema.$defs.SourceFenceMismatch),database=new Database(":memory:");
 try{for(const row of corpus.diagnostic){
  const baseline={rows:[{sourceDirectory:"/p/node_modules",kind:"link",stamp:"old",target:"/old"},{sourceDirectory:"/q/node_modules",kind:"link",stamp:"old",target:"/old"}],parents:[{path:"/p",identity:JSON.stringify(["1","2","3","4","5","6","7"])}],ancestors:[{path:"/",identity:JSON.stringify(["1","2","3"])}]},current=structuredClone(baseline),before=maps(baseline);
  if(row.section==="rows"){if(row.field==="long-target")current.rows[0]!.target="x".repeat(4096);else if(row.field==="missing")current.rows.pop();else if(row.field==="reorder")current.rows.reverse();else if(row.field==="serialization")current.rows[0]=Object.fromEntries(Object.entries(current.rows[0]!).reverse())as typeof baseline.rows[number];else if(row.field!=="none")Object.assign(current.rows[0]!,{[row.field]:row.field==="stamp"?"new":"/new"});}
  if(row.section==="parents"){const value=JSON.parse(current.parents[0]!.identity);value[row.field==="mtime"?5:1]="9";current.parents[0]!.identity=JSON.stringify(value);}
  if(row.section==="ancestors")current.ancestors[0]!.identity=JSON.stringify(["1","2","9"]);
  const result=javascriptDependencyPreservationSourceMismatchV1(current,before.expected,before.parents,before.ancestors),spec=(database.query("with v as (select ?1 a,?2 b) select case when json_extract(a,'$.rows[0].sourceDirectory')<>json_extract(b,'$.rows[0].sourceDirectory') then 'rows|sourceDirectory|$.rows[0].sourceDirectory|0' when json_extract(a,'$.rows[0].stamp')<>json_extract(b,'$.rows[0].stamp') then 'rows|stamp|$.rows[0].stamp|0' when json_extract(a,'$.rows[0].target')<>json_extract(b,'$.rows[0].target') then 'rows|target|$.rows[0].target|0' when json_array_length(a,'$.rows')<>json_array_length(b,'$.rows') then 'rows|entry|$.rows[1]|1' when json_extract(a,'$.rows[0]')<>json_extract(b,'$.rows[0]') then 'rows|serialization|$.rows[0]|0' when json_extract(json_extract(a,'$.parents[0].identity'),'$[1]')<>json_extract(json_extract(b,'$.parents[0].identity'),'$[1]') then 'parents|identity.ino|$[1]|0' when json_extract(json_extract(a,'$.ancestors[0].identity'),'$[2]')<>json_extract(json_extract(b,'$.ancestors[0].identity'),'$[2]') then 'ancestors|identity.mode|$[2]|0' end spec from v").get(JSON.stringify(baseline),JSON.stringify(current))as{spec:string|null}).spec;
  const oracle=spec===null?null:(()=>{const[section,field,jsonPath,indexText]=spec.split("|"),index=Number(indexText),indexed=(value:Observation)=>field!.startsWith("identity.")?value[section as "parents"|"ancestors"][0]!.identity:JSON.stringify(value),read=(value:Observation)=>(database.query("select case when typeof(v)='text' then substr(v,1,2048) else v end value from (select json_extract(?1,?2) v)").get(indexed(value),jsonPath!)as{value:unknown}).value;return{section,index,path:section==="rows"?index===0?"/p/node_modules":"/q/node_modules":section==="parents"?"/p":"/",field,expected:read(baseline),actual:read(current)};})();
  expect(oracle,row.id).toEqual(row.expected);expect(result,row.id).toEqual(row.expected);if(result)expect(validate(result),row.id).toBe(true);
  expect(accepts(()=>requireJavascriptDependencyPreservationSourceUnchangedV1(current,before.expected,before.parents,before.ancestors)),row.id).toBe(result===null);let message:string|null=null;try{requireJavascriptDependencyPreservationSourceUnchangedV1(current,before.expected,before.parents,before.ancestors);}catch(error){message=(error as Error).message;}expect(message,row.id).toBe(result===null?null:"Exact current Store Source fence changed: "+JSON.stringify(result));
 }
 for(const row of corpus.diagnosticInvalid)expect(validate({...corpus.diagnostic[1]!.expected,[row.field]:row.value}),row.id).toBe(false);
 const path="/"+"x".repeat(256),retained={rows:[{sourceDirectory:path,kind:"link",stamp:"old",target:"/old"}],parents:[],ancestors:[]},current=structuredClone(retained),before=maps(retained);current.rows[0]!.target="/new";let message:string|null=null;try{requireJavascriptDependencyPreservationSourceUnchangedV1(current,before.expected,before.parents,before.ancestors);}catch(error){message=(error as Error).message;}expect(message).toBe("Exact current Store Source fence changed");
 }finally{database.close();}
});

test("dependency Source fence mismatch gives bounded neutral DATA and preserves rejection",()=>{
 const schema=installSchema,run=requireJavascriptDependencyPreservationSourceUnchangedV1,valid=new Ajv({strict:true}).compile(schema.$defs.SourceFenceDiagnostic),db=new Database(":memory:"),stamp=JSON.stringify(["1","2","16832","1","0","10","11"]);
 try{for(const row of corpus.neutralDiagnostic){
  const original={rows:[{sourceDirectory:"/workspace/p/node_modules",kind:"link",stamp,target:"/store/current/node_modules"}],parents:[{path:"/workspace/p",identity:stamp}],ancestors:[{path:"/workspace",identity:JSON.stringify(["1","3","16832"])}]},current=structuredClone(original),expected=new Map(original.rows.map(r=>[r.sourceDirectory,r])),parents=new Map(original.parents.map(r=>[r.path,r.identity])),ancestors=new Map(original.ancestors.map(r=>[r.path,r.identity]));
  if(row.mode==="endpoint-time"){const s=JSON.parse(current.rows[0]!.stamp);s[5]="99";current.rows[0]!.stamp=JSON.stringify(s);}
  if(row.mode==="endpoint-target")current.rows[0]!.target="/store/peer/node_modules";
  if(row.mode==="parent-device"){const s=JSON.parse(current.parents[0]!.identity);s[0]="9";current.parents[0]!.identity=JSON.stringify(s);}
  if(row.mode==="ancestor-inode"){const s=JSON.parse(current.ancestors[0]!.identity);s[1]="9";current.ancestors[0]!.identity=JSON.stringify(s);}
  if(row.mode==="endpoint-count")current.rows=[];
  if(row.mode==="endpoint-order")current.rows[0]!.sourceDirectory="/workspace/peer/node_modules";
  if(row.mode==="bounded-path")current.rows[0]!.sourceDirectory="/workspace/"+"p".repeat(300);
  if(row.mode==="all-fields"){current.rows[0]={sourceDirectory:"/workspace/peer/node_modules",kind:"file",target:"/store/peer/node_modules",stamp:JSON.stringify(["9","9","9","9","9","99","99"])};}
  let error:any;try{run(current,expected,parents,ancestors);}catch(e){error=e;}
  expect(error?.message,row.id).toStartWith("Exact current Store Source fence changed");
  expect(error?.cause,row.id).toEqual({version:1,purpose:"current-store-source-fence-mismatch",section:row.section,index:0,path:row.path,fields:row.fields});
  expect(valid(error.cause),row.id).toBe(true);
  expect(valid({...error.cause,authority:true}),row.id).toBe(false);
  expect(JSON.stringify(error.cause)).not.toContain("/store/");
  if(row.mode==="endpoint-time"){const changed=db.query("SELECT group_concat(CASE a.key WHEN 0 THEN 'dev' WHEN 1 THEN 'ino' WHEN 2 THEN 'mode' WHEN 3 THEN 'nlink' WHEN 4 THEN 'size' WHEN 5 THEN 'mtimeNs' WHEN 6 THEN 'ctimeNs' END) fields FROM json_each(?1) a JOIN json_each(?2) b ON a.key=b.key WHERE a.value<>b.value").get(original.rows[0]!.stamp,current.rows[0]!.stamp) as {fields:string};expect(changed.fields.split(",")).toEqual(error.cause.fields);}
  expect(()=>run(original,expected,parents,ancestors)).not.toThrow();
  const parentTime=structuredClone(original),s=JSON.parse(parentTime.parents[0]!.identity);s[5]="99";parentTime.parents[0]!.identity=JSON.stringify(s);expect(()=>run(parentTime,expected,parents,ancestors)).not.toThrow();
 }}finally{db.close();}
});
