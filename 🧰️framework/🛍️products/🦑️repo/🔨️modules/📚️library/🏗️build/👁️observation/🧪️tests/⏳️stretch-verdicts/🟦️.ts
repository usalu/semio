/** 🫟️ Contract: the stored rows of a shared computation that many owners ask for one after the other in one synchronous stretch (the assembly authority of a read, outside any validation round) are hashed once while its stamp is unchanged, not once per owner: the workspace Cargo.toml was hashed 579 times and 89 953 file hashes (262 MB) blocked the loop of the reader for the first native owner; an edit moves the change time and is seen at once. @see ../../🟨️.mjs */
import {afterAll,expect,test} from "bun:test";
import * as fs from "node:fs";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {memoizedObservedComputationV1,observedFs,withObservedWorldV1} from "../../🟨️.mjs";

const roots:string[]=[],FILES=60,OWNERS=50;
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

const sources=(bytes:number)=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"observation-stretch-"));roots.push(root);mkdirSync(join(root,"s"));
 for(let index=0;index<FILES;index++)writeFileSync(join(root,"s",`${index}.txt`),`source ${index} `.padEnd(bytes,"x"));
 return root;
};
const counting=()=>{const reads={count:0};return{reads,port:new Proxy(fs,{get:(target,key)=>key==="readFileSync"?(...args:any[])=>{reads.count++;return (target as any).readFileSync(...args);}:(target as any)[key]})};};
const compute=(root:string)=>()=>{let total=0;for(let index=0;index<FILES;index++)total+=String(observedFs.readFileSync(join(root,"s",`${index}.txt`),"utf8")).length;return total;};

test("the parent does under 50 ms of cpu per owner after the first when the owners run one after the other in one stretch",async()=>{
 const root=sources(1536*1024),cache=new Map();
 await Bun.sleep(2100);
 const cpu=()=>{const usage=process.cpuUsage();return(usage.user+usage.system)/1000;};
 memoizedObservedComputationV1(cache,"shared",compute(root));
 memoizedObservedComputationV1(cache,"shared",compute(root));
 const before=cpu();
 for(let owner=1;owner<OWNERS;owner++)memoizedObservedComputationV1(cache,"shared",compute(root));
 expect((cpu()-before)/(OWNERS-1)).toBeLessThan(50);
},60000);

test("the parent does under 50 ms of cpu per owner after the first when every owner runs in a stretch of its own",async()=>{
 const root=sources(1536*1024),cache=new Map();
 await Bun.sleep(2100);
 const cpu=()=>{const usage=process.cpuUsage();return(usage.user+usage.system)/1000;};
 memoizedObservedComputationV1(cache,"shared",compute(root));
 await Promise.resolve();
 const before=cpu();
 for(let owner=1;owner<OWNERS;owner++){await new Promise(done=>setImmediate(done));memoizedObservedComputationV1(cache,"shared",compute(root));}
 expect((cpu()-before)/(OWNERS-1)).toBeLessThan(50);
},60000);

test("a rewrite of the same size is seen by the next stretch although the file was remembered",async()=>{
 const root=sources(64),cache=new Map();
 await Bun.sleep(2100);
 memoizedObservedComputationV1(cache,"shared",compute(root));
 await new Promise(done=>setImmediate(done));memoizedObservedComputationV1(cache,"shared",compute(root));
 await new Promise(done=>setImmediate(done));memoizedObservedComputationV1(cache,"shared",compute(root));
 writeFileSync(join(root,"s","0.txt"),"source 0 ".padEnd(64,"y"));
 await new Promise(done=>setImmediate(done));
 let computed=0;
 memoizedObservedComputationV1(cache,"shared",()=>{computed++;return compute(root)();});
 expect(computed).toBe(1);
},30000);

test("an edit between two stretches refutes the remembered rows",async()=>{
 const root=sources(64),cache=new Map();
 await Bun.sleep(2100);
 let first=0;
 withObservedWorldV1(fs,()=>{first=memoizedObservedComputationV1(cache,"shared",compute(root));memoizedObservedComputationV1(cache,"shared",compute(root));});
 await Bun.sleep(10);
 writeFileSync(join(root,"s","0.txt"),"edited, and much longer than the source it replaces ".repeat(8));
 let computed=0;
 const second=withObservedWorldV1(fs,()=>memoizedObservedComputationV1(cache,"shared",()=>{computed++;return compute(root)();}));
 expect(computed).toBe(1);
 expect(second).toBeGreaterThan(first);
},30000);
