import {expect,test} from "bun:test";
import {existsSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {constants,tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import cases from "./🧫️fixtures/🔣️.json";

const command=new URL("../../⌨️command/🟦️.ts",import.meta.url).href;
const fixtureGrandchild=`
import { existsSync, writeFileSync } from "node:fs";
const root = process.env.SEMIO_CAPACITY_SIGNAL_FIXTURE;
const signal = process.env.SEMIO_CAPACITY_SIGNAL_CASE;
process.on(signal, () => writeFileSync(root + "/signal", signal));
writeFileSync(root + "/ready", String(process.pid));
setInterval(() => { if (existsSync(root + "/release")) process.exit(0); }, 20);
setTimeout(() => process.exit(2), 8000);
`;
const fixtureChild=`
import { spawn } from "node:child_process";
spawn(process.execPath, ["-e", process.env.SEMIO_CAPACITY_SIGNAL_GRANDCHILD], { detached: true, stdio: "ignore" });
`;
const fixtureRunner=`
import { runCapacityCommandV1 } from ${JSON.stringify(command)};
const host = {version:1,atMs:Date.now(),platform:process.platform,cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:null,openFilesMaximum:null,attachedImages:null};
process.exitCode = await runCapacityCommandV1(["run","--memory=100","--",process.execPath,"-e",process.env.SEMIO_CAPACITY_SIGNAL_CHILD], {directory:process.env.SEMIO_CAPACITY_SIGNAL_FIXTURE,environment:process.env,write:()=>undefined,sample:async()=>({host,swappedOutMegabytes:0})});
`;

async function waitFor(condition:()=>boolean):Promise<boolean>{
 for(let attempt=0;attempt<100;attempt++){
  if(condition())return true;
  await sleep(20);
 }
 return false;
}

for(const {signal,status} of cases)test.skipIf(process.platform==="win32")(`capacity run holds its grant while the grandchild handles ${signal}`,async()=>{
 const directory=mkdtempSync(join(tmpdir(),"hb-signal-")),release=join(directory,"release"),ready=join(directory,"ready"),seen=join(directory,"signal");
 const {SEMIO_HOST_CAPACITY_GRANT:_grant,SEMIO_HOST_CAPACITY_PLAN:_plan,...environment}=process.env;
 const fixture=Bun.spawn([process.execPath,"-e",fixtureRunner],{env:{...environment,LANG:"en_US.UTF-8",SEMIO_CAPACITY_SIGNAL_FIXTURE:directory,SEMIO_CAPACITY_SIGNAL_CASE:signal,SEMIO_CAPACITY_SIGNAL_CHILD:fixtureChild,SEMIO_CAPACITY_SIGNAL_GRANDCHILD:fixtureGrandchild},stdout:"ignore",stderr:"inherit"});
 try{
  expect(await waitFor(()=>existsSync(ready))).toBe(true);
  expect(readdirSync(join(directory,"grants"))).toHaveLength(1);
  fixture.kill(signal as "SIGINT"|"SIGTERM");
  expect(await waitFor(()=>existsSync(seen))).toBe(true);
  expect(readFileSync(seen,"utf8")).toBe(signal);
  expect(readdirSync(join(directory,"grants"))).toHaveLength(1);
  writeFileSync(release,"");
  expect(status).toBe(128+constants.signals[signal as "SIGINT"|"SIGTERM"]);
  expect(await fixture.exited).toBe(status);
  expect(readdirSync(join(directory,"grants"))).toHaveLength(0);
 }finally{
  writeFileSync(release,"");
  if(fixture.exitCode===null)fixture.kill("SIGTERM");
  await fixture.exited;
  rmSync(directory,{recursive:true,force:true});
 }
},12000);
