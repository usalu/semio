/** 🧭️ Law: compiler capacity (rank 3) is admitted before any artifact input lease (rank 6) is taken, and the whole queue for it is admission wait that no deadline counts.
 * @see ../../📥️inputs/🟦️.ts */
import {expect,test}from"bun:test";
import{mkdtempSync,readFileSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{admissionWaitedMillisecondsV1}from"../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";
import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
declareBuildTestEnvelopeV1({owner:"test:vite-acquisition-order",maximumElapsedMilliseconds:120000,capabilities:{}});
import{acquireCompilerCapacityLeaseV1,scriptCompilerAdmissionControlV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";

const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const slice=(file:URL,start:string,end:string):string=>{const source=readFileSync(file,"utf8"),from=source.indexOf(start),to=source.indexOf(end,from);expect(from).toBeGreaterThan(-1);expect(to).toBeGreaterThan(from);return source.slice(from,to);};
const ordered=(body:string):void=>{
 const capacity=body.indexOf("withViteCompilerCapacityV1("),inputs=body.indexOf("withViteArtifactInputsV1("),state=body.indexOf("withCompilerState(");
 expect(capacity).toBeGreaterThan(-1);expect(capacity).toBeLessThan(inputs);expect(inputs).toBeLessThan(state);
 expect(body).toContain("withCompilerState({capacity:null");expect(body).not.toContain("countedCompilerCapacityRequestV1");
};

test("a Vite build admits compiler capacity before it takes its artifact inputs",()=>{
 ordered(slice(new URL("../../🟦️.ts",import.meta.url),"export async function buildViteArtifact(","\nexport type ViteService"));
});

test("the distribution compiler admits compiler capacity before it takes its artifact inputs",()=>{
 ordered(slice(new URL("../../../../../../../../🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🏗️compiler/🟦️.ts",import.meta.url),"export async function compileDistributionProductV1(","\n/** 🏗️ Runs the real compiler"));
});

test("the queue for compiler capacity is admission wait that stops counting once the grant is held",()=>withBuildTestInvocationV1(async original=>{
 const directory=mkdtempSync(join(tmpdir(),"semio-vite-order-")),control=scriptCompilerAdmissionControlV1(original,"test:vite-acquisition-order",60000);
 try{
  const held=await acquireCompilerCapacityLeaseV1({directory,control,environment:{},request:{kind:"whole-host"}}),before=admissionWaitedMillisecondsV1(),queued=acquireCompilerCapacityLeaseV1({directory,control,environment:{},request:{kind:"exact",parallelism:1}});
  await sleep(400);expect(admissionWaitedMillisecondsV1()-before).toBeGreaterThanOrEqual(300);
  held.release();const lease=await queued;lease.release();
  const settled=admissionWaitedMillisecondsV1();await sleep(150);expect(admissionWaitedMillisecondsV1()).toBe(settled);
 }finally{rmSync(directory,{recursive:true,force:true});}
}));
