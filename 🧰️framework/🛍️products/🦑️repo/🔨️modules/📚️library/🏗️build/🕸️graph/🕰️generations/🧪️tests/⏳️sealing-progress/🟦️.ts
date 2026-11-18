/** ⏳️ Contract: after the last owner is inferred, the steps that store the fragments, merge the fact index and seal the generation are never silent: each says in English and German that it starts, and the fragment step counts its owners, so a read at "owners N of N" is never mistaken for a hang. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {loadGenerationGraphV1,renderGenerationMessageV1} from "../../🔌️load/🟨️.mjs";
import {createMockHost,ringFiles,seed} from "../🏃️world/🟦️.ts";

const COUNT=60,scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-sealing-progress-"));
const read=async(language:"en"|"de")=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(COUNT));
  const mock=createMockHost(dir,{language});
  await loadGenerationGraphV1(mock.host as never,{});
  return mock;
 }finally{rmSync(dir,{recursive:true,force:true});}
};

test("a cold build says storing, indexing and sealing between its first and its last line, with counts, in both languages",async()=>{
 for(const language of ["en","de"] as const){
  const mock=await read(language),keys=mock.said.map(row=>row.key);
  const at=["cold","storing","indexing","sealing","published"].map(key=>keys.indexOf(key));
  expect(at.every(index=>index>=0)).toBe(true);
  expect([...at].sort((a,b)=>a-b)).toEqual(at);
  const line=(key:string)=>mock.said.find(row=>row.key===key)!.text;
  expect(line("storing")).toContain(String(COUNT));
  expect(line("indexing")).toContain(String(COUNT));
  for(const key of ["storing","indexing","sealing"])expect(line(key)).not.toContain("{");
 }
 const en=renderGenerationMessageV1("storing" as never,{count:COUNT},"en"),de=renderGenerationMessageV1("storing" as never,{count:COUNT},"de");
 expect(en).not.toBe(de);
});

test("the fragment step counts its owners from zero to all, never backwards, so the host line shows how far it is",async()=>{
 const mock=await read("en");
 const counters=mock.published.flatMap(resource=>{const match=/^build:graph-phase:fragments:(\d+):(\d+)/u.exec(resource);return match?[[Number(match[1]),Number(match[2])]]:[];});
 expect(counters.length).toBeGreaterThanOrEqual(2);
 expect(counters[0]![0]).toBe(0);
 expect(counters.at(-1)).toEqual([COUNT,COUNT]);
 expect(counters.every(([done,total],index)=>done<=total&&(index===0||done>=counters[index-1]![0]))).toBe(true);
});
