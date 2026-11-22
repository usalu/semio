import {openOwnerCheckpointV1} from "../../🟦️.ts";
import cases from "../🔣️.json";

const directory=process.argv[2]!;
const checkpoint=await openOwnerCheckpointV1(directory,cases.scope,cases.format,{
 check:()=>{},
 yieldContinuation:async()=>{},
 publish:(_paths,_plan,write)=>{write();return true;},
});
for(const [index,question] of cases.questions.slice(0,2).entries()){
 const {owner,targets,inputs}=question;
 checkpoint.save(owner+"\0"+JSON.stringify([targets,inputs]),{
  facts:{project:{name:owner,targets:{}},files:["manifest-"+index],dependencies:[]},
  rows:[{kind:"exists",path:"/root/manifest-"+index,fact:true}],
 });
}
process.stdout.write("ready\n");
await new Promise<void>(accept=>setTimeout(accept,30000));
