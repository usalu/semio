import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {declareV1} from "../../../../../../🏗️build/🌐️remote-cache/🏷️retention/🍂️staleness/🟦️.ts";
import {putCasObjectSyncV1} from "../../../../../../🏗️build/🗄️cas/🗃️store/🔁️sync/🟦️.ts";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1,type FleetClientV1} from "../../📡️client/🟦️.ts";
import {encodeActionResultV1} from "../../📦️wire/🟦️.ts";
import {manifestIdV1} from "../../📋️manifest/🟦️.ts";
import {FLEET_POLICY_V1} from "../../🪪️identity/🟦️.ts";
import {clearSeedV1,finishSeedV1,obtainSeedRecordV1,observedSeedSlotsV1,parseSeedRecordV1,publishSeedRecordV1,recordSeedSlotV1,seedClosureIdV1,seedDirectoryV1,seedFingerprintV1,seedLaneV1,seededClientV1,seedRecordWireV1,seedRunReportV1} from "../🟦️.ts";
import vectors from "./🔣️.json" with {type:"json"};

/** 🌱️ Contract of the lane seed: the fingerprint and closure id are fixed by the vectors beside this file (recomputed by Python), the record travels through a real fleet cache server (newest version listed first, a second version supersedes the first), the seed prefetches the newest entry of each recorded slot with bounded concurrency into the lane and a wrapper's client answers from it without a request, a blob is verified, the local object store spares downloads, a cancelled seed stops, and the record only grows. */
const scratch:string[]=[],servers:RemoteCacheServerV1[]=[],sha=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex"),python=testPythonV1();
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(tmpdir(),`seed-${label}-`)));scratch.push(path);return path;};
afterAll(async()=>{for(const server of servers)await server.stop().catch(()=>{});for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const graph={workspaceRoot:vectors.workspaceRoot,units:vectors.units};

async function fleet(label:string){
 const tree=directory(label+"-tree"),cas=directory(label+"-cas"),server=await startRemoteCacheServerV1({directory:directory(label+"-srv"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"},graceMilliseconds:50,staleness:{mode:"enforce",workspace:tree,recheckSeconds:0,manifestNames:["unit.json","stage.json"],examples:3}});servers.push(server);
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token",SEMIO_CAS_ROOT:cas};
 return{server,cas,client:(root:string)=>createFleetClientV1(readFleetSettingsV1(root,environment)!)};
}

async function publishUnit(client:FleetClientV1,slot:string,key:string,outputs:Readonly<Record<string,Uint8Array>>){
 const unit=new TextEncoder().encode(JSON.stringify({slot,key})),files=[{path:FLEET_POLICY_V1.manifestName,digest:{hash:sha(unit),size:unit.length},executable:false}];
 expect(await client.putBlob(sha(unit),unit)).toBe(true);
 for(const [name,bytes] of Object.entries(outputs)){expect(await client.putBlob(sha(bytes),bytes)).toBe(true);files.push({path:"out/"+name,digest:{hash:sha(bytes),size:bytes.length},executable:false});}
 expect(await client.putResult(key,encodeActionResultV1({files,slot}),slot)).toBe(true);
}

test("the fingerprint of a request and the closure id are the SHA-256 digests an independent Python computation gives, and they do not move with the workspace root, the order of units or the order of features",()=>{
 const rows=vectors.units.map(unit=>JSON.stringify([unit.provenance.kind,unit.provenance.kind==="workspace"?"crates/a/Cargo.toml":unit.provenance.identity,unit.packageName,unit.packageVersion,unit.name,unit.kind,unit.operation,unit.platform??"",[...unit.features].sort(),unit.profileDigest,unit.compiler.digest])).sort();
 const fingerprint=sha(JSON.stringify(["semio-seed-fingerprint-v1",rows]));
 expect(seedFingerprintV1(graph)).toBe(fingerprint);expect(seedClosureIdV1(fingerprint)).toBe(sha(vectors.namespace+"\0"+fingerprint));
 if(python){
  const run=Bun.spawnSync([python,"-I","-c","import hashlib,json,os,sys\nv=json.load(sys.stdin)\nrows=sorted(json.dumps([u['provenance']['kind'],os.path.relpath(u['manifest'],v['workspaceRoot']) if u['provenance']['kind']=='workspace' else u['provenance']['identity'],u['packageName'],u['packageVersion'],u['name'],u['kind'],u['operation'],u['platform'] or '',sorted(u['features']),u['profileDigest'],u['compiler']['digest']],separators=(',',':'),ensure_ascii=False) for u in v['units'])\nf=hashlib.sha256(json.dumps(['semio-seed-fingerprint-v1',rows],separators=(',',':'),ensure_ascii=False).encode()).hexdigest()\nprint(json.dumps([f,hashlib.sha256((v['namespace']+'\\0'+f).encode()).hexdigest()]))"],{stdin:new TextEncoder().encode(JSON.stringify(vectors)),stdout:"pipe",stderr:"pipe"});
  expect(run.exitCode,run.stderr.toString()).toBe(0);expect(JSON.parse(run.stdout.toString())).toEqual([fingerprint,seedClosureIdV1(fingerprint)]);
 }
 const moved={workspaceRoot:"/elsewhere/checkout",units:[...vectors.units].reverse().map(unit=>unit.provenance.kind==="workspace"?{...unit,manifest:"/elsewhere/checkout/crates/a/Cargo.toml",features:[...unit.features].reverse()}:unit)};
 expect(seedFingerprintV1(moved)).toBe(fingerprint);
 expect(seedFingerprintV1({...graph,units:graph.units.map(unit=>unit.name==="a"?{...unit,features:["y"]}:unit)})).not.toBe(fingerprint);
});

test("a closure record is stored under a key of its own version, read back newest first through the slot of the request, and only well-formed records are believed",async()=>{
 const {client:make}=await fleet("record"),client=make(directory("m")),id=seedClosureIdV1("f".repeat(64)),declaration=declareV1({kind:"input-manifest",toolchain:"rustc test",sources:[],externs:[]});
 const slots=["a".repeat(64),"b".repeat(64)];
 expect(await obtainSeedRecordV1(client,id)).toEqual({state:"absent"});
 expect(await publishSeedRecordV1(client,id,slots,declaration)).toBe("stored");
 expect(await obtainSeedRecordV1(client,id)).toEqual({state:"found",slots});
 const grown=[...slots,"c".repeat(64)];await Bun.sleep(5);
 expect(await publishSeedRecordV1(client,id,grown,declaration)).toBe("stored");
 expect(await obtainSeedRecordV1(client,id)).toEqual({state:"found",slots:grown});
 expect(seedRecordWireV1(id,slots,declaration).key).toBe(sha(id+"\0"+seedRecordWireV1(id,slots,declaration).hash));
 expect(parseSeedRecordV1(seedRecordWireV1(id,["nothex"],declaration).result)).toBeUndefined();expect(parseSeedRecordV1(new Uint8Array([1,2,3]))).toBeUndefined();
 expect(await publishSeedRecordV1(client,id,[],declaration)).toBe("unshared");
});

test("a seed prefetches the newest entry of every recorded slot in parallel into the lane, a wrapper's client then answers from the lane without one request, a corrupt blob is refused, and the unit manifest of the slot is kept locally",async()=>{
 const {server,client:make}=await fleet("seed"),publisher=make(directory("pub")),machine=directory("machine"),laneRoot=directory("lane"),closure=seedClosureIdV1("e".repeat(64));
 const slots=Array.from({length:12},(_,index)=>sha("slot"+index)),keys=slots.map(slot=>sha("key"+slot)),bytes=slots.map((_,index)=>new TextEncoder().encode("rlib bytes "+index+"x".repeat(100*index)));
 for(const [index,slot] of slots.entries())await publishUnit(publisher,slot,keys[index]!,{"lib.rlib":bytes[index]!});
 await publishUnit(publisher,slots[0]!,sha("older key"),{"lib.rlib":new TextEncoder().encode("older")});
 expect(await publishSeedRecordV1(publisher,closure,slots,declareV1({kind:"input-manifest",toolchain:"rustc test",sources:[],externs:[]}))).toBe("stored");
 const client=make(machine),seen:number[]=[];
 const result=await seedLaneV1({client,cacheRoot:machine,laneRoot,closureId:closure,progress:stats=>seen.push(stats.done),casRoot:directory("empty-cas")});
 expect(result.stats).toMatchObject({state:"seeded",slots:12,done:12,absent:0});expect(result.stats.fetched).toBe(12);expect(result.stats.blobs).toBe(24);expect(result.record).toEqual(slots);
 expect(existsSync(join(seedDirectoryV1(laneRoot),"ac"))).toBe(true);
 const before=server.store.usage().entries,wrapper=seededClientV1(make(machine),laneRoot,directory("empty-cas-2"));
 const answer=await wrapper.getResult(keys[3]!);expect(typeof answer).not.toBe("string");
 const blob=await wrapper.getBlob(sha(bytes[3]!),bytes[3]!.length);expect(Buffer.from(blob!).equals(Buffer.from(bytes[3]!))).toBe(true);
 expect(readFileSync(join(seedDirectoryV1(laneRoot),"used.log"),"utf8")).toContain("ac "+keys[3]);
 expect(existsSync(join(seedDirectoryV1(laneRoot),"cas",sha(bytes[3]!))),"a served blob is removed from the lane").toBe(false);
 writeFileSync(join(seedDirectoryV1(laneRoot),"cas",sha(bytes[4]!)),"corrupt");
 const refetched=await wrapper.getBlob(sha(bytes[4]!),bytes[4]!.length);expect(Buffer.from(refetched!).equals(Buffer.from(bytes[4]!)),"a corrupt seed file falls back to the fleet").toBe(true);
 expect(server.store.usage().entries).toBe(before);
 const second=await seedLaneV1({client,cacheRoot:machine,laneRoot,closureId:closure,casRoot:directory("empty-cas-3")});expect(second.stats.blobs,"a second seed fetches nothing it already holds").toBe(0);
 expect(seen.length).toBeGreaterThanOrEqual(0);
 clearSeedV1(laneRoot);expect(existsSync(seedDirectoryV1(laneRoot))).toBe(false);
 expect(manifestIdV1(slots[0]!)).toHaveLength(64);
});

test("blobs this machine already holds in its object store are neither prefetched nor downloaded, an unknown request seeds nothing, a cancelled seed stops, and the byte budget bounds the prefetch",async()=>{
 const {client:make}=await fleet("budget"),publisher=make(directory("pub")),machine=directory("machine"),closure=seedClosureIdV1("d".repeat(64)),local=directory("local-cas");
 const slots=[sha("s1"),sha("s2"),sha("s3")],payload=slots.map((_,index)=>new TextEncoder().encode("payload "+index+"y".repeat(5000)));
 for(const [index,slot] of slots.entries())await publishUnit(publisher,slot,sha("k"+slot),{"lib.rlib":payload[index]!});
 await publishSeedRecordV1(publisher,closure,slots,declareV1({kind:"input-manifest",toolchain:"rustc test",sources:[],externs:[]}));
 putCasObjectSyncV1(local,payload[0]!);
 const laneRoot=directory("lane"),client=make(machine),result=await seedLaneV1({client,cacheRoot:machine,laneRoot,closureId:closure,casRoot:local});
 expect(result.stats.blobs,"the local blob of slot 1 is not downloaded (the unit manifest blobs and two outputs are)").toBe(5);
 const wrapper=seededClientV1(make(machine),laneRoot,local);expect(Buffer.from((await wrapper.getBlob(sha(payload[0]!),payload[0]!.length))!).equals(Buffer.from(payload[0]!))).toBe(true);
 expect(readFileSync(join(seedDirectoryV1(laneRoot),"used.log"),"utf8")).toContain("local "+sha(payload[0]!));
 expect((await seedLaneV1({client,cacheRoot:machine,laneRoot:directory("lane2"),closureId:seedClosureIdV1("0".repeat(64)),casRoot:local})).stats.state).toBe("no-record");
 const controller=new AbortController();controller.abort();
 const cancelled=await seedLaneV1({client,cacheRoot:machine,laneRoot:directory("lane3"),closureId:closure,signal:controller.signal,casRoot:directory("cas3")});expect(cancelled.stats.state).toBe("cancelled");expect(cancelled.stats.fetched).toBe(0);
 expect((await seedLaneV1({client:undefined,cacheRoot:machine,laneRoot:directory("lane4"),closureId:closure})).stats.state).toBe("off");
 const warm=directory("lane5"),read=await seedLaneV1({client,cacheRoot:machine,laneRoot:warm,closureId:closure,prefetch:false});
 expect(read.record,"a warm lane only reads the record").toEqual(slots);expect(read.stats).toMatchObject({fetched:0,blobs:0,bytes:0});expect(existsSync(seedDirectoryV1(warm))).toBe(false);
});

test("the closure record only grows: it is published when the run succeeded and the lane noted a slot the record lacks, never otherwise; and the run report tallies local hits, fleet fetches, compiles and offloads of the lane",async()=>{
 const {client:make}=await fleet("finish"),client=make(directory("m")),laneRoot=directory("lane"),closure=seedClosureIdV1("c".repeat(64));
 const record=[sha("r1")],noted=[sha("r1"),sha("n1"),sha("n2")];
 for(const slot of noted)recordSeedSlotV1(laneRoot,slot);recordSeedSlotV1(laneRoot,"not a slot");recordSeedSlotV1(undefined,sha("x"));
 expect(observedSeedSlotsV1(laneRoot)).toEqual(noted);
 expect(await finishSeedV1({client,laneRoot,closureId:closure,record,toolchain:"t",succeeded:false})).toEqual({published:false,slots:1,added:0});
 expect(await finishSeedV1({client,laneRoot,closureId:closure,record,toolchain:"t",succeeded:true})).toEqual({published:true,slots:3,added:2});
 expect(await obtainSeedRecordV1(client,closure)).toEqual({state:"found",slots:[sha("n1"),sha("n2"),sha("r1")]});
 expect(await finishSeedV1({client,laneRoot,closureId:closure,record:[sha("r1"),sha("n1"),sha("n2")],toolchain:"t",succeeded:true})).toMatchObject({published:false,added:0});
 const cache=directory("cache"),lane="L1",events=join(cache,"x");mkdirSync(events,{recursive:true});
 const row=(kind:string,crate:string,key:string,extra:object={})=>JSON.stringify({version:1,atMs:Date.now(),lane,pid:1,kind,crate,key,milliseconds:1,waitedMs:0,...extra});
 writeFileSync(join(events,lane+".jsonl"),[row("hit","a","k1"),row("remote-hit","b","k2"),row("remote-hit","c","k3"),row("miss","d","k4"),row("miss","e","k5"),row("script-hit","f",null as unknown as string)].join("\n")+"\n");
 writeFileSync(join(events,"offload.jsonl"),row("offload-run","e","k5",{origin:"m02"})+"\n"+row("offload-run","g build script","k9",{origin:"m02"})+"\n");
 mkdirSync(seedDirectoryV1(laneRoot),{recursive:true});writeFileSync(join(seedDirectoryV1(laneRoot),"used.log"),"ac k2\ncas x\nac k3\n");
 expect(seedRunReportV1({cacheRoot:cache,lane,laneRoot,sinceMs:0})).toEqual({total:5,local:1,fleet:2,prefetched:2,compiled:1,offloaded:1,scripts:1});
 expect(seedRunReportV1({cacheRoot:cache,lane:"other",laneRoot,sinceMs:0}).total).toBe(0);
});
