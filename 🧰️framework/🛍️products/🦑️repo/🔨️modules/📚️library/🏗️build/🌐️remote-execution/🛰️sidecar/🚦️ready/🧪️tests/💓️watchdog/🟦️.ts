import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{spawn}from"node:child_process";
import{appendFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,statSync,writeFileSync}from"node:fs";
import{join,resolve}from"node:path";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};
import{READY_POLICY_V1 as P,READY_STAGES_V1 as STAGES,readyTreeMegabytesV1,runReadyStageV1,type ReadyInflightV1}from"../../🟦️.ts";
import{READY_TREE_POLICY_V1 as TREE}from"../../🌲️process-tree/🟦️.ts";

const CHILD=`const fs=require("node:fs");const [log,end,busy,stop]=process.argv.slice(1);fs.writeFileSync(log,busy==="step"?"Reading the project graph slice (9000 s, beyond the expected 2 s): waiting for owners / Warte auf Projekte.\\n":"taken 1 of 9 owners\\n");
const finish=exit=>{fs.writeFileSync(end,JSON.stringify({exit,signal:null,spawnFailed:false}));process.exit(exit);};
process.on("SIGTERM",()=>finish(143));process.on("SIGINT",()=>finish(130));
if(busy==="cpu")setInterval(()=>{const until=performance.now()+90;while(performance.now()<until){}const cpu=process.cpuUsage();fs.writeFileSync(stop+".cpu",String((cpu.user+cpu.system)/1000));},100);
setInterval(()=>{if(fs.existsSync(stop))finish(0);},100);setTimeout(()=>finish(0),16000);`;

