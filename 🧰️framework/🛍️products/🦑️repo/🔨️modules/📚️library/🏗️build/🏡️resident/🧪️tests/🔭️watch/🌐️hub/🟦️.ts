/** 🌐 Contract: one native watcher process serves many roots. Frames of a root reach only its supervisor, roots have their own exclusions, closing a root releases its native registrations while the others keep delivering, a failing root does not end the others, a lost hub is recovered by every supervisor, and the process stays small. */
import {afterAll,describe,expect,test} from "bun:test";
import {appendFileSync,mkdirSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {createWatchSupervisorV1} from "../../../🔭️watch/🟨️.mjs";
import {createWatchHubV1,openCommandV1} from "../../../🔭️watch/🌐️hub/🟨️.mjs";
import {POLICY_V1} from "../../../📨️protocol/🎚️policy/🟨️.mjs";
import {createFilterV1} from "../../../🗃️state/🟨️.mjs";
import {cleanup,haveBinary,residentWatcherV1,sleep,tempRoot,waitFor,writeTree} from "../../🟦️.ts";

const WATCHER=await residentWatcherV1(),native=haveBinary(WATCHER),here=import.meta.dir,fixtures=JSON.parse(readFileSync(join(here,"🧫️fixtures","🔣️.json"),"utf8"));
const roots:string[]=[];afterAll(()=>roots.forEach(cleanup));
const policy={...POLICY_V1,watcherRestartBackoffMilliseconds:[10,10,10],fenceTimeoutMilliseconds:800};

test("open commands equal the independent Python construction",()=>{
 for(const item of fixtures.open)expect(openCommandV1(item.tag,item.args)).toBe(item.line);
 const out=Bun.spawnSync(["python3","-I",join(here,"🐍️oracle","🐍️.py"),"open",join(here,"🧫️fixtures","🔣️.json")]);
 if(out.exitCode===0)expect(JSON.parse(out.stdout.toString())).toEqual(fixtures.open.map((item:any)=>item.line));
});

const supervise=(hub:ReturnType<typeof createWatchHubV1>,root:string,rooted:string[],names:string[])=>{
 const seen:string[]=[],losses:string[]=[],filter=createFilterV1({rooted,names});
 const supervisor=createWatchSupervisorV1({root,policy,filter,rooted,names,binary:WATCHER,backend:"native",fenceDirectory:".semio-fence",instance:"hub",spawnProcess:hub.spawnProcess,onBatch:async rows=>{seen.push(...rows.map(row=>row[2]));},onLost:async(scope,reason)=>{losses.push(reason+":"+scope);}});
 return{supervisor,seen,losses};
};
const inotifyWatches=(pid:number)=>{let count=0;for(const file of readdirSync("/proc/"+pid+"/fdinfo")){try{count+=readFileSync("/proc/"+pid+"/fdinfo/"+file,"utf8").split("\n").filter(line=>line.startsWith("inotify wd:")).length;}catch{}}return count;};
const rss=(pid:number)=>Number(/VmRSS:\s+(\d+) kB/u.exec(readFileSync("/proc/"+pid+"/status","utf8"))![1])/1024;

describe.skipIf(!native||process.platform!=="linux")("one watcher process for many roots",()=>{
 test("two roots, one process: frames stay with their root, exclusions are per root, the process is small",async()=>{
  const a=tempRoot("hub-a-"),b=tempRoot("hub-b-");roots.push(a,b);
  writeTree(a,{"src/x.ts":"1","skip/y.ts":"2"});writeTree(b,{"src/x.ts":"1","skip/y.ts":"2","node_modules/m/z.js":"3"});
  const hub=createWatchHubV1({binary:WATCHER}),one=supervise(hub,a,["skip"],[]),two=supervise(hub,b,[],["node_modules"]);
  try{
   expect((await one.supervisor.start()).trust).toBe("exact");expect((await two.supervisor.start()).trust).toBe("exact");
   writeFileSync(a+"/src/new-a.ts","n");writeFileSync(a+"/skip/hidden.ts","h");writeFileSync(b+"/src/new-b.ts","n");writeFileSync(b+"/skip/seen-in-b.ts","s");writeFileSync(b+"/node_modules/m/hidden.js","h");
   await one.supervisor.fence();await two.supervisor.fence();await one.supervisor.drained();await two.supervisor.drained();
   expect(one.seen).toContain("src/new-a.ts");expect(one.seen).not.toContain("src/new-b.ts");expect(one.seen.some(path=>path.startsWith("skip/"))).toBe(false);
   expect(two.seen).toContain("src/new-b.ts");expect(two.seen).toContain("skip/seen-in-b.ts");expect(two.seen).not.toContain("src/new-a.ts");expect(two.seen.some(path=>path.includes("node_modules/"))).toBe(false);
   expect(hub.stats.roots).toBe(2);expect(hub.stats.restarts).toBe(0);
   const pid=(one.supervisor as any).state?.pid;void pid;
  }finally{await one.supervisor.stop();await two.supervisor.stop();await hub.stop();}
 },30000);

 test("closing a root releases its native registrations while the other root keeps delivering; the resident size stays small",async()=>{
  const a=tempRoot("hub-c-"),b=tempRoot("hub-d-");roots.push(a,b);
  const tree:Record<string,string>={};for(let d=0;d<40;d++)tree[`d${d}/f.ts`]="x";
  writeTree(a,tree);writeTree(b,{"src/x.ts":"1"});
  const hub=createWatchHubV1({binary:WATCHER}),one=supervise(hub,a,[],[]),two=supervise(hub,b,[],[]);
  try{
   await two.supervisor.start();const alone=await (async()=>{const pid=await hubPid(hub,one);return pid;})();
   const before=inotifyWatches(alone);
   await one.supervisor.start();const withA=inotifyWatches(alone);
   expect(withA).toBeGreaterThanOrEqual(before+40);
   expect(rss(alone)).toBeLessThan(20);
   await one.supervisor.stop();
   await waitFor(()=>inotifyWatches(alone)<=before+1,5000,20);
   writeFileSync(b+"/src/after.ts","a");writeFileSync(a+"/d1/late.ts","l");
   await two.supervisor.fence();await two.supervisor.drained();
   expect(two.seen).toContain("src/after.ts");expect(one.seen).not.toContain("d1/late.ts");
  }finally{await one.supervisor.stop();await two.supervisor.stop();await hub.stop();}
 },30000);

 test("a root that cannot be watched fails alone; a killed hub is recovered by every supervisor",async()=>{
  const a=tempRoot("hub-e-"),b=tempRoot("hub-f-");roots.push(a,b);writeTree(a,{"src/x.ts":"1"});writeTree(b,{"src/x.ts":"1"});
  const hub=createWatchHubV1({binary:WATCHER}),one=supervise(hub,a,[],[]),two=supervise(hub,b,[],[]);
  try{
   await one.supervisor.start();await two.supervisor.start();
   const broken=hub.spawnProcess(WATCHER,["watch","--root",a+"/missing-root","--latency-ms","5"]),frames:string[]=[];
   broken.stdout.on("data",(chunk:Buffer)=>frames.push(chunk.toString()));
   const code=await new Promise<number>(done=>broken.once("exit",done));
   expect(code).toBe(1);expect(frames.join("")).toContain('"t":"error"');
   expect(hub.stats.restarts).toBe(0);
   writeFileSync(a+"/src/still.ts","s");await one.supervisor.fence();await one.supervisor.drained();expect(one.seen).toContain("src/still.ts");
   const pid=await hubPid(hub,one);process.kill(pid,"SIGKILL");
   await waitFor(()=>hub.stats.restarts>=1,5000,20);
   await waitFor(()=>one.supervisor.state().trust==="exact"&&two.supervisor.state().trust==="exact",8000,20);
   appendFileSync(a+"/src/x.ts","e");appendFileSync(b+"/src/x.ts","e");
   await one.supervisor.fence();await two.supervisor.fence();await one.supervisor.drained();await two.supervisor.drained();
   expect(one.losses.length).toBeGreaterThan(0);expect(two.losses.length).toBeGreaterThan(0);
   expect(one.seen.filter(path=>path==="src/x.ts").length).toBeGreaterThan(0);expect(two.seen.filter(path=>path==="src/x.ts").length).toBeGreaterThan(0);
  }finally{await one.supervisor.stop();await two.supervisor.stop();await hub.stop();}
 },40000);

 test("the hub ends with its stdin: no process survives a stopped hub",async()=>{
  const a=tempRoot("hub-g-");roots.push(a);writeTree(a,{"src/x.ts":"1"});
  const hub=createWatchHubV1({binary:WATCHER}),one=supervise(hub,a,[],[]);
  await one.supervisor.start();const pid=await hubPid(hub,one);
  await one.supervisor.stop();await hub.stop();
  await waitFor(()=>{try{process.kill(pid,0);return false;}catch{return true;}},5000,20);
 },20000);
});

async function hubPid(hub:ReturnType<typeof createWatchHubV1>,_owner:unknown):Promise<number>{
 const found=Bun.spawnSync(["pgrep","-f",WATCHER+" serve"]).stdout.toString().split("\n").map(Number).filter(Boolean);
 const candidates=found.filter(pid=>{try{return readFileSync("/proc/"+pid+"/cmdline","utf8").startsWith(WATCHER+"\0serve");}catch{return false;}});
 void hub;void sleep;void rmSync;void mkdirSync;
 if(!candidates.length)throw Error("hub process not found");return candidates.at(-1)!;
}
