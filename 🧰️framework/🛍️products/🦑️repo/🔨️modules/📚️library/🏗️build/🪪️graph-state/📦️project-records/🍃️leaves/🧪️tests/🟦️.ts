import{afterAll,expect,test}from"bun:test";
import{existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{createProjectFleetPortV1,type ProjectRecordInfoV1}from"../../🟦️.ts";
import{adoptLeavesV1,leafPortWithDirectPutV1,leafReferencesOfTextV1,leafSourcesV1,leafWidthV1,stagedLeafPortV1}from"../🟦️.ts";
import{graphFleetClientV1,fetchGraphEntryV1}from"../../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../🧶️nx/🟦️.ts";
import{projectRecordStateV1}from"../../🟦️.ts";
import type{ClosurePortV1}from"../../../../🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts";

/** 🍃️ The leaf pieces a record names (inventories of source closures and cargo locks, files of the producing machine only) travel with the record: published into the blob store of the fleet cache before the entry, listed in its note, fetched and verified by a machine whose store is empty before the record is judged, and an entry whose piece nobody has is incomplete, never stale. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`leaves-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(bytes:string|Uint8Array)=>createHash("sha256").update(bytes).digest("hex");
const independent=(bytes:string)=>{const hasher=new Bun.CryptoHasher("sha256");hasher.update(bytes);return hasher.digest("hex");};
const NS={source:"build-input-closures",lock:"build-lock-closures"} as const;

const storeOf=(directory:string):ClosurePortV1=>{
 const folder=(namespace:string)=>join(directory,namespace),path=(namespace:string,key:string)=>join(folder(namespace),key+".json");
 return{directory:folder as ClosurePortV1["directory"],size:(namespace,key)=>existsSync(path(namespace,key))?readFileSync(path(namespace,key)).length:undefined,read:async(namespace,key)=>existsSync(path(namespace,key))?readFileSync(path(namespace,key)):undefined,put:async(namespace,key,bytes)=>{mkdirSync(folder(namespace),{recursive:true});writeFileSync(path(namespace,key),bytes);}};
};
const put=async(store:ClosurePortV1,namespace:keyof typeof NS,text:string)=>{const key=sha(text);await store.put(NS[namespace],key,Buffer.from(text));return key;};
const imageOf=(source:readonly string[],lock:readonly string[])=>JSON.stringify({name:"alpha",tasks:[...source.map(key=>({env:`SEMIO_SOURCE_CLOSURE_${key}`})),...lock.map(key=>({env:`SEMIO_CARGO_LOCK_${key}`}))]});
const infoOf=(root:string,name:string,image:string,taken:unknown[]):ProjectRecordInfoV1=>({cachePath:join(root,"np",name+".json"),fleet:"libs/"+name,keyText:JSON.stringify(["rev",null,[["authority"],root+"/libs/"+name,{name}]]),exportRecord:()=>({digest:sha(name),authority:[],observations:[{kind:"file",path:`${root}/libs/${name}/a.ts`,fact:{sha256:"x",size:1,mtimeMs:1,ctimeMs:1,ino:1}}],image}),async import(portable){taken.push(portable);return true;}});
async function world(){
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 return{server,environment:{SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"}};
}

test("the leaves of a text are the digests its tokens name, each once, with their namespace",()=>{
 const a="a".repeat(64),b="b".repeat(64);
 expect(leafReferencesOfTextV1(`x SEMIO_CARGO_LOCK_${b} y SEMIO_SOURCE_CLOSURE_${a} SEMIO_SOURCE_CLOSURE_${a} SEMIO_SOURCE_CLOSURE_short`)).toEqual([{namespace:NS.source,hash:a},{namespace:NS.lock,hash:b}]);
 expect(leafReferencesOfTextV1("nothing")).toEqual([]);
 expect(leafWidthV1(1)).toBe(2);expect(leafWidthV1(64)).toBe(8);expect(leafWidthV1(4)).toBe(4);
});

test("a leaf this machine lacks becomes a source that reads nothing, so the publication refuses instead of storing a record nobody can use",async()=>{
 const store=storeOf(temp("src")),have=await put(store,"source","{\"version\":1,\"files\":[\"a.ts\"]}"),lack="c".repeat(64);
 const sources=leafSourcesV1(store,imageOf([have,lack],[]));
 expect(sources.map(row=>row.hash).sort()).toEqual([have,lack].sort());
 expect(await sources.find(row=>row.hash===have)!.read()).toBeDefined();expect(await sources.find(row=>row.hash===lack)!.read()).toBeUndefined();
});

test("adoption fetches the missing leaves verified by hash, keeps held ones, reports what nobody has and refuses wrong bytes",async()=>{
 const {server,environment}=await world();
 try{
  const settings=graphFleetSettingsV1(temp("root"),environment)!,client=graphFleetClientV1(settings),good="{\"version\":1,\"files\":[\"b.ts\"]}",key=sha(good),forged="d".repeat(64);
  expect((await client.putBlob(key,Buffer.from(good))).stored).toBe(true);
  expect((await client.putBlob(forged,Buffer.from("not the bytes"))).stored,"the cache accepts only bytes that hash to the name").toBe(false);
  const store=storeOf(temp("own")),held=await put(store,"source","{\"held\":1}");
  const result=await adoptLeavesV1({client,port:store,leaves:[{namespace:NS.source,hash:key,size:good.length},{namespace:NS.source,hash:held,size:11},{namespace:NS.lock,hash:"e".repeat(64),size:5},{namespace:NS.source,hash:forged,size:13}]});
  expect(result.fetched).toBe(1);expect(result.present).toBe(1);expect(result.missing.map(row=>row.hash).sort()).toEqual(["e".repeat(64),forged].sort());
  expect(readFileSync(join(store.directory(NS.source),key+".json"),"utf8")).toBe(good);expect(independent(good)).toBe(key);
 }finally{await server.stop();}
});

test("a record published by one machine is complete for a machine with another root and an empty store: leaves are in the fleet before the entry, listed in its note, and in the taker's store before the record is imported",async()=>{
 const {server,environment}=await world();
 try{
  const rootA=temp("a"),rootB=temp("b"),storeA=storeOf(join(rootA,"leaves")),storeB=storeOf(join(rootB,"leaves"));
  const shared=await put(storeA,"source","{\"version\":1,\"files\":[\"shared.ts\"]}"),own=await put(storeA,"source","{\"version\":1,\"files\":[\"alpha.ts\"]}"),lock=await put(storeA,"lock","{\"version\":1,\"manifest\":\"alpha/Cargo.toml\"}");
  const a=createProjectFleetPortV1({root:rootA,environment,owner:"a:1",leaves:storeA})!,b=createProjectFleetPortV1({root:rootB,environment,owner:"b:1",leaves:storeB})!;
  const image=imageOf([shared,own],[lock]),gateA=await a.gate(infoOf(rootA,"alpha",image,[]));gateA.publish();await a.flush();
  expect(a.stats().published).toBe(1);
  const client=graphFleetClientV1(graphFleetSettingsV1(rootB,environment)!),found=await fetchGraphEntryV1(client,projectRecordStateV1({root:rootA,fleet:"libs/alpha",keyText:infoOf(rootA,"alpha",image,[]).keyText,platform:(await import("../../../🟦️.ts")).graphPlatformV1()}));
  expect(found&&found!=="fault"?found.note.leaves!.map(row=>row.hash).sort():[]).toEqual([shared,own,lock].sort());
  for(const hash of[shared,own,lock])expect(await client.hasBlob(hash),"every leaf is in the fleet cas").toBe(true);
  const taken:unknown[]=[],importedWith:boolean[]=[],info=infoOf(rootB,"alpha",image,taken);
  const gateB=await b.gate({...info,async import(portable){importedWith.push([shared,own].every(key=>storeB.size(NS.source,key)!==undefined)&&storeB.size(NS.lock,lock)!==undefined);return info.import(portable);}});
  expect(gateB.seeded).toBe(true);expect(importedWith).toEqual([true]);expect(b.stats().leaves).toBe(3);expect(b.stats().incomplete).toBe(0);
  for(const [namespace,hash] of[[NS.source,shared],[NS.source,own],[NS.lock,lock]] as const)expect(sha(readFileSync(join(storeB.directory(namespace),hash+".json")))).toBe(hash);
  const c=createProjectFleetPortV1({root:rootB,environment,owner:"b:2",leaves:storeB})!,again=await c.gate(infoOf(rootB,"alpha",image,[]));
  expect(again.seeded).toBe(true);expect(c.stats().leaves,"held leaves are not fetched again").toBe(0);
 }finally{await server.stop();}
});

test("a second owner that shares leaves with the first sends only the leaves the fleet lacks",async()=>{
 const {server,environment}=await world();
 try{
  const root=temp("a"),store=storeOf(join(root,"leaves")),shared=await put(store,"source","{\"version\":1,\"files\":[\"shared.ts\"]}"),one=await put(store,"source","{\"version\":1,\"files\":[\"one.ts\"]}"),two=await put(store,"source","{\"version\":1,\"files\":[\"two.ts\"]}");
  const port=createProjectFleetPortV1({root,environment,owner:"a:1",leaves:store})!;
  let puts=0;const client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!),count=async()=>{let n=0;for(const hash of[shared,one,two])if(await client.hasBlob(hash))n++;return n;};
  for(const [name,keys] of[["one",[shared,one]],["two",[shared,two]]] as const){const gate=await port.gate(infoOf(root,name,imageOf(keys,[]),[]));gate.publish();await port.flush();puts+=await count();}
  expect(puts).toBe(2+3);expect(port.stats().published).toBe(2);
 }finally{await server.stop();}
});

test("a record whose leaf the producer does not hold is not published, and an entry whose leaf the fleet lost is incomplete for the taker (typed, not stale) and healed by the republication of whoever infers it",async()=>{
 const {server,environment}=await world();
 try{
  const rootA=temp("a"),rootB=temp("b"),storeA=storeOf(join(rootA,"leaves")),storeB=storeOf(join(rootB,"leaves")),here=await put(storeA,"source","{\"version\":1,\"files\":[\"x.ts\"]}"),ghost="f".repeat(64);
  const a=createProjectFleetPortV1({root:rootA,environment,owner:"a:1",leaves:storeA})!;
  const bad=await a.claim(infoOf(rootA,"ghost",imageOf([here,ghost],[]),[]));
  expect(bad.state).toBe("granted");if(bad.state==="granted"){expect(await bad.publish()).toBe(false);expect(bad.code()).toBe("leaf-missing");}
  const gate=await a.gate(infoOf(rootA,"beta",imageOf([here],[]),[]));gate.publish();await a.flush();
  const lines:string[]=[],b=createProjectFleetPortV1({root:rootB,environment,owner:"b:1",leaves:storeB,say:line=>lines.push(line)})!;
  const lossy=graphFleetClientV1(graphFleetSettingsV1(rootB,environment)!),dropped=createProjectFleetPortV1({root:rootB,environment,owner:"b:2",leaves:storeB,say:line=>lines.push(line),client:{...lossy,getBlob:async(hash,size)=>hash===here?undefined:lossy.getBlob(hash,size)} as typeof lossy})!;
  const taken:unknown[]=[],gateB=await dropped.gate(infoOf(rootB,"beta",imageOf([here],[]),taken));
  expect(gateB.seeded).toBe(false);expect(taken.length,"the record is not imported without its leaf").toBe(0);expect(dropped.stats().incomplete).toBe(1);expect(lines.join("\n")).toContain("Blattstücke");
  gateB.publish();await dropped.flush();expect(dropped.stats().reinferred,"the inferring reader republishes the entry").toBe(1);
  expect(b.stats().incomplete).toBe(0);
 }finally{await server.stop();}
});

test("leaves produced under two different roots for the same inputs are byte-identical and name no machine path",async()=>{
 const {captureSourceInputInventoryV1,captureCargoLockInventoryV1}=await import("../../../../📥️inputs/📋️data/📥️load/🟨️.mjs");
 const make=(label:string)=>{const root=temp(label);mkdirSync(join(root,"lib","src"),{recursive:true});writeFileSync(join(root,"lib","src","a.rs"),"fn main(){}");writeFileSync(join(root,"lib","Cargo.toml"),"[package]\nname=\"a\"\n");return root;};
 const one=make("root-one-long-name"),two=make("r2");
 const source=(root:string)=>captureSourceInputInventoryV1({version:1,files:["lib/Cargo.toml","lib/src/a.rs"]},root).text,lock=(root:string)=>captureCargoLockInventoryV1({version:1,manifest:"lib/Cargo.toml"},root).text;
 expect(source(one)).toBe(source(two));expect(lock(one)).toBe(lock(two));
 for(const text of[source(one),lock(one)]){expect(text).not.toContain(one);expect(text).not.toContain(tmpdir());}
 expect(sha(source(one))).toBe(independent(source(two)));
});

test("a process without the writer custody of the graph store still stores a fetched leaf, as the same immutable file under its digest, and refuses bytes that do not hash to the name",async()=>{
 const base=storeOf(temp("nocustody")),port=leafPortWithDirectPutV1({...base,put:async()=>{throw Error("Original bounded graph writer custody required");}}),text="{\"version\":1,\"files\":[\"c.ts\"]}",key=sha(text);
 await port.put(NS.source,key,Buffer.from(text));
 expect(readFileSync(join(base.directory(NS.source),key+".json"),"utf8")).toBe(text);
 await expect(port.put(NS.source,"9".repeat(64),Buffer.from(text))).rejects.toThrow("differs from its name");
 expect(base.size(NS.source,"9".repeat(64))).toBeUndefined();
});

test("a leaf that only the staging directory of the inference holds is read from there, any other from the store of the machine, and a put goes to the store",async()=>{
 const staging=temp("staging"),own=storeOf(temp("own")),inStore=await put(own,"source","{\"version\":1,\"files\":[\"store.ts\"]}"),stagedText="{\"version\":1,\"files\":[\"staged.ts\"]}",staged=sha(stagedText);
 mkdirSync(join(staging,NS.source),{recursive:true});writeFileSync(join(staging,NS.source,staged+".json"),stagedText);
 const port=stagedLeafPortV1(staging,own);
 expect(port.size(NS.source,staged)).toBe(stagedText.length);expect(Buffer.from((await port.read(NS.source,staged))!).toString("utf8")).toBe(stagedText);
 expect(port.size(NS.source,inStore)).toBeDefined();expect(port.size(NS.source,"9".repeat(64))).toBeUndefined();expect(await port.read(NS.source,"9".repeat(64))).toBeUndefined();
 await port.put(NS.lock,sha("lock"),Buffer.from("lock"));expect(own.size(NS.lock,sha("lock"))).toBe(4);
 const alone=stagedLeafPortV1(staging,undefined);expect(alone.size(NS.source,staged)).toBe(stagedText.length);expect(alone.size(NS.source,inStore)).toBeUndefined();
});

test("a pool child publishes the record of an owner it just inferred although its leaves exist only in its staging directory, and the leaves reach the fleet",async()=>{
 const {server,environment}=await world();
 try{
  const root=temp("pool"),staging=temp("pool-staging"),text=["{\"version\":1,\"files\":[\"one.rs\"]}","{\"version\":1,\"manifest\":\"one/Cargo.toml\"}"],source=sha(text[0]!),lock=sha(text[1]!);
  mkdirSync(join(staging,NS.source),{recursive:true});mkdirSync(join(staging,NS.lock),{recursive:true});
  writeFileSync(join(staging,NS.source,source+".json"),text[0]!);writeFileSync(join(staging,NS.lock,lock+".json"),text[1]!);
  const image=imageOf([source],[lock]),bare=createProjectFleetPortV1({root,environment,owner:"p:1",leaves:storeOf(temp("empty"))})!;
  const refused=await bare.claim(infoOf(root,"one",image,[]));
  expect(refused.state).toBe("granted");if(refused.state==="granted"){expect(await refused.publish(),"without the staging directory the leaves are missing").toBe(false);expect(refused.code()).toBe("leaf-missing");}
  const port=createProjectFleetPortV1({root,environment,owner:"p:2",leaves:stagedLeafPortV1(staging,storeOf(temp("empty2")))})!,granted=await port.claim(infoOf(root,"one",image,[]));
  expect(granted.state).toBe("granted");if(granted.state==="granted")expect(await granted.publish()).toBe(true);
  const client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!);
  for(const hash of[source,lock])expect(await client.hasBlob(hash),"the staged leaf is in the fleet cas").toBe(true);
 }finally{await server.stop();}
});
