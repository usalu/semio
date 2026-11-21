import{afterAll,expect,test}from"bun:test";
import {graphSourceV1} from "../../../../../../../../../../🏗️build/🕸️graph/🧪️tests/📖️source/🟦️.ts";
import{linkSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath,pathToFileURL}from"node:url";
import{spawnSync}from"node:child_process";
import{brotliCompressSync}from"node:zlib";

const scan=await import(new URL("../../🔎️scan/🟦️.ts",import.meta.url).href) as typeof import("../../🔎️scan/🟦️.ts");
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const hex=(character:string)=>character.repeat(64),old=(Date.now()-3600000)/1000;
const control=(cancelled=()=>false)=>({cancelled,progress:()=>{}});

/** 🧪️ An image root with one selection group whose `np` holds two records naming three of five blocks. */
function image(){
 const parent=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(parent,{recursive:true});
 const root=mkdtempSync(join(realpathSync(parent),"supersede-"));roots.push(root);
 const group=join(root,"inference","selections",hex("1")),np=join(group,"np");mkdirSync(join(np,"ob"),{recursive:true});
 for(const[name,blocks]of[["a".repeat(24),[hex("2"),hex("3")]],["b".repeat(24),[hex("3"),hex("4")]]]as const)
  writeFileSync(join(np,name+".json"),brotliCompressSync(Buffer.from(JSON.stringify({digest:hex("a"),resultSha256:hex("b"),authorityObservations:[],blocks,observations:[]})+"\n{}")));
 for(const digit of["2","3","4","5","6"]){const path=join(np,"ob",hex(digit)+".br");writeFileSync(path,"x");utimesSync(path,old,old);}
 return{root,np};
}

test("a group under its writer lease loses exactly the blocks no record names",async()=>{
 const{root,np}=image(),leases:string[]=[];
 const report=await scan.supersedeBuildGraphObservationBlocksV1(root,{lease:async(kind,identity)=>{leases.push(kind+":"+identity);return{release(){leases.push("released");}};}},control());
 expect(report).toMatchObject({groups:1,busy:0,removed:2,headers:2,cancelled:false});
 expect(readdirSync(join(np,"ob")).sort()).toEqual(["2","3","4"].map(digit=>hex(digit)+".br"));
 expect(leases).toEqual(["selection:"+hex("1"),"released"]);
});

test("a dry run removes nothing and a busy group is left alone",async()=>{
 const{root,np}=image();
 expect((await scan.supersedeBuildGraphObservationBlocksV1(root,{lease:async()=>({release(){}})},control(),{dryRun:true})).removed).toBe(2);
 expect(readdirSync(join(np,"ob")).length).toBe(5);
 const busy=await scan.supersedeBuildGraphObservationBlocksV1(root,{lease:async()=>undefined},control());
 expect(busy).toMatchObject({groups:0,busy:1,removed:0});expect(readdirSync(join(np,"ob")).length).toBe(5);
});

test("a cancelled scan touches nothing",async()=>{
 const{root,np}=image();
 const report=await scan.supersedeBuildGraphObservationBlocksV1(root,{lease:async()=>({release(){}})},control(()=>true));
 expect(report).toMatchObject({cancelled:true,removed:0});expect(readdirSync(join(np,"ob")).length).toBe(5);
});

