/** 🌊️ Contract: a graph read of the server is started asking for wave rows, and each row of the real child process becomes one line of the server log in the language of the machine; a failed read is explained by its last line that is not a row. */
import{expect,test}from"bun:test";
import{chmodSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createGraphWaveMeterV1,renderGraphWaveRowV1}from"../../../../../🕸️graph/📈️wave-row/🟨️.mjs";
import{wantedServerPolicyV1}from"../../../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";
import{inferGraphReadV1}from"../../../🕸️graph/🟦️.ts";

const fakeRepo=(script:string)=>{const dir=mkdtempSync(join(tmpdir(),"graph-wave-")),path=join(dir,"repo");writeFileSync(path,"#!/bin/sh\n"+script+"\n");chmodSync(path,0o755);return{dir,path};};
const rowOf=(generation:number,done:number)=>renderGraphWaveRowV1(createGraphWaveMeterV1({now:()=>0}).take({generation,parentMegabytes:3100+generation,done,total:96}));
const readOf=(command:string,say:(line:string)=>void,over:object={})=>inferGraphReadV1({command,cwd:tmpdir(),environment:{},reason:"refresh",signal:undefined,language:"en",say,policy:{...wantedServerPolicyV1,inferStallMilliseconds:5000,watchMilliseconds:40},...over});

test("the read is started asking for rows, every row of the child is one log line and the generation is the newest row",async()=>{
 const{dir,path}=fakeRepo(`echo "asked=$SEMIO_GRAPH_WAVE_ROWS" >&2\necho '${rowOf(11,48)}' >&2\necho 'graph: generation 11 published' >&2\nsleep 0.1\necho '${rowOf(12,96)}' >&2\nexit 0`);
 try{
  const lines:string[]=[],result=await readOf(path,line=>lines.push(line));
  expect(result.ok).toBe(true);expect(result.generation).toBe(12);
  expect(lines).toHaveLength(2);
  expect(lines[0]).toStartWith("graph-server: wave 11 published: 48 of 96 projects");expect(lines[0]).toContain("parent 3111 MB");
  expect(lines[1]).toStartWith("graph-server: wave 12 published: 96 of 96 projects");
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("the child sees SEMIO_GRAPH_WAVE_ROWS=1 and the lines come in German on a German machine",async()=>{
 const{dir,path}=fakeRepo(`if [ "$SEMIO_GRAPH_WAVE_ROWS" = 1 ]; then echo '${rowOf(4,48)}' >&2; fi\nexit 0`);
 try{
  const lines:string[]=[],result=await readOf(path,line=>lines.push(line),{language:"de"});
  expect(result.generation).toBe(4);expect(lines).toHaveLength(1);expect(lines[0]).toStartWith("graph-server: Welle 4 veröffentlicht: 48 von 96 Projekten");
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a failed read is explained by its last line that is not a row, and a missing say callback is harmless",async()=>{
 const{dir,path}=fakeRepo(`echo 'graph: the fleet cache refused' >&2\necho '${rowOf(6,48)}' >&2\nexit 3`);
 try{
  const result=await readOf(path,()=>{throw new Error("log closed");});
  expect(result.ok).toBe(false);expect(result.status).toBe(3);expect(result.detail).toBe("graph: the fleet cache refused");
  const quiet=await inferGraphReadV1({command:path,cwd:tmpdir(),environment:{},reason:"refresh",signal:undefined,language:"en",policy:{...wantedServerPolicyV1,watchMilliseconds:40}});
  expect(quiet.generation).toBe(6);
 }finally{rmSync(dir,{recursive:true,force:true});}
});
