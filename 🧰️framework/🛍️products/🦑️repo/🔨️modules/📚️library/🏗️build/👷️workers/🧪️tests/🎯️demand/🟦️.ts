import {expect,test} from "bun:test";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {planWorkersV1,readWorkerPolicyV1} from "../../🟦️.ts";

const policy=readWorkerPolicyV1(buildPolicy.workers),host={cores:10,memoryBytes:Math.ceil(1559/0.3)*1048576},counts=(plan:ReturnType<typeof planWorkersV1>)=>Object.fromEntries(Object.entries(plan.kinds).map(([kind,row])=>[kind,row.workers?row.maxInstances:0]));

/** 🎯️ Worker instances follow the planned actions: an activation without tests or checks gets none for them, and scarce instances go to the busiest kinds. Target `👷️workers/🧪️tests/🎯️demand/🟦️.ts`. */
test("scarce worker instances are derived from the kinds the invocation plans",()=>{
 expect(planWorkersV1(policy,host).memory.instances).toBe(3);
 expect(counts(planWorkersV1(policy,host,{jobs:2,demand:{build:30,script:12,generate:1}}))).toEqual({test:0,check:0,generate:1,build:1,script:1,service:0});
 expect(counts(planWorkersV1(policy,host,{jobs:2,demand:{build:30,script:12}}))).toEqual({test:0,check:0,generate:0,build:2,script:1,service:0});
 expect(counts(planWorkersV1(policy,host,{jobs:2,demand:{build:1,script:40,check:7,test:2}}))).toEqual({test:1,check:1,generate:0,build:0,script:1,service:0});
 expect(counts(planWorkersV1(policy,host,{jobs:2,demand:{}}))).toEqual({test:0,check:0,generate:0,build:0,script:0,service:0});
 expect(counts(planWorkersV1(policy,host,{jobs:2}))).toEqual({test:1,check:1,generate:1,build:0,script:0,service:0});
});
