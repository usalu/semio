import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {readdirSync,readFileSync,statSync} from "node:fs";
import {join,resolve} from "node:path";
import {fileURLToPath} from "node:url";

/** 🧪️ A unit whose process ends before its work settled must fail, never succeed: Cargo starts the dependants of a unit that reported success, and they find no output (`extern location for <crate> does not exist`, `could not execute process <build script> (never executed)`), which is what a fleet request that failed in a process with nothing else to wait for did to the norm test runs (ticket GET-NORM-PLUGIN-WORKING-END-TO-END, vanishing-artifacts.md).
 * @see https://nodejs.org/api/process.html#processexitcode */
const here=fileURLToPath(new URL(".",import.meta.url)),exit=resolve(here,"../🟦️.ts"),unitCache=resolve(here,"../..");
const run=(body:string)=>spawnSync(process.execPath,["-e",`import {exitWithStatusOfV1,UNFINISHED_EXIT_STATUS_V1} from ${JSON.stringify(exit)};${body}`],{encoding:"utf8",timeout:60000});

test("work that never settles ends the process with the unfinished status, never with success",()=>{
 const child=run("exitWithStatusOfV1(()=>new Promise(()=>{}));");
 expect(child.status,child.stderr).toBe(70);
});

test("the unfinished status is the one the compiler stand-in reports",async()=>{
 expect((await import(exit)).UNFINISHED_EXIT_STATUS_V1).toBe(70);
 expect((await import(resolve(unitCache,"📜️script.ts"))).UNFINISHED_EXIT_STATUS_V1).toBe(70);
});

test("settled work ends the process with its own status",()=>{
 for(const status of[0,1,7])expect(run(`exitWithStatusOfV1(async()=>${status});`).status).toBe(status);
 const lingering=run("setInterval(()=>{},60000);exitWithStatusOfV1(async()=>0);");
 expect(lingering.status).toBe(0);
});

test("work that rejects ends the process with status 1 and says why",()=>{
 const child=run("exitWithStatusOfV1(async()=>{throw new Error('boom');});");
 expect(child.status).toBe(1);expect(child.stderr).toContain("boom");
});

test("no process main of the unit cache sets its status from a bare then or an await, every one goes through the guard",()=>{
 const found:string[]=[],walk=(directory:string)=>{
  for(const name of readdirSync(directory)){
   const path=join(directory,name);
   if(statSync(path).isDirectory()){if(!name.includes("tests")&&!name.includes("fixtures"))walk(path);continue;}
   if(!name.endsWith(".ts")||path===exit)continue;
   if(/process\.exitCode\s*=\s*await\b|\.then\(\s*\(?status\)?\s*=>\s*\{\s*process\.exitCode\s*=\s*status/u.test(readFileSync(path,"utf8")))found.push(path.slice(unitCache.length+1));
  }
 };
 walk(unitCache);
 expect(found).toEqual([]);
});
