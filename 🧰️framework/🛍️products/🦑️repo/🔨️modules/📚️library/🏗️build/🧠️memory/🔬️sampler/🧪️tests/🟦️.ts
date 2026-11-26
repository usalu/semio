/** 🔬️ Contract: the sampling profile of a worker counts the self function and the distinct functions of every stack trace, by uptime bucket, bounded, and reaches the slice report as the source `profile`. @see ../🟨️.mjs */
import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import schema from"../🧬️schema/🔣️.json";
import{addSampleTracesV1,createSampleBookV1,sampleBucketV1,startSliceSamplerV1,summarizeSampleBookV1,SLICE_SAMPLER_POLICY_V1}from"../🟨️.mjs";
import{buildSliceReportV1}from"../../📋️slice-report/🟨️.mjs";

const frame=(name:string,line:number,file="/x/y/"+name+".mjs")=>({name,sourceURL:file,line,column:1}),native=(name:string)=>({name,sourceURL:"",line:4294967295,column:0});
const stacks=[["a","b","c"],["a","b"],["d","b","c"],["c"],["a","a","b"],["e"],["a","b","c"],["b","c"]];
const traces=stacks.map(names=>({frames:names.map((name,at)=>frame(name,at+1))}));

test("self and inclusive counts agree with SQLite and the Summary schema",()=>{
 const book=createSampleBookV1();addSampleTracesV1(book,traces);const summary=summarizeSampleBookV1(book),db=new Database(":memory:");
 try{
  db.exec("create table f(trace integer,depth integer,name text,line integer)");const insert=db.query("insert into f values(?,?,?,?)");
  traces.forEach((trace,at)=>trace.frames.forEach((row,depth)=>insert.run(at,depth,row.name,row.line)));
  const self=db.query<{name:string;line:number;n:number},[]>("select name,line,count(*) n from f where depth=0 group by name,line order by n desc,name").all();
  const inclusive=db.query<{name:string;line:number;n:number},[]>("select name,line,count(distinct trace) n from f group by name,line order by n desc,name").all();
  expect(summary.self.map(row=>[row.name,row.line,row.samples])).toEqual(self.map(row=>[row.name,row.line,row.n]));
  expect(summary.inclusive.map(row=>[row.name,row.line,row.samples])).toEqual(inclusive.map(row=>[row.name,row.line,row.n]));
  expect(summary.samples).toBe(traces.length);
  const ajv=new Ajv({strict:true});ajv.addSchema(schema);expect(ajv.validate(schema.$id+"#/definitions/Summary",summary)).toBe(true);
 }finally{db.close();}
});

test("a key beyond the maximum is dropped and counted, frames beyond the maximum are cut",()=>{
 const policy={...SLICE_SAMPLER_POLICY_V1,maximumKeys:3,maximumFrames:2},book=createSampleBookV1();
 addSampleTracesV1(book,traces,policy);expect(book.self.size).toBeLessThanOrEqual(3);expect(book.inclusive.size).toBeLessThanOrEqual(3);expect(book.dropped).toBeGreaterThan(0);
});

test("buckets follow the uptime and the sampler reports through the slice report and stops cleanly",()=>{
 expect([0,149,150,399,400,9999].map(seconds=>sampleBucketV1(seconds))).toEqual([0,0,1,1,2,2]);
 let drains=0,uptime=10,tick:(()=>void)|undefined,cleared=false;
 const engine={startSamplingProfiler(){},samplingProfilerStackTraces(){drains++;return{interval:0.001,traces:drains<=3?traces:[]};}};
 const stop=startSliceSamplerV1({engine,environment:{SEMIO_SLICE_SAMPLER:"on"},uptime:()=>uptime,setTimer:(run:()=>void)=>{tick=run;return{unref(){}};},clearTimer:()=>{cleared=true;}} as never);
 tick!();uptime=200;tick!();uptime=500;tick!();
 const report=buildSliceReportV1({reason:"interval"}) as {sources:{profile:{books:{samples:number}[];buckets:number[]}}};
 expect(report.sources.profile.books.map(book=>book.samples)).toEqual([traces.length,traces.length,traces.length]);expect(report.sources.profile.buckets).toEqual([150,400]);
 stop();expect(cleared).toBe(true);expect((buildSliceReportV1({reason:"interval"}) as {sources:Record<string,unknown>}).sources.profile).toBeUndefined();
 const off=startSliceSamplerV1({engine,environment:{}});expect(typeof off).toBe("function");
 expect(startSliceSamplerV1({engine:{},environment:{SEMIO_SLICE_SAMPLER:"on"}})).toBeInstanceOf(Function);
});

test("the time in natives is charged to the first function of the script and its caller, checked against SQLite",()=>{
 const stacks=[[native("readFileSync"),frame("load",3),frame("settle",9)],[native("update"),native("hash"),frame("listDigest",5),frame("save",2)],[native("readFileSync"),frame("load",3),frame("settle",9)],[frame("load",3),frame("settle",9)],[native("gc")]],book=createSampleBookV1();
 addSampleTracesV1(book,stacks.map(frames=>({frames})));const summary=summarizeSampleBookV1(book),db=new Database(":memory:");
 try{
  db.exec("create table f(trace integer,depth integer,name text,line integer,script integer)");const insert=db.query("insert into f values(?,?,?,?,?)");
  stacks.forEach((trace,at)=>trace.forEach((row,depth)=>insert.run(at,depth,row.name,row.line,row.sourceURL===""?0:1)));
  const rows=db.query<{name:string;line:number;parent:string;n:number},[]>("with first as(select trace,min(depth) d from f where script=1 group by trace) select a.name name,a.line line,coalesce((select b.name from f b where b.trace=a.trace and b.script=1 and b.depth>a.depth order by b.depth limit 1),'(top)') parent,count(*) n from f a join first on a.trace=first.trace and a.depth=first.d group by a.name,a.line,parent order by n desc,a.name").all();
  expect(summary.attributed.map(row=>[row.name,row.line,row.parent,row.samples])).toEqual(rows.map(row=>[row.name,row.line,row.parent,row.n]));
  expect(summary.attributed.some(row=>row.name==="gc")).toBe(false);
  const ajv=new Ajv({strict:true});ajv.addSchema(schema);expect(ajv.validate(schema.$id+"#/definitions/Summary",summary)).toBe(true);
 }finally{db.close();}
});

test("a worker does not run the profiler unless asked",()=>{
 let started=0;const engine={startSamplingProfiler(){started++;},samplingProfilerStackTraces(){return{traces:[]};}};
 startSliceSamplerV1({engine,environment:{}})();startSliceSamplerV1({engine,environment:{SEMIO_SLICE_SAMPLER:"off"}})();expect(started).toBe(0);
 startSliceSamplerV1({engine,environment:{SEMIO_SLICE_SAMPLER:"on"},setTimer:()=>({unref(){}}),clearTimer(){}} as never)();expect(started).toBe(1);
});
