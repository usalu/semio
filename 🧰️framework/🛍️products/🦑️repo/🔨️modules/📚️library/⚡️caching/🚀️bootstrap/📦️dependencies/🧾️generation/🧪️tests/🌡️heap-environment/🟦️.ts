/** 🌡️ Contract: an environment variable the repository itself sets to size a process (the engine heap and garbage collector of the admission grant) never seals a dependency generation: the receipt of a generation must not go stale because two invocations were granted different memory, since that makes every invocation rebuild the dependencies another one just built. Every such variable the heap policy declares is ignored by the Bun producer fingerprint, and the fingerprint of a process with another grant equals the one of a process without. */
import {expect,test} from "bun:test";
import bun from "../../../🔌️managers/🍞️bun/🔣️.json";
import pool from "../../../../../../🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🧬️schema/🔣️.json";
import {observeJavascriptDependencyGenerationProducerV1} from "../../💾️storage/🟦️.ts";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";

const policy=(pool as {$defs:{Policy:{const:{runtime:{bun:{heap:{variable:string};environment:Record<string,string>}}}}}}).$defs.Policy.const.runtime.bun;
const sizing=[policy.heap.variable,...Object.keys(policy.environment)];

test("every variable of the heap policy for Bun is ignored by the producer fingerprint, in the case the environment spells it",()=>{
 const ignored=new Set(bun.producerInputs.environmentIgnored);
 expect(sizing.length).toBeGreaterThanOrEqual(3);
 for(const name of sizing)expect(ignored.has(name.toUpperCase()),name).toBe(true);
 for(const name of bun.producerInputs.environmentIgnored)expect(name,name).toBe(name.toUpperCase());
});

test("two invocations granted different memory seal the same producer fingerprint; a variable that selects the registry still does",()=>{
 const root=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"heap-env-")));
 try{
  const ambient=(env:Record<string,string>)=>Object.freeze({env,home:join(root,"home"),config:join(root,"config")});
  const observe=(env:Record<string,string>)=>JSON.stringify(observeJavascriptDependencyGenerationProducerV1(root,join(root,"store"),["."],ambient(env)).environment);
  const none=observe({}),small=observe({[policy.heap.variable]:String(2*1024**3),...policy.environment}),large=observe({[policy.heap.variable]:String(12*1024**3),...policy.environment});
  expect(small).toBe(none);expect(large).toBe(none);
  expect(observe({NPM_CONFIG_REGISTRY:"https://example.invalid/"})).not.toBe(none);
  expect(observe({BUN_CONFIG_REGISTRY:"https://example.invalid/"})).not.toBe(none);
 }finally{rmSync(root,{recursive:true,force:true});}
});
