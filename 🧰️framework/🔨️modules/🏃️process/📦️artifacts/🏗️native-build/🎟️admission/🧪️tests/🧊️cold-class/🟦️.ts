/** 🧊️ Contract: a unit without measurement is estimated by its class (workspace or external), an external unit never above its budget, the size of a unit counts its own sources and never those of nested packages, and a cold estimate above the parent grant never refuses the unit: it runs alone in the whole grant. The pure rules equal an independent Python oracle; a measured estimate above the grant is still refused. */
import {spawnSync} from "node:child_process";
import {createConnection} from "node:net";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {expect,test} from "bun:test";
import {requireTestToolV1} from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {ADMISSION_POLICY_V1,externalSourceV1,predictPeakBytesV1,sourceKlocV1,startAdmissionBrokerV1,unitKeyV1,type AdmitRequestV1,type GrantGrowthPortV1,type UnitObservationV1} from "../../🟦️.ts";
import requests from "../🧫️fixtures/🔣️.json" with {type:"json"};
import corpus from "./🔣️.json";

const MiB=1<<20,base=requests.requests.valid[0]! as unknown as AdmitRequestV1,pass={id:"host-release-00",profile:"release",target:"host"};
const sizeOf=async(tree:(typeof corpus.trees)[number])=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"cold-tree-"));
 try{
  for(const [path,lines,mods] of tree.files as [string,number,string[]][]){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),path.endsWith(".rs")?mods.map(mod=>`mod ${mod};\n`).join("")+"\n".repeat(lines-mods.length):"x");}
  return Math.round(await sourceKlocV1(join(root,"lib.rs"))*1000);
 }finally{rmSync(root,{recursive:true,force:true});}
};

const estimate=(item:(typeof corpus.estimates)[number])=>predictPeakBytesV1([],{kloc:item.kloc,component:null,external:item.external,...item.budgetBytes===null?{}:{budgetBytes:item.budgetBytes}});

test("the corpus states the policy constants of the schema",()=>{
 const policy=ADMISSION_POLICY_V1;
 expect(corpus.policy).toEqual({externalBaseBytes:policy.coldExternalBaseBytes,externalBytesPerKloc:policy.coldExternalBytesPerKloc,floorBytes:policy.floorBytes,workspaceBaseBytes:policy.coldBaseBytes,workspaceBytesPerKloc:policy.coldBytesPerKloc,markers:policy.externalSourceMarkers,skipDirectories:policy.sourceSkipDirectories,manifestFile:policy.packageManifestFile});
});

test("cold estimates, the external class and the own source size equal the independent Python oracle",async()=>{
 const result=spawnSync(requireTestToolV1("python"),["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"});
 expect(result.status,result.stderr).toBe(0);
 expect(JSON.parse(result.stdout)).toEqual({estimates:corpus.estimates.map(estimate),sources:corpus.sources.map(row=>externalSourceV1(row.source)),trees:await Promise.all(corpus.trees.map(sizeOf))});
 for(const row of corpus.sources)expect(externalSourceV1(row.source),row.source).toBe(row.external);
});

test("an external estimate covers its measured peak, fits the smallest grant and never exceeds its budget",()=>{
 for(const item of corpus.estimates.filter(row=>row.external&&row.budgetBytes!==null)){
  expect(estimate(item),item.id).toBeLessThanOrEqual(item.budgetBytes!);
  if(item.measuredPeakBytes!==null)expect(estimate(item),item.id).toBeGreaterThan(item.measuredPeakBytes);
 }
 expect(estimate(corpus.estimates.find(row=>row.id==="libc-large-flat-crate")!)).toBeLessThan(1536*MiB);
});

const connect=(endpoint:string,request:Record<string,unknown>)=>{
 const socket=createConnection(endpoint),queue:Record<string,unknown>[]=[],readers:((reply:Record<string,unknown>)=>void)[]=[];
 let partial="";
 socket.setTimeout(20000);
 socket.on("timeout",()=>socket.destroy(new Error("Admission reply timed out")));
 socket.on("error",()=>undefined);
 socket.on("connect",()=>socket.write(JSON.stringify(request)+"\n"));
 socket.on("data",chunk=>{partial+=chunk;let next:number;while((next=partial.indexOf("\n"))>=0){const reply=JSON.parse(partial.slice(0,next));partial=partial.slice(next+1);const reader=readers.shift();if(reader)reader(reply);else queue.push(reply);}});
 return {socket,next:()=>queue.length?Promise.resolve(queue.shift()!):new Promise<Record<string,unknown>>((accept,reject)=>{readers.push(accept);socket.once("error",reject);})};
};

const admit=async(row:{parentMiB:number;source:string;measuredMiB:number|null;overheadMiB?:number;growth?:GrantGrowthPortV1;lines?:number})=>{
 const request:AdmitRequestV1={...base,crate:"unit_a",package:"unit-a",crateTypes:["lib"],target:"host",externs:[],pid:1,pass:pass.id,source:row.source,primary:false},key=unitKeyV1(request,pass.profile);
 const history=new Map<string,readonly UnitObservationV1[]>(row.measuredMiB===null?[]:[[key,[{v:1,key,peakRssBytes:row.measuredMiB*MiB,outputBytes:0,wallMs:1000,cpuMs:1000,remainingMs:0,atMs:1,lines:row.lines??50_000}]]]);
 const broker=await startAdmissionBrokerV1({physicalBytes:16*1024*MiB,history,parentMemoryAllowanceBytes:row.parentMiB*MiB,...row.growth?{growth:row.growth}:{},available:()=>undefined,warden:()=>"calm",cancels:()=>[]});
 broker.registerPass(pass);
 if(row.overheadMiB)broker.observe({nonRustcBytes:row.overheadMiB*MiB,swapOutBytesPerSecond:null});
 const client=connect(broker.endpoint,request);
 try{let reply=await client.next();if(reply.op==="verdict"&&reply.admitted===false)reply=await client.next();return {reply,predicted:broker.stats().predictedRunningBytes};}finally{client.socket.destroy();await broker.close();}
};

