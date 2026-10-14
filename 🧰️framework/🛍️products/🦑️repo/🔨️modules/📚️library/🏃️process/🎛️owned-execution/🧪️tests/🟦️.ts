import {test,expect} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {readFileSync,existsSync,mkdirSync,rmSync} from "node:fs";
import {join} from "node:path";
import * as receiver from "../🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import corpus from "./🔣️.json";
import schema from "../📥️invocation/🧬️schema/🔣️.json";

type Policy=typeof corpus.policy;
type Run=(invocation:ScriptInvocation,command:string,args:string[],cwd:string,label:string,policy:Policy)=>Promise<void>;
const run=()=>{const value=(receiver as unknown as Record<string,unknown>).runRepositoryCommand;expect(typeof value).toBe("function");return value as Run;};
test("Repository receiving policy independently admits original time and distinct line authority",()=>{
 const admits=new Ajv({strict:true}).compile(schema);expect(admits(corpus.policy)).toBe(true);
 for(const policy of [{...corpus.policy,maximumElapsedMilliseconds:0},{...corpus.policy,output:{maximumBytes:4096}},{...corpus.policy,output:{...corpus.policy.output,maximumLines:65537}}])expect(admits(policy)).toBe(false);
});
for(const row of corpus.cases)test("Repository original receiving "+row.name,async()=>{
 const execute=run(),base=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!base)throw Error("Original receiving artifact directory required");
 const directory=join(base,"rr",row.mode),marker=join(directory,"acquired");mkdirSync(directory,{recursive:true});
 const original=new AbortController(),events:string[]=[],started=performance.now();let published=false;
 const invocation:ScriptInvocation={policy:{version:1,owner:row.name,maximumElapsedMilliseconds:5000},capabilities:{artifactDirectory:directory},control:{signal:original.signal,remainingMilliseconds:()=>5000-(performance.now()-started),publish:async progress=>{events.push(progress.stage);if(row.mode==="reject")throw Error("publication refused");await new Promise<void>(done=>setTimeout(done,20));published=true;},yieldContinuation:async()=>{events.push("yield");await new Promise<void>(done=>setImmediate(done));}}};
 if(row.mode==="cancel")original.abort();
 const source=row.mode==="lines"?'process.stdout.write("a\\nb\\nc\\n")':'require("node:fs").writeFileSync(process.argv[1],"ready");process.stdout.write("ready\\n")';
 const policy:Policy={...corpus.policy,maximumElapsedMilliseconds:row.mode==="zero"?0:5000};
 let outcome="ready",refusal:unknown;try{const pending=execute(invocation,"node",["-e",source,marker],process.cwd(),row.name,policy);expect(existsSync(marker)).toBe(false);await pending;}catch(error){outcome="refused";refusal=error;}
 const oracle=spawnSync("node",["-e",String.raw`const {spawnSync}=require("node:child_process");const mode=process.argv[1];if(["cancel","reject","zero"].includes(mode)){process.stdout.write("refused");}else{const child=spawnSync(process.execPath,["-e",mode==="lines"?'console.log("a");console.log("b");console.log("c")':'console.log("ready")'],{encoding:"utf8"});const lines=child.stdout.split(String.fromCharCode(10)).filter(Boolean).length;process.stdout.write(child.status===0&&lines<=2?"ready":"refused");}`,row.mode],{encoding:"utf8",timeout:5000});
 expect(oracle.status).toBe(0);expect(oracle.stdout).toBe(row.expected);expect(outcome).toBe(row.expected);
 if(row.mode==="cancel")expect(refusal).toBe(original.signal.reason);
 if(row.mode==="reject")expect(String(refusal)).toContain("publication refused");
 if(row.mode==="zero")expect(String(refusal)).toMatch(/(repository command policy|Finite original.*child|child.*authority)/iu);
 if(row.mode==="lines")expect(String(refusal)).toContain("output line limit");
 if(row.mode==="normal"){expect(published).toBe(true);expect(readFileSync(marker,"utf8")).toBe("ready");expect(events).toEqual(["loading","yield","complete","yield"]);}else if(["cancel","reject","zero"].includes(row.mode))expect(existsSync(marker)).toBe(false);
 console.log("[DEBUG] repository-receiving "+JSON.stringify({case:row.name,outcome,oracle:oracle.stdout,published,events}));
 rmSync(directory,{recursive:true,force:true});
},15000);
