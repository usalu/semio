import{spawn}from"node:child_process";
import{readFileSync,renameSync,writeFileSync}from"node:fs";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};

/** 🏃️ The process a stage runs under: it starts the real command with the output it was given, forwards termination, and writes `{exit, signal, spawnFailed, endedMs}` (schema `Exit`: the signal that ended the command, `null` for a plain exit) to the exit file (tmp + rename) when the command ends, so a stage that outlives the process that started it, a restart of the `ready` child or of the agent, is found finished by whoever looks next. Usage: `<bun> wrapper <exit file> <command> [arguments...]`. */
const argv=process.argv.slice(2),prefix="--launch-record=",record=argv[0]?.startsWith(prefix)?argv.shift()!.slice(prefix.length):undefined;
const [exitFile,command,...args]=argv;
if(exitFile===undefined||command===undefined)process.exit(2);
if(record!==undefined){
 const policy=schema.definitions.Policy.const,deadline=performance.now()+policy.leaseStaleMilliseconds;
 const admitted=():boolean=>{try{const row=JSON.parse(readFileSync(record,"utf8"));return row.pid===process.pid&&row.exitFile===exitFile&&row.launch?.version===1&&row.stop===undefined;}catch{return false;}};
 while(!admitted()){
  if(performance.now()>=deadline)process.exit(2);
  await new Promise<void>(done=>setTimeout(done,policy.pollMilliseconds));
 }
}
const end=(exit:number|null,spawnFailed:boolean,signal:string|null=null):void=>{
 writeFileSync(`${exitFile}.tmp`,JSON.stringify({exit,signal,spawnFailed,endedMs:Date.now()}));
 renameSync(`${exitFile}.tmp`,exitFile);
 process.exit(0);
};
const child=spawn(command,args,{stdio:"inherit",windowsHide:true});
child.on("error",()=>end(null,true));
child.on("close",(code,signal)=>end(code,false,signal));
for(const name of["SIGTERM","SIGINT"] as const)process.on(name,()=>{try{child.kill(name);}catch{}});