test("the build removes superseded blocks after, never inside, its builder custody, and the collector removes them first",()=>{
 const host=graphSourceV1(),custody=host.indexOf("()=>withGenerationCustodyV1("),plan=host.indexOf('await markBuildCapacityPhaseV1("plan");',custody),build=host.indexOf("await sweepSupersededBlocksV1(",custody),sweep=host.indexOf("minimumBlocks:GRAPH_LIMITS_V1.sweepMinimumBlocks,blocksPerHeader:GRAPH_LIMITS_V1.sweepBlocksPerHeader");
 expect(custody).toBeGreaterThan(0);expect(plan).toBeGreaterThan(custody);expect(build).toBeGreaterThan(plan);expect(sweep).toBeGreaterThan(build);
 expect(host.slice(plan,build)).toMatch(/if\s*\([^)]*\brebuilding\b[^)]*\bimage\s*!==\s*undefined\s*\)\s*$/u);
 expect(host).toContain("catch(error){if(invocation.control.signal.aborted)throw error;process.stderr.write(\"graph: sweep");
 const reclaim=readFileSync(fileURLToPath(new URL("../../../../../../../../../../🏗️build/🧹️cache-gc/🚰️reclaim/🟦️.ts",import.meta.url)),"utf8");
 expect(reclaim.indexOf("ports.supersede")).toBeGreaterThan(0);expect(reclaim.indexOf("ports.supersede")).toBeLessThan(reclaim.indexOf("ports.index(image)"));
});

/** 🖼️ One selection group whose records name some closure inventories, plus old and young inventories of both namespaces. */
function closureImage(){
 const parent=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(parent,{recursive:true});
 const root=mkdtempSync(join(realpathSync(parent),"closures-"));roots.push(root);
 const np=join(root,"inference","selections",hex("1"),"np");mkdirSync(np,{recursive:true});
 const record=(name:string,refs:string[])=>writeFileSync(join(np,name+".json"),JSON.stringify({refs}));
 record("a".repeat(24),["source:"+hex("a"),"lock:"+hex("d")]);record("b".repeat(24),["source:"+hex("b")]);
 for(const[namespace,digit,age]of[["build-input-closures","a",2],["build-input-closures","b",2],["build-input-closures","c",2],["build-input-closures","e",0],["build-lock-closures","d",2],["build-lock-closures","f",2]]as const){
  const path=join(root,namespace,hex(digit)+".json");mkdirSync(join(root,namespace),{recursive:true});writeFileSync(path,"{}");const time=(Date.now()-age*3600000)/1000;utimesSync(path,time,time);
 }
 return{root,np,record};
}
const closureNames=(root:string)=>["build-input-closures","build-lock-closures"].flatMap(namespace=>{try{return readdirSync(join(root,namespace)).map(name=>namespace+"/"+name);}catch{return[];}}).sort();
type ReadPortV1=Parameters<typeof scan.indexBuildGraphMembersV1>[1]["read"];
const closureRead=(refuse=false):ReadPortV1=>async(requests,accept)=>{for(const file of requests){if(refuse)throw Error("Original closure read child refused");const bytes=readFileSync(file.path),value=JSON.parse(bytes.toString("utf8"));accept(file,{version:1,rows:[{path:file.path,hash:"0".repeat(64),expandedBytes:bytes.length,publicationGraph:null}],references:[...(value.refs??[])].sort(),links:[]});}};
const independentSurvivors=(root:string,graceHours:number)=>{
 const model=`import json,os,re,sys,time
root,grace=sys.argv[1],float(sys.argv[2])*3600
refs=set()
for base,_,names in os.walk(os.path.join(root,"inference")):
    for name in names:
        if name.endswith(".json") and base.endswith("np"):
            refs.update(json.load(open(os.path.join(base,name)))["refs"])
keep=[]
for namespace,prefix in (("build-input-closures","source:"),("build-lock-closures","lock:")):
    d=os.path.join(root,namespace)
    for name in sorted(os.listdir(d)) if os.path.isdir(d) else []:
        age=time.time()-os.stat(os.path.join(d,name)).st_mtime
        if prefix+name[:64] in refs or age<grace: keep.append(namespace+"/"+name)
print(json.dumps(sorted(keep)))`,run=Bun.spawnSync(["python3","-I","-c",model,root,String(graceHours)]);
 if(run.exitCode!==0)throw Error(run.stderr.toString());return JSON.parse(run.stdout.toString()) as string[];
};

