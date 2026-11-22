import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const launcherModule=join(import.meta.dir,"..","..","🟦️.ts");

test("the launcher hands the service over to a successor that binds the same port before the first child drains and leaves: clients see no failed request, both generations answer, and the successor stays supervised",async()=>{
 const folder=mkdtempSync(join(tmpdir(),"sidecar-handover-"));scratch.push(folder);
 const entry=join(folder,"role.ts"),runner=join(folder,"run.ts"),probe=Bun.serve({port:0,hostname:"127.0.0.1",fetch:()=>new Response("")}),port=probe.port;probe.stop(true);
 writeFileSync(entry,`
const successor=process.env.SEMIO_SIDECAR_SUCCESSOR==="1",generation=successor?"2":"1";
const server=Bun.serve({port:${port},hostname:"127.0.0.1",reusePort:true,fetch:async()=>{await new Promise(done=>setTimeout(done,5));return new Response(generation);}});
process.send?.({type:"listening"});
if(!successor)setTimeout(()=>{process.send?.({type:"successor"});},500);
process.on("SIGTERM",async()=>{await server.stop(false);process.exit(75);});
setInterval(()=>{},1000);
`);
 writeFileSync(runner,`import {runRemoteExecutionSidecarLauncherV1} from ${JSON.stringify(launcherModule)};\nawait runRemoteExecutionSidecarLauncherV1("cache",${JSON.stringify(entry)},{});\n`);
 const launcher=Bun.spawn([process.execPath,runner],{stdout:"pipe",stderr:"pipe"});
 const failures:string[]=[],seen:Record<string,number>={};let running=true;
 try{
  for(let attempt=0;attempt<100;attempt++){try{if((await fetch(`http://127.0.0.1:${port}/`)).status===200)break;}catch{}await new Promise(done=>setTimeout(done,50));}
  const client=async():Promise<void>=>{while(running){try{const response=await fetch(`http://127.0.0.1:${port}/`),text=await response.text();if(response.status!==200)failures.push(String(response.status));else seen[text]=(seen[text]??0)+1;}catch(error){failures.push(String((error as Error).message));}}};
  const clients=Array.from({length:4},()=>client());
  await new Promise(done=>setTimeout(done,3500));
  running=false;await Promise.all(clients);
  expect(failures).toEqual([]);
  expect(seen["1"]??0).toBeGreaterThan(0);expect(seen["2"]??0).toBeGreaterThan(0);
  const last=await (await fetch(`http://127.0.0.1:${port}/`)).text();expect(last,"the successor serves after the handover").toBe("2");
 }finally{running=false;launcher.kill("SIGTERM");await launcher.exited;}
},60000);

test("when the first successor dies before it listens the old process keeps serving and the next successor takes over, with no failed request: the launcher hands the service over to a successor that binds the same port before the first child drains and leaves: clients see no failed request, both generations answer, and the successor stays supervised",async()=>{
 const folder=mkdtempSync(join(tmpdir(),"sidecar-handover-retry-"));scratch.push(folder);
 const entry=join(folder,"role.ts"),runner=join(folder,"run.ts"),probe=Bun.serve({port:0,hostname:"127.0.0.1",fetch:()=>new Response("")}),port=probe.port;probe.stop(true);
 writeFileSync(entry,`
import {existsSync,writeFileSync} from "node:fs";const successor=process.env.SEMIO_SIDECAR_SUCCESSOR==="1",generation=successor?"2":"1";if(successor&&!existsSync(${JSON.stringify(folder+"/marker")})){writeFileSync(${JSON.stringify(folder+"/marker")},"x");process.exit(1);}
const server=Bun.serve({port:${port},hostname:"127.0.0.1",reusePort:true,fetch:async()=>{await new Promise(done=>setTimeout(done,5));return new Response(generation);}});
process.send?.({type:"listening"});
if(!successor)setTimeout(()=>{process.send?.({type:"successor"});},500);
process.on("SIGTERM",async()=>{await server.stop(false);process.exit(75);});
setInterval(()=>{},1000);
`);
 writeFileSync(runner,`import {runRemoteExecutionSidecarLauncherV1} from ${JSON.stringify(launcherModule)};\nawait runRemoteExecutionSidecarLauncherV1("cache",${JSON.stringify(entry)},{});\n`);
 const launcher=Bun.spawn([process.execPath,runner],{stdout:"pipe",stderr:"pipe"});
 const failures:string[]=[],seen:Record<string,number>={};let running=true;
 try{
  for(let attempt=0;attempt<100;attempt++){try{if((await fetch(`http://127.0.0.1:${port}/`)).status===200)break;}catch{}await new Promise(done=>setTimeout(done,50));}
  const client=async():Promise<void>=>{while(running){try{const response=await fetch(`http://127.0.0.1:${port}/`),text=await response.text();if(response.status!==200)failures.push(String(response.status));else seen[text]=(seen[text]??0)+1;}catch(error){failures.push(String((error as Error).message));}}};
  const clients=Array.from({length:4},()=>client());
  await new Promise(done=>setTimeout(done,6500));
  running=false;await Promise.all(clients);
  expect(failures).toEqual([]);
  expect(seen["1"]??0).toBeGreaterThan(0);expect(seen["2"]??0).toBeGreaterThan(0);
  const last=await (await fetch(`http://127.0.0.1:${port}/`)).text();expect(last,"the successor serves after the handover").toBe("2");
 }finally{running=false;launcher.kill("SIGTERM");await launcher.exited;}
},60000);
