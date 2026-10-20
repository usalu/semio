import{test,expect}from"bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import*as owner from"../🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🔣️.json"with{type:"json"};
const tick=()=>new Promise<void>(done=>setImmediate(done));

test("eight metadata admissions drain before deterministic ordered reduction or original failure",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);const map=Reflect.get(owner,"forEachMetadataBatchV1");expect(typeof map).toBe("function");
 for(const row of fixture.cases){const db=new Database(":memory:"),marker=Error(row.name),cancel=Error("actual cancellation marker"),accepted:number[]=[],reduced:number[]=[],gates=new Map<number,()=>void>(),events:{kind:string;index:number;active:number}[]=[];let active=0,maximum=0,settled=false,failed=false,caught:unknown;
  try{const pending=map(Array.from({length:row.count},(_,i)=>i),async(index:number)=>{accepted.push(index);active++;maximum=Math.max(maximum,active);events.push({kind:"accepted",index,active});await new Promise<void>(done=>gates.set(index,done));try{const failure=row.failures.find(value=>value.index===index);if(failure)throw failure.reason==="undefined"?undefined:failure.reason==="cancel"?cancel:marker;return index*2;}finally{active--;events.push({kind:"settled",index,active});}},(index:number,value:number)=>{expect(value).toBe(index*2);reduced.push(index);events.push({kind:"reduced",index,active});}).then(()=>{settled=true;},(error:unknown)=>{failed=true;caught=error;settled=true;});
   while(!settled){await tick();const batch=[...gates.keys()].sort((a,b)=>b-a);if(!batch.length)continue;expect(active).toBeLessThanOrEqual(schema.definitions.Protocol.const.maximumAcceptedMetadata);for(const index of batch){gates.get(index)!();gates.delete(index);await tick();if(index===batch[0]&&batch.length>1)expect(settled).toBe(false);}}
   await pending;expect(active).toBe(0);expect(maximum).toBeLessThanOrEqual(8);expect(accepted).toHaveLength(row.expectedAccepted);expect(failed).toBe(row.expectedFailure!=="none");if(failed)expect(caught).toBe(row.expectedFailure==="undefined"?undefined:row.expectedFailure==="cancel"?cancel:marker);else expect(reduced).toEqual(accepted);db.exec("create table events(sequence integer,kind text,idx integer,active integer)");events.forEach((event,index)=>db.query("insert into events values(?,?,?,?)").run(index,event.kind,event.index,event.active));expect(db.query("select max(active)<=8 ok from events").get()).toEqual({ok:events.length?1:null});expect(db.query("select count(*) n from events where kind='reduced' and active!=0").get()).toEqual({n:0});expect(reduced).toEqual([...reduced].sort((a,b)=>a-b));
  }finally{for(const release of gates.values())release();db.close();}
 }
});

test("the 64-operation continuation is serialized and a failed barrier poisons late admission",async()=>{
 const Barrier=Reflect.get(owner,"MetadataContinuationBarrierV1");expect(typeof Barrier).toBe("function");const barrier=new Barrier();let entered=0,settled=0,active=0,maximum=0;const releases:Array<()=>void>=[],work:Array<Promise<unknown>>=[];for(let i=0;i<136;i++)work.push(Promise.resolve(barrier.checkpoint(async()=>{entered++;active++;maximum=Math.max(maximum,active);await new Promise<void>(done=>releases.push(done));active--;})).then(()=>{settled++;}));await tick();expect(entered).toBe(1);expect(settled).toBe(63);releases[0]!();await tick();expect(entered).toBe(2);expect(settled).toBe(127);releases[1]!();await Promise.all(work);expect(settled).toBe(136);expect(entered).toBe(2);expect(maximum).toBe(1);
 const broken=new Barrier();for(let i=0;i<63;i++)await broken.checkpoint(async()=>{});let rejected=false;try{await broken.checkpoint(async()=>{throw undefined;});}catch(error){rejected=true;expect(error).toBeUndefined();}expect(rejected).toBe(true);let calls=0;try{await broken.checkpoint(async()=>{calls++;});throw Error("Failed continuation admitted late work");}catch(error){expect(error).toBeUndefined();}expect(calls).toBe(0);
});

import{mkdtemp,mkdir,writeFile,utimes,symlink,rm}from"node:fs/promises";
import{dirname,join,isAbsolute}from"node:path";
import{scanCargoBuildUnits,scanCargoIncrementalUnits}from"../../🟦️.ts";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{captureOwnedCommandV1}from"@semio-tech/process/execution";
import{checkResourceLeaseControl}from"@semio-tech/process/leases";
import type{CacheScanTimingV1}from"../../⏱️measurement/🟦️.ts";

