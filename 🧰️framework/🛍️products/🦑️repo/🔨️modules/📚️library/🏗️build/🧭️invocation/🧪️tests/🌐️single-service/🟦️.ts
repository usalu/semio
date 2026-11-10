/** 🌐️ Contract: eight concurrent `repo run --dry-run` clients for eight different projects start exactly one service process; that process answers every plan (one pid in every load record, one workspace, one index) and no other planner process exists, while every client executes against the slice it was handed and the scheduler admitted all of them. */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {requestHostStatusV1} from "../../../🏡️resident/🟨️.mjs";

const here=fileURLToPath(new URL(".",import.meta.url)),root=realpathSync(fileURLToPath(new URL("../../../../../../../../../",import.meta.url)));
const base=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","ss-")),runtime=join(base,"run"),implementation="b".repeat(64),loadLog=join(base,"loads.ndjson");
mkdirSync(runtime,{recursive:true});
const environment={...process.env,XDG_RUNTIME_DIR:runtime,TMPDIR:base,LOCALAPPDATA:runtime,SEMIO_BUILD_TOOL:"nx",SEMIO_SCHEDULER_DIRECTORY:join(base,"scheduler"),SEMIO_HOST_CAPACITY_DIRECTORY:join(base,"capacity"),SEMIO_AGENT_SESSION_DIRECTORY:join(base,"sessions")} as Record<string,string>;
afterAll(async()=>{const status=await requestHostStatusV1({implementation,environment});if(status)try{process.kill(status.pid,"SIGTERM");}catch{}rmSync(base,{recursive:true,force:true});});
const processes=()=>spawnSync("ps",["-eo","pid=,args="],{encoding:"utf8"}).stdout.split("\n").filter(Boolean);

test.skipIf(process.platform==="win32")("eight dry-run clients, one service process, one index, no other planner process",async()=>{
 const projects=Array.from({length:8},(_,index)=>"fixture-project-"+index);
 const children=projects.map(project=>Bun.spawn([process.execPath,join(here,"🧑️client-main.ts"),root,implementation,join(here,"🖥️service-main.mjs"),loadLog,"run",project+":build","--dry-run"],{env:environment,stdout:"pipe",stderr:"pipe"}));
 const results=await Promise.all(children.map(async child=>({code:await child.exited,out:await new Response(child.stdout).text(),err:await new Response(child.stderr).text()})));
 results.forEach((row,index)=>{expect(row.code,row.err).toBe(0);expect(JSON.parse(row.out.trim().split("\n").pop()!)).toEqual({executed:1,projects:[projects[index]],handoffCleared:true,exitCode:0});});
 const loads=readFileSync(loadLog,"utf8").trim().split("\n").map(line=>JSON.parse(line) as {pid:number}),pids=new Set(loads.map(row=>row.pid));
 expect(loads).toHaveLength(8);expect(pids.size).toBe(1);
 const status=await requestHostStatusV1({implementation,environment});
 expect(status?.pid).toBe([...pids][0]);expect(status?.workspaces).toHaveLength(1);expect(status?.indexes).toBe(1);
 const rows=processes();
 expect(rows.filter(row=>row.includes("🖥️service-main.mjs")&&row.includes(implementation)&&row.includes(loadLog))).toHaveLength(1);
 expect(rows.filter(row=>/graph-plan|build:graph|semio-graph-plan/u.test(row))).toHaveLength(0);
 const log=JSON.parse(spawnSync(process.execPath,["-e","process.stdout.write(JSON.stringify(require('fs').readdirSync(process.argv[1])))",join(base,"scheduler")],{encoding:"utf8"}).stdout) as string[];
 expect(log.length).toBeGreaterThan(0);
},120000);
