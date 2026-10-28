/** 🧪 Validate owned browser provisioning lifecycle with portable cases and opaque tool spies. */
import assert from "node:assert/strict";
import fs from "node:fs/promises";
import {launchBrowser} from "../🧰️tools.mjs";

const fixture=JSON.parse(await fs.readFile(new URL("./🔣️.cases.json",import.meta.url),"utf8"));
for(const sample of fixture.cases){
  const calls=[],controller=new AbortController(),healthy={id:"healthy"};
  const errors={missing:new Error(fixture.missingMessage),unrelated:new Error("browserType.launch: browser crashed"),unrelatedMissingText:new Error("File loader: Executable doesn't exist at /unrelated"),provision:new Error("Provision failed"),retry:new Error("Retry failed")};
  let index=0;
  if(sample.abort==="before")controller.abort();
  let result,failure;
  try{
    result=await launchBrowser({signal:controller.signal,launch:async()=>{calls.push("launch");const step=sample.launch[index++];if(step==="healthy")return healthy;throw errors[step]},provision:async()=>{calls.push("provision");if(sample.abort==="provision")controller.abort();if(sample.provision==="fail")throw errors.provision}});
  }catch(error){failure=error}
  assert.deepEqual(calls,sample.calls,sample.id);
  if(sample.result)assert.equal(result,healthy,sample.id);
  else if(sample.error)assert.equal(failure,errors[sample.error],sample.id);
  else assert.equal(failure?.name,sample.errorName,sample.id);
  console.log("PASS "+sample.id);
}
