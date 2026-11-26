/** 🎫️ Contract: the broker starts a unit only when the measured available memory admits it. A unit that does not fit stays queued and starts when availability rises; a warden at guard holds every start whatever the wait; a crate the warden cancelled needs its cancelled megabytes times the factor and twice the wait; while it holds a unit back the broker says so in English and German. Every port is a fake and the clock is fake. */
import {createConnection,type Socket} from "node:net";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {expect,test} from "bun:test";
import {brokerWouldWaitV1,recallProvenanceV1,replayCachedUnitV1} from "../../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️broker/🟦️.ts";
import {ADMISSION_POLICY_V1,sourceKlocV1,startAdmissionBrokerV1,unitKeyV1,type AdmitRequestV1,type RecallRequestV1,type UnitObservationV1,type UnitRecordV1} from "../../../🟦️.ts";
import requests from "../../🧫️fixtures/🔣️.json" with {type:"json"};
import corpus from "./🔣️.json" with {type:"json"};

const MiB=1048576,GiB=1024*MiB,base=requests.requests.valid[0]! as unknown as AdmitRequestV1,pass={id:"wasm-release-00",profile:"release",target:"wasm32-wasip2"};
const unit=(crate:string):AdmitRequestV1=>({...base,crate,package:crate.replaceAll("_","-"),crateTypes:["lib"],target:"host",externs:[],pid:1});
const historyOf=(crates:readonly string[],peakBytes:number)=>new Map<string,readonly UnitObservationV1[]>(crates.map(crate=>[unitKeyV1(unit(crate),pass.profile),[{v:1,key:unitKeyV1(unit(crate),pass.profile),peakRssBytes:peakBytes,outputBytes:0,wallMs:1000,cpuMs:1000,remainingMs:0,atMs:1,lines:0}]]));
const margin=Math.floor(16*GiB*ADMISSION_POLICY_V1.available.marginBasisPoints/10000);

type Client={rows:Record<string,unknown>[];verdict:(count:number)=>Promise<boolean>;send:(row:object)=>void;close:()=>void};
const connect=(endpoint:string,request:AdmitRequestV1|RecallRequestV1):Promise<Client>=>new Promise((done,fail)=>{
 const rows:Record<string,unknown>[]=[],socket:Socket=createConnection(endpoint);let text="";
 const waiters:(()=>void)[]=[];
 socket.on("error",fail);
 socket.on("data",chunk=>{text+=chunk;for(let at=text.indexOf("\n");at>=0;at=text.indexOf("\n")){rows.push(JSON.parse(text.slice(0,at)));text=text.slice(at+1);}for(const wake of waiters.splice(0))wake();});
 socket.on("connect",()=>{socket.write(JSON.stringify(request)+"\n");done({rows,send:row=>{socket.write(JSON.stringify(row)+"\n");},close:()=>socket.destroy(),verdict:count=>new Promise<boolean>((accept,reject)=>{
  const timer=setTimeout(()=>reject(Error("no verdict "+count)),5000),check=()=>{if(rows.length>=count){clearTimeout(timer);accept(rows[count-1]!.admitted as boolean);}else waiters.push(check);};check();
 })});});
});
const settle=()=>new Promise<void>(accept=>setTimeout(accept,150));
const open=(ports:Partial<Parameters<typeof startAdmissionBrokerV1>[0]>,crates:readonly string[],peakBytes=2*GiB)=>startAdmissionBrokerV1({physicalBytes:15*GiB,totalBytes:16*GiB,history:historyOf(crates,peakBytes),...ports}).then(broker=>{broker.registerPass(pass);return broker;});
const resample=(broker:Awaited<ReturnType<typeof open>>)=>broker.observe({nonRustcBytes:0,swapOutBytesPerSecond:null});

test("the broker learns only after confirming the exact local compiler input", async () => {
 const localInputKey="a".repeat(64),verified:UnitRecordV1[]=[],request={...unit("forecast_checked"),localInputKey};
 const broker=await open({available:()=>16*GiB,warden:()=>"calm",cancels:()=>[],onVerifiedLocal:record=>verified.push(record)},[request.crate]);
 try{
  const client=await connect(broker.endpoint,request);
  expect(await client.verdict(1)).toBe(true);
  client.send({v:1,op:"done",exit:0,peakRssBytes:256*MiB,userMs:12,sysMs:3,startMs:1,endMs:16,rmetaMs:null});
  await settle();
  client.close();
  const recall=async(key?:string)=>{
   const query=await connect(broker.endpoint,{v:1,op:"recall",pass:request.pass,crate:request.crate,outDir:request.outDir,extraFilename:request.extraFilename,...key?{localInputKey:key}:{}});
   await query.verdict(1);
   expect(query.rows[0]?.record).toBeDefined();
   query.close();
  };
  await recall("b".repeat(64));
  await recall();
  expect(verified).toEqual([]);
  const ordinaryCargo = {
   [ADMISSION_POLICY_V1.brokerEnvironment]:broker.endpoint,
   [ADMISSION_POLICY_V1.passEnvironment]:request.pass,
  };
  const {v:_version,op:_operation,pass:_pass,pid:_pid,localInputKey:_inputKey,...facts}=request;
  expect(await recallProvenanceV1(ordinaryCargo,facts)).toBeUndefined();
  expect(await replayCachedUnitV1(ordinaryCargo,facts,"cache-key",null)).toBe(false);
  expect(await brokerWouldWaitV1(ordinaryCargo,facts)).toBeUndefined();
  expect(await brokerWouldWaitV1(ordinaryCargo,facts,localInputKey)).toBe(false);
  expect(await recallProvenanceV1(ordinaryCargo,facts,localInputKey)).toMatchObject({peakRssBytes:256*MiB});
  await recall(localInputKey);
  expect(await replayCachedUnitV1({...ordinaryCargo,SEMIO_UNIT_CACHE_BROKER_REPLAY:"on"},facts,"c".repeat(64),null)).toBe(true);
  expect(verified.map(row=>[row.key,row.peakRssBytes])).toEqual([[unitKeyV1(request,pass.profile),256*MiB]]);
 }finally{await broker.close();}
});

