/** 🪓️ Contract: a unit the warden cancelled above its runtime ceiling teaches the forecast the peak it reached, a unit that failed, was cached or was killed from outside does not. The observation of every corpus case equals the independent Python oracle and the recorded peak is the one the unit reached.
 * @see ../../🏔️ceiling/🟦️.ts */
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {ADMISSION_POLICY_V1,localUnitObservationV1,unitCeilingCancelledV1,unitObservationV1,type UnitRecordV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

const policy=ADMISSION_POLICY_V1.unitCeiling,oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url)),schemaFile=fileURLToPath(new URL("../../🧬️schema/🔣️.json",import.meta.url)),corpusFile=fileURLToPath(new URL("./🔣️.json",import.meta.url));
const record=(item:(typeof corpus.cases)[number]):UnitRecordV1=>({key:item.key,exit:item.exit,peakRssBytes:item.peakRssBytes,startMs:1000,endMs:61000,userMs:50000,sysMs:2000,lines:1200,localInputKey:"input",artifact:"/out/unit",...item.cache?{cache:{key:"k",recorded:true}}:{}}) as unknown as UnitRecordV1;

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,extra:1})).toBe(false);
});

test("the observation of every corpus case equals the independent Python oracle",()=>{
 const run=spawnSync(requireTestToolV1("python"),["-I",oracle,schemaFile,corpusFile],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const expected=JSON.parse(run.stdout) as boolean[];
 expect(expected).toEqual(corpus.cases.map(item=>item.recorded));
 expect(corpus.cases.map(item=>unitObservationV1(record(item),60000,1)!==null),"the scheduler record").toEqual(expected);
 expect(corpus.cases.map(item=>localUnitObservationV1(record(item),()=>1)!==null),"the local record").toEqual(expected);
});

test("a cancelled unit is recorded with the peak it reached, above its ceiling, so the forecast sizes the next attempt above it",()=>{
 const item=corpus.cases[0]!,row=unitObservationV1(record(item),60000,1)!;
 expect(row.key).toBe(item.key);
 expect(row.peakRssBytes).toBe(item.peakRssBytes);
 expect(Math.ceil(row.peakRssBytes/1048576)).toBeGreaterThan((policy.classes as Readonly<Record<string,{finalMegabytes:number}>>)[policy.workerClass]!.finalMegabytes);
 expect(unitCeilingCancelledV1(policy,record(item))).toBe(true);
 expect(unitCeilingCancelledV1(policy,{...record(item),peakRssBytes:0})).toBe(false);
 expect(unitCeilingCancelledV1(policy,{...record(item),key:"malformed"})).toBe(false);
 expect(schema.definitions.Policy.const.unitCeiling.classes["16g"].finalMegabytes).toBe(policy.classes["16g"].finalMegabytes);
});
