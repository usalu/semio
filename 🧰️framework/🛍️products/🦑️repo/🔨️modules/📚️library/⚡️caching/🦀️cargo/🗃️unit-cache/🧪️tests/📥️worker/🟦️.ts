import {acquireSlotV1,acquireUnitClaimV1,sleepV1} from "../../🔒️lock/🟦️.ts";

/** 🧑‍🏭️ A separate OS process for the lock tests and the slow compiler shim of the scenario tests. */
const [mode,...rest]=process.argv.slice(2);
const print=(row:object)=>console.log(JSON.stringify(row));
if(mode==="claim"){
 const [directory,name,hold]=rest,asked=Date.now(),claim=await acquireUnitClaimV1(directory!,name!,{maximumWaitMs:60000});
 const acquired=Date.now();await sleepV1(Number(hold));claim?.release();
 print({role:"claim",pid:process.pid,asked,acquired,released:Date.now(),held:claim!==undefined});
}else if(mode==="die"){
 const [directory,name]=rest,claim=await acquireUnitClaimV1(directory!,name!,{});print({role:"die",pid:process.pid,held:claim!==undefined});
 await sleepV1(600000);
}else if(mode==="slot"){
 const [root,count,hold]=rest,asked=Date.now(),claim=await acquireSlotV1(root!,Number(count));
 const acquired=Date.now();await sleepV1(Number(hold));claim.release();
 print({role:"slot",pid:process.pid,asked,acquired,released:Date.now()});
}else throw Error("Unknown worker mode");
