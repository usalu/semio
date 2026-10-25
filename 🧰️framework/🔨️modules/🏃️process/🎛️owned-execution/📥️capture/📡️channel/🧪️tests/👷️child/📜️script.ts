import{receiveScriptProcessInvocation,readIssuedScriptProcessEnvelopeV1}from"../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import corpus from"../🔣️.json";

/** 🧪️ Receives only the parent-issued fixed interactive test birth. */
async function runScriptProgram():Promise<void>{const selected=process.env.SEMIO_OWNED_CHANNEL_CASE;if(!selected||selected==="blocked-close"||!corpus.cases.some(row=>row.case===selected))throw Error("Closed channel Source child case required");await receiveScriptProcessInvocation(process.env,async invocation=>{console.error("[TRACE] actual channel diagnostic");console.log(JSON.stringify({pid:process.pid,birth:readIssuedScriptProcessEnvelopeV1(invocation).policy.maximumElapsedMilliseconds}));if(selected==="terminal"){process.stdout.end();await Bun.sleep(200);}else if(selected==="exchange"){const{createInterface}=await import("node:readline");let count=0;for await(const line of createInterface({input:process.stdin})){console.log(line.toUpperCase());if(++count===2){process.stdin.destroy();break;}}}else if(selected==="overflow"){await Bun.sleep(100);process.stdout.write("x".repeat(4096));await new Promise(()=>{});}else{process.stdin.resume();await new Promise(()=>{});}});}

if(import.meta.main)await runScriptProgram();
