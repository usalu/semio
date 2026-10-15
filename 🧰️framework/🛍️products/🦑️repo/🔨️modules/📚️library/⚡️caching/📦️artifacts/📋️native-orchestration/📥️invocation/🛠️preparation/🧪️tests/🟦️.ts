import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {cargoPreparationControlV1} from "../🟦️.ts";
import type {NativeCallerPhaseV1} from "../../🟦️.ts";
import policy from "../🔣️policy.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
test("preparation checks and yields every step while Original progress follows finite cadence",async()=>{
 expect(new Ajv({strict:true}).validate(schema,policy)).toBe(true);let time=0,published=0,yielded=0,checked=0,failed=false;const controller=new AbortController(),phase={authority:{} as NativeCallerPhaseV1["authority"],leaseControl:{signal:controller.signal,remainingMilliseconds:()=>10000,publish:async()=>{},yieldContinuation:async()=>{yielded++;}},budget:()=>{checked++;if(failed)throw Error("Original refusal");return 10000;},advance:async()=>{published++;yielded++;},options:()=>({signal:controller.signal,output:{maximumBytes:1024,maximumLines:16}})} satisfies NativeCallerPhaseV1,control=cargoPreparationControlV1(phase,()=>time),database=new Database(":memory:");
 try{for(let index=0;index<fixture.steps;index++){time=index*fixture.stepMilliseconds;await control.advance({operation:fixture.selectedOperation as "list",path:"owner",completedOperations:index+1});}const expected=database.query("WITH RECURSIVE steps(n) AS (SELECT 0 UNION ALL SELECT n+1 FROM steps WHERE n+1<?) SELECT count(DISTINCT CAST(n*?/ ? AS INTEGER)) AS total FROM steps").get(fixture.steps,fixture.stepMilliseconds,policy.progressMilliseconds) as {total:number};expect(published).toBe(fixture.publications);expect(published).toBe(expected.total);expect(yielded).toBe(fixture.steps);expect(checked).toBeGreaterThanOrEqual(fixture.steps*2);failed=true;await expect(control.advance({operation:"list",path:"owner",completedOperations:fixture.refusalOperation})).rejects.toThrow("Original refusal");expect(control.signal).toBe(controller.signal);console.log("[DEBUG] Actual preparation provider reduced 16384 operations to 17 awaited publications; every step yielded and retained Original checks, independently matching SQLite cadence");}finally{database.close();}
});
