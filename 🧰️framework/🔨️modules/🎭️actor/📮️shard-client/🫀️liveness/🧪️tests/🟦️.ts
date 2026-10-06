import {expect,test} from "bun:test";
import Ajv from "ajv";
import {createRequire} from "node:module";
import schema from "../🧬️schema/🔣️.json";
import {SHARD_LIVENESS_POLICY} from "../🟦️.ts";

test("canonical shard liveness policy agrees with two independent validators",()=>{
 const validate=new Ajv({strict:true}).compile(schema),oracle=createRequire(import.meta.url)("jsonschema");
 for(const value of [SHARD_LIVENESS_POLICY,{...SHARD_LIVENESS_POLICY,heartbeatTimeoutMs:0},{...SHARD_LIVENESS_POLICY,missedLimit:1.5},{...SHARD_LIVENESS_POLICY,extra:true}])expect(validate(value)).toBe(oracle.validate(value,schema).valid);
 expect(validate(SHARD_LIVENESS_POLICY)).toBe(true);
 expect(SHARD_LIVENESS_POLICY.progressIntervalMs).toBeLessThan(SHARD_LIVENESS_POLICY.heartbeatTimeoutMs);
 expect(SHARD_LIVENESS_POLICY.pluginLoadIdleTimeoutMs).toBeLessThan(SHARD_LIVENESS_POLICY.pluginLoadCeilingMs);
 console.log("[DEBUG] canonical shard watchdog policy owns six timing values; two independent validators agree");
});
