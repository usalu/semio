/** 🔭️ Contract: a farm answers the live count of its slots (`snapshot`: busy, idle, starting, not started, queued jobs, whether a job runs alone, restarts), so the line of a parked owner names the children that really exist instead of the fixed words "no children" it printed for every pool, also while nine children ran; a read that waits on its children can tell a farm without children from a farm whose children are busy.
 * @see ../../🏊️pool/🟨️.mjs @see ../../🟨️.mjs `describe` */
import{expect,test}from"bun:test";
import{EventEmitter}from"node:events";
import{testPythonV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{createFarmV1,GRAPH_FARM_POLICY_V1}from"../../🏊️pool/🟨️.mjs";

class Scripted extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;
 constructor(readonly pid:number){super();}
 send(message:{type:string}){if(message.type==="shutdown"){this.exitCode=0;setImmediate(()=>this.emit("exit"));}return true;}
 kill(){this.exitCode=0;setImmediate(()=>this.emit("exit"));return true;}
}

test("a farm that started three children counts them in its snapshot and every slot is accounted for",async()=>{
 let pid=900000;
 const farm=createFarmV1({entry:"x",service:"s",session:{},workers:3,megabytes:64,environment:{LANG:"en_US.UTF-8"},spawnProcess:(()=>new Scripted(pid++)) as never} as never);
 await new Promise(resolve=>setTimeout(resolve,50));
 const snapshot=(farm as unknown as {snapshot:Record<string,unknown>}).snapshot;
 expect(snapshot).toBeDefined();
 for(const key of ["busy","idle","starting","empty","queued","restarts"])expect(typeof snapshot[key]).toBe("number");
 expect(typeof snapshot.alone).toBe("boolean");
 expect((snapshot.busy as number)+(snapshot.idle as number)+(snapshot.starting as number)+(snapshot.empty as number)).toBe(3);
 expect(snapshot.starting).toBeGreaterThan(0);
 await farm.close();
},20000);

test("the line of a parked owner reads the snapshot the farm answers",()=>{
 const source=require("node:fs").readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8") as string;
 expect(source).toContain("pool.farm.snapshot");
});

for(const interruption of ["timeout","close"] as const)test(`a blocked measurement can settle by ${interruption} after the worker replies`,async()=>{
 const oracle=Bun.spawnSync([testPythonV1()!,"-I","-c",`import json,sqlite3
db=sqlite3.connect(":memory:")
db.execute("CREATE TABLE slots (phase TEXT)")
db.execute("INSERT INTO slots VALUES ('settling')")
counts=dict(db.execute("SELECT phase,count(*) FROM slots GROUP BY phase"))
print(json.dumps([counts.get(phase,0) for phase in ('busy','idle','starting','empty','settling','retiring')]))`]);
 expect(oracle.exitCode,oracle.stderr.toString()).toBe(0);
 const expected=JSON.parse(oracle.stdout.toString());
 let sampled!:(signal:AbortSignal|undefined)=>void, finish!:(bytes:number)=>void;
 const started=new Promise<AbortSignal|undefined>(done=>{sampled=done;});
 const hold=new Promise<number>(done=>{finish=done;});
 const child=new Scripted(900000);
 const send=child.send.bind(child);
 child.send=(message:{type:string;id?:number})=>{
  if(message.type==="run")queueMicrotask(()=>child.emit("message",{type:"result",id:message.id,reply:{weight:1,value:7},rssMegabytes:100}));
  return send(message);
 };
 const farm=createFarmV1({entry:"x",service:"s",session:{},workers:1,megabytes:1000,environment:{},policy:{...GRAPH_FARM_POLICY_V1,settleFootprintMilliseconds:interruption==="close"?10000:200},spawnProcess:(()=>child) as never,footprintOf:(_pid:number,options?:{signal:AbortSignal})=>{sampled(options?.signal);return hold;}});
 let deadline:ReturnType<typeof setTimeout>|undefined;
 try{
  child.emit("message",{type:"ready"});
  const job=farm.run({name:"owner"}),signal=await started,snapshot=farm.snapshot as unknown as Record<string,number>;
  const pending=farm.pending;
  const completed=interruption==="close"?Promise.all([job,farm.close()]).then(([result])=>result):job;
  const reply=await Promise.race([completed,new Promise<never>((_,reject)=>{deadline=setTimeout(()=>reject(Error("measurement held a completed reply")),1200);})]);
  expect(reply).toMatchObject({reply:{value:7}});
  expect(["busy","idle","starting","empty","settling","retiring"].map(phase=>snapshot[phase])).toEqual(expected);
  expect(pending).toBe(1);
  expect(signal?.aborted).toBe(true);
  if(interruption==="close")expect(child.exitCode).toBe(0);
  else {
   expect(farm.snapshot.idle).toBe(1);
   finish(9000*1048576);
   await Promise.resolve();
   expect(farm.stats().recycled).toBe(0);
  }
 }finally{
  clearTimeout(deadline);finish(1);await farm.close();
 }
},20000);
