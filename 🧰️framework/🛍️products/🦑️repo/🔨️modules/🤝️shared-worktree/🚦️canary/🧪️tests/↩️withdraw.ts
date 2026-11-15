import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {ensureStoreDirectoryV1,storePathsV1} from "../../📒️journal/🟦️.ts";
import {readRunHistoryV1,recordRefusedV1,recordRunV1,suspectCodeV1,withdrawRefusedV1} from "../../🛡️resilience/🟦️.ts";
import {blamesWatchdogV1,publishRefusalV1,readRefusalsOfV1,refusalProofMsV1,watchdogWindowMsV1,withdrawRefusalV1} from "../🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`withdraw-${label}-`));scratch.push(path);return path;};
const D="d".repeat(64),E="e".repeat(64);

test("the watchdog blames only a code that was ended within three minutes of its first run: the kills of 4 and 12 minutes after the switch (g1-server 11:17 and 11:26) blame nothing",()=>{
 expect(watchdogWindowMsV1).toBe(180_000);
 const first=Date.parse("2026-11-14T11:13:41+01:00");
 const blames=(minutes:number)=>blamesWatchdogV1({firstRunMs:first,endedMs:first+minutes*60_000,windowMs:watchdogWindowMsV1});
 expect([blames(0.5),blames(3)]).toEqual([true,true]);
 expect([blames(3.01),blames(4),blames(12.3)]).toEqual([false,false,false]);
 expect(refusalProofMsV1).toBe(600_000);
});

test("a refusal a machine has disproved is withdrawn from its history and from its record in the store, other digests stay, and the code is no longer suspect",async()=>{
 const root=temp("tree"),store=temp("store");
 await ensureStoreDirectoryV1(store,storePathsV1(store).origins);
 await recordRunV1(root,D,1000);await recordRefusedV1(root,D);await recordRefusedV1(root,E);
 await publishRefusalV1(store,"g1-m01",{digest:D,reason:"the watchdog ended this code while it was new"},5);
 await publishRefusalV1(store,"g1-m01",{digest:E,reason:"self-check failed"},6);
 expect(suspectCodeV1(await readRunHistoryV1(root),"x".repeat(64),D)).toBe("refused before");
 const history=await withdrawRefusedV1(root,D);
 expect(history.refused).toEqual([E]);
 expect(suspectCodeV1(history,D,D)).toBeUndefined();
 expect(await withdrawRefusalV1(store,"g1-m01",D)).toBe(true);
 expect((await readRefusalsOfV1(store,"g1-m01"))!.map(item=>item.digest)).toEqual([E]);
 expect(await withdrawRefusalV1(store,"g1-m01",D)).toBe(false);
 expect(await withdrawRefusalV1(store,"g1-m09",D)).toBe(false);
 expect((await withdrawRefusedV1(root,D)).refused).toEqual([E]);
});
