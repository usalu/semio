/** 🧢️ Contract: no compile unit holds more than the compile-memory budget of its kind (library or final link) in the active worker class of the admission policy unless it is on the closed carving list, and a carving entry that fits again leaves the list. Only fresh ledger rows count (they record the source size and match the latest one). The verdicts and the fresh rows of every corpus case equal an independent Python oracle that reads the policy from the schema file. */
import {existsSync} from "node:fs";
import {join,resolve} from "node:path";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {ADMISSION_POLICY_V1,coldBudgetWarningV1,compileUnitOfCommandV1,runtimeCeilingOfCommandV1,UNIT_CEILING_POLICY_V1,formatUnitCeilingV1,freshUnitObservationsV1,predictPeakBytesV1,readUnitHistoryV1,unitBudgetKindV1,unitBudgetMegabytesV1,unitCeilingAlarmV1,unitCeilingVerdictV1,unitHistoryPathV1,unitKeyPartsV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";
import frontendThreads from "../🧵️frontend-threads/🔣️.json";

const policy=ADMISSION_POLICY_V1.unitCeiling,oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url)),schemaFile=fileURLToPath(new URL("../../🧬️schema/🔣️.json",import.meta.url)),corpusFile=fileURLToPath(new URL("./🔣️.json",import.meta.url));

test("the policy and the verdict are closed by the schema and the corpus by its own",()=>{
 const ajv=new Ajv({strict:false}).addSchema(schema);
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,extra:1})).toBe(false);
 const definition=ajv.getSchema(`${schema.$id}#/definitions/UnitCeilingVerdictV1`)!;
 for(const item of corpus.cases)expect(definition(unitCeilingVerdictV1(policy,item.observations)),item.id+JSON.stringify(definition.errors)).toBe(true);
 expect(definition({...unitCeilingVerdictV1(policy,[]),extra:1})).toBe(false);
 expect(policy.workerClass).toBe("16g");
 expect(policy.classes["16g"]).toEqual({hostMegabytes:16384,libraryMegabytes:1536,finalMegabytes:4096,finalBackend:"serial"});
 expect(policy.classes["16g"].libraryMegabytes).toBeLessThan(policy.classes["16g"].finalMegabytes);
 expect(new Set(policy.carving.map(entry=>`${entry.crate}|${entry.target}`)).size).toBe(policy.carving.length);
 for(const entry of policy.carving)expect(entry.measuredMegabytes).toBeGreaterThan(policy.classes["16g"].libraryMegabytes);
 for(const entry of policy.carving){
  expect(entry.budgetMegabytes,`${entry.crate}|${entry.target} declares a runtime budget of at least its measured peak`).toBeGreaterThanOrEqual(entry.measuredMegabytes);
  expect(entry.budgetMegabytes,`${entry.crate}|${entry.target} declares at most 10 percent headroom`).toBeLessThanOrEqual(Math.ceil(entry.measuredMegabytes*1.1));
  expect(entry.budgetMegabytes,`${entry.crate}|${entry.target} stays below the cap share of the physical memory`).toBeLessThan(Math.floor(policy.classes["16g"].hostMegabytes*6000/10000));
 }
 expect(policy.carving.find(entry=>entry.crate==="semio_hub_cad_component")).toMatchObject({measuredMegabytes:8036,budgetMegabytes:8500});
 expect(policy.carving.filter(entry=>/^semio_(hub_sourcing_component|s_plugin_sourcing_(beams|slabs|windows)_component)$/u.test(entry.crate)),"the sourcing closure finals are carved under the final budget").toEqual([]);
 expect(policy.carving.find(entry=>entry.crate==="semio_hub_sourcing")).toMatchObject({measuredMegabytes:2824,budgetMegabytes:3100});
 for(const [crate,measured,budget] of [["semio_hub_stdio",8953,9450],["semio_hub_stdio_component",9306,9820],["semio_hub_stdio_office_component",4231,4470],["semio_hub_stdio_office",7939,8400],["semio_hub_space_component",5055,5300],["semio_hub_process_component",8944,9800],["semio_s_artifact_stdio_pdf_model",3525,3720],["semio_s_artifact_bim_model_snapshot",3016,3190],["semio_s_artifact_fem_2d_model",1549,1640]] as const)expect(policy.carving.find(entry=>entry.crate===crate&&entry.target==="wasm32-wasip2"),crate).toMatchObject({measuredMegabytes:measured,budgetMegabytes:budget});
 for(const entry of policy.carving)expect(entry.budgetMegabytes,`${entry.crate} fits one heavy job on a 16 GB host: hub aggregate and component units (semio_hub_*, built only under the single-slot resource guard) up to 10000 MB, everything else up to 9000 MB`).toBeLessThanOrEqual(entry.crate.startsWith("semio_hub_")?10000:9000);
 for(const family of policy.families){
  expect(family.budgetMegabytes,`${family.family} budgets at least its class measurement`).toBeGreaterThanOrEqual(family.measuredMegabytes);
  expect(family.budgetMegabytes,`${family.family} declares at most 10 percent over its class measurement`).toBeLessThanOrEqual(Math.ceil(family.measuredMegabytes*1.1));
  expect(family.budgetMegabytes,`${family.family} stays below the cap share of the physical memory`).toBeLessThan(Math.floor(policy.classes["16g"].hostMegabytes*6000/10000));
  expect(family.measuredMegabytes).toBeGreaterThan(policy.classes["16g"].libraryMegabytes);
  expect(family.source.length).toBeGreaterThan(0);
  expect(()=>new RegExp(family.pattern,"u")).not.toThrow();
 }
 expect(new Set(policy.families.map(family=>family.family)).size).toBe(policy.families.length);
 for(const entry of policy.carving)expect(policy.families.some(family=>family.target===entry.target&&new RegExp(`^(?:${family.pattern})$`,"u").test(entry.crate)),`${entry.crate} is listed once: a family member has no entry of its own`).toBe(false);
 expect(UNIT_CEILING_POLICY_V1).toBe(policy);
 expect(()=>unitCeilingVerdictV1({...policy,workerClass:"nope"},[])).toThrow("Unknown worker class");
});

