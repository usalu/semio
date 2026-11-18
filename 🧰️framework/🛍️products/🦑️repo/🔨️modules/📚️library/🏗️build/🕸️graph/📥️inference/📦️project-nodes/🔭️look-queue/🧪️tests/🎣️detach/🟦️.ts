/** 🎣️ Law: an owner that asks while its batch is still queued is looked at in a job of its own, so the answer of a fast owner never waits for the slowest owner of its batch; the other owners of the batch keep their order and are answered once each.
 * @see ../../🟨️.mjs `createLookQueueV1` */
import{expect,test}from"bun:test";
import{createLookQueueV1}from"../../🟨️.mjs";

test("the asked owner leaves a queued batch and is answered before the slow owner of that batch",async()=>{
 const jobs:string[][]=[];
 const farm={width:()=>1,observe:async(job:any)=>{const owners=job.owners.map((owner:any)=>owner.name);jobs.push(owners);await Bun.sleep(owners.includes("slow")?600:5);return{looks:owners.map((name:string)=>({name,native:false,pipeline:false,head:["h"],verdict:"holds",headerDigest:"d"}))};}};
 const queue=createLookQueueV1({farm:farm as never,jobOf:name=>({name}),refuse:(name,reason)=>Error(name+reason)});
 const fillers=Array.from({length:14},(_,at)=>"f"+at);
 queue.start([["blocker",...fillers],["slow","fast","other"]]);
 const begun=performance.now();
 const [fast,other]=await Promise.all([queue.ask("fast").then(()=>performance.now()-begun),queue.ask("other").then(()=>performance.now()-begun)]);
 expect(fast).toBeLessThan(400);expect(other).toBeLessThan(400);
 await queue.ask("slow");
 const looked=jobs.flat();for(const name of["fast","other","slow"])expect(looked.filter(each=>each===name).length).toBe(1);
 expect(jobs.filter(owners=>owners.includes("fast")).every(owners=>!owners.includes("slow"))).toBe(true);
});
