/** 🪂️ Contract: a service that a client starts implicitly (no launchd route) is no part of that client's owned execution. Tearing the client down (the owned-tree kill of `terminateOwnedProcessTree`, `ps` ancestry on POSIX, `taskkill /T` on Windows) ends the client and leaves the service, its own session and group, its output in the service log, and every other client's load running. The control, a plain detached child, is still a descendant and is ended with its client.
 * @see ../../🏗️composition/🌐️host/🪂️detach/🟨️.mjs
 * @see ../../../../../../../🔨️modules/🏃️process/🪓️termination/🟦️.ts */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {spawn,spawnSync} from "node:child_process";
import {terminateOwnedProcessTree} from "../../../../../../../../🔨️modules/🏃️process/🪓️termination/🟦️.ts";
import {waitFor} from "../🟦️.ts";

const base=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","dt-"));
const CLIENT=fileURLToPath(new URL("./🧑️client.mjs",import.meta.url)),pids:number[]=[];
afterAll(()=>{for(const pid of pids)try{process.kill(pid,"SIGKILL");}catch{}rmSync(base,{recursive:true,force:true});});
const alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch{return false;}};
const posix=process.platform!=="win32";
const start=async(mode:string)=>{
 const client=spawn(process.execPath,[CLIENT,mode,join(base,mode+".log")],{stdio:["ignore","pipe","ignore"]});
 let text="";client.stdout.on("data",chunk=>{text+=String(chunk);});
 await waitFor(()=>text.includes("\n"),15000,20);
 const service=Number(text.trim());pids.push(client.pid!,service);
 return{client:client.pid!,service};
};
const parentOf=(pid:number)=>Number(spawnSync("ps",["-o","ppid=","-p",String(pid)],{encoding:"utf8"}).stdout.trim());
const groupOf=(pid:number)=>Number(spawnSync("ps",["-o","pgid=","-p",String(pid)],{encoding:"utf8"}).stdout.trim());

test("tearing down the owned execution of a client leaves the service it started through the relay and a bystander running",async()=>{
 const bystander=await start("relay"),{client,service}=await start("relay");
 expect(alive(service)).toBe(true);
 if(posix){expect(parentOf(service),"no descendant of the client").not.toBe(client);expect(groupOf(service),"a session and group of its own").toBe(service);}
 terminateOwnedProcessTree(client);
 expect(await waitFor(()=>!alive(client),10000,20)).toBe(true);
 await new Promise(done=>setTimeout(done,500));
 expect(alive(service),"the service survives its starter").toBe(true);expect(alive(bystander.service),"another client's service survives").toBe(true);expect(alive(bystander.client),"another client survives").toBe(true);
},40000);

test.skipIf(!posix)("control: a plainly detached child is still a descendant of its client and is ended with the owned execution",async()=>{
 const{client,service}=await start("plain");
 expect(parentOf(service)).toBe(client);
 terminateOwnedProcessTree(client);
 expect(await waitFor(()=>!alive(service),10000,20)).toBe(true);
},40000);
