import{test,expect}from"bun:test";
import {testPythonV1} from "../../../🧰️toolchain/🟦️.ts";
import{join,dirname,basename}from"node:path";
import{createIssuedResourceControlV1,requireIssuedScriptInvocationV1,withIssuedResourceOperationV1,type ScriptInvocation}from"../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{acquireResourceLease,resourceLeasePathV1}from"../../../../🔒️leases/🟦️.ts";
import{captureOwnedCommandV1}from"../../../../🎛️owned-execution/🟦️.ts";
import{withBuildTestInvocationV1,registerBuildTestCleanupV1,registerBuildTestLawV1}from"../../🟦️.ts";
import corpus from"../🔣️.json"with{type:"json"};
const requested=process.env.SEMIO_RECEIVER_CASE,kind=requested?.replace(/^registered-/u,""),directory=process.env.SEMIO_RECEIVER_DIRECTORY,marker=Error("receiver primary marker");
if(!corpus.cases.some(row=>row.name===requested)||!directory)throw Error("Exact owned receiver child fixture required");
const emit=(phase:string,facts:object={})=>console.log("[TRACE] Receiver event "+JSON.stringify({phase,...facts})),sleep=(ms:number)=>new Promise<void>(done=>setTimeout(done,ms));let released=false,firstPolicy:unknown,firstSignal:unknown;
const suite=process.env.SEMIO_RECEIVER_SUITE;let shared:{settled:Promise<void>;run:<T>(body:(value:{calls:number})=>Promise<T>)=>Promise<T>}|undefined;
registerBuildTestCleanupV1(async()=>{emit("cleanup",{released});let refused=false;try{await withBuildTestInvocationV1(async()=>{emit("late-effect");});}catch{refused=true;}emit(refused?"late-birth-refused":"late-birth-accepted");if(suite){const api=await import("../../🟦️.ts");let late=false;try{api.registerBuildTestFixtureV1(async()=>{emit("late-effect");});}catch{late=true;}emit(late?"late-suite-refused":"late-effect");}});
for(let index=0;index<(kind==="normal"?2:1);index++){const name="actual receiver "+kind+" "+index,law=(supplied?:ScriptInvocation)=>{
 let accept!:()=>void,refuse!:(error:unknown)=>void;const entered=new Promise<void>((resolve,reject)=>{accept=resolve;refuse=reject;});void entered.catch(()=>{});
 const execute=async(original:ScriptInvocation)=>{
  requireIssuedScriptInvocationV1(original);const facts=original.capabilities as{artifacts?:unknown};if(typeof facts.artifacts!=="string"||dirname(directory)!==facts.artifacts||!basename(directory).startsWith("receiver-"))throw Error("Authored receiver fixture root required");
  if(suite){const api=await import("../../🟦️.ts"),register=Reflect.get(api,"registerBuildTestFixtureV1");expect(typeof register).toBe("function");shared??=register(async(root:ScriptInvocation,retain:(value:{calls:number})=>Promise<void>)=>{requireIssuedScriptInvocationV1(root);expect(root).not.toBe(original);const suiteControl=createIssuedResourceControlV1(root,"fixture:receiver-suite",15000),suiteLease=await acquireResourceLease({directory,resource:"receiver-suite",mode:"exclusive",signal:suiteControl.signal,control:suiteControl,maximumDatabaseBytes:65536});emit("suite-open");try{if(suite==="early"){void retain({calls:0});return;}await retain({calls:0});emit("suite-stop");if(suite==="error"||suite==="undefined"){throw suite==="error"?marker:undefined;}}finally{suiteLease.release();emit("suite-released");}});if(index===0)void shared!.settled.catch(error=>emit("primary",{preserved:error===(suite==="undefined"?undefined:marker)}));await shared!.run(async value=>{value.calls++;expect(value.calls).toBe(index+1);emit("suite-use",{calls:value.calls});});if(index===1&&(suite==="normal"||suite==="detached"))void shared!.run(async()=>{await sleep(corpus.drainMilliseconds);emit("suite-user-drained");if(suite==="detached")throw marker;}).catch(error=>emit("primary",{preserved:error===marker}));}
  emit("alias",{samePolicy:index===0||firstPolicy===original.policy,differentSignal:index===0||firstSignal!==original.control.signal});firstPolicy=original.policy;firstSignal=original.control.signal;
  const control=createIssuedResourceControlV1(original,"fixture:build-receiver",15000),lease=await acquireResourceLease({directory,resource:"receiver-gate",mode:"exclusive",signal:control.signal,control,maximumDatabaseBytes:65536});released=false;emit("lease-held");
  const python=testPythonV1()??Bun.which("python");if(!python)throw Error("Independent Python SQLite oracle required");
  try{const value=await captureOwnedCommandV1(python,["-c","import sqlite3,sys,json\nc=sqlite3.connect('file:'+sys.argv[1]+'?mode=rw',uri=True,timeout=0)\ntry:\n c.execute('BEGIN EXCLUSIVE');print(json.dumps(False))\nexcept sqlite3.OperationalError as e:\n if str(e)!='database is locked':raise\n print(json.dumps(True))\nfinally:c.close()",resourceLeasePathV1(directory,"receiver-gate")],directory,{control,output:{maximumBytes:4096,maximumLines:16}});expect(JSON.parse(value)).toBe(true);emit("independent-ex-blocked");}catch(error){lease.release();throw error;}
  const gate=withIssuedResourceOperationV1(control,async()=>{try{if(kind==="timeout"||kind==="funded"){await new Promise<void>(done=>{const timer=setTimeout(done,corpus.fallbackMilliseconds),abort=()=>{clearTimeout(timer);done();};if(kind==="funded"){original.control.signal.addEventListener("abort",abort,{once:true});if(original.control.signal.aborted)abort();}});emit(original.control.signal.aborted?"cancelled":"late-effect");}await sleep(corpus.drainMilliseconds);if(kind==="detached")throw marker;}finally{lease.release();released=true;emit("lease-released");}});
  accept();
  if(kind==="error"||kind==="undefined"){void gate.catch(()=>{});throw kind==="error"?marker:undefined;}
  if(kind==="detached"){void gate.catch(()=>{});return;}
  await gate;
 };const work=supplied?execute(supplied):withBuildTestInvocationV1(execute);
 if(kind==="detached"){void work.catch(error=>{refuse(error);emit("primary",{preserved:error===marker});});return entered;}
 if(requested?.startsWith("registered-"))return work.catch(error=>{emit("primary",{preserved:error===(kind==="undefined"?undefined:marker)});throw error;});
 if(kind==="error"||kind==="undefined")return work.then(()=>{throw Error("Actual primary callback failure required");},error=>{expect(error).toBe(kind==="error"?marker:undefined);emit("primary",{preserved:true});});
 return work;
};if(kind==="timeout"||kind==="funded"||requested?.startsWith("registered-"))registerBuildTestLawV1(name,"receiver-proof",law);else test(name,()=>law(),15000);}
if(kind==="timeout")registerBuildTestLawV1("next law must refuse while original accepted work drains","receiver-proof",async()=>{emit("late-effect");});
