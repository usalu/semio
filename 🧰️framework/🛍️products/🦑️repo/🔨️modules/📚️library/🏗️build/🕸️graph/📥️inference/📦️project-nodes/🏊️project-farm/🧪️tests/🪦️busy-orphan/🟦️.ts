/** 🪦️ Contract: a farm worker whose read process is killed ends within seconds even when its event loop is held by CPU bound work (a closed channel is only noticed by a loop that runs). Oracle: the process id of a real worker, probed by the operating system (`kill -0`). */
import{expect,test}from"bun:test";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{pathToFileURL}from"node:url";
import{serialize}from"node:v8";

const worker=join(import.meta.dir,"..","..","👷️worker","🟨️.mjs"),service=pathToFileURL(join(import.meta.dir,"..","🧫️fixtures","🪦️busy-service.mjs")).href;
const alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch(error){return(error as{code?:string}).code==="EPERM";}};

test("a busy worker ends within seconds of the kill of its read process",async()=>{
 const dir=mkdtempSync(join(tmpdir(),"farm-busy-orphan-")),sessionFile=join(dir,"session.bin");writeFileSync(sessionFile,serialize({}));
 const code=`const {spawn}=require("node:child_process");const c=spawn(process.execPath,[${JSON.stringify(worker)}],{stdio:["ignore","inherit","inherit","ipc"],env:{...process.env,SEMIO_GRAPH_WORKER_INIT:${JSON.stringify(JSON.stringify({service,sessionFile,slot:0,generation:0}))}}});console.log(c.pid);setInterval(()=>{},1000);`;
 const stub=Bun.spawn([process.execPath,"-e",code],{stdout:"pipe",stderr:"inherit"});
 let pid=0;
 try{
  pid=Number(new TextDecoder().decode((await stub.stdout.getReader().read()).value).trim());
  await Bun.sleep(1500);expect(alive(pid),"the worker runs").toBe(true);
  stub.kill("SIGKILL");await stub.exited;
  const began=Date.now();while(alive(pid)&&Date.now()-began<15000)await Bun.sleep(100);
  expect(alive(pid),"the worker outlived its read").toBe(false);expect(Date.now()-began).toBeLessThan(10000);
 }finally{try{process.kill(pid,"SIGKILL");}catch{}rmSync(dir,{recursive:true,force:true});}
},60000);
