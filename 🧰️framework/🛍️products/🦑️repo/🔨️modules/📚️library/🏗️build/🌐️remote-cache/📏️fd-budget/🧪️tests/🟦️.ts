import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {readRemoteCacheFileLimitsV1,remoteCacheFileBudgetOfMachineV1,remoteCacheFileBudgetV1,remoteCacheWidthV1} from "../🟦️.ts";
import {scanRemoteCacheDirectoryV1} from "../../🧹️collector/🟦️.ts";
import {RemoteCacheStoreV1} from "../../💾️store/🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

test("the file budget is a quarter of the soft limit and of what one process may hold, at most one percent of the kernel table, never below 64, and a conservative number without readable limits",()=>{
 expect(remoteCacheFileBudgetV1({maxFiles:245760,perProcess:122880,soft:10240})).toBe(2457);
 expect(remoteCacheFileBudgetV1({maxFiles:10_000_000,perProcess:122880,soft:1024})).toBe(256);
 expect(remoteCacheFileBudgetV1({maxFiles:245760,perProcess:122880,soft:100})).toBe(64);
 expect(remoteCacheFileBudgetV1({maxFiles:null,perProcess:null,soft:null})).toBe(256);
 expect(remoteCacheFileBudgetOfMachineV1({SEMIO_REMOTE_CACHE_FILE_BUDGET:"96"})).toBe(96);
 const limits=readRemoteCacheFileLimitsV1();if(process.platform==="darwin"||process.platform==="linux"){expect(limits.maxFiles).toBeGreaterThan(1000);expect(limits.soft).toBeGreaterThan(0);}
});

test("a fan-out is as wide as the budget allows at its cost per operation, and at least one",()=>{
 expect(remoteCacheWidthV1(64,2457)).toBe(64);expect(remoteCacheWidthV1(64,40)).toBe(40);expect(remoteCacheWidthV1(64,8,4)).toBe(2);expect(remoteCacheWidthV1(64,0)).toBe(1);expect(remoteCacheWidthV1(8,1000,3)).toBe(8);
});

test("a scan asked for an absurd width is cut to the budget and still indexes everything",async()=>{
 const root=mkdtempSync(join(tmpdir(),"rcache-fd-"));scratch.push(root);
 for(let index=0;index<200;index++){const key=index.toString(16).padStart(64,"0"),folder=join(root,"v1","main","ac",key.slice(0,2));mkdirSync(folder,{recursive:true});writeFileSync(join(folder,key),"x");}
 const store=await RemoteCacheStoreV1.open({directory:root,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeMilliseconds:0,fileBudget:64,startupScan:"background"});
 await store.refresh(10_000);
 expect(store.usage().entries).toBe(200);
 await store.close();
 expect((await scanRemoteCacheDirectoryV1(join(root,"v1"),{concurrency:3})).size,"a narrow scan sees the same entries").toBe(200);
});

test("two processes' scans of one directory never overlap: the second waits for the turn, and a scan that fails gives the turn back",async()=>{
 const root=mkdtempSync(join(tmpdir(),"rcache-turn-"));scratch.push(root);
 const open=()=>RemoteCacheStoreV1.open({directory:root,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeMilliseconds:0,scanRetryMilliseconds:15,startupScan:"background"});
 const first=await open(),second=await open();
 let running=0,most=0;const work=(milliseconds:number,fail=false)=>async():Promise<void>=>{running++;most=Math.max(most,running);await new Promise(done=>setTimeout(done,milliseconds));running--;if(fail)throw Error("scan failed");};
 const results=await Promise.allSettled([first.exclusively(work(120,true)),second.exclusively(work(60)),first.exclusively(work(30))]);
 expect(most,"never two scans at once").toBe(1);
 expect(results.map(row=>row.status)).toEqual(["rejected","fulfilled","fulfilled"]);
 await first.close();await second.close();
});
