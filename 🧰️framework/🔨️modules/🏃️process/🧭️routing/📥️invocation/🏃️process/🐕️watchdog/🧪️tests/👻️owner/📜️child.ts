/** 👻️ Child: an Original Script process whose work only ends on cancellation and that owns a grandchild process. */
import{spawn}from"node:child_process";
import{receiveScriptProcessInvocation}from"../../../🟦️.ts";
await receiveScriptProcessInvocation(process.env,async invocation=>{
 const grandchild=spawn(process.execPath,["-e","setInterval(()=>{},1000)"],{stdio:"ignore"});
 process.stdout.write(JSON.stringify({grandchild:grandchild.pid})+"\n");
 await new Promise<never>((_,fail)=>invocation.control.signal.addEventListener("abort",()=>fail(invocation.control.signal.reason),{once:true}));
});
