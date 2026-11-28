import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {existsSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {eventuallyV1,startClusterV1} from "../../../../🧪️tests/🧰️cluster/🟦️.ts";
import {remoteExecutionHostPropertiesV1} from "../../../../🟦️.ts";
import type {OffloadSessionV1} from "../../../🟦️.ts";
import {offloadCheckoutTestV1} from "../../🔌️client/🟦️.ts";
import {CHECKOUT_TEST_POLICY_V1,type CheckoutTestCutV1} from "../../📐️policy/🟦️.ts";
import type {CheckoutTestReplicaV1} from "../../🧭️replica/🟦️.ts";
import {createCheckoutTestsV1} from "../../👷️worker/🟦️.ts";

const sha=(text:string):string=>createHash("sha256").update(text).digest("hex"),cut=(bytes:number):CheckoutTestCutV1=>({"g1-m01":{segment:2,bytes}});
const scratch:string[]=[],temporary=():string=>{const path=mkdtempSync(join(tmpdir(),"semio-checkout-scheduler-"));scratch.push(path);return path;};
const replica=(origin:string,overrides:Partial<CheckoutTestReplicaV1>={}):CheckoutTestReplicaV1=>({origin,cut:async()=>cut(500),ready:async()=>({ready:true,detail:"ready"}),digest:()=>sha("file"),...overrides});
const RESULT=(file:string)=>JSON.stringify({version:1,status:"passed",seconds:0.1,bail:false,counts:{passed:1,failed:0,skipped:0,notRun:0},files:[{file,mode:"plain",table:null,suite:"plain",status:"passed",seconds:0.1,pass:1,fail:0}]});
const passing=(_:string,file:string)=>[process.execPath,"-e",`process.stderr.write('ran '+${JSON.stringify(file)}+'\\n');process.stdout.write(${JSON.stringify(RESULT(file))});`] as readonly string[];
const capability=(origin:string,overrides:Partial<CheckoutTestReplicaV1>,argv:(repo:string,file:string)=>readonly string[]=passing)=>createCheckoutTestsV1({root:temporary(),replica:replica(origin,overrides),repo:process.execPath,environment:{...process.env},argv,waitMilliseconds:0,pollMilliseconds:1,sleep:async()=>{}});
const properties=[...remoteExecutionHostPropertiesV1(),{name:CHECKOUT_TEST_POLICY_V1.property.format,value:CHECKOUT_TEST_POLICY_V1.property.formatValue}];

test("the real scheduler places a test file only on a worker that offers checkout tests, moves it on from a worker that is not Ready, runs it on the one that is, and stops its process when the request is cancelled",async()=>{
 const cluster=await startClusterV1({workers:0}),session:OffloadSessionV1={endpoint:cluster.server.grpcUrl,content:cluster.content,instance:"main",token:undefined,executionToken:undefined,workspace:undefined};
 let peers:readonly string[]|undefined;
 const ask=(file:string,signal?:AbortSignal,onOutput?:(text:string)=>void)=>offloadCheckoutTestV1({root:temporary(),memoRoot:temporary(),environment:{},file,budgetSeconds:30,language:"en",retryMilliseconds:0,...signal?{signal}:{},...onOutput?{onOutput}:{},ports:{replica:replica("g1-server",peers===undefined?{}:{peers:async()=>peers!}),openSession:async()=>session}});
 try{
  expect(await ask("a/ok.ts"),"no machine at all").toMatchObject({kind:"local",reason:"no-idle-machine"});
  cluster.startWorker({name:"plain",origin:"g1-m04",properties:remoteExecutionHostPropertiesV1()});
  await eventuallyV1(()=>cluster.events.filter(line=>line.startsWith("worker-joined")).length===1);
  expect(await ask("a/ok.ts"),"a worker that offers no checkout tests").toMatchObject({kind:"local",reason:"no-idle-machine"});
  const pidFile=join(temporary(),"pid");
  cluster.startWorker({name:"behind",origin:"g1-m05",properties:[...properties,{name:"semio-pin",value:"g1-m05"}],checkoutTests:capability("g1-m05",{cut:async()=>cut(1)})});
  cluster.startWorker({name:"ready",origin:"g1-m06",properties:[...properties,{name:"semio-pin",value:"g1-m06"}],checkoutTests:capability("g1-m06",{},(_,file)=>[process.execPath,"-e",`const fs=require('fs');if(${JSON.stringify(file)}.includes('slow')){fs.writeFileSync(${JSON.stringify(pidFile)},String(process.pid));setInterval(()=>{},1000);}else{process.stderr.write('ran '+${JSON.stringify(file)}+'\\n');process.stdout.write(${JSON.stringify(RESULT("a/ok.ts"))});}`])});
  await eventuallyV1(()=>cluster.events.filter(line=>line.startsWith("worker-joined")).length===3);
  peers=["g1-m05"];
  expect(await ask("a/ok.ts"),"the only machine that is offered is behind: it refuses and the scheduler answers at once").toMatchObject({kind:"local",reason:"no-idle-machine"});
  peers=["g1-m05","g1-m06"];
  const done=await ask("a/ok.ts");
  expect(done.kind,JSON.stringify(done)+cluster.events.join("\n")).toBe("done");
  if(done.kind==="done"){expect(done.origin).toBe("g1-m06");expect(done.exitCode).toBe(0);expect(done.stderr).toContain("ran a/ok.ts");expect(done.result).toMatchObject({status:"passed"});}
  const live:string[]=[],streamed=await ask("a/ok.ts",undefined,text=>{live.push(text);});
  expect(streamed.kind).toBe("done");
  if(streamed.kind==="done"){expect(streamed.streamed,"the pieces of the output went through the real cache server").toBe(true);expect(live.join("")).toContain("ran a/ok.ts");expect(streamed.stderr).toBe("");}
  expect(cluster.events.some(line=>line.startsWith("refused")&&line.includes("not-ready")),"the worker that is behind refused the file and the scheduler moved it on: "+cluster.events.join(" | ")).toBe(true);
  const controller=new AbortController(),pending=ask("a/slow.ts",controller.signal);
  await eventuallyV1(()=>existsSync(pidFile),30000,"the test process did not start on the worker");
  const pid=Number(readFileSync(pidFile,"utf8"));
  controller.abort();
  expect(await pending).toEqual({kind:"local",reason:"cancelled",detail:""});
  await eventuallyV1(()=>{try{process.kill(pid,0);return false;}catch{return true;}},30000,"the test process of the worker is still running after the cancel");
 }finally{
  await cluster.stop();
  for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});
 }
},120000);
