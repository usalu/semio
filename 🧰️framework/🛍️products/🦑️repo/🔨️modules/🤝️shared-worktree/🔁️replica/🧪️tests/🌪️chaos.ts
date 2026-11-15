import {expect,test} from "bun:test";
import {writeFileSync} from "node:fs";
import {join} from "node:path";
import {buildFleetV1,checkFleetV1,disposeFleetV1,gitV1,performV1,replayV1,rngV1,runChaosV1,settleV1,shrinkV1,type ChaosOpV1,type ChaosReportV1} from "./🌪️harness.ts";

const posixTest=process.platform==="win32"||Bun.spawnSync(["git","--version"]).exitCode!==0?test.skip:test;
const quickSeeds=[1,2,3,4,5,6,8,9,10,11,12,13],replay=process.env.SEMIO_CHAOS_SEED,soak=Number(process.env.SEMIO_CHAOS_SOAK??"0"),machines=Number(process.env.SEMIO_CHAOS_MACHINES??"3"),steps=Number(process.env.SEMIO_CHAOS_STEPS??"25");
const soakTest=soak>0&&process.platform!=="win32"?test:test.skip;

const explain=(report:ChaosReportV1):string=>`seed ${report.seed} (replay: SEMIO_CHAOS_SEED=${report.seed} SEMIO_CHAOS_MACHINES=${report.machines} SEMIO_CHAOS_STEPS=${report.ops.length})\n${report.violations.join("\n")}\ntrace:\n${report.notes.map((note,index)=>`${index} ${JSON.stringify(report.ops[index]).slice(0,160)} -> ${note}`).join("\n")}`;
const op=(value:ChaosOpV1):ChaosOpV1=>value;
const all=(count:number,rounds:number=6):ChaosOpV1=>op({op:"rounds",m:0,set:Array.from({length:count},(_,index)=>index),count:rounds});

test("the generator is deterministic per seed, differs between seeds, stays in range and has a fixed first draw",()=>{
 const draw=(seed:number)=>{const rng=rngV1(seed);return Array.from({length:6},()=>rng.next());};
 expect(draw(7)).toEqual(draw(7));
 expect(draw(7)).not.toEqual(draw(8));
 expect(draw(7).every(value=>value>=0&&value<1)).toBe(true);
 expect(draw(1)[0]).toBeCloseTo(0.6270739405881613,12);
 const rng=rngV1(3);
 expect(Array.from({length:200},()=>rng.int(5)).every(value=>value>=0&&value<5)).toBe(true);
 expect(rngV1(3).hex(8)).toMatch(/^[0-9a-f]{8}$/u);
});

test("the shrinker keeps exactly the operations the failure needs, in order, within its budget",async()=>{
 const items=Array.from({length:40},(_,index)=>index);
 const needs=async(candidate:readonly number[])=>candidate.includes(7)&&candidate.includes(31)&&candidate.indexOf(7)<candidate.indexOf(31);
 expect(await shrinkV1(items,needs)).toEqual([7,31]);
 let runs=0;
 expect((await shrinkV1(items,async()=>{runs++;return true;},5)).length).toBeLessThan(40);
 expect(runs).toBeLessThanOrEqual(5);
 expect(await shrinkV1([1,2,3],async()=>false)).toEqual([1,2,3]);
});

posixTest("negative controls: an unpublished local edit, an unpublished commit and a stray stage entry on one machine are all reported by the invariant check",async()=>{
 const fleet=await buildFleetV1(3);
 try{
  expect(await settleV1(fleet)).toBeGreaterThan(0);
  expect(await checkFleetV1(fleet)).toEqual([]);
  await performV1(fleet,op({op:"create",m:1,path:"stray.txt",text:"only here\n"}));
  await performV1(fleet,op({op:"add",m:1,files:["stray.txt"]}));
  await performV1(fleet,op({op:"commit",m:1,message:"unpublished"}));
  await performV1(fleet,op({op:"create",m:2,path:"untracked.txt",text:"only there\n"}));
  fleet.messages.pop();
  const bad=(await checkFleetV1(fleet)).map(item=>item.split(":")[0]);
  for(const category of ["tree","head","write-tree","stage","status","oracle","history","verify","verdict"])expect(bad).toContain(category);
  expect(gitV1(fleet.machines[1]!,["log","-1","--format=%s"]).out.trim()).toBe("unpublished");
 }finally{await disposeFleetV1(fleet);}
},120_000);

