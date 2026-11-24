/** 🎫️ Contract: the broker starts a unit only when the measured available memory admits it. A unit that does not fit stays queued and starts when availability rises; a warden at guard holds every start whatever the wait; a crate the warden cancelled needs its cancelled megabytes times the factor and twice the wait; while it holds a unit back the broker says so in English and German. Every port is a fake and the clock is fake. */
import {createConnection,type Socket} from "node:net";
import {expect,test} from "bun:test";
import {ADMISSION_POLICY_V1,startAdmissionBrokerV1,unitKeyV1,type AdmitRequestV1,type UnitObservationV1} from "../../../🟦️.ts";
import requests from "../../🧫️fixtures/🔣️.json" with {type:"json"};

const MiB=1048576,GiB=1024*MiB,base=requests.requests.valid[0]! as unknown as AdmitRequestV1,pass={id:"wasm-release-00",profile:"release",target:"wasm32-wasip2"};
const unit=(crate:string):AdmitRequestV1=>({...base,crate,package:crate.replaceAll("_","-"),crateTypes:["lib"],target:"host",externs:[],pid:1});
const historyOf=(crates:readonly string[],peakBytes:number)=>new Map<string,readonly UnitObservationV1[]>(crates.map(crate=>[unitKeyV1(unit(crate),pass.profile),[{v:1,key:unitKeyV1(unit(crate),pass.profile),peakRssBytes:peakBytes,outputBytes:0,wallMs:1000,cpuMs:1000,remainingMs:0,atMs:1}]]));
const margin=Math.floor(16*GiB*ADMISSION_POLICY_V1.available.marginBasisPoints/10000);

type Client={rows:Record<string,unknown>[];verdict:(count:number)=>Promise<boolean>;close:()=>void};
const connect=(endpoint:string,request:AdmitRequestV1):Promise<Client>=>new Promise((done,fail)=>{
 const rows:Record<string,unknown>[]=[],socket:Socket=createConnection(endpoint);let text="";
 const waiters:(()=>void)[]=[];
 socket.on("error",fail);
 socket.on("data",chunk=>{text+=chunk;for(let at=text.indexOf("\n");at>=0;at=text.indexOf("\n")){rows.push(JSON.parse(text.slice(0,at)));text=text.slice(at+1);}for(const wake of waiters.splice(0))wake();});
 socket.on("connect",()=>{socket.write(JSON.stringify(request)+"\n");done({rows,close:()=>socket.destroy(),verdict:count=>new Promise<boolean>((accept,reject)=>{
  const timer=setTimeout(()=>reject(Error("no verdict "+count)),5000),check=()=>{if(rows.length>=count){clearTimeout(timer);accept(rows[count-1]!.admitted as boolean);}else waiters.push(check);};check();
 })});});
});
const settle=()=>new Promise<void>(accept=>setTimeout(accept,150));
const open=(ports:Partial<Parameters<typeof startAdmissionBrokerV1>[0]>,crates:readonly string[],peakBytes=2*GiB)=>startAdmissionBrokerV1({physicalBytes:15*GiB,totalBytes:16*GiB,history:historyOf(crates,peakBytes),...ports}).then(broker=>{broker.registerPass(pass);return broker;});
const resample=(broker:Awaited<ReturnType<typeof open>>)=>broker.observe({nonRustcBytes:0,swapOutBytesPerSecond:null});

test("a unit that does not fit is not started, and starts when the availability rises",async()=>{
 let available=3*GiB;const broker=await open({now:()=>1_000_000,available:()=>available,warden:()=>"calm",cancels:()=>[]},["semio_a"]);
 try{
  const client=await connect(broker.endpoint,unit("semio_a"));
  expect(await client.verdict(1)).toBe(false);
  await settle();resample(broker);await settle();
  expect(broker.stats()).toMatchObject({running:0,waiting:1});
  expect(client.rows).toHaveLength(1);
  available=2*GiB*1.15+margin+1;resample(broker);
  expect(await client.verdict(2)).toBe(true);
  expect(broker.stats()).toMatchObject({running:1,waiting:0});
  client.close();
 }finally{await broker.close();}
});

test("the broker re-evaluates on its own poll timer while a unit is held back",async()=>{
 let available=1*GiB;const broker=await open({available:()=>available,warden:()=>"calm",cancels:()=>[]},["semio_a"]);
 try{
  const client=await connect(broker.endpoint,unit("semio_a"));
  expect(await client.verdict(1)).toBe(false);
  available=16*GiB;
  expect(await client.verdict(2)).toBe(true);
  client.close();
 }finally{await broker.close();}
});

test("a warden at guard holds every start, whatever the wait; calm releases it",async()=>{
 let clock=1_000_000,level:"calm"|"guard"|"danger"="guard";const broker=await open({now:()=>clock,available:()=>16*GiB,warden:()=>level,cancels:()=>[]},["semio_a"]);
 try{
  const client=await connect(broker.endpoint,unit("semio_a"));
  expect(await client.verdict(1)).toBe(false);
  clock+=10*ADMISSION_POLICY_V1.available.waitMilliseconds;resample(broker);await settle();
  expect(broker.stats()).toMatchObject({running:0,waiting:1});
  level="danger";resample(broker);await settle();
  expect(broker.stats()).toMatchObject({running:0,waiting:1});
  level="calm";resample(broker);
  expect(await client.verdict(2)).toBe(true);
  client.close();
 }finally{await broker.close();}
});

