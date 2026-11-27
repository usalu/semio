/** 🛡️ Contract: the resident service belongs to no client. Whoever starts it, killing that client (an agent ending its own `repo run`) neither ends the service nor the slice workers it carries: the service keeps neither the owner pid nor the admitted deadline of its starter, in the direct spawn and in the launchd job alike; the only owner it watches is its own spawner. The two cancellation cases of the shared load, a killed client with a follower and a client of two that goes, live in the suites `slice-share` and `cancel`.
 * @see ../../🏗️composition/🌐️host/🟨️.mjs `spawnHostServiceV1`
 * @see ../../🏗️composition/🌐️host/🍎️launchd/🟨️.mjs `allowedEnvironmentV1`
 * @see ../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts `startScriptProcessOwnerWatchV1` */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {spawn} from "node:child_process";
import {spawnHostServiceV1} from "../../🏗️composition/🌐️host/🟨️.mjs";
import {allowedEnvironmentV1} from "../../🏗️composition/🌐️host/🍎️launchd/🟨️.mjs";
import {sleep,waitFor} from "../🟦️.ts";

const base=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","ck-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const WATCH=fileURLToPath(new URL("./👁️owner-watch.mjs",import.meta.url));
const alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch{return false;}};
const OWNER="SEMIO_SCRIPT_PROCESS_OWNER";

const killedClient=async()=>{
 const client=spawn(process.execPath,["-e","setInterval(()=>{},1000)"],{stdio:"ignore"});
 await waitFor(()=>client.pid!==undefined&&alive(client.pid),5000,20);
 const pid=client.pid!;client.kill("SIGKILL");await new Promise(done=>client.once("exit",done));
 return pid;
};

test("an owner watch that inherits a killed client as owner ends its process: the cause of the service shutdowns with reason cancelled",async()=>{
 const pid=await killedClient(),watched=join(base,"control-watched"),lost=join(base,"control-lost");
 const child=spawn(process.execPath,[WATCH,watched,lost],{env:{...process.env,[OWNER]:String(pid)},stdio:"ignore"});
 try{expect(await waitFor(()=>existsSync(lost),15000,50)).toBe(true);expect(readFileSync(lost,"utf8")).toBe(String(pid));}finally{child.kill("SIGKILL");}
},30000);

test("a service started by a client does not take the client as its owner: killing the client leaves it running",async()=>{
 const directory=join(base,"runtime"),watched=join(base,"service-watched"),lost=join(base,"service-lost");
 const client=spawn(process.execPath,["-e","setInterval(()=>{},1000)"],{stdio:"ignore"});
 await waitFor(()=>client.pid!==undefined&&alive(client.pid),5000,20);
 const start=spawnHostServiceV1({command:[process.execPath,WATCH,watched,lost],environment:{PATH:process.env.PATH??"",SEMIO_SERVICE_LAUNCHD:"off",[OWNER]:String(client.pid),SEMIO_SCRIPT_PROCESS_INVOCATION:"{}"}});
 const handle=start({runtime:{directory,log:join(base,"service.log")}}) as {pid:number}|undefined;
 try{
  expect(await waitFor(()=>existsSync(watched),15000,50)).toBe(true);
  expect(readFileSync(watched,"utf8"),"the service carries no owner of its starter").toBe("");
  client.kill("SIGKILL");await new Promise(done=>client.once("exit",done));
  await sleep(5000);
  expect(existsSync(lost),"the service was not ended with its starter").toBe(false);
  expect(alive(handle!.pid)).toBe(true);
 }finally{if(handle?.pid)try{process.kill(handle.pid,"SIGKILL");}catch{}client.kill("SIGKILL");}
},40000);

test("the launchd job and its consent probe carry no owner, no deadline and no secret of the client that asked for them",()=>{
 const allowed=allowedEnvironmentV1({[OWNER]:"4711",SEMIO_SCRIPT_PROCESS_INVOCATION:"{}",SEMIO_LANGUAGE:"en",HOME:"/h",PATH:"/p",SEMIO_X_TOKEN:"t"});
 expect(Object.keys(allowed).sort()).toEqual(["HOME","PATH","SEMIO_LANGUAGE"]);
});
