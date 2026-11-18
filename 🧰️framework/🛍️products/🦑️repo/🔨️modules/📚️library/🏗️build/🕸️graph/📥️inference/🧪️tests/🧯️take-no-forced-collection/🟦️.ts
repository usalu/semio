/** 🧯️ Contract: a reader that accepts stored owner records forces no full collection per owner. The yield points of the inference ran `releaseIfGrownV1`, which collects, shrinks and collects again whenever the managed memory grew by 256 MiB (a walk of the whole heap: 37 s of 192 s for one owner on a cold machine, where the intake of closure entries grows the heap by hundreds of megabytes per entry). A take pass only re-reads stored rows and compiles nothing, so its release waits for `takeReleaseGrowthBytes` (four times as much) and, with a gate, still for the pressure the gate reads: accepting owners that grow the heap by less than that forces no collection, and a growth far beyond it collects once per that growth, never once per owner. Outside a take pass (an inference that compiles) the release stays as it was.
 * @see ../../🧭️script-inputs/🟨️.mjs `yieldInferenceV1`
 * @see ../../../../🧠️memory/🦶️footprint/🟨️.mjs `releaseIfGrownV1` */
import{expect,test}from"bun:test";
import{runClosureTakeV1,yieldInferenceV1}from"../../🧭️script-inputs/🟨️.mjs";
import{FOOTPRINT_POLICY_V1}from"../../../../🧠️memory/🦶️footprint/🟨️.mjs";
import{libraryBootstrap}from"../../🟨️.mjs";

const OWNERS=30,YIELDS=64,PER_OWNER=16*1048576;
let extra=0;
const growing=()=>{const real=process.memoryUsage;const fake=Object.assign(()=>{const row=real();return{...row,heapUsed:row.heapUsed+extra};},{rss:real.rss});process.memoryUsage=fake as unknown as typeof process.memoryUsage;return{grow:(bytes:number)=>{extra+=bytes;},stop:()=>{process.memoryUsage=real;}};};
const counting=()=>{const bun=globalThis.Bun as unknown as{gc:(synchronous?:boolean)=>void},real=bun.gc;let calls=0;bun.gc=(synchronous?:boolean)=>{calls++;return real.call(bun,synchronous);};return{calls:()=>calls,stop:()=>{bun.gc=real;}};};
const owner=async(memory:ReturnType<typeof growing>,bytes:number)=>{for(let at=0;at<YIELDS;at++){memory.grow(bytes/YIELDS);await yieldInferenceV1({},"accept",at);}};

test("the take release waits for more growth than the inference release",()=>{
 expect(FOOTPRINT_POLICY_V1.takeReleaseGrowthBytes).toBeGreaterThanOrEqual(4*FOOTPRINT_POLICY_V1.releaseGrowthBytes);
});

test("accepting owners in a take pass forces no collection while the heap grows by less than the declared amount",async()=>{
 await libraryBootstrap;
 const spy=counting(),memory=growing();
 try{
  expect(OWNERS*PER_OWNER).toBeLessThan(FOOTPRINT_POLICY_V1.takeReleaseGrowthBytes);
  await runClosureTakeV1(async()=>{for(let at=0;at<OWNERS;at++)await owner(memory,PER_OWNER);});
  expect(spy.calls(),"no forced collection for "+OWNERS+" owners").toBe(0);
 }finally{spy.stop();memory.stop();}
},120000);

test("a growth far beyond it collects once per pressure event, never once per owner",async()=>{
 await libraryBootstrap;await Bun.sleep(2500);
 const spy=counting(),memory=growing(),owners=8,each=FOOTPRINT_POLICY_V1.takeReleaseGrowthBytes;
 try{
  await runClosureTakeV1(async()=>{for(let at=0;at<owners;at++){await owner(memory,each);await Bun.sleep(1200);}});
  expect(spy.calls(),"it collects").toBeGreaterThan(0);
  expect(spy.calls(),"at most one release (two collections) per pressure event").toBeLessThanOrEqual(2*owners);
 }finally{spy.stop();memory.stop();}
},120000);

test("outside a take pass the same growth still releases the memory a compile leaves behind",async()=>{
 await libraryBootstrap;await Bun.sleep(2500);
 const spy=counting(),memory=growing();
 try{
  for(let at=0;at<4;at++)await owner(memory,2*FOOTPRINT_POLICY_V1.releaseGrowthBytes);
  expect(spy.calls(),"the inference keeps its release").toBeGreaterThan(0);
 }finally{spy.stop();memory.stop();}
},120000);