test("the verdicts and the fresh rows of every corpus case equal the independent Python oracle",()=>{
 const run=spawnSync(requireTestToolV1("python"),["-I",oracle,schemaFile,corpusFile],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const expected=JSON.parse(run.stdout);
 expect(expected.verdicts).toEqual(corpus.cases.map(item=>unitCeilingVerdictV1(policy,item.observations)));
 const fresh=(rows:{key:string;atMs:number;lines?:number}[])=>freshUnitObservationsV1(rows).map(row=>[row.key,row.atMs]).sort((left,right)=>left[0]!<right[0]!?-1:left[0]!>right[0]!?1:(left[1] as number)-(right[1] as number));
 expect(expected.fresh).toEqual(corpus.freshCases.map(item=>fresh(item.rows)));
});

test("the runtime ceiling of every corpus command line equals the independent Python oracle: library, final, carving budget, target and crate types as the law reads them",()=>{
 const run=spawnSync(requireTestToolV1("python"),["-I",oracle,schemaFile,corpusFile],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const expected=JSON.parse(run.stdout).ceilings;
 expect(expected).toHaveLength(corpus.commands.length);
 expect(corpus.commands.map(item=>runtimeCeilingOfCommandV1(policy,item.command))).toEqual(expected);
 const by=(id:string)=>runtimeCeilingOfCommandV1(policy,corpus.commands.find(item=>item.id===id)!.command);
 expect(by("listed-final-declared-budget")).toEqual({unit:"semio_hub_cad_component|wasm32-wasip2",kind:"final",megabytes:8500,listed:true,measuredMegabytes:8036});
 expect(by("listed-hub-sourcing-equals-form")).toMatchObject({megabytes:3100,listed:true});
 expect(by("unlisted-final-budget")).toEqual({unit:"semio_new_hub|wasm32-wasip2",kind:"final",megabytes:4096,listed:false});
 expect(by("listed-crate-other-target-is-unlisted")).toMatchObject({megabytes:1536,listed:false});
 expect(by("probe-without-a-crate-name")).toBeNull();
 expect(by("listed-hub-aggregate-library-declared-budget"),"a wasm composition lib is a library by its crate type; it is carved, not reclassified").toEqual({unit:"semio_hub_stdio|wasm32-wasip2",kind:"library",megabytes:9450,listed:true,measuredMegabytes:8953});
 expect(by("listed-hub-office-aggregate-rlib-declared-budget")).toMatchObject({kind:"library",megabytes:8400,listed:true,measuredMegabytes:7939});
 expect(by("listed-hub-space-component-final-declared-budget")).toEqual({unit:"semio_hub_space_component|wasm32-wasip2",kind:"final",megabytes:5300,listed:true,measuredMegabytes:5055});
 expect(by("listed-hub-aggregate-above-previous-budget-still-library")).toMatchObject({kind:"library",megabytes:9450,listed:true});
 expect(by("listed-hub-office-aggregate-dual-crate-type-is-final")).toMatchObject({kind:"final",megabytes:8400,listed:true});
 expect(by("listed-hub-stdio-component-final-declared-budget")).toEqual({unit:"semio_hub_stdio_component|wasm32-wasip2",kind:"final",megabytes:9820,listed:true,measuredMegabytes:9306});
 expect(by("listed-hub-stdio-office-component-final-declared-budget")).toEqual({unit:"semio_hub_stdio_office_component|wasm32-wasip2",kind:"final",megabytes:4470,listed:true,measuredMegabytes:4231});
 expect(by("listed-fem-2d-model-just-above-library-ceiling")).toEqual({unit:"semio_s_artifact_fem_2d_model|wasm32-wasip2",kind:"library",megabytes:1640,listed:true,measuredMegabytes:1549});
 expect(by("listed-ifc-catalog-entities-shard-00"),"a shard without an entry of its own gets the budget of its family, not the library ceiling").toEqual({unit:"semio_s_artifact_stdio_ifc4x3_add2_catalog_entities_00|wasm32-wasip2",kind:"library",megabytes:3470,listed:true,measuredMegabytes:3282});
 expect(by("unmatched-ifc-name-stays-unlisted"),"a name that is no shard of the family matches no pattern").toEqual({unit:"semio_s_artifact_stdio_ifc4x3_add2_catalog_entities_extra|wasm32-wasip2",kind:"library",megabytes:1536,listed:false});
 expect(by("family-wrong-target-is-unlisted")).toMatchObject({megabytes:1536,listed:false});
 expect(by("listed-ifc2x3-family-variance")).toEqual({unit:"semio_s_artifact_stdio_ifc2x3_catalog_entities_00|wasm32-wasip2",kind:"library",megabytes:3630,listed:true,measuredMegabytes:3440});
 expect(by("listed-pdf-model-drifted-budget")).toMatchObject({megabytes:3720,measuredMegabytes:3525});
 expect(by("listed-bim-model-snapshot-library-declared-budget")).toMatchObject({kind:"library",megabytes:3190,measuredMegabytes:3016});
 for(const id of ["unlisted-test-executable-host","unlisted-bin-host","unlisted-staticlib-wasm"])expect(by(id),id).toMatchObject({kind:"final",megabytes:4096,listed:false});
 expect(compileUnitOfCommandV1("rustc --crate-name a --crate-type rlib --crate-type cdylib")).toEqual({crate:"a",target:"host",types:["rlib","cdylib"]});
 for(const item of corpus.commands){const ceiling=runtimeCeilingOfCommandV1(policy,item.command);if(ceiling)expect(ceiling.megabytes,item.id).toBeGreaterThanOrEqual(ceiling.kind==="final"?policy.classes["16g"].finalMegabytes:policy.classes["16g"].libraryMegabytes);}
 expect(()=>runtimeCeilingOfCommandV1({...policy,workerClass:"nope"},"rustc --crate-name a")).toThrow("Unknown worker class");
});

test("the final allowance is tied to the serial back end: every granted cdylib final of the scheduler corpus appends it",()=>{
 expect(policy.classes[policy.workerClass as keyof typeof policy.classes].finalBackend).toBe("serial");
 const finals=frontendThreads.cases.filter(item=>item.args.some((arg:string)=>arg==="cdylib"||arg==="--crate-type=cdylib")&&typeof item.tokens==="number"&&item.tokens>=1);
 expect(finals.length).toBeGreaterThanOrEqual(4);
 for(const item of finals)expect(item.expected.join(" "),item.id).toContain("-Z no-parallel-backend");
});

test("the budget kind follows the crate types and a library may not use the final allowance",()=>{
 expect(unitBudgetKindV1("a@b|wasm32-wasip2|prototype|cdylib")).toBe("final");
 expect(unitBudgetKindV1("a@b|wasm32-wasip2|prototype|cdylib,rlib")).toBe("final");
 expect(unitBudgetKindV1("a@b|wasm32-wasip2|prototype|lib")).toBe("library");
 expect(unitBudgetKindV1("a@b|host|prototype|proc-macro")).toBe("library");
 expect(unitBudgetMegabytesV1(policy,"a@b|wasm32-wasip2|prototype|lib")).toBe(1536);
 expect(unitBudgetMegabytesV1(policy,"a@b|wasm32-wasip2|prototype|cdylib")).toBe(4096);
 const by=(id:string)=>unitCeilingVerdictV1(policy,corpus.cases.find(item=>item.id===id)!.observations);
 expect(by("library-exactly-at-budget").exceeded).toEqual([]);
 expect(by("library-one-mib-above-unlisted").exceeded.map(unit=>[unit.kind,unit.peakMegabytes,unit.budgetMegabytes,unit.listed])).toEqual([["library",1537,1536,false]]);
 expect(by("library-one-byte-above-rounds-up").exceeded.map(unit=>unit.peakMegabytes)).toEqual([1537]);
 expect(by("library-in-final-budget-is-exceeded").exceeded.map(unit=>[unit.kind,unit.peakMegabytes])).toEqual([["library",3000]]);
 expect(by("final-between-budgets-fits")).toMatchObject({exceeded:[],ok:true});
 expect(by("mixed-kinds-one-crate-listed-library-fits")).toMatchObject({exceeded:[],ok:false,resolved:[{crate:"semio_hub_cad_component",peakMegabytes:3000,budgetMegabytes:4096}]});
 expect(by("listed-library-exceeds-and-final-fits")).toMatchObject({resolved:[],ok:true});
 expect(by("listed-library-exceeds-and-final-fits").exceeded.map(unit=>[unit.kind,unit.listed])).toEqual([["library",true]]);
 expect(by("host-proc-macro-is-a-library").exceeded.map(unit=>unit.peakMegabytes)).toEqual([1600]);
 expect(by("listed-dwg-library-fits-again").resolved.map(entry=>entry.crate)).toEqual(["semio_s_artifact_stdio_dwg_snapshot_objects"]);
});

test("only fresh rows count: rows without the source size and rows of another source size are dropped",()=>{
 const by=(id:string)=>freshUnitObservationsV1(corpus.freshCases.find(item=>item.id===id)!.rows).map(row=>row.atMs).sort((left,right)=>left-right);
 expect(by("fresh-empty")).toEqual([]);
 expect(by("fresh-rows-without-lines-are-stale")).toEqual([2]);
 expect(by("fresh-only-without-lines-leaves-nothing")).toEqual([]);
 expect(by("fresh-split-source-drops-old-peaks")).toEqual([2,3]);
 expect(by("fresh-tolerance-boundary-is-inclusive")).toEqual([1,2,3]);
 expect(by("fresh-just-outside-tolerance")).toEqual([2]);
 expect(by("fresh-zero-lines-matches-only-zero")).toEqual([2,3]);
 expect(by("fresh-keys-are-independent")).toEqual([2,3,4]);
 expect(by("fresh-latest-wins-over-file-order")).toEqual([9]);
});

test("a unit without fresh history raises one bilingual cold warning above its budget and never one for a listed crate or a quiet estimate",()=>{
 const key="p@semio_cold|wasm32-wasip2|prototype|lib",cold=predictPeakBytesV1([],{kloc:30,component:null});
 expect(cold).toBeGreaterThan(1536*1048576);
 const warning=coldBudgetWarningV1(policy,key,cold)!;
 expect(warning.split(" / ")).toHaveLength(2);
 expect(warning).toContain("cold estimate is 1815 MB");
 expect(warning).toContain("Kaltschätzung beträgt 1815 MB");
 expect(coldBudgetWarningV1(policy,key,1536*1048576)).toBeNull();
 expect(coldBudgetWarningV1(policy,"p@semio_cold|wasm32-wasip2|prototype|cdylib",cold)).toBeNull();
 expect(coldBudgetWarningV1(policy,"p@semio_hub_cad_component|wasm32-wasip2|prototype|lib",cold)).toBeNull();
 expect(coldBudgetWarningV1(policy,"malformed",cold)).toBeNull();
 expect(predictPeakBytesV1([],{kloc:4,component:null})).toBeLessThan(1536*1048576);
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
 expect(by("empty-ledger")).toEqual({version:1,workerClass:"16g",libraryMegabytes:1536,finalMegabytes:4096,exceeded:[],resolved:[],ok:true});
 expect(by("exactly-at-ceiling").exceeded).toEqual([]);
 expect(by("one-mib-above-unlisted").exceeded.map(unit=>[unit.peakMegabytes,unit.listed])).toEqual([[4097,false]]);
 expect(by("one-byte-above-rounds-up").exceeded.map(unit=>unit.peakMegabytes)).toEqual([4097]);
 expect(by("listed-and-resolved").resolved.map(entry=>[entry.crate,entry.peakMegabytes,entry.budgetMegabytes])).toEqual([["semio_hub_cad_component",4000,4096]]);
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
 expect(formatUnitCeilingV1("en",unlisted)).toEqual(["unit semio-other-component@semio_other_component|wasm32-wasip2|release|cdylib peaked at 5000 MB, above the final link budget of 4096 MB (16g worker class); it is not on the carving list: carve it along the domain taxonomy or declare it"]);
 expect(formatUnitCeilingV1("de",unlisted)).toHaveLength(1);
 expect(formatUnitCeilingV1("en",stale)).toEqual(["carving entry semio_hub_cad_component now peaks at 4000 MB, within its budget of 4096 MB: remove it from the carving list"]);
 const row={key:"p@semio_other_component|wasm32-wasip2|release|cdylib",peakRssBytes:5000*1048576};
 const alarm=unitCeilingAlarmV1(policy,row)!;
 expect(alarm.split(" / ")).toHaveLength(2);
 expect(alarm).toContain("peaked at 5000 MB");
 expect(alarm).toContain("erreichte 5000 MB");
 expect(unitCeilingAlarmV1(policy,{...row,peakRssBytes:4096*1048576})).toBeNull();
 expect(unitCeilingAlarmV1(policy,{key:"p@semio_hub_cad_component|wasm32-wasip2|release|cdylib",peakRssBytes:6550*1048576})).toBeNull();
});

test("Law: no unit of this machine's real ledger exceeds its compile-memory budget without being on the carving list",async()=>{
 const path=unitHistoryPathV1(join(resolve(fileURLToPath(new URL("../../../../../../../../",import.meta.url))),".cache","b","r","m","components"));
 if(!existsSync(path)){console.log(`[unit-ceiling] no ledger at ${path}: skipped`);return;}
 const verdict=unitCeilingVerdictV1(policy,freshUnitObservationsV1([...(await readUnitHistoryV1(path)).values()].flat()));
 console.log(`[unit-ceiling] ${path}: ${JSON.stringify(verdict)}`);
 expect(verdict.exceeded.filter(unit=>!unit.listed),formatUnitCeilingV1("en",verdict).join("\n")).toEqual([]);
});
