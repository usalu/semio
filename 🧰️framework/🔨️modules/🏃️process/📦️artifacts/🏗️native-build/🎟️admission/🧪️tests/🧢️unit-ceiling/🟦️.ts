/** 🧢️ Contract: no compile unit holds more than the unit ceiling of the admission policy unless it is on the closed carving list, and a carving entry that fits again leaves the list. The verdict of every corpus case (the measured final peaks of the telemetry and the edge cases) equals an independent Python oracle that reads the policy from the schema file. */
import {existsSync} from "node:fs";
import {join,resolve} from "node:path";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {ADMISSION_POLICY_V1,formatUnitCeilingV1,readUnitHistoryV1,unitCeilingAlarmV1,unitCeilingVerdictV1,unitHistoryPathV1,unitKeyPartsV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

const policy=ADMISSION_POLICY_V1.unitCeiling,oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url)),schemaFile=fileURLToPath(new URL("../../🧬️schema/🔣️.json",import.meta.url)),corpusFile=fileURLToPath(new URL("./🔣️.json",import.meta.url));

test("the policy and the verdict are closed by the schema and the corpus by its own",()=>{
 const ajv=new Ajv({strict:false}).addSchema(schema);
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,extra:1})).toBe(false);
 const definition=ajv.getSchema(`${schema.$id}#/definitions/UnitCeilingVerdictV1`)!;
 for(const item of corpus.cases)expect(definition(unitCeilingVerdictV1(policy,item.observations)),item.id+JSON.stringify(definition.errors)).toBe(true);
 expect(definition({...unitCeilingVerdictV1(policy,[]),extra:1})).toBe(false);
 expect(policy.megabytes).toBe(4096);
 expect(policy.hostMegabytes).toBe(16384);
 expect(policy.carving.map(entry=>entry.crate)).toEqual(["semio_hub_cad_component","semio_hub_puzzle_component","semio_s_artifact_stdio_dwg_snapshot_objects"]);
});

test("the verdict of every corpus case equals the independent Python oracle",()=>{
 const run=spawnSync(requireTestToolV1("python"),["-I",oracle,schemaFile,corpusFile],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 expect(JSON.parse(run.stdout)).toEqual(corpus.cases.map(item=>unitCeilingVerdictV1(policy,item.observations)));
});

test("the measured final peaks exceed the ceiling only at cad and puzzle, both listed; dag stays within it",()=>{
 const verdict=unitCeilingVerdictV1(policy,corpus.cases.find(item=>item.id==="measured-telemetry")!.observations);
 expect(verdict.exceeded.map(unit=>[unit.crate,unit.peakMegabytes,unit.listed])).toEqual([["semio_hub_cad_component",6550,true],["semio_hub_puzzle_component",6148,true]]);
 expect(verdict.resolved).toEqual([]);
 expect(verdict.ok).toBe(true);
 expect(formatUnitCeilingV1("en",verdict)).toEqual([]);
});

test("the edge cases follow the prose of the law",()=>{
 const by=(id:string)=>unitCeilingVerdictV1(policy,corpus.cases.find(item=>item.id===id)!.observations);
 expect(by("empty-ledger")).toEqual({version:1,ceilingMegabytes:4096,exceeded:[],resolved:[],ok:true});
 expect(by("exactly-at-ceiling").exceeded).toEqual([]);
 expect(by("one-mib-above-unlisted").exceeded.map(unit=>[unit.peakMegabytes,unit.listed])).toEqual([[4097,false]]);
 expect(by("one-byte-above-rounds-up").exceeded.map(unit=>unit.peakMegabytes)).toEqual([4097]);
 expect(by("listed-and-resolved").resolved.map(entry=>[entry.crate,entry.peakMegabytes])).toEqual([["semio_hub_cad_component",4000]]);
 expect(by("listed-and-resolved").ok).toBe(false);
 expect(by("listed-but-unobserved")).toMatchObject({resolved:[],ok:true});
 expect(by("two-profiles-one-crate").exceeded.map(unit=>unit.peakMegabytes)).toEqual([5000]);
 expect(by("malformed-keys-ignored").exceeded).toEqual([]);
 expect(by("unlisted-exceeder-not-ok")).toMatchObject({ok:false});
 expect(by("same-crate-other-target-is-unlisted").exceeded[0]?.listed).toBe(false);
 expect(by("ordering-ties-by-key").exceeded.map(unit=>unit.crate)).toEqual(["semio_c_component","semio_a_component","semio_b_component"]);
 expect(unitKeyPartsV1("semio-hub-cad-component@semio_hub_cad_component|wasm32-wasip2|release|cdylib")).toEqual({crate:"semio_hub_cad_component",target:"wasm32-wasip2"});
 expect(unitKeyPartsV1("a@b|c")).toBeNull();
});

test("the lines name each unlisted exceeder and each stale entry in English and German, and a completed unit raises one bilingual alarm",()=>{
 const unlisted=unitCeilingVerdictV1(policy,corpus.cases.find(item=>item.id==="unlisted-exceeder-not-ok")!.observations),stale=unitCeilingVerdictV1(policy,corpus.cases.find(item=>item.id==="listed-and-resolved")!.observations);
 expect(formatUnitCeilingV1("en",unlisted)).toEqual(["unit semio-other-component@semio_other_component|wasm32-wasip2|release|cdylib peaked at 5000 MB, above the unit ceiling of 4096 MB (16 GB class); it is not on the carving list: reduce it or declare it"]);
 expect(formatUnitCeilingV1("de",unlisted)).toHaveLength(1);
 expect(formatUnitCeilingV1("en",stale)).toEqual(["carving entry semio_hub_cad_component now peaks at 4000 MB, at or below the ceiling of 4096 MB: remove it from the carving list"]);
 const row={key:"p@semio_other_component|wasm32-wasip2|release|cdylib",peakRssBytes:5000*1048576};
 const alarm=unitCeilingAlarmV1(policy,row)!;
 expect(alarm.split(" / ")).toHaveLength(2);
 expect(alarm).toContain("peaked at 5000 MB");
 expect(alarm).toContain("erreichte 5000 MB");
 expect(unitCeilingAlarmV1(policy,{...row,peakRssBytes:4096*1048576})).toBeNull();
 expect(unitCeilingAlarmV1(policy,{key:"p@semio_hub_cad_component|wasm32-wasip2|release|cdylib",peakRssBytes:6550*1048576})).toBeNull();
});

test("Law: no unit of this machine's real ledger exceeds the unit ceiling without being on the carving list",async()=>{
 const path=unitHistoryPathV1(join(resolve(fileURLToPath(new URL("../../../../../../../../",import.meta.url))),".cache","b","r","m","components"));
 if(!existsSync(path)){console.log(`[unit-ceiling] no ledger at ${path}: skipped`);return;}
 const verdict=unitCeilingVerdictV1(policy,[...(await readUnitHistoryV1(path)).values()].flat());
 console.log(`[unit-ceiling] ${path}: ${JSON.stringify(verdict)}`);
 expect(verdict.exceeded.filter(unit=>!unit.listed),formatUnitCeilingV1("en",verdict).join("\n")).toEqual([]);
});
