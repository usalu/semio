import{spawn}from"node:child_process";
import{renameSync,writeFileSync}from"node:fs";

/** 🏃️ The process a stage runs under: it starts the real command with the output it was given, forwards termination, and writes `{exit, signal, spawnFailed, endedMs}` (schema `Exit`: the signal that ended the command, `null` for a plain exit) to the exit file (tmp + rename) when the command ends, so a stage that outlives the process that started it, a restart of the `ready` child or of the agent, is found finished by whoever looks next. Usage: `<bun> wrapper <exit file> <command> [arguments...]`. */
const [exitFile,command,...args]=process.argv.slice(2);
if(exitFile===undefined||command===undefined)process.exit(2);
const end=(exit:number|null,spawnFailed:boolean,signal:string|null=null):void=>{
 writeFileSync(`${exitFile}.tmp`,JSON.stringify({exit,signal,spawnFailed,endedMs:Date.now()}));
 renameSync(`${exitFile}.tmp`,exitFile);
 process.exit(0);
};
const child=spawn(command,args,{stdio:"inherit",windowsHide:true});
child.on("error",()=>end(null,true));
child.on("close",(code,signal)=>end(code,false,signal));
for(const name of["SIGTERM","SIGINT"] as const)process.on(name,()=>{try{child.kill(name);}catch{}});
