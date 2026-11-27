import {spawnSync} from "node:child_process";

const RELAY_V1=`const{spawn}=require("node:child_process"),{openSync}=require("node:fs"),[argv,cwd,log]=JSON.parse(process.argv[1]),fd=openSync(log,"a",0o600),child=spawn(argv[0],argv.slice(1),{cwd,env:process.env,stdio:["ignore",fd,fd],detached:true,windowsHide:true});child.unref();process.stdout.write(String(child.pid)+"\\n",()=>process.exit(0));`;

/** 🪂️ Starts a process that belongs to no owned execution of the caller: a short relay starts it in a session and process group of its own (`detached`: `setsid` on POSIX, a detached process on Windows) with its output in `log` and no stream shared with the caller, and ends at once, so the process is no descendant of the caller any more (a tree kill of the caller, `ps`-based on POSIX and `taskkill /T` on Windows, finds no link from the caller to it). Answers its pid, `undefined` when it did not start.
 * @see ../../../🧪️tests/🪂️detach/🟦️.ts */
export function spawnDetachedV1({argv,cwd,env,log,run=spawnSync}){
 const relay=run(process.execPath,["-e",RELAY_V1,JSON.stringify([argv,cwd,log])],{env,stdio:["ignore","pipe","ignore"],encoding:"utf8",windowsHide:true,timeout:30000});
 const pid=Number.parseInt(String(relay.stdout??"").trim(),10);
 return Number.isSafeInteger(pid)&&pid>0?pid:undefined;
}
