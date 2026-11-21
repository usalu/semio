import {test,expect} from "bun:test";
import {createHash} from "node:crypto";
import {spawn} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,writeFileSync,existsSync,rmSync,chmodSync,openSync,closeSync} from "node:fs";
import {join,dirname,resolve} from "node:path";
const readyPath=resolve(import.meta.dir,"../../../🟦️.ts");
const {READY_POLICY_V1:P,createReadyPortsV1,runReadyStageV1,readyPlatformV1}=await import(readyPath);
const {READY_TREE_POLICY_V1:T}=await import(resolve(dirname(readyPath),"🌲️process-tree","🟦️.ts"));
const {READY_CONTAINMENT_POLICY_V1:C,readyOwnerTokenV1}=await import(resolve(dirname(readyPath),"🌲️process-tree","🧹️containment","🟦️.ts"));
const producer=join(import.meta.dir,"./🎞️fixture/🟦️.ts");
const wait=async(check:()=>boolean)=>{for(let i=0;i<350;i++){if(check())return;await Bun.sleep(20);}throw Error("Private stage child did not advance");};
const wrapper=`
 const {spawn}=require("node:child_process"),fs=require("node:fs");
 const [source,end,running]=process.argv.slice(1);
 const producer=spawn(process.execPath,["test",source],{stdio:"inherit"});
 process.once("SIGTERM",()=>{producer.kill();const receipt=JSON.stringify({exit:143,signal:null,spawnFailed:false,endedMs:Date.now()});fs.writeFileSync(end+".stop",fs.readFileSync(running));fs.writeFileSync(end+".copy",receipt);fs.writeFileSync(end,receipt);process.exit(143);});
 setInterval(()=>{},100);
`;
test.skipIf(process.platform==="win32")("physical adopted tools uses one absolute grace under exact descendant EX standing",async()=>{
 const scratch=process.env.SEMIO_TEST_ARTIFACT_DIR!,root=mkdtempSync(join(scratch,"cp-")),directory=join(root,...P.directory),registry=join(root,"r"),file=join(directory,P.inflightName),log=join(directory,"tools.log"),exitFile=join(directory,"tools.exit");
 mkdirSync(directory,{recursive:true});mkdirSync(registry,{mode:0o700});
 const startedMs=Date.now(),owner=readyOwnerTokenV1(root,"tools",startedMs),out=openSync(log,"w");
 const env={...process.env,SEMIO_TEST_ARTIFACT_DIR:root,SEMIO_READY_TEST_PRODUCER:"1",SEMIO_LEASE_OWNER:owner,SEMIO_LEASE_REGISTRY_DIRECTORY:registry},stop=new AbortController();
 let manager:ReturnType<typeof spawn>;
 try{manager=spawn(process.execPath,["-e",wrapper,producer,exitFile,file],{cwd:root,env,stdio:["ignore",out,out],detached:true});}finally{closeSync(out);}
 const childDone=new Promise<void>(done=>manager.once("exit",()=>done())),ttl=setTimeout(()=>stop.abort(),29000);
 try{
  const pid=manager.pid!;
  const launch={version:1,platform:readyPlatformV1(),digest:"f".repeat(64),recipe:"d".repeat(64),prerequisites:[]};
  const identity=createHash("sha256").update(JSON.stringify(["tools",pid,startedMs])).digest("hex");
  const watchdog={version:1,owner:identity,peakMegabytes:1,progressAt:startedMs,stallAt:startedMs,armedAt:startedMs,progressKey:null,phase:null,phaseAt:startedMs,offset:0,seen:[],step:{identity:null,movedAt:startedMs,seen:[]},holderSince:null,holderEvidence:null,semantics:{},cpu:[[pid,0]]};
  writeFileSync(file,JSON.stringify({stage:"tools",pid,startedMs,exitFile,log,launch,watchdog}));
  await wait(()=>existsSync(join(root,"producer-ready")));
  const soft=3600,stage={name:"tools",kind:"repo" as const,arguments:["tools","setup","all"],timeoutMilliseconds:soft,inputs:[],bootstrapInput:false};
  const tree={...T,stallMilliseconds:30000,stallStopMilliseconds:30000,semanticsSources:[]};
  const containment={...C,fastSampleMilliseconds:250,slowSampleMilliseconds:250,fastWindowMilliseconds:30000};
  const watch=(signal:AbortSignal)=>runReadyStageV1({root,environment:env,stage,tree,containment,speed:1,signal,jobs:1,onLine:()=>undefined});
  const first=new AbortController(),handoff=setTimeout(()=>first.abort(),Math.max(0,startedMs+4*soft+320-Date.now()));
  try{
   const detached=await watch(first.signal);
   expect(detached).toMatchObject({detached:true,startedMs});
  }finally{clearTimeout(handoff);}
  const persisted=JSON.parse(readFileSync(file,"utf8")),witness=persisted.watchdog.toolsGrace,counter=witness.counters.find((row:{phase:string})=>row.phase===witness.current);
  expect(persisted.pid).toBe(pid);
  expect(witness.proof.pid).toBe(Number(readFileSync(join(root,"producer-ready"),"utf8")));
  expect(counter.previousAt).toBeGreaterThan(startedMs);
  expect(counter.advancedAt).toBeGreaterThan(counter.previousAt);
  const sampleLimit=2*containment.slowSampleMilliseconds+P.pollMilliseconds;
  await Bun.sleep(sampleLimit+100);
  expect(Date.now()-witness.sampleAt).toBeGreaterThan(sampleLimit);
  const result=await watch(stop.signal);
  const atStop=readFileSync(exitFile+".stop","utf8");
  if(!result.timedOut||result.physicalEnd?.endedMs===undefined)throw Error("Expected owned physical timeout: "+JSON.stringify(result)+" log: "+readFileSync(log,"utf8").slice(-1800));
  expect(result.physicalEnd.endedMs-startedMs).toBeGreaterThanOrEqual(5*soft);
  expect(result.physicalEnd.endedMs-startedMs).toBeLessThan(5*soft+2500);
  expect(result.watchdog?.deadline).toBe("hard-cap");
  expect(result.watchdog?.toolsGraceDenial).toBe("budget-exhausted");
  expect(JSON.parse(atStop)).toMatchObject({stage:"tools",pid,startedMs,stop:"timeout",deadline:"hard-cap",toolsGraceDenial:"budget-exhausted"});
  writeFileSync(file,atStop);
  writeFileSync(exitFile,readFileSync(exitFile+".copy"));
  const recovered=await watch(stop.signal);
  expect(recovered.watchdog).toMatchObject({deadline:"hard-cap",toolsGraceDenial:"budget-exhausted"});
  expect(recovered.physicalEnd).toEqual(result.physicalEnd);
  const fleet=join(root,"fleet"),origin="tools-fixture",endedMs=recovered.physicalEnd!.endedMs;
  const ports=createReadyPortsV1({root,home:root,environment:{SEMIO_FLEET_STORE_ROOT:fleet},origin,group:undefined,stages:[stage],say:()=>undefined});
  ports.timing?.({stage:"tools",startedMs,endedMs,seconds:Math.round((endedMs-startedMs)/1000),exit:recovered.exit,outcome:"stage-timeout",peakMegabytes:recovered.peakMegabytes,attempts:1,digest:recovered.physicalEnd!.digest,endEvidence:recovered.physicalEnd!.endEvidence,watchdog:recovered.watchdog});
  const mirrored=JSON.parse(readFileSync(join(fleet,"v1","origins",origin,P.timingsName),"utf8"));
  expect(mirrored.runs[0]?.watchdog).toEqual(recovered.watchdog);
 }finally{clearTimeout(ttl);stop.abort();if(manager.exitCode===null)try{process.kill(-manager.pid!,"SIGKILL");}catch{}await childDone;rmSync(root,{recursive:true,force:true});}
},35000);
