/** 🤲️ Tests for the shared image reader: one lease per burst, the idle release, the maximum hold, the cached sample and, with the real lease database, that a reader keeps an exclusive writer out and a waiting writer is never starved by it. */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{testScratchRootV1}from"../../../../../../🧪️testing/🏖️sandbox/🟦️.ts";
import{testPythonV1}from"../../../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{acquireQueuedResourceLease,acquireResourceLease}from"../../../../../../🔒️leases/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};
import{NATIVE_IMAGE_SHARED_READ_POLICY_V1,createNativeImageSharedReaderV1,createNativeImageSharedSampleV1,nativeImageSharedReadResourceV1,withNativeImageSharedReadV1,type NativeImageSharedLeasePortV1}from"../🟦️.ts";
const none=new AbortController().signal;
const fake=()=>{
 const log={acquires:[] as number[],releases:0,requests:[] as {resource:string;timeoutMs?:number}[],failNext:false};
 const clock={now:0,timers:[] as {at:number;run:()=>void;live:boolean}[]};
 const port:NativeImageSharedLeasePortV1={async acquire(input){
  if(log.failNext){log.failNext=false;throw new Error("lease timed out");}
  log.acquires.push(clock.now);log.requests.push({resource:input.resource,...input.timeoutMs===undefined?{}:{timeoutMs:input.timeoutMs}});
  return{release(){log.releases++;}};
 }};
 const after=(milliseconds:number,run:()=>void)=>{const row={at:clock.now+milliseconds,run,live:true};clock.timers.push(row);return()=>{row.live=false;};};
 const advance=(to:number)=>{for(;;){const due=clock.timers.filter(row=>row.live&&row.at<=to).sort((left,right)=>left.at-right.at)[0];if(due===undefined)break;due.live=false;clock.now=due.at;due.run();}clock.now=to;};
 const reader=(extra:object={})=>createNativeImageSharedReaderV1({leaseDirectory:"/leases",imageDirectory:"/ws/.cache/b/g",signal:none,port,now:()=>clock.now,after,...extra});
 return{log,clock,advance,reader};
};
const settle=()=>new Promise<void>(resolve=>setTimeout(resolve,0));
test("the policy and texts are closed, bilingual and bounded",()=>{
 const policy=NATIVE_IMAGE_SHARED_READ_POLICY_V1;
 expect(policy.idleReleaseMilliseconds).toBeLessThan(policy.maximumHoldMilliseconds);expect(policy.sampleWaitMilliseconds).toBeGreaterThan(0);
 for(const row of Object.values(schema.definitions.Texts.const))expect(Object.keys(row).sort()).toEqual(["de","en"]);
 expect(validateJsonSchemaSubset({type:"object",required:["Policy","Texts"]},schema.definitions)).toEqual([]);
 expect(nativeImageSharedReadResourceV1("/ws/.cache/b/g")).toBe("native-fixed-image:/ws/.cache/b/g");
});
for(const row of corpus.timelines)test(`the leases taken for the timeline "${row.name}" equal the independent simulation`,async()=>{
 const rig=fake(),reader=rig.reader({maximumHoldMilliseconds:row.maximum,idleReleaseMilliseconds:row.idle});
 for(const[start,duration]of row.ops){rig.advance(start);await reader.run(()=>{rig.clock.now=start+duration;});}
 await reader.close();
 const python=testPythonV1();if(python===null)throw Error("Python required by the reader oracle");
 const directory=mkdtempSync(join(testScratchRootV1("shared-read"),"oracle-"));
 try{
  writeFileSync(join(directory,"t.json"),JSON.stringify(corpus.timelines));
  const found=spawnSync(python,["-I",join(import.meta.dir,"🐍️.py"),"acquires",join(directory,"t.json")],{encoding:"utf8"});
  if(found.status!==0)throw found.error??Error(found.stderr);
  const answers=JSON.parse(found.stdout.trim()) as number[][];
  expect(rig.log.acquires).toEqual(answers[corpus.timelines.indexOf(row)]!);expect(rig.log.releases).toBe(rig.log.acquires.length);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
test("concurrent operations share one lease and the lease names the image resource",async()=>{
 const rig=fake(),reader=rig.reader();
 expect(await Promise.all([1,2,3,4].map(n=>reader.run(async()=>{await settle();return n*2;})))).toEqual([2,4,6,8]);
 expect(rig.log.acquires.length).toBe(1);expect(rig.log.requests[0]!.resource).toBe("native-fixed-image:/ws/.cache/b/g");expect(rig.log.releases).toBe(0);
 rig.advance(250);expect(rig.log.releases).toBe(1);await reader.close();expect(rig.log.releases).toBe(1);
});
test("an operation started inside a running operation of the same reader runs inside it, also past the maximum hold, instead of waiting for itself",async()=>{
 const rig=fake(),reader=rig.reader({maximumHoldMilliseconds:2000}),order:string[]=[];
 await reader.run(async()=>{order.push("outer");rig.advance(2500);await reader.run(()=>{order.push("inner");});order.push("outer-end");});
 expect(order).toEqual(["outer","inner","outer-end"]);expect(rig.log.acquires).toEqual([0]);
 await reader.close();expect(rig.log.releases).toBe(1);
});
test("past the maximum hold new operations wait for the running ones, the lease is released and taken again",async()=>{
 const rig=fake(),reader=rig.reader({maximumHoldMilliseconds:2000}),order:string[]=[];let finish:()=>void=()=>{};
 const slow=reader.run(()=>new Promise<void>(resolve=>{order.push("slow-start");finish=()=>{order.push("slow-end");resolve();};}));
 await settle();rig.advance(2500);
 const late=reader.run(()=>{order.push("late");});
 await settle();expect(order).toEqual(["slow-start"]);expect(rig.log.releases).toBe(0);
 finish();await slow;await late;
 expect(order).toEqual(["slow-start","slow-end","late"]);expect(rig.log.acquires).toEqual([0,2500]);expect(rig.log.releases).toBe(1);
 await reader.close();expect(rig.log.releases).toBe(2);
});
test("a refused lease rejects the operation, is not cached and the next operation asks again; a closed reader refuses in both languages",async()=>{
 const rig=fake(),reader=rig.reader({timeoutMs:1000});rig.log.failNext=true;
 await expect(reader.run(()=>1)).rejects.toThrow("lease timed out");
 expect(await reader.run(()=>2)).toBe(2);expect(rig.log.requests[0]!.timeoutMs).toBe(1000);
 await reader.close();await expect(reader.run(()=>3)).rejects.toThrow("The shared image reader is closed / Der geteilte Image-Leser ist geschlossen");
});
test("the sample answers the last value at once, refreshes at most once per interval and keeps the value when a refresh is refused",async()=>{
 const rig=fake(),reader=rig.reader({timeoutMs:1000});let value=10;
 const sample=createNativeImageSharedSampleV1<number|null>({reader,read:()=>value,initial:null,refreshMilliseconds:5000,now:()=>rig.clock.now});
 expect(sample.value()).toBeNull();expect(sample.value()).toBeNull();await sample.settled();expect(sample.value()).toBe(10);
 value=11;rig.advance(1000);expect(sample.value()).toBe(10);await sample.settled();expect(sample.value()).toBe(10);
 rig.advance(5000);rig.log.failNext=true;expect(sample.value()).toBe(10);await sample.settled();expect(sample.value()).toBe(10);
 rig.advance(10000);sample.value();await sample.settled();expect(sample.value()).toBe(11);await reader.close();
});
test("the one-shot read holds the lease only while its body runs and releases it when the body throws",async()=>{
 const rig=fake();
 const port:NativeImageSharedLeasePortV1={acquire:async input=>{const lease=await rig.log.requests.push({resource:input.resource});return{release(){rig.log.releases+=lease;}};}};
 expect(await withNativeImageSharedReadV1({leaseDirectory:"/l",imageDirectory:"/i",signal:none,port},()=>7)).toBe(7);expect(rig.log.releases).toBe(1);
 await expect(withNativeImageSharedReadV1({leaseDirectory:"/l",imageDirectory:"/i",signal:none,port},()=>{throw new Error("boom");})).rejects.toThrow("boom");expect(rig.log.releases).toBe(3);
});
test("with the real lease database a reader keeps the exclusive lease out, releases it when idle, and a waiting writer is not starved by later readers",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("shared-read"),"lease-")),image=join(directory,"image");mkdirSync(image);
 const resource=nativeImageSharedReadResourceV1(image),busy=()=>acquireResourceLease({directory,resource,mode:"exclusive",signal:none,timeoutMs:0});
 try{
  const reader=createNativeImageSharedReaderV1({leaseDirectory:directory,imageDirectory:image,signal:none,idleReleaseMilliseconds:50,maximumHoldMilliseconds:400,timeoutMs:300});
  await reader.run(()=>1);await expect(busy()).rejects.toThrow("timed out");
  await new Promise(resolve=>setTimeout(resolve,200));(await busy()).release();
  const holder=await acquireQueuedResourceLease({directory,resource,mode:"shared",owner:"holder",signal:none,sharedCohort:true});
  const writer=acquireQueuedResourceLease({directory,resource,mode:"exclusive",owner:"writer",signal:none});
  await new Promise(resolve=>setTimeout(resolve,100));
  const script=`import{createNativeImageSharedReaderV1}from${JSON.stringify(new URL("../🟦️.ts",import.meta.url).href)};const reader=createNativeImageSharedReaderV1({leaseDirectory:${JSON.stringify(directory)},imageDirectory:${JSON.stringify(image)},signal:new AbortController().signal,timeoutMs:150});try{await reader.run(()=>1);process.exit(3);}catch(error){process.exit(String(error?.message).includes("timed out")?0:4);}`;
  const late=spawnSync(process.execPath,["-e",script],{encoding:"utf8",env:{...process.env}});
  expect(late.status).toBe(0);
  holder.release();(await writer).release();await reader.close();
  expect(await withNativeImageSharedReadV1({leaseDirectory:directory,imageDirectory:image,signal:none},()=>4)).toBe(4);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
