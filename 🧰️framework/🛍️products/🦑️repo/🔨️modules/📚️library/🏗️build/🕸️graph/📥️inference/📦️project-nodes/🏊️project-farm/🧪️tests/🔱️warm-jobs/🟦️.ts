/** 🔱️ Contract: a warm job weighs almost nothing for the recycling of a farm child: a child that is replaced after four derivations serves hundreds of warm jobs on one process, while a derivation still counts one and the memory ceiling still replaces a child.
 * @see ../../🏊️pool/🟨️.mjs */
import{expect,test}from"bun:test";
import{dirname,join}from"node:path";
import{fileURLToPath,pathToFileURL}from"node:url";
import{GRAPH_FARM_POLICY_V1,createFarmV1}from"../../🏊️pool/🟨️.mjs";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";

const here=dirname(fileURLToPath(import.meta.url)),worker=join(here,"..","..","👷️worker","🟨️.mjs"),service=pathToFileURL(join(here,"🧫️fixtures","🟨️.mjs")).href;
const farm=(options:Record<string,unknown>={})=>createFarmV1({entry:worker,service,session:{identity:"law"},workers:1,megabytes:100000,environment:{...process.env},accept:(identity:unknown)=>identity==="law",policy:{...GRAPH_FARM_POLICY_V1,recycleAfterProjects:4},...options} as never);

test("120 warm jobs run on one child process while 8 derivations replace it twice",async()=>{
 const pool=farm(),pids=new Set<number>();
 for(let n=0;n<120;n++)pids.add(((await pool.run({warm:true,n})).reply as{pid:number}).pid);
 expect(pids.size).toBe(1);expect(pool.stats().recycled).toBe(0);
 for(let n=0;n<8;n++)await pool.run({n});
 expect(pool.stats().recycled).toBeGreaterThanOrEqual(2);
 await pool.close();
},120000);

test("the memory ceiling still replaces a child that serves warm jobs",async()=>{
 const pool=farm({megabytes:1,policy:{...GRAPH_FARM_POLICY_V1,recycleAfterProjects:4,recycleCeilingFactor:0}});
 for(let n=0;n<3;n++)await pool.run({warm:true,n});
 expect(pool.stats().recycled).toBe(3);
 await pool.close();
},60000);

test("real children that die now and then never disable the pool: the restart budget counts the deaths in a row, not the deaths of a day",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"semio-die-")),pool=farm();let replies=0;
 for(let n=0;n<30;n++){const outcome=await pool.run({warm:true,n,...(n%3===1?{dieFile:join(directory,"die-"+n)}:{})});if(outcome.reply!==undefined)replies++;}
 expect(replies).toBe(30);expect(pool.stats().restarts).toBeLessThanOrEqual(1);expect(pool.usable).toBe(true);
 await pool.close();rmSync(directory,{recursive:true,force:true});
},120000);
