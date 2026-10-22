/** 🍳️ Performance SLO and bench conformance: statistics, verdicts, series, baseline comparison, selection, edits, tool markers and process-tree memory are checked against language-agnostic fixtures, an independent Python oracle (standard library `statistics`, NumPy as the third-party reference) and the repository validator next to Ajv; the probes run against scripted ports with stub measurements, the spawner and the whole command against a stand-in CLI as real processes; the CI job definition is parsed. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {copyFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {homedir,tmpdir} from "node:os";
import {dirname,join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import table from "../🎯️slo/🔣️.json" with {type:"json"};
import catalog from "../../../⚙️operations/🔣️catalog.json" with {type:"json"};
import {benchBaselineV1,benchCompareV1,benchHostV1,benchJudgeV1,benchLoadClassV1,benchMarkersV1,benchMutateV1,benchReportV1,benchSelectV1,benchSeriesV1,benchSloIssuesV1,benchStatsV1,benchTallyV1,renderBenchComparisonV1,renderBenchListV1,renderBenchReportV1,type BenchProbeV1,type BenchReportV1,type BenchRowV1,type BenchSampleV1,type BenchSeriesV1,type BenchSloSetV1,type BenchSloV1,type BenchToolV1} from "../🟦️.ts";
import {benchCliPortsV1,benchConcurrencyFromReportV1,benchRunArgumentsV1,benchRunProbeV1,benchRunV1,benchSpawnV1,benchTreeRssV1,type BenchHostPortsV1,type BenchPortsV1,type BenchRunV1,type BenchSpawnSpecV1} from "../🏃️runner/🟦️.ts";
import {benchArgumentsV1,benchOptionNamesV1,runBenchCommandV1,type BenchCommandPortsV1} from "../⌨️command/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../..//🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

const set=table as unknown as BenchSloSetV1,policy=schema.definitions.Policy.const,plain=(value:unknown):any=>JSON.parse(JSON.stringify(value));
const ajv=new Ajv({strict:true,allowUnionTypes:true}),define=(name:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${name}`});
const oracle=join(import.meta.dir,"🐍️oracle/🐍️.py"),schemaPath=join(import.meta.dir,"../🧬️schema/🔣️.json"),stubPath=join(import.meta.dir,"🎭️stub/🟦️.ts");
const python=(op:string,cases:readonly unknown[]):any[]|null=>{const run=spawnSync("python3",["-I",oracle],{input:JSON.stringify({op,schemaPath,cases}),encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const differences=(actual:any,expected:any,path="$"):string[]=>{
 if(typeof actual==="number"&&typeof expected==="number")return Math.abs(actual-expected)<=1e-9*Math.max(1,Math.abs(actual),Math.abs(expected))?[]:[`${path}: ${actual} != ${expected}`];
 if(Array.isArray(actual)&&Array.isArray(expected))return actual.length!==expected.length?[`${path}: length ${actual.length} != ${expected.length}`]:actual.flatMap((value,at)=>differences(value,expected[at],`${path}[${at}]`));
 if(actual&&expected&&typeof actual==="object"&&typeof expected==="object"){const keys=new Set([...Object.keys(actual),...Object.keys(expected)]);return [...keys].flatMap(key=>differences(actual[key],expected[key],`${path}.${key}`));}
 return Object.is(actual,expected)?[]:[`${path}: ${JSON.stringify(actual)} != ${JSON.stringify(expected)}`];
};
const same=(actual:unknown,expected:unknown)=>expect(differences(plain(actual),plain(expected))).toEqual([]);
const text=(value:{en:string;de:string})=>value;
const rowOf=(slo:BenchSloV1,series:BenchSeriesV1|undefined,tool:"nx"|"bazel"|"neutral"="nx")=>benchJudgeV1(slo,tool,series);

test("the schema is valid and validates the fixture, the SLO table and the result shapes the way the repository validator does",()=>{
 expect(ajv.validateSchema(schema)).toBe(true);
 expect(define("Fixture")(fixture)).toBe(true);
 expect(define("SloSet")(table)).toBe(true);
 expect(validateJsonSchemaSubset(schema.definitions.SloSet,table,schema)).toEqual([]);
 const title=text({en:"a",de:"b"}),values={wallMs:1},sample={index:0,ok:true,exit:0,values,startKind:null,loadPerCore:0.1},stats={n:1,min:1,p50:1,p95:1,p99:1,max:1,mean:1};
 const slo={id:"a",title,probe:"p",metric:"wallMs",statistic:"p50",direction:"lower",goal:1,gate:2,tools:["nx"],enforcement:"advisory",tolerance:{relative:0,absolute:0},source:"user-goal"};
 for(const[name,value,ok]of [
  ["Sample",sample,true],["Sample",{...sample,values:{bogus:1}},false],["Sample",{...sample,values:{wallMs:-1}},false],["Sample",{...sample,startKind:"nope"},false],["Sample",{...sample,loadPerCore:null,exit:null},true],["Sample",{...sample,index:-1},false],
  ["Stats",stats,true],["Stats",{...stats,n:0},false],["Stats",{...stats,p50:"1"},false],
  ["Slo",slo,true],["Slo",{...slo,statistic:"p90"},false],["Slo",{...slo,goal:null,gate:null},true],["Slo",{...slo,tools:[]},false],["Slo",{...slo,tools:["cargo"]},false],["Slo",{...slo,enforcement:"maybe"},false],["Slo",{...slo,source:"hunch"},false],["Slo",{...slo,tolerance:{relative:-1,absolute:0}},false],["Slo",{...slo,extra:1},false],
  ["Probe",{id:"p",kind:"noop",title,runs:3,warmup:0},true],["Probe",{id:"p",kind:"noop",title,runs:0,warmup:0},false],["Probe",{id:"p",kind:"noop",title,runs:3,warmup:21},false],["Probe",{id:"P",kind:"noop",title,runs:3,warmup:0},false],["Probe",{id:"p",kind:"other",title,runs:3,warmup:0},false],["Probe",{id:"p",kind:"concurrency",title,runs:1,warmup:0,sessions:9},false],
  ["EditUnit",{id:"u",language:"ts",target:"a:b",file:"x.ts",op:"private-body",title},true],["EditUnit",{id:"u",language:"cobol",target:"a:b",file:"x.ts",op:"private-body",title},false],["EditUnit",{id:"u",language:"ts",target:"nocolon",file:"x.ts",op:"private-body",title},false],["EditUnit",{id:"u",language:"ts",target:"a:b",file:"x".repeat(256),op:"private-body",title},false],
  ["Host",benchHostV1({platform:"p",arch:"a",cpus:4,memoryBytes:1,load1:1}),true],["Host",{...benchHostV1({platform:"p",arch:"a",cpus:0,memoryBytes:1,load1:1})},false],
  ["Tally",{pass:0,warn:0,fail:0,unproven:0,recorded:0,blockingFailed:0},true],["Tally",{pass:0,warn:0,fail:0,unproven:0,recorded:0},false],
  ["CompareTally",{improved:0,same:0,regressed:0,inconclusive:0,incomparable:0,new:0,missing:0,blockingRegressed:0},true],["Reason",{},false],["Reason","host-saturated",true],["Reason","bored",false]
 ]as const){
  const own=validateJsonSchemaSubset(schema.definitions[name as keyof typeof schema.definitions],value,schema).length===0;
  expect(own,`${name} ${JSON.stringify(value)}`).toBe(ok);
  expect(define(name)(value),`ajv ${name} ${JSON.stringify(value)}`).toBe(ok);
 }
});

test("the authored SLO table is admitted and every inconsistency is found",()=>{
 expect(benchSloIssuesV1(table)).toEqual([]);
 const mutate=(change:(copy:any)=>void)=>{const copy=plain(table);change(copy);return benchSloIssuesV1(copy);};
 const has=(issues:string[],part:string)=>expect(issues.some(issue=>issue.includes(part)),`${part} in ${JSON.stringify(issues)}`).toBe(true);
 has(mutate(copy=>{copy.slos.push({...copy.slos[0]});}),"duplicate objective");
 has(mutate(copy=>{copy.probes.push({...copy.probes[0]});}),"duplicate probe");
 has(mutate(copy=>{copy.units.push({...copy.units[0]});}),"duplicate unit");
 has(mutate(copy=>{copy.slos[0].probe="nope";}),"unknown probe");
 has(mutate(copy=>{copy.slos[0].metric="startMs";}),"does not yield startMs");
 has(mutate(copy=>{copy.slos.find((row:any)=>row.id==="cli-help-p99").statistic="p99";copy.probes.find((row:any)=>row.id==="cli-help").runs=50;}),"needs 100 samples");
 has(mutate(copy=>{copy.slos[0].goal=500;copy.slos[0].gate=400;}),"beyond the gate");
 has(mutate(copy=>{copy.slos[0].gate=null;}),"a goal needs a gate");
 has(mutate(copy=>{const row=copy.slos.find((value:any)=>value.metric==="startMs");row.direction="higher";}),"a start time is judged downwards");
 has(mutate(copy=>{copy.slos.find((row:any)=>row.enforcement==="blocking").goal=null;copy.slos.find((row:any)=>row.enforcement==="blocking").gate=null;}),"blocking objective needs a gate");
 has(mutate(copy=>{copy.slos[0].tools=["nx"];}),"neutral belongs to cli-help");
 has(mutate(copy=>{copy.slos.find((row:any)=>row.probe==="noop-run-oneshot").tools=["neutral"];}),"neutral belongs to cli-help");
 has(mutate(copy=>{delete copy.probes.find((row:any)=>row.id==="noop-run-oneshot").target;}),"target is required for noop");
 has(mutate(copy=>{copy.probes.find((row:any)=>row.id==="cli-help").target="a:b";}),"target is not allowed for cli-help");
 has(mutate(copy=>{copy.probes.find((row:any)=>row.kind==="edit").unit="ghost";}),"unknown unit");
 has(mutate(copy=>{delete copy.probes.find((row:any)=>row.kind==="concurrency").workload;}),"workload is required");
 has(mutate(copy=>{copy.probes.find((row:any)=>row.id==="graph-scope-cold").runs=2;}),"a cold probe runs once");
 has(mutate(copy=>{copy.probes.find((row:any)=>row.id==="cli-help").cold=true;}),"only a graph probe can be cold");
 has(mutate(copy=>{copy.probes.find((row:any)=>row.id==="cli-help").memory=true;}),"only noop and edit probes measure memory");
 has(mutate(copy=>{copy.probes.find((row:any)=>row.id==="graph-scope-warm").lane="resident";}),"no resident lane");
 expect(mutate(copy=>{copy.units[0].file="x".repeat(300);}).length).toBeGreaterThan(0);
 expect(mutate(copy=>{delete copy.labels.pass;}).length).toBeGreaterThan(0);
 expect(mutate(copy=>{copy.version=2;}).length).toBeGreaterThan(0);
});

test("the objectives the user asked for are in the table, enforced where they are reachable and recorded where they are not",()=>{
 const by=new Map(set.slos.map(row=>[row.id,row])),probes=new Map(set.probes.map(row=>[row.id,row]));
 for(const verb of ["run","test"]){
  const resident=by.get(`noop-${verb}-resident-p95`)!;
  expect([resident.goal,resident.gate,resident.statistic,resident.metric,resident.enforcement,resident.source,probes.get(resident.probe)!.lane]).toEqual([300,300,"p95","wallMs","blocking","user-goal","resident"]);
  expect(resident.tools).toEqual(["nx","bazel"]);
  const oneshot=by.get(`noop-${verb}-oneshot-p50`)!;
  expect([oneshot.goal,oneshot.gate,oneshot.enforcement]).toEqual([300,1000,"advisory"]);
 }
 for(const unit of set.units){
  const start=by.get(`edit-${unit.id}-resident-start-p95`)!;
  expect([start.goal,start.gate,start.statistic,start.metric,start.enforcement,start.source]).toEqual([500,500,"p95","startMs","blocking","user-goal"]);
  expect(by.get(`edit-${unit.id}-oneshot-start-p50`)!.goal).toBe(500);
  for(const lane of ["oneshot","resident"]){const green=by.get(`edit-${unit.id}-${lane}-green-p50`)!;expect([green.goal,green.gate,green.metric]).toEqual([null,null,"greenMs"]);}
  expect(probes.get(`edit-${unit.id}-resident`)!.runs).toBeGreaterThanOrEqual(policy.minimumSamples.p95);
 }
 expect(set.units.map(row=>row.language).sort()).toEqual(["go","py","rs","ts"]);
 for(const verb of ["build","test"]){const hit=by.get(`cache-second-run-${verb}-min`)!;expect([hit.goal,hit.gate,hit.direction,hit.statistic,hit.metric,hit.enforcement,hit.tolerance.relative,hit.tolerance.absolute]).toEqual([1,1,"higher","min","hitRatio","blocking",0,0]);}
 expect(by.get("cli-help-p95")!.goal).toBe(200);
 expect(by.get("graph-scope-cold-max")!.gate).toBe(300000);
 expect(probes.get("graph-scope-cold")!.cold).toBe(true);
 expect(set.slos.filter(row=>row.metric==="peakRssBytes").length).toBe(2);
 for(const slo of set.slos)expect(policy.sources).toContain(slo.source);
});

test("percentiles, mean and extremes equal the oracle (statistics.quantiles) and NumPy's linear percentile",()=>{
 for(const row of fixture.percentiles)same(benchStatsV1(row.values),row.expected);
 const cases:{values:number[]}[]=[];let seed=12345;const next=()=>{seed=(seed*1103515245+12345)%2147483648;return seed/2147483648;};
 for(let at=0;at<150;at++)cases.push({values:Array.from({length:1+Math.floor(next()*140)},()=>Math.round(next()*1e6)/100)});
 const expected=python("stats",cases)!;
 cases.forEach((row,at)=>same(benchStatsV1(row.values),expected[at]));
 const numpy=python("stats-numpy",cases);
 if(numpy===null)console.log("numpy is not installed: third-party percentile check skipped");
 else cases.forEach((row,at)=>{if(row.values.length>=1)same(benchStatsV1(row.values),numpy[at]);});
 expect(benchStatsV1([])).toBeNull();
 expect(benchStatsV1([7])).toEqual({n:1,min:7,p50:7,p95:7,p99:7,max:7,mean:7});
 expect(benchStatsV1([1,2,3,4,5,6,7,8,9,10])!.p50).toBe(5.5);
});

test("the load of the host is classified per core, a host without a load is unknown",()=>{
 for(const row of fixture.loads)expect(benchLoadClassV1(row.perCore)).toBe(row.class as never);
 expect(python("load",fixture.loads.map(row=>({perCore:row.perCore})))).toEqual(fixture.loads.map(row=>row.class));
 expect(benchLoadClassV1(Number.NaN)).toBe("unknown");
 const host=benchHostV1({platform:"darwin",arch:"arm64",cpus:10,memoryBytes:100,load1:25});
 expect([host.loadPerCore,host.loadClass,host.hostClass]).toEqual([2.5,"saturated","darwin-arm64-10"]);
 expect(benchHostV1({platform:"win32",arch:"x64",cpus:8,memoryBytes:100,load1:null}).loadClass).toBe("unknown");
});

test("series built from samples equal the oracle: failed samples are counted and left out, the load class is the busiest moment",()=>{
 for(const row of fixture.series)same(benchSeriesV1(row.input as never),row.series);
 const random=(seed:number):any[]=>{let state=seed;const next=()=>{state=(state*1103515245+12345)%2147483648;return state/2147483648;};return Array.from({length:6},(_,index)=>({index,ok:next()>0.2,exit:0,values:next()>0.1?{wallMs:Math.round(next()*1e4)/10,startMs:Math.round(next()*1e3)/10}:{},startKind:["marker","first-output","exit",null][Math.floor(next()*4)],loadPerCore:next()>0.2?Math.round(next()*30)/10:null}));};
 const cases=Array.from({length:40},(_,at)=>({probe:"p",tool:"nx",lane:null,status:"measured",reason:null,samples:random(at+1)})),expected=python("series",cases)!;
 cases.forEach((row,at)=>same(benchSeriesV1(row as never),expected[at]));
});

test("an objective is judged from its series exactly as the oracle judges it, including sample minimums and saturated hosts",()=>{
 for(const row of fixture.judgements)same(benchJudgeV1(row.slo as never,row.slo.tools[0] as BenchToolV1,row.series as never),row.row);
 const byName=new Map(fixture.judgements.map(row=>[row.name,row.row]));
 expect(byName.get("fail-saturated-is-unproven")).toMatchObject({status:"unproven",reason:"host-saturated",inflated:true});
 expect(byName.get("pass-saturated-stays-pass")).toMatchObject({status:"pass",inflated:false});
 expect(byName.get("p95-19-samples")).toMatchObject({status:"unproven",reason:"too-few-samples",n:19});
 expect(byName.get("p95-20-samples")!.status).toBe("pass");
 expect(byName.get("p99-99-samples")!.status).toBe("unproven");
 expect(byName.get("p99-100-samples")!.status).toBe("pass");
 expect(byName.get("higher-warn")!.status).toBe("warn");
 expect(byName.get("recorded")!.status).toBe("recorded");
 expect([byName.get("equals-goal-passes")!.status,byName.get("equals-gate-warns")!.status,byName.get("equals-gate-only-passes")!.status]).toEqual(["pass","warn","pass"]);
 expect(["start-all-markers-pass","start-first-output-pass-is-unproven","start-first-output-fail-is-fail","start-exit-pass-is-pass","start-exit-fail-is-unproven","start-mixed-fail-is-unproven","start-first-output-recorded","start-first-output-fail-saturated"].map(name=>byName.get(name)!.status)).toEqual(["pass","unproven","fail","pass","unproven","unproven","recorded","unproven"]);
 expect(byName.get("start-mixed-pass-is-unproven")).toMatchObject({status:"unproven",reason:"start-unobserved",value:100});
 expect(byName.get("start-first-output-fail-saturated")).toMatchObject({reason:"host-saturated"});
 const slos:any[]=[],sampleSets:any[]=[];let seed=99;const next=()=>{seed=(seed*1103515245+12345)%2147483648;return seed/2147483648;};
 for(let at=0;at<120;at++){
  const statistic=policy.statistics[Math.floor(next()*policy.statistics.length)]!,lower=next()>0.3,goal=next()>0.3?Math.round(next()*500):null,gate=goal===null?(next()>0.5?Math.round(next()*900):null):goal+(lower?1:-1)*Math.round(next()*400),count=Math.floor(next()*40);
  slos.push({id:"x",title:{en:"x",de:"x"},probe:"p",metric:"wallMs",statistic,direction:lower?"lower":"higher",goal,gate,tools:["nx"],enforcement:"advisory",tolerance:{relative:0.1,absolute:5},source:"provisional"});
  sampleSets.push({probe:"p",tool:"nx",lane:null,status:"measured",reason:null,samples:Array.from({length:count},(_,index)=>({index,ok:next()>0.1,exit:0,values:{wallMs:Math.round(next()*1000)},startKind:null,loadPerCore:next()>0.5?3:0.1}))});
 }
 const built=python("series",sampleSets)!,expected=python("judge",slos.map((value,at)=>({slo:value,tool:"nx",series:built[at]})))!;
 slos.forEach((value,at)=>same(benchJudgeV1(value,"nx",built[at]),expected[at]));
});

test("the verdicts are counted and only failed blocking objectives count as blocking failures",()=>{
 for(const row of fixture.tallies)expect(benchTallyV1(row.rows as never)).toEqual(row.tally);
 const rows=fixture.judgements.map(row=>row.row);
 expect(benchTallyV1(rows as never).blockingFailed).toBe(rows.filter(row=>row.status==="fail"&&row.enforcement==="blocking").length);
 expect(benchTallyV1(rows as never).blockingFailed).toBeGreaterThan(0);
});

test("a report is compared with a baseline like the oracle does: tolerance edges, direction, host class, saturation, new and missing rows",()=>{
 for(const row of fixture.comparisons)same(benchCompareV1({version:1,labels:set.labels,units:[],probes:[],slos:row.slos} as never,row.current as never,row.baseline as never),row.comparison);
 const find=(name:string)=>fixture.comparisons.find(row=>row.name===name)!.comparison;
 expect(find("boundary-exact-is-same").rows[0]!.status).toBe("same");
 expect(find("boundary-over-regressed").rows[0]!.status).toBe("regressed");
 expect(find("boundary-lower-exact").rows[0]!.status).toBe("same");
 expect(find("boundary-lower-over-improved").rows[0]!.status).toBe("improved");
 expect(find("regressed-and-blocking").tally.blockingRegressed).toBe(1);
 expect(find("saturated-current-inconclusive").rows[0]).toMatchObject({status:"inconclusive",reason:"host-saturated"});
 expect(find("saturated-improvement-stays").rows[0]!.status).toBe("improved");
 expect(find("host-class-differs").tally.incomparable).toBe(3);
 expect(find("not-missing-when-probe-absent").tally.missing).toBe(0);
 const random=(seed:number)=>{let state=seed;const next=()=>{state=(state*1103515245+12345)%2147483648;return state/2147483648;};return next;};
 const next=random(4242),slos:BenchSloV1[]=[
  {id:"a",title:{en:"a",de:"a"},probe:"p",metric:"wallMs",statistic:"p50",direction:"lower",goal:300,gate:900,tools:["nx"],enforcement:"advisory",tolerance:{relative:0.15,absolute:10},source:"provisional"},
  {id:"h",title:{en:"h",de:"h"},probe:"c",metric:"hitRatio",statistic:"min",direction:"higher",goal:1,gate:1,tools:["nx"],enforcement:"blocking",tolerance:{relative:0.02,absolute:0},source:"user-goal"}];
 const build=(load:number)=>{
  const series=[benchSeriesV1({probe:"p",tool:"nx",lane:null,status:"measured",reason:null,samples:Array.from({length:5},(_,index)=>({index,ok:true,exit:0,values:{wallMs:Math.round(100+next()*400)},startKind:null,loadPerCore:load}))}),
   benchSeriesV1({probe:"c",tool:"nx",lane:null,status:"measured",reason:null,samples:Array.from({length:3},(_,index)=>({index,ok:true,exit:0,values:{hitRatio:next()>0.5?1:Math.round((0.8+next()*0.2)*100)/100},startKind:null,loadPerCore:0.2}))})];
  const rows=[benchJudgeV1(slos[0]!,"nx",series[0]),benchJudgeV1(slos[1]!,"nx",series[1])],host=benchHostV1({platform:"x",arch:"y",cpus:10,memoryBytes:1,load1:load*10});
  return benchReportV1({startedAt:"a",finishedAt:"b",host,hostAfter:host,tools:["nx"],runsOverride:null,series,rows,comparison:null});
 };
 const cases=Array.from({length:80},()=>({slos,current:build(next()>0.7?2:0.2),baseline:build(next()>0.7?2:0.2)})),expected=python("compare",cases.map(row=>plain(row)))!;
 cases.forEach((row,at)=>same(benchCompareV1({version:1,labels:set.labels,units:[],probes:[],slos} as never,row.current,row.baseline),expected[at]));
 expect(()=>benchBaselineV1({version:2})).toThrow(/Bench-Bericht/u);
 expect(()=>benchBaselineV1(null)).toThrow();
 expect(benchBaselineV1(plain(fixture.comparisons[0]!.baseline)).kind).toBe("bench-report");
});

test("objectives are selected by id, probe, kind or all, probes are paired with the chosen implementations and cold probes run first",()=>{
 for(const row of fixture.selection){
  if(row.error!==null)expect(()=>benchSelectV1(row.set as never,row.ids as never,row.tools as never),row.name).toThrow();
  else expect(benchSelectV1(row.set as never,row.ids as never,row.tools as never).map(pair=>`${pair.probe.id}|${pair.tool}|${pair.slos.map(slo=>slo.id).join(",")}`),row.name).toEqual(row.pairs);
 }
 expect(python("select",fixture.selection.map(row=>({set:row.set,ids:row.ids,tools:row.tools})))!.map((row:any)=>row.pairs)).toEqual(fixture.selection.map(row=>row.pairs));
 const all=benchSelectV1(set,"all",["nx","bazel"]);
 expect(all[0]!.probe.cold).toBe(true);
 expect(all.filter(pair=>pair.tool==="neutral").length).toBe(1);
 expect(all.length).toBe(set.probes.length*2-1);
 expect(benchSelectV1(set,["kind:cache"],["nx"]).map(pair=>pair.probe.id)).toEqual(["cache-second-run-build","cache-second-run-test"]);
 expect(()=>benchSelectV1(set,["kind:cache"],[])).toThrow(/Implementierungen/u);
 expect(()=>benchSelectV1(set,["nope"],["nx"])).toThrow(/Unbekanntes Ziel/u);
});

test("edits append one line per language, equal to the oracle, never change an existing line and always differ from the input",()=>{
 for(const row of fixture.mutations){
  const unit={id:"u",language:row.language,target:"a:b",file:"f",op:row.op,title:{en:"a",de:"b"}} as never;
  expect(benchMutateV1(row.text,unit,row.nonce),row.name).toBe(row.result);
  expect(row.result.startsWith(row.text)).toBe(true);
  expect(row.result).not.toBe(row.text);
 }
 expect(()=>benchMutateV1("x",{id:"u",language:"ts",target:"a:b",file:"f",op:"private-body",title:{en:"a",de:"b"}},"BAD")).toThrow();
});

test("tool marker lines are found like the oracle finds them and only at the start of a line",()=>{
 for(const row of fixture.markers){const patterns=benchMarkersV1(row.tool as never),at=row.lines.findIndex(line=>patterns.some(pattern=>pattern.test(line)));expect(at<0?null:at,row.name).toBe(row.first);}
 expect(benchMarkersV1("neutral")).toEqual([]);
});

test("the resident memory of a process tree is summed from a process table of either system and the answer equals the oracle",()=>{
 for(const row of fixture.memory)expect(benchTreeRssV1(row.table,row.pid,row.system),row.name).toBe(row.rss);
 expect(python("tree",fixture.memory.map(row=>({table:row.table,pid:row.pid,system:row.system})))).toEqual(fixture.memory.map(row=>row.rss));
 const live=spawnSync("ps",["-A","-o","pid=,ppid=,rss="],{encoding:"utf8"});
 if(live.status===0){const mine=benchTreeRssV1(live.stdout,process.pid,process.platform);expect(mine).not.toBeNull();expect(mine!).toBeGreaterThan(10*1024*1024);}
});

test("a run of a probe is built from the exact commands: the lanes differ only by the service flag",()=>{
 expect(benchRunArgumentsV1("a:b","oneshot",false)).toEqual(["run","a:b"]);
 expect(benchRunArgumentsV1("a:b","oneshot",true)).toEqual(["run","a:b",policy.oneshotFlag]);
 expect(benchRunArgumentsV1("a:b","resident",true)).toEqual(["run","a:b",policy.residentFlag]);
 expect(benchRunArgumentsV1("a:b",null,true,["--dry-run"])).toEqual(["run","a:b","--dry-run",policy.oneshotFlag]);
});

const title=text({en:"x",de:"x"});
const unit={id:"u",language:"ts" as const,target:"@x/y:test",file:"src/u.ts",op:"private-body" as const,title};
const probe=(over:Partial<BenchProbeV1>&{id:string;kind:BenchProbeV1["kind"]}):BenchProbeV1=>({title,runs:4,warmup:1,...over});
const mini=(probes:BenchProbeV1[],slos:BenchSloV1[]=[]):BenchSloSetV1=>({version:1,labels:set.labels,units:[unit],probes,slos});
const slo=(over:Partial<BenchSloV1>&{id:string;probe:string}):BenchSloV1=>({title,metric:"wallMs",statistic:"p50",direction:"lower",goal:300,gate:1000,tools:["nx"],enforcement:"advisory",tolerance:{relative:0.15,absolute:25},source:"provisional",...over});
const finished=(over:Partial<BenchRunV1>={}):BenchRunV1=>({exit:0,wallMs:100,startedMs:1000,firstOutputMs:10,markerMs:null,peakRssBytes:null,...over});
type ScriptV1=Partial<{spawn:(spec:BenchSpawnSpecV1,call:number,files:Map<string,string>)=>BenchRunV1|Error;cache:BenchPortsV1["cache"];concurrency:BenchPortsV1["concurrency"];fresh:boolean;service:boolean;load:number|null;files:Record<string,string>;cancelAtCheck:number;abortAtSpawn:number}>;
const scripted=(script:ScriptV1={})=>{
 const calls:BenchSpawnSpecV1[]=[],log:string[]=[],progress:string[]=[],files=new Map(Object.entries(script.files??{[unit.file]:"export const a=1;\n"})),controller=new AbortController();
 let sequence=0,checks=0;
 const ports:BenchPortsV1={
  read:async path=>files.get(path)!,
  write:async(path,content)=>{files.set(path,content);log.push(`write:${path}`);},
  spawn:async(spec,signal)=>{calls.push(spec);log.push(`spawn:${spec.args.join(" ")}`);if(signal.aborted)throw signal.reason;const outcome=(script.spawn??(()=>finished()))(spec,calls.length-1,files);if(script.abortAtSpawn!==undefined&&calls.length-1===script.abortAtSpawn)controller.abort(Error("stop now"));if(outcome instanceof Error)throw outcome;return outcome;},
  cache:script.cache??(async()=>({cacheable:4,executedCacheable:0,uncachedExecuted:1,total:5})),
  concurrency:script.concurrency??(async()=>({makespanMs:1000,serialMs:2500,maxUnrelatedWaitMs:0,failedRules:0})),
  service:{available:async()=>script.service??false,start:async()=>{log.push("service:start");},stop:async()=>{log.push("service:stop");}},
  host:()=>benchHostV1({platform:"test",arch:"x",cpus:10,memoryBytes:1e10,load1:script.load===undefined?2:script.load===null?null:script.load*10}),
  clock:()=>"2026-10-21T00:00:00.000Z",fresh:script.fresh??false,now:()=>1000,nonce:()=>(++sequence).toString(36).padEnd(8,"a"),
  check:()=>{checks++;if(script.cancelAtCheck&&checks>=script.cancelAtCheck){controller.abort(Error("cancelled"));throw controller.signal.reason;}},
  progress:async stage=>{progress.push(stage);},signal:controller.signal
 };
 return {ports,calls,log,progress,files,controller};
};
const runProbe=(value:BenchProbeV1,script:ScriptV1={},tool:"nx"|"bazel"|"neutral"="nx",override:number|null=null)=>{const world=scripted({...script,load:script.load===undefined?0.1:script.load});return benchRunProbeV1(mini([value]),value,tool,{override,service:script.service??false},world.ports).then(series=>({series,world}));};

test("the command line help probe discards the warm-up, measures every run and runs without an implementation",async()=>{
 const{series,world}=await runProbe(probe({id:"cli-help",kind:"cli-help",runs:4,warmup:1}),{spawn:(_,call)=>finished({wallMs:(call+1)*10})},"neutral");
 expect(world.calls.length).toBe(5);
 expect(world.calls.every(call=>call.tool==="neutral"&&call.args.join(" ")==="--help"&&!call.memory)).toBe(true);
 expect(series.samples.map(row=>row.values.wallMs)).toEqual([20,30,40,50]);
 expect(series.stats.wallMs!.p50).toBe(35);
 expect(series.failed).toBe(0);
 const broken=await runProbe(probe({id:"cli-help",kind:"cli-help",runs:3,warmup:0}),{spawn:(_,call)=>call===1?Error("boom"):call===2?finished({exit:1}):finished()},"neutral");
 expect([broken.series.failed,broken.series.stats.wallMs!.n]).toEqual([2,1]);
 expect(world.progress[0]).toBe("cli-help:neutral:1/5");
 const counted=await runProbe(probe({id:"cli-help",kind:"cli-help",runs:100,warmup:3}),{},"neutral",5);
 expect(counted.world.calls.length).toBe(8);
});

test("a warm no-op primes once, then measures; a failing prime makes the probe unproven instead of slow",async()=>{
 const first=await runProbe(probe({id:"n",kind:"noop",lane:"oneshot",target:"@x/y:build",runs:4,warmup:1}),{spawn:(_,call)=>finished({wallMs:call===0?9999:call*10})});
 expect(first.world.calls.length).toBe(6);
 expect(first.series.samples.map(row=>row.values.wallMs)).toEqual([20,30,40,50]);
 expect(first.world.log.filter(line=>line.startsWith("write:")).length).toBe(0);
 const withService=await runProbe(probe({id:"n",kind:"noop",lane:"oneshot",target:"@x/y:build",runs:1,warmup:0}),{service:true});
 expect(withService.world.calls.every(call=>call.args.includes(policy.oneshotFlag))).toBe(true);
 const bad=await runProbe(probe({id:"n",kind:"noop",lane:"oneshot",target:"@x/y:build"}),{spawn:()=>finished({exit:1})});
 expect([bad.world.calls.length,bad.series.samples.length,bad.series.failed]).toEqual([1,1,1]);
 const row=benchJudgeV1(slo({id:"s",probe:"n"}),"nx",bad.series);
 expect([row.status,row.reason]).toEqual(["unproven","probe-failed"]);
});

test("the resident lane needs the service: it is unavailable without it, started before and stopped after, also on failure and cancellation",async()=>{
 const resident=probe({id:"r",kind:"noop",lane:"resident",target:"@x/y:build",runs:2,warmup:0});
 const absent=await runProbe(resident,{service:false});
 expect([absent.series.status,absent.series.reason,absent.world.calls.length]).toEqual(["unavailable","service-unavailable",0]);
 expect(benchJudgeV1(slo({id:"s",probe:"r"}),"nx",absent.series)).toMatchObject({status:"unproven",reason:"service-unavailable"});
 const present=await runProbe(resident,{service:true});
 expect(present.world.log.filter(line=>line.startsWith("service:")||line.startsWith("spawn:"))).toEqual(["service:start","spawn:run @x/y:build --service=require","spawn:run @x/y:build --service=require","spawn:run @x/y:build --service=require","service:stop"]);
 const failing=scripted({service:true,spawn:(_,call)=>call===1?Error("tool crashed"):finished(),load:0.1});
 const series=await benchRunProbeV1(mini([resident]),resident,"nx",{override:null,service:true},failing.ports);
 expect(series.failed).toBe(1);
 expect(failing.log.at(-1)).toBe("service:stop");
 const cancelled=scripted({service:true,cancelAtCheck:3,load:0.1});
 await expect(benchRunProbeV1(mini([resident]),resident,"nx",{override:null,service:true},cancelled.ports)).rejects.toThrow("cancelled");
 expect(cancelled.log.at(-1)).toBe("service:stop");
 const aborted=scripted({service:true,load:0.1,abortAtSpawn:1});
 await expect(benchRunProbeV1(mini([resident]),resident,"nx",{override:null,service:true},aborted.ports)).rejects.toThrow("stop now");
 expect(aborted.log.at(-1)).toBe("service:stop");
});

test("an edit probe edits for every sample, times the start from the edit, restores the file byte for byte and tells how the start was seen",async()=>{
 const original="export const a=1;\n",editProbe=probe({id:"e",kind:"edit",lane:"oneshot",unit:"u",runs:3,warmup:1}),seen:string[]=[];
 const{series,world}=await runProbe(editProbe,{spawn:(_,call,files)=>{seen.push(files.get(unit.file)!);return finished({startedMs:1005,wallMs:100,firstOutputMs:20,markerMs:call%2?40:null});}});
 expect(seen[0]).toBe(original);
 expect(seen.slice(1).every(content=>content.startsWith(original)&&content.length>original.length&&/void \d+;\n$/u.test(content))).toBe(true);
 expect(new Set(seen.slice(1)).size).toBe(4);
 expect(world.files.get(unit.file)).toBe(original);
 expect(world.log.filter(line=>line==="write:src/u.ts").length).toBe(8);
 const marked=series.samples.filter(row=>row.startKind==="marker"),firstOutput=series.samples.filter(row=>row.startKind==="first-output");
 expect(marked.every(row=>row.values.startMs===45&&row.values.greenMs===105)).toBe(true);
 expect(firstOutput.every(row=>row.values.startMs===25&&row.values.greenMs===105)).toBe(true);
 expect(series.startKinds.marker+series.startKinds["first-output"]).toBe(3);
 const silent=await runProbe(editProbe,{spawn:()=>finished({startedMs:1005,wallMs:100,firstOutputMs:null,markerMs:null})});
 expect(silent.series.samples.every(row=>row.startKind==="exit"&&row.values.startMs===105)).toBe(true);
 const red=await runProbe(editProbe,{spawn:(_,call)=>call===0?finished():finished({exit:1,markerMs:30,startedMs:1002})});
 expect(red.series.samples.every(row=>!row.ok&&row.startKind==="marker"&&Object.keys(row.values).length===0)).toBe(true);
 expect(benchJudgeV1(slo({id:"s",probe:"e",metric:"startMs"}),"nx",red.series)).toMatchObject({status:"unproven",reason:"probe-failed"});
 expect(red.world.files.get(unit.file)).toBe(original);
 const broken=await runProbe(editProbe,{spawn:(_,call)=>call===0?finished({exit:1}):finished()});
 expect([broken.world.calls.length,broken.series.failed,broken.world.files.get(unit.file)]).toEqual([1,1,original]);
 await expect(runProbe(probe({id:"e",kind:"edit",lane:"oneshot",unit:"ghost"}),{})).rejects.toThrow(/Unbekannte Einheit/u);
});

test("cancelling in the middle of an edit probe restores the edited file",async()=>{
 const world=scripted({load:0.1,abortAtSpawn:3}),editProbe=probe({id:"e",kind:"edit",lane:"oneshot",unit:"u",runs:5,warmup:0});
 await expect(benchRunProbeV1(mini([editProbe]),editProbe,"nx",{override:null,service:false},world.ports)).rejects.toThrow("stop now");
 expect(world.files.get(unit.file)).toBe("export const a=1;\n");
 const checked=scripted({load:0.1,cancelAtCheck:4});
 await expect(benchRunProbeV1(mini([editProbe]),editProbe,"nx",{override:null,service:false},checked.ports)).rejects.toThrow("cancelled");
 expect(checked.files.get(unit.file)).toBe("export const a=1;\n");
});

test("the cold graph probe runs once on a freshly prepared copy and says so when the copy is not fresh",async()=>{
 const cold=probe({id:"g",kind:"graph",lane:"oneshot",target:"@x/y:test",runs:1,warmup:0,cold:true});
 const stale=await runProbe(cold,{fresh:false});
 expect([stale.series.status,stale.series.reason,stale.world.calls.length]).toEqual(["unavailable","not-cold",0]);
 const fresh=await runProbe(cold,{fresh:true},"nx",9);
 expect(fresh.world.calls.map(call=>call.args.join(" "))).toEqual(["run @x/y:test --dry-run"]);
 expect(fresh.series.samples.length).toBe(1);
 const warm=await runProbe(probe({id:"g",kind:"graph",lane:"oneshot",target:"@x/y:test",runs:2,warmup:1}),{});
 expect(warm.world.calls.map(call=>call.args.join(" "))).toEqual(Array(4).fill("run @x/y:test --dry-run"));
});

test("memory probes keep the peak of the process tree and are unsupported where no sample could be taken",async()=>{
 const memory=probe({id:"m",kind:"noop",lane:"oneshot",target:"@x/y:build",runs:3,warmup:0,memory:true});
 const measured=await runProbe(memory,{spawn:(spec,call)=>finished({peakRssBytes:spec.memory?100+call:null})});
 expect(measured.world.calls.slice(1).every(call=>call.memory)).toBe(true);
 expect(measured.series.samples.map(row=>row.values)).toEqual([{peakRssBytes:101},{peakRssBytes:102},{peakRssBytes:103}]);
 const none=await runProbe(memory,{});
 expect([none.series.status,none.series.reason]).toEqual(["unavailable","unsupported"]);
});

test("the cache probe judges the second run: cacheable tasks that executed are misses, uncached tasks are not counted",async()=>{
 const hit=probe({id:"c",kind:"cache",target:"@x/y:build",runs:3,warmup:0}),counts=[{cacheable:4,executedCacheable:0,uncachedExecuted:1,total:5},{cacheable:4,executedCacheable:1,uncachedExecuted:1,total:5},{cacheable:8,executedCacheable:0,uncachedExecuted:0,total:8},{cacheable:4,executedCacheable:2,uncachedExecuted:0,total:4}];
 const{series,world}=await runProbe(hit,{cache:async()=>counts.shift()!},"bazel");
 expect(world.calls.length).toBe(0);
 expect(series.samples.map(row=>row.values.hitRatio)).toEqual([0.75,1,0.5]);
 expect(series.stats.hitRatio!.min).toBe(0.5);
 const none=await runProbe(hit,{cache:async()=>({cacheable:0,executedCacheable:0,uncachedExecuted:0,total:2})});
 expect(none.series.failed).toBe(3);
 const crash=await runProbe(hit,{cache:async()=>{throw Error("no graph");}});
 expect([crash.series.samples.length,crash.series.failed]).toEqual([1,1]);
 const row=benchJudgeV1(slo({id:"h",probe:"c",metric:"hitRatio",statistic:"min",direction:"higher",goal:1,gate:1,enforcement:"blocking"}),"nx",series);
 expect([row.status,row.value]).toEqual(["fail",0.5]);
});

test("the concurrency probe reports makespan, the longest wait for an unrelated project and the speed-up over the serial reference",async()=>{
 const conc=probe({id:"k",kind:"concurrency",workload:"disjoint-projects",sessions:3,runs:2,warmup:0}),asked:string[]=[];
 const{series}=await runProbe(conc,{concurrency:async(tool,workload,sessions)=>{asked.push(`${tool}/${workload}/${sessions}`);return {makespanMs:1000,serialMs:2500,maxUnrelatedWaitMs:120,failedRules:2};}},"bazel");
 expect(asked).toEqual(["bazel/disjoint-projects/3","bazel/disjoint-projects/3"]);
 expect(series.samples[0]!.values).toEqual({makespanMs:1000,maxUnrelatedWaitMs:120,speedup:2.5});
 const zero=await runProbe(conc,{concurrency:async()=>({makespanMs:0,serialMs:0,maxUnrelatedWaitMs:0,failedRules:0})});
 expect(zero.series.samples[0]!.values).toEqual({makespanMs:0,maxUnrelatedWaitMs:0});
});

test("the concurrency report of the harness is condensed to makespan, serial span, longest unrelated wait and failed rules",()=>{
 const wait=(waitMs:number,relation:string)=>({class:"x",resource:"r",mode:"shared",source:"lease",precise:true,startMs:0,endMs:waitMs,waitMs,holderPid:null,position:null,relation});
 const step=(waits:unknown[])=>({index:0,kind:"run",command:[],startMs:0,endMs:1,exit:0,waits,observation:null,observed:"none",outputs:null,stdout:null});
 const report={version:1,passed:1,failed:1,skipped:0,owners:[],runs:[{workload:"w",tool:"nx",criteria:"default",rules:[{rule:"a",kind:"no-failure",owners:["x"],status:"fail",value:null,limit:null,detail:[]},{rule:"b",kind:"no-failure",owners:["x"],status:"pass",value:null,limit:null,detail:[]}],measurement:{workload:"w",tool:"nx",uncached:[],sessions:[{id:"a",group:"g1",startMs:0,endMs:1000,steps:[step([wait(700,"unrelated"),wait(4000,"related")])]},{id:"b",group:"g2",startMs:100,endMs:1500,steps:[step([wait(50,"external")])]}],serial:[{id:"a",group:"g1",startMs:0,endMs:800,steps:[]},{id:"b",group:"g2",startMs:800,endMs:1700,steps:[]}]}}]};
 expect(benchConcurrencyFromReportV1(report as never)).toEqual({makespanMs:1500,serialMs:1700,maxUnrelatedWaitMs:700,failedRules:1});
 expect(()=>benchConcurrencyFromReportV1({...report,runs:[]} as never)).toThrow(/Gleichzeitigkeitsbericht/u);
 const lonely=plain(report);lonely.runs[0].measurement.serial=null;lonely.runs[0].measurement.sessions=[];
 expect(benchConcurrencyFromReportV1(lonely)).toEqual({makespanMs:0,serialMs:0,maxUnrelatedWaitMs:0,failedRules:1});
});

const fullSet=():BenchSloSetV1=>{
 const probes=[probe({id:"help",kind:"cli-help",runs:3,warmup:0}),probe({id:"cold",kind:"graph",lane:"oneshot",target:"@x/y:test",runs:1,warmup:0,cold:true}),probe({id:"noop",kind:"noop",lane:"oneshot",target:"@x/y:build",runs:3,warmup:0}),probe({id:"res",kind:"noop",lane:"resident",target:"@x/y:build",runs:3,warmup:0}),probe({id:"hit",kind:"cache",target:"@x/y:build",runs:3,warmup:0})];
 return mini(probes,[
  slo({id:"help-p50",probe:"help",tools:["neutral"],goal:100,gate:200}),slo({id:"cold-max",probe:"cold",statistic:"max",goal:null,gate:5000}),slo({id:"noop-p50",probe:"noop",tools:["nx","bazel"],goal:100,gate:150}),
  slo({id:"res-p50",probe:"res",goal:50,gate:50,enforcement:"blocking"}),slo({id:"hit-min",probe:"hit",metric:"hitRatio",statistic:"min",direction:"higher",goal:1,gate:1,enforcement:"blocking"}),slo({id:"noop-rec",probe:"noop",goal:null,gate:null,tools:["nx"]})]);
};

test("a whole run measures the selected probes in order, judges every objective, annotates the host and compares with the baseline",async()=>{
 const behave=(spec:BenchSpawnSpecV1,call:number)=>finished({wallMs:spec.args[0]==="--help"?80:call*40+100}),world=scripted({load:0.1,fresh:true,service:false,spawn:behave}),full=fullSet(),pairs=benchSelectV1(full,"all",["nx","bazel"]);
 const report=await benchRunV1(full,pairs,{override:null,tools:["nx","bazel"],maxLoad:null,baseline:null},world.ports);
 expect(pairs.map(pair=>`${pair.probe.id}/${pair.tool}`)).toEqual(["cold/nx","help/neutral","noop/bazel","noop/nx","res/nx","hit/nx"]);
 expect(world.progress.filter(line=>/^[a-z]+:[a-z]+:\d+\/6$/u.test(line)).slice(0,2)).toEqual(["cold:nx:1/6","help:neutral:2/6"]);
 expect(report.rows.map(row=>`${row.slo}/${row.tool}`)).toEqual(["help-p50/neutral","cold-max/nx","noop-p50/bazel","noop-p50/nx","res-p50/nx","hit-min/nx","noop-rec/nx"]);
 const rows=new Map(report.rows.map(row=>[`${row.slo}/${row.tool}`,row]));
 expect(rows.get("help-p50/neutral")).toMatchObject({status:"pass",value:80,n:3});
 expect(rows.get("cold-max/nx")).toMatchObject({status:"pass",value:100,n:1});
 expect(rows.get("res-p50/nx")).toMatchObject({status:"unproven",reason:"service-unavailable"});
 expect(rows.get("hit-min/nx")).toMatchObject({status:"pass",value:1});
 expect(rows.get("noop-rec/nx")!.status).toBe("recorded");
 expect(report.host.hostClass).toBe("test-x-10");
 expect([report.tally.blockingFailed,report.tally.recorded]).toEqual([0,1]);
 expect(report.tally.unproven).toBe(1);
 expect(report.kind).toBe("bench-report");
 expect(define("Report")(plain(report)),JSON.stringify(define("Report").errors)).toBe(true);
 const second=await benchRunV1(full,pairs,{override:null,tools:["nx","bazel"],maxLoad:null,baseline:report},scripted({load:0.1,fresh:true,spawn:behave}).ports);
 expect(second.comparison).not.toBeNull();
 expect(second.comparison!.rows.map(row=>row.status).sort()).toEqual(["incomparable","same","same","same","same","same","same"]);
 const slower=await benchRunV1(full,pairs,{override:null,tools:["nx","bazel"],maxLoad:null,baseline:report},scripted({load:0.1,fresh:true,spawn:(spec,call)=>finished({wallMs:spec.args[0]==="--help"?800:call*40+900})}).ports);
 expect(slower.comparison!.tally.regressed).toBeGreaterThan(0);
 expect(slower.comparison!.rows.find(row=>row.slo==="help-p50")).toMatchObject({status:"regressed",base:80,current:800});
});

test("a blocking objective that fails is counted, advisory failures are not, and a run refuses a busy host",async()=>{
 const full=mini([probe({id:"hit",kind:"cache",target:"@x/y:build",runs:3,warmup:0}),probe({id:"noop",kind:"noop",lane:"oneshot",target:"@x/y:build",runs:3,warmup:0})],[
  slo({id:"hit-min",probe:"hit",metric:"hitRatio",statistic:"min",direction:"higher",goal:1,gate:1,enforcement:"blocking"}),slo({id:"noop-p50",probe:"noop",goal:100,gate:200})]);
 const world=scripted({load:0.1,cache:async()=>({cacheable:10,executedCacheable:1,uncachedExecuted:0,total:10}),spawn:()=>finished({wallMs:5000})});
 const report=await benchRunV1(full,benchSelectV1(full,"all",["nx"]),{override:null,tools:["nx"],maxLoad:null,baseline:null},world.ports);
 expect([report.tally.fail,report.tally.blockingFailed]).toEqual([2,1]);
 const busy=scripted({load:2.5});
 await expect(benchRunV1(full,benchSelectV1(full,"all",["nx"]),{override:null,tools:["nx"],maxLoad:2,baseline:null},busy.ports)).rejects.toThrow(/Last je Kern 2\.50/u);
 expect(busy.calls.length).toBe(0);
 const unknown=scripted({load:null});
 const report2=await benchRunV1(full,benchSelectV1(full,"all",["nx"]),{override:null,tools:["nx"],maxLoad:2,baseline:null},unknown.ports);
 expect(report2.host.loadClass).toBe("unknown");
 const stopped=scripted({load:0.1,cancelAtCheck:2});
 await expect(benchRunV1(full,benchSelectV1(full,"all",["nx"]),{override:null,tools:["nx"],maxLoad:null,baseline:null},stopped.ports)).rejects.toThrow("cancelled");
});

test("reports render in both languages as text and markdown without a missing label, and the markdown table is well formed",()=>{
 const sample=fixture.comparisons.find(row=>row.name==="regressed-and-blocking")!,report=plain(sample.current) as BenchReportV1,comparison=benchCompareV1({version:1,labels:set.labels,units:[],probes:[],slos:sample.slos} as never,report,plain(sample.baseline));
 const withComparison={...report,comparison} as BenchReportV1,misses=new Set<string>();
 const probing=new Proxy(set.labels,{get(target,key){if(typeof key==="string"&&!(key in target))misses.add(key);return (target as any)[key];}});
 const view:BenchSloSetV1={...set,labels:probing as never,slos:sample.slos as never};
 const outputs:Record<string,string>={};
 for(const language of ["en","de"] as const)for(const format of ["text","markdown"] as const){
  const rendered=renderBenchReportV1(withComparison,view,language,format);
  outputs[`${language}-${format}`]=rendered;
  expect(rendered).toContain(language==="en"?"Performance objectives":"Leistungsziele");
  expect(rendered).toContain(language==="en"?"Comparison with the baseline":"Vergleich mit der Referenz");
  for(const row of sample.slos)expect(rendered).toContain(row.id);
  expect(renderBenchComparisonV1(comparison,view,language,format).length).toBeGreaterThan(50);
  expect(renderBenchListV1(set,language,format)).toContain("noop-run-resident-p95");
 }
 expect([...misses]).toEqual([]);
 expect(outputs["en-text"]).not.toBe(outputs["de-text"]);
 const tables:string[][]=[];let current:string[]=[];
 for(const line of outputs["en-markdown"]!.split("\n")){if(line.startsWith("|"))current.push(line);else if(current.length){tables.push(current);current=[];}}
 if(current.length)tables.push(current);
 expect(tables.length).toBe(2);
 for(const block of tables){const pipes=(line:string)=>line.replaceAll("\\|","").split("|").length;expect(block.length).toBeGreaterThan(3);expect(/^\| :?-+/u.test(block[1]!)).toBe(true);for(const line of block)expect(pipes(line)).toBe(pipes(block[0]!));}
 expect(outputs["en-text"]).toContain("!");
 expect(renderBenchListV1(set,"en","text")).toContain("user-goal");
});

test("the option parser accepts every documented form, refuses every other and matches the operations catalogue",()=>{
 const ok=(args:string[])=>benchArgumentsV1(args);
 expect(ok(["list"]).operation).toBe("list");
 expect(ok(["run"])).toMatchObject({slo:"all",tools:["nx","bazel"],json:false,prepare:false,runs:undefined});
 expect(ok(["run","--slo=a,kind:edit","--runs=25","--baseline","x.json","--save-baseline=y.json","--markdown=z.md","--max-load=1.5","--implementations=nx","--root=/r","--file=f","--json","--prepare","--lang=de"])).toMatchObject({slo:["a","kind:edit"],runs:25,baseline:"x.json",saveBaseline:"y.json",markdown:"z.md",maxLoad:1.5,tools:["nx"],root:"/r",file:"f",json:true,prepare:true,language:"de"});
 expect(ok(["run","--slo=all,a"]).slo).toBe("all");
 expect(ok(["compare","--current=a","--baseline=b","--markdown=m"])).toMatchObject({operation:"compare",current:"a",baseline:"b"});
 for(const bad of [[],["frobnicate"],["run","extra"],["run","--runs=0"],["run","--runs=1001"],["run","--runs=x"],["run","--runs=1.5"],["run","--max-load=0"],["run","--max-load=-1"],["run","--max-load=abc"],["run","--slo"],["run","--slo","--json"],["run","--slo=a","--slo=b"],["run","--json=1"],["run","--implementations=cargo"],["run","--lang=fr"],["run","--nope"],["compare"],["compare","--current=a"],["compare","--baseline=b"],["run","--current=a"],["list","--prepare"],["list","--runs=3"],["compare","--current=a","--baseline=b","--save-baseline=c"],["compare","--current=a","--baseline=b","--max-load=1"]])expect(()=>ok(bad),JSON.stringify(bad)).toThrow(/Erwartet|Unbekannt|braucht|Wiederholte|gehört|gehören|ganze Zahl|positive Zahl/u);
 const entry=(catalog as any).operations.find((row:any)=>row.name==="bench"),names=benchOptionNamesV1(),flags=(catalog as any).flags;
 expect(entry).toBeDefined();
 expect(entry.requires).toBe("none");
 expect([...entry.flags].sort()).toEqual([...names.valued,...names.plain].filter(name=>name!=="--lang").sort());
 for(const name of names.valued.filter(value=>value!=="--lang")){const flag=flags.find((row:any)=>row.name===name);expect(flag.value,name).toBe("required");expect(flag.operations).toContain("bench");}
 for(const name of names.plain){const flag=flags.find((row:any)=>row.name===name);expect(flag.value,name).toBe("none");expect(flag.operations).toContain("bench");}
 expect(entry.subcommands.map((row:any)=>row.name)).toEqual(["list","run","compare"]);
 for(const sub of entry.subcommands){expect(sub.synopsis.startsWith("bench ")).toBe(true);for(const flag of sub.flags??[])expect(entry.flags).toContain(flag);expect(sub.summary.en).not.toBe(sub.summary.de);}
 expect(entry.summary.en).not.toBe(entry.summary.de);
 expect(entry.implementations).toBeUndefined();
 expect([entry.progress,entry.cancellable,entry.dryRun]).toEqual([true,true,false]);
});

const makeRoot=(prefix:string)=>{
 const base=mkdtempSync(join(tmpdir(),prefix)),script="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts",catalogPath="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔣️catalog.json";
 mkdirSync(dirname(join(base,script)),{recursive:true});copyFileSync(stubPath,join(base,script));
 mkdirSync(dirname(join(base,catalogPath)),{recursive:true});writeFileSync(join(base,catalogPath),JSON.stringify({operations:[{name:"run"}]}));
 mkdirSync(join(base,"src"),{recursive:true});writeFileSync(join(base,"src","u.ts"),"export const a=1;\n");
 return {base,script,catalogPath};
};
const hostPorts=(root:string,script:string,environment:Record<string,string|undefined>,signal:AbortSignal,over:Partial<BenchHostPortsV1>={}):BenchHostPortsV1=>({live:"/nonexistent-live",root,script,executable:process.execPath,scratch:root,signal,environment,run:async()=>"",nxRecord:()=>"",adapter:async()=>{throw Error("no adapter");},inventory:async()=>[],now:()=>performance.now(),check:()=>{},progress:async()=>{},fresh:false,catalog:"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔣️catalog.json",...over});

test("the spawner measures wall time, the first output and the first tool marker of a real process on a monotonic clock",async()=>{
 const{base,script}=makeRoot("bench-spawn-");
 try{
  const ports=benchCliPortsV1(hostPorts(base,script,{...process.env,BENCH_STUB_PRELUDE_MS:"60",BENCH_STUB_START_MS:"250",BENCH_STUB_RUN_MS:"150"},new AbortController().signal),new AbortController().signal);
  const nx=await ports.spawn({tool:"nx",args:["run","a:b"],memory:false},new AbortController().signal);
  expect(nx.exit).toBe(0);
  expect(nx.firstOutputMs!).toBeGreaterThanOrEqual(55);
  expect(nx.markerMs!).toBeGreaterThanOrEqual(240);
  expect(nx.markerMs!).toBeGreaterThan(nx.firstOutputMs!);
  expect(nx.wallMs).toBeGreaterThanOrEqual(nx.markerMs!+140);
  expect(nx.startedMs).toBeGreaterThan(0);
  expect(nx.peakRssBytes).toBeNull();
  const bazel=await ports.spawn({tool:"bazel",args:["run","a:b"],memory:false},new AbortController().signal);
  expect(bazel.markerMs!).toBeGreaterThanOrEqual(240);
  const neutral=await ports.spawn({tool:"neutral",args:["run","a:b"],memory:false},new AbortController().signal);
  expect(neutral.markerMs).toBeNull();
  expect(neutral.firstOutputMs!).toBeGreaterThanOrEqual(55);
  const help=await ports.spawn({tool:"neutral",args:["--help"],memory:false},new AbortController().signal);
  expect([help.exit,help.markerMs]).toEqual([0,null]);
  const unknown=await ports.spawn({tool:"nx",args:["frobnicate"],memory:false},new AbortController().signal);
  expect(unknown.exit).toBe(2);
 }finally{rmSync(base,{recursive:true,force:true});}
},60000);

test("the spawner samples the peak memory of the whole process tree and terminates the tree when cancelled",async()=>{
 const{base,script}=makeRoot("bench-mem-");
 try{
  const env={...process.env,BENCH_STUB_ALLOC_MB:"200",BENCH_STUB_RUN_MS:"900"},ports=benchCliPortsV1(hostPorts(base,script,env,new AbortController().signal),new AbortController().signal);
  const ps=spawnSync("ps",["-A","-o","pid=,ppid=,rss="],{encoding:"utf8"});
  const measured=await ports.spawn({tool:"nx",args:["run","a:b"],memory:true},new AbortController().signal);
  if(ps.status===0){expect(measured.peakRssBytes!).toBeGreaterThan(150*1024*1024);expect(measured.peakRssBytes!).toBeLessThan(4*1024*1024*1024);}
  else expect(measured.peakRssBytes).toBeNull();
  const tiny=await ports.spawn({tool:"nx",args:["run","a:b"],memory:false},new AbortController().signal);
  expect(tiny.peakRssBytes).toBeNull();
  const slow=benchCliPortsV1(hostPorts(base,script,{...process.env,BENCH_STUB_RUN_MS:"30000"},new AbortController().signal),new AbortController().signal),controller=new AbortController(),started=performance.now();
  const pending=slow.spawn({tool:"nx",args:["run","a:b"],memory:false},controller.signal);
  setTimeout(()=>controller.abort(Error("enough")),400);
  await expect(pending).rejects.toThrow("enough");
  expect(performance.now()-started).toBeLessThan(15000);
  await expect(slow.spawn({tool:"nx",args:["run","a:b"],memory:false},controller.signal)).rejects.toThrow("enough");
 }finally{rmSync(base,{recursive:true,force:true});}
},90000);

test("real ports run an edit probe against a stand-in CLI: the edit is seen by the process, a red build is a failed sample, the file is restored",async()=>{
 const{base,script}=makeRoot("bench-edit-"),file=join(base,"src","u.ts"),original=readFileSync(file,"utf8");
 try{
  const env={...process.env,BENCH_STUB_WATCH:file,BENCH_STUB_FAIL_TOKEN:"@@never@@",BENCH_STUB_PRELUDE_MS:"20",BENCH_STUB_START_MS:"120",BENCH_STUB_RUN_MS:"40"},signal=new AbortController().signal,real=benchCliPortsV1(hostPorts(base,script,env,signal),signal);
  const editProbe=probe({id:"e",kind:"edit",lane:"oneshot",unit:"u",runs:3,warmup:0}),full=mini([editProbe]);
  const green=await benchRunProbeV1(full,editProbe,"nx",{override:null,service:false},real);
  expect(green.failed).toBe(0);
  expect(green.samples.every(row=>row.startKind==="marker"&&row.values.startMs!>=110&&row.values.greenMs!>row.values.startMs!)).toBe(true);
  expect(readFileSync(file,"utf8")).toBe(original);
  const redEnv={...env,BENCH_STUB_FAIL_TOKEN:"void"},red=benchCliPortsV1(hostPorts(base,script,redEnv,signal),signal);
  const broken=await benchRunProbeV1(full,editProbe,"bazel",{override:null,service:false},red);
  expect([broken.failed,broken.samples.length]).toEqual([3,3]);
  expect(readFileSync(file,"utf8")).toBe(original);
  const outside=benchCliPortsV1(hostPorts(base,script,env,signal),signal);
  await expect(outside.read("../escape.txt")).rejects.toThrow(/Wegwerf-Verzeichnis|disposable|repository-relative/u);
  await expect(outside.write("/abs/path",""))
   .rejects.toThrow();
 }finally{rmSync(base,{recursive:true,force:true});}
},90000);

test("real ports observe a cached run through the incremental harness: cacheable tasks that executed are misses, uncached tasks are counted apart",async()=>{
 const{base,script}=makeRoot("bench-cache-"),signal=new AbortController().signal,record=join(base,"run.json"),executed=new Set<string>(),commands:string[]=[];
 const plan={tasks:{tasks:{"a:build":{},"b:build":{}},dependencies:{"a:build":["b:build"],"b:build":[]}}};
 const run=async(_command:string,args:readonly string[])=>{
  const words=args.slice(3);commands.push(words.join(" "));
  if(words[0]==="graph"){const file=words[1]!.slice("--file=".length);writeFileSync(file,"{}");return file+"\n";}
  if(words[0]==="run"&&words.includes("--dry-run"))return "[script] planning\n{\n"+JSON.stringify(plan,null,1).slice(2);
  writeFileSync(record,JSON.stringify({tasks:[...executed].map(taskId=>({taskId,cacheStatus:"cache-miss",status:0})).concat(["a:build","b:build"].filter(id=>!executed.has(id)).map(taskId=>({taskId,cacheStatus:"local-cache-hit",status:0})))}));
  return "";
 };
 const adapter=async()=>({
  planBazelTasksV1:()=>({selected:["a:build"],tasks:{"a:build":{id:"a:build",dependencies:["b:build"],definition:{inputs:[],outputs:["x"]}},"b:build":{id:"b:build",dependencies:[],definition:{inputs:[],outputs:[]}}}}),
  bazelCacheDecisionV1:(task:{id:string})=>({cache:task.id==="a:build"}),bazelTaskCoverageV1:()=>({family:"aggregate"}),
  bazelSourceIndexV1:(inventory:readonly string[])=>({files:inventory,selectAll:()=>({marks:new Uint8Array(inventory.length)})}),bazelTaskInputsV1:()=>[]
 });
 try{
  const ports=benchCliPortsV1(hostPorts(base,script,{...process.env},signal,{run,adapter:adapter as never,nxRecord:()=>record,scratch:base}),signal);
  executed.add("b:build");
  expect(await ports.cache("nx","@x/y:build")).toEqual({cacheable:1,executedCacheable:0,uncachedExecuted:1,total:2});
  executed.add("a:build");
  expect(await ports.cache("nx","@x/y:build")).toEqual({cacheable:1,executedCacheable:1,uncachedExecuted:1,total:2});
  expect(commands.filter(command=>command.startsWith("graph")).length).toBe(1);
  expect(commands.filter(command=>command.includes("--dry-run")).length).toBe(1);
  const hit=probe({id:"c",kind:"cache",target:"@x/y:build",runs:2,warmup:0});
  executed.delete("a:build");
  const series=await benchRunProbeV1(mini([hit]),hit,"nx",{override:null,service:false},ports);
  expect(series.samples.map(row=>row.values.hitRatio)).toEqual([1,1]);
 }finally{rmSync(base,{recursive:true,force:true});}
},60000);

test("real ports run the concurrency harness for one workload and implementation and read its JSON report, also when its criteria fail",async()=>{
 const{base,script}=makeRoot("bench-conc-"),signal=new AbortController().signal,asked:string[][]=[];
 const wait=(waitMs:number,relation:string)=>({class:"x",resource:"r",mode:"shared",source:"lease",precise:true,startMs:0,endMs:waitMs,waitMs,holderPid:null,position:null,relation});
 const step=(waits:unknown[])=>({index:0,kind:"run",command:[],startMs:0,endMs:1,exit:0,waits,observation:null,observed:"none",outputs:null,stdout:null});
 const report={version:1,passed:0,failed:1,skipped:0,owners:[],runs:[{workload:"w",tool:"bazel",criteria:"default",rules:[{rule:"a",kind:"no-failure",owners:["x"],status:"fail",value:null,limit:null,detail:[]}],measurement:{workload:"w",tool:"bazel",uncached:[],sessions:[{id:"a",group:"g",startMs:0,endMs:900,steps:[step([wait(300,"unrelated")])]},{id:"b",group:"h",startMs:50,endMs:1000,steps:[step([])]}],serial:[{id:"a",group:"g",startMs:0,endMs:800,steps:[]},{id:"b",group:"h",startMs:800,endMs:1600,steps:[]}]}}]};
 try{
  const commandPorts={} as BenchCommandPortsV1,failing=async(_live:string,args:readonly string[],ports:BenchCommandPortsV1)=>{asked.push([...args]);ports.write(JSON.stringify(report));throw Error("Konformität der Gleichzeitigkeit verletzt");};
  const ports=benchCliPortsV1(hostPorts(base,script,{...process.env},signal,{commandPorts,concurrencyCommand:failing as never}),signal);
  expect(await ports.concurrency("bazel","disjoint-projects",3)).toEqual({makespanMs:1000,serialMs:1600,maxUnrelatedWaitMs:300,failedRules:1});
  expect(asked[0]).toEqual(["concurrency","run","--workloads=disjoint-projects","--sessions=3","--implementations=bazel","--json",`--root=${base}`]);
  const silent=benchCliPortsV1(hostPorts(base,script,{...process.env},signal,{commandPorts,concurrencyCommand:(async()=>{throw Error("not prepared");}) as never}),signal);
  await expect(silent.concurrency("nx","w",1)).rejects.toThrow("not prepared");
  const without=benchCliPortsV1(hostPorts(base,script,{...process.env},signal),signal);
  await expect(without.concurrency("nx","w",1)).rejects.toThrow(/Befehlsanschlüsse/u);
 }finally{rmSync(base,{recursive:true,force:true});}
},60000);

test("real ports find the resident service in the operations catalogue of the copy and start and stop it through the CLI",async()=>{
 const{base,script,catalogPath}=makeRoot("bench-svc-"),flag=join(base,"service.flag"),signal=new AbortController().signal;
 try{
  const env={...process.env,BENCH_STUB_SERVICE:flag},calls:string[]=[];
  const host=hostPorts(base,script,env,signal,{run:async(command,args)=>{calls.push(args.slice(2).join(" "));const done=spawnSync(command,[...args],{env:env as never,encoding:"utf8"});expect(done.status).toBe(0);return done.stdout;}});
  const ports=benchCliPortsV1(host,signal);
  expect(await ports.service.available()).toBe(false);
  writeFileSync(join(base,catalogPath),JSON.stringify({operations:[{name:"run"},{name:"service"}]}));
  expect(await ports.service.available()).toBe(true);
  await ports.service.start();expect(existsSync(flag)).toBe(true);
  await ports.service.stop();expect(existsSync(flag)).toBe(false);
  expect(calls).toEqual(["service start","service stop"]);
  writeFileSync(join(base,catalogPath),"not json");
  expect(await ports.service.available()).toBe(false);
 }finally{rmSync(base,{recursive:true,force:true});}
},60000);

const commandPorts=(live:string,root:string,environment:Record<string,string|undefined>,over:Partial<BenchCommandPortsV1>={}):{ports:BenchCommandPortsV1;out:string[];reports:Map<string,unknown>;texts:Map<string,string>;progress:string[]}=>{
 const out:string[]=[],reports=new Map<string,unknown>(),texts=new Map<string,string>(),progress:string[]=[],controller=new AbortController();
 const ports:BenchCommandPortsV1={
  write:line=>{out.push(line);},progress:async stage=>{progress.push(stage);},check:()=>{},signal:controller.signal,environment,platform:process.platform,tmpdir:()=>tmpdir(),home:()=>homedir(),now:()=>performance.now(),
  run:async()=>"",report:async(file,value)=>{reports.set(file,value);},files:async()=>["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔣️catalog.json"],adapter:async()=>{throw Error("no adapter");},nxRecord:()=>"",exists:async path=>existsSync(path),
  scratch:async()=>mkdtempSync(join(tmpdir(),"bench-scratch-")),readText:async path=>{const found=texts.get(path);if(found===undefined)throw Error("ENOENT "+path);return found;},writeText:async(path,content)=>{texts.set(path,content);},...over
 };
 return {ports,out,reports,texts,progress};
};

test("the command lists, runs against a stand-in CLI, stores a baseline and compares with it; blocking failures and regressions fail the exit code after the reports are written",async()=>{
 const{base,script}=makeRoot("bench-cmd-"),live=mkdtempSync(join(tmpdir(),"bench-live-")),env={...process.env,BENCH_STUB_HELP_MS:"30",BENCH_STUB_PRELUDE_MS:"10",BENCH_STUB_START_MS:"40",BENCH_STUB_RUN_MS:"20",SEMIO_LANGUAGE:"en"};
 try{
  const listed=commandPorts(live,base,env);
  await runBenchCommandV1(live,["bench","list","--lang=de"],listed.ports);
  expect(listed.out[0]).toContain("Ziele");
  const asJson=commandPorts(live,base,env);
  await runBenchCommandV1(live,["bench","list","--json"],asJson.ports);
  expect(JSON.parse(asJson.out[0]!).slos.length).toBe(set.slos.length);
  const missing=commandPorts(live,join(base,"nowhere"),env);
  await expect(runBenchCommandV1(live,["bench","run","--slo=cli-help-p95","--root",join(base,"nowhere")],missing.ports)).rejects.toThrow(/Prepare the disposable root/u);
  await expect(runBenchCommandV1(live,["bench","run","--slo=nope","--root",base],commandPorts(live,base,env).ports)).rejects.toThrow(/Unbekanntes Ziel/u);
  await expect(runBenchCommandV1(live,["bench","run","--root",live],commandPorts(live,live,env).ports)).rejects.toThrow(/separate|getrennt/u);
  const run=commandPorts(live,base,env,{});
  await runBenchCommandV1(live,["bench","run","--slo=probe:cli-help,probe:noop-run-oneshot,probe:cache-second-run-build","--implementations=nx","--runs=25","--root",base,"--file","report.json","--markdown","report.md","--save-baseline","baseline.json","--lang=en"],{...run.ports,overrides:{cache:async()=>({cacheable:6,executedCacheable:0,uncachedExecuted:2,total:8})}});
  const report=run.reports.get("report.json") as BenchReportV1;
  expect(define("Report")(plain(report)),JSON.stringify(define("Report").errors)).toBe(true);
  expect(report.rows.map(row=>row.slo)).toEqual(["cli-help-p50","cli-help-p95","cli-help-p99","noop-run-oneshot-p50","cache-second-run-build-min"]);
  const rows=new Map(report.rows.map(row=>[row.slo,row]));
  expect(rows.get("cli-help-p95")!.n).toBe(25);
  expect(rows.get("cli-help-p95")!.value!).toBeGreaterThanOrEqual(30);
  expect(rows.get("cli-help-p99")).toMatchObject({status:"unproven",reason:"too-few-samples"});
  expect(rows.get("noop-run-oneshot-p50")!.n).toBe(25);
  expect(rows.get("cache-second-run-build-min")).toMatchObject({status:"pass",value:1});
  expect(run.texts.get("report.md")).toContain("| ");
  expect(JSON.parse(run.texts.get("baseline.json")!).kind).toBe("bench-report");
  expect(run.out[0]).toContain("Performance objectives");
  expect(run.progress.some(stage=>/^cli-help:neutral:1\/\d+$/u.test(stage))).toBe(true);
  const again=commandPorts(live,base,env);
  again.texts.set("baseline.json",run.texts.get("baseline.json")!);
  await runBenchCommandV1(live,["bench","run","--slo=probe:cache-second-run-build","--implementations=nx","--root",base,"--baseline","baseline.json","--json"],{...again.ports,overrides:{cache:async()=>({cacheable:6,executedCacheable:0,uncachedExecuted:2,total:8})}});
  const compared=JSON.parse(again.out[0]!) as BenchReportV1;
  expect(compared.comparison!.rows.map(row=>row.status)).toEqual(["same"]);
  const failing=commandPorts(live,base,env);
  await expect(runBenchCommandV1(live,["bench","run","--slo=probe:cache-second-run-build","--implementations=nx","--root",base,"--file","fail.json","--markdown","fail.md","--lang=de"],{...failing.ports,overrides:{cache:async()=>({cacheable:6,executedCacheable:2,uncachedExecuted:0,total:6})}})).rejects.toThrow(/verbindliche Leistungsziele verletzt/u);
  expect((failing.reports.get("fail.json") as BenchReportV1).tally.blockingFailed).toBe(1);
  expect(failing.texts.get("fail.md")).toContain("Leistungsziele");
  const regress=commandPorts(live,base,env);
  regress.texts.set("baseline.json",run.texts.get("baseline.json")!);
  await expect(runBenchCommandV1(live,["bench","run","--slo=probe:cache-second-run-build","--implementations=nx","--root",base,"--baseline","baseline.json"],{...regress.ports,overrides:{cache:async()=>({cacheable:6,executedCacheable:1,uncachedExecuted:0,total:6})}})).rejects.toThrow(/blocking performance objectives failed/u);
  const cmp=commandPorts(live,base,env);
  cmp.texts.set("cur.json",JSON.stringify(report));cmp.texts.set("base.json",run.texts.get("baseline.json")!);
  await runBenchCommandV1(live,["bench","compare","--current=cur.json","--baseline=base.json","--markdown=cmp.md"],cmp.ports);
  expect(cmp.out[0]).toContain("Comparison with the baseline");
  expect(cmp.texts.get("cmp.md")).toContain("| ");
  const worse=plain(report) as BenchReportV1&{rows:BenchRowV1[]};
  const cache=worse.rows.find(row=>row.slo==="cache-second-run-build-min")!;(cache as any).value=0.5;(cache as any).status="fail";
  const worseCmp=commandPorts(live,base,env);
  worseCmp.texts.set("cur.json",JSON.stringify(worse));worseCmp.texts.set("base.json",JSON.stringify(report));
  await expect(runBenchCommandV1(live,["bench","compare","--current=cur.json","--baseline=base.json","--json"],worseCmp.ports)).rejects.toThrow(/Die Leistung hat sich bei 1 verbindlichen Zielen verschlechtert/u);
  expect(JSON.parse(worseCmp.out[0]!).tally.blockingRegressed).toBe(1);
  const junk=commandPorts(live,base,env);junk.texts.set("cur.json","{}");junk.texts.set("base.json","nope");
  await expect(runBenchCommandV1(live,["bench","compare","--current=cur.json","--baseline=base.json"],junk.ports)).rejects.toThrow(/Bericht .* nicht lesbar|Cannot read the report|Bench-Bericht/u);
 }finally{rmSync(base,{recursive:true,force:true});rmSync(live,{recursive:true,force:true});}
},120000);

test("the command measures an edit probe of the authored table end to end: markers of both implementations, restored file, start and green judged",async()=>{
 const{base}=makeRoot("bench-e2e-"),live=mkdtempSync(join(tmpdir(),"bench-live-")),unitOf=set.units.find(row=>row.id==="ts")!,file=join(base,unitOf.file),original="export const a=1;\n";
 mkdirSync(dirname(file),{recursive:true});writeFileSync(file,original);
 try{
  const env={...process.env,SEMIO_LANGUAGE:"en",BENCH_STUB_WATCH:file,BENCH_STUB_FAIL_TOKEN:"@@never@@",BENCH_STUB_PRELUDE_MS:"10",BENCH_STUB_START_MS:"80",BENCH_STUB_RUN_MS:"30"},world=commandPorts(live,base,env);
  await runBenchCommandV1(live,["bench","run","--slo=edit-ts-oneshot-start-p50,edit-ts-oneshot-green-p50","--runs=3","--root",base,"--json"],world.ports);
  const report=JSON.parse(world.out[0]!) as BenchReportV1,rows=new Map(report.rows.map(row=>[`${row.slo}/${row.tool}`,row]));
  expect([...rows.keys()]).toEqual(["edit-ts-oneshot-start-p50/bazel","edit-ts-oneshot-start-p50/nx","edit-ts-oneshot-green-p50/bazel","edit-ts-oneshot-green-p50/nx"]);
  for(const tool of ["nx","bazel"]){
   const start=rows.get(`edit-ts-oneshot-start-p50/${tool}`)!,green=rows.get(`edit-ts-oneshot-green-p50/${tool}`)!,series=report.series.find(row=>row.probe==="edit-ts-oneshot"&&row.tool===tool)!;
   expect(series.startKinds).toEqual({marker:3,"first-output":0,exit:0});
   expect(start.value!).toBeGreaterThanOrEqual(75);
   expect(start.status===("pass")||start.status==="unproven",`${tool} ${start.status}`).toBe(true);
   expect(green.status).toBe("recorded");
   expect(green.value!).toBeGreaterThan(start.value!);
  }
  expect(readFileSync(file,"utf8")).toBe(original);
  expect(world.progress.some(stage=>/^edit-ts-oneshot:bazel:\d+\/4$/u.test(stage))).toBe(true);
 }finally{rmSync(base,{recursive:true,force:true});rmSync(live,{recursive:true,force:true});}
},120000);

test("the command prepares a fresh copy on request so that the cold graph probe measures, and cancelling restores the edited files",async()=>{
 const live=mkdtempSync(join(tmpdir(),"bench-live-")),prepared=join(mkdtempSync(join(tmpdir(),"bench-root-")),"sinc"),script="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts",catalogPath="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔣️catalog.json";
 try{
  for(const path of [script,catalogPath]){mkdirSync(dirname(join(live,path)),{recursive:true});}
  copyFileSync(stubPath,join(live,script));writeFileSync(join(live,catalogPath),JSON.stringify({operations:[{name:"run"}]}));
  const env={...process.env,SEMIO_LANGUAGE:"en",BENCH_STUB_START_MS:"10"},steps:string[][]=[],world=commandPorts(live,prepared,env,{run:async(_command,args)=>{steps.push([...args].slice(3));return "";}});
  await runBenchCommandV1(live,["bench","run","--slo=graph-scope-cold-max","--implementations=nx","--prepare","--root",prepared,"--json"],world.ports);
  const report=JSON.parse(world.out[0]!) as BenchReportV1;
  expect(existsSync(join(prepared,script))).toBe(true);
  expect(steps.map(step=>step.join(" "))).toEqual(["preflight","tools setup all","dependencies sync --all","tools setup all"]);
  expect(report.rows[0]).toMatchObject({slo:"graph-scope-cold-max",status:"pass",n:1});
  const warm=commandPorts(live,prepared,env);
  await runBenchCommandV1(live,["bench","run","--slo=graph-scope-cold-max","--implementations=nx","--root",prepared,"--json"],warm.ports);
  expect(JSON.parse(warm.out[0]!).rows[0]).toMatchObject({status:"unproven",reason:"not-cold"});
  expect(world.progress.some(stage=>stage.startsWith("prepare"))).toBe(true);
 }finally{rmSync(live,{recursive:true,force:true});rmSync(dirname(prepared),{recursive:true,force:true});}
},120000);

test("the build workflow carries the bench job: after smoke, not blocking, pinned actions, no implementation named, a baseline kept per runner class",()=>{
 const find=(from:string):string=>{for(let at=from;;at=dirname(at)){const path=join(at,".github","workflows","build-system.yml");if(existsSync(path))return path;if(dirname(at)===at)throw Error("build workflow not found");}};
 const raw=readFileSync(find(import.meta.dir),"utf8"),workflow=(Bun as any).YAML.parse(raw) as any,job=workflow.jobs.bench;
 expect(Object.keys(workflow.jobs)).toEqual(expect.arrayContaining(["plan","smoke","affected","parity","bench"]));
 expect(job).toBeDefined();
 expect(job["continue-on-error"]).toBe(true);
 expect(job.needs).toEqual(["smoke"]);
 expect(job["timeout-minutes"]).toBeLessThanOrEqual(180);
 expect(job.strategy["fail-fast"]).toBe(false);
 expect(job.strategy.matrix.include.map((row:any)=>row.name)).toEqual(["linux","macos"]);
 const steps:any[]=job.steps,runs=steps.flatMap(step=>step.run?[String(step.run)]:[]).join("\n");
 for(const wanted of ["repo preflight","repo tools setup all","repo dependencies sync --all","repo bench list","repo incremental prepare --fresh",'repo bench run "${args[@]}"',"--save-baseline","--baseline"])expect(runs,wanted).toContain(wanted);
 expect(runs).not.toMatch(/\b(nx|bazel|bazelisk|cargo|npx)\b/u);
 for(const step of steps)if(step.uses&&!step.uses.startsWith("./"))expect(step.uses,step.uses).toMatch(/^[\w.-]+\/[\w./-]+@[0-9a-f]{40}$/u);
 const names=steps.map(step=>step.name);
 expect(names.indexOf("Restore the baseline of this runner class")).toBeLessThan(names.indexOf("Measure the objectives and compare with the baseline"));
 expect(names.indexOf("Prepare the disposable copy")).toBeLessThan(names.indexOf("Measure the objectives and compare with the baseline"));
 const save=steps.find(step=>step.name==="Save the baseline (default branch)");
 expect(save.if).toContain("promoted");
 const measure=steps.find(step=>step.name==="Measure the objectives and compare with the baseline");
 expect(measure.env.REF).toBe("${{ github.ref }}");
 expect(String(measure.run)).not.toContain("${{");
 const upload=steps.find(step=>step.uses?.startsWith("actions/upload-artifact"));
 expect(upload.with["retention-days"]).toBeLessThanOrEqual(90);
 expect(workflow.permissions).toEqual({contents:"read"});
 for(const other of ["plan","smoke","affected","parity"])expect(workflow.jobs[other].steps.length).toBeGreaterThan(0);
 expect(raw).toContain("#   bench    -");
 for(const step of steps)if(step.uses&&!step.uses.startsWith("./"))expect(raw,step.uses).toContain(`${step.uses} # v`);
});
