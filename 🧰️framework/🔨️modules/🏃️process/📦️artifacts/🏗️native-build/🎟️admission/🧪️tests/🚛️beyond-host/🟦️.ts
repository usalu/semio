/** 🚛️ Contract: a unit known to need more than the final budget of the worker class is beyond every machine of that class, so it queues for a bigger one instead of compiling there.
 * @see ../../🏔️ceiling/🟦️.ts */
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {expect,test} from "bun:test";
import {requireTestToolV1} from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {ADMISSION_POLICY_V1,unitBeyondHostV1} from "../../🟦️.ts";
import vectors from "./🔣️.json";

const policy=ADMISSION_POLICY_V1.unitCeiling;

test("the vectors follow the policy numbers and every case answers as written",()=>{
 expect(vectors.workerClass).toBe(policy.workerClass);
 expect(vectors.classHostMegabytes).toBe(policy.classes["16g"].hostMegabytes);
 expect(vectors.finalMegabytes).toBe(policy.classes["16g"].finalMegabytes);
 for(const row of vectors.cases)expect(unitBeyondHostV1(policy,{crate:row.crate,target:row.target},row.learned??undefined,row.host),row.name).toBe(row.beyond);
 expect(()=>unitBeyondHostV1({...policy,workerClass:"nope"},{crate:"a",target:"b"},undefined,1)).toThrow("Unknown worker class");
});

test("every case equals the independent Python oracle",()=>{
 const result=spawnSync(requireTestToolV1("python"),["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("../../🧬️schema/🔣️.json",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"});
 expect(result.status,result.stderr).toBe(0);
 const model=JSON.parse(result.stdout) as boolean[];
 expect(model).toEqual(vectors.cases.map(row=>row.beyond));
 expect(vectors.cases.map(row=>unitBeyondHostV1(policy,{crate:row.crate,target:row.target},row.learned??undefined,row.host))).toEqual(model);
});
