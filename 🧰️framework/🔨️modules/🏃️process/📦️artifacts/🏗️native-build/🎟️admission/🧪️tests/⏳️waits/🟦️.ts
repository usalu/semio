/** ⏳️ Contract: a waiting compile unit is never silent: the broker prints what it waits for and for how long (English and German) and publishes the rows of every waiting unit; a grant growth waits beside the units that fit, not in front of them.
 * @see ../../🟦️.ts (`startAdmissionBrokerV1`, `formatWaitV1`) */
import {spawnSync} from "node:child_process";
import {createConnection} from "node:net";
import {fileURLToPath} from "node:url";
import {expect,test} from "bun:test";
import {requireTestToolV1} from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {formatWaitV1,startAdmissionBrokerV1,unitKeyV1,type AdmitRequestV1,type GrantGrowthPortV1,type GrowthHostV1,type UnitObservationV1,type UnitWaitV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import requests from "../🧫️fixtures/🔣️.json" with {type:"json"};
import corpus from "./🔣️.json";

const MiB=1<<20,base=requests.requests.valid[0]! as unknown as AdmitRequestV1,pass={id:"wasm-release-00",profile:"release",target:"host"};
const unit=(crate:string):AdmitRequestV1=>({...base,crate,package:crate,crateTypes:["lib"],target:"host",externs:[],pid:1});
const history=(units:Readonly<Record<string,number>>)=>new Map<string,readonly UnitObservationV1[]>(Object.entries(units).map(([crate,peak])=>{
 const key=unitKeyV1(unit(crate),pass.profile);
 return [key,[{v:1,key,peakRssBytes:peak*MiB,outputBytes:0,wallMs:1000,cpuMs:1000,remainingMs:0,atMs:1,lines:0}]];
}));
const connect=(endpoint:string,request:AdmitRequestV1)=>{
 const socket=createConnection(endpoint),queue:Record<string,unknown>[]=[],readers:((reply:Record<string,unknown>)=>void)[]=[];
 let partial="";
 socket.setTimeout(8000);
 socket.on("error",()=>undefined);
 socket.on("connect",()=>socket.write(JSON.stringify(request)+"\n"));
 socket.on("data",chunk=>{partial+=chunk;let next:number;while((next=partial.indexOf("\n"))>=0){const reply=JSON.parse(partial.slice(0,next));partial=partial.slice(next+1);const reader=readers.shift();if(reader)reader(reply);else queue.push(reply);}});
 return {socket,next:()=>queue.length?Promise.resolve(queue.shift()!):new Promise<Record<string,unknown>>(accept=>{readers.push(accept);})};
};
const eventually=async(check:()=>boolean)=>{const started=Date.now();while(!check()&&Date.now()-started<4000)await new Promise(done=>setTimeout(done,10));return check();};

test("the status lines of waiting units equal the independent Python oracle in both languages",()=>{
 const result=spawnSync(requireTestToolV1("python"),["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url)),fileURLToPath(new URL("../../🧬️schema/🔣️.json",import.meta.url))],{encoding:"utf8"});
 expect(result.status,result.stderr).toBe(0);
 expect(JSON.parse(result.stdout)).toEqual(corpus.lines);
});

test("every waiting text exists in English and German with the same placeholders",()=>{
 const en=schema.definitions.Messages.const.en.waits,de=schema.definitions.Messages.const.de.waits,holes=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(row=>row[1]).sort();
 expect(Object.keys(de).sort()).toEqual(Object.keys(en).sort());
 for(const name of Object.keys(en) as (keyof typeof en)[]){expect(holes(de[name]),name).toEqual(holes(en[name]));expect(formatWaitV1("en",name,{}),name).not.toBe(formatWaitV1("de",name,{}));}
});

test("a unit queued behind a running unit prints its wait in both languages and is published until it runs",async()=>{
 const lines:string[]=[],published:(readonly UnitWaitV1[])[]=[];
 const broker=await startAdmissionBrokerV1({physicalBytes:1024*MiB,reserveBytes:0,history:history({first:600,second:600}),available:()=>undefined,warden:()=>"calm",cancels:()=>[],log:line=>lines.push(line),onWaits:rows=>published.push(rows),waits:{publishMilliseconds:20,progressMilliseconds:40}});
 broker.registerPass(pass);
 const first=connect(broker.endpoint,unit("first")),second=connect(broker.endpoint,unit("second"));
 try{
  expect(await first.next()).toMatchObject({op:"verdict",admitted:true});
  expect(await second.next()).toMatchObject({op:"verdict",admitted:false});
  expect(await eventually(()=>published.some(rows=>rows.some(row=>row.unit==="second"&&row.reason==="queue"&&row.place===1&&row.count===1&&row.running===1)))).toBe(true);
  expect(await eventually(()=>lines.some(line=>line.includes("unit second waits in the broker queue (place 1 of 1)")&&line.includes(" / Einheit second wartet in der Warteschlange des Brokers (Platz 1 von 1)")))).toBe(true);
  first.socket.destroy();
  expect(await second.next()).toMatchObject({op:"verdict",admitted:true});
  expect(await eventually(()=>published.at(-1)?.length===0)).toBe(true);
 }finally{first.socket.destroy();second.socket.destroy();await broker.close();}
});

test("a grant growth is a published wait beside the units that fit, which run meanwhile",async()=>{
 const published:(readonly UnitWaitV1[])[]=[],host:GrowthHostV1={origin:"g1-server",totalMegabytes:32768,availableMegabytes:3000,floorMegabytes:2048,interactiveReserveMegabytes:4096,budgetBasisPoints:9000};
 let signal:AbortSignal|undefined;
 const port:GrantGrowthPortV1={local:()=>host,fleet:()=>[],acquire:ask=>new Promise(accept=>{signal=ask.signal;ask.signal.addEventListener("abort",()=>accept({granted:false,reason:"cancelled"}),{once:true});})};
 const broker=await startAdmissionBrokerV1({physicalBytes:32*1024*MiB,history:history({big:4894,small:200}),parentMemoryAllowanceBytes:1536*MiB,growth:port,available:()=>undefined,warden:()=>"calm",cancels:()=>[],onWaits:rows=>published.push(rows),waits:{publishMilliseconds:20,progressMilliseconds:40}});
 broker.registerPass(pass);
 const big=connect(broker.endpoint,unit("big")),small=connect(broker.endpoint,unit("small"));
 try{
  expect(await big.next()).toMatchObject({op:"verdict",admitted:false});
  expect(await small.next()).toMatchObject({op:"verdict",admitted:true});
  expect(await eventually(()=>published.some(rows=>rows.some(row=>row.unit==="big"&&row.reason==="growth")))).toBe(true);
  big.socket.destroy();
  expect(await eventually(()=>signal?.aborted===true)).toBe(true);
 }finally{big.socket.destroy();small.socket.destroy();await broker.close();}
});

test("the driver's own memory above the grant is grown by the difference, not waited out: the unit that fits the grant alone runs at once",async()=>{
 const asks:number[]=[],host:GrowthHostV1={origin:"g1-server",totalMegabytes:32768,availableMegabytes:20000,floorMegabytes:2048,interactiveReserveMegabytes:4096,budgetBasisPoints:9000};
 const port:GrantGrowthPortV1={local:()=>host,fleet:()=>[],acquire:async ask=>{asks.push(ask.extraMegabytes);return{granted:true,release:()=>undefined};}};
 const broker=await startAdmissionBrokerV1({physicalBytes:16*1024*MiB,history:history({first:256}),parentMemoryAllowanceBytes:1333*MiB,growth:port,available:()=>undefined,warden:()=>"calm",cancels:()=>[]});
 broker.registerPass(pass);
 broker.observe({nonRustcBytes:1500*MiB,swapOutBytesPerSecond:null});
 const first=connect(broker.endpoint,unit("first"));
 try{
  expect(await first.next()).toMatchObject({op:"verdict",admitted:false});
  expect(await first.next()).toMatchObject({op:"verdict",admitted:true});
  expect(asks).toHaveLength(1);
  expect(asks[0]).toBe(Math.ceil((Math.ceil(1.15*256*MiB)+1500*MiB-1333*MiB)/MiB));
 }finally{first.socket.destroy();await broker.close();}
});

test("a unit that stalls because the driver grew after it arrived is rescued by a further growth, never left idle",async()=>{
 const asks:number[]=[],host:GrowthHostV1={origin:"g1-server",totalMegabytes:32768,availableMegabytes:20000,floorMegabytes:2048,interactiveReserveMegabytes:4096,budgetBasisPoints:9000};
 let level:"guard"|"calm"="guard";
 const port:GrantGrowthPortV1={local:()=>host,fleet:()=>[],acquire:async ask=>{asks.push(ask.extraMegabytes);return{granted:true,release:()=>undefined};}};
 const broker=await startAdmissionBrokerV1({physicalBytes:16*1024*MiB,history:history({first:1500}),parentMemoryAllowanceBytes:2000*MiB,growth:port,available:()=>undefined,warden:()=>level,cancels:()=>[],waits:{publishMilliseconds:20,progressMilliseconds:40}});
 broker.registerPass(pass);
 const first=connect(broker.endpoint,unit("first"));
 try{
  expect(await first.next()).toMatchObject({op:"verdict",admitted:false});
  broker.observe({nonRustcBytes:1000*MiB,swapOutBytesPerSecond:null});
  level="calm";
  expect(await first.next()).toMatchObject({op:"verdict",admitted:true});
  expect(asks).toHaveLength(1);
  expect(asks[0]).toBe(Math.ceil((Math.ceil(1.15*1500*MiB)+1000*MiB-2000*MiB)/MiB));
 }finally{first.socket.destroy();await broker.close();}
});
