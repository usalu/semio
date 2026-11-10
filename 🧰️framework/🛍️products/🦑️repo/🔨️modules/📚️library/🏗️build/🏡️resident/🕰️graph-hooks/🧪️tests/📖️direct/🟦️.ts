/** 📖️ Contract: with the real watcher and the real detached journal service and no resident repo service anywhere, an edit is found from the journal alone (a handful of stat calls, no scan), a killed journal service costs exactly one announced scan, and the journal is trusted again afterwards. */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {lstat,readdir} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {readChangeStateV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import {createFrontierV1} from "../../../../🕸️graph/🕰️generations/🧭️frontier/🟨️.mjs";
import {walkFrontierV1} from "../../../../🕸️graph/🕰️generations/🌿️frontier-walk/🟨️.mjs";
import {changeJournalDirectoryV1} from "../../../📜️journal/🟦️.ts";
import {residentWatcherV1,sleep,waitFor} from "../../../🧪️tests/🟦️.ts";
import {journalGraphSeamsV1} from "../../🔌️host/🟨️.mjs";
import {directJournalPortV1,journalCursorStoreV1} from "../../📖️direct/🟦️.ts";

const sandbox=realpathSync(mkdtempSync(join(tmpdir(),"graph-journal-")));
const binary=await residentWatcherV1();
const environment={...process.env,XDG_RUNTIME_DIR:sandbox,SEMIO_REPO_CHANGES:"on",SEMIO_REPO_WATCH_BINARY:binary,SEMIO_BUILD_SERVICE:"off"} as Record<string,string>;
const roots:string[]=[];
afterAll(async()=>{
 for(const root of roots){try{const state=readChangeStateV1(changeJournalDirectoryV1(root,environment));if(state)process.kill(state.pid,"SIGTERM");}catch{}}
 await sleep(500);rmSync(sandbox,{recursive:true,force:true});
});

test("journal-driven membership and fact checks without a resident service, including a killed journal service",async()=>{
 const root=join(sandbox,"w");roots.push(root);
 for(const[path,content]of Object.entries({"a/x/project.json":"{}","a/x/src/m.ts":"1","a/y/Cargo.toml":"c","b/z.feature":"f","b/node_modules/q/project.json":"{}"})){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content);}
 const counts={stat:0,list:0,scans:0},ports={
  list:async(directory:string)=>{counts.list++;try{return(await readdir(join(root,directory),{withFileTypes:true})).map(entry=>({name:entry.name,kind:entry.isSymbolicLink()?"other" as const:entry.isDirectory()?"directory" as const:entry.isFile()?"file" as const:"other" as const}));}catch{return undefined;}},
  stat:async(path:string)=>{counts.stat++;try{const status=await lstat(join(root,path));return{directory:status.isDirectory(),file:status.isFile(),mtimeMs:status.mtimeMs,ino:status.ino,mode:status.mode};}catch{return undefined;}},
  descend:(name:string)=>name!=="node_modules",
  member:(path:string)=>{const name=path.slice(path.lastIndexOf("/")+1);return name==="project.json"||name==="Cargo.toml"||name.endsWith(".feature");},
 };
 const notices:any[]=[],checked:string[][]=[],journal=directJournalPortV1(root,environment),store=journalCursorStoreV1(root,environment);
 const seams=journalGraphSeamsV1({root,journal,store,ports,digest:members=>"m"+members.length,membership:async()=>{counts.scans++;const walk=await walkFrontierV1(ports);return{digest:"scan-"+counts.scans,frontier:createFrontierV1({capturedAt:Date.now(),...walk}),changes:[]};},holds:async rows=>{checked.push(rows.map(row=>row.path));return true;},notify:notice=>notices.push(notice)});
 let generation=1;
 const cold=await seams.membership(undefined);
 expect(counts.scans).toBe(1);expect(notices).toEqual([]);
 seams.commit({generation,owners:[],covered:undefined});
 expect(readChangeStateV1(changeJournalDirectoryV1(root,environment))).toMatchObject({ready:true,trust:"exact"});

 writeFileSync(join(root,"a/x/src/m.ts"),"edited");writeFileSync(join(root,"b/new.feature"),"n");rmSync(join(root,"a/y/Cargo.toml"));
 counts.stat=0;counts.list=0;
 const live=await seams.membership({manifest:{generation,membership:cold.digest},frontier:cold.frontier});
 expect(counts.scans).toBe(1);expect(counts.list).toBe(0);expect(counts.stat).toBeLessThanOrEqual(12);
 expect(live.changes.filter((change:any)=>!change.directory).map((change:any)=>change.kind+" "+change.path).sort()).toEqual(["added b/new.feature","removed a/y/Cargo.toml"]);
 const fresh=createFrontierV1({capturedAt:0,...await walkFrontierV1(ports)});
 expect(live.frontier.members).toEqual(fresh.members);expect(live.frontier.directories.map((row:any)=>row[0])).toEqual(fresh.directories.map(row=>row[0]));
 expect(await seams.holds([{kind:"file",path:root+"/a/x/src/m.ts",fact:{}},{kind:"file",path:root+"/b/z.feature",fact:{}},{kind:"readdir",path:root+"/b/node_modules",fact:{}}])).toBe(true);
 expect(checked).toEqual([[root+"/a/x/src/m.ts",root+"/b/node_modules"]]);
 seams.commit({generation:++generation,owners:[],covered:undefined});

 const state=readChangeStateV1(changeJournalDirectoryV1(root,environment))!;
 process.kill(state.pid,"SIGKILL");
 await waitFor(()=>{try{process.kill(state.pid,0);return false;}catch{return true;}},5000,20);
 await sleep(5500);
 writeFileSync(join(root,"b/other.feature"),"o");renameSync(join(root,"a/x"),join(root,"a/w"));
 const scansBefore=counts.scans;
 const recovered=await seams.membership({manifest:{generation,membership:live.digest},frontier:live.frontier});
 expect(counts.scans).toBe(scansBefore+1);expect(notices.length).toBe(1);expect(["no-service","epoch","starting","unrecorded"]).toContain(notices[0].reason);
 expect(recovered.frontier.members.map((row:any)=>row[0])).toContain("a/w/project.json");
 seams.commit({generation:++generation,owners:[],covered:undefined});
 await sleep(200);writeFileSync(join(root,"b/third.feature"),"t");
 const again=await seams.membership({manifest:{generation,membership:recovered.digest},frontier:recovered.frontier});
 expect(counts.scans).toBe(scansBefore+1);expect(notices.length).toBe(1);
 expect(again.frontier.members.map((row:any)=>row[0])).toContain("b/third.feature");
},90000);
