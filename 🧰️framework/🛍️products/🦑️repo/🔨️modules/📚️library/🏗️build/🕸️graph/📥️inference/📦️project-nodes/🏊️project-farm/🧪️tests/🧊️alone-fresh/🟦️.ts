/** 🧊️ Contract: a job that runs alone runs in a child that served no job before, so the footprint recorded as "this owner needed N MB even when it ran alone" is the owner's own and never the residue of the jobs an earlier child kept (the stops of 4.2 to 7.2 GB recorded for owners that measure 0.6 GB in a fresh process).
 * @see ../../🏊️pool/🟨️.mjs */
import{expect,test}from"bun:test";
import{mkdtempSync,readdirSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath,pathToFileURL}from"node:url";
import{GRAPH_FARM_POLICY_V1,createFarmV1}from"../../🏊️pool/🟨️.mjs";

const here=dirname(fileURLToPath(import.meta.url)),worker=join(here,"..","..","👷️worker","🟨️.mjs"),service=pathToFileURL(join(here,"..","🧯️cap","🧫️fixtures","🟨️.mjs")).href;
type Event={event:string;fields:Record<string,unknown>};
const open=(events:Event[],workers:number)=>createFarmV1({entry:worker,service,session:{identity:"law"},workers,megabytes:100000,environment:{...process.env},accept:(identity:unknown)=>identity==="law",report:(event:string,fields:Record<string,unknown>)=>{events.push({event,fields});},policy:{...GRAPH_FARM_POLICY_V1,footprintMilliseconds:25,terminateGraceMilliseconds:1000,holdRecheckMilliseconds:20},capMegabytes:350,aloneCapMegabytes:650}as never);

test("a job that passes the cap runs alone in a child that served no job before, never in one that kept the residue of earlier jobs",async()=>{
 const dir=mkdtempSync(join(tmpdir(),"farm-fresh-")),events:Event[]=[],pool=open(events,3);
 try{
  const warm:any[]=await Promise.all([0,1,2].map(n=>pool.run({n,megabytes:[40],real:true,delay:600,dir})));
  const served=new Set(warm.map(answer=>answer.reply.pid));
  expect(served.size,"three children served one small job each").toBe(3);
  const heavy:any=await pool.run({n:9,megabytes:[500,100],real:true,delay:600,dir});
  expect(heavy.reply?.run,"the second run is the alone run").toBe(1);
  expect(served.has(heavy.reply.pid),"the alone run is in a fresh child").toBe(false);
  const first=readdirSync(dir).filter(name=>name.startsWith("pid-")).length;
  expect(first).toBeGreaterThan(0);
  expect(events.filter(each=>each.event==="capped"&&each.fields.alone===false).length).toBe(1);
 }finally{await pool.close();rmSync(dir,{recursive:true,force:true});}
},180000);
