import{test}from"bun:test";
import{writeFileSync}from"node:fs";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1}from"../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{checkResourceLeaseControl}from"../../../../../🔒️leases/🟦️.ts";
const selected=process.env.SEMIO_BUN_FILE_CASE;
if(selected!=="fixture-output"&&selected!=="fixture-cohort")test.skip("owned Bun sibling requires a closed actual parent test selection",()=>{});
else test("owned Bun sibling actual process birth",async()=>receiveScriptProcessInvocation(process.env,async original=>{
 const control=createIssuedResourceControlV1(original,"test:bun-file-sibling",0),marker=process.env.SEMIO_BUN_FILE_MARKER;if(!marker)throw Error("Actual file cohort marker required");checkResourceLeaseControl(control);writeFileSync(marker+"-sibling",String(process.pid));
 if(selected==="fixture-cohort"){}else await Bun.sleep(30000);checkResourceLeaseControl(control);
}),60000);