for(const row of corpus.cases)test("watchdog adoption preserves "+row.id,async()=>{
 const scratch=process.env.SEMIO_TEST_ARTIFACT_DIR;
 if(!scratch)throw Error("Test artifact directory required");
 const root=mkdtempSync(join(resolve(scratch),"wd-")),directory=join(root,...P.directory);
 mkdirSync(directory,{recursive:true});
 const file=join(directory,P.inflightName),log=join(directory,"tools.log"),end=join(directory,"tools.exit"),semantics=join(root,"semantics.json"),stop=join(root,"stop"),startedMs=Date.now();
 writeFileSync(semantics,JSON.stringify({revision:1}));
 const stage={...STAGES.find(stage=>stage.name==="tools")!,timeoutMilliseconds:60000};
 const tree={...TREE,stallStopMilliseconds:2500,staleStageMilliseconds:row.mode==="semantics"?4500:TREE.staleStageMilliseconds,codeQuietMilliseconds:row.mode==="semantics"?900:TREE.codeQuietMilliseconds,semanticsSources:[{file:"semantics.json",pointer:"/revision"}]};
 const child=spawn(process.execPath,["-e",CHILD,log,end,row.mode,stop],{cwd:root,stdio:"ignore",windowsHide:true});
 const exited=new Promise<void>(done=>child.once("exit",()=>done()));
 const db=new Database(":memory:");
 try{
  expect(child.pid).toBeGreaterThan(0);
  const active={stage:stage.name,pid:child.pid!,startedMs,exitFile:end,log,supervisor:process.pid};
  for(let count=0;!existsSync(log)&&count<150;count++)await Bun.sleep(20);
  expect(existsSync(log)).toBe(true);
  writeFileSync(file,JSON.stringify(row.mode==="malformed"?{...active,watchdog:{version:1,progressAt:Number.MAX_SAFE_INTEGER}}:active));
  const run=(signal:AbortSignal)=>runReadyStageV1({root,environment:{},stage,tree,speed:1,signal,jobs:1,onLine:()=>undefined});
  const detach=async(delay:number)=>{
   const controller=new AbortController(),timer=setTimeout(()=>controller.abort(),delay);
   try{expect(await run(controller.signal)).toMatchObject({detached:true,startedMs});}
   finally{clearTimeout(timer);}
   const record=JSON.parse(readFileSync(file,"utf8")) as ReadyInflightV1;
   expect(record).toMatchObject(active);
   expect(record.watchdog,JSON.stringify(schema.definitions.Watchdog.required)).toBeDefined();
   return record.watchdog!;
  };
  const originalLogTime=Math.floor(statSync(log).mtimeMs),first=await detach(700);
  expect(first.stallAt).toBe(Math.max(startedMs,originalLogTime));
  if("priorMegabytes" in row){
   const current=JSON.parse(readFileSync(file,"utf8")) as ReadyInflightV1;
   const owner=row.mode==="foreign"?"0".repeat(64):first.owner;
   writeFileSync(file,JSON.stringify({...current,watchdog:{...first,owner,peakMegabytes:row.priorMegabytes}}));
  }
  if(row.mode==="vanished"){
   child.kill("SIGKILL");await exited;
   expect(existsSync(end)).toBe(false);
   const result=await run(new AbortController().signal);
   const expected=db.query<{peak:number},[number,number]>("select max(?1,?2) peak").get(row.priorMegabytes,row.nextMegabytes)!.peak;
   expect(result).toMatchObject({startedMs,peakMegabytes:expected,spawnFailed:false,exit:null});
   expect(result.tail.join("\n")).toContain("vanished without an exit status");
   return;
  }
  if(row.mode==="output")appendFileSync(log,"taken 2 of 9 owners\nNX   Running target build for project x and 2 tasks they depend on:\n");
  if(row.mode==="semantics")writeFileSync(semantics,JSON.stringify({revision:2}));
  if(row.mode==="cpu"&&process.platform!=="win32"){
   const baseline=new Map(first.cpu).get(child.pid!);
   expect(baseline).toBeDefined();
   const target=baseline!+tree.livenessCpuMilliseconds+100,limit=performance.now()+8000;
   const observed=()=>existsSync(stop+".cpu")?Number(readFileSync(stop+".cpu","utf8")):0;
   while(observed()<target&&performance.now()<limit)await Bun.sleep(30);
   expect(observed()).toBeGreaterThanOrEqual(target);
  }
  if(row.mode==="gap")await Bun.sleep(tree.stallStopMilliseconds+500);
  const second=await detach(700);
  if(row.mode==="gap"){expect(second.unwatchedMilliseconds).toBeGreaterThanOrEqual(tree.stallStopMilliseconds+500);expect(second.stallAt).toBe(first.stallAt);}
  else if(row.mode==="quiet")expect(second.unwatchedMilliseconds).toBeLessThan(tree.stallStopMilliseconds);
  const third=await detach(700);
  if(row.mode==="foreign")expect(second.armedAt).toBeGreaterThan(first.armedAt);
  else expect(second.armedAt).toBe(first.armedAt);
  expect(third.armedAt).toBe(second.armedAt);
  expect(third.semantics).toEqual(first.semantics);
  expect(third.phaseAt).toBe(second.phaseAt);
  if(row.mode!=="output")expect(third.phaseAt).toBe(first.phaseAt);
  const cpuAvailable=process.platform!=="win32";
  const expected=db.query<{advances:number},[string,number]>("select (?1='output' or (?1='cpu' and ?2=1)) advances").get(row.mode,Number(cpuAvailable))!;
  expect(second.stallAt>first.stallAt).toBe(Boolean(expected.advances));
  expect(second.progressAt>first.progressAt).toBe(Boolean(expected.advances));
  if(row.mode==="output"){
   expect(second.offset).toBeGreaterThan(first.offset);
   expect(second.phase).toBe("executing");
   expect(second.phaseAt).toBeGreaterThan(first.phaseAt);
   expect(second.step.movedAt).toBeGreaterThan(first.step.movedAt!);
  }
  if(row.mode!=="cpu"){
   expect(third.stallAt).toBe(second.stallAt);
   expect(third.step.movedAt).toBe(second.step.movedAt);
   expect(third.offset).toBe(second.offset);
  }
  if("priorMegabytes" in row){
   const sample=readyTreeMegabytesV1(child.pid+" 1 "+row.nextMegabytes*1024,child.pid!);
   expect(sample).toBe(row.nextMegabytes);
   const expected=db.query<{peak:number},[number,number]>("select max(?1,?2) peak").get(row.priorMegabytes,sample)!.peak;
   if(row.mode==="peak")expect(third.peakMegabytes).toBe(expected);
   else expect(third.peakMegabytes).toBeLessThan(expected);
   writeFileSync(stop,"");
   const result=await run(new AbortController().signal);
   expect(result).toMatchObject({exit:0,detached:false,startedMs});
   if(row.mode==="peak")expect(result.peakMegabytes).toBe(expected);
   else expect(result.peakMegabytes??0).toBeLessThan(expected);
  }
  if(row.mode==="quiet"||row.mode==="gap"||row.mode==="semantics"||row.mode==="step"){
   const result=await run(new AbortController().signal);
   expect(result).toMatchObject({detached:false,startedMs,stalled:true});
   if(row.mode==="semantics")expect(result.codeChanged).toBe("semantics.json");
   if(row.mode==="step")expect(result.stepOverdue?.text).toBe("waiting for owners / Warte auf Projekte");
   expect(existsSync(file)).toBe(false);
  }
 }finally{
  db.close();
  if(child.exitCode===null&&child.signalCode===null)child.kill("SIGKILL");
  await exited;
  rmSync(root,{recursive:true,force:true});
 }
},35000);