test("actual metadata batches preserve full opposite-kind coverage, ties, symlinks and missing roots",async()=>{
 await receiveScriptProcessInvocation(process.env,async original=>{
  const workspace=(original.capabilities as{workspaceRoot?:unknown}).workspaceRoot;if(typeof workspace!=="string"||!isAbsolute(workspace))throw Error("Actual finite metadata fixture root required");const control=createIssuedResourceControlV1(original,"test:metadata-batches",120000),artifacts=join(workspace,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated"),python=testPythonV1()??Bun.which("python");if(!python)throw Error("Independent metadata oracle required");await mkdir(artifacts,{recursive:true});const root=await mkdtemp(join(artifacts,"mb-")),external=await mkdtemp(join(artifacts,"mx-")),log=console.log,summaries:CacheScanTimingV1[]=[];
  try{for(const file of fixture.files){const path=join(root,file.path);await mkdir(dirname(path),{recursive:true});await writeFile(path,"x".repeat(file.bytes),{flag:"wx"});await utimes(path,file.time,file.time);}await mkdir(join(external,"build/foreign/hash"),{recursive:true});await writeFile(join(external,"build/foreign/hash/hidden"),"not admitted");await symlink(external,join(root,"p/build/foreign"),"dir");await symlink(join(external,"absent"),join(root,"p/build/a/h/missing"));
   console.log=(...values:unknown[])=>{const text=String(values[0]),prefix="[DEBUG] Cargo scanner timing ";if(text.startsWith(prefix))summaries.push(JSON.parse(text.slice(prefix.length)));else log(...values);};const actual={build:await scanCargoBuildUnits(root,control,undefined,true,"initial"),...await scanCargoIncrementalUnits(root,control,undefined,true,"initial")};expect(summaries).toHaveLength(2);expect(summaries.every(row=>row.maximumInFlight<=8)).toBe(true);expect(summaries.some(row=>row.maximumInFlight>1)).toBe(true);
   const oracle=String.raw`import os,stat,json,sys
root=sys.argv[1]
def named(kind):
 out=[]
 for parent,dirs,files in os.walk(root,followlinks=False):
  depth=len(os.path.relpath(parent,root).split(os.sep)) if parent!=root else 0
  if depth>8:dirs[:]=[];continue
  for name in list(dirs):
   p=os.path.join(parent,name)
   if os.path.islink(p):dirs.remove(name);continue
   if name==kind:out.append(p);dirs.remove(name)
 return out
def measured(directory):
 files=[]
 for parent,dirs,names in os.walk(directory,followlinks=False):
  for name in names:
   p=os.path.join(parent,name);s=os.lstat(p)
   if stat.S_ISREG(s.st_mode):files.append((p,s))
 return summarize(directory,files)
def summarize(directory,files):
 size=0;maximum=0;sources=[]
 for p,s in files:
  size+=s.st_size
  for field,value in [('atimeNs',s.st_atime_ns),('mtimeNs',s.st_mtime_ns)]:
   if value>maximum:maximum=value;sources=[]
   if value==maximum:sources.append(dict(path=os.path.relpath(p,directory).replace(os.sep,'/'),field=field,device=str(s.st_dev),inode=str(s.st_ino),bytes=str(s.st_size),atimeNs=str(s.st_atime_ns),mtimeNs=str(s.st_mtime_ns),ctimeNs=str(s.st_ctime_ns)))
 result=dict(bytes=size,recencyMs=(maximum//1000000000)*1000+(maximum%1000000000)/1000000)
 if sources:result['recencySource']=min(sources,key=lambda x:(x['path'],x['field']))
 return result
def unit(p,facts,kind,held=False):return dict(path=os.path.relpath(p,root).replace(os.sep,'/'),lockHeld=held,kind=kind,**facts)
build=[];units=[];stale=[]
for parent in named('build'):
 for package in os.listdir(parent):
  p=os.path.join(parent,package)
  if not os.path.isdir(p) or os.path.islink(p):continue
  for name in os.listdir(p):
   q=os.path.join(p,name)
   if os.path.isdir(q) and not os.path.islink(q):build.append(unit(q,measured(q),'cargo-build'))
for parent in named('incremental'):
 for crate in os.listdir(parent):
  p=os.path.join(parent,crate)
  if not os.path.isdir(p) or os.path.islink(p):continue
  sessions=[]
  for name in os.listdir(p):
   q=os.path.join(p,name)
   if not name.startswith('s-') or name.endswith('.lock') or not os.path.isdir(q) or os.path.islink(q):continue
   facts=measured(q);lock=os.path.join(p,'-'.join(name.split('-')[:3])+'.lock')
   if os.path.isfile(lock) and not os.path.islink(lock):
    extra=summarize(q,[(lock,os.lstat(lock))]);facts['bytes']+=extra['bytes']
    if extra['recencyMs']>facts['recencyMs']:facts['recencyMs']=extra['recencyMs'];facts['recencySource']=extra['recencySource']
    elif extra['recencyMs']==facts['recencyMs'] and extra.get('recencySource'):facts['recencySource']=min([facts['recencySource'],extra['recencySource']],key=lambda x:(x['path'],x['field']))
   sessions.append((name,facts))
  working=[r for r in sessions if r[0].endswith('-working')];final=sorted([r for r in sessions if not r[0].endswith('-working')],key=lambda x:-x[1]['recencyMs'])
  for name,facts in final[1:]:stale.append(unit(os.path.join(p,name),facts,'cargo-incremental-session'))
  retained=final[:1]+working;facts=dict(bytes=sum(r[1]['bytes'] for r in retained),recencyMs=max([0]+[r[1]['recencyMs'] for r in retained]));sources=[]
  for name,s in retained:
   if s['recencyMs']==facts['recencyMs'] and s.get('recencySource'):sources.append(dict(s['recencySource'],path=os.path.normpath(os.path.join(name,s['recencySource']['path'])).replace(os.sep,'/')))
  if sources:facts['recencySource']=min(sources,key=lambda x:(x['path'],x['field']))
  units.append(unit(p,facts,'cargo-incremental',bool(working)))
print(json.dumps(dict(build=build,units=units,staleSessions=stale)))`;
   const expected=JSON.parse(await captureOwnedCommandV1(python,["-c",oracle,root],root,{control,env:process.env,output:{maximumBytes:32768,maximumLines:64}}));for(const key of["build","units","staleSessions"]as const){const sort=<T extends{path:string}>(rows:readonly T[])=>[...rows].sort((a,b)=>a.path.localeCompare(b.path));expect(sort(actual[key])).toEqual(sort(expected[key]));}const comparison=new Database(":memory:");try{comparison.exec("create table results(side text,kind text,path text,bytes integer,recency real,held integer,source text)");for(const side of["actual","expected"]as const)for(const kind of["build","units","staleSessions"]as const)for(const row of(side==="actual"?actual:expected)[kind])comparison.query("insert into results values(?,?,?,?,?,?,?)").run(side,kind,row.path,row.bytes,row.recencyMs,Number(row.lockHeld),JSON.stringify(row.recencySource??null));const columns="kind,path,bytes,recency,held,"+["path","field","device","inode","bytes","atimeNs","mtimeNs","ctimeNs"].map(key=>"json_extract(source,'$."+key+"')").join(",");expect(comparison.query("select count(*) n from (select "+columns+" from results where side='actual' except select "+columns+" from results where side='expected')").get()).toEqual({n:0});expect(comparison.query("select count(*) n from (select "+columns+" from results where side='expected' except select "+columns+" from results where side='actual')").get()).toEqual({n:0});}finally{comparison.close();}expect(actual.build.find(row=>row.path==="p/build/a/h")?.recencySource?.path).toBe("a");expect(actual.units.find(row=>row.path==="p/incremental/top")?.lockHeld).toBe(true);expect(actual.units.some(row=>row.path.includes("sub/incremental/inner"))).toBe(true);expect(actual.build.some(row=>row.path.includes("other/build/nested/h"))).toBe(true);expect(await scanCargoBuildUnits(join(root,"absent"),control)).toEqual([]);expect(await scanCargoIncrementalUnits(join(root,"absent"),control)).toEqual({units:[],staleSessions:[]});
   const abort=new AbortController(),cancel=Error("actual accepted metadata cancellation"),cancelled=createIssuedResourceControlV1(original,"test:accepted-metadata-drain",120000,abort.signal),releases=new Map<number,()=>void>(),accepted:number[]=[];let done=false,caught:unknown;const pending=owner.forEachMetadataBatchV1(Array.from({length:19},(_,index)=>index),async index=>{checkResourceLeaseControl(cancelled);accepted.push(index);await new Promise<void>(release=>releases.set(index,release));checkResourceLeaseControl(cancelled);return index;},()=>{throw Error("Cancelled metadata was reduced");}).then(()=>{done=true;},error=>{caught=error;done=true;});await tick();expect(accepted).toHaveLength(8);abort.abort(cancel);for(let index=7;index>0;index--)releases.get(index)!();await tick();expect(done).toBe(false);expect(accepted).toHaveLength(8);checkResourceLeaseControl(control);releases.get(0)!();await pending;expect(done).toBe(true);expect(caught).toBe(cancel);expect(accepted).toHaveLength(8);const mode=process.env.SEMIO_BUILD_MODE;if(mode!=="prototype"&&mode!=="release")throw Error("Exact batch fixture mode required");const evidence=JSON.stringify({version:1,summaries,actual});expect(Buffer.byteLength(evidence)).toBeLessThanOrEqual(16384);await writeFile(join(artifacts,"rust-metadata-batch-fixture-"+mode+".json"),evidence);console.log=log;log("[DEBUG] Actual nested opposite-kind metadata, same-scan tie Source, working protection, symlink exclusion and absent roots match independent Python enumeration");
  }finally{console.log=log;await rm(root,{recursive:true});await rm(external,{recursive:true});}
 });
},120000);
