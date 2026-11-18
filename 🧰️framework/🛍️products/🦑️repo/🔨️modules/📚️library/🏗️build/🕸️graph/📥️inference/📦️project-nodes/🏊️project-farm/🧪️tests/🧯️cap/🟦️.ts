/** 🧯️ Contract: a worker above its footprint cap is stopped and its job runs alone; a job above the alone cap fails with a typed bilingual refusal naming it; nothing else is lost. The decision table lives in `🔣️.json` and is evaluated by an independent Python model; the real footprint of a real child is checked against `ps`.
 * @see ../../🏊️pool/🟨️.mjs */
import{expect,test}from"bun:test";
import{mkdtempSync,readFileSync,readdirSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath,pathToFileURL}from"node:url";
import{testPythonV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{GRAPH_FARM_POLICY_V1,GRAPH_FARM_MESSAGES_V1,GraphFarmOverrunErrorV1,createFarmV1,farmProcessAliveV1}from"../../🏊️pool/🟨️.mjs";
import{FOOTPRINT_POLICY_V1}from"../../../../../../🧠️memory/🦶️footprint/🟨️.mjs";
import{PLAN_STAGE_POLICY_V1}from"../../../../../../🧠️memory/🪜️plan-stage/🟨️.mjs";
import fixture from"./🔣️.json";

const MiB=1048576,here=dirname(fileURLToPath(import.meta.url)),worker=join(here,"..","..","👷️worker","🟨️.mjs"),service=pathToFileURL(join(here,"🧫️fixtures","🟨️.mjs")).href,python=testPythonV1(),oracle=join(here,"🐍️.py");
type Event={event:string;fields:Record<string,unknown>};
const open=(options:Record<string,unknown>,events:Event[],workers=3)=>createFarmV1({entry:worker,service,session:{identity:"law"},workers,megabytes:100000,environment:{...process.env},accept:(identity:unknown)=>identity==="law",report:(event:string,fields:Record<string,unknown>)=>{events.push({event,fields});},policy:{...GRAPH_FARM_POLICY_V1,footprintMilliseconds:25,terminateGraceMilliseconds:1000,holdRecheckMilliseconds:20},capMegabytes:fixture.capMegabytes,aloneCapMegabytes:fixture.aloneCapMegabytes,...options}as never);
const scripted=(dir:string)=>async(pid:number)=>{try{return Number(readFileSync(join(dir,"pid-"+pid),"utf8"))*MiB;}catch{return undefined;}};
const classify=(answer:any)=>answer.overrun!==undefined?"overrun":answer.reply.run>0?"alone-reply":"reply";

test("the footprint cap stops a worker above it, runs its job alone and fails a job that overruns alone, exactly as the independent model says",async()=>{
 expect(python,"python3 is required for the independent model").toBeTruthy();
 for(const[index,row]of fixture.cases.entries()){
  const model=JSON.parse(Bun.spawnSync([python!,"-I",oracle,join(here,"🔣️.json"),String(index)]).stdout.toString()),dir=mkdtempSync(join(tmpdir(),"farm-cap-")),events:Event[]=[],pool=open({footprintOf:scripted(dir)},events);
  try{
   const answers:any[]=await Promise.all(row.jobs.map(job=>pool.run({n:job.n,name:"job-"+job.n,megabytes:job.megabytes,delay:500,dir})));
   const actual:Record<string,string>={};answers.forEach((answer,at)=>{actual[String(row.jobs[at]!.n)]=classify(answer);});
   expect(actual,row.name).toEqual(model.outcomes);
   expect(events.filter(each=>each.event==="capped").length,row.name+": stopped runs "+JSON.stringify(events.filter(each=>each.event==="capped"))).toBe(model.stopped);
   const alone=answers.filter(answer=>answer.reply?.run>0).map(answer=>answer.reply),others=answers.filter(answer=>answer.reply!==undefined).map(answer=>answer.reply);
   for(const one of alone)for(const other of others)if(other.n!==one.n)expect(other.start>=one.end||other.end<=one.start,row.name+": job "+other.n+" ran beside alone job "+one.n).toBe(true);
   for(const answer of answers)if(answer.overrun)expect(answer.overrun.capMegabytes).toBe(fixture.aloneCapMegabytes);
   expect(pool.stats().restarts,"a capped worker is no lost worker").toBe(0);expect(pool.stats().lost).toBe(0);
  }finally{await pool.close();rmSync(dir,{recursive:true,force:true});}
 }
},180000);

test("the refusal of an overrun names the owner and both numbers in English and German, from the declared texts",()=>{
 const error=new GraphFarmOverrunErrorV1({project:"@semio-tech/heavy-owner",megabytes:4300,capMegabytes:4200},{});
 expect(error.name).toBe("GraphFarmOverrunErrorV1");expect(error.code).toBe("graph.farm-overrun");expect(error.project).toBe("@semio-tech/heavy-owner");
 const lines=error.message.split("\n");expect(lines.length).toBe(2);
 for(const line of lines){expect(line).toContain("@semio-tech/heavy-owner");expect(line).toContain("4300");expect(line).toContain("4200");}
 expect(lines[0]).toBe(GRAPH_FARM_MESSAGES_V1.en.overrun.replaceAll("{project}","@semio-tech/heavy-owner").replace("{megabytes}","4300").replace("{cap}","4200"));
 expect(lines[1]).toBe(GRAPH_FARM_MESSAGES_V1.de.overrun.replaceAll("{project}","@semio-tech/heavy-owner").replace("{megabytes}","4300").replace("{cap}","4200"));
 for(const key of["capped","cappedAlone","overrun"]){expect(Object.keys(GRAPH_FARM_MESSAGES_V1.en)).toContain(key);expect(Object.keys(GRAPH_FARM_MESSAGES_V1.de)).toContain(key);}
});

test("a real child that holds memory is read at its real footprint, stopped by SIGTERM and gone, and its job passes alone; a job above the alone cap fails",async()=>{
 const dir=mkdtempSync(join(tmpdir(),"farm-cap-real-")),events:Event[]=[],pool=open({capMegabytes:350,aloneCapMegabytes:650},events,2);
 try{
  const fits:any=await pool.run({n:0,megabytes:[500,500],real:true,delay:2500,dir});
  expect(fits.reply.run,"second run").toBe(1);expect(fits.reply.held).toBe(500*MiB);
  expect(fits.reply.rssMegabytes,"ps agrees that the child holds what it allocated").toBeGreaterThanOrEqual(500);
  expect(fits.footprintMegabytes,"the pool read the real footprint").toBeGreaterThanOrEqual(450);
  const capped=events.find(each=>each.event==="capped");expect(capped?.fields.alone).toBe(false);expect(Number(capped?.fields.megabytes)).toBeGreaterThan(350);expect(capped?.fields.project).toBeUndefined();
  const pids=readdirSync(dir).filter(name=>name.startsWith("pid-")).map(name=>Number(name.slice(4))).filter(pid=>pid!==fits.reply.pid);
  expect(pids.length).toBe(1);
  const until=Date.now()+10000;while(farmProcessAliveV1(pids[0]!)&&Date.now()<until)await Bun.sleep(50);
  expect(farmProcessAliveV1(pids[0]!),"the stopped child is gone").toBe(false);
  const heavy:any=await pool.run({n:1,megabytes:[800,800],real:true,delay:2500,dir});
  expect(heavy.overrun.capMegabytes).toBe(650);expect(heavy.overrun.megabytes).toBeGreaterThan(650);
  expect(events.filter(each=>each.event==="capped").length).toBe(3);
 }finally{await pool.close();rmSync(dir,{recursive:true,force:true});}
},180000);

test("the declared caps agree with the declared peak, the measured children and the hard share of the plan ceiling, and the pool takes them from the plan stage policy",()=>{
 const P=GRAPH_FARM_POLICY_V1,F=FOOTPRINT_POLICY_V1,S=PLAN_STAGE_POLICY_V1,C=S.farmChildFootprintMegabytes["project-farm"];
 const limit=6553*F.hardPercent/100,declared=S.farmWorkerPeakMegabytes["project-farm"];
 expect(declared,"the declared peak holds the measured one").toBeGreaterThanOrEqual(930);
 expect(C.typicalMegabytes,"the typical child holds the measured maximum of a normal child").toBeGreaterThanOrEqual(647);
 expect(C.capMegabytes,"a normal child never trips the cap").toBeGreaterThan(declared);
 expect(C.aloneCapMegabytes).toBeGreaterThan(C.capMegabytes);
 expect(S.parentFootprintMegabytes+C.typicalMegabytes+C.capMegabytes,"parent, one sibling at its typical size and a capped child").toBeLessThanOrEqual(limit);
 expect(S.parentFootprintMegabytes+C.aloneCapMegabytes,"parent and one child alone").toBeLessThanOrEqual(limit);
 expect("childFootprintCapMegabytes" in P,"one source of the caps").toBe(false);
 expect(P.footprintMilliseconds).toBeGreaterThan(0);expect(P.footprintMilliseconds).toBeLessThanOrEqual(1000);expect(P.terminateGraceMilliseconds).toBeGreaterThan(0);
});