test("closure inventories no leaf names are removed, referenced and young ones stay, and an independent model agrees",async()=>{
 const{root}=closureImage(),expected=independentSurvivors(root,1),dry=await scan.supersedeBuildGraphClosuresV1(root,{lease:async()=>({release(){}}),read:closureRead()},control(),{dryRun:true,graceMilliseconds:3600000});
 expect(dry).toMatchObject({candidates:2,removed:2,unvouched:0,busy:0});expect(closureNames(root).length).toBe(6);
 const report=await scan.supersedeBuildGraphClosuresV1(root,{lease:async()=>({release(){}}),read:closureRead()},control(),{graceMilliseconds:3600000});
 expect(report).toMatchObject({candidates:2,removed:2,unvouched:0,busy:0,cancelled:false});expect(closureNames(root)).toEqual(expected);
 expect(closureNames(root)).toEqual(["build-input-closures/"+hex("a")+".json","build-input-closures/"+hex("b")+".json","build-input-closures/"+hex("e")+".json","build-lock-closures/"+hex("d")+".json"]);
});

test("nothing is removed while a leaf's references are unproven or a group writer is busy",async()=>{
 const{root}=closureImage();
 expect(await scan.supersedeBuildGraphClosuresV1(root,{lease:async()=>({release(){}}),read:closureRead(true)},control())).toMatchObject({unvouched:1,removed:0});
 expect(await scan.supersedeBuildGraphClosuresV1(root,{lease:async()=>({release(){}}),read:closureRead()},control(),{maximumPending:1})).toMatchObject({unvouched:1,removed:0});
 expect(await scan.supersedeBuildGraphClosuresV1(root,{lease:async()=>undefined,read:closureRead()},control(),{attempts:2})).toMatchObject({busy:2,removed:0});
 expect(closureNames(root).length).toBe(6);
});

test("an inventory a writer starts naming while the sweep takes the group leases survives",async()=>{
 const{root,record}=closureImage();let named=false;
 const report=await scan.supersedeBuildGraphClosuresV1(root,{lease:async()=>{if(!named){named=true;record("c".repeat(24),["source:"+hex("c")]);}return{release(){}};},read:closureRead()},control(),{graceMilliseconds:3600000});
 expect(report.removed).toBe(1);expect(closureNames(root)).toContain("build-input-closures/"+hex("c")+".json");expect(closureNames(root)).not.toContain("build-lock-closures/"+hex("f")+".json");
});

test("first generation publication changes the closure census cut",async()=>{
 const{root}=closureImage(),leases:string[]=[];
 const before=closureNames(root),directory=join(root,"generations","o"),fragment=join(directory,hex("9")+".json.br");
 const report=await scan.supersedeBuildGraphClosuresV1(root,{lease:async kind=>{
  leases.push(kind);
  if(kind==="canonical"){mkdirSync(directory,{recursive:true});writeFileSync(fragment,JSON.stringify({refs:["source:"+hex("c")]}));}
  return{release(){}};
 },read:closureRead()},control());
 expect(report).toMatchObject({candidates:2,unvouched:1,removed:0});expect(closureNames(root)).toEqual(before);
 expect(leases).toContain("canonical");
 const next=await scan.supersedeBuildGraphClosuresV1(root,{lease:async()=>({release(){}}),read:closureRead()},control());
 expect(next).toMatchObject({candidates:1,removed:1});expect(closureNames(root)).toContain("build-input-closures/"+hex("c")+".json");
});

