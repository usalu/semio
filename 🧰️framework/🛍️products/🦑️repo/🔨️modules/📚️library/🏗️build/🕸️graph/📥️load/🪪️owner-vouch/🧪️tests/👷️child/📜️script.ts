import {hash} from "node:crypto";
import {openOwnerCheckpointV1} from "../../../💾️owner-checkpoint/🟦️.ts";
import {observeOwnerFactsMemoizedV1} from "../../../🟦️.ts";
import {openOwnerVouchV1,vouchedOwnerV1} from "../../🟦️.ts";
import {rowsOf,feedOf,ownerQuestions,CHECKPOINT_SCOPE,CHECKPOINT_FORMAT} from "../🧰️support/🟦️.ts";

const [directory,root,ledger]=process.argv.slice(2) as [string,string,string],clock={at:0};
const checkpoint=await openOwnerCheckpointV1(directory,CHECKPOINT_SCOPE,CHECKPOINT_FORMAT,{check:()=>{},yieldContinuation:async()=>{},publish:(_paths,_plan,write)=>{write();return true;}});
const vouch=await openOwnerVouchV1({feed:feedOf([]),root,file:ledger,key:"k",insensitive:false,now:()=>(clock.at+=20000)});
for(const [index,question] of ownerQuestions.slice(0,3).entries()){
 const suffix=question.owner+"\0"+JSON.stringify([question.targets,question.inputs]),rows=rowsOf(root,index),vouched=vouchedOwnerV1(vouch,hash("sha256",suffix,"hex"),async()=>true);
 await observeOwnerFactsMemoizedV1({
  key:"scope\0"+suffix,hold:rows2=>vouched.hold(rows2 as never),
  observation:{captureObservedSourceInputsV1:async(compute:()=>Promise<unknown>)=>({value:await compute(),observations:rows}),retainObservedRowsV1:()=>{}} as never,
  observe:async()=>({project:{name:question.owner,targets:{}},files:[],dependencies:[]}),
  recover:()=>checkpoint.read(suffix),persist:entry=>checkpoint.save(suffix,entry as never),outcome:()=>vouched.outcome(),
 });
}
checkpoint.flush();
process.stdout.write("ready\n");
await new Promise<void>(accept=>setTimeout(accept,30000));
