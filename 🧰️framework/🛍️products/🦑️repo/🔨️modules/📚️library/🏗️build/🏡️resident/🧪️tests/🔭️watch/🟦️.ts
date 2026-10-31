import hotTree from "./📥️hot.json";
import restartProtocol from "./📥️restart.json";
import startupCorpus from "./📥️startup.json";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {afterAll,describe,expect,test} from "bun:test";
import {EventEmitter} from "node:events";
import {PassThrough} from "node:stream";
import {appendFileSync,mkdirSync,renameSync,rmSync,writeFileSync,lstatSync,symlinkSync} from "node:fs";
import {spawn,spawnSync} from "node:child_process";
import {createWatchSupervisorV1} from "../../🔭️watch/🟨️.mjs";
import {POLICY_V1,validatorV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {createFilterV1} from "../../🗃️state/🟨️.mjs";
import {cleanup,haveBinary,residentWatcherV1,sleep,snapshot,tempRoot,waitFor,writeTree} from "../🟦️.ts";

const WATCHER=await residentWatcherV1(),native=haveBinary(WATCHER);
const roots:string[]=[];
afterAll(()=>roots.forEach(cleanup));
const filter=createFilterV1({rooted:[".git"],names:["node_modules"]});
const policy={...POLICY_V1,watcherRestartBackoffMilliseconds:[10,10,10],fenceTimeoutMilliseconds:600};

function make(root:string,extra:Record<string,unknown>={}){
 const seen:(readonly [string,number,string])[]=[],losses:string[]=[],trusts:string[]=[];
 const supervisor=createWatchSupervisorV1({root,policy,filter,rooted:[".git"],names:["node_modules"],binary:WATCHER,backend:"native",fenceDirectory:".semio-fence",instance:"t",
  onBatch:async rows=>{seen.push(...rows);},onLost:async(scope,reason)=>{losses.push(reason+":"+scope);},onTrust:t=>trusts.push(t.trust),...extra});
 return{supervisor,seen,losses,trusts};
}

describe.skipIf(!native)("native watcher",()=>{
 test("reports create, modify, remove, rename and directory creation; never excluded paths",async()=>{
  const root=tempRoot("watch-basic-");roots.push(root);writeTree(root,{"src/a.ts":"a","src/old.ts":"o"});
  const{supervisor,seen}=make(root);const state=await supervisor.start();
  expect(state.trust).toBe("exact");
  writeFileSync(root+"/src/new.ts","n");appendFileSync(root+"/src/a.ts","x");rmSync(root+"/src/old.ts");renameSync(root+"/src/new.ts",root+"/src/moved.ts");for(const directory of hotTree.directories)mkdirSync(root+"/"+directory,{recursive:true});writeTree(root,hotTree.files);
  mkdirSync(root+"/node_modules/x",{recursive:true});writeFileSync(root+"/node_modules/x/y.js","y");mkdirSync(root+"/.git",{recursive:true});writeFileSync(root+"/.git/HEAD","h");
  await supervisor.fence();await supervisor.drained();
  const paths=new Set(seen.map(r=>r[2]));
  expect(new Ajv({strict:true}).validate({type:"object",additionalProperties:false,required:["directories","files","expected"],properties:{directories:{type:"array",items:{type:"string"}},files:{type:"object",additionalProperties:{type:"string"}},expected:{type:"array",items:{type:"string"}}}},hotTree)).toBe(true);
  for(const expected of["src/a.ts","src/old.ts","src/moved.ts",...hotTree.expected])expect(paths.has(expected),expected).toBe(true);
  expect(seen.find(r=>r[2]==="lib")![1]).toBe(1);
  expect([...paths].some(p=>p.includes("node_modules")||p.startsWith(".git"))).toBe(false);
  await supervisor.stop();
 });

 test("a fence resolves only after every earlier write has been delivered",async()=>{
  const root=tempRoot("watch-fence-");roots.push(root);mkdirSync(root+"/d");
  const{supervisor,seen}=make(root);await supervisor.start();
  for(let round=0;round<25;round++){
   const names:string[]=[];
   for(let i=0;i<40;i++){const p="d/r"+round+"_"+i;names.push(p);writeFileSync(root+"/"+p,"x");}
   const result=await supervisor.fence();await supervisor.drained();
   expect(result.ok).toBe(true);
   const got=new Set(seen.map(r=>r[2]));
   expect(names.filter(p=>!got.has(p))).toEqual([]);
  }
  expect(supervisor.stats.fenceMisses).toBe(0);
  await supervisor.stop();
 },30000);

 test("concurrent fences share work but each covers writes made before its own call",async()=>{
  const root=tempRoot("watch-fence-share-");roots.push(root);
  const{supervisor,seen}=make(root);await supervisor.start();
  const results=await Promise.all(Array.from({length:12},async(_,i)=>{
   writeFileSync(root+"/c"+i,"x");const p="c"+i;
   const r=await supervisor.fence();await supervisor.drained();
   return{ok:r.ok,delivered:seen.some(row=>row[2]===p)};
  }));
  expect(results.every(r=>r.ok&&r.delivered)).toBe(true);
  expect(supervisor.stats.fences).toBeLessThan(12+2);
  await supervisor.stop();
 });

 test("closing stdin ends the watcher; killing the parent cannot leave an orphan",async()=>{
  const root=tempRoot("watch-orphan-");roots.push(root);
  const child=spawn(WATCHER,["watch","--root",root],{stdio:["pipe","pipe","ignore"]});
  await new Promise(resolve=>child.stdout.once("data",resolve));
  const exited=new Promise<number|null>(resolve=>child.once("exit",resolve));
  child.stdin.end();
  expect(await Promise.race([exited,sleep(2000).then(()=>"timeout")])).toBe(0);
  const script=`import {spawn} from "node:child_process";const c=spawn(${JSON.stringify(WATCHER)},["watch","--root",${JSON.stringify(root)}],{stdio:["pipe","pipe","ignore"]});c.stdout.once("data",()=>{process.stdout.write(String(c.pid)+"\\n",()=>setTimeout(()=>process.kill(process.pid,"SIGKILL"),50));});`;
  const out=await new Promise<{stdout:string;stderr:string}>((resolve,reject)=>{const p=spawn(process.execPath,["-e",script],{env:{...process.env,FORCE_COLOR:"1"},stdio:["ignore","pipe","pipe"]});let stdout="",stderr="";p.stdout.on("data",c=>stdout+=c);p.stderr.on("data",c=>stderr+=c);p.on("error",reject);p.on("close",()=>resolve({stdout:stdout.trim(),stderr}));});
  const pid=Number(out.stdout);expect(pid,out.stderr).toBeGreaterThan(1);
  expect(await waitFor(()=>{try{process.kill(pid,0);return false;}catch{return true;}},3000)).toBe(true);
 },15000);

 test("restart replays persistent history or explicitly requires independent reconciliation",async()=>{
  const root=tempRoot("watch-replay-");roots.push(root);mkdirSync(root+"/src");
  const first=make(root);await first.supervisor.start();writeFileSync(root+"/src/a","1");await first.supervisor.fence();await first.supervisor.drained();
  const saved={cursor:first.supervisor.state().cursor,uuid:first.supervisor.state().uuid};await first.supervisor.stop();
  for(let i=0;i<30;i++)writeFileSync(root+"/src/off"+i,"x");
  await sleep(300);
  const second=make(root,{restore:saved});const state=await second.supervisor.start();
  await second.supervisor.drained();const db=new Database(":memory:");try{
   expect(new Ajv({strict:true}).validate({type:"array",items:{type:"object",additionalProperties:false,required:["backend","history"],properties:{backend:{enum:["inotify","rdcw","fsevents"]},history:{type:"boolean"}}}},restartProtocol)).toBe(true);
   const row=restartProtocol.find(row=>row.backend===state.backend);expect(row).toBeDefined();const oracle=db.query("SELECT ?1 AND length(?2)>0 AND length(?3)>0 replay").get(row!.history?1:0,saved.cursor,saved.uuid) as {replay:number};expect(state.replay).toBe(Boolean(oracle.replay));
   if(state.replay){expect(await waitFor(()=>second.seen.filter(r=>r[2].startsWith("src/off")).length>=30,3000)).toBe(true);expect(second.losses).toEqual([]);}
   else{expect(second.losses).toEqual(["restart:"]);const scan=spawnSync(WATCHER,["scan","--root",root,"--exclude",".git","--exclude-name","node_modules"],{encoding:"utf8"});expect(scan.status).toBe(0);const actual=JSON.parse(scan.stdout).members.map((row:[string,number])=>row[0]).filter((path:string)=>path.startsWith("src/off")).sort(),expected=[...snapshot(root,path=>filter.excluded(path)).files.keys()].filter(path=>path.startsWith("src/off")).sort();expect(actual).toEqual(expected);expect(expected).toHaveLength(30);}
  }finally{db.close();}
  await second.supervisor.stop();
 });

 test("scan equals a naive walk: same files, modes, directory stamps",()=>{
  const root=tempRoot("watch-scan-");roots.push(root);
  writeTree(root,{"a/b/c.ts":"c","a/d.ts":"d","e.txt":"e","node_modules/x/y.js":"y",".git/HEAD":"h","ü/ß.ts":"u"});
  const out=spawnSync(WATCHER,["scan","--root",root,"--exclude",".git","--exclude-name","node_modules","--threads","3"],{encoding:"utf8"});
  const frame=JSON.parse(out.stdout);
  expect(validatorV1("WatcherFrame")).toBeDefined();
  const naive=snapshot(root,rel=>filter.excluded(rel));
  expect(frame.members.map((m:any)=>m[0])).toEqual([...naive.files.keys()].sort());
  for(const[path,mode]of frame.members)if(mode!==0)expect(naive.files.get(path)).toBe(mode);
  expect(frame.directories.map((d:any)=>d[0])).toEqual([...naive.directories.keys()].sort());
  for(const[path,mtime,ino]of frame.directories){const n=naive.directories.get(path)!;expect(mtime).toBe(n[0]);if(ino!==0)expect(ino).toBe(n[1]);}
  if(process.platform==="win32"){expect(frame.directories.every((d:any)=>d[2]===0)).toBe(true);expect(frame.members.every((m:any)=>m[1]===0)).toBe(true);}
 });

 test("bad arguments produce one error frame and exit 2; version is a valid frame",()=>{
  const bad=spawnSync(WATCHER,["watch"],{encoding:"utf8"});
  expect(bad.status).toBe(2);expect(validatorV1("WatcherFrame")(JSON.parse(bad.stdout.trim().split("\n")[0]!))).toEqual([]);
  expect(spawnSync(WATCHER,["frobnicate"],{encoding:"utf8"}).status).toBe(2);
  expect(validatorV1("WatcherFrame")(JSON.parse(spawnSync(WATCHER,["version"],{encoding:"utf8"}).stdout))).toEqual([]);
  expect(spawnSync(WATCHER,["watch","--root","/definitely/not/here"],{encoding:"utf8"}).status).toBe(2);
 });

 test("every frame the real watcher prints validates against the schema",async()=>{
  const root=tempRoot("watch-schema-");roots.push(root);
  const child=spawn(WATCHER,["watch","--root",root,"--latency-ms","5"],{stdio:["pipe","pipe","ignore"]});
  let text="";child.stdout.on("data",c=>text+=c);
  await sleep(300);for(let i=0;i<20;i++){writeFileSync(root+"/f"+i,"x");if(i%5===0)mkdirSync(root+"/d"+i);}
  await sleep(500);child.stdin.end();await new Promise(r=>child.once("exit",r));
  const validate=validatorV1("WatcherFrame");const lines=text.trim().split("\n");
  expect(lines.length).toBeGreaterThan(2);
  for(const line of lines)expect(validate(JSON.parse(line))).toEqual([]);
 });
});

/** 🎭 A scripted stand-in for the watcher process. */
function fakeChild(script:(child:any)=>void){
 const child:any=new EventEmitter();child.stdout=new PassThrough();child.stdin={end(){}};child.kill=()=>child.emit("exit",null);
 queueMicrotask(()=>script(child));return child;
}
const frame=(child:any,value:unknown)=>child.stdout.write(JSON.stringify(value)+"\n");
const ready={v:1,t:"ready",backend:"fsevents",root:"/x",cursor:"1",uuid:"U",replay:false,latencyMs:5,native:true};

describe("funded startup lifecycle model",()=>{
 for(const row of startupCorpus.cases)test(row.name,async()=>{
  const db=new Database(":memory:"),root=tempRoot("watch-start-");roots.push(root);
  const abort=new AbortController();if(row.abort===0)abort.abort(Error("owner cancelled"));
  const born=performance.now();let launched=0,readyTimer:ReturnType<typeof setTimeout>|undefined;
  const remaining=()=>row.remaining===null?null:Math.max(0,row.remaining-(performance.now()-born));
  const expected=db.query("select (?1 is not null and ?1>0 and ?2<?1 and (?3 is null or ?3>?2)) accepted").get(row.remaining,row.ready,row.abort) as {accepted:number};
  expect(Boolean(expected.accepted)).toBe(row.accepted);
  const{supervisor}=make(root,{policy:{...policy,fenceTimeoutMilliseconds:startupCorpus.fenceMilliseconds},startupControl:{signal:abort.signal,remainingMilliseconds:remaining},binary:"scripted-model",spawnProcess:()=>{
   launched++;const child=fakeChild(child=>{readyTimer=setTimeout(()=>frame(child,ready),row.ready);});
   child.stdin.end=()=>{clearTimeout(readyTimer);queueMicrotask(()=>child.emit("exit",0));};return child;
  }});
  const cancel=row.abort!==null&&row.abort>0?setTimeout(()=>abort.abort(Error("owner cancelled")),row.abort):undefined;
  try{
   const state=await supervisor.start();expect(state.backend==="fsevents").toBe(row.accepted);expect(launched>0).toBe(row.launched);
   expect(state.trust).not.toBe("exact");
   if(row.accepted)expect(supervisor.stats.fenceMisses).toBe(1);
  }finally{clearTimeout(cancel);await supervisor.stop();db.close();}
 });
 test("retired timers and frames cannot settle a later launch",async()=>{
  const root=tempRoot("watch-start-generation-");roots.push(root);const timing=startupCorpus.lifecycle,timers:ReturnType<typeof setTimeout>[]=[];let launches=0;
  const{supervisor}=make(root,{policy:{...policy,fenceTimeoutMilliseconds:timing.fence},binary:"scripted-model",spawnProcess:()=>{
   const generation=++launches,child=fakeChild(child=>{
    timers.push(setTimeout(()=>frame(child,{...ready,cursor:generation===1?"first":"second"}),generation===1?timing.firstReady:timing.secondReady));
    if(generation===1){timers.push(setTimeout(()=>child.emit("exit",1),timing.firstExit));timers.push(setTimeout(()=>frame(child,{...ready,cursor:"stale"}),timing.staleReady));}
   });child.stdin.end=()=>queueMicrotask(()=>child.emit("exit",0));return child;
  }});
  const born=performance.now();try{
   await supervisor.start();await sleep(Math.max(0,timing.checkpoint-(performance.now()-born)));expect(supervisor.state().cursor).toBe("first");
   await sleep(Math.max(0,timing.settled-(performance.now()-born)));expect(launches).toBe(2);expect(supervisor.state().cursor).toBe("second");
  }finally{timers.forEach(clearTimeout);await supervisor.stop();}
 });
});

describe("supervisor against a scripted watcher",()=>{
 test("a watch limit makes the trust permanent verify: unwatched directories can never be vouched for",async()=>{
  const root=tempRoot("watch-limit-");roots.push(root);
  const{supervisor,losses,trusts}=make(root,{binary:"fake",spawnProcess:()=>fakeChild(child=>{frame(child,ready);frame(child,{v:1,t:"lost",at:1,reason:"watch-limit",scope:"deep/dir"});})});
  await supervisor.start();await sleep(50);
  expect(losses).toContain("watch-limit:deep/dir");expect(trusts).toContain("verify");expect(supervisor.state().trust).toBe("verify");
  await supervisor.stop();
 },10000);

 test("lost frames, malformed batches and path escapes reach onLost, never onBatch",async()=>{
  const root=tempRoot("watch-fake-");roots.push(root);
  const{supervisor,seen,losses}=make(root,{binary:"fake",spawnProcess:()=>fakeChild(child=>{
   frame(child,ready);
   frame(child,{v:1,t:"lost",at:1,reason:"overflow",scope:"src"});
   frame(child,{v:1,t:"batch",n:1,at:1,cursor:"2",c:[["m",0,"../etc/passwd"]]});
   frame(child,{v:1,t:"batch",n:2,at:1,cursor:"3",c:[["m",0,"/abs"]]});
   frame(child,{v:1,t:"batch",n:3,at:1,cursor:"4",c:[["m",0,"ok/file"]]});
  })});
  const started=await supervisor.start();void started;
  await sleep(100);await supervisor.drained();
  expect(losses).toContain("overflow:src");expect(losses.filter(l=>l.startsWith("read-error")).length).toBe(2);
  expect(seen.map(r=>r[2])).toEqual(["ok/file"]);
  expect(supervisor.state().cursor).toBe("4");
  await supervisor.stop();
 });

 test("without any fence event the canary degrades trust instead of lying",async()=>{
  const root=tempRoot("watch-canary-");roots.push(root);
  const{supervisor}=make(root,{binary:"fake",spawnProcess:()=>fakeChild(child=>frame(child,ready))});
  const state=await supervisor.start();
  expect(state.trust).toBe("verify");expect(supervisor.stats.fenceMisses).toBe(1);
  await supervisor.stop();
 },10000);

 test("a watcher that keeps dying is restarted with backoff, then replaced by the portable backend",async()=>{
  const root=tempRoot("watch-restart-");roots.push(root);let spawned=0;
  const{supervisor,losses,trusts}=make(root,{binary:"fake",spawnProcess:()=>{spawned++;return fakeChild(child=>{if(spawned===1){frame(child,ready);setTimeout(()=>child.emit("exit",1),20);}else child.emit("exit",1);});}});
  await supervisor.start().catch(()=>{});
  await waitFor(()=>supervisor.state().backend==="portable",8000);
  expect(spawned).toBeGreaterThanOrEqual(policy.watcherRestartsBeforePortable);
  expect(supervisor.state().backend).toBe("portable");expect(trusts).toContain("verify");expect(losses.some(l=>l.startsWith("watcher-exit"))).toBe(true);
  await supervisor.stop();
 },15000);

 test("poll and none backends never claim exact trust and have no fence",async()=>{
  const root=tempRoot("watch-none-");roots.push(root);
  for(const backend of["poll","none"]){
   const{supervisor}=make(root,{backend});const state=await supervisor.start();
   expect(state.trust).not.toBe("exact");expect((await supervisor.fence()).ok).toBe(false);await supervisor.stop();
  }
 });
});

describe("strict workspace watch custody",()=>{
 test("private physical fence ancestry refuses symlinks before any native launch",async()=>{
  const root=tempRoot("watch-fence-link-"),outside=tempRoot("watch-outside-");roots.push(root,outside);symlinkSync(outside,root+"/private",process.platform==="win32"?"junction":"dir");let launches=0;
  const{supervisor}=make(root,{binary:"fake",privateFence:true,failureMode:"terminal",fenceDirectory:"private/f",spawnProcess:()=>{launches++;throw Error("Unowned native launch");}});
  try{await expect(supervisor.start()).rejects.toThrow("No-follow");expect(launches).toBe(0);}finally{await supervisor.stop();}
 });

 test("a missing physical canary refuses startup without a portable backend",async()=>{
  const root=tempRoot("watch-strict-");roots.push(root);let launches=0;
  const{supervisor}=make(root,{binary:"fake",privateFence:true,failureMode:"terminal",spawnProcess:()=>{launches++;return fakeChild(child=>frame(child,ready));}});
  try{await expect(supervisor.start()).rejects.toThrow("physical canary");expect(supervisor.state().backend).toBe("fsevents");expect(supervisor.state().trust).toBe("verify");expect(launches).toBe(1);}finally{await supervisor.stop();}
 });
 test("a native coverage loss permanently refuses every later fence",async()=>{
  const root=tempRoot("watch-strict-loss-");roots.push(root);
  const{supervisor,losses}=make(root,{binary:"fake",privateFence:true,failureMode:"terminal",spawnProcess:()=>fakeChild(child=>{frame(child,ready);frame(child,{v:1,t:"lost",at:1,reason:"registration-error",scope:"source"});})});
  try{await expect(supervisor.start()).rejects.toThrow("coverage");await expect(supervisor.fence()).rejects.toThrow("coverage");expect(losses).toEqual(["registration-error:source"]);expect(supervisor.state().trust).toBe("verify");}finally{await supervisor.stop();}
 });
});