posixTest("a fixed trace replays to a converged fleet: stage, commit, in-place and atomic edits, rename, symlink, executable bit, crash restart, repair, rebase and a short outage",async()=>{
 const trace:ChaosOpV1[]=[
  op({op:"create",m:1,path:"d0/f0.txt",text:"one\n"}),op({op:"add",m:1,files:["d0/f0.txt"]}),op({op:"outage",m:2,down:true}),
  op({op:"modify",m:0,path:"a.txt",text:"xdeadbeef\n"}),op({op:"replace",m:0,path:"b.txt",text:"xcafebabe\n"}),op({op:"rename",m:1,path:"d0/f0.txt",to:"d1/f1.txt"}),
  op({op:"chmod",m:0,path:"a.txt",exec:true}),op({op:"symlink",m:0,path:"l0",target:"b.txt"}),op({op:"commit",m:0,message:"from-0"}),op({op:"commit",m:1,message:"from-1"}),
  op({op:"crash",m:1}),op({op:"repair",m:0}),op({op:"rebase",m:1}),op({op:"outage",m:2,down:false}),op({op:"restart",m:0}),all(3,2),
  op({op:"tree",m:2,files:["t0/a/c/x.txt","t0/b/c/y.txt"],text:"x00000001\n"}),op({op:"delete",m:2,path:"dir/c.txt"}),op({op:"status",m:0}),
 ];
 const report=await replayV1(3,trace,{restartCheck:true});
 expect(report.violations,report.violations.join("\n")).toEqual([]);
},180_000);

posixTest("seeded chaos converges (quick, fixed seeds)",async()=>{
 const failures:string[]=[];
 for(const seed of replay===undefined?quickSeeds:[Number(replay)]){
  const report=await runChaosV1({seed,machines,steps,restartCheck:true,profile:process.env.SEMIO_CHAOS_PROFILE==="hostile"?"hostile":"mixed",loose:process.env.SEMIO_CHAOS_LOOSE==="1"});
  if(report.violations.length>0)failures.push(explain(report));
 }
 expect(failures,failures.join("\n\n")).toEqual([]);
},900_000);

soakTest("seeded chaos converges (soak: SEMIO_CHAOS_SOAK seconds of fresh seeds on 4 to 5 machines)",async()=>{
 const end=Date.now()+soak*1000,base=Number(process.env.SEMIO_CHAOS_BASE??Date.now()%1_000_000_000),failures:string[]=[];
 let seed=base,runs=0;
 console.log(`[chaos soak] base seed ${base}`);
 while(Date.now()<end&&failures.length<3){
  const report=await runChaosV1({seed,machines:4+(seed%2),steps:Number(process.env.SEMIO_CHAOS_STEPS??"120"),restartCheck:true});
  runs++;seed++;
  if(report.violations.length>0){failures.push(explain(report));writeFileSync(join(process.env.TMPDIR??"/tmp",`chaos-soak-${report.seed}.json`),JSON.stringify(report,null,1));}
 }
 console.log(`[chaos soak] ${runs} runs, ${failures.length} failures`);
 expect(failures,failures.join("\n\n")).toEqual([]);
},soak*1000+600_000);

posixTest("D1: a case-only rename on one machine and an edit of the old name on another leave b.txt and B.txt live at once; each machine queues the other forever and the fleet stays lagging",async()=>{
 const report=await replayV1(3,[op({op:"case",m:0,path:"b.txt",loose:true}),op({op:"modify",m:1,path:"b.txt",text:"xbbbbbbbb\n"})],{settleMilliseconds:8000});
 expect(report.violations,report.violations.join("\n")).toEqual([]);
},120_000);

posixTest("D2: a directory replaced by a file on one machine and an edit inside the directory on another leave both registers live; the file/directory conflict queues forever and an EEXIST error event is emitted",async()=>{
 const report=await replayV1(3,[op({op:"dirswap",m:0,path:"dir",text:"xaaaaaaaa\n",loose:true}),op({op:"modify",m:1,path:"dir/c.txt",text:"xbbbbbbbb\n"})],{settleMilliseconds:8000});
 expect(report.violations,report.violations.join("\n")).toEqual([]);
},120_000);

posixTest("D3: a remote put below a path that is a symlink locally is written through the symlink, so the file lands at the link target (dir/x.txt) and is then published as a new path",async()=>{
 const report=await replayV1(3,[op({op:"symlink",m:0,path:"ln",target:"dir"}),op({op:"create",m:1,path:"ln/x.txt",text:"xaaaaaaaa\n"})],{settleMilliseconds:8000});
 expect(report.violations,report.violations.join("\n")).toEqual([]);
},120_000);

posixTest("D4: 40 files deleted and 40 symlinks created in the same directory in one publish make the applying machine emit ENOENT (symlink) or EEXIST (mkdir) error events, because the link branch of materialise has no retry against the parallel prune of the emptied directory",async()=>{
 const names=Array.from({length:40},(_,index)=>`p/a${index}.txt`);
 const trace:ChaosOpV1[]=[op({op:"tree",m:0,files:names,text:"xaaaaaaaa\n"}),all(3,4),...names.map(path=>op({op:"delete",m:0,path})),...names.map((_,index)=>op({op:"symlink",m:0,path:`p/l${index}`,target:"b.txt"}))];
 const found:string[]=[];
 for(let attempt=0;attempt<5;attempt++)found.push(...(await replayV1(3,trace,{settleMilliseconds:8000})).violations.filter(item=>item.startsWith("error")));
 expect(found,found.join("\n")).toEqual([]);
},300_000);