test("a leaf whose observation blocks are gone names nothing, any other refusal keeps every inventory",async()=>{
 const{root,np,record}=closureImage();record("d".repeat(24),["source:"+hex("c")]);const aged=(Date.now()-2*3600000)/1000;
 const youngRoot=closureImage();youngRoot.record("d".repeat(24),["source:"+hex("c")]);
 utimesSync(join(np,"d".repeat(24)+".json"),aged,aged);
 const deadRead:ReadPortV1=async(requests,accept)=>{for(const file of requests){if(file.path.endsWith("d".repeat(24)+".json"))throw Object.assign(Error("Original closure read child refused"),{closureReadFailure:{worker:{failure:{message:"Authenticated observation closure block required"}}}});await closureRead()([file],accept);}};
 const report=await scan.supersedeBuildGraphClosuresV1(root,{lease:async()=>({release(){}}),read:deadRead},control(),{graceMilliseconds:3600000});
 expect(report).toMatchObject({unvouched:0,removed:2});expect(closureNames(root)).not.toContain("build-input-closures/"+hex("c")+".json");
 expect(await scan.supersedeBuildGraphClosuresV1(youngRoot.root,{lease:async()=>({release(){}}),read:deadRead},control(),{graceMilliseconds:3600000})).toMatchObject({unvouched:1,removed:0});
 const other=closureImage();other.record("d".repeat(24),["source:"+hex("c")]);
 const otherRead:ReadPortV1=async(requests,accept)=>{for(const file of requests){if(file.path.endsWith("d".repeat(24)+".json"))throw Object.assign(Error("Original closure read child refused"),{closureReadFailure:{worker:{failure:{message:"Bounded closure index required"}}}});await closureRead()([file],accept);}};
 expect(await scan.supersedeBuildGraphClosuresV1(other.root,{lease:async()=>({release(){}}),read:otherRead},control(),{graceMilliseconds:3600000})).toMatchObject({unvouched:1,removed:0});expect(closureNames(other.root).length).toBe(6);
});


test("a known reference-bearing leaf outside the physical read policy keeps every inventory",async()=>{
 for(const invalidity of ["hardlink","directory"]){
  const{root,np,record}=closureImage();
  const path=join(np,"e".repeat(24)+".json");
  if(invalidity==="hardlink"){record("e".repeat(24),["source:"+hex("c")]);linkSync(path,join(root,"external-alias"));}
  else mkdirSync(path);
  const report=await scan.supersedeBuildGraphClosuresV1(root,{lease:async()=>{throw Error("A read-only preview does not hold a writer lease");},read:closureRead()},control(),{dryRun:true});
  expect(report,invalidity).toMatchObject({unvouched:1,candidates:0,removed:0});
  expect(closureNames(root)).toHaveLength(6);
 }
});


test("unresolved essential proof stops a preview while indexing keeps its later partial results",async()=>{
 const{root,record}=closureImage(),b="b".repeat(24)+".json",later="c".repeat(24)+".json",requested:string[]=[];
 record("c".repeat(24),["source:"+hex("c")]);
 const read:ReadPortV1=async(requests,accept)=>{
  for(const file of requests){
   const name=file.path.slice(-29);requested.push(name);
   if(name===b)throw Error("Essential closure proof unavailable");
   await closureRead()([file],accept);
  }
 };
 const ports={lease:async()=>({release(){}}),read};
 expect(await scan.supersedeBuildGraphClosuresV1(root,ports,control(),{dryRun:true})).toMatchObject({unvouched:1,candidates:0,removed:0});
 expect(requested).toEqual(["a".repeat(24)+".json",b,b]);
 requested.length=0;
 const partial=await scan.indexBuildGraphMembersV1(root,ports,control());
 expect(partial).toMatchObject({refused:1,indexed:2});
 expect(requested).toEqual(["a".repeat(24)+".json",b,b,later]);
});