test("a unit that never fits starts after the wait because nothing runs, and unknown availability never blocks",async()=>{
 let clock=1_000_000;const broker=await open({now:()=>clock,available:()=>1*GiB,warden:()=>"calm",cancels:()=>[]},["semio_a"]);
 try{
  const client=await connect(broker.endpoint,unit("semio_a"));
  expect(await client.verdict(1)).toBe(false);
  clock+=ADMISSION_POLICY_V1.available.waitMilliseconds-1;resample(broker);await settle();
  expect(broker.stats().running).toBe(0);
  clock+=1;resample(broker);
  expect(await client.verdict(2)).toBe(true);
  client.close();
 }finally{await broker.close();}
 const blind=await open({available:()=>undefined,warden:()=>undefined,cancels:()=>[]},["semio_a"]);
 try{const client=await connect(blind.endpoint,unit("semio_a"));expect(await client.verdict(1)).toBe(true);client.close();}finally{await blind.close();}
});

test("the unrealized remainder of a running unit counts against the room of the next",async()=>{
 const broker=await open({now:()=>1_000_000,available:()=>margin+Math.ceil(2*GiB*1.15)*1.5,warden:()=>"calm",cancels:()=>[]},["semio_a","semio_b"]);
 try{
  const first=await connect(broker.endpoint,unit("semio_a"));
  expect(await first.verdict(1)).toBe(true);
  const second=await connect(broker.endpoint,unit("semio_b"));
  expect(await second.verdict(1)).toBe(false);
  await settle();
  expect(broker.stats()).toMatchObject({running:1,waiting:1});
  first.close();second.close();
 }finally{await broker.close();}
});

test("a crate the warden cancelled needs its cancelled megabytes times the factor and twice the wait",async()=>{
 let clock=1_000_000,available=margin+2000*MiB;const asked:number[]=[];
 const cancels=(since:number)=>{asked.push(since);return [{command:"rustc --crate-name semio_x --edition 2024 src/lib.rs",megabytes:2000},{command:"rustc --crate-name other --edition 2024",megabytes:9000}];};
 const broker=await open({now:()=>clock,available:()=>available,warden:()=>"calm",cancels},["semio_x","semio_x_other"],1*GiB);
 try{
  const other=await connect(broker.endpoint,unit("semio_x_other"));
  expect(await other.verdict(1)).toBe(true);
  expect(asked[0]).toBe(1_000_000-ADMISSION_POLICY_V1.available.cancelledMemoryMilliseconds);
  other.close();
  const client=await connect(broker.endpoint,unit("semio_x"));
  expect(await client.verdict(1)).toBe(false);
  available=margin+2000*MiB+1;resample(broker);await settle();
  expect(broker.stats().running).toBe(0);
  available=margin+2500*MiB-1;resample(broker);await settle();
  expect(broker.stats().running).toBe(0);
  available=margin+2500*MiB;resample(broker);
  expect(await client.verdict(2)).toBe(true);
  client.close();
 }finally{await broker.close();}
 available=margin;clock=1_000_000;
 const slow=await open({now:()=>clock,available:()=>available,warden:()=>"calm",cancels},["semio_x"],1*GiB);
 try{
  const client=await connect(slow.endpoint,unit("semio_x"));
  expect(await client.verdict(1)).toBe(false);
  clock+=ADMISSION_POLICY_V1.available.waitMilliseconds;resample(slow);await settle();
  expect(slow.stats().running).toBe(0);
  clock+=ADMISSION_POLICY_V1.available.waitMilliseconds-1;resample(slow);await settle();
  expect(slow.stats().running).toBe(0);
  clock+=1;resample(slow);
  expect(await client.verdict(2)).toBe(true);
  client.close();
 }finally{await slow.close();}
});

test("a held-back unit is announced once in English and German with its numbers, and a warden hold says so",async()=>{
 let clock=1_000_000,level:"calm"|"guard"="calm";const lines:string[]=[];
 const broker=await open({now:()=>clock,available:()=>2*GiB,warden:()=>level,cancels:()=>[],log:line=>lines.push(line)},["semio_a"]);
 try{
  const client=await connect(broker.endpoint,unit("semio_a"));
  expect(await client.verdict(1)).toBe(false);
  resample(broker);resample(broker);await settle();
  expect(lines).toHaveLength(1);
  expect(lines[0]).toContain("unit semio_a is held back");
  expect(lines[0]).toContain("Einheit semio_a wird zurückgehalten");
  expect(lines[0]).toContain(`${Math.round(margin/MiB)} MB`);
  expect(lines[0]).toContain("2048 MB available");
  clock+=60_000;level="guard";resample(broker);await settle();
  expect(lines).toHaveLength(2);
  expect(lines[1]).toContain("host warden is at level guard");
  expect(lines[1]).toContain("waited 60 s");
  client.close();
 }finally{await broker.close();}
});
