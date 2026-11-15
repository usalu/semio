/** 📈️ Contract: a published wave leaves one machine readable row (generation, projects done of total, seconds, parent footprint, managed memory, peak with its projects, collections, farm tally) that validates against its schema, survives the stderr line and is worded in English and German; the meter hands over exactly what happened since the previous row. */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{readFileSync}from"node:fs";
import Ajv from"ajv";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import{GRAPH_WAVE_POLICY_V1,announceGraphWaveV1,GRAPH_WAVE_TEXTS_V1,createGraphWaveMeterV1,graphWaveRowsWantedV1,parseGraphWaveRowV1,renderGraphWaveRowV1,renderGraphWaveTextV1}from"../🟨️.mjs";

const validate=new Ajv({strict:false}).compile({...schema.definitions.Row,definitions:schema.definitions}),MB=1048576;

test("only a read the server started writes rows",()=>{
 expect(graphWaveRowsWantedV1({})).toBe(false);expect(graphWaveRowsWantedV1({SEMIO_GRAPH_WAVE_ROWS:"1"})).toBe(true);expect(graphWaveRowsWantedV1({SEMIO_GRAPH_WAVE_ROWS:"0"})).toBe(false);
 expect(GRAPH_WAVE_POLICY_V1.languages).toEqual(["en","de"]);
});

test("the meter hands over the wave it saw, then starts the next one from nothing",()=>{
 let clock=1_000_000;const meter=createGraphWaveMeterV1({now:()=>clock});
 meter.progress({total:703,done:0});
 meter.observe(300*MB,["a"]);meter.observe(900*MB,["b","c","d","e","f"]);meter.observe(500*MB,["z"]);
 meter.parent({peakBytes:850*MB,liveBytes:420*MB,collections:2,collectMilliseconds:1800});meter.parent({peakBytes:100*MB,liveBytes:410*MB,collections:1,collectMilliseconds:700});
 meter.farm({workers:5,projects:40,adopted:37,failed:2,lost:1,declined:0});meter.farm({workers:3,projects:8,adopted:8,failed:0,lost:0,declined:0});
 clock+=87_400;
 const row=meter.take({generation:128,parentMegabytes:4210,done:96});
 expect(row).toEqual({version:1,generation:128,done:96,total:703,seconds:87,parentMegabytes:4210,managedMegabytes:410,peakMegabytes:900,peakProjects:["b","c","d","e"],collections:3,collectMilliseconds:2500,farm:{workers:5,projects:48,adopted:45,failed:2,lost:1,declined:0}});
 expect(validate(row),JSON.stringify(validate.errors)).toBe(true);
 clock+=60_000;
 const next=meter.take({generation:129,parentMegabytes:null,done:144});
 expect(next).toEqual({version:1,generation:129,done:144,total:703,seconds:60,parentMegabytes:null,managedMegabytes:0,peakMegabytes:0,peakProjects:[],collections:0,collectMilliseconds:0,farm:{workers:0,projects:0,adopted:0,failed:0,lost:0,declined:0}});
 expect(validate(next),JSON.stringify(validate.errors)).toBe(true);
});

test("the total never falls below the projects done",()=>{
 const meter=createGraphWaveMeterV1({now:()=>0});
 expect(meter.take({generation:1,done:50,total:48}).total).toBe(50);
});

test("a row survives its stderr line, also inside other output; other lines and broken rows read as nothing",()=>{
 const meter=createGraphWaveMeterV1({now:()=>0}),row=meter.take({generation:7,parentMegabytes:1200,done:48,total:4700}),line=renderGraphWaveRowV1(row);
 expect(line.startsWith(GRAPH_WAVE_POLICY_V1.marker)).toBe(true);expect(line.includes("\n")).toBe(false);
 expect(parseGraphWaveRowV1(line)).toEqual(row);expect(parseGraphWaveRowV1("noise "+line)).toEqual(row);
 expect(parseGraphWaveRowV1("graph: generation 7 published")).toBeUndefined();
 expect(parseGraphWaveRowV1(GRAPH_WAVE_POLICY_V1.marker+"{")).toBeUndefined();
 expect(parseGraphWaveRowV1(GRAPH_WAVE_POLICY_V1.marker+JSON.stringify({...row,generation:-1}))).toBeUndefined();
 expect(parseGraphWaveRowV1(GRAPH_WAVE_POLICY_V1.marker+JSON.stringify({...row,farm:{workers:1}}))).toBeUndefined();
 expect(parseGraphWaveRowV1(GRAPH_WAVE_POLICY_V1.marker+JSON.stringify({...row,peakProjects:["a","b","c","d","e"]}))).toBeUndefined();
});

test("every parsed row is one the schema accepts and every rejected one breaks the schema",()=>{
 const meter=createGraphWaveMeterV1({now:()=>0}),good=meter.take({generation:3,parentMegabytes:10,done:1,total:2});
 const bad=[{...good,version:2},{...good,seconds:1.5},{...good,peakProjects:[""]},{...good,farm:{...good.farm,lost:-1}},{...good,extra:1}];
 for(const row of bad){expect(validate(row)).toBe(false);expect(parseGraphWaveRowV1(GRAPH_WAVE_POLICY_V1.marker+JSON.stringify(row))).toBeUndefined();}
});

