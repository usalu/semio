/** 🛑️ Contract: while the process tree stands at the hard share of its memory ceiling (`hold` answers true) a farm dispatches no job beside a running one, yet the first job always runs and every queued job finishes, so a held farm slows down and never stops; without a hold the same farm runs jobs side by side.
 * @see ../../🏊️pool/🟨️.mjs */
import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{existsSync,mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import fixture from"./🔣️.json"with{type:"json"};
import{dirname,join}from"node:path";
import{fileURLToPath,pathToFileURL}from"node:url";
import{GRAPH_FARM_POLICY_V1,choosePoolV1,createFarmV1}from"../../🏊️pool/🟨️.mjs";
import{farmHeldV1}from"../../🟨️.mjs";
import{FOOTPRINT_POLICY_V1}from"../../../../../../🧠️memory/🦶️footprint/🟨️.mjs";
import{PLAN_STAGE_POLICY_V1}from"../../../../../../🧠️memory/🪜️plan-stage/🟨️.mjs";

const here=dirname(fileURLToPath(import.meta.url)),worker=join(here,"..","..","👷️worker","🟨️.mjs"),service=pathToFileURL(join(here,"🧫️fixtures","🟨️.mjs")).href;
const farm=(options:Record<string,unknown>={})=>createFarmV1({entry:worker,service,session:{identity:"law"},workers:2,megabytes:100000,environment:{...process.env},accept:(identity:unknown)=>identity==="law",policy:{...GRAPH_FARM_POLICY_V1,holdRecheckMilliseconds:20},...options} as never);
type Window={start:number;end:number};
const overlap=(rows:Window[])=>Math.max(...rows.map(({start})=>rows.filter(row=>row.start<=start&&start+25<row.end).length));
const run=async(pool:ReturnType<typeof farm>,count:number)=>(await Promise.all(Array.from({length:count},(_,n)=>pool.run({n,delay:120})))).map(answer=>answer.reply as Window);

test("two successive jobs can both overlap one other job without all three running together",()=>{
 expect(overlap([{start:0,end:300},{start:40,end:120},{start:180,end:260}])).toBe(2);
});

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


for(const vector of fixture.selection)test(`the native farm ranks the ${vector.name} by actual dispatch eligibility`,async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
 if(artifacts===undefined)throw Error("Owned test artifacts directory required");
 mkdirSync(artifacts,{recursive:true});
 const root=mkdtempSync(join(artifacts,"farm-")),started=join(root,"started"),release=join(root,"release");
 let samples=0,held=vector.hold;
 const pools=[farm({workers:2,hold:()=>{samples++;return held;},footprintOf:()=>0}),farm({workers:1,hold:()=>vector.hold,footprintOf:()=>0})];
 const until=async(check:()=>boolean)=>{const end=Date.now()+10000;while(!check()){if(Date.now()>=end)throw Error("Farm fixture did not reach the expected state");await Bun.sleep(10);}};
 const slots=(pool:typeof pools[number])=>(pool as typeof pool&{snapshot:{idle:number;busy:number}}).snapshot;
 try{
  expect(await Promise.all(pools.map(pool=>pool.ready))).toEqual([true,true]);
  await until(()=>slots(pools[0]!).idle===2);
  const active=pools[0]!.run({n:1,started,release});
  await until(()=>existsSync(started));
  const native=pools.map(pool=>({idle:slots(pool).idle,busy:slots(pool).busy}));
  expect(native).toEqual(vector.pools);
  const sqlite=new Database(":memory:");
  try{
   const oracle=sqlite.query(`SELECT cast(key AS integer) AS chosen FROM json_each(?)
    WHERE json_extract(value,'$.idle')>0 AND (NOT ? OR json_extract(value,'$.busy')=0)
    ORDER BY cast(key AS integer) LIMIT 1`).get(JSON.stringify(native),Number(vector.hold)) as {chosen:number};
   const select=(candidates:typeof pools)=>choosePoolV1(candidates.map(pool=>({ready:true,usable:pool.usable,idle:pool.idle,pending:pool.pending,width:pool.width})));
   const observed=samples,selected=select(pools);
   expect(samples).toBe(observed);
   expect(selected).toBe(vector.expected);
   expect(selected).toBe(oracle.chosen);
   expect((await pools[selected]!.run({n:2})).reply).toMatchObject({n:2});
   expect(existsSync(release)).toBe(false);
   if(vector.hold){
    held=false;
    const heldPool=pools[0]!,before=samples;
    expect(heldPool.idle).toBe(false);
    expect(select([heldPool])).toBe(0);
    expect(samples).toBe(before);
    expect((await heldPool.run({n:3})).reply).toMatchObject({n:3});
    expect(samples).toBeGreaterThan(before);
    expect(existsSync(release)).toBe(false);
   }
  }finally{sqlite.close();}
  writeFileSync(release,"release");
  expect((await active).reply).toMatchObject({n:1});
 }finally{writeFileSync(release,"release");await Promise.all(pools.map(pool=>pool.close()));rmSync(root,{recursive:true,force:true});}
},30000);
