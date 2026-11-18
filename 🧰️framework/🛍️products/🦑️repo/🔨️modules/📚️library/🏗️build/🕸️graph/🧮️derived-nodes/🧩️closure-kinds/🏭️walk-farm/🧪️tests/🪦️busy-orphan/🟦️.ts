/** 🪦️ Contract: a walk farm child whose read process is killed ends within seconds even when its event loop is held by CPU bound work (a closed channel is only noticed by a loop that runs). Oracle: the process id of a real child, probed by the operating system (`kill -0`). */
import{expect,test}from"bun:test";
import{join}from"node:path";

const child=join(import.meta.dir,"..","..","👷️child","🟨️.mjs"),busy=join(import.meta.dir,"🪦️busy-deriver.mjs");
const alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch(error){return(error as{code?:string}).code==="EPERM";}};

test("a busy walk child ends within seconds of the kill of its read process",async()=>{
 const code=`const {spawn}=require("node:child_process");const c=spawn(process.execPath,[${JSON.stringify(child)},"/","${busy}"],{stdio:["pipe","ignore","ignore"]});console.log(c.pid);setInterval(()=>{},1000);`;
 const stub=Bun.spawn([process.execPath,"-e",code],{stdout:"pipe",stderr:"inherit"});
 let pid=0;
 try{
  pid=Number(new TextDecoder().decode((await stub.stdout.getReader().read()).value).trim());
  await Bun.sleep(2000);expect(alive(pid),"the child runs").toBe(true);
  stub.kill("SIGKILL");await stub.exited;
  const began=Date.now();while(alive(pid)&&Date.now()-began<15000)await Bun.sleep(100);
  expect(alive(pid),"the child outlived its read").toBe(false);expect(Date.now()-began).toBeLessThan(10000);
 }finally{try{process.kill(pid,"SIGKILL");}catch{}}
},60000);