const closureWorkerV1=fileURLToPath(new URL("../../../👷️closure-read-worker/🟨️.mjs",import.meta.url));
/** 🔬️ Asks the REAL closure read child about one leaf and turns its failure packet into the refusal the child port throws. */
const realRefusalV1=(root:string,file:{path:string;stamp:string;kind:string;bytes:number}):Error=>{
 const input=JSON.stringify({version:1,maximumReplyBytes:16764800})+"\n"+JSON.stringify({files:[file],imageRoot:root,remainingBytes:1<<30,root:process.cwd()})+"\nnull\n";
 const run=spawnSync("node",["--input-type=module","-e","const module=await import(process.argv[1]);await module.runCapturedBuildSourceV1();",pathToFileURL(closureWorkerV1).href],{input,cwd:root,encoding:"utf8"});
 return Object.assign(Error("Original closure read child refused"),{closureReadFailure:{version:1,worker:JSON.parse(run.stdout.split("\n").find(Boolean)!),terminal:{status:run.status,signal:null}}});
};
/** 🧮️ The independent oracle: Python's zlib says whether the bytes are a complete gzip or zlib stream at all. */
const pythonInflatesV1=(path:string):boolean=>{
 const run=Bun.spawnSync(["python3","-I","-c","import sys,zlib\nd=zlib.decompressobj(47)\ntry:\n d.decompress(open(sys.argv[1],'rb').read());print(int(d.eof))\nexcept zlib.error:print(0)",path]);
 return run.stdout.toString().trim()==="1";
};

test("a settled leaf the real closure read child cannot inflate names nothing, a young one and any other refusal keep every inventory",async()=>{
 const build=(age:number)=>{const image=closureImage(),np=join(image.root,"inference","np"),leaf=join(np,"e".repeat(24)+".json");mkdirSync(np);writeFileSync(leaf,Buffer.from([0]));const time=(Date.now()-age*3600000)/1000;utimesSync(leaf,time,time);return{...image,leaf};};
 const{root,leaf}=build(2);expect(pythonInflatesV1(leaf),"the independent decoder agrees that the leaf is no complete stream").toBe(false);
 const refusing=(error:()=>Error|undefined):ReadPortV1=>async(requests,accept)=>{for(const file of requests){if(file.path.endsWith("e".repeat(24)+".json")){const refusal=error();if(refusal)throw refusal;}await closureRead()([file],accept);}};
 const real=(image:{root:string}):ReadPortV1=>async(requests,accept)=>{for(const file of requests){if(file.path.endsWith("e".repeat(24)+".json"))throw realRefusalV1(image.root,file as never);await closureRead()([file],accept);}};
 const dry=await scan.supersedeBuildGraphClosuresV1(root,{lease:async()=>({release(){}}),read:real({root})},control(),{dryRun:true,graceMilliseconds:3600000});
 expect(dry,"the settled undecodable leaf does not stall the sweep").toMatchObject({unvouched:0,candidates:2,removed:2});expect(closureNames(root).length).toBe(6);
 const report=await scan.supersedeBuildGraphClosuresV1(root,{lease:async()=>({release(){}}),read:real({root})},control(),{graceMilliseconds:3600000});
 expect(report).toMatchObject({unvouched:0,removed:2,cancelled:false});expect(closureNames(root)).toEqual(["build-input-closures/"+hex("a")+".json","build-input-closures/"+hex("b")+".json","build-input-closures/"+hex("e")+".json","build-lock-closures/"+hex("d")+".json"]);
 const young=build(0);expect(await scan.supersedeBuildGraphClosuresV1(young.root,{lease:async()=>({release(){}}),read:real(young)},control(),{graceMilliseconds:3600000})).toMatchObject({unvouched:1,removed:0});
 const probe=realRefusalV1(root,{path:leaf,stamp:[(row=>[row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].join(":"))(lstatSync(leaf,{bigint:true}))].join(""),kind:"inference",bytes:1}) as Error&{closureReadFailure:{worker:{failure:{code:string};closurePayloadRefusal:{check:string}}}};
 expect(probe.closureReadFailure.worker.failure.code).toBe("Z_BUF_ERROR");expect(probe.closureReadFailure.worker.closurePayloadRefusal.check).toBe("inflation");
 const other=build(2);expect(await scan.supersedeBuildGraphClosuresV1(other.root,{lease:async()=>({release(){}}),read:refusing(()=>Object.assign(Error("refused"),{closureReadFailure:{worker:{failure:{code:"EIO",message:"read failed"},closurePayloadRefusal:null}}}))},control(),{graceMilliseconds:3600000})).toMatchObject({unvouched:1,removed:0});expect(closureNames(other.root).length).toBe(6);
 const memory=build(2);expect(await scan.supersedeBuildGraphClosuresV1(memory.root,{lease:async()=>({release(){}}),read:refusing(()=>Object.assign(Error("refused"),{closureReadFailure:{worker:{failure:{code:"Z_MEM_ERROR",message:"out of memory"},closurePayloadRefusal:{check:"inflation"}}}}))},control(),{graceMilliseconds:3600000})).toMatchObject({unvouched:1,removed:0});
 const budget=build(2);expect(await scan.supersedeBuildGraphClosuresV1(budget.root,{lease:async()=>({release(){}}),read:refusing(()=>Object.assign(Error("refused"),{closureReadFailure:{worker:{failure:{code:"ERR_BUFFER_TOO_LARGE",message:"too large"},closurePayloadRefusal:{check:"inflation"}}}}))},control(),{graceMilliseconds:3600000})).toMatchObject({unvouched:1,removed:0});
});