test("compiler reports remember different peaks for same-named build scripts by package and version",async()=>{
 const scripts=corpus.scripts.map(row=>({...unit("build_script_build"),package:row.package,version:row.version,crateTypes:["bin"],outDir:"/out/"+row.package+"-"+row.version}));
 const history=new Map<string,readonly UnitObservationV1[]>();
 for(const [index,request] of scripts.entries()){
  const key=unitKeyV1(request,pass.profile),peakRssBytes=corpus.scripts[index]!.peakMiB*MiB;
  history.set(key,[{v:1,key,peakRssBytes,outputBytes:0,wallMs:1000,cpuMs:1000,remainingMs:0,atMs:1,lines:0}]);
 }
 expect(history.size).toBe(scripts.length);
 for(const [index,request] of scripts.entries()){
  const broker=await open({history,available:()=>16*GiB,warden:()=>"calm",cancels:()=>scripts.filter(other=>other!==request).map(other=>({command:"rustc --crate-name build_script_build --edition=2021",buildScriptOutput:other.outDir,megabytes:3000}))},[]);
  try{
   const client=await connect(broker.endpoint,request);
   expect(await client.verdict(1)).toBe(true);
   expect(broker.stats().predictedRunningBytes).toBe(Math.ceil(corpus.scripts[index]!.peakMiB*MiB*ADMISSION_POLICY_V1.headroom));
   client.close();
  }finally{await broker.close();}
 }
});

test("an exact cancellation record still raises the forecast of the same build script",async()=>{
 const request={...unit("build_script_build"),package:"beta",version:"2.0.0",outDir:"/out/beta-2.0.0"};
 const broker=await open({available:()=>16*GiB,warden:()=>"calm",cancels:()=>[{command:"rustc --crate-name build_script_build --edition=2021",buildScriptOutput:request.outDir,megabytes:3000}]},[]);
 try{
  const client=await connect(broker.endpoint,request);
  expect(await client.verdict(1)).toBe(true);
  expect(broker.stats().predictedRunningBytes).toBe(3750*MiB);
  client.close();
 }finally{await broker.close();}
});

test("a build script sizes its Rust mounts and retains the directory estimate if its sources are uncertain",async()=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"script-"));
 try{
  for(const [name,source] of Object.entries(corpus.source.files))writeFileSync(join(root,name),source);
  const entry=join(root,"build.rs");
  expect(await sourceKlocV1(entry,root)*1000).toBe(corpus.source.mountedLines);
  writeFileSync(entry,"#[cfg(unconfirmed)]\nfn main() {}\n");
  expect(await sourceKlocV1(entry,root)*1000).toBe(corpus.source.fallbackLines);
 }finally{rmSync(root,{recursive:true,force:true});}
});

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

test("a warden at guard or danger holds every start, even if available memory is unknown",async()=>{
 let clock=1_000_000,level:"calm"|"guard"|"danger"="guard";const broker=await open({now:()=>clock,available:()=>level==="danger"?undefined:16*GiB,warden:()=>level,cancels:()=>[]},["semio_a"]);
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

test("an idle broker holds low memory past its wait bound during active swap and resumes when it subsides",async()=>{
 let clock=1_000_000;const lines:string[]=[],broker=await open({now:()=>clock,available:()=>GiB,warden:()=>"calm",cancels:()=>[],log:line=>lines.push(line)},["semio_a"]);
 try{
  const client=await connect(broker.endpoint,unit("semio_a"));
  expect(await client.verdict(1)).toBe(false);
  clock+=ADMISSION_POLICY_V1.available.waitMilliseconds;
  broker.observe({nonRustcBytes:0,swapOutBytesPerSecond:ADMISSION_POLICY_V1.swapOutBytesPerSecond+1});
  expect(broker.stats()).toMatchObject({running:0,waiting:1});
  expect(lines.at(-1)).toContain("host is actively swapping");
  resample(broker);
  expect(await client.verdict(2)).toBe(true);
  client.close();
 }finally{await broker.close();}
});

test("the low-memory wait bound needs a calm warden reading to release a unit",async()=>{
 let clock=1_000_000,level:"calm"|undefined;const broker=await open({now:()=>clock,available:()=>GiB,warden:()=>level,cancels:()=>[]},["semio_a"]);
 try{
  const client=await connect(broker.endpoint,unit("semio_a"));
  expect(await client.verdict(1)).toBe(false);
  clock+=ADMISSION_POLICY_V1.available.waitMilliseconds;
  resample(broker);
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