test("a cold unit above the parent grant runs alone in the whole grant instead of being refused; a measured one is still refused",async()=>{
 const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"cold-class-"));
 try{
  const workspace=join(directory,"ws","lib.rs"),external=join(directory,"registry","src","idx","syn-2","lib.rs");
  mkdirSync(join(directory,"ws"),{recursive:true});mkdirSync(join(directory,"registry","src","idx","syn-2"),{recursive:true});
  writeFileSync(workspace,"\n".repeat(50_000));writeFileSync(external,"\n".repeat(50_000));
  const cold=await admit({parentMiB:1536,source:workspace,measuredMiB:null});
  expect(cold.reply).toMatchObject({op:"verdict",admitted:true});
  expect(cold.predicted).toBe(1536*MiB);
  const tiny=await admit({parentMiB:1024,source:external,measuredMiB:null});
  expect(tiny.reply).toMatchObject({op:"verdict",admitted:true});
  expect(tiny.predicted).toBe(Math.ceil(ADMISSION_POLICY_V1.coldExternalBaseBytes+ADMISSION_POLICY_V1.coldExternalBytesPerKloc*50));
  const measured=await admit({parentMiB:1536,source:workspace,measuredMiB:2048});
  expect(measured.reply).toMatchObject({op:"refused",reason:"parent-memory"});
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a unit that fits the grant but not the driver's own memory beside it waits a bounded time for growth, then runs alone instead of waiting for good",async()=>{
 for(const overheadMiB of [1161,1700]){
  const asks:number[]=[],growth:GrantGrowthPortV1={local:()=>({origin:"h",totalMegabytes:32768,floorMegabytes:2048,interactiveReserveMegabytes:2048,budgetBasisPoints:10000,availableMegabytes:20000}),fleet:()=>[],acquire:async ask=>{asks.push(ask.waitMilliseconds);return {granted:false,reason:"timeout"};}};
  const result=await admit({parentMiB:1536,source:base.source,measuredMiB:330,lines:0,overheadMiB,growth});
  expect(result.reply,String(overheadMiB)).toMatchObject({op:"verdict",admitted:true});
  expect(asks).toEqual([ADMISSION_POLICY_V1.growth.overheadWaitMilliseconds]);
  expect(result.predicted).toBe(Math.ceil(1.15*330*MiB));
 }
 expect(ADMISSION_POLICY_V1.growth.overheadWaitMilliseconds).toBeLessThan(ADMISSION_POLICY_V1.growth.waitMilliseconds);
});

test("a queued unit that stalls on the driver's memory with nothing running is rescued by bounded growth, then runs alone",async()=>{
 const big:AdmitRequestV1={...base,crate:"big_a",package:"big-a",crateTypes:["lib"],target:"host",externs:[],pid:1,pass:pass.id},small:AdmitRequestV1={...base,crate:"small_b",package:"small-b",crateTypes:["lib"],target:"host",externs:[],pid:2,pass:pass.id};
 const seed=(request:AdmitRequestV1,peakMiB:number)=>{const key=unitKeyV1(request,pass.profile);return [key,[{v:1 as const,key,peakRssBytes:peakMiB*MiB,outputBytes:0,wallMs:1000,cpuMs:1000,remainingMs:0,atMs:1,lines:0}]] as const;};
 const asks:number[]=[],growth:GrantGrowthPortV1={local:()=>({origin:"h",totalMegabytes:32768,floorMegabytes:2048,interactiveReserveMegabytes:2048,budgetBasisPoints:10000,availableMegabytes:20000}),fleet:()=>[],acquire:async ask=>{asks.push(ask.waitMilliseconds);return {granted:false,reason:"timeout"};}};
 const broker=await startAdmissionBrokerV1({physicalBytes:16*1024*MiB,history:new Map([seed(big,1100),seed(small,330)]),parentMemoryAllowanceBytes:1536*MiB,growth,waits:{publishMilliseconds:20,progressMilliseconds:0},available:()=>undefined,warden:()=>"calm",cancels:()=>[]});
 broker.registerPass(pass);
 const first=connect(broker.endpoint,big),second=connect(broker.endpoint,small);
 try{
  expect(await first.next()).toMatchObject({op:"verdict",admitted:true});
  expect(await second.next()).toMatchObject({op:"verdict",admitted:false});
  broker.observe({nonRustcBytes:1161*MiB,swapOutBytesPerSecond:null});
  first.socket.write(JSON.stringify({v:1,op:"done",exit:0,peakRssBytes:1100*MiB,userMs:1,sysMs:1,startMs:1,endMs:2,rmetaMs:null})+"\n");
  expect(await second.next()).toMatchObject({op:"verdict",admitted:true});
  expect(asks).toEqual([ADMISSION_POLICY_V1.growth.overheadWaitMilliseconds]);
 }finally{first.socket.destroy();second.socket.destroy();await broker.close();}
});
