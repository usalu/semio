import {adoptAgentSessionV1,agentSessionAddressV1,currentAgentSessionV1} from "../../🟦️.ts";
import {acquireResourceLease} from "../../../🔒️leases/🟦️.ts";

const spec=JSON.parse(process.argv[2]!) as Record<string,any>;
const send=(line:string):void=>{process.stdout.write(line+"\n");};
const reader=Bun.stdin.stream().getReader(),decoder=new TextDecoder();
let buffered="";
async function line():Promise<string>{
 for(;;){
  const at=buffered.indexOf("\n");
  if(at>=0){const text=buffered.slice(0,at);buffered=buffered.slice(at+1);return text;}
  const chunk=await reader.read();if(chunk.done)return "";
  buffered+=decoder.decode(chunk.value);
 }
}

if(spec.mode==="open"){
 const session=currentAgentSessionV1({worktree:spec.worktree,ancestry:()=>spec.ancestry??[],...spec.agent===undefined?{}:{agent:spec.agent},...spec.ownerPid===undefined?{}:{ownerPid:spec.ownerPid}});
 send("SESSION "+JSON.stringify({id:session.id,address:agentSessionAddressV1(session),anchors:session.anchors,agent:session.agent}));
}else if(spec.mode==="adopt"){
 const session=adoptAgentSessionV1(process.env,{worktree:spec.worktree,ancestry:()=>spec.ancestry??[]});
 send("ADOPTED "+JSON.stringify({id:session.id,address:process.env.SEMIO_AGENT_SESSION}));
 const child=Bun.spawnSync([process.execPath,"-e","const {currentAgentSessionV1}=await import("+JSON.stringify(new URL("../../🟦️.ts",import.meta.url).href)+");console.log(JSON.stringify({id:currentAgentSessionV1({ancestry:()=>{throw Error('a child must not walk the ancestry')}}).id}))"],{env:process.env,stdout:"pipe",stderr:"pipe"});
 send("CHILD "+child.stdout.toString().trim()+(child.exitCode===0?"":" FAILED "+child.stderr.toString()));
}else if(spec.mode==="lease"){
 adoptAgentSessionV1(process.env,{worktree:spec.worktree,ancestry:()=>spec.ancestry??[]});
 const held=await acquireResourceLease({directory:spec.directory,resource:spec.resource,mode:"exclusive",signal:new AbortController().signal});
 send("HELD");await line();held.release();send("DONE");
}else if(spec.mode==="hold"){
 send("READY "+process.pid);await line();
}
process.exit(0);