test("an observation block is never read on its own, its record header carries the references",async()=>{
 const{root}=closureImage(),np=join(root,"inference","selections",hex("1"),"np"),ob=join(np,"ob");mkdirSync(ob,{recursive:true});for(const name of["a","b"])rmSync(join(np,name.repeat(24)+".json"));
 writeFileSync(join(np,"c".repeat(24)+".json"),brotliCompressSync(Buffer.from(JSON.stringify({digest:hex("a"),resultSha256:hex("b"),authorityObservations:[],blocks:[hex("7")],observations:[]})+"\n{}")));
 const aged=(Date.now()-2*3600000)/1000;for(const digit of["7","8"]){const path=join(ob,hex(digit)+".br");writeFileSync(path,"x");utimesSync(path,aged,aged);}
 utimesSync(join(np,"c".repeat(24)+".json"),aged,aged);
 const requested:string[]=[],read:ReadPortV1=async(requests,accept)=>{for(const file of requests){requested.push(file.path);if(file.path.endsWith(".br")||file.path.endsWith("c".repeat(24)+".json")){accept(file,{version:1,rows:[{path:file.path,hash:"0".repeat(64),expandedBytes:1,publicationGraph:null}],references:file.path.endsWith("c".repeat(24)+".json")?["source:"+hex("c")]:[],links:[]});continue;}await closureRead()([file],accept);}};
 const report=await scan.supersedeBuildGraphClosuresV1(root,{lease:async()=>({release(){}}),read},control(),{graceMilliseconds:3600000});
 expect(report).toMatchObject({unvouched:0,cancelled:false});
 expect(requested.some(path=>path.endsWith(".br"))).toBe(false);expect(requested.some(path=>path.endsWith("c".repeat(24)+".json"))).toBe(true);
 expect(closureNames(root)).toContain("build-input-closures/"+hex("c")+".json");
});

test("indexing vouches for an observation block by its own bytes without asking the closure read child",async()=>{
 const{root}=closureImage(),ob=join(root,"inference","selections",hex("1"),"np","ob");mkdirSync(ob,{recursive:true});writeFileSync(join(ob,hex("7")+".br"),"block");
 const requested:string[]=[],report=await scan.indexBuildGraphMembersV1(root,{lease:async()=>({release(){}}),read:async(requests,accept)=>{for(const file of requests){requested.push(file.path);await closureRead()([file],accept);}}},control());
 expect(report.refused).toBe(0);expect(requested.some(path=>path.endsWith(".br"))).toBe(false);
 expect(readFileSync(join(root,"inference","selections",hex("1"),"members.jsonl"),"utf8")).toContain("np/ob/"+hex("7")+".br");
});

