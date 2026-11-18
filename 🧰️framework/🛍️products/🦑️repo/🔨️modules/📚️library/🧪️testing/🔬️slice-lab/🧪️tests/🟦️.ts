import {describe,expect,test} from "bun:test";
import {join} from "node:path";
import {mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {SLICE_LAB_POLICY_V1 as P,readSliceLabOptionsV1,sliceLabEnvironmentV1,sliceLabImagesV1,sliceLabActivityV1,sliceLabLimitMegabytesV1,sliceLabLiveFilesV1,sliceLabDuplicatesV1,sliceLabFleetSummaryV1,treeChurnV1,sliceLabPhaseV1,sliceLabPlansV1,sliceLabProcessRowsV1,sliceLabReportV1,sliceLabStagesV1,sliceLabTextV1,sliceLabVerdictV1,type SliceLabSampleV1} from "../🟦️.ts";
import {hostProfileV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪞️profile/🟦️.ts";

/** 🔬️ The slice lab: options are closed, the stage plan replaces only what provisions tools, the environment of a machine declares its state root and its host, the verdict names the wall, the process table and the image table are read from their text. */
const options=readSliceLabOptionsV1({machines:2,memoryMegabytes:8192,cores:4,slice:["a","b"],timeoutSeconds:60});
const sample=(patch:Partial<SliceLabSampleV1>={}):SliceLabSampleV1=>({atSeconds:2,machine:"m1",stage:"workspace",state:"running",phase:"taking",done:3,total:40,owners:{},processes:4,busy:2,footprintMegabytes:900,cpu:100,loopLagMs:null,activity:"-",...patch});
describe("slice lab",()=>{
 test("options are closed",()=>{
  expect(()=>readSliceLabOptionsV1({...options,machines:9})).toThrow();
  expect(()=>readSliceLabOptionsV1({...options,memoryMegabytes:12000})).toThrow();
  expect(()=>readSliceLabOptionsV1({...options,extra:1})).toThrow();
 });
 test("the stage plan keeps the five stage names and runs the real read only in workspace",()=>{
  const stages=sliceLabStagesV1(options);
  expect(stages.map(stage=>stage.name)).toEqual(["native","repo-mcp","dependencies","tools","workspace"]);
  expect(stages.slice(0,4).every(stage=>stage.arguments.join(" ")==="--help")).toBe(true);
  expect(stages[4]!.arguments).toEqual(["run-many","--projects=a,b",`--targets=${P.target}`,"--dry-run","--lang=en"]);
 });
 test("a machine declares its state root, its host and a runtime directory that fits a socket",()=>{
  const [first,second]=sliceLabPlansV1(options,"ab12","/tmp/work","/checkout"),environment=sliceLabEnvironmentV1(first!,"/tmp/overlay.ts",{FOO:"1",SEMIO_FLEET_GROUP:"live",SEMIO_REMOTE_CACHE_TOKEN:"x",TMPDIR:"/live"});
  expect(environment.SEMIO_MACHINE_STATE_DIRECTORY).toBe(join("/tmp/work","m1","s"));
  expect(environment.SEMIO_FLEET_GROUP).toBe(P.group);
  expect(environment.SEMIO_REMOTE_CACHE_TOKEN).toBeUndefined();
  expect(environment.FOO).toBe("1");
  expect(environment.BUN_OPTIONS).toBe("--preload=/tmp/overlay.ts");
  expect(environment[P.markerEnvironment]).toBe("ab12");
  expect(Buffer.byteLength(join(first!.runtime,"semio-repo","501","h-123456789abc.sock"))).toBeLessThan(96);
  expect(first!.origin).not.toBe(second!.origin);
  expect(first!.store).toBe(second!.store);
  expect(hostProfileV1(environment,{totalMegabytes:32768,cores:10,declared:false})).toEqual({totalMegabytes:8192,cores:4,declared:true});
  expect(sliceLabLimitMegabytesV1(16384)).toBe(6553);
  expect(sliceLabLimitMegabytesV1(8192)).toBe(3276);
 });
 test("the process table keeps the rows of the run by the marker in the environment",()=>{
  const text=[` 10 1 2048 0.5 0 /bin/bun a ${P.markerEnvironment}=ab12 ${P.machineEnvironment}=m1 HOME=/x`,` 11 10 4096 120.0 5 /bin/repo ${P.markerEnvironment}=ab12 ${P.machineEnvironment}=m2`,` 12 1 100 0.0 0 /bin/other ${P.markerEnvironment}=zz99`,` 13 1 100 0.0 0 /bin/none FOO=1`].join("\n");
  const rows=sliceLabProcessRowsV1(text,"ab12");
  expect(rows.map(row=>[row.pid,row.machine,row.rssMegabytes])).toEqual([[10,"m1",2],[11,"m2",4]]);expect(rows.map(row=>row.nice)).toEqual([0,5]);
 });
 test("the image table keeps the images below a directory with their device",()=>{
  const text=["framework: 1","================================================","image-path      : /work/m1/s/b/g/fixed.dmg","/dev/disk9           \tGUID_partition_scheme","/dev/disk9s1\tApple_APFS","================================================","image-path      : /elsewhere/x.dmg","/dev/disk8\tGUID_partition_scheme"].join("\n");
  expect(sliceLabImagesV1(text,"/work")).toEqual([{path:"/work/m1/s/b/g/fixed.dmg",device:"/dev/disk9"}]);
 });
 test("the phase of the read is parsed from its progress line",()=>{
  expect(sliceLabPhaseV1("Reading the project graph slice (12 s): taken 3 of 40.")).toEqual({phase:"taken",done:3,total:40});
  expect(sliceLabPhaseV1("something else")).toBeUndefined();
 });
 test("the verdict names the wall with stage, phase, text and the last twenty lines",()=>{
  const tail=Array.from({length:30},(_,at)=>`line ${at}`),verdict=sliceLabVerdictV1({options:{...options,machines:1},machines:["m1"],samples:[sample(),sample({atSeconds:4,phase:"inferring",done:5,total:40,footprintMegabytes:4000})],results:{m1:{ready:false,failure:{stage:"workspace",code:"stage-failed",exit:1,tail},timings:[{stage:"workspace",seconds:9,outcome:"stage-failed"}]}},duplicateOwners:0,leftoverProcesses:0,leftoverImages:0,elapsedSeconds:9,load:3,timedOut:false});
  expect(verdict.outcome).toBe("wall");
  expect(verdict.pass).toBe(false);
  expect(verdict.walls[0]).toMatchObject({stage:"workspace",phase:"inferring",code:"stage-failed",text:"line 29"});
  expect(verdict.walls[0]!.tail).toHaveLength(20);
  expect(verdict.withinLimit).toBe(false);
  expect(verdict.peakFootprintMegabytes.m1).toBe(4000);
  expect(sliceLabReportV1("en",options,verdict,[sample()])).toContain("stopped at a wall: stage workspace, phase inferring");
  expect(sliceLabReportV1("de",options,verdict,[sample()])).toContain("an einer Wand gestoppt: Stufe workspace, Phase inferring");
 });
 test("ready on every machine with no leftovers passes, a leftover process fails",()=>{
  const ok={options:{...options,machines:1},machines:["m1"],samples:[sample()],results:{m1:{ready:true,failure:null,timings:[]}},duplicateOwners:0,leftoverProcesses:0,leftoverImages:0,elapsedSeconds:9,load:3,timedOut:false};
  expect(sliceLabVerdictV1(ok)).toMatchObject({outcome:"ready",pass:true});
  expect(sliceLabVerdictV1({...ok,leftoverProcesses:1})).toMatchObject({outcome:"ready",pass:false});
  expect(sliceLabVerdictV1({...ok,results:{}})).toMatchObject({outcome:"timeout",pass:false});
 });
 test("files of the live state that a process holds open are named",()=>{
  const text=["p1","f3","aw","n/live/.cache/f/ready/lease.json","f4","au","n/live/.nx/workspace-data/x","f5","ar","n/live/.cache/b/t/m/bun/x","f6","aw","n/live/src/a.ts","f7","aw","n/work/m1/s/f/x"].join("\n");
  expect(sliceLabLiveFilesV1(text,"/live")).toEqual(["/live/.cache/f/ready/lease.json","/live/.nx/workspace-data/x"]);
 });
 test("a cache entry stored by two machines counts once as duplicate work, a retry of one machine counts nothing",()=>{
  const put=(key:string,status=200,origin="m1")=>({method:"PUT",key,kind:"ac" as const,origin,status,atSeconds:1});
  expect(sliceLabDuplicatesV1([put("a"),put("a",200,"m2"),put("b"),put("b"),put("c",500),put("c",500,"m2")])).toBe(1);
 });
 test("every text exists in English and German",()=>{
  for(const key of ["ready","wall","timeout","slice","skipped","leftovers","duplicates","peak","load","live","nice","lag","busy","published","notExercised","windows","usage"]){expect(sliceLabTextV1("en",key)).not.toBe(key);expect(sliceLabTextV1("de",key)).not.toBe(key);}
 });

 test("the activity is read from the end of the service log: a tool install, a command of the build script, a missing module",()=>{
  expect(sliceLabActivityV1(["[script] {\"owner\":\"repository:build\",\"command\":\"build:source-admission\",\"stage\":\"running\"}","Downloading the pinned cargo-nextest 0.9.140 for darwin-arm64 (1 bytes).","[repository:command] still running after 30 s"].join("\n"))).toBe("tools");
  expect(sliceLabActivityV1(["info: downloading 8 components","[script] {\"owner\":\"repository:build\",\"command\":\"build:selected-declaration\",\"stage\":\"running\"}","[repository:command] still running after 30 s"].join("\n"))).toBe("build:selected-declaration");
  expect(sliceLabActivityV1("Cannot find module '/x/render.ts'\n[capacity] admitted graph-plan: 1 job(s)")).toBe("capacity");
  expect(sliceLabActivityV1("[capacity] admitted graph-plan: 1 job(s)\nCannot find module '/x/render.ts'")).toBe("module-missing");
  expect(sliceLabActivityV1("")).toBe("-");
 });
 test("the verdict fails on a parent loop lag of 200 ms and names the seconds per activity and the busy share",()=>{
  const ok={options:{...options,machines:1},machines:["m1"],samples:[sample({activity:"tools",loopLagMs:20,processes:4,busy:1}),sample({activity:"build:selected-declaration",loopLagMs:150,processes:4,busy:3})],results:{m1:{ready:true,failure:null,timings:[]}},duplicateOwners:0,leftoverProcesses:0,leftoverImages:0,elapsedSeconds:9,load:3,timedOut:false};
  expect(sliceLabVerdictV1({...ok,published:["a".repeat(64)+".json"]})).toMatchObject({pass:true,lagWithin:true,loopLagMaxMs:150,busyShare:50,activitySeconds:{tools:2,"build:selected-declaration":2},published:["a".repeat(64)+".json"]});
  const slow=sliceLabVerdictV1({...ok,samples:[sample({loopLagMs:200})]});
  expect(slow).toMatchObject({lagWithin:false,pass:false});
  expect(sliceLabReportV1("en",options,slow,[sample()])).toContain("maximum 200 ms (limit 200 ms)");
  expect(sliceLabReportV1("de",options,slow,[sample()])).toContain("Maximum 200 ms (Grenze 200 ms)");
 });
 test("a run that executes drops --dry-run and names the stagger in its options",()=>{
  const run=readSliceLabOptionsV1({...options,execute:true,stagger:-1});
  expect(sliceLabStagesV1(run)[4]!.arguments).toEqual(["run-many","--projects=a,b",`--targets=${P.target}`,"--lang=en"]);
  expect(()=>readSliceLabOptionsV1({...options,stagger:-2})).toThrow();
  expect(()=>readSliceLabOptionsV1({...options,execute:"yes"})).toThrow();
 });
 test("the verdict reports the owners a machine had at most and the seconds from its own start to ready",()=>{
  const ledger=(present:number,fetched:number,here:number)=>({total:10,present,fetched,here});
  const verdict=sliceLabVerdictV1({options,machines:["m1","m2"],samples:[sample({machine:"m1",atSeconds:30,owners:ledger(0,0,10)}),sample({machine:"m2",atSeconds:50,owners:ledger(0,4,0)}),sample({machine:"m2",atSeconds:60,owners:ledger(2,8,0)})],results:{m1:{ready:true,failure:null,timings:[]},m2:{ready:true,failure:null,timings:[]}},duplicateOwners:0,leftoverProcesses:0,leftoverImages:0,elapsedSeconds:70,load:3,timedOut:false,starts:{m1:0,m2:40},detached:4});
  expect(verdict.ownersSeen).toEqual({m1:{total:10,present:0,fetched:0,here:10},m2:{total:10,present:2,fetched:8,here:0}});
  expect(verdict.readySeconds).toEqual({m1:30,m2:20});
  expect(verdict.detached).toBe(4);
  expect(sliceLabReportV1("en",options,verdict,[])).toContain("2 / 8 / 0 of 10");
  expect(sliceLabReportV1("de",options,verdict,[])).toContain("2 / 8 / 0 of 10");
 });
 test("the fleet summary counts the requests by origin, kind, method and status",()=>{
  const request=(origin:string,method:string,status:number)=>({method,key:"k",kind:"ac" as const,origin,status,atSeconds:1});
  expect(sliceLabFleetSummaryV1([request("a","PUT",200),request("a","PUT",200),request("b","GET",200),request("b","GET",404)])).toEqual({"a ac PUT 200":2,"b ac GET 200":1,"b ac GET 404":1});
 });
 test("the churn of the tree counts the files below framework that changed after the marker",()=>{
  const root=mkdtempSync(join(tmpdir(),"churn-")),marker=join(root,"marker");
  try{
   mkdirSync(join(root,"🧰️framework","a"),{recursive:true});mkdirSync(join(root,"🧰️framework","node_modules"),{recursive:true});
   writeFileSync(join(root,"🧰️framework","a","old.ts"),"1");writeFileSync(marker,"");
   const past=new Date(Date.now()-60000),future=new Date(Date.now()+60000);
   utimesSync(join(root,"🧰️framework","a","old.ts"),past,past);utimesSync(marker,new Date(),new Date());
   writeFileSync(join(root,"🧰️framework","a","new.ts"),"2");utimesSync(join(root,"🧰️framework","a","new.ts"),future,future);
   writeFileSync(join(root,"🧰️framework","node_modules","dep.js"),"3");utimesSync(join(root,"🧰️framework","node_modules","dep.js"),future,future);
   expect(treeChurnV1(root,marker)).toEqual({count:1,sample:[join("🧰️framework","a","new.ts")]});
  }finally{rmSync(root,{recursive:true,force:true});}
 });
});
