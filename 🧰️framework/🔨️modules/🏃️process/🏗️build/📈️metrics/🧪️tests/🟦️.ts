import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {copyFileSync,mkdtempSync,openSync,closeSync,readFileSync,readdirSync,rmSync,statSync,writeFileSync,writeSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {METRICS_CATALOGUE_V1,METRICS_RING_V1,metricSupportedOnV1,METRICS_SOURCES_V1,METRICS_TIERS_V1,closeMetricsRingsV1,createCounterDeltasV1,openMetricsRingsV1,readRingFileV1,readSeriesV1,recordSampleV1,type MetricsPointV1,type MetricsTierV1} from "../🟦️.ts";
import {requireTestToolV1} from "../../../🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {metricWindowTotalsV1,pointShareV1} from "../🪟️windows/🟦️.ts";

/** 📈️ Contract: the catalogue is complete and bilingual, the tiers fold exactly, a torn row reads as absent, a restart changes nothing, and TypeScript, Python and Rust read the same bytes into the same series. */
const here=import.meta.dir,oracle=join(here,"🐍️.py"),golden=join(here,"🧫️fixtures","🏅️golden"),S=schema.definitions;
const scratch=(name:string):string=>mkdtempSync(join(tmpdir(),`metrics-${name}-`));
const python=(args:string[],input?:string):string=>{const run=spawnSync(requireTestToolV1("python"),[oracle,...args],{input,encoding:"utf8",maxBuffer:1<<28});expect(run.status,run.stderr).toBe(0);return run.stdout;};
const SMALL:readonly MetricsTierV1[]=[{id:"t0",bucketMs:1000,buckets:8},{id:"t1",bucketMs:4000,buckets:6},{id:"t2",bucketMs:16000,buckets:5}];
const IDS=["host.cpu.busy","host.memory.available","build.units.compiled","ready.stage"];
const BASE=1_800_000_000_000;
const lcg=(seed:number)=>{let state=seed;return ():number=>{state=(state*1103515245+12345)%2147483648;return state/2147483648;};};

function samplesOf(seed:number,ticks:number):{at:number;values:Record<string,number>}[]{
 const next=lcg(seed),out:{at:number;values:Record<string,number>}[]=[];
 for(let tick=0;tick<ticks;tick++){
  if(next()<0.12)continue;
  for(let repeat=0;repeat<(next()<0.2?2:1);repeat++){
   const values:Record<string,number>={};
   for(const id of IDS)if(next()<0.85)values[id]=Math.floor(next()*400)/4;
   out.push({at:BASE+tick*1000+repeat*400+Math.floor(next()*300),values});
  }
 }
 return out;
}
function feed(directory:string,samples:readonly {at:number;values:Record<string,number>}[],reopenEvery=0):void{
 let handle=openMetricsRingsV1({directory,ids:IDS,tiers:SMALL,createdMs:BASE});
 samples.forEach((sample,at)=>{
  if(reopenEvery>0&&at>0&&at%reopenEvery===0&&Math.floor(samples[at-1]!.at/1000)!==Math.floor(sample.at/1000)){closeMetricsRingsV1(handle);handle=openMetricsRingsV1({directory,ids:IDS,tiers:SMALL,createdMs:BASE});}
  recordSampleV1(handle,sample.at,sample.values);
 });
 closeMetricsRingsV1(handle);
}
const rowsOf=(directory:string)=>SMALL.map(tier=>readRingFileV1(join(directory,`${tier.id}.ring`))!);
const pythonSeries=(directory:string,from:number,to:number,id:string):string=>python(["series",directory,String(from),String(to)]).split("\n").filter(row=>row.startsWith(id+"\t")).map(row=>row.slice(id.length+1)).join("\n");
const lines=(series:{metrics:Readonly<Record<string,readonly MetricsPointV1[]>>},ids:readonly string[]):string=>ids.flatMap(id=>(series.metrics[id]??[]).map(point=>[id,...point].join("\t"))).join("\n");
const blocks=(directory:string,ids:readonly string[]):string=>readFileSync(join(golden,"queries.tsv"),"utf8").trim().split("\n").map(row=>{const [name,from,to]=row.split("\t");const body=lines(readSeriesV1({directory,fromMs:Number(from),toMs:Number(to)}),ids);return `# ${name}\n${body}${body===""?"":"\n"}`;}).join("");

test("catalogue: unique dotted ids, known kinds, units and sources, bilingual labels, every metric of the design", () => {
 const ids=METRICS_CATALOGUE_V1.map(metric=>metric.id),pattern=new RegExp(S.Metric.properties.id.pattern);
 expect(new Set(ids).size).toBe(ids.length);
 expect(ids.length).toBeGreaterThanOrEqual(60);
 for(const metric of METRICS_CATALOGUE_V1){
  expect(metric.id,metric.id).toMatch(pattern);
  expect(S.Metric.properties.kind.enum,metric.id).toContain(metric.kind);
  expect(Object.keys(S.Units.const),metric.id).toContain(metric.unit);
  expect(Object.keys(METRICS_SOURCES_V1),`${metric.id} has no collector`).toContain(metric.source);
  expect(metric.label.en.trim().length,metric.id).toBeGreaterThan(0);
  expect(metric.label.de.trim().length,metric.id).toBeGreaterThan(0);
  expect(Object.keys(metric).filter(key=>key!=="platforms").sort(),metric.id).toEqual(["id","kind","label","scope","source","unit"]);
  expect(Object.keys(S.Scopes.const),metric.id).toContain((metric as unknown as {scope:string}).scope);
 }
 expect(S.Metric.properties.kind.enum.sort()).toEqual(Object.keys(S.Kinds.const).sort());
 expect(S.Metric.properties.unit.enum.sort()).toEqual(Object.keys(S.Units.const).sort());
 expect(S.Metric.properties.scope.enum.sort()).toEqual(Object.keys(S.Scopes.const).sort());
 expect(METRICS_CATALOGUE_V1.filter(metric=>(metric as unknown as {scope:string}).scope==="fleet").map(metric=>metric.id).sort(),"the fleet-scope metrics are the facts one service knows").toEqual(["cache.remote.bytes","cache.remote.entries","exec.offload.in","exec.queue.queued","exec.queue.running"]);
 expect(new Set(METRICS_CATALOGUE_V1.map(metric=>metric.source))).toEqual(new Set(Object.keys(METRICS_SOURCES_V1)));
 for(const id of ["host.cpu.busy","host.memory.available","host.disk.free","host.swap.used","build.units.compiled","build.units.restored.local","build.units.restored.fleet","cache.remote.ac.hit","cache.remote.ac.miss","cache.local.hit","exec.offload.in","exec.offload.out","exec.refused","graph.read.owners","graph.read.facts","ready.stage","process.repo.count"])expect(ids,id).toContain(id);
 expect(ids.filter(id=>id.startsWith("metrics.self.")).length).toBeGreaterThanOrEqual(5);
 for(const entry of [...Object.values(S.Kinds.const),...Object.values(S.Units.const),...Object.values(S.Scopes.const),...Object.values(METRICS_SOURCES_V1),...METRICS_TIERS_V1])expect(Object.keys((entry as {label:object}).label).sort()).toEqual(["de","en"]);
 for(const language of ["en","de"] as const)expect(Object.keys(S.Texts.const[language]).sort()).toEqual(Object.keys(S.Texts.const.en).sort());
});

test("platforms: only the metrics a platform has no source for list the platforms that have one, and an unlisted platform is n/a", () => {
 const listed=METRICS_CATALOGUE_V1.filter(metric=>metric.platforms!==undefined);
 expect(listed.map(metric=>metric.id).sort()).toEqual(["host.cpu.load","host.files.open","host.images.attached","host.memory.compressed","host.swap.rate","host.swap.used","process.repo.cpu","warden.headroom"]);
 for(const metric of listed){
  expect(metric.platforms!.length,metric.id).toBeGreaterThan(0);expect(new Set(metric.platforms).size,metric.id).toBe(metric.platforms!.length);
  for(const platform of metric.platforms!)expect(S.Metric.properties.platforms.items.enum,metric.id).toContain(platform);
  expect(metric.platforms,`${metric.id}: the Windows host sampler does not ask for it`).not.toContain("win32");
  expect(metricSupportedOnV1(metric.id,"win32"),metric.id).toBe(false);
  expect(metricSupportedOnV1(metric.id,metric.platforms![0]!),metric.id).toBe(true);
 }
 expect(metricSupportedOnV1("host.cpu.busy","win32")).toBe(true);expect(metricSupportedOnV1("no.such.metric","win32")).toBe(true);
 expect(metricSupportedOnV1("host.memory.compressed","linux")).toBe(false);expect(metricSupportedOnV1("host.cpu.load","linux")).toBe(true);
 const independent=JSON.parse(python(["platforms",join(here,"..","📇️catalogue","🔣️.json")])) as Record<string,string[]|null>;
 for(const metric of METRICS_CATALOGUE_V1)expect(independent[metric.id]??null,metric.id).toEqual((metric.platforms as string[]|undefined)??null);
});

test("readers: a ring directory with AppleDouble stubs (._t0.ring) beside the rings reads exactly as without", () => {
 const directory=scratch("appledouble");
 feed(directory,samplesOf(7,40));
 const before=lines(readSeriesV1({directory,fromMs:BASE-1000,toMs:BASE+60_000,ids:IDS}),IDS);
 for(const tier of SMALL)writeFileSync(join(directory,`._${tier.id}.ring`),Buffer.alloc(4096,1));
 expect(lines(readSeriesV1({directory,fromMs:BASE-1000,toMs:BASE+60_000,ids:IDS}),IDS)).toBe(before);
 expect(before.length).toBeGreaterThan(0);
 rmSync(directory,{recursive:true,force:true});
});

test("tiers: the five tiers of the design, each a multiple of the one before, sized by the schema", () => {
 expect(METRICS_TIERS_V1.map(tier=>[tier.id,tier.bucketMs,tier.buckets])).toEqual([["t0",1000,300],["t1",10_000,360],["t2",60_000,1440],["t3",600_000,1008],["t4",3_600_000,2160]]);
 expect(METRICS_TIERS_V1.map(tier=>tier.bucketMs*tier.buckets)).toEqual([300_000,3_600_000,86_400_000,604_800_000,7_776_000_000]);
 METRICS_TIERS_V1.forEach((tier,index)=>{if(index>0)expect(tier.bucketMs%METRICS_TIERS_V1[index-1]!.bucketMs).toBe(0);});
 expect(METRICS_RING_V1.slotBytes).toBe(36);
 expect(S.Ring.description).toContain("SMRG");
});

test("fold: every tier row equals the fold of the raw samples of its window (Python oracle), across wraps and gaps", () => {
 for(const seed of [1,2,3]){
  const directory=scratch("fold"),samples=samplesOf(seed,400);
  feed(directory,samples);
  const expected=JSON.parse(python(["fold"],JSON.stringify({ids:IDS,tiers:SMALL,samples}))) as {tier:number;rows:{start:number;slots:number[][]}[]}[];
  const actual=rowsOf(directory);
  expected.forEach((tier,index)=>{
   expect(actual[index]!.rows.map(row=>row.start),`seed ${seed} tier ${index}`).toEqual(tier.rows.map(row=>row.start));
   actual[index]!.rows.forEach((row,at)=>expect(row.slots,`seed ${seed} tier ${index} row ${row.start}`).toEqual(tier.rows[at]!.slots as never));
   expect(tier.rows.length).toBeGreaterThan(0);
  });
  expect(expected[0]!.rows.length).toBeGreaterThan(3);expect(expected[0]!.rows.length).toBeLessThanOrEqual(SMALL[0]!.buckets);
  rmSync(directory,{recursive:true,force:true});
 }
});

test("fold: min, max, sum, count and last of a window with several samples", () => {
 const directory=scratch("one"),handle=openMetricsRingsV1({directory,ids:IDS,tiers:SMALL});
 for(const [offset,value] of [[0,4],[100,-2],[900,10],[950,3.5]] as const)recordSampleV1(handle,BASE+offset,{"host.cpu.busy":value});
 recordSampleV1(handle,BASE+1000,{"host.cpu.busy":1});
 closeMetricsRingsV1(handle);
 const [first,second]=readRingFileV1(join(directory,"t0.ring"))!.rows;
 expect(first!.slots[0]).toEqual([4,-2,10,15.5,3.5]);
 expect(first!.slots[1]).toEqual([0,0,0,0,0]);
 expect(second!.slots[0]).toEqual([1,1,1,1,1]);
 rmSync(directory,{recursive:true,force:true});
});

test("a restart changes nothing: rings written with reopens equal rings written in one go", () => {
 const samples=samplesOf(7,300),one=scratch("whole"),many=scratch("reopen");
 feed(one,samples);feed(many,samples,5);
 expect(rowsOf(many)).toEqual(rowsOf(one));
 expect(rowsOf(one)[2]!.rows.length).toBeGreaterThan(2);
 rmSync(one,{recursive:true,force:true});rmSync(many,{recursive:true,force:true});
});

test("a torn row reads as absent and a missing or stale cursor is found again", () => {
 const directory=scratch("torn"),samples=Array.from({length:30},(_,at)=>({at:BASE+at*1000+10,values:{"host.cpu.busy":at/4}}));
 feed(directory,samples);
 const path=join(directory,"t0.ring"),whole=readRingFileV1(path)!,newest=whole.rows.at(-1)!,previous=whole.rows.at(-2)!;
 expect(whole.head).toBe(newest.start);
 const rowBytes=2*METRICS_RING_V1.stampBytes+IDS.length*METRICS_RING_V1.slotBytes,header=readFileSync(path).readUInt32LE(28);
 const patch=(offset:number,bytes:Buffer):void=>{const fd=openSync(path,"r+");try{writeSync(fd,bytes,0,bytes.length,offset);}finally{closeSync(fd);}};
 const slot=((newest.start/1000)%whole.bucketCount)*rowBytes+header;
 patch(slot+rowBytes-8,Buffer.alloc(8,0xab));
 const torn=readRingFileV1(path)!;
 expect(torn.rows.at(-1)!.start).toBe(previous.start);
 expect(torn.head).toBe(previous.start);
 expect(Object.values(readSeriesV1({directory,fromMs:BASE,toMs:BASE+100_000}).metrics).flat().some(point=>point[0]===newest.start&&point[2]===0)).toBe(false);
 patch(slot+rowBytes-8,Buffer.from(readFileSync(path).subarray(slot,slot+8)));
 patch(40,Buffer.alloc(8,0));
 expect(readRingFileV1(path)!.head).toBe(newest.start);
 const stale=Buffer.alloc(8);stale.writeUInt32LE((newest.start-2000)%4294967296,0);stale.writeUInt32LE(Math.floor((newest.start-2000)/4294967296),4);
 patch(40,stale);
 expect(readRingFileV1(path)!.head).toBe(newest.start);
 const again=openMetricsRingsV1({directory,ids:IDS,tiers:SMALL});
 expect(recordSampleV1(again,newest.start,{"host.cpu.busy":1}).dropped).toBe(1);
 expect(recordSampleV1(again,newest.start+1000,{"host.cpu.busy":1}).accepted).toBe(1);
 closeMetricsRingsV1(again);
 rmSync(directory,{recursive:true,force:true});
});

test("rejected samples are counted, never thrown; a changed catalogue starts empty rings", () => {
 const directory=scratch("drop"),handle=openMetricsRingsV1({directory,ids:IDS,tiers:SMALL});
 expect(recordSampleV1(handle,BASE+5000,{"host.cpu.busy":1,"no.such.metric":2,"ready.stage":Number.NaN})).toEqual({accepted:1,dropped:1,unknown:1});
 expect(recordSampleV1(handle,BASE+6000,{"host.cpu.busy":1}).accepted).toBe(1);
 expect(recordSampleV1(handle,BASE+1000,{"host.cpu.busy":1}).accepted).toBe(0);
 closeMetricsRingsV1(handle);
 expect(()=>recordSampleV1(handle,BASE,{})).toThrow(" / ");
 expect(readRingFileV1(join(directory,"t0.ring"))!.rows.length).toBe(2);
 const other=openMetricsRingsV1({directory,ids:[...IDS,"host.cpu.cores"],tiers:SMALL});
 closeMetricsRingsV1(other);
 expect(readRingFileV1(join(directory,"t0.ring"))!.rows.length).toBe(0);
 expect(()=>openMetricsRingsV1({directory,ids:["a.b","a.b"],tiers:SMALL})).toThrow(" / ");
 expect(()=>openMetricsRingsV1({directory,ids:IDS,tiers:[{id:"t0",bucketMs:1000,buckets:4},{id:"t1",bucketMs:1500,buckets:4}]})).toThrow(" / ");
 rmSync(directory,{recursive:true,force:true});
});

test("a merged series is dense recent and coarse old, never overlaps and narrows by ids", () => {
 const directory=scratch("merge"),samples=samplesOf(5,200);
 feed(directory,samples);
 const to=BASE+200_000,series=readSeriesV1({directory,fromMs:BASE,toMs:to});
 for(const id of IDS){
  const points=series.metrics[id]!;
  expect(points.length,id).toBeGreaterThan(8);
  points.forEach((point,at)=>{
   if(point[8]===0)expect(point[0]%point[1]).toBe(0);
   expect(point[0]+point[1]).toBeLessThanOrEqual(to);
   if(at>0){const before=points[at-1]!;expect(before[0]+before[1],`${id} overlap at ${point[0]}`).toBeLessThanOrEqual(point[0]);expect(before[2],`${id} finer than newer at ${point[0]}`).toBeGreaterThanOrEqual(point[2]);}
  });
  expect(new Set(points.map(point=>point[2])).size).toBeGreaterThan(1);
 }
 const narrow=readSeriesV1({directory,fromMs:BASE,toMs:to,ids:["ready.stage"]});
 expect(Object.keys(narrow.metrics)).toEqual(["ready.stage"]);
 expect(narrow.metrics["ready.stage"]).toEqual(series.metrics["ready.stage"]);
 expect(()=>readSeriesV1({directory,fromMs:BASE,toMs:to,ids:["x.y"]})).toThrow(" / ");
 expect(()=>readSeriesV1({directory,fromMs:to,toMs:BASE})).toThrow("leer");
 expect(readSeriesV1({directory:join(directory,"absent"),fromMs:0,toMs:1}).metrics).toEqual({});
 rmSync(directory,{recursive:true,force:true});
});

test("a merged series of steady data has no gap at tier boundaries; straddling coarse buckets are clipped and partial", () => {
 const directory=scratch("gapless"),samples=Array.from({length:150},(_,at)=>({at:BASE+at*1000+10,values:{"host.cpu.busy":at/4,"ready.stage":1}}));
 feed(directory,samples);
 const to=BASE+150_000,points=readSeriesV1({directory,fromMs:BASE,toMs:to}).metrics["host.cpu.busy"]!;
 points.forEach((point,at)=>{if(at>0)expect(points[at-1]![0]+points[at-1]![1],`gap before ${point[0]}`).toBe(point[0]);});
 expect(points.at(-1)![0]+points.at(-1)![1]).toBe(BASE+150_000);
 const partial=points.filter(point=>point[8]===1);
 expect(partial.length).toBeGreaterThan(0);
 for(const point of partial){const whole=readRingFileV1(join(directory,`t${point[2]}.ring`))!.rows.find(row=>row.start===point[0])!;expect(point[1]).toBeLessThan(SMALL[point[2]]!.bucketMs);expect(point[3]).toBe(whole.slots[0]![0]);expect(point[6]).toBe(whole.slots[0]![3]);}
 const viaTs=readSeriesV1({directory,fromMs:BASE,toMs:to}).metrics;
 expect(viaTs["host.cpu.busy"]!.map(point=>point.join("\t")).join("\n")).toBe(pythonSeries(directory,BASE,to,"host.cpu.busy"));
 rmSync(directory,{recursive:true,force:true});
});

test("counter deltas equal the Python oracle, a decrease is a restart", () => {
 const next=lcg(99),readings:{id:string;value:number}[]=[];
 const totals:Record<string,number>={a:0,b:100};
 for(let index=0;index<300;index++){const id=next()<0.5?"a":"b";totals[id]=next()<0.1?Math.floor(next()*5):totals[id]!+Math.floor(next()*20);readings.push({id,value:totals[id]!});}
 const expected=JSON.parse(python(["delta"],JSON.stringify({readings}))) as (number|null)[],deltas=createCounterDeltasV1();
 expect(readings.map(row=>deltas.delta(row.id,row.value))).toEqual(expected);
 expect(expected.filter(value=>value===null).length).toBe(2);
});

test("golden: the TypeScript writer reproduces the checked-in rings byte for byte, TypeScript and Python read them into the checked-in series", () => {
 const input=JSON.parse(readFileSync(join(golden,"🔣️.json"),"utf8")) as {createdMs:number;ids:string[];tiers:MetricsTierV1[];samples:{at:number;values:Record<string,number>}[]};
 const directory=scratch("golden"),handle=openMetricsRingsV1({directory,ids:input.ids,tiers:input.tiers,createdMs:input.createdMs});
 for(const sample of input.samples)recordSampleV1(handle,sample.at,sample.values);
 closeMetricsRingsV1(handle);
 const text=blocks(directory,input.ids);
 if(process.env["METRICS_GOLDEN_WRITE"]==="1"){for(const tier of input.tiers)copyFileSync(join(directory,`${tier.id}.ring`),join(golden,`${tier.id}.ring`));writeFileSync(join(golden,"series.tsv"),text);}
 for(const tier of input.tiers)expect(readFileSync(join(directory,`${tier.id}.ring`)).equals(readFileSync(join(golden,`${tier.id}.ring`))),`${tier.id}.ring drifted from the golden file`).toBe(true);
 expect(text).toBe(readFileSync(join(golden,"series.tsv"),"utf8"));
 expect(blocks(golden,input.ids)).toBe(text);
 const queries=readFileSync(join(golden,"queries.tsv"),"utf8").trim().split("\n"),perQuery=text.split("# ").slice(1);
 queries.forEach((row,at)=>{
  const [name,from,to]=row.split("\t"),viaPython=python(["series",golden,from!,to!]).trimEnd();
  expect(`${name}\n${viaPython}${viaPython===""?"":"\n"}`,`python reader, query ${name}`).toBe(perQuery[at]);
 });
 expect(text).toContain("\t2\t");
 expect(text).toContain("\t0\t");
 rmSync(directory,{recursive:true,force:true});
});

test("production sizes: one file per tier of rowBytes * buckets, and a write costs microseconds", () => {
 const directory=scratch("size"),handle=openMetricsRingsV1({directory,createdMs:BASE}),ids=METRICS_CATALOGUE_V1.map(metric=>metric.id);
 const values=Object.fromEntries(ids.map((id,at)=>[id,at*0.25]));
 const started=performance.now();
 for(let tick=0;tick<200;tick++)recordSampleV1(handle,BASE+tick*1000,values);
 const perTick=(performance.now()-started)/200;
 closeMetricsRingsV1(handle);
 const rowBytes=2*METRICS_RING_V1.stampBytes+ids.length*METRICS_RING_V1.slotBytes,idsBytes=Math.ceil(new TextEncoder().encode(ids.join("\n")).length/8)*8;
 for(const tier of METRICS_TIERS_V1)expect(statSync(join(directory,`${tier.id}.ring`)).size).toBe(METRICS_RING_V1.headerBytes+idsBytes+tier.buckets*rowBytes);
 expect(readdirSync(directory).filter(name=>name.endsWith(".tmp"))).toEqual([]);
 expect(perTick).toBeLessThan(5);
 expect(readRingFileV1(join(directory,"t1.ring"))!.rows.length).toBeGreaterThan(10);
 rmSync(directory,{recursive:true,force:true});
});

test("windows: a counter's trailing totals equal the sum of its raw samples (exact inside the finest tier, a clipped share beyond it); no rings is null", () => {
 const directory=scratch("windows"),samples=Array.from({length:100},(_,index)=>({at:BASE+index*1000+10,values:{"build.units.compiled":index%5+1}})),now=BASE+100_000;
 feed(directory,samples);
 const raw=(seconds:number):number=>samples.filter(sample=>sample.at>=now-seconds*1000).reduce((sum,sample)=>sum+sample.values["build.units.compiled"],0);
 const found=metricWindowTotalsV1({directory,ids:["build.units.compiled"],nowMs:now,windowsSeconds:[4,8,32]})!;
 expect(found.windowsSeconds).toEqual([4,8,32]);
 expect(found.totals["build.units.compiled"]![0]).toBe(raw(4));expect(found.totals["build.units.compiled"]![1]).toBe(raw(8));
 expect(Math.abs(found.totals["build.units.compiled"]![2]!-raw(32))).toBeLessThanOrEqual(Math.ceil(raw(32)*0.15));
 expect(metricWindowTotalsV1({directory:join(directory,"absent"),ids:["build.units.compiled"],nowMs:now,windowsSeconds:[4]})).toBeNull();
 expect(metricWindowTotalsV1({directory,ids:["x.y"],nowMs:now,windowsSeconds:[4]})).toBeNull();
 expect(pointShareV1([0,10,0,1,0,0,100,0,0],5,20)).toBe(50);expect(pointShareV1([0,10,0,1,0,0,100,0,0],10,20)).toBe(0);
 rmSync(directory,{recursive:true,force:true});
});
