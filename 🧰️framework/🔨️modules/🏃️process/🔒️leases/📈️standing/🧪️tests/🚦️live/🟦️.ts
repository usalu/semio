/** 🧪️ Tests for leases standing live. */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync} from "node:fs";
import {join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {acquireQueuedResourceLease,type ResourceLease} from "../../../🟦️.ts";
import {leaseProcessStartedAtMsV1,leaseRegistryFolderV1,readLeaseRegistryEntriesV1} from "../../../🧭️order/🗂️registry/🟦️.ts";
import {registerLeaseImplementationProviderV1,registerLeaseYieldHandlerV1,type LeasePriorityV1} from "../../🟦️.ts";
import {leaseReclaimPortsV1,observeLeaseHolderV1,readLeaseYieldRequestV1} from "../../🤝️yield/🟦️.ts";

/** 🚦️ Contract: real processes prove the queue order of priority classes, the cooperative yield of a stale holder, the safety of the reclaim and that a crashed holder releases its lease. */
const worker=fileURLToPath(new URL("../📥️worker/🟦️.ts",import.meta.url)),output=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!output)throw Error("Explicit ticket generated artifact directory required");
mkdirSync(resolve(output),{recursive:true});
const base=mkdtempSync(join(resolve(output),"ls-")),registry=join(base,"r"),leases=join(base,"l");
mkdirSync(registry);mkdirSync(leases);process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=registry;
type Child={process:Bun.Subprocess<"ignore","pipe","pipe">;events:Record<string,unknown>[];stderr:string[];done:Promise<void>};
const children:Child[]=[];
function spawnWorker(config:Record<string,unknown>,environment:Record<string,string>={}):Child{
 const process=Bun.spawn([globalThis.process.execPath,worker,JSON.stringify(config)],{stdin:"ignore",stdout:"pipe",stderr:"pipe",env:{...globalThis.process.env,SEMIO_LEASE_REGISTRY_DIRECTORY:registry,...environment}}),child:Child={process,events:[],stderr:[],done:undefined as unknown as Promise<void>};
 const read=async(stream:ReadableStream<Uint8Array>,take:(line:string)=>void)=>{let text="";for await(const chunk of stream){text+=new TextDecoder().decode(chunk);let at=text.indexOf("\n");while(at>=0){take(text.slice(0,at));text=text.slice(at+1);at=text.indexOf("\n");}}};
 child.done=Promise.all([read(process.stdout,line=>{if(line.startsWith("{"))child.events.push(JSON.parse(line));}),read(process.stderr,line=>child.stderr.push(line))]).then(()=>undefined);
 children.push(child);return child;
}
const until=async(condition:()=>boolean,milliseconds:number,what:string):Promise<void>=>{const end=Date.now()+milliseconds;while(!condition()){if(Date.now()>end)throw Error("Timed out waiting for "+what);await new Promise(done=>setTimeout(done,50));}};
afterAll(()=>{for(const child of children)try{child.process.kill("SIGKILL");}catch{}rmSync(base,{recursive:true,force:true});});

test("priority classes are served interactive, then agent in arrival order, then background, whatever the arrival order",async()=>{
 const directory=join(base,"queue"),resource="priority-order",holder=await acquireQueuedResourceLease({directory,resource,mode:"exclusive",signal:new AbortController().signal,owner:"holder"});
 const order:string[]=[],requests=(["background","agent","agent","interactive"] as LeasePriorityV1[]).map((priority,index)=>({priority,name:`${priority}-${index}`}));
 const pending:Promise<void>[]=[];
 for(const request of requests){
  pending.push(acquireQueuedResourceLease({directory,resource,mode:"exclusive",signal:new AbortController().signal,owner:request.name,priority:request.priority}).then(lease=>{order.push(request.name);lease.release();}));
  await new Promise(done=>setTimeout(done,40));
 }
 const queue=readdirSync(directory).find(name=>name.endsWith(".queue"))!,tickets=readdirSync(join(directory,queue)).sort();
 const rows=tickets.map(name=>({owner:name.split("-").slice(2).join("-"),row:JSON.parse(readFileSync(join(directory,queue,name),"utf8")) as {priority?:string}}));
 expect(rows.find(row=>row.owner==="interactive-3")!.row.priority).toBe("interactive");expect(rows.find(row=>row.owner==="background-0")!.row.priority).toBe("background");expect(rows.find(row=>row.owner==="agent-1")!.row.priority).toBeUndefined();
 expect(rows[0]!.owner).toBe("interactive-3");
 holder.release();await Promise.all(pending);
 expect(order).toEqual(["interactive-3","agent-1","agent-2","background-0"]);
});

