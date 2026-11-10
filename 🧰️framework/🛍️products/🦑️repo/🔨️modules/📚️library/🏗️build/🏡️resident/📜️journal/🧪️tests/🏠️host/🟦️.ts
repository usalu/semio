/** 🏠️ Contract: the change journals of many workspaces run in one host process over one native watcher process; each workspace sees only its own changes, a stopped host is replaced by a fresh one with a new epoch for everybody. */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {readChangeStateV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import {createChangeJournalClientV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {liveChildPidsV1} from "../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";
import {observeTreeMegabytesV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";
import {residentWatcherV1,sleep,waitFor} from "../../../🧪️tests/🟦️.ts";
import {changeJournalDirectoryV1,changeJournalHostDirectoryV1,startChangeJournalServiceV1} from "../../🟦️.ts";

const sandbox=realpathSync(mkdtempSync(join(tmpdir(),"journal-host-")));
const binary=await residentWatcherV1();
const environment={...process.env,XDG_RUNTIME_DIR:sandbox,SEMIO_REPO_CHANGES:"on"} as Record<string,string>;
const roots:string[]=[];
afterAll(async()=>{for(const root of roots){try{process.kill(readChangeStateV1(changeJournalDirectoryV1(root,environment))!.pid,"SIGTERM");}catch{}}await sleep(500);rmSync(sandbox,{recursive:true,force:true});});

test("three workspaces, one host process, one watcher process; changes stay with their workspace; a killed host is replaced",async()=>{
 const workspaces=["a","b","c"].map(name=>{const root=join(sandbox,name);mkdirSync(root,{recursive:true});roots.push(root);return{root,directory:changeJournalDirectoryV1(root,environment)};});
 const start=(workspace:{root:string;directory:string})=>startChangeJournalServiceV1({...workspace,environment,arguments:["--binary",binary]});
 await Promise.all(workspaces.map(start));
 const states=workspaces.map(workspace=>readChangeStateV1(workspace.directory)!);
 for(const state of states)expect(state).toMatchObject({ready:true,trust:"exact"});
 expect(new Set(states.map(state=>state.pid)).size).toBe(1);expect(new Set(states.map(state=>state.epoch)).size).toBe(3);
 const host=states[0]!.pid,watchers=liveChildPidsV1(host);expect(watchers).toHaveLength(1);
 const memory=await observeTreeMegabytesV1(host,undefined,undefined,undefined,undefined,"resident");expect(memory).not.toBeNull();
 const clients=workspaces.map(workspace=>createChangeJournalClientV1({directory:workspace.directory,root:workspace.root,environment,start:()=>start(workspace)}));
 const cursors=await Promise.all(clients.map(client=>client.cursor())) as {epoch:string;seq:number}[];
 writeFileSync(join(workspaces[0]!.root,"only-a.ts"),"a");writeFileSync(join(workspaces[2]!.root,"only-c.ts"),"c");
 const answers=await Promise.all(clients.map((client,index)=>client.since(cursors[index]!,[""])));
 expect(answers.map(answer=>answer.exact?answer.rows.map(row=>row[2]):"refused")).toEqual([["only-a.ts"],[],["only-c.ts"]]);
 expect(readChangeStateV1(changeJournalHostDirectoryV1(workspaces[0]!.root,environment)+"/never")).toBeUndefined();
 process.kill(host,"SIGKILL");
 await waitFor(()=>{try{process.kill(host,0);return false;}catch{return true;}},5000,20);
 await sleep(5500);
 const fresh=await clients[1]!.cursor() as {epoch:string};
 expect(fresh.epoch).not.toBe(cursors[1]!.epoch);
 expect(readChangeStateV1(workspaces[1]!.directory)!.pid).not.toBe(host);
},60000);
