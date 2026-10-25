import {test,expect} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {createScriptProcessEnvelope,scriptInvocationEnvironment,withScriptProcessEnvelope,SCRIPT_PROCESS_INVOCATION_ENV} from "../../🟦️.ts";
import envelopeSchema from "../../🧬️schema/🔣️.json";
import policySchema from "../../../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
for(const row of corpus.cases)test("Original integer wire deadline "+row.owner,async()=>{
 const oracle=spawnSync("python3",["-c","import sys\nvalue=float(sys.argv[1])\nprint('finite-integer' if value.is_integer() and value>=1 else 'refused',end='')",String(row.maximumElapsedMilliseconds)],{encoding:"utf8",timeout:5000});
 expect(oracle.status).toBe(0);expect(oracle.stdout).toBe(row.expected);
 const policy={version:1 as const,owner:row.owner,maximumElapsedMilliseconds:row.maximumElapsedMilliseconds};
 if(row.expected==="refused"){expect(()=>createScriptProcessEnvelope(policy,{token:row.owner},Date.now())).toThrow();return;}
 const before=Date.now(),envelope=createScriptProcessEnvelope(policy,{token:row.owner},before);
 expect(Number.isSafeInteger(envelope.deadlineEpochMilliseconds)).toBe(true);expect(envelope.deadlineEpochMilliseconds).toBe(before+row.maximumElapsedMilliseconds);
 await withScriptProcessEnvelope(envelope,async original=>{
  const remaining=original.control.remainingMilliseconds(),wire=JSON.parse(scriptInvocationEnvironment(original,{})[SCRIPT_PROCESS_INVOCATION_ENV]!);
  expect(Number.isInteger(remaining)).toBe(true);expect(remaining).toBeGreaterThan(0);expect(remaining).toBeLessThanOrEqual(row.maximumElapsedMilliseconds);
  expect(new Ajv({strict:true}).addSchema(policySchema).compile(envelopeSchema)(wire)).toBe(true);
  expect(Number.isSafeInteger(wire.deadlineEpochMilliseconds)).toBe(true);expect(wire.deadlineEpochMilliseconds).toBe(envelope.deadlineEpochMilliseconds);expect(wire.policy).toEqual(policy);expect(wire.capabilities).toEqual({token:row.owner});
 });
},15000);
