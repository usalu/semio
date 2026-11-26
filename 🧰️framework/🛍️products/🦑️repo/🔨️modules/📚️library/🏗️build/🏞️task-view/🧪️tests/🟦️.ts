import {afterAll,expect,test} from "bun:test";
import {existsSync,lstatSync,readFileSync,readdirSync,readlinkSync,realpathSync,renameSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {TASK_VIEW_POLICY_V1,TASK_VIEW_PORTS_V1,digestTaskViewInputsV1,openTaskViewV1,taskRootModeV1,taskViewInputsDigestV1,type TaskViewPortV1} from "../🟦️.ts";
import {growTreeV1,hasOracleV1,oracleV1,removeV1,scratchV1,sleepV1,snapshotV1} from "./🧰️world/🟦️.ts";

const scratch:string[]=[];
const world=(tag:string)=>{const root=scratchV1(tag);scratch.push(root);return root;};
afterAll(()=>{for(const root of scratch)removeV1(root);});
const ready=<T extends {kind:string}>(outcome:T):Extract<T,{kind:"ready"}>=>{expect(outcome.kind,JSON.stringify(outcome)).toBe("ready");return outcome as Extract<T,{kind:"ready"}>;};

test("the execution root is one switch: the environment first, then the class of the task, then the default, and the policy is closed",()=>{
 const policy=TASK_VIEW_POLICY_V1;
 expect(Object.keys(policy).sort()).toEqual(["collection","executionRoot","layout","materialise","version"]);
 expect(taskRootModeV1({environment:{}})).toBe("live");
 expect(taskRootModeV1({taskClass:"readiness-probe",environment:{}})).toBe("view");
 expect(taskRootModeV1({taskClass:"interactive",environment:{}})).toBe("live");
 expect(taskRootModeV1({taskClass:"unknown",environment:{}})).toBe("live");
 expect(taskRootModeV1({taskClass:"readiness-probe",environment:{SEMIO_TASK_ROOT:"live"}}),"turned back to live without code").toBe("live");
 expect(taskRootModeV1({taskClass:"interactive",environment:{SEMIO_TASK_ROOT:"view"}})).toBe("view");
 expect(taskRootModeV1({taskClass:"readiness-probe",environment:{SEMIO_TASK_ROOT:"nonsense"}}),"an unknown value is no declaration").toBe("view");
 expect(policy.materialise.strategies).toEqual(["reflink","copy","hardlink"]);
 expect(policy.layout.maximumPathCharacters,"CLAUDE.md: no path beyond 256 characters").toBeLessThan(256);
});

test.skipIf(!hasOracleV1)("a view holds exactly the declared inputs: digests equal Python's hashlib, the tree equals `diff -r`, mtimes and links are kept",async()=>{
 const live=world("tv-live"),directory=world("tv-views"),paths=growTreeV1(live,120);
 symlinkSync("d0/e0/f0.rs",join(live,"alias.rs"));
 const when=new Date(1700000000000);for(const path of paths.slice(0,40))utimesSync(join(live,path),when,when);
 const inputs=await digestTaskViewInputsV1(live,[...paths,"alias.rs"]);
 const model=oracleV1({op:"digest-files",paths:paths.map(path=>join(live,path))}) as string[];
 inputs.filter(row=>row.link===undefined).forEach((row,index)=>expect(row.digest).toBe(model[index]));
 expect<unknown>(taskViewInputsDigestV1(inputs)).toBe(oracleV1({op:"inputs-digest",rows:inputs}));
 const view=ready(await openTaskViewV1({source:live,directory,hash:"cafe0001",inputs})).view;
 expect(view.receipt.files).toBe(121);
 expect(oracleV1({op:"digest-files",paths:paths.map(path=>join(view.root,path))})).toEqual(model);
 expect(readlinkSync(join(view.root,"alias.rs"))).toBe("d0/e0/f0.rs");
 for(const path of paths.slice(0,40))expect(lstatSync(join(view.root,path)).mtimeMs,"cargo compares mtimes: they are kept").toBe(lstatSync(join(live,path)).mtimeMs);
 const diff=Bun.spawnSync(["diff","-r",live,view.root],{stdout:"pipe",stderr:"pipe"});
 expect(diff.exitCode,diff.stdout.toString()).toBe(0);
 expect((await view.verify()).ok).toBe(true);
 await view.release();
});

test("churn law: every file of the closure is rewritten every 20 ms in the live tree while the view is read, the view keeps the start bytes",async()=>{
 const live=world("tv-live"),directory=world("tv-views"),paths=growTreeV1(live,60),start=snapshotV1(live);
 const inputs=await digestTaskViewInputsV1(live,paths),view=ready(await openTaskViewV1({source:live,directory,hash:"cafe0002",inputs})).view;
 let round=0,running=true;
 const writer=(async()=>{while(running){round++;for(const path of paths)writeFileSync(join(live,path),`edited ${round}\n`);await sleepV1(20);}})();
 const seen:Record<string,string>[]=[];
 for(let pass=0;pass<5;pass++){seen.push(snapshotV1(view.root));await sleepV1(40);}
 running=false;await writer;
 expect(round).toBeGreaterThan(5);
 for(const row of seen)expect(row,"a hermetic task cannot observe a change").toEqual(start);
 expect(snapshotV1(live)).not.toEqual(start);
 expect((await view.verify()).ok,"nothing to detect on a clone").toBe(true);
 await view.release();
});

test("sharing law: the same hash opens the same view without touching a file, concurrent opens give one view, another hash another one",async()=>{
 const live=world("tv-live"),directory=world("tv-views"),paths=growTreeV1(live,40),inputs=await digestTaskViewInputsV1(live,paths);
 let calls=0;
 const spy:TaskViewPortV1={name:"reflink",file:(from,to)=>{calls++;return TASK_VIEW_PORTS_V1.reflink.file(from,to);}};
 const first=ready(await openTaskViewV1({source:live,directory,hash:"cafe0003",inputs,ports:{reflink:spy}})).view;
 expect(first.receipt.reused).toBe(false);expect(calls).toBe(40);
 const second=ready(await openTaskViewV1({source:live,directory,hash:"cafe0003",inputs,ports:{reflink:spy}})).view;
 expect(second.receipt.reused).toBe(true);expect(second.root).toBe(first.root);expect(calls,"a rerun reuses the view").toBe(40);
 const racing=await Promise.all([1,2,3,4].map(()=>openTaskViewV1({source:live,directory,hash:"cafe0004",inputs})));
 const roots=new Set(racing.map(row=>ready(row).view.root));
 expect(roots.size).toBe(1);
 expect(readdirSync(directory).filter(name=>name.startsWith(".tmp-")),"no loser leaves a staging directory").toEqual([]);
 const other=ready(await openTaskViewV1({source:live,directory,hash:"cafe0005",inputs})).view;
 expect(other.root).not.toBe(first.root);
 expect(readdirSync(join(directory,".use")).filter(name=>name.startsWith("cafe0003")).length).toBe(2);
 await first.release();await second.release();
 expect(readdirSync(join(directory,".use")).filter(name=>name.startsWith("cafe0003"))).toEqual([]);
});

test("a view that no longer matches its manifest is made again, never reused",async()=>{
 const live=world("tv-live"),directory=world("tv-views"),paths=growTreeV1(live,10),inputs=await digestTaskViewInputsV1(live,paths);
 const first=ready(await openTaskViewV1({source:live,directory,hash:"cafe0006",inputs})).view;
 writeFileSync(join(first.root,paths[3]!),"tampered");
 expect((await first.verify()).ok).toBe(false);
 const again=ready(await openTaskViewV1({source:live,directory,hash:"cafe0006",inputs})).view;
 expect(again.receipt.reused).toBe(false);
 expect(readFileSync(join(again.root,paths[3]!),"utf8")).toBe(readFileSync(join(live,paths[3]!),"utf8"));
});

test("moved law: a live file that changed between hashing and pinning is reported, nothing is published, and a new hash pins the new state",async()=>{
 const live=world("tv-live"),directory=world("tv-views"),paths=growTreeV1(live,30),inputs=await digestTaskViewInputsV1(live,paths);
 writeFileSync(join(live,paths[7]!),"changed after the hash was taken");writeFileSync(join(live,paths[2]!),"also changed");
 const outcome=await openTaskViewV1({source:live,directory,hash:"cafe0007",inputs});
 expect(outcome.kind).toBe("moved");
 if(outcome.kind==="moved"){expect(outcome.paths).toEqual([paths[2]!,paths[7]!].sort());expect(outcome.en).toContain("cafe0007");expect(outcome.de).toContain("Aufgabe");}
 expect(existsSync(join(directory,"cafe0007"))).toBe(false);
 expect(readdirSync(directory).filter(name=>name.startsWith(".tmp-"))).toEqual([]);
 expect(readdirSync(join(directory,".use")),"a refused open leaves no lease").toEqual([]);
 const vanished=await openTaskViewV1({source:live,directory,hash:"cafe0008",inputs:[...inputs,{path:"gone.rs",digest:"0".repeat(64)}]});
 expect(vanished.kind).toBe("moved");
 const fresh=ready(await openTaskViewV1({source:live,directory,hash:"cafe0009",inputs:await digestTaskViewInputsV1(live,paths)})).view;
 expect(readFileSync(join(fresh.root,paths[7]!),"utf8")).toBe("changed after the hash was taken");
});

test("cost law: the work is proportional to the number of files, a clone reads no input bytes, progress is monotonic, nothing is left after the end",async()=>{
 const live=world("tv-live"),directory=world("tv-views");
 const calls:Record<number,number>={};
 for(const count of [50,200]){
  const paths=growTreeV1(live,count,count),inputs=await digestTaskViewInputsV1(live,paths);let made=0;const done:number[]=[];
  const spy:TaskViewPortV1={name:"reflink",file:(from,to)=>{made++;return TASK_VIEW_PORTS_V1.reflink.file(from,to);}};
  const view=ready(await openTaskViewV1({source:live,directory,hash:"cost"+count+"xxxx",inputs,ports:{reflink:spy},progress:row=>{if(row.phase==="pin")done.push(row.done);}})).view;
  calls[count]=made;
  expect(view.receipt.strategy).toBe("reflink");
  expect(made).toBe(count);
  expect(done).toEqual([...done].sort((a,b)=>a-b));
  expect(view.receipt.bytes).toBeGreaterThan(0);
 }
 expect(calls[200]!/calls[50]!).toBe(4);
});

test("cancellation law: an aborted opening rejects, leaves no staging directory and no lease, and the next opening succeeds",async()=>{
 const live=world("tv-live"),directory=world("tv-views"),paths=growTreeV1(live,600),inputs=await digestTaskViewInputsV1(live,paths),control=new AbortController();
 await expect(openTaskViewV1({source:live,directory,hash:"cafe0010",inputs,signal:control.signal,progress:row=>{if(row.done>=256)control.abort();}})).rejects.toThrow();
 expect(readdirSync(directory).filter(name=>name!==".use")).toEqual([]);
 expect(readdirSync(join(directory,".use"))).toEqual([]);
 ready(await openTaskViewV1({source:live,directory,hash:"cafe0010",inputs}));
});

test("path law: a root that would exceed the path bound is refused in both languages and the task stays on the live tree",async()=>{
 const live=world("tv-live"),directory=world("tv-views");
 const long="a/".repeat(110)+"f.rs";
 const outcome=await openTaskViewV1({source:live,directory,hash:"cafe0011",inputs:[{path:long,digest:"0".repeat(64)}]});
 expect(outcome.kind).toBe("refused");
 if(outcome.kind==="refused"){expect(outcome.code).toBe("path-too-long");expect(outcome.en).toContain("live tree");expect(outcome.de).toContain("lebenden Baum");}
 await expect(openTaskViewV1({source:live,directory,hash:"cafe0012",inputs:[{path:"../x",digest:"0".repeat(64)}]})).rejects.toThrow("Relative input path");
 await expect(openTaskViewV1({source:live,directory,hash:"x",inputs:[]})).rejects.toThrow("task hash");
});

test("rename-replace by the replica leaves a hardlinked pin intact, a view is never written by the replica",()=>{
 const daemon=readFileSync(join(import.meta.dir,"..","..","..","..","🤝️shared-worktree","🔁️replica","🟦️.ts"),"utf8");
 expect(daemon).toContain("rename(temporary,target)");
 const live=world("tv-live");writeFileSync(join(live,"a"),"old");
 const pinned=join(live,"pin");require("node:fs").linkSync(join(live,"a"),pinned);
 writeFileSync(join(live,"a.new"),"new");renameSync(join(live,"a.new"),join(live,"a"));
 expect(readFileSync(pinned,"utf8")).toBe("old");
});

test("the root of a view is its real path, so a remapping and the unit cache see one spelling",async()=>{
 const live=world("tv-live"),directory=world("tv-views"),paths=growTreeV1(live,3),inputs=await digestTaskViewInputsV1(live,paths);
 const view=ready(await openTaskViewV1({source:live,directory,hash:"cafe0013",inputs})).view;
 expect(view.root).toBe(realpathSync(view.root));
 expect(view.root.startsWith(realpathSync(directory))).toBe(true);
});

test("freshness law: a rewrite of equal bytes does not move the time a view gives the file, new bytes get a time not before the opening",async()=>{
 const live=world("tv-live"),directory=world("tv-views"),paths=growTreeV1(live,12),first=await digestTaskViewInputsV1(live,paths);
 const one=ready(await openTaskViewV1({source:live,directory,hash:"cafe0014",inputs:first})).view,before=(path:string)=>lstatSync(join(one.root,path)).mtimeMs;
 await sleepV1(30);
 for(const path of paths)writeFileSync(join(live,path),readFileSync(join(live,path)));
 const changed=Date.now();writeFileSync(join(live,paths[5]!),"new bytes\n");
 const two=ready(await openTaskViewV1({source:live,directory,hash:"cafe0015",inputs:await digestTaskViewInputsV1(live,paths)})).view;
 for(const path of paths.filter(path=>path!==paths[5]))expect(Math.abs(lstatSync(join(two.root,path)).mtimeMs-before(path)),path).toBeLessThan(1);
 expect(lstatSync(join(two.root,paths[5]!)).mtimeMs).toBeGreaterThanOrEqual(changed);
 expect(lstatSync(join(live,paths[0]!)).mtimeMs).toBeGreaterThan(before(paths[0]!));
});

test("cargo tests build in a scratch outside every .cargo/config.toml: the suite scratch is the sandbox and the world follows the artifact directory",()=>{
 const rows:{id:string;scratch?:string}[]=JSON.parse(readFileSync(join(import.meta.dirname,"../../🗂️suites/🔣️.json"),"utf8")).suites,row=rows.find(entry=>entry.id==="build-task-view");
 expect(row?.scratch).toBe("sandbox");
 const saved=process.env.SEMIO_TEST_ARTIFACT_DIR,artifacts=world("tv-artifacts");
 try{
  process.env.SEMIO_TEST_ARTIFACT_DIR=artifacts;
  const root=world("tv-cargo");
  expect(root.startsWith(artifacts)).toBe(true);
 }finally{if(saved===undefined)delete process.env.SEMIO_TEST_ARTIFACT_DIR;else process.env.SEMIO_TEST_ARTIFACT_DIR=saved;}
});
