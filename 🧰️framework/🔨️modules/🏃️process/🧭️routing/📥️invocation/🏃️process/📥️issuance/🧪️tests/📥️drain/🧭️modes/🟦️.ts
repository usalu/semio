import{expect,test}from"bun:test";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1}from"../../../../🟦️.ts";
import{captureOwnedCommandResultV1}from"../../../../../../../🎛️owned-execution/🟦️.ts";
import{proveAcceptedOriginalDrainV1}from"../🟦️.ts";
import corpus from"./🔣️.json";
const selected=process.env.SEMIO_DRAIN_MODE;

/** 🌊️ Drives every actual drain mode of the issuance law, each as the only Original of its own process, so the law family is routed and not left without a caller. @see ../🟦️.ts */
if(selected===undefined)test("every actual drain mode keeps accepted work inside its Original until it settles",async()=>receiveScriptProcessInvocation(process.env,async invocation=>{
 const control=createIssuedResourceControlV1(invocation,"test:drain-modes",120000);
 for(const row of corpus.modes){
  const result=await captureOwnedCommandResultV1(process.execPath,["test",import.meta.path],process.cwd(),{control,env:{...process.env,SEMIO_DRAIN_MODE:row.mode},output:{maximumBytes:32768,maximumLines:256}});
  expect(result.status,row.mode+result.stdout+result.stderr).toBe(0);
  const records=result.stdout.split(/\r?\n/u).filter(line=>line.startsWith("actual-drain-mode:"));expect(records,row.mode).toHaveLength(1);
  const actual=JSON.parse(records[0]!.slice("actual-drain-mode:".length)) as Record<string,boolean>;
  expect(Object.keys(actual).sort(),row.mode).toEqual([...row.keys]);expect(Object.values(actual).every(value=>value===true),row.mode).toBe(true);
 }
}),240000);
else test("actual drain mode "+selected,async()=>{
 const row=corpus.modes.find(item=>item.mode===selected);if(!row)throw Error("Closed actual drain mode required");
 const result=await proveAcceptedOriginalDrainV1(row.mode,undefined as never);console.log("actual-drain-mode:"+JSON.stringify(result));
},90000);
