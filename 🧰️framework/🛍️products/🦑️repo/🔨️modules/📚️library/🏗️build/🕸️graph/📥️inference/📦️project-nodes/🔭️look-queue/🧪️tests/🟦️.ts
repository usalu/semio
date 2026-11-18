/** 🔭️ Contract and law: the looks of a take pass are farm jobs. Owners go to the farm in batches (one batch of each lane in turn), at most as many batches run at once as children are ready and every ready child is used, an owner that asks first moves its batch to the front, an answer is used once and not after `freshMilliseconds`, a farm that is gone answers undefined and a farm that failed names the owner. The parent that only dispatches and adopts stays responsive: 200 owners over 4 children keep its event loop under 200 ms late and its CPU under 50 ms per owner.
 * @see ../🟨️.mjs */
import{expect,test}from"bun:test";
import{LOOK_POLICY_V1,createLookQueueV1,lookBatchSizeV1,lookBatchesV1,lookQueuedTextV1}from"../🟨️.mjs";

const names=(prefix:string,count:number)=>Array.from({length:count},(_,at)=>prefix+at);
const refuse=(name:string,reason:string)=>Object.assign(Error(name+": "+reason),{code:"graph.farm-job-failed"});
const farmOf=(width:number,milliseconds=15,answer?:(job:any)=>any)=>{
 const seen={running:0,most:0,jobs:0,owners:0,sizes:[] as number[],order:[] as string[]};
 return{seen,width:()=>width,observe:async(job:any)=>{
  seen.running++;seen.most=Math.max(seen.most,seen.running);seen.jobs++;seen.owners+=job.owners.length;seen.sizes.push(job.owners.length);seen.order.push(job.owners[0].name);
  try{await Bun.sleep(milliseconds);return answer?answer(job):{looks:job.owners.map((owner:any)=>({name:owner.name,native:true,pipeline:false,head:["a",owner.name],verdict:"holds",headerDigest:"d"}))};}finally{seen.running--;}
 }};
};
const jobOf=(name:string)=>({name});

test("a batch holds enough owners to feed every child several times and never more than the declared maximum",()=>{
 expect(lookBatchSizeV1(4775,9)).toBe(LOOK_POLICY_V1.maximumOwnersPerJob);
 expect(lookBatchSizeV1(40,4)).toBe(Math.ceil(40/(4*LOOK_POLICY_V1.jobsPerChild)));expect(lookBatchSizeV1(200,4)).toBe(LOOK_POLICY_V1.maximumOwnersPerJob);
 expect(lookBatchSizeV1(3,4)).toBe(1);expect(lookBatchSizeV1(0,0)).toBe(1);
});

test("batches take one slice of each lane in turn and every name is in exactly one batch in its lane's order",()=>{
 const long=names("l",7),short=names("s",2),batches=lookBatchesV1([long,short],3);
 expect(batches).toEqual([["l0","l1","l2"],["s0","s1"],["l3","l4","l5"],["l6"]]);
 expect(batches.flat().sort()).toEqual([...long,...short].sort());
 expect(lookBatchesV1([[],[]],3)).toEqual([]);expect(long.length).toBe(7);
});

test("the queued line says its numbers in both languages",()=>{
 const fields={owners:200,jobs:34,size:6,children:4};
 expect(lookQueuedTextV1("en",fields)).toContain("200 owners in 34 farm jobs");
 expect(lookQueuedTextV1("de",fields)).toContain("200 Projekte in 34 Farm-Aufgaben");
 expect(lookQueuedTextV1("de",fields)).toContain("Elternprozess");
});

test("200 owners in two lanes over 4 children: every child is used, never more jobs run than children are ready, every owner is answered once, the short lane is not held behind the long one",async()=>{
 const farm=farmOf(4),queue=createLookQueueV1({farm:farm as never,jobOf,refuse}),long=names("l",150),short=names("s",50),said:string[]=[];
 const queued=createLookQueueV1({farm:farm as never,jobOf,refuse,say:text=>said.push(text("en"))});queued.start([long,short]);
 expect(said[0]).toContain("200 owners");
 const answered=new Map<string,any>(),lane=async(list:string[])=>{let next=0;await Promise.all(Array.from({length:12},async()=>{while(next<list.length){const name=list[next++];answered.set(name,await queued.ask(name));}}));};
 await Promise.all([lane(long),lane(short)]);
 expect(answered.size).toBe(200);for(const[name,reply]of answered)expect(reply.head[1]).toBe(name);
 expect(farm.seen.most,"all four children used").toBe(4);
 expect(farm.seen.owners,"every owner looked at once").toBe(200);
 expect(farm.seen.sizes.every(size=>size<=LOOK_POLICY_V1.maximumOwnersPerJob)).toBe(true);
 expect(farm.seen.order.slice(0,4).some(first=>first.startsWith("s")),"a batch of the short lane is among the first four").toBe(true);
 expect(queued.stats().held,"nothing is left unclaimed").toBe(0);expect(queue.stats().jobs).toBe(0);
});