test("the log line is worded in English and in German with every number of the row",()=>{
 const row={version:1 as const,generation:128,done:96,total:4700,seconds:87,parentMegabytes:4210,managedMegabytes:410,peakMegabytes:900,peakProjects:["hub-a","hub-b"],collections:3,collectMilliseconds:2500,farm:{workers:5,projects:48,adopted:45,failed:2,lost:1,declined:0}};
 const en=renderGraphWaveTextV1(row,"en"),de=renderGraphWaveTextV1(row,"de");
 expect(en).toBe("graph-server: wave 128 published: 96 of 4700 projects, 87 s since the last, parent 4210 MB (managed 410 MB after the collection, peak 900 MB at hub-a, hub-b, 3 collections), farm: 45 of 48 projects adopted, 2 failed, 1 lost, 5 workers");
 expect(de).toBe("graph-server: Welle 128 veröffentlicht: 96 von 4700 Projekten, 87 s seit der letzten, Elternprozess 4210 MB (verwaltet 410 MB nach der Bereinigung, Höchstwert 900 MB bei hub-a, hub-b, 3 Bereinigungen), Farm: 45 von 48 Projekten übernommen, 2 fehlgeschlagen, 1 verloren, 5 Arbeiter");
 expect(renderGraphWaveTextV1({...row,parentMegabytes:null,peakProjects:[]},"en")).toContain("parent unread MB (managed 410 MB after the collection, peak 900 MB, 3 collections)");
 expect(renderGraphWaveTextV1({...row,parentMegabytes:null,peakProjects:[]},"de")).toContain("Elternprozess unlesbar MB");
 expect(Object.keys(GRAPH_WAVE_TEXTS_V1.en).sort()).toEqual(Object.keys(GRAPH_WAVE_TEXTS_V1.de).sort());
});


test("a read the server did not start writes nothing and does not touch the meter",async()=>{
 const lines:string[]=[],meter=createGraphWaveMeterV1({now:()=>0});
 expect(await announceGraphWaveV1({generation:1,environment:{},write:line=>lines.push(line),meter,observe:()=>{throw new Error("must not be read");}})).toBeUndefined();expect(lines).toEqual([]);
});

test("the announcement reads the footprint of the pid, hands the wave over and writes one parsable line",async()=>{
 const lines:string[]=[],meter=createGraphWaveMeterV1({now:()=>0}),pids:number[]=[];
 meter.parent({peakBytes:700*MB,liveBytes:300*MB,collections:1,collectMilliseconds:90});
 const row=await announceGraphWaveV1({generation:9,info:{done:48,total:300},environment:{SEMIO_GRAPH_WAVE_ROWS:"1"},write:line=>lines.push(line),meter,pid:4242,observe:async pid=>{pids.push(pid);return 3999.6;}});
 expect(pids).toEqual([4242]);expect(lines).toHaveLength(1);
 expect(parseGraphWaveRowV1(lines[0]!)).toEqual(row);expect(row).toMatchObject({generation:9,done:48,total:300,parentMegabytes:4000,managedMegabytes:300,peakMegabytes:700,collections:1});
});

test("a footprint that cannot be read, that throws or that hangs leaves the field null and still writes the row",async()=>{
 for(const observe of[()=>null,()=>{throw new Error("footprint refused");},()=>new Promise<number>(()=>{}),()=>Number.NaN]){
  const lines:string[]=[],row=await announceGraphWaveV1({generation:2,environment:{SEMIO_GRAPH_WAVE_ROWS:"1"},write:line=>lines.push(line),meter:createGraphWaveMeterV1({now:()=>0}),observe:observe as never,limitMilliseconds:30});
  expect(row?.parentMegabytes).toBeNull();expect(lines).toHaveLength(1);
 }
});

test("a failing write port never fails the caller",async()=>{
 expect(await announceGraphWaveV1({generation:2,environment:{SEMIO_GRAPH_WAVE_ROWS:"1"},write:()=>{throw new Error("stderr closed");},meter:createGraphWaveMeterV1({now:()=>0}),observe:()=>1})).toBeUndefined();
});

test("the system footprint of this very process is read and agrees with the independent footprint tool",async()=>{
 const lines:string[]=[];
 const row=await announceGraphWaveV1({generation:1,environment:{SEMIO_GRAPH_WAVE_ROWS:"1"},write:line=>lines.push(line),meter:createGraphWaveMeterV1({now:()=>0})});
 expect(row).toBeDefined();expect(typeof row!.parentMegabytes).toBe(process.platform==="darwin"||process.platform==="linux"?"number":typeof row!.parentMegabytes);
 if(process.platform==="darwin"){
  const text=spawnSync("footprint",[String(process.pid)],{encoding:"utf8"}).stdout,found=/Footprint:\s+([\d.]+)\s+(KB|MB|GB)/u.exec(text);
  if(found&&row!.parentMegabytes!==null){const tool=Number(found[1])*({KB:1/1024,MB:1,GB:1024} as Record<string,number>)[found[2]!]!;expect(Math.abs(row!.parentMegabytes-tool)).toBeLessThan(Math.max(300,tool*0.25));}
 }
},60000);

const source=(path:string)=>readFileSync(new URL(path,import.meta.url),"utf8");

test("the host writes the row of every published wave before it may pause, and the load hands the build information through to the host",()=>{
 const host=source("../../🕰️host/🟦️.ts"),published=host.split("\n").find(line=>line.includes("    published:async(generation:number"))!;
 expect(published).toContain("(generation:number,info?:{done:number;total:number})");
 expect(published).toContain("await announceGraphWaveV1({generation,info})");
 expect(published.indexOf("announceGraphWaveV1")).toBeLessThan(published.indexOf("yieldBackgroundGraphV1"));
 expect(source("../../🕰️generations/🔌️load/🟨️.mjs")).toContain("published:builder&&host.published?(generation,info)=>host.published(generation,info):undefined");
});
