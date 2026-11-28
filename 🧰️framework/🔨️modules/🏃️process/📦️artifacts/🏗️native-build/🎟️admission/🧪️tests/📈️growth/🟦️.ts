/** 📈️ Contract: a unit that needs more than the parent memory grant is never refused while this host or the fleet can hold it: on a host with the room the grant grows by the difference (a supplementary booking beside the grant, bounded in time, cancellable, reported in English and German), a unit this host can never hold is queued for a larger machine and never compiled here, and only a unit no machine of the fleet holds is refused, naming the unit, its need and the largest host.
 * @see ../../🏔️ceiling/🟦️.ts (`decideGrantGrowthV1`)
 * @see ../../🟦️.ts (`startAdmissionBrokerV1`) */
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {createConnection} from "node:net";
import {fileURLToPath} from "node:url";
import {expect,test} from "bun:test";
import {requireTestToolV1} from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {ADMISSION_POLICY_V1,decideGrantGrowthV1,formatGrowthV1,hostHoldableMegabytesV1,localHoldableMegabytesV1,startAdmissionBrokerV1,unitBeyondHostV1,unitKeyV1,type AdmitRequestV1,type GrantGrowthPortV1,type GrowthHostV1,type UnitObservationV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import requests from "../🧫️fixtures/🔣️.json" with {type:"json"};
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

const MiB=1<<20,base=requests.requests.valid[0]! as unknown as AdmitRequestV1,pass={id:"wasm-release-00",profile:"release",target:"host"};
const unit=(crate:string):AdmitRequestV1=>({...base,crate,package:crate,crateTypes:["lib"],target:"host",externs:[],pid:1});
const history=(units:Readonly<Record<string,number>>)=>new Map<string,readonly UnitObservationV1[]>(Object.entries(units).map(([crate,peak])=>{
 const key=unitKeyV1(unit(crate),pass.profile);
 return [key,[{v:1,key,peakRssBytes:peak*MiB,outputBytes:0,wallMs:1000,cpuMs:1000,remainingMs:0,atMs:1,lines:0}]];
}));
const host=(origin:string,total:number,available?:number):GrowthHostV1=>({origin,totalMegabytes:total,floorMegabytes:2048,interactiveReserveMegabytes:4096,budgetBasisPoints:9000,...available===undefined?{}:{availableMegabytes:available}});
const connect=(endpoint:string,request:AdmitRequestV1)=>{
 const socket=createConnection(endpoint),queue:Record<string,unknown>[]=[],readers:((reply:Record<string,unknown>)=>void)[]=[];
 let partial="";
 socket.setTimeout(4000);
 socket.on("timeout",()=>socket.destroy(new Error("Admission reply timed out")));
 socket.on("error",()=>undefined);
 socket.on("connect",()=>socket.write(JSON.stringify(request)+"\n"));
 socket.on("data",chunk=>{partial+=chunk;let next:number;while((next=partial.indexOf("\n"))>=0){const reply=JSON.parse(partial.slice(0,next));partial=partial.slice(next+1);const reader=readers.shift();if(reader)reader(reply);else queue.push(reply);}});
 return {socket,next:()=>queue.length?Promise.resolve(queue.shift()!):new Promise<Record<string,unknown>>((accept,reject)=>{readers.push(accept);socket.once("error",reject);})};
};
const eventually=async(check:()=>boolean)=>{const started=Date.now();while(!check()&&Date.now()-started<2000)await new Promise(done=>setTimeout(done,10));return check();};

type Ask=Parameters<GrantGrowthPortV1["acquire"]>[0];
const scenario=async(options:Readonly<{peakMiB:number;parentMiB:number;local:GrowthHostV1|undefined;fleet:readonly GrowthHostV1[]|undefined;growthPolicy?:Partial<typeof ADMISSION_POLICY_V1.growth>;acquire:(ask:Ask)=>ReturnType<GrantGrowthPortV1["acquire"]>}>,body:(context:Readonly<{broker:Awaited<ReturnType<typeof startAdmissionBrokerV1>>;client:ReturnType<typeof connect>;lines:string[];asks:Ask[]}>)=>Promise<void>)=>{
 const lines:string[]=[],asks:Ask[]=[],port:GrantGrowthPortV1={local:()=>options.local,fleet:()=>options.fleet,acquire:ask=>{asks.push(ask);return options.acquire(ask);}};
 const broker=await startAdmissionBrokerV1({physicalBytes:32*1024*MiB,history:history({big:options.peakMiB}),parentMemoryAllowanceBytes:options.parentMiB*MiB,growth:port,available:()=>undefined,warden:()=>"calm",cancels:()=>[],log:line=>lines.push(line),...options.growthPolicy?{growthPolicy:options.growthPolicy}:{}});
 broker.registerPass(pass);
 const client=connect(broker.endpoint,unit("big"));
 try{await body({broker,client,lines,asks});}finally{client.socket.destroy();await broker.close();}
};

test("the vectors are valid and every case equals the law and the independent Python oracle",()=>{
 expect(new Ajv({strict:true}).validate(corpusSchema,corpus),"corpus schema").toBe(true);
 const law=corpus.cases.map(row=>decideGrantGrowthV1({needMegabytes:row.need,grantMegabytes:row.grant,local:row.local as GrowthHostV1,fleet:(row.fleet as GrowthHostV1[]|null)??undefined}));
 for(const [index,row] of corpus.cases.entries())expect<string>(law[index]!.kind,row.name).toBe(row.expected.kind);
 const result=spawnSync(requireTestToolV1("python"),["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"});
 expect(result.status,result.stderr).toBe(0);
 expect(law).toEqual(JSON.parse(result.stdout));
});

test("the holdable memory follows the host-capacity arithmetic and a unit above it is beyond the machine whatever the class says",()=>{
 expect(hostHoldableMegabytesV1(host("a",32768))).toBe(23961);
 expect(hostHoldableMegabytesV1(host("a",1024))).toBe(0);
 const total=16384,holdable=localHoldableMegabytesV1(total),policy=ADMISSION_POLICY_V1.unitCeiling;
 expect(holdable).toBeGreaterThan(0);expect(holdable).toBeLessThan(total);
 const target={crate:"unlisted_unit",target:"host"};
 expect(unitBeyondHostV1(policy,target,holdable-1,total+1,holdable)).toBe(false);
 expect(unitBeyondHostV1(policy,target,holdable+1,total+1,holdable)).toBe(true);
 expect(unitBeyondHostV1(policy,target,holdable+1,total+1)).toBe(false);
});

test("on a big host a unit above the grant gets a grown grant, runs and gives the booking back when it is done",async()=>{
 let released=0;
 await scenario({peakMiB:4894,parentMiB:1536,local:host("g1-server",32768,20000),fleet:[host("g1-m01",16384)],acquire:async()=>({granted:true,release:()=>{released++;}})},async({broker,client,lines,asks})=>{
  expect(await client.next()).toMatchObject({op:"verdict",admitted:false});
  expect(await client.next()).toMatchObject({op:"verdict",admitted:true});
  expect(asks).toHaveLength(1);
  expect(asks[0]!.extraMegabytes).toBeGreaterThan(4894-1536);expect(asks[0]!.waitMilliseconds).toBe(ADMISSION_POLICY_V1.growth.waitMilliseconds);
  expect(lines.some(line=>line.includes("asking the host to grow the grant")&&line.includes("Zuteilung")&&line.includes("big"))).toBe(true);
  expect(lines.some(line=>line.includes("grew to")&&line.includes("gewachsen"))).toBe(true);
  expect(broker.stats().running).toBe(1);expect(released).toBe(0);
  client.socket.write(JSON.stringify({v:1,op:"done",exit:0,peakRssBytes:4894*MiB,userMs:1,sysMs:1,startMs:1,endMs:2,rmetaMs:null})+"\n");
  expect(await eventually(()=>released===1)).toBe(true);
 });
});

test("on a small host a unit it can never hold is queued beyond the host, never compiled and never waited for here",async()=>{
 const validate=new Ajv({strict:true}).compile(schema.definitions.RefusalV1);
 await scenario({peakMiB:12000,parentMiB:1536,local:host("g1-m01",16384,14000),fleet:[host("g1-server",32768),host("g1-m01",16384)],acquire:async()=>({granted:true,release:()=>{}})},async({client,lines,asks})=>{
  const reply=await client.next();
  expect(validate(reply)).toBe(true);
  expect(reply).toMatchObject({op:"refused",reason:"beyond-host",allowanceBytes:hostHoldableMegabytesV1(host("g1-m01",16384))*MiB});
  expect(asks).toHaveLength(0);
  expect(lines.some(line=>line.includes("queued for a larger machine")&&line.includes("größere Maschine")&&line.includes("big"))).toBe(true);
 });
});

test("only a unit no machine of the fleet can hold is refused, with a bilingual reason naming the unit, its need and the largest host",async()=>{
 const validate=new Ajv({strict:true}).compile(schema.definitions.RefusalV1);
 await scenario({peakMiB:30000,parentMiB:1536,local:host("g1-m01",16384,14000),fleet:[host("g1-server",32768),host("g1-m01",16384)],acquire:async()=>({granted:true,release:()=>{}})},async({client,lines,asks})=>{
  const reply=await client.next();
  expect(validate(reply)).toBe(true);
  expect(reply).toMatchObject({op:"refused",reason:"no-host-fits",allowanceBytes:hostHoldableMegabytesV1(host("g1-server",32768))*MiB});
  expect(asks).toHaveLength(0);
  const line=lines.find(row=>row.includes("no machine of the fleet can hold unit big"))!;
  expect(line).toBeDefined();expect(line).toContain("g1-server");expect(line).toContain("Keine Maschine der Flotte kann Einheit big halten");expect(line).toContain("23961");
 });
});

test("a growth that is not granted in time ends as the old refusal, and a vanished client cancels the wait",async()=>{
 await scenario({peakMiB:4894,parentMiB:1536,local:host("g1-server",32768,20000),fleet:[],acquire:async()=>({granted:false,reason:"timeout"})},async({client,lines})=>{
  expect(await client.next()).toMatchObject({op:"verdict",admitted:false});
  expect(await client.next()).toMatchObject({op:"verdict",admitted:true});
  expect(lines.some(line=>line.includes("got no grant growth")&&line.includes("kein Zuteilungswachstum"))).toBe(true);
  expect(lines.some(line=>line.includes("alone")||line.includes("allein"))).toBe(true);
 });
 let signal:AbortSignal|undefined;
 await scenario({peakMiB:4894,parentMiB:1536,local:host("g1-server",32768,3000),fleet:[],acquire:ask=>new Promise(accept=>{signal=ask.signal;ask.signal.addEventListener("abort",()=>accept({granted:false,reason:"cancelled"}),{once:true});})},async({broker,client,asks})=>{
  expect(await client.next()).toMatchObject({op:"verdict",admitted:false});
  expect(await eventually(()=>asks.length===1)).toBe(true);
  client.socket.destroy();
  expect(await eventually(()=>signal?.aborted===true)).toBe(true);
  expect(broker.stats()).toMatchObject({running:0,waiting:0});
 });
});

test("a growth wait that really waits keeps the client connected with heartbeats and ends granted once room appears",async()=>{
 let released=0,answer:(row:Awaited<ReturnType<GrantGrowthPortV1["acquire"]>>)=>void=()=>undefined;
 await scenario({peakMiB:2191,parentMiB:1536,local:host("g1-m07",16384,6000),fleet:[],growthPolicy:{progressMilliseconds:30},acquire:()=>new Promise(accept=>{answer=accept;})},async({broker,client,asks})=>{
  expect(await client.next()).toMatchObject({op:"verdict",admitted:false});
  expect(await eventually(()=>asks.length===1)).toBe(true);
  expect(asks[0]!.waitMilliseconds).toBe(ADMISSION_POLICY_V1.growth.waitMilliseconds);
  expect(await client.next()).toMatchObject({op:"verdict",admitted:false});
  expect(await client.next()).toMatchObject({op:"verdict",admitted:false});
  expect(client.socket.destroyed).toBe(false);expect(asks[0]!.signal.aborted).toBe(false);
  expect(broker.stats()).toMatchObject({running:0});
  answer({granted:true,release:()=>{released++;}});
  expect(await eventually(()=>broker.stats().running===1)).toBe(true);
  expect(asks[0]!.signal.aborted).toBe(false);
  client.socket.write(JSON.stringify({v:1,op:"done",exit:0,peakRssBytes:2191*MiB,userMs:1,sysMs:1,startMs:1,endMs:2,rmetaMs:null})+"\n");
  expect(await eventually(()=>released===1)).toBe(true);
 });
});

test("a gate that answers cancelled while the client is still connected never closes the connection: the unit runs alone beside its grant",async()=>{
 await scenario({peakMiB:2191,parentMiB:1536,local:host("g1-m07",16384,6000),fleet:[],acquire:async()=>({granted:false,reason:"cancelled"})},async({client,lines})=>{
  expect(await client.next()).toMatchObject({op:"verdict",admitted:false});
  expect(await client.next()).toMatchObject({op:"verdict",admitted:true});
  expect(client.socket.destroyed).toBe(false);
  expect(lines.some(line=>line.includes("cancelled after"))).toBe(false);
 });
});

test("without a port, with unknown host facts or under a parent that grants nothing the broker keeps refusing as before",async()=>{
 const noPort=await startAdmissionBrokerV1({physicalBytes:32*1024*MiB,history:history({big:4894}),parentMemoryAllowanceBytes:1536*MiB,available:()=>undefined,warden:()=>"calm",cancels:()=>[]});
 noPort.registerPass(pass);
 const first=connect(noPort.endpoint,unit("big"));
 try{expect(await first.next()).toMatchObject({op:"refused",reason:"parent-memory"});}finally{first.socket.destroy();await noPort.close();}
 await scenario({peakMiB:4894,parentMiB:1536,local:undefined,fleet:[],acquire:async()=>({granted:true,release:()=>{}})},async({client,asks})=>{expect(await client.next()).toMatchObject({op:"refused",reason:"parent-memory"});expect(asks).toHaveLength(0);});
 await scenario({peakMiB:4894,parentMiB:0,local:host("g1-server",32768,20000),fleet:[],acquire:async()=>({granted:true,release:()=>{}})},async({client,asks})=>{expect(await client.next()).toMatchObject({op:"refused",reason:"parent-memory"});expect(asks).toHaveLength(0);});
});

test("every growth text exists in English and German with the same placeholders",()=>{
 const en=schema.definitions.Messages.const.en.growth,de=schema.definitions.Messages.const.de.growth,holes=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(row=>row[1]).sort();
 expect(Object.keys(de).sort()).toEqual(Object.keys(en).sort());
 for(const name of Object.keys(en) as (keyof typeof en)[]){expect(holes(de[name]),name).toEqual(holes(en[name]));expect(formatGrowthV1("en",name,{}),name).not.toBe(formatGrowthV1("de",name,{}));}
});
