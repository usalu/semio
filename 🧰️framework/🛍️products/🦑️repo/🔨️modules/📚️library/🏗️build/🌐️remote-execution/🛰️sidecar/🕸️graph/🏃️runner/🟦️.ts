import{closeSync,openSync,renameSync,rmSync,statSync,writeFileSync,writeSync}from"node:fs";

/** 🏃️ The detached runner of one graph read: the graph server starts it in a session of its own, it starts `repo show projects`, appends the read's progress output to the log file and keeps the read record (`ReadRecord` in `../../../../🪪️graph-state/🙋️wanted/🧬️schema/🔣️.json`) fresh every beat, so a graph server, a build agent or a launcher that is restarted for new code finds the read still running and attaches to it instead of losing it. It stops the read (SIGINT, SIGKILL ten seconds later) when it receives SIGINT or SIGTERM, and when nobody touched the watch file for the orphan time (a server that never came back). It writes the exit status of the read into the record and ends. Arguments: record file, log file, watch file, orphan milliseconds, beat milliseconds, working directory, reason, then the command. */
export async function runGraphReadRunnerV1(argv:readonly string[],environment:Readonly<Record<string,string|undefined>>=process.env):Promise<number>{
 const[record,log,watch,orphanText,beatText,cwd,reason,...command]=argv,orphan=Number(orphanText),beat=Number(beatText),startedMs=Date.now(),fd=openSync(log,"w");
 let read:number|null=null,status:number|null=null,endedMs:number|null=null,stopping=false;
 const write=():void=>{
  const temporary=`${record}.${process.pid}.tmp`;
  try{writeFileSync(temporary,JSON.stringify({version:1,reason,runner:process.pid,read,startedMs,beatMs:Date.now(),log,status,endedMs}));renameSync(temporary,record);}catch{try{rmSync(temporary,{force:true});}catch{}}
 };
 const child=Bun.spawn([...command],{cwd,windowsHide:true,stdin:"ignore",stdout:"ignore",stderr:"pipe",env:environment as Record<string,string>});
 read=child.pid;write();
 const stop=():void=>{if(stopping)return;stopping=true;try{child.kill("SIGINT");}catch{}setTimeout(()=>{try{child.kill("SIGKILL");}catch{}},10_000).unref();};
 for(const name of["SIGINT","SIGTERM"]as const)process.on(name,stop);
 const timer=setInterval(()=>{
  write();
  let touched=startedMs;try{touched=Math.max(startedMs,statSync(watch).mtimeMs);}catch{}
  if(Date.now()-touched>orphan)stop();
 },beat);
 const pump=(async()=>{for await(const chunk of child.stderr)try{writeSync(fd,chunk);}catch{}})();
 const code=await child.exited;
 await pump;clearInterval(timer);status=code??1;endedMs=Date.now();write();
 try{closeSync(fd);}catch{}
 return 0;
}

if(import.meta.main)process.exit(await runGraphReadRunnerV1(process.argv.slice(2)));
