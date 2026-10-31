/** 🏭 Pool of resident executors on the service side: starts executor processes, hands each one command at a time, retires them, and reports when the repository refuses to run commands inside a resident invocation. */
import {spawn} from "node:child_process";

function markerFilter(marker,deliver,seen){
 let held="";
 return chunk=>{
  held+=chunk;
  const at=held.indexOf(marker);
  if(at>=0){if(at>0)deliver(held.slice(0,at));held=held.slice(at+marker.length);seen();return;}
  const keep=Math.min(held.length,marker.length-1);
  if(held.length>keep){deliver(held.slice(0,held.length-keep));held=held.slice(held.length-keep);}
 };
}

function lines(stream,onLine){
 let text="";
 stream.on("data",chunk=>{text+=chunk.toString("utf8");let at;while((at=text.indexOf("\n"))>=0){onLine(text.slice(0,at));text=text.slice(at+1);}});
}

/** 🏭 Creates the pool; `command` starts one executor process (it must speak the fd 3 protocol of `serveExecutorV1`). */
export function createExecutorPoolV1({command,environment,cwd,size,readyTimeoutMilliseconds=120000,spawnProcess=spawn,log=()=>undefined}){
 const executors=[];let refused=false,closed=false,starts=0;
 const start=()=>{
  starts++;
  const child=spawnProcess(command[0],command.slice(1),{cwd,env:environment,stdio:["ignore","pipe","pipe","pipe"]});
  const executor={child,state:"starting",pending:undefined,listeners:new Map()};
  child.stdio[3].on("error",error=>{
   log("executor-protocol",String(error?.message??error));
   executor.pending?.resolve({exit:70,lost:true});executor.pending=undefined;
   if(!closed)child.kill("SIGTERM");
  });
  executor.ready=new Promise(resolve=>{
   const timer=setTimeout(()=>{if(executor.state==="starting"){child.kill("SIGKILL");resolve(false);}},readyTimeoutMilliseconds);timer.unref?.();
   lines(child.stdio[3],line=>{
    let frame;try{frame=JSON.parse(line);}catch{return;}
    if(frame.t==="ready"&&executor.state==="starting"){executor.state="idle";clearTimeout(timer);resolve(true);}
    else if(frame.t==="fatal"){log("executor-fatal",frame.reason+": "+frame.message);}
    else if(frame.t==="done"){const waiting=executor.pending;executor.pending=undefined;if(frame.refusal==="authority")refused=true;executor.state="idle";waiting?.resolve({exit:frame.exit,refusal:frame.refusal,code:frame.code,fields:frame.fields});}
   });
   child.once("exit",()=>{
    const index=executors.indexOf(executor);if(index>=0)executors.splice(index,1);
    clearTimeout(timer);if(executor.state==="starting")resolve(false);
    executor.state="dead";executor.pending?.resolve({exit:70,lost:true});executor.pending=undefined;
   });
  });
  child.stderr.on("data",()=>{});child.stdout.on("data",()=>{});
  executors.push(executor);return executor;
 };
 return{
  /** 🚀 Starts executors up to `size` unless the repository refused them. */
  top(){if(closed||refused)return;while(executors.length<size)start();},
  get refused(){return refused;},
  idleCount:()=>executors.filter(executor=>executor.state==="idle").length,
  /** ▶ Runs one command in an idle executor; undefined when none is ready or the mode is refused (the caller uses a one-shot worker). */
  take(){
   if(refused)return undefined;
   const executor=executors.find(candidate=>candidate.state==="idle");
   if(!executor){if(!closed&&executors.length<size)start();return undefined;}
   executor.state="busy";return executor;
  },
  run(executor,request,{onOutput,signal}){
   return new Promise(resolve=>{
    const child=executor.child,marker="\u0000semio-end:"+request.id+"\u0000\n";
    let seen=0,done;const finish=()=>{if(seen===2&&done){signal?.removeEventListener("abort",cancel);resolve(done);}};
    child.stdout.removeAllListeners("data");child.stderr.removeAllListeners("data");
    const filterOut=markerFilter(marker,text=>void onOutput("stdout",text),()=>{seen++;finish();}),filterErr=markerFilter(marker,text=>void onOutput("stderr",text),()=>{seen++;finish();});
    child.stdout.on("data",chunk=>filterOut(chunk.toString("utf8")));child.stderr.on("data",chunk=>filterErr(chunk.toString("utf8")));
    const cancel=()=>child.kill("SIGINT");
    if(signal?.aborted)cancel();else signal?.addEventListener("abort",cancel,{once:true});
    executor.pending={resolve:value=>{done=value;if(value.lost){seen=2;}finish();}};
    child.stdio[3].write(JSON.stringify(request)+"\n");
   });
  },
  stats:()=>({executors:executors.length,starts,refused}),
  async close(){closed=true;for(const executor of[...executors]){try{executor.child.stdio[3].write('{"t":"stop"}\n');}catch{}setTimeout(()=>executor.child.kill("SIGTERM"),2000).unref?.();}},
 };
}
