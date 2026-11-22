/** 🛰️ Fleet probe conformance: the scenario document and the outcome follow one JSON schema (checked with Ajv), the laws of the warm second machine, the twin build and the simulated 16 GB host equal an independent Python model on language-agnostic fixtures, and every scenario runs on this one machine with a second workspace and cache directory and yields exactly its declared metrics. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {fleetDiskLawsV1,fleetFirstTaskV1,fleetHostV1,fleetReadyV1,fleetMemoryLawsV1,fleetOutcomeV1,fleetScenarioIssuesV1,fleetScenariosV1,fleetSchemaV1,fleetTwinV1,fleetWarmV1} from "../🟦️.ts";
import {runFleetScenarioChildV1} from "../🏃️runner/🟦️.ts";
import slo from "../../🎯️slo/🔣️.json" with {type:"json"};

const oracle=join(import.meta.dir,"🐍️oracle/🐍️.py"),ajv=new Ajv({strict:false,allErrors:true}),schema=fleetSchemaV1 as unknown as {definitions:Record<string,unknown>};
const check=(name:string)=>ajv.compile({$ref:`#/definitions/${name}`,definitions:schema.definitions} as never);
const python=(op:string,cases:readonly unknown[]):any[]=>{const run=spawnSync(process.env.SEMIO_TEST_PYTHON_BINARY??"python3",["-I",oracle],{input:JSON.stringify({op,cases}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};

test("the scenario document matches its schema, every probe of the objective table names a declared scenario and every metric is declared",()=>{
 expect(check("Scenarios")(fleetScenariosV1)).toBe(true);
 expect(fleetScenarioIssuesV1()).toEqual([]);
 expect(fleetScenarioIssuesV1({version:1,scenarios:[fleetScenariosV1.scenarios[0],fleetScenariosV1.scenarios[0]]})).toEqual(["duplicate scenario unit-cache-warm"]);
 const probes=(slo as any).probes.filter((row:any)=>row.kind==="fleet"),ids=fleetScenariosV1.scenarios.map(row=>row.id).sort();
 expect(probes.map((row:any)=>row.scenario).sort()).toEqual(ids);
 for(const row of (slo as any).slos){const probe=(slo as any).probes.find((p:any)=>p.id===row.probe);if(probe.kind==="fleet")expect(fleetScenariosV1.scenarios.find(s=>s.id===probe.scenario)!.metrics,row.id).toContain(row.metric);}
});

test("warm second machine, twin build, memory, disk and host laws equal the Python model",()=>{
 for(const row of fixture.warm){expect(fleetWarmV1(row)).toEqual(row.expected);}
 expect(python("warm",fixture.warm)).toEqual(fixture.warm.map(row=>row.expected));
 expect(python("warm",fixture.badWarm)).toEqual(fixture.badWarm.map(()=>null));
 for(const row of fixture.badWarm)expect(()=>fleetWarmV1(row)).toThrow(/Zahl|count/u);
 for(const row of fixture.twin)expect(fleetTwinV1(row as never)).toEqual(row.expected);
 expect(python("twin",fixture.twin)).toEqual(fixture.twin.map(row=>row.expected));
 expect(()=>fleetTwinV1({outputs:[],answers:[]})).toThrow(/outputs/u);
 for(const row of fixture.memory){expect(fleetMemoryLawsV1(row as never),row.name).toEqual(row.expected);}
 expect(python("memory",fixture.memory)).toEqual(fixture.memory.map(row=>row.expected));
 for(const row of fixture.disk){expect(fleetDiskLawsV1(row),row.name).toEqual(row.expected);}
 expect(python("disk",fixture.disk)).toEqual(fixture.disk.map(row=>row.expected));
 for(const row of fixture.host){expect(fleetHostV1(row as never)).toEqual(row.expected);}
 expect(python("host",fixture.host)).toEqual(fixture.host.map(row=>row.expected));
});

test("an outcome carries exactly the metrics of its scenario and validates against the closed schema",()=>{
 const valid=check("Outcome");
 for(const row of fixture.outcomes){
  if(row.valid){const done=fleetOutcomeV1(row.scenario,row.values);expect(valid(done)).toBe(true);}
  else expect(()=>fleetOutcomeV1(row.scenario,row.values),row.name).toThrow();
 }
 expect(valid({version:1,scenario:"x",values:{bogusMs:1}})).toBe(false);
});

const scratchRun=async(id:string)=>runFleetScenarioChildV1(id,new AbortController().signal);

test("on one machine a second workspace finds every unit and every Nx task the first one published and nothing else",async()=>{
 for(const id of ["unit-cache-warm","nx-cache-warm"])expect((await scratchRun(id)).values,id).toEqual({hitRatio:1,executedActions:0});
},120000);

test("one unit built in two directories writes identical bytes and no divergence",async()=>{
 expect((await scratchRun("twin-build")).values).toEqual({divergentWrites:0});
},120000);

test("the simulated 16 GB host keeps every capacity law and its disk images within the free bytes",async()=>{
 const {values}=await scratchRun("host-16g");
 expect(values.capacityBreaches).toBe(0);expect(values.reservedBytes!).toBeLessThanOrEqual(16*2**30*0.75);expect(values.storeBytes!).toBeLessThanOrEqual(56*2**30*0.9);
},120000);

test("under forced local pressure a unit completes on another worker, and with the scheduler down it compiles locally in time",async()=>{
 const moved=(await scratchRun("offload-pressure")).values;
 expect(moved.offloadedUnits).toBe(1);expect(moved.offloadMs!).toBeLessThan(60000);
 expect((await scratchRun("offload-fallback")).values.fallbackMs!).toBeLessThan(60000);
},300000);

test("an unknown scenario and a cancelled run are refused",async()=>{
 await expect(scratchRun("nope")).rejects.toThrow(/Unbekanntes|Unknown/u);
 const controller=new AbortController();controller.abort(Error("stop"));
 await expect(runFleetScenarioChildV1("unit-cache-warm",controller.signal)).rejects.toThrow("stop");
 const left=mkdtempSync(join(tmpdir(),"fleet-probe-check-"));rmSync(left,{recursive:true,force:true});
});

test("production fleet probe code never imports from a tests folder, and the live cache objectives are blocking while the live offload ones stay advisory",()=>{
 for(const file of ["../🟦️.ts","../🏃️runner/🟦️.ts","../🧰️harness/🟦️.ts"]){
  const imports=readFileSync(join(import.meta.dir,file),"utf8").split("\n").filter(line=>/^\s*(import|export)\b.*from\s|await import\(/u.test(line)).join("\n");
  expect(imports,file).not.toContain("🧪️tests");
 }
 const live=(slo as any).slos.filter((row:any)=>row.id.startsWith("fleet-live-"));
 expect(live.length).toBeGreaterThanOrEqual(10);
 for(const row of live)expect(row.enforcement,row.id).toBe(/^fleet-live-(unit|nx|bazel)-/u.test(row.id)?"blocking":"advisory");
 expect(fleetScenariosV1.scenarios.filter(row=>row.id.startsWith("live-")).map(row=>row.id).sort()).toEqual(["live-bazel-cache","live-nx-cache","live-offload","live-run-warm","live-unit-cache"]);
});

test("fleet readiness and the time to the first task equal the Python model, an empty fleet and a taskless run are refused",()=>{
 for(const row of fixture.ready){expect(fleetReadyV1(row.receipts,row.nowMs,row.maximumAgeMs),row.name).toEqual(row.expected);}
 expect(python("ready",fixture.ready)).toEqual(fixture.ready.map(row=>row.expected));
 expect(python("ready",fixture.badReady)).toEqual(fixture.badReady.map(()=>null));
 for(const row of fixture.badReady)expect(()=>fleetReadyV1(row.receipts,row.nowMs,row.maximumAgeMs)).toThrow(/receipt|Beleg/u);
 for(const row of fixture.firstTask){expect(fleetFirstTaskV1(row.lines,row.wallMs),row.name).toEqual(row.expected);}
 expect(python("firsttask",fixture.firstTask)).toEqual(fixture.firstTask.map(row=>row.expected));
 for(const row of fixture.badFirstTask)expect(()=>fleetFirstTaskV1(row.lines,row.wallMs)).toThrow(/task|Task/u);
 expect(python("firsttask",fixture.badFirstTask)).toEqual([null]);
});
