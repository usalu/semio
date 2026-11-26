/** 🪆️ Compiler admission consumes only the original parent memory grant. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {createConnection} from "node:net";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir,uptime} from "node:os";
import {join} from "node:path";
import {ADMISSION_POLICY_V1,parentCompilerMemoryAllowanceBytesV1,startAdmissionBrokerV1,unitKeyV1,type AdmitRequestV1,type CachedRequestV1,type UnitObservationV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import requests from "../🧫️fixtures/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};

const MiB=1<<20,base=requests.requests.valid[0]! as unknown as AdmitRequestV1,pass={id:"wasm-release-00",profile:"release",target:"host"};
const unit=(crate:string):AdmitRequestV1=>({...base,crate,package:crate,crateTypes:["lib"],target:"host",externs:[],pid:1});
const history=(units:Readonly<Record<string,number>>)=>new Map<string,readonly UnitObservationV1[]>(Object.entries(units).map(([crate,peak])=>{
 const key=unitKeyV1(unit(crate),pass.profile);
 return [key,[{v:1,key,peakRssBytes:peak*MiB,outputBytes:0,wallMs:1000,cpuMs:1000,remainingMs:0,atMs:1,lines:0}]];
}));
const connect=(endpoint:string,request:CachedRequestV1|Omit<AdmitRequestV1,"op">&{op:"admit"|"probe"})=>{
 const socket=createConnection(endpoint),queue:Record<string,unknown>[]=[],readers:((reply:Record<string,unknown>)=>void)[]=[];
 let partial="";
 socket.setTimeout(2000);
 socket.on("timeout",()=>socket.destroy(new Error("Admission reply timed out")));
 socket.on("error",()=>undefined);
 socket.on("connect",()=>socket.write(JSON.stringify(request)+"\n"));
 socket.on("data",chunk=>{partial+=chunk;let next:number;while((next=partial.indexOf("\n"))>=0){const reply=JSON.parse(partial.slice(0,next));partial=partial.slice(next+1);const reader=readers.shift();if(reader)reader(reply);else queue.push(reply);}});
 return {socket,next:()=>queue.length?Promise.resolve(queue.shift()!):new Promise<Record<string,unknown>>((accept,reject)=>{readers.push(accept);socket.once("error",reject);})};
};

test("live parent reservation is authoritative in the actual inherited environment",()=>{
 const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"parent-runtime-")),id="native_parent_001",db=new Database(":memory:");
 try {
  for(const [index,row] of fixture.ledgers.entries()){
   const scope=join(directory,String(index));mkdirSync(join(scope,"grants"),{recursive:true});
   const environment:Record<string,string>={SEMIO_HOST_CAPACITY_DIRECTORY:scope};
   if(row.carriedMiB!==null)environment.SEMIO_HOST_CAPACITY_PLAN=JSON.stringify({version:1,id,jobs:4,cpu:4,memoryMegabytes:row.carriedMiB,fixedMegabytes:256,floorMegabytes:2048});
   if(row.claim!=="none")environment.SEMIO_HOST_CAPACITY_GRANT=row.claim==="matching"?id:"native_parent_002";
   if(row.reservedMiB!==null)writeFileSync(join(scope,"grants",id+".json"),JSON.stringify({id,operation:"run",jobs:2,cpu:2,reservedMegabytes:row.reservedMiB,admittedAtMs:Date.now(),observedMegabytes:null,pid:process.pid,bootAtMs:Math.round((Date.now()-uptime()*1000)/1000)*1000,...row.state==="quiet"?{quiet:true}:row.state==="lent"?{lentSinceMs:Date.now()}:{} }));
   const oracle=db.query("select case when ?1='none' then null when ?1='matching' and ?2='live' then ?3 else 0 end as allowance").get(row.claim,row.state,row.reservedMiB) as {allowance:number|null};
   expect(oracle.allowance,row.name).toBe(row.expectedMiB);
   expect(parentCompilerMemoryAllowanceBytesV1(environment),row.name).toBe(oracle.allowance===null?undefined:oracle.allowance*MiB);
  }
 }finally{db.close();rmSync(directory,{recursive:true,force:true});}
});

test("broker refuses permanently oversized local work before the first waiter or remote fallback",async()=>{
 const db=new Database(":memory:"),validate=new Ajv({strict:true}).compile(schema.definitions.RefusalV1);
 try{for(const row of fixture.broker){
  const parent=row.parentMiB===null?undefined:row.parentMiB*MiB,expected=db.query("select case when ?1 is not null and (?2*1048576*23+19)/20>?1 then 'refused' else 'admitted' end as verdict").get(parent??null,row.peakMiB) as {verdict:string};
  expect(expected.verdict,row.name).toBe(row.expected);
  let clock=1_000_000;
  const broker=await startAdmissionBrokerV1({physicalBytes:16*1024*MiB,history:history({first:row.peakMiB}),parentMemoryAllowanceBytes:parent,now:()=>clock,available:()=>row.availableMiB===null?undefined:row.availableMiB*MiB,warden:()=>"calm",cancels:()=>[]});
  broker.registerPass(pass);
  const probe=connect(broker.endpoint,{...unit("first"),op:"probe"});
  const client=connect(broker.endpoint,unit("first"));
  try{
   expect(await probe.next(),row.name).toMatchObject({op:"headroom",wait:row.expected==="refused"});
   const first=await client.next();
   if(row.expected==="refused"){
    expect(validate(first),row.name).toBe(true);
    expect(first,row.name).toMatchObject({op:"refused",reason:"parent-memory",allowanceBytes:parent});
    clock+=ADMISSION_POLICY_V1.available.waitMilliseconds;
    broker.observe({nonRustcBytes:0,swapOutBytesPerSecond:null});
    expect(broker.stats(),row.name).toMatchObject({running:0,waiting:0});
   }else if(row.wait){
    expect(first,row.name).toMatchObject({op:"verdict",admitted:false});
    clock+=ADMISSION_POLICY_V1.available.waitMilliseconds;
    broker.observe({nonRustcBytes:0,swapOutBytesPerSecond:null});
    expect(await client.next(),row.name).toMatchObject({op:"verdict",admitted:true});
   }else expect(first,row.name).toMatchObject({op:"verdict",admitted:true});
  }finally{client.socket.destroy();probe.socket.destroy();await broker.close();}
 }}finally{db.close();}
});

test("parent room accounts for driver overhead and other live compilers",async()=>{
 const row=fixture.overhead,oracle=new Database(":memory:");
 try{
  expect(oracle.query("select ?1*20-?2*20 < ?3*23 overhead, ?1*20 < (?3+?4)*23 concurrency").get(row.parentMiB,row.nonCompilerMiB,row.peakMiB,row.secondPeakMiB)).toEqual({overhead:1,concurrency:1});
 }finally{oracle.close();}
 let clock=1_000_000;
 const broker=await startAdmissionBrokerV1({physicalBytes:16*1024*MiB,history:history({first:row.peakMiB,second:row.secondPeakMiB}),parentMemoryAllowanceBytes:row.parentMiB*MiB,now:()=>clock,available:()=>undefined,warden:()=>"calm",cancels:()=>[]});
 broker.registerPass(pass);
 broker.observe({nonRustcBytes:row.nonCompilerMiB*MiB,swapOutBytesPerSecond:null});
 const first=connect(broker.endpoint,unit("first"));
 let second:ReturnType<typeof connect>|undefined;
 try{
  expect(await first.next()).toMatchObject({op:"verdict",admitted:false});
  clock+=ADMISSION_POLICY_V1.nonRustcWindowMs+1;
  broker.observe({nonRustcBytes:0,swapOutBytesPerSecond:null});
  expect(await first.next()).toMatchObject({op:"verdict",admitted:true});
  second=connect(broker.endpoint,unit("second"));
  expect(await second.next()).toMatchObject({op:"verdict",admitted:false});
  first.socket.destroy();
  expect(await second.next()).toMatchObject({op:"verdict",admitted:true});
 }finally{first.socket.destroy();second?.socket.destroy();await broker.close();}
});

test("fleet-only pass accepts cached outputs but refuses local fallback before spawning the compiler",async()=>{
 const row=fixture.fleetOnly,request=unit("fleet"),broker=await startAdmissionBrokerV1({physicalBytes:16*1024*MiB,history:history({fleet:row.peakMiB}),parentMemoryAllowanceBytes:row.parentMiB*MiB,available:()=>undefined,warden:()=>"calm",cancels:()=>[]});
 broker.registerPass({...pass,placement:"fleet-only"});
 const probe=connect(broker.endpoint,{...request,op:"probe"});
 const cached=connect(broker.endpoint,{...request,op:"cached",cacheKey:"a".repeat(64),provenance:null});
 const fallback=connect(broker.endpoint,request);
 let retry:ReturnType<typeof connect>|undefined;
 try{
  expect(await probe.next()).toMatchObject({op:"headroom",wait:true});
  expect(await cached.next()).toMatchObject({op:"verdict",admitted:true});
  const refusal=await fallback.next();
  expect(new Ajv({strict:true}).compile(schema.definitions.RefusalV1)(refusal)).toBe(true);
  expect(refusal).toMatchObject({op:"refused",reason:row.reason,allowanceBytes:row.allowanceBytes});
  expect(broker.placementFailed(pass.id)).toBe(true);
  expect(broker.stats()).toMatchObject({running:0,waiting:0});
  broker.registerPass(pass);
  expect(broker.placementFailed(pass.id)).toBe(false);
  retry=connect(broker.endpoint,request);
  expect(await retry.next()).toMatchObject({op:"verdict",admitted:true});
 }finally{probe.socket.destroy();cached.socket.destroy();fallback.socket.destroy();retry?.socket.destroy();await broker.close();}
});
