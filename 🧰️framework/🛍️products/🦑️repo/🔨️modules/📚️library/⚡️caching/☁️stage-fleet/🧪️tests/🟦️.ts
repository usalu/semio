import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {sha256} from "@noble/hashes/sha2.js";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {slotIdV1} from "../../../🏗️build/🪢️slots/🟦️.ts";
import {runToolSyncV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import {stageCacheV1,stageKeyDigestV1,type StageKeyV1} from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🗃️stage-cache/🟦️.ts";
import {cargoScratchBaseV1} from "../../🦀️cargo/🗃️unit-cache/🧪️tests/🏖️scratch/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1,type FleetClientV1} from "../../🦀️cargo/🗃️unit-cache/🛰️fleet/📡️client/🟦️.ts";
import {obtainFleetEntryV1} from "../🎟️flight/🟦️.ts";
import {STAGE_FLEET_POLICY_V1,settleStageSlotV1,sharedStageCacheV1,stageFleetClientV1,stageFleetDecoratorV1,stageFleetEventTextV1,stageFleetTextV1,stageRemoteKeyV1,stageSlotIdV1,type StageFleetEventV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

/** ☁️ The stage cache's fleet tier is proven on one machine: scratch roots at different paths act as machines against a cache server the test starts on a free port with a temporary store and test tokens. The first machine computes and shares, the second computes nothing and gets the same bytes, racing machines compute once, and a cache that is down, denies, stalls, serves corrupt blobs or has a dead claim holder never fails or hangs a stage. */
const base=cargoScratchBaseV1(),scratch:string[]=[],servers:RemoteCacheServerV1[]=[],stops:Array<()=>void>=[];
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(base,"sf-"+label+"-")));scratch.push(path);return path;};
afterAll(async()=>{for(const stop of stops)stop();for(const server of servers)await server.stop().catch(()=>{});if(!process.env.SEMIO_TEST_KEEP)for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const policy=STAGE_FLEET_POLICY_V1,noble=(text:string)=>Buffer.from(sha256(new TextEncoder().encode(text))).toString("hex"),token="test-write-token-not-a-secret";

async function serve():Promise<RemoteCacheServerV1>{
 const server=await startRemoteCacheServerV1({directory:directory("srv"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:token}});
 servers.push(server);return server;
}
const environment=(url:string,secret=token,access="read-write")=>({SEMIO_REMOTE_CACHE_ENDPOINT:url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:access,SEMIO_REMOTE_CACHE_TOKEN:secret});
type Machine=Readonly<{root:string;stages:string;client:FleetClientV1|undefined;cache:ReturnType<typeof sharedStageCacheV1>;events:StageFleetEventV1[]}>;
function machine(name:string,url:string|undefined,options:{secret?:string;access?:string;fetch?:typeof fetch;now?:()=>number;claim?:Parameters<typeof sharedStageCacheV1>[1] extends infer O?O extends {claim?:infer C}?C:never:never}={}):Machine{
 const root=directory(name),stages=join(root,"stages"),events:StageFleetEventV1[]=[];
 const client=url===undefined?undefined:createFleetClientV1(readFleetSettingsV1(join(root,"state"),environment(url,options.secret,options.access))!,options.fetch===undefined?{}:{fetch:options.fetch});
 return {root,stages,client,events,cache:sharedStageCacheV1(stageCacheV1(stages),{client,report:event=>events.push(event),...(options.now?{now:options.now}:{}),...(options.claim?{claim:options.claim}:{})})};
}
const keyOf=(stage:string,input:string,tool="t1"):StageKeyV1=>({stage,schemaVersion:1,inputs:{component:noble(input)},tools:{tool}});
const payload=(seed:string)=>[["a.js",`export const seed="${seed}";\n`],["interfaces/b.d.ts",`// ${seed}\n`],["core.wasm",seed.repeat(5000)],["empty.txt",""]] as const;
function producer(seed:string,state:{calls:number},delay=0){
 return async(target:string)=>{state.calls++;if(delay)await new Promise(done=>setTimeout(done,delay));for(const [path,text] of payload(seed)){mkdirSync(dirname(join(target,path)),{recursive:true});writeFileSync(join(target,path),text);}};
}
const tree=(root:string):Record<string,string>=>{const rows:Record<string,string>={};const walk=(prefix:string)=>{for(const entry of readdirSync(join(root,prefix),{withFileTypes:true})){const path=prefix?`${prefix}/${entry.name}`:entry.name;if(entry.isDirectory())walk(path);else rows[path]=noble(readFileSync(join(root,path),"utf8"));}};walk("");return rows;};

test("the schema is closed: the policy, key vectors and slot index validate, unknown fields are refused, both languages carry the same placeholders",()=>{
 const ajv=new Ajv({strict:false});ajv.addSchema(schema);
 const validator=(name:string)=>{const validate=ajv.getSchema(`${schema.$id}#/definitions/${name}`);if(!validate)throw Error(name);return validate;};
 for(const row of fixture.vectors)expect(validator("KeyVector")(row),JSON.stringify(validator("KeyVector").errors)).toBe(true);
 expect(validator("KeyVector")({...fixture.vectors[0],extra:1})).toBe(false);
 expect(validator("KeyVector")({...fixture.vectors[0],remoteKey:"xyz"})).toBe(false);
 expect(validator("SlotIndex")({version:1,rows:[{digest:"a".repeat(64),storedAt:1}]})).toBe(true);
 expect(validator("SlotIndex")({version:1,rows:[{digest:"a".repeat(64)}]})).toBe(false);
 for(const [name,texts] of Object.entries(schema.definitions.Messages.const)){
  const holes=(text:string)=>[...text.matchAll(/\{(\w+)\}/gu)].map(match=>match[1]).sort();
  expect(texts.en.length,name).toBeGreaterThan(10);expect(texts.de.length,name).toBeGreaterThan(10);expect(holes(texts.de),name).toEqual(holes(texts.en));
 }
 expect(stageFleetTextV1("de","served",{stage:"transpile",slot:"m",files:3,bytes:9})).toContain("Flotten-Cache");
 expect(stageFleetTextV1("en","served",{stage:"transpile",slot:"m",files:3,bytes:9})).toContain("3 files");
});

test("remote keys and slot ids equal the fixture vectors, Noble SHA-256 and an independent Python derivation, and move with each input",()=>{
 for(const row of fixture.vectors){
  const text=JSON.stringify([policy.namespace,"stage",row.platform,row.digest]);
  expect(stageRemoteKeyV1(row.digest,row.platform)).toBe(row.remoteKey);expect(noble(text)).toBe(row.remoteKey);
  expect(stageSlotIdV1(row.stage,row.schemaVersion,row.slot)).toBe(row.slotId);expect(slotIdV1("stage-output",[row.stage,String(row.schemaVersion),row.slot])).toBe(row.slotId);
 }
 const run=(()=>{try{return runToolSyncV1("python",["-I","-c",`import hashlib,json,sys\nrows=json.load(sys.stdin)\nout=[]\nfor r in rows:\n    j=lambda v: json.dumps(v,separators=(",",":"),ensure_ascii=False)\n    out.append([hashlib.sha256(j([r["ns"],"stage",r["platform"],r["digest"]]).encode()).hexdigest(),hashlib.sha256(j(["semio-slot-v1","stage-output",r["stage"],str(r["schemaVersion"]),r["slot"]]).encode()).hexdigest()])\nprint(json.dumps(out))`],{encoding:"utf8",input:JSON.stringify(fixture.vectors.map(row=>({...row,ns:policy.namespace})))});}catch{return undefined;}})();
 if(run&&run.status===0)expect(JSON.parse(run.stdout)).toEqual(fixture.vectors.map(row=>[row.remoteKey,row.slotId]));
 else console.log("[stage-fleet] no Python found: the independent derivation was not run");
 const first=fixture.vectors[0]!;
 expect(stageRemoteKeyV1(first.digest,"linux-x64")).not.toBe(first.remoteKey);expect(stageRemoteKeyV1("d".repeat(64),first.platform)).not.toBe(first.remoteKey);
 expect(stageSlotIdV1("optimize",1,first.slot)).not.toBe(first.slotId);expect(stageSlotIdV1(first.stage,2,first.slot)).not.toBe(first.slotId);expect(stageSlotIdV1(first.stage,first.schemaVersion,"other")).not.toBe(first.slotId);
 expect(stageRemoteKeyV1(first.digest)).toBe(stageRemoteKeyV1(first.digest,`${process.platform}-${process.arch}`));
});

test("two machines at different paths: the first computes and shares, the second computes nothing and gets the same bytes",async()=>{
 const server=await serve(),one=machine("m1",server.url),two=machine("m2",server.url),key=keyOf("transpile","component-1"),state={calls:0};
 const first=await one.cache.run(key,join(one.root,"out"),producer("alpha",state),undefined,"mod");
 expect([first.hit,state.calls]).toEqual([false,1]);
 expect(one.events.map(event=>event.kind)).toEqual(["shared"]);expect(one.events[0]).toMatchObject({stage:"transpile",slot:"mod",files:4});
 const second=await two.cache.run(key,join(two.root,"out"),producer("alpha",state),undefined,"mod");
 expect([second.hit,state.calls]).toEqual([true,1]);
 expect(tree(join(two.root,"out"))).toEqual(tree(join(one.root,"out")));expect(second.files).toEqual(first.files);
 expect(two.events.map(event=>event.kind)).toEqual(["served"]);expect(two.cache.fleet()).toEqual({served:1,shared:0,waited:0,skipped:0});expect(two.cache.stats()).toMatchObject({hits:0,fleetHits:1,misses:0});
 const third=await two.cache.run(key,join(two.root,"again"),producer("alpha",state),undefined,"mod");
 expect([third.hit,state.calls,two.events.length]).toEqual([true,1,1]);
 expect(two.cache.stats()).toMatchObject({hits:1,fleetHits:1,misses:0});
 const changed=await two.cache.run(keyOf("transpile","component-1","t2"),join(two.root,"changed"),producer("beta",state),undefined,"mod");
 expect([changed.hit,state.calls]).toEqual([false,2]);
 const status=await (await fetch(`${server.url}/status`,{headers:{authorization:`Bearer ${token}`}})).json() as {entries:number};
 expect(status.entries).toBeGreaterThanOrEqual(2);
});

test("a platform that differs does not see the entry; a request without a slot never touches the fleet",async()=>{
 const server=await serve(),one=machine("p1",server.url),key=keyOf("optimize","core-1"),state={calls:0};
 await one.cache.run(key,join(one.root,"out"),producer("gamma",state),undefined,"mod/core.wasm");
 const other=machine("p2",server.url),shifted=sharedStageCacheV1(stageCacheV1(other.stages),{client:other.client!,platform:"linux-x64"});
 const result=await shifted.run(key,join(other.root,"out"),producer("gamma",state),undefined,"mod/core.wasm");
 expect([result.hit,state.calls]).toEqual([false,2]);
 const bare=machine("p3",server.url);
 await bare.cache.run(keyOf("optimize","core-2"),join(bare.root,"out"),producer("delta",state));
 expect([state.calls,bare.events.length,bare.cache.fleet()]).toEqual([3,0,{served:0,shared:0,waited:0,skipped:0}]);
});

test("machines racing for one key compute it exactly once; the others wait for the upload and receive it",async()=>{
 const server=await serve(),machines=[machine("r1",server.url),machine("r2",server.url),machine("r3",server.url)],key=keyOf("probe","component-race"),state={calls:0};
 const results=await Promise.all(machines.map((m,index)=>new Promise<void>(done=>setTimeout(done,index*30)).then(()=>m.cache.run(key,join(m.root,"out"),producer("race",state,700),undefined,"mod"))));
 expect(state.calls).toBe(1);expect(results.filter(result=>result.hit).length).toBe(2);
 const outputs=machines.map(m=>tree(join(m.root,"out")));expect(outputs[1]).toEqual(outputs[0]!);expect(outputs[2]).toEqual(outputs[0]!);
 expect(machines.flatMap(m=>m.events).filter(event=>event.kind==="waited").length).toBe(2);
},60000);

test("a claim whose holder died lapses and the next machine computes",async()=>{
 const server=await serve(),ghost=createFleetClientV1(readFleetSettingsV1(join(directory("ghost"),"s"),environment(server.url))!),key=keyOf("describe","component-dead"),digest=stageKeyDigestV1(key),remote=stageRemoteKeyV1(digest);
 expect((await ghost.claim(remote,"ghost:1:dead",1000,5000)).state).toBe("granted");
 const m=machine("d1",server.url),state={calls:0},started=Date.now();
 const result=await m.cache.run(key,join(m.root,"out"),producer("late",state),undefined,"mod");
 expect([result.hit,state.calls]).toEqual([false,1]);expect(Date.now()-started).toBeLessThan(20000);expect(m.events.map(event=>event.kind)).toEqual(["shared"]);
 const wait=await ghost.claim(remote,"ghost:2:late",1000);expect(wait.state).toBe("stored");
},60000);

test("a holder that is slower than the bound is overtaken and the stage is computed here",async()=>{
 const server=await serve(),holder=createFleetClientV1(readFleetSettingsV1(join(directory("slowholder"),"s"),environment(server.url))!),key=keyOf("optimize","component-slow"),remote=stageRemoteKeyV1(stageKeyDigestV1(key));
 expect((await holder.claim(remote,"slow:1:aa",60000,200)).state).toBe("granted");
 const m=machine("s1",server.url,{claim:{minimumWaitMilliseconds:300,waitFactor:1,maximumWaitMilliseconds:600}}),state={calls:0};
 const result=await m.cache.run(key,join(m.root,"out"),producer("slow",state),undefined,"mod");
 expect([result.hit,state.calls]).toEqual([false,1]);expect(m.events.map(event=>event.kind)).toEqual(["shared"]);
},60000);

test("a cache that is down, denies, is read-only, stalls or serves corrupt blobs never fails a stage",async()=>{
 const state={calls:0},key=keyOf("transpile","component-faults");
 const down=machine("f-down","http://127.0.0.1:1");
 expect((await down.cache.run(key,join(down.root,"out"),producer("down",state),undefined,"mod")).hit).toBe(false);expect(down.events.map(event=>event.kind)).toEqual(["skipped"]);
 const server=await serve(),denied=machine("f-denied",server.url,{secret:"wrong"});
 expect((await denied.cache.run(key,join(denied.root,"out"),producer("denied",state),undefined,"mod")).hit).toBe(false);expect(denied.events[0]!.kind).toBe("skipped");
 const writer=machine("f-writer",server.url);await writer.cache.run(keyOf("transpile","component-shared"),join(writer.root,"out"),producer("shared",state),undefined,"mod");
 const reader=machine("f-reader",server.url,{access:"read-only"}),before=state.calls;
 expect((await reader.cache.run(keyOf("transpile","component-shared"),join(reader.root,"out"),producer("shared",state),undefined,"mod")).hit).toBe(true);expect(state.calls).toBe(before);
 await reader.cache.run(keyOf("transpile","component-readonly"),join(reader.root,"out2"),producer("ro",state),undefined,"mod");
 expect(reader.events.at(-1)!.kind).toBe("skipped");
 const probe=machine("f-probe",server.url);expect((await probe.cache.run(keyOf("transpile","component-readonly"),join(probe.root,"out"),producer("ro",state),undefined,"mod")).hit).toBe(false);
 const stall=Bun.serve({port:0,fetch:()=>new Promise<Response>(()=>{})});stops.push(()=>stall.stop(true));
 const stalled=machine("f-stall",`http://127.0.0.1:${stall.port}`),started=Date.now();
 expect((await stalled.cache.run(key,join(stalled.root,"out"),producer("stall",state),undefined,"mod")).hit).toBe(false);expect(Date.now()-started).toBeLessThan(15000);expect(stalled.events[0]!.kind).toBe("skipped");
 const corrupt:typeof fetch=(async(input:RequestInfo|URL,init?:RequestInit)=>{const response=await fetch(input,init);if(String(input).includes("/cas/")&&(init?.method??"GET")==="GET"&&response.status===200){const bytes=new Uint8Array(await response.arrayBuffer());bytes[0]=bytes[0]!^255;return new Response(bytes,{status:200});}return response;}) as typeof fetch;
 const broken=machine("f-corrupt",server.url,{fetch:corrupt}),calls=state.calls;
 const result=await broken.cache.run(keyOf("transpile","component-shared"),join(broken.root,"out"),producer("shared",state),undefined,"mod");
 expect([result.hit,state.calls]).toEqual([false,calls+1]);expect(tree(join(broken.root,"out"))).toEqual(tree(join(writer.root,"out")));
},90000);

test("a forged entry under the right key is a miss: a manifest for another key or a file list that disagrees is never installed",async()=>{
 const server=await serve(),one=machine("g1",server.url),state={calls:0},key=keyOf("transpile","component-forge"),other=keyOf("transpile","component-other");
 await one.cache.run(other,join(one.root,"out"),producer("other",state),undefined,"mod");
 const stolen=await one.client!.getResult(stageRemoteKeyV1(stageKeyDigestV1(other)));expect(typeof stolen).toBe("object");
 expect(await one.client!.putResult(stageRemoteKeyV1(stageKeyDigestV1(key)),stolen as Uint8Array,stageSlotIdV1("transpile",1,"mod"))).toBe(true);
 const two=machine("g2",server.url),before=state.calls,result=await two.cache.run(key,join(two.root,"out"),producer("forged",state),undefined,"mod");
 expect([result.hit,state.calls]).toEqual([false,before+1]);expect(readFileSync(join(two.root,"out","a.js"),"utf8")).toContain("forged");
});

test("a local hit of an entry the fleet never saw is offered once, so a warm machine seeds the fleet",async()=>{
 const root=directory("warm"),stages=join(root,"stages"),state={calls:0},key=keyOf("probe","component-warm");
 await stageCacheV1(stages).run(key,join(root,"out"),producer("warm",state));
 const server=await serve(),client=createFleetClientV1(readFleetSettingsV1(join(root,"state"),environment(server.url))!),cache=sharedStageCacheV1(stageCacheV1(stages),{client});
 const hit=await cache.run(key,join(root,"out2"),producer("warm",state),undefined,"mod");expect([hit.hit,state.calls]).toEqual([true,1]);
 const fresh=machine("warm2",server.url),again=await fresh.cache.run(key,join(fresh.root,"out"),producer("warm",state),undefined,"mod");
 expect([again.hit,state.calls]).toEqual([true,1]);expect(tree(join(fresh.root,"out"))).toEqual(tree(join(root,"out2")));
 expect(existsSync(join(stages,".offered",stageRemoteKeyV1(stageKeyDigestV1(key))))).toBe(true);
});

test("only the synchronized state is kept: a newer key of a slot supersedes the older ones after the grace, nothing else is touched",async()=>{
 let now=1_000_000;
 const m=machine("local",undefined,{now:()=>now}),state={calls:0},run=(stage:string,input:string,slot:string|undefined,out:string)=>m.cache.run(keyOf(stage,input),join(m.root,out),producer(input,state),undefined,slot);
 const present=(stage:string,input:string)=>existsSync(join(m.stages,stage,stageKeyDigestV1(keyOf(stage,input))));
 await run("transpile","v1","mod","o1");await run("transpile","other-module","other","o2");await run("optimize","v1","mod","o3");await run("transpile","unslotted",undefined,"o4");
 now+=1000;await run("transpile","v2","mod","o5");
 expect([present("transpile","v1"),present("transpile","v2")]).toEqual([true,true]);
 now+=policy.localGraceMilliseconds+1;await run("transpile","v3","mod","o6");
 expect([present("transpile","v1"),present("transpile","v2"),present("transpile","v3")]).toEqual([false,true,true]);
 expect([present("transpile","other-module"),present("optimize","v1"),present("transpile","unslotted")]).toEqual([true,true,true]);
 now+=policy.localGraceMilliseconds+1;await run("transpile","v2","mod","o7");
 expect([present("transpile","v2"),present("transpile","v3")]).toEqual([true,true]);
 now+=policy.localGraceMilliseconds+1;await run("transpile","v2","mod","o8");
 expect([present("transpile","v3"),present("transpile","v2")]).toEqual([false,true]);
 const index=JSON.parse(readFileSync(join(m.stages,".slots","transpile",`${stageSlotIdV1("transpile",1,"mod")}.json`),"utf8"));
 expect(new Ajv({strict:false}).addSchema(schema).getSchema(`${schema.$id}#/definitions/SlotIndex`)!(index)).toBe(true);expect(index.rows.length).toBe(1);
 expect(await settleStageSlotV1(m.stages,{stage:"transpile",slotId:stageSlotIdV1("transpile",1,"mod"),digest:stageKeyDigestV1(keyOf("transpile","v2")),now,graceMilliseconds:0})).toEqual([]);
});

test("the flight is bounded: an unreachable cache is a fault at once, no claims means compute, aborting stops the waiting",async()=>{
 const client=createFleetClientV1(readFleetSettingsV1(join(directory("flight"),"s"),environment("http://127.0.0.1:1"))!);
 const fault=await obtainFleetEntryV1({client,key:"a".repeat(64),owner:"o",fetch:async()=>"fault"});expect(fault).toMatchObject({outcome:"compute",reason:"fault"});
 const hit=await obtainFleetEntryV1({client,key:"a".repeat(64),owner:"o",fetch:async()=>"hit"});expect(hit).toMatchObject({outcome:"hit",via:"cache"});
 const readOnly=createFleetClientV1(readFleetSettingsV1(join(directory("flight2"),"s"),environment("http://127.0.0.1:1","x","read-only"))!);
 expect(await obtainFleetEntryV1({client:readOnly,key:"a".repeat(64),owner:"o",fetch:async()=>"miss"})).toMatchObject({outcome:"compute",reason:"no-claims"});
 const thrown=await obtainFleetEntryV1({client,key:"a".repeat(64),owner:"o",fetch:async()=>{throw Error("boom");}});expect(thrown).toMatchObject({outcome:"compute",reason:"error"});
});

test("the driver's decoration reads the cache from the environment, reports in both languages, and stays local when the switch is off or nothing is named",async()=>{
 const server=await serve(),state={calls:0},key=keyOf("transpile","component-decor"),lines:string[]=[];
 const make=(env:Record<string,string|undefined>,language:"en"|"de")=>{const root=directory("decor"),cache=stageFleetDecoratorV1({environment:env,workspaceRoot:root,report:event=>lines.push(stageFleetEventTextV1(language,event))})(stageCacheV1(join(root,"stages")));return {root,cache};};
 const one=make(environment(server.url),"en"),two=make(environment(server.url),"de");
 await one.cache.run(key,join(one.root,"out"),producer("decor",state),undefined,"mod");await two.cache.run(key,join(two.root,"out"),producer("decor",state),undefined,"mod");
 expect(state.calls).toBe(1);expect(lines[0]).toContain("was shared with the fleet cache");expect(lines[1]).toContain("kam aus dem Flotten-Cache");
 for(const env of[{},{...environment(server.url),SEMIO_STAGE_CACHE_REMOTE:"off"},{...environment(server.url),SEMIO_UNIT_CACHE_REMOTE:"off"}]){
  const root=directory("decor-off");expect(stageFleetClientV1({stateRoot:join(root,"s"),workspaceRoot:root,environment:env})).toBeUndefined();
  const local=stageFleetDecoratorV1({environment:env,workspaceRoot:root})(stageCacheV1(join(root,"stages"))),before=state.calls;
  expect((await local.run(key,join(root,"out"),producer("decor",state),undefined,"mod")).hit).toBe(false);expect(state.calls).toBe(before+1);
 }
});
