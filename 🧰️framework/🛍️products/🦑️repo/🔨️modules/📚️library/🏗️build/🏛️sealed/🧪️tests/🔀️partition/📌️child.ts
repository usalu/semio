import {adoptPartitionPinV1,releasePartitionPinV1} from "../../🔀️partition/📌️retention/🟦️.ts";
import {sealedPolicyV1} from "../../🟦️.ts";
const [root,original]=process.argv.slice(2),policy={...sealedPolicyV1,quietMilliseconds:0,maximumBytes:1,keepPrevious:0};
if(!root||!original)throw Error("Original reader required");
const token=adoptPartitionPinV1(root,original,policy);process.stdout.write(token+"\n");
try{await new Promise<void>(resolve=>process.stdin.once("data",()=>resolve()));}finally{releasePartitionPinV1(root,token,policy);}
