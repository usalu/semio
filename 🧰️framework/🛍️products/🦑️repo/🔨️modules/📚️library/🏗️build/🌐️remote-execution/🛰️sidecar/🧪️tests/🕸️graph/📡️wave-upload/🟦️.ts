/** 📡️ Contract: every wave row of the real child process reaches the wave upload of the graph server, a failing listener never harms the read or its log, and the server runs the upload only on a serving machine, beside its read, and stops it with the server. */
import{expect,test}from"bun:test";
import{chmodSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createGraphWaveMeterV1,renderGraphWaveRowV1}from"../../../../../🕸️graph/📈️wave-row/🟨️.mjs";
import{createGenerationUploaderV1}from"../../../../../🕸️graph/🕰️generations/📡️wave-upload/🟦️.ts";
import{wantedServerPolicyV1}from"../../../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";
import{inferGraphReadV1}from"../../../🕸️graph/🟦️.ts";

const fakeRepo=(script:string)=>{const dir=mkdtempSync(join(tmpdir(),"graph-wave-up-")),path=join(dir,"repo");writeFileSync(path,"#!/bin/sh\n"+script+"\n");chmodSync(path,0o755);return{dir,path};};
const rowOf=(generation:number,done:number)=>renderGraphWaveRowV1(createGraphWaveMeterV1({now:()=>0}).take({generation,parentMegabytes:3100,done,total:96}));
const readOf=(command:string,over:object)=>inferGraphReadV1({command,cwd:tmpdir(),environment:{},reason:"refresh",signal:undefined,language:"en",policy:{...wantedServerPolicyV1,inferStallMilliseconds:5000,watchMilliseconds:40},...over});

test("each row of the child reaches the wave upload, which sends the generation with the number of projects still stale",async()=>{
 const{dir,path}=fakeRepo(`echo '${rowOf(11,48)}' >&2\nsleep 0.1\necho '${rowOf(12,72)}' >&2\nexit 0`);
 try{
  const sent:{stale?:number}[]=[],lines:string[]=[];let generation=11;
  const uploader=createGenerationUploaderV1({source:{pointer:async()=>generation,upload:async({stale})=>{sent.push({stale});return{status:"uploaded",documents:10,sent:2,bytes:1048576};}},language:"en",say:line=>lines.push(line)});
  const seen:number[][]=[];
  const result=await readOf(path,{say:()=>undefined,onWave:(row:{generation:number;done:number;total:number})=>{seen.push([row.generation,row.done,row.total]);uploader.wave(row);}});
  expect(result.ok).toBe(true);expect(seen).toEqual([[11,48,96],[12,72,96]]);
  generation=12;uploader.tick();await uploader.idle();
  expect(sent).toEqual([{stale:24}]);expect(lines[0]).toContain("24 projects still stale");
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a listener that throws harms neither the read nor the log line",async()=>{
 const{dir,path}=fakeRepo(`echo '${rowOf(5,48)}' >&2\nexit 0`);
 try{
  const lines:string[]=[],result=await readOf(path,{say:(line:string)=>lines.push(line),onWave:()=>{throw new Error("uploader broke");}});
  expect(result.ok).toBe(true);expect(result.generation).toBe(5);expect(lines).toHaveLength(1);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("the server creates the upload only on a serving machine, feeds it from the read, ticks it every round and stops it with the server",()=>{
 const source=readFileSync(new URL("../../../🕸️graph/🟦️.ts",import.meta.url),"utf8");
 expect(source).toContain("onWave:row=>uploader?.wave(row)");
 expect(source.indexOf("if(serving){")).toBeGreaterThan(source.indexOf("const serving="));
 expect(source.indexOf("createGenerationUploaderV1({")).toBeGreaterThan(source.indexOf("if(serving){"));
 expect(source).toContain("setInterval(()=>uploader?.tick(),policy.roundMilliseconds)");
 expect(source.indexOf("clearInterval(tickTimer)")).toBeGreaterThan(source.indexOf("await runWantedServerV1("));
 expect(source).toContain("uploader?.stop();await uploader?.idle()");
 expect(source).toContain("policy.serveTotalMegabytes");
});
