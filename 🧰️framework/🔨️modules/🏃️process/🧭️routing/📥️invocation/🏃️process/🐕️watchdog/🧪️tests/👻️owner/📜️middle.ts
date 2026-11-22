/** 👻️ Middle: spawns the owned child with the owner handed over by the real envelope environment, reports both pids, then waits to be killed. */
import{spawn}from"node:child_process";
import{fileURLToPath}from"node:url";
import{createScriptProcessEnvelope,scriptProcessEnvironment}from"../../../🟦️.ts";
const envelope=createScriptProcessEnvelope({version:1,owner:"test:owner-watch",maximumElapsedMilliseconds:0},{},Date.now());
const child=spawn(process.execPath,[fileURLToPath(new URL("./📜️child.ts",import.meta.url))],{env:scriptProcessEnvironment(envelope,process.env) as NodeJS.ProcessEnv,stdio:["ignore","inherit","inherit"]});
process.stdout.write(JSON.stringify({middle:process.pid,child:child.pid})+"\n");
setInterval(()=>{},1000);