test("groups that share a writer namespace take one lease, and a busy namespace is waited for on its own",async()=>{
 const{root}=closureImage();mkdirSync(join(root,"generations"),{recursive:true});
 const held=new Set<string>(),busyOnce=new Set(["selection\0"+hex("1")]),queued:string[]=[];
 const lease=async(kind:string,identity:string)=>{const key=kind+"\0"+identity;if(held.has(key)||busyOnce.has(key))return undefined;held.add(key);return{release(){held.delete(key);}};};
 const queue=async(kind:string,identity:string)=>{const key=kind+"\0"+identity;queued.push(key);busyOnce.delete(key);return lease(kind,identity);};
 const report=await scan.supersedeBuildGraphClosuresV1(root,{lease:lease as never,queue:queue as never,read:closureRead()},control(),{graceMilliseconds:3600000});
 expect(report).toMatchObject({busy:1,removed:2,unvouched:0});expect(queued).toEqual(["selection\0"+hex("1")]);expect(held.size).toBe(0);
});


test("Law: an urgent sweep queues for the writer behind overlapping shared cohort holders and is served once they release, later cohort arrivals wait behind it, and a sweep that is not urgent leaves a busy group alone as before",async()=>{
 const leases=await import("../../../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts") as typeof import("../../../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts");
 const{root}=image(),directory=mkdtempSync(join(realpathSync(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"sweep-leases-"));roots.push(directory);
 const resourceOf=(kind:string,identity:string)=>"sweep-test:"+kind+":"+identity,events:string[]=[],signal=new AbortController().signal;
 const port=scan.createSweepLeasePortV1({
  acquire:async(kind,identity)=>{try{return await leases.acquireResourceLease({directory,resource:resourceOf(kind,identity),mode:"exclusive",signal,timeoutMs:0});}catch(error){if(/timed out/iu.test(String((error as Error).message)))return undefined;throw error;}},
  acquireQueued:async(kind,identity,timeoutMs)=>{try{return await leases.acquireQueuedResourceLease({directory,resource:resourceOf(kind,identity),mode:"exclusive",owner:"sweeper",signal,timeoutMs});}catch(error){if(/timed out/iu.test(String((error as Error).message)))return undefined;throw error;}},
  waitMilliseconds:20000});
 const resource=resourceOf("selection",hex("1")),cohort=(owner:string)=>leases.acquireQueuedResourceLease({directory,resource,mode:"shared",sharedCohort:true,owner,signal});
 const first=await cohort("assembler"),second=await cohort("joiner");
 const quick=await scan.supersedeBuildGraphObservationBlocksV1(root,{lease:port},control(),{minimumBlocks:0});
 expect(quick,"not urgent: the group is busy, nothing waits").toMatchObject({groups:0,busy:1,removed:0});
 const sweep=scan.supersedeBuildGraphObservationBlocksV1(root,{lease:port},control(),{minimumBlocks:1,graceMilliseconds:0}).then(report=>{events.push("swept");return report;});
 await new Promise(done=>setTimeout(done,400));
 expect(events,"the sweeper waits and holds nothing").toEqual([]);
 const facade=fileURLToPath(new URL("../../../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts",import.meta.url)),script=`const l=await import(${JSON.stringify(facade)});const x=await l.acquireQueuedResourceLease({directory:${JSON.stringify(directory)},resource:${JSON.stringify(resource)},mode:"shared",sharedCohort:true,owner:"late",signal:new AbortController().signal});process.stdout.write("late\\n");x.release();process.exit(0);`;
 const child=Bun.spawn(["bun","-e",script],{stdout:"pipe",stderr:"inherit"});
 const lateText=new Response(child.stdout).text().then(text=>{events.push(text.trim());});
 await new Promise(done=>setTimeout(done,1500));
 expect(events,"a later cohort arrival of another process queues behind the sweeper").toEqual([]);
 first.release();second.release();
 const report=await sweep;await lateText;
 expect(events,"the sweep first, the later arrival after it").toEqual(["swept","late"]);
 expect(report.removed,"the superseded blocks are gone").toBe(2);
},30000);
