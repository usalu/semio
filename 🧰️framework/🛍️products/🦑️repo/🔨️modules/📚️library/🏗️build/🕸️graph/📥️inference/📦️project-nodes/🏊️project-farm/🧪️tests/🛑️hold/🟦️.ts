/** 🛑️ Contract: while the process tree stands at the hard share of its memory ceiling (`hold` answers true) a farm dispatches no job beside a running one, yet the first job always runs and every queued job finishes, so a held farm slows down and never stops; without a hold the same farm runs jobs side by side.
 * @see ../../🏊️pool/🟨️.mjs */
import{expect,test}from"bun:test";
import{dirname,join}from"node:path";
import{fileURLToPath,pathToFileURL}from"node:url";
import{GRAPH_FARM_POLICY_V1,createFarmV1}from"../../🏊️pool/🟨️.mjs";
import{farmHeldV1}from"../../🟨️.mjs";
import{FOOTPRINT_POLICY_V1}from"../../../../../../🧠️memory/🦶️footprint/🟨️.mjs";
import{PLAN_STAGE_POLICY_V1}from"../../../../../../🧠️memory/🪜️plan-stage/🟨️.mjs";

const here=dirname(fileURLToPath(import.meta.url)),worker=join(here,"..","..","👷️worker","🟨️.mjs"),service=pathToFileURL(join(here,"🧫️fixtures","🟨️.mjs")).href;
const farm=(options:Record<string,unknown>={})=>createFarmV1({entry:worker,service,session:{identity:"law"},workers:2,megabytes:100000,environment:{...process.env},accept:(identity:unknown)=>identity==="law",policy:{...GRAPH_FARM_POLICY_V1,holdRecheckMilliseconds:20},...options} as never);
type Window={start:number;end:number};
const overlap=(rows:Window[])=>Math.max(...rows.map(row=>rows.filter(other=>other.start+25<row.end&&row.start+25<other.end).length));
const run=async(pool:ReturnType<typeof farm>,count:number)=>(await Promise.all(Array.from({length:count},(_,n)=>pool.run({n,delay:120})))).map(answer=>answer.reply as Window);

test("a held farm runs its jobs one after the other and finishes all of them, an unheld farm runs two side by side",async()=>{
 let asked=0;const held=farm({hold:()=>{asked++;return true;}});
 const rows=await run(held,4);await held.close();
 expect(rows.length).toBe(4);expect(asked).toBeGreaterThan(0);expect(overlap(rows),"no job beside a running one while held").toBe(1);
 const open=farm(),free=await run(open,4);await open.close();
 expect(overlap(free),"without a hold both workers run").toBe(2);
},120000);

test("a hold that lifts while jobs wait lets the farm run side by side again",async()=>{
 let held=true;const pool=farm({hold:()=>held}),work=run(pool,6);
 await new Promise(done=>setTimeout(done,150));held=false;
 const rows=await work;await pool.close();
 expect(rows.length).toBe(6);expect(overlap(rows)).toBe(2);
},120000);

test("the farm holds when the tree plus the child it would admit passes the hard share of the ceiling, never without a gate, and reads a footprint no older than the declared meter time",()=>{
 const MiB=1048576,grant=6553*MiB,allowance=PLAN_STAGE_POLICY_V1.farmWorkerPeakMegabytes["project-farm"],limit=grant*FOOTPRINT_POLICY_V1.hardPercent/100;
 const gate={grantBytes:()=>grant},at=(treeMegabytes:number)=>farmHeldV1(gate as never,allowance,{bytes:()=>treeMegabytes*MiB} as never);
 for(const tree of[0,1000,2500,3000,3350,3400,3500,5000,9400]){const model=tree*MiB+allowance*MiB>limit;expect(at(tree),tree+" MB").toBe(model);}
 expect(at(1000)).toBe(false);expect(at(5000)).toBe(true);
 expect(farmHeldV1(undefined,allowance,{bytes:()=>9999*MiB} as never)).toBe(false);
 expect(farmHeldV1({grantBytes:()=>undefined} as never,allowance,{bytes:()=>9999*MiB} as never)).toBe(false);
 expect(GRAPH_FARM_POLICY_V1.holdMeterMilliseconds).toBeLessThan(5000);expect(GRAPH_FARM_POLICY_V1.holdMeterMilliseconds).toBeGreaterThan(0);
});
