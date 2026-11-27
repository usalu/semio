import {expect,test} from "bun:test";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {watchChangesV1} from "../../🔒️lock/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";
import cases from "./🔣️.json" with {type:"json"};

const fake=()=>({on:()=>undefined,close:()=>undefined}) as never;

for(const row of cases.cases)test(`a waiter on ${row.platform} opens ${row.watchesOpened} directory watches and ${row.waitsAtMostWatchlessPoll?"sleeps at most the watchless poll":"sleeps the whole fallback"}`,async()=>{
 const opened:string[]=[],listener=watchChangesV1(Array.from({length:row.directories},(_,index)=>`/d${index}`),{platform:row.platform,watch:(directory=>{opened.push(String(directory));return fake();}) as never}),started=Date.now();
 expect(opened.length).toBe(row.watchesOpened);
 await listener.wait(row.waitsAtMostWatchlessPoll?3000:60,undefined);
 const slept=Date.now()-started;
 if(row.waitsAtMostWatchlessPoll)expect(slept).toBeLessThan(UNIT_CACHE_POLICY_V1.lock.watchlessPollMilliseconds*1.2+250);else expect(slept).toBeGreaterThanOrEqual(40);
 listener.close();
});

test.skipIf(process.platform==="darwin")("the native waiter still wakes at once when a claim file appears, with the real watch of this host",async()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-watchless-")),listener=watchChangesV1([root],{platform:"linux"});
 try{
  const started=Date.now(),waiting=listener.wait(5000);
  setTimeout(()=>writeFileSync(join(root,"claim.lock"),"x"),30);
  await waiting;
  expect(Date.now()-started).toBeLessThan(2500);
 }finally{listener.close();rmSync(root,{recursive:true,force:true});}
});
