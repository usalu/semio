import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {join} from "node:path";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {cargoScratchBaseV1} from "../../../🧪️tests/🏖️scratch/🟦️.ts";
import {recordLookupV1,type UnitLookupV1} from "../../../💾️store/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1} from "../../📡️client/🟦️.ts";
import {readBuildCountersV1} from "../../📊️counters/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import {safeKeyValueV1,collectKeySamplesV1,inCommonKeySampleV1,compareKeySamplesV1,isKeySamplesV1,publishKeySamplesV1,readKeyComparisonV1,readKeySamplesRefV1,type KeySamplesV1} from "../🟦️.ts";

/** 🔑️ Key samples are digests of the most recent misses; a difference between origins names the component. */
const base=cargoScratchBaseV1(),scratch:string[]=[],servers:RemoteCacheServerV1[]=[];
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(base,"ks-"+label+"-")));scratch.push(path);return path;};
afterAll(async()=>{for(const server of servers)await server.stop().catch(()=>{});for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const slot=(text:string)=>createHash("sha256").update(text).digest("hex"),digest=(text:string)=>createHash("sha256").update(text).digest("hex").slice(0,12);
const row=(crate:string,atMs:number,components:Record<string,string>,outcome:UnitLookupV1["outcome"]="miss",slotId=slot(crate)):UnitLookupV1=>({version:1,atMs,lane:"l",pid:1,crate,key:"k"+atMs,outcome,slot:slotId,components});
const validator=()=>{const ajv=new Ajv({strict:false});ajv.addSchema(schema);return ajv.getSchema(`${schema.$id}#/definitions/KeySamples`)!;};

test("the common sample is the same set of crates on every machine in one window and changes with the window",()=>{
 const names=Array.from({length:400},(_,index)=>"crate"+index),now=5*1800000+7,picked=names.filter(name=>inCommonKeySampleV1(name,now));
 expect(picked.length).toBeGreaterThan(20);expect(picked.length).toBeLessThan(120);
 expect(names.filter(name=>inCommonKeySampleV1(name,now+1000))).toEqual(picked);
 expect(names.filter(name=>inCommonKeySampleV1(name,now+1800000))).not.toEqual(picked);
 const hit=picked[0]!,other=names.find(name=>!inCommonKeySampleV1(name,now))!;
 const document=collectKeySamplesV1([row(hit,1,{source:"aaaa"},"local-hit"),row(other,2,{source:"bbbb"},"local-hit")],now);
 expect(document.samples.map(sample=>sample.crate),"a hit of a common crate is sampled, a hit of another crate is not").toEqual([hit]);
});

test("the newest miss of every unit is kept, hits and units without a slot are not, the most recent ones win, and the document is stable and validates",()=>{
 const rows=[row("a",1,{source:"aaaa"}),row("a",5,{source:"bbbb",toolchain:"c3c3"}),row("b",3,{source:"d7d7"},"local-hit"),row("c",4,{source:"e8e8"}),{...row("d",6,{source:"f9f9"}),slot:undefined} as UnitLookupV1,row("e",2,{source:"1111"})];
 const document=collectKeySamplesV1(rows,100,2);
 expect(document.samples.map(sample=>[sample.crate,sample.atMs])).toEqual([["a",5],["c",4]]);
 expect(document.samples[0]!.components).toEqual({source:"bbbb",toolchain:"c3c3"});
 expect(validator()(document)).toBe(true);expect(isKeySamplesV1(JSON.parse(JSON.stringify(document)))).toBe(true);
 expect(isKeySamplesV1({...document,samples:[{...document.samples[0]!,components:{source:"/Users/me/secret"}}]}),"a path is no digest").toBe(false);
 expect(validator()({...document,samples:[{...document.samples[0]!,components:{source:"/Users/me/secret"}}]})).toBe(false);
 expect(collectKeySamplesV1(rows,200,2).samples).toEqual(document.samples);
});

test("a unit seen by two origins under different keys names the differing components, and says whether the source was the same",()=>{
 const doc=(rows:UnitLookupV1[]):KeySamplesV1=>collectKeySamplesV1(rows,1);
 const base={source:"aaaa",toolchain:"a1a1","env:CARGO_PKG_NAME":"a2a2","extern:core":"c3c3","args.metadata":"a4a4"};
 const origins={
  m1:doc([row("alpha",1,base),row("beta",1,{...base,source:"1111"}),row("solo",1,base)]),
  m2:doc([row("alpha",2,{...base,"args.metadata":"a5a5"}),row("beta",2,{...base,source:"2222","extern:core":"a6a6"}),row("same",2,base)]),
  m3:doc([row("alpha",3,{...base,"args.metadata":"a5a5"}),row("same",3,base)]),
 };
 const result=compareKeySamplesV1(origins);
 expect(result.origins).toBe(3);expect(result.units).toBe(3);
 expect(result.diffs.map(diff=>[diff.crate,diff.origins,diff.differs,diff.sameSource])).toEqual([["alpha",["m1","m2","m3"],["args.metadata"],true],["beta",["m1","m2"],["extern:core","source"],false]]);
 expect(result.byComponent).toEqual({"args.metadata":1,"extern:*":1,source:1});
 const naive=(crate:string)=>{const sets=Object.values(origins).flatMap(document=>document.samples.filter(sample=>sample.crate===crate));return[...new Set(sets.flatMap(sample=>Object.keys(sample.components)))].filter(name=>new Set(sets.map(sample=>sample.components[name])).size>1).sort();};
 expect(result.diffs.map(diff=>diff.differs)).toEqual([naive("alpha"),naive("beta")]);
});

test("an origin publishes its sample once as a blob of the fleet cache, republishes only when a component changed, and the counters carry the reference",async()=>{
 const server=await startRemoteCacheServerV1({directory:directory("srv"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});servers.push(server);
 const root=directory("root"),workspace=directory("work"),environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token"};
 const client=createFleetClientV1(readFleetSettingsV1(root,environment)!);
 recordLookupV1(root,{lane:"l",crate:"alpha",key:"k1",outcome:"miss",slot:slot("alpha"),components:{source:"aaaa",toolchain:"a1a1"}});
 const first=await publishKeySamplesV1(client,{root,workspace});
 expect(first?.count).toBe(1);expect(readKeySamplesRefV1(workspace)).toEqual(first);expect(readBuildCountersV1(workspace).keySamples).toEqual(first);
 const response=await fetch(server.url+"/main/cas/"+first!.blob,{headers:{authorization:"Bearer w-token"}}),bytes=new Uint8Array(await response.arrayBuffer());expect(response.status).toBe(200);
 const document=JSON.parse(new TextDecoder().decode(bytes));expect(validator()(document)).toBe(true);expect(createHash("sha256").update(bytes).digest("hex")).toBe(first!.blob);
 expect(await publishKeySamplesV1(client,{root,workspace}),"nothing changed: the same reference").toEqual(first);
 recordLookupV1(root,{lane:"l",crate:"alpha",key:"k2",outcome:"miss",slot:slot("alpha"),components:{source:"bbbb",toolchain:"a1a1"}});
 const second=await publishKeySamplesV1(client,{root,workspace});expect(second?.blob).not.toBe(first!.blob);
 const readOnly=createFleetClientV1(readFleetSettingsV1(root,{...environment,SEMIO_REMOTE_CACHE_ACCESS:"read-only"})!);
 expect(await publishKeySamplesV1(readOnly,{root,workspace:directory("work2")}),"a read-only machine publishes nothing").toBeUndefined();
});

test("the comparison reads the blobs the origins published, leaves out a missing, altered or malformed one, and compares the rest",async()=>{
 const make=(rows:UnitLookupV1[])=>{const text=JSON.stringify(collectKeySamplesV1(rows,1)),blob=createHash("sha256").update(text).digest("hex");return{text,ref:{blob,count:1,atMs:1}};};
 const one=make([row("alpha",1,{source:"aaaa",platform:"a1a1"})]),two=make([row("alpha",2,{source:"aaaa",platform:"b2b2"})]),lost=make([row("alpha",3,{source:"aaaa"})]),forged=make([row("alpha",4,{source:"aaaa"})]);
 const store=new Map<string,string>([[one.ref.blob,one.text],[two.ref.blob,two.text],[forged.ref.blob,'{"version":1,"atMs":1,"samples":[]}']]);
 const result=await readKeyComparisonV1({m1:one.ref,m2:two.ref,m3:lost.ref,m4:forged.ref},async blob=>store.has(blob)?new TextEncoder().encode(store.get(blob)):undefined);
 expect(result?.origins).toBe(2);expect(result?.diffs.map(diff=>[diff.crate,diff.differs,diff.sameSource])).toEqual([["alpha",["platform"],true]]);
 expect(await readKeyComparisonV1({m3:lost.ref},async()=>undefined)).toBeUndefined();
});

test("flag texts are published without any directory, differing directories stay distinguishable, and the comparison shows the texts of the differing components",()=>{
 const flags="-Z=threads=0 --remap-path-prefix=/Users/alice/.cargo=/cargo --diagnostic-width=100";
 const safe=safeKeyValueV1(flags),other=safeKeyValueV1(flags.replace("alice","bob"));
 expect(safe).not.toMatch(/[\\/]/u);expect(safe).toContain("-Z=threads=0");expect(safe).toContain("--diagnostic-width=100");expect(other).not.toBe(safe);expect(safeKeyValueV1("x /a/b y")).toMatch(/^x <path#[0-9a-f]{8}> y$/u);
 const withValues=(crate:string,at:number,text:string):UnitLookupV1=>({...row(crate,at,{source:"aaaa","args.diagnostics":text.length.toString(16).padStart(4,"0")}),values:{"args.diagnostics":text,"args.remap":"/Users/alice/x"}});
 const one=collectKeySamplesV1([withValues("alpha",1,"--json=a")],1),two=collectKeySamplesV1([withValues("alpha",2,"--json=a,b")],1);
 expect(one.samples[0]!.values).toEqual({"args.diagnostics":"--json=a"});expect(validator()(one)).toBe(true);
 expect(isKeySamplesV1({...one,samples:[{...one.samples[0]!,values:{"args.diagnostics":"/Users/me"}}]})).toBe(false);
 const result=compareKeySamplesV1({m1:one,m2:two});
 expect(result.diffs[0]!.values).toEqual({"args.diagnostics":{m1:"--json=a",m2:"--json=a,b"}});
});
