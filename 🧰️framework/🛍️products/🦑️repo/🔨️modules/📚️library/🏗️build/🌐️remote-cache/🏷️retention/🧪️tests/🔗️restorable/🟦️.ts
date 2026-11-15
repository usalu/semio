import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {existsSync,mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {CAS_POLICY_V1} from "../../../../🗄️cas/🧬️schema/🟦️.ts";
import {createCasStoreV1,type CasStoreV1} from "../../../../🗄️cas/🗃️store/🟦️.ts";
import {slotIdV1} from "../../../../🪢️slots/🟦️.ts";
import {parseRemoteCacheActionResultV1} from "../../../🔬️action-result/🟦️.ts";
import {createClaimTableV1} from "../../../🎟️claims/🟦️.ts";
import {collectRemoteCacheDirectoryV1} from "../../../🧹️collector/🟦️.ts";
import {RemoteCacheStoreV1} from "../../../💾️store/🟦️.ts";

const scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`rcache-live-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const sha=(value:Uint8Array|string)=>createHash("sha256").update(value).digest("hex");
const text=(value:string)=>new TextEncoder().encode(value);
const slot=(name:string)=>slotIdV1("nx-task",[name,"build","","p"]);
const body=(bytes:Uint8Array)=>(async function*(){yield bytes;})();
function seeded(seed:number):()=>number{return ()=>{seed=(seed+0x6d2b79f5)|0;let value=Math.imul(seed^(seed>>>15),1|seed);value^=value+Math.imul(value^(value>>>7),61|value);return ((value^(value>>>14))>>>0)/4294967296;};}

const quiet=(minAgeMs:number)=>({...CAS_POLICY_V1,minAgeMs,partition:{lowUsedFraction:0.98,highUsedFraction:0.99,criticalUsedFraction:0.999}});
const varint=(value:number):number[]=>{const out:number[]=[];let rest=value;while(rest>=0x80){out.push((rest&0x7f)|0x80);rest=Math.floor(rest/0x80);}out.push(rest);return out;};
const result=(blobs:readonly Uint8Array[]):Uint8Array=>{
 const parts:number[]=[];
 blobs.forEach((blob,index)=>{
  const digest=[0x0a,0x40,...text(sha(blob)),0x10,...varint(blob.length)],path=text(`out${index}`),file=[0x0a,path.length,...path,0x12,digest.length,...digest];
  parts.push(0x12,...varint(file.length),...file);
 });
 return new Uint8Array(parts);
};

type Rig={store:RemoteCacheStoreV1;host:CasStoreV1;folder:string};
async function rig(label:string,options:{grace?:number;minAge?:number;claimed?:(namespace:string,kind:"ac"|"cas"|"nx",key:string)=>boolean}={}):Promise<Rig>{
 const folder=directory(label),host=createCasStoreV1({root:directory(label+"-cas"),policy:quiet(options.minAge??0)});
 const store=await RemoteCacheStoreV1.open({directory:folder,host,maximumBytes:2**30,maximumEntryBytes:2**22,maximumAgeMilliseconds:0,graceMilliseconds:options.grace??80,...options.claimed?{claimed:options.claimed}:{}});
 return {store,host,folder};
}
const blob=async(r:Rig,bytes:Uint8Array)=>{expect(["created","exists"]).toContain(await r.store.put("main","cas",sha(bytes),body(bytes),{mode:"content",verify:true,declaredLength:bytes.length}));return sha(bytes);};
const action=async(r:Rig,key:string,blobs:readonly Uint8Array[],name:string|null)=>{
 for(const bytes of blobs)await blob(r,bytes);
 expect(await r.store.put("main","ac",key,body(result(blobs)),{mode:"replace",...name===null?{}:{slot:{slot:slot(name)}}})).toMatch(/created|replaced/u);
};
const archive=async(r:Rig,key:string,bytes:Uint8Array,name:string|null)=>{expect(await r.store.put("main","nx",key,body(bytes),{mode:"keep",declaredLength:bytes.length,...name===null?{}:{slot:{slot:slot(name)}}})).toBe("created");};
const restorable=async(r:Rig,kind:"ac"|"nx",key:string,expected:readonly Uint8Array[]):Promise<boolean>=>{
 if(kind==="nx"){const entry=await r.store.read("main","nx",key);if(!entry)return false;const bytes=await r.host.read(sha(expected[0]!));return bytes!==undefined&&Buffer.from(bytes).equals(Buffer.from(expected[0]!));}
 const bytes=await r.store.readBytes("main","ac",key,2**20);if(!bytes)return false;
 const parsed=parseRemoteCacheActionResultV1(bytes);
 for(const value of expected){const present=await r.host.read(sha(value));if(!present||!Buffer.from(present).equals(Buffer.from(value)))return false;}
 return parsed.references.length===expected.length;
};
const gone=(r:Rig,bytes:Uint8Array)=>!existsSync(r.host.objectPath(sha(bytes)));

test("unit outputs: the current action result restores whole after the older state went, shared outputs stay, outputs only the old state used go",async()=>{
 const r=await rig("unit"),shared=text("rlib-shared".repeat(30)),oldOnly=text("rmeta-old".repeat(30)),newOnly=text("rmeta-new".repeat(30)),dep=text("dep-info".repeat(30));
 await action(r,"a".repeat(64),[shared,oldOnly,dep],"unit:x");await action(r,"b".repeat(64),[shared,newOnly,dep],"unit:x");
 await sleep(150);await r.store.collect();
 expect(await restorable(r,"ac","b".repeat(64),[shared,newOnly,dep])).toBe(true);expect(await r.store.read("main","ac","a".repeat(64))).toBeUndefined();
 expect(gone(r,oldOnly)).toBe(true);expect(gone(r,shared)).toBe(false);expect(gone(r,dep)).toBe(false);await r.store.close();
});

test("Nx archives: the current archive restores, the superseded body is reclaimed, an unslotted archive and a second slot are untouched",async()=>{
 const r=await rig("nx"),one=text("archive-1".repeat(40)),two=text("archive-2".repeat(40)),other=text("archive-other".repeat(40)),plain=text("archive-plain".repeat(40));
 await archive(r,"n1",one,"task:a");await archive(r,"n2",two,"task:a");await archive(r,"o1",other,"task:b");await archive(r,"p1",plain,null);
 await sleep(150);await r.store.collect();await r.store.collect();
 for(const[key,bytes]of[["n2",two],["o1",other],["p1",plain]] as const)expect(await restorable(r,"nx",key,[bytes]),key).toBe(true);
 expect(await r.store.read("main","nx","n1")).toBeUndefined();expect(gone(r,one)).toBe(true);await r.store.close();
});

test("graph documents (package F): a result with a large document blob and a stdout note; only the superseded state's document goes",async()=>{
 const r=await rig("graph"),doc1=text("graph-document-1".repeat(2000)),doc2=text("graph-document-2".repeat(2000)),note=text("Note: root neutral");
 await action(r,"c".repeat(64),[doc1,note],"graph-state:linux");await action(r,"d".repeat(64),[doc2,note],"graph-state:linux");
 await sleep(150);await r.store.collect();
 expect(await restorable(r,"ac","d".repeat(64),[doc2,note])).toBe(true);expect(gone(r,doc1)).toBe(true);expect(gone(r,note)).toBe(false);await r.store.close();
});

test("install packs, pinned downloads, crate archives and wasm payloads uploaded as bare blobs, and install manifests stored unslotted, survive any amount of churn around them",async()=>{
 const r=await rig("bare"),pack=text("install-pack-tar-gz".repeat(100)),download=text("pinned-download".repeat(100)),crate=text("crate-archive".repeat(100)),wasm=text("wasm-payload".repeat(100)),manifest=text("rows\nof\nthe manifest".repeat(10));
 const bare=[pack,download,crate,wasm,manifest];for(const bytes of bare)await blob(r,bytes);
 await archive(r,`inst-${"f".repeat(64)}`,manifest,null);
 for(let round=0;round<4;round++){await action(r,String(round).repeat(64),[text("churn"+round),crate],"churn");await sleep(100);await r.store.collect();}
 for(const bytes of bare)expect(gone(r,bytes),sha(bytes).slice(0,8)).toBe(false);
 expect(await restorable(r,"nx",`inst-${"f".repeat(64)}`,[manifest])).toBe(true);
 await r.store.close();
});

test("a blob that a pin or a recent access protects survives although a superseded result referenced it (the digest shared with a pinned download or a wasm payload)",async()=>{
 const pinnedRig=await rig("pinned",{minAge:0}),pinned=text("pinned-and-referenced".repeat(50));
 await action(pinnedRig,"1".repeat(64),[pinned],"u:p");await action(pinnedRig,"2".repeat(64),[text("replacement")],"u:p");
 await pinnedRig.host.pin("fleet-fetch-holder",[sha(pinned)],60_000);await sleep(150);await pinnedRig.store.collect();
 expect(gone(pinnedRig,pinned)).toBe(false);await pinnedRig.store.close();
 const recent=await rig("recent",{minAge:600_000}),payload=text("wasm-payload-shared".repeat(50));
 await action(recent,"3".repeat(64),[payload],"u:w");await action(recent,"4".repeat(64),[text("replacement2")],"u:w");await sleep(150);await recent.store.collect();
 expect(gone(recent,payload),"accessed within the host store's minimum age").toBe(false);await recent.store.close();
});

test("claims: a live claim keeps a superseded entry, its lapse (a capped lifetime) does not delete a fresh entry, and both ending lets it go",async()=>{
 let clock=1_000_000;const table=createClaimTableV1({now:()=>clock,sweepMilliseconds:0});
 const r=await rig("claims",{grace:80,claimed:(namespace,kind,key)=>kind!=="cas"&&table.claimed(namespace,kind,key)}),one=text("claim-1".repeat(30)),two=text("claim-2".repeat(30));
 await archive(r,"k1",one,"claimed");expect(table.claim("main","nx","k1",{owner:"a:1:x",ttlMilliseconds:60_000}).state).toBe("granted");
 await archive(r,"k2",two,"claimed");await sleep(150);await r.store.collect();
 expect(await restorable(r,"nx","k1",[one]),"live claim").toBe(true);
 clock+=30*60_000;expect(table.claimed("main","nx","k1"),"lifetime cap passed").toBe(false);
 await archive(r,"k3",text("claim-3".repeat(30)),"claimed");expect(table.claim("main","nx","k2",{owner:"b:1:y",ttlMilliseconds:60_000}).state).toBe("granted");
 await r.store.collect();expect(await restorable(r,"nx","k3",[text("claim-3".repeat(30))]),"fresh entry in grace").toBe(true);
 expect(await restorable(r,"nx","k2",[two]),"k2 is superseded just now and claimed").toBe(true);
 await sleep(150);clock+=30*60_000;await r.store.collect();
 expect(await r.store.read("main","nx","k1")).toBeUndefined();expect(await r.store.read("main","nx","k2")).toBeUndefined();expect(await restorable(r,"nx","k3",[text("claim-3".repeat(30))])).toBe(true);
 await r.store.close();
});

test("property: after any collection, store-driven or external, every live entry of every kind restores whole, the newest key of every slot is live, and bare blobs stay",async()=>{
 const next=seeded(20261114);
 for(let trial=0;trial<6;trial++){
  const r=await rig("prop"+trial,{grace:30}),slots=3,live=new Map<string,{kind:"ac"|"nx";blobs:Uint8Array[]}>(),newest=new Map<number,string>(),bare:Uint8Array[]=[];let counter=0;
  for(let index=0;index<4;index++){const bytes=text(`bare-${trial}-${index}`.repeat(10));bare.push(bytes);await blob(r,bytes);}
  const pool=Array.from({length:6},(_,index)=>text(`shared-${trial}-${index}`.repeat(12)));
  for(let round=0;round<14;round++){
   for(let step=0;step<1+Math.floor(next()*3);step++){
    const slotIndex=Math.floor(next()*slots),key=`k${trial}x${++counter}`,name=next()<0.15?null:`prop:${slotIndex}`;
    if(next()<0.5){const bytes=text(`archive-${key}`.repeat(8));await archive(r,key,bytes,name);live.set(key,{kind:"nx",blobs:[bytes]});}
    else{const blobs=[pool[Math.floor(next()*pool.length)]!,text(`own-${key}`.repeat(8))];if(next()<0.5)blobs.push(pool[Math.floor(next()*pool.length)]!);const unique=[...new Map(blobs.map(value=>[sha(value),value])).values()];const hex=sha(key);await action(r,hex,unique,name);live.set(hex,{kind:"ac",blobs:unique});if(name!==null)newest.set(slotIndex,hex);continue;}
    if(name!==null)newest.set(slotIndex,key);
   }
   await sleep(Math.floor(next()*50));
   if(next()<0.5)await r.store.collect();else await collectRemoteCacheDirectoryV1({root:r.store.root,maximumBytes:2**30,maximumAgeMilliseconds:0,dryRun:false,verifyResults:true,hasBlobs:hashes=>r.host.has(hashes),graceMilliseconds:30,claimed:()=>false,reclaim:async(hashes,dryRun)=>{const{reclaimCasObjectsV1}=await import("../../../../🗄️cas/🪦️reclaim/🟦️.ts");const freed=await reclaimCasObjectsV1(r.host,hashes,{dryRun});return {entries:freed.entries,bytes:freed.bytes};}});
   for(const[key,entry]of live){
    const present=entry.kind==="nx"?await r.store.read("main","nx",key)!==undefined:await r.store.readBytes("main","ac",key,2**20)!==undefined;
    if(present)expect(await restorable(r,entry.kind,key,entry.blobs),`trial ${trial} round ${round}: ${key} restores whole`).toBe(true);
   }
   for(const key of newest.values()){const entry=live.get(key)!;expect(await restorable(r,entry.kind,key,entry.blobs),`trial ${trial} round ${round}: newest ${key} of its slot is live`).toBe(true);}
   for(const bytes of bare)expect(gone(r,bytes),`trial ${trial} round ${round}: bare blob`).toBe(false);
  }
  await r.store.close();
 }
});

test("an external collection that verifies action results asks the host store for their blobs: live results whose blobs are in the host store are not dangling, results whose blob is gone are",async()=>{
 const r=await rig("dangling"),kept=text("kept-output".repeat(20)),lost=text("lost-output".repeat(20));
 await action(r,"7".repeat(64),[kept],null);await action(r,"8".repeat(64),[lost],null);
 const{unlink}=await import("node:fs/promises");await unlink(r.host.objectPath(sha(lost)));
 const request={root:r.store.root,maximumBytes:2**30,maximumAgeMilliseconds:0,verifyResults:true,graceMilliseconds:80};
 const plan=await collectRemoteCacheDirectoryV1({...request,dryRun:true,hasBlobs:hashes=>r.host.has(hashes)});
 expect(plan.locked?undefined:plan.value.removed).toMatchObject({dangling:1,entries:1});
 const without=await collectRemoteCacheDirectoryV1({...request,dryRun:true});
 expect(without.locked?undefined:without.value.removed.dangling,"without the host port every result looks dangling, so the command always passes it").toBe(2);
 await collectRemoteCacheDirectoryV1({...request,dryRun:false,hasBlobs:hashes=>r.host.has(hashes)});
 expect(await restorable(r,"ac","7".repeat(64),[kept])).toBe(true);expect(await r.store.read("main","ac","8".repeat(64))).toBeUndefined();await r.store.close();
});

test("the host store's own eviction (cap, age or partition pressure) cannot take a blob of the synchronized state: current entries pin their blobs, an unrelated blob does not survive",async()=>{
 const{collectCasV1}=await import("../../../../🗄️cas/🧹️collector/🟦️.ts"),r=await rig("evict"),unit=[text("evict-out-1".repeat(30)),text("evict-out-2".repeat(30))],nx=text("evict-archive".repeat(30)),stray=text("evict-stray".repeat(30));
 await action(r,"9".repeat(64),unit,"evict:u");await archive(r,"e1",nx,"evict:n");await blob(r,stray);
 expect(await r.store.pinLive()).toBe(3);
 const outcome=await collectCasV1({root:r.host.root,policy:{...quiet(0),capBytes:1}});
 expect(outcome.status).toBe("collected");expect(outcome.evicted).toBeGreaterThan(0);
 expect(await restorable(r,"ac","9".repeat(64),unit)).toBe(true);expect(await restorable(r,"nx","e1",[nx])).toBe(true);expect(gone(r,stray)).toBe(true);
 await r.store.close();
 const after=await collectCasV1({root:r.host.root,policy:{...quiet(0),capBytes:1}});expect(after.evicted,"the pin ends with the server").toBeGreaterThan(0);expect(gone(r,unit[0]!)).toBe(true);
});
