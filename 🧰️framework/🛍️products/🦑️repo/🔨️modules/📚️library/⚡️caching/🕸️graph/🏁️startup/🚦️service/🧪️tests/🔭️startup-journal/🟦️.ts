/** 🔭️ Contract: the Nx daemon's own workspace startup (`openNxDaemonWorkspaceWatcherV1`, the body of the service's `workspace()`) derives the closed Source policy and the pinned watcher binary of the repository, starts the host change journal on demand and delivers the daemon's sealing events from it, with no watcher process of its own. */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {readChangeStateV1} from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import {liveChildPidsV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";
import {changeJournalDirectoryV1} from "../../../../../../🏗️build/🏡️resident/📜️journal/🟦️.ts";
import {repositoryCachePartitionRootV1} from "../../../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import workspaceWatchPin from "../../../../../../🏗️build/🏡️resident/🔭️watch/🦀️rust/🔣️.json" with {type:"json"};
import {openNxDaemonWorkspaceWatcherV1} from "../../🟦️.ts";

const repositoryRoot=(()=>{for(let at=import.meta.dir;;at=dirname(at)){if(existsSync(join(at,"bunfig.toml"))&&existsSync(join(at,"nx.json")))return at;if(dirname(at)===at)throw Error("Repository root not found");}})();
const binary=join(repositoryCachePartitionRootV1(repositoryRoot,"tools"),"repo-watch",workspaceWatchPin.sources[0]!.sha256.slice(0,16),process.platform==="win32"?"semio-repo-watch.exe":"semio-repo-watch");
const sandbox=realpathSync(mkdtempSync(join(tmpdir(),"nx-startup-")));
const environment={...process.env,XDG_RUNTIME_DIR:sandbox,SEMIO_REPO_CHANGES:"on"} as Record<string,string>;
const sleep=(milliseconds:number)=>new Promise(resolve=>setTimeout(resolve,milliseconds));
afterAll(async()=>{try{process.kill(readChangeStateV1(changeJournalDirectoryV1(join(sandbox,"w"),environment))!.pid,"SIGTERM");}catch{}await sleep(500);rmSync(sandbox,{recursive:true,force:true});});

test.skipIf(!existsSync(binary))("the daemon workspace startup delivers sealing events from the host journal and refuses nothing the Source policy observes",async()=>{
 const workspaceRoot=join(sandbox,"w");mkdirSync(join(workspaceRoot,"src"),{recursive:true});writeFileSync(join(workspaceRoot,"src","a.ts"),"a");
 const abort=new AbortController(),control={signal:abort.signal,remainingMilliseconds:()=>60000,publish:async()=>{},yieldContinuation:async()=>{}};
 const{watcher,policy}=await openNxDaemonWorkspaceWatcherV1({workspaceRoot,repositoryRoot,control:control as never,signal:abort.signal,startupControl:control as never,environment});
 try{
  expect(policy.roots).toContain(".🧬semio");expect(policy.names).toContain("node_modules");
  const state=readChangeStateV1(changeJournalDirectoryV1(workspaceRoot,environment))!;
  expect(state).toMatchObject({ready:true,trust:"exact"});expect(liveChildPidsV1(state.pid)).toHaveLength(1);
  const received:string[]=[];watcher.watch((error,events)=>{if(error)throw error;received.push(...events.map(event=>event.type+" "+event.path));});
  writeFileSync(join(workspaceRoot,"src","b.ts"),"b");writeFileSync(join(workspaceRoot,"src","a.ts"),"changed");mkdirSync(join(workspaceRoot,"node_modules","x"),{recursive:true});writeFileSync(join(workspaceRoot,"node_modules","x","y.js"),"y");
  const flushed=[...await watcher.forceFlushPending(),...received.map(row=>({type:row.split(" ")[0],path:row.split(" ")[1]!}))].map(event=>event.type+" "+event.path);
  expect(flushed).toContain("create src/b.ts");expect(flushed.some(row=>row.endsWith("src/a.ts"))).toBe(true);expect(flushed.some(row=>row.includes("node_modules"))).toBe(false);
 }finally{await watcher.stop();}
 await expect(watcher.forceFlushPending()).resolves.toBeDefined();
},60000);

test("a startup whose control is already cancelled opens nothing",async()=>{
 const abort=new AbortController();abort.abort(Error("cancelled"));
 const control={signal:abort.signal,remainingMilliseconds:()=>60000,publish:async()=>{},yieldContinuation:async()=>{}};
 await expect(openNxDaemonWorkspaceWatcherV1({workspaceRoot:join(sandbox,"never"),repositoryRoot,control:control as never,signal:abort.signal,startupControl:control as never,environment})).rejects.toBeDefined();
 expect(existsSync(join(sandbox,"never"))).toBe(false);
});
