import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
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
 const host=readFileSync(fileURLToPath(new URL("../../../../../../../../../../🏗️build/🕸️graph/🟦️.ts",import.meta.url)),"utf8"),build=host.indexOf("generationScopeV1);\n      if(image!==undefined)await sweepSupersededBlocksV1("),sweep=host.indexOf("minimumBlocks:24576,blocksPerHeader:6");
 expect(build).toBeGreaterThan(0);expect(sweep).toBeGreaterThan(0);expect(host).toContain("catch(error){if(invocation.control.signal.aborted)throw error;process.stderr.write(\"graph: sweep");
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