test("a waiter on newer code asks a stale holder to yield, in German, and the holder yields where its result would be discarded",async()=>{
 const chosen=["SEMIO_LANGUAGE","LC_ALL","LANG"].map(name=>[name,process.env[name]] as const);for(const [name] of chosen)delete process.env[name];
 try{
 registerLeaseImplementationProviderV1({components:()=>({"graph.generation":"aaaaaaaaaaaa"}),bound:resource=>resource.startsWith("graph:")?["graph.generation"]:[],moved:resource=>resource.startsWith("graph:")?["graph.generation"]:[]});
 const directory=join(base,"yield"),resource="graph:builder";let lease!:ResourceLease;const yielded:{reason:string;message:string}[]=[];
 registerLeaseYieldHandlerV1(request=>{yielded.push({reason:request.reason,message:request.message});lease.release();});
 lease=await acquireQueuedResourceLease({directory:resolve(directory),resource,mode:"exclusive",signal:new AbortController().signal,owner:"older"});
 await until(()=>readLeaseRegistryEntriesV1(registry,resolve(directory),Date.now(),false).some(entry=>entry.pid===process.pid&&entry.holds.some(row=>row.resource===resource)),5000,"the holder's registry entry");
 await new Promise(done=>setTimeout(done,20));
 const waiter=spawnWorker({directory:resolve(directory),resource,role:"wait",generation:"bbbbbbbbbbbb"},{SEMIO_LANGUAGE:"de"});
 await until(()=>waiter.events.some(row=>row.event==="acquired"),40000,"the newer waiter to acquire after the stale holder yielded");
 expect(yielded).toHaveLength(1);expect(yielded[0]!.reason).toBe("stale");expect(yielded[0]!.message).toContain("läuft mit neuerem Code (graph.generation)");
 await waiter.done;expect(waiter.stderr.join("\n")).toContain(`Prozess ${process.pid} gebeten, ${resource} freizugeben (stale)`);
 }finally{for(const [name,value] of chosen){if(value===undefined)delete process.env[name];else process.env[name]=value;}}
},60000);

test("Law: a waiter whose claimed newer code the disk does not hold any more (the edit was reverted) never makes a healthy holder yield: the holder re-reads the stamps itself",async()=>{
 registerLeaseImplementationProviderV1({components:()=>({"graph.generation":"aaaaaaaaaaaa"}),bound:resource=>resource.startsWith("graph:")?["graph.generation"]:[],moved:()=>[]});
 const directory=join(base,"revert"),resource="graph:reverted";let lease!:ResourceLease;const yielded:string[]=[];
 registerLeaseYieldHandlerV1(request=>{yielded.push(request.reason);lease.release();});
 lease=await acquireQueuedResourceLease({directory:resolve(directory),resource,mode:"exclusive",signal:new AbortController().signal,owner:"older"});
 await until(()=>readLeaseRegistryEntriesV1(registry,resolve(directory),Date.now(),false).some(entry=>entry.pid===process.pid&&entry.holds.some(row=>row.resource===resource)),5000,"the holder's registry entry");
 await new Promise(done=>setTimeout(done,20));
 const waiter=spawnWorker({directory:resolve(directory),resource,role:"wait",generation:"bbbbbbbbbbbb"},{SEMIO_LANGUAGE:"en"});
 await new Promise(done=>setTimeout(done,16000));
 expect(readLeaseYieldRequestV1(registry,resolve(directory),process.pid,leaseProcessStartedAtMsV1,Date.now()),"the request is filed (the waiter believes it is newer)").toBeDefined();
 expect(yielded,"the holder compares with the disk, finds nothing moved and keeps the lease").toEqual([]);
 expect(waiter.events.some(row=>row.event==="acquired")).toBe(false);
 lease.release();
 await until(()=>waiter.events.some(row=>row.event==="acquired"),40000,"the waiter after the holder finished");
 await waiter.done;
},90000);

test("a holder on the same code is never asked to yield, a stuck holder is asked, and only a verified, ignored, silent holder is stopped; its crash releases the lease",async()=>{
 const directory=resolve(join(base,"stuck")),resource="repository-graph-facts:/w:shard:7",holder=spawnWorker({directory,resource,role:"hold",generation:"aaaaaaaaaaaa"});
 await until(()=>holder.events.some(row=>row.event==="acquired"),20000,"the holder");
 const pid=holder.process.pid,entryOf=()=>readLeaseRegistryEntriesV1(registry,directory,Date.now(),false).find(entry=>entry.pid===pid);
 await until(()=>entryOf()!==undefined,10000,"the registry entry of the holder");
 const entry=entryOf()!,written:string[]=[],ports={...leaseReclaimPortsV1,write:(text:string)=>{written.push(text);}},environment={SEMIO_LEASE_RECLAIM:"terminate",SEMIO_LANGUAGE:"en"},now=Date.now(),observe=(nowMs:number,override=ports)=>observeLeaseHolderV1({directory,resource,headPid:pid,headMode:"exclusive",headArrivalMs:now-1000,nowMs,environment,root:registry,ports:override});
 expect((await observe(now))?.holderState).toBe("progressing");expect(readLeaseYieldRequestV1(registry,directory,pid,entry.startedAtMs,now)).toBeUndefined();
 const stuck=now+25*60000;
 expect((await observe(stuck))?.holderState).toBe("stuck");
 const request=readLeaseYieldRequestV1(registry,directory,pid,entry.startedAtMs,stuck);
 expect([request?.reason,request?.requesterPid]).toEqual(["stuck",process.pid]);expect(written.join("")).toContain(`Asked process ${pid} to yield`);
 await observe(stuck+10000);expect(holder.process.exitCode).toBeNull();
 const wrongStart={...ports,osStartedAtMs:()=>entry.startedAtMs+3600000};
 await observe(stuck+180000,wrongStart);expect(holder.process.exitCode).toBeNull();expect(written.join("")).toContain("its start time does not match the registered one");
 await observe(stuck+180000);
 await until(()=>holder.process.exitCode!==null||holder.process.signalCode!==null,10000,"the stopped holder to exit; notices: "+JSON.stringify(written));
 expect(written.join("")).toContain(`Stopped process ${pid}: it held ${resource}`);
 const log=readFileSync(join(leaseRegistryFolderV1(registry,directory),"reclaim.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line));expect(log[0]).toMatchObject({pid,resource,by:process.pid,state:"stuck"});
 const next=await acquireQueuedResourceLease({directory,resource,mode:"exclusive",signal:new AbortController().signal,owner:"after",timeoutMs:15000});next.release();
 expect(existsSync(directory)).toBe(true);
},90000);