test("LAW: the parent of 200 owners over 4 children stays under 200 ms of event-loop lag and 50 ms of CPU per owner",async()=>{
 const farm=farmOf(4,25),queue=createLookQueueV1({farm:farm as never,jobOf,refuse}),all=names("o",200);
 let worst=0,last=performance.now();const timer=setInterval(()=>{const at=performance.now();worst=Math.max(worst,at-last-10);last=at;},10);
 const cpu=process.cpuUsage();queue.start([all.slice(0,100),all.slice(100)]);
 await Promise.all(all.map(name=>queue.ask(name)));
 const used=process.cpuUsage(cpu);clearInterval(timer);
 expect(worst,"worst event-loop lateness in ms").toBeLessThan(200);
 expect((used.user+used.system)/1000/200,"parent CPU ms per owner").toBeLessThan(50);
});

test("no more owners fly than the lead allows until somebody asks, and an owner that asks first moves its batch to the front",async()=>{
 const farm=farmOf(2,10),queue=createLookQueueV1({farm:farm as never,jobOf,refuse}),all=names("o",300);
 queue.start([all]);await Bun.sleep(150);
 const size=lookBatchSizeV1(300,2),window=2*LOOK_POLICY_V1.leadJobsPerChild*size;
 expect(queue.stats().held+queue.stats().flying,"dispatched but unclaimed owners").toBeLessThanOrEqual(window+size);
 expect(farm.seen.owners).toBeLessThan(300);
 const before=farm.seen.jobs,reply=await queue.ask("o299");
 expect(reply?.head[1]).toBe("o299");expect(farm.seen.jobs,"its batch was brought forward").toBeGreaterThan(before);
});

test("an answer is used once, a second ask looks again, and an answer older than the fresh time is asked again",async()=>{
 let clock=0;const farm=farmOf(1,1),queue=createLookQueueV1({farm:farm as never,jobOf,refuse,now:()=>clock});
 queue.start([["a","b"]]);await Bun.sleep(30);
 expect((await queue.ask("a"))?.name).toBe("a");const once=farm.seen.jobs;
 clock+=LOOK_POLICY_V1.freshMilliseconds+1;
 expect((await queue.ask("b"))?.name,"stale answer").toBe("b");expect(farm.seen.jobs).toBe(once+1);expect(queue.stats().again).toBe(1);
 expect((await queue.ask("a"))?.name,"second ask").toBe("a");expect(farm.seen.jobs).toBe(once+2);
});

test("a farm that is gone answers undefined, a failed job and a failed owner name the owner, the others of the batch are answered",async()=>{
 const gone=createLookQueueV1({farm:farmOf(1,1,()=>undefined) as never,jobOf,refuse});gone.start([["a","b"]]);
 expect(await gone.ask("a")).toBeUndefined();expect(await gone.ask("b")).toBeUndefined();
 const broken=createLookQueueV1({farm:{width:()=>1,observe:async()=>{throw Error("child lost");}} as never,jobOf,refuse});broken.start([["a","b"]]);
 await expect(broken.ask("a")).rejects.toThrow("a: child lost");await expect(broken.ask("b")).rejects.toThrow("b: child lost");
 const partial=createLookQueueV1({farm:farmOf(1,1,job=>({looks:[{name:"a",head:[],verdict:"holds"},{name:"b",failed:"entry stale"}]})) as never,jobOf,refuse});partial.start([["a","b","c"]]);
 expect((await partial.ask("a"))?.name).toBe("a");
 await expect(partial.ask("b")).rejects.toThrow("b: entry stale");
 await expect(partial.ask("c")).rejects.toThrow("c: the child gave no answer");
});
