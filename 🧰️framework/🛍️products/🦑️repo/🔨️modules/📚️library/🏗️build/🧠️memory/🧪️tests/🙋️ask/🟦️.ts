/** 🙋️ Contract: the governor of a read claims from the one budget of its machine through the ask port: the first waiting owner (alone when the cost table gave it a solo limit) while it waits, an alone owner while it runs, nothing when no owner waits or runs alone. */
import{expect,test}from"bun:test";
import{createMemoryGovernorV1}from"../../🟨️.mjs";

const MB=1048576;
const port=()=>{const rows:Record<string,unknown>[]=[];return{rows,ask:{set:(row:Record<string,unknown>)=>{rows.push({op:"set",...row});},clear:()=>{rows.push({op:"clear"});}}};};
const until=async(check:()=>boolean)=>{for(let at=0;at<400&&!check();at++)await new Promise(done=>setTimeout(done,10));expect(check()).toBe(true);};

test("a waiting solo owner claims alone with its cost, keeps the claim while it runs and withdraws it when it ends",async()=>{
 const {rows,ask}=port();
 const governor=createMemoryGovernorV1({workers:2,headroom:()=>2000*MB,estimate:key=>({a:1000,b:7000}[key as "a"|"b"])*MB,ask});
 let freeA!:()=>void,freeB!:()=>void;const holdA=new Promise<void>(done=>{freeA=done;}),holdB=new Promise<void>(done=>{freeB=done;});
 const a=governor.run(async()=>{await holdA;},{kind:"k",key:"a"});
 const b=governor.run(async()=>{await holdB;},{kind:"k",key:"b",limitBytes:8000*MB});
 await until(()=>rows.some(row=>row.op==="set"&&row.key==="b"));
 expect(rows.find(row=>row.op==="set"&&row.key==="b")).toEqual({op:"set",key:"b",needMegabytes:7000,alone:true});
 freeA();await a;
 await until(()=>rows.at(-1)?.op==="set"&&rows.at(-1)?.key==="b");
 freeB();await b;
 expect(rows.at(-1)).toEqual({op:"clear"});
 governor.dispose();
});

test("a waiting owner without a solo limit claims what it needs but not alone, and the claim goes when it is admitted and ends",async()=>{
 const {rows,ask}=port();
 const governor=createMemoryGovernorV1({workers:2,headroom:()=>1500*MB,estimate:key=>({a:1000,c:900}[key as "a"|"c"])*MB,ask});
 let freeA!:()=>void,freeC!:()=>void;const holdA=new Promise<void>(done=>{freeA=done;}),holdC=new Promise<void>(done=>{freeC=done;});
 const a=governor.run(async()=>{await holdA;},{kind:"k",key:"a"});
 const c=governor.run(async()=>{await holdC;},{kind:"k",key:"c"});
 await until(()=>rows.some(row=>row.op==="set"&&row.key==="c"));
 expect(rows.find(row=>row.op==="set"&&row.key==="c")).toEqual({op:"set",key:"c",needMegabytes:900,alone:false});
 freeA();freeC();await Promise.all([a,c]);
 expect(rows.at(-1)).toEqual({op:"clear"});
 governor.dispose();
});

test("a governor without a port runs as before, and a port that throws never fails a run",async()=>{
 const plain=createMemoryGovernorV1({workers:1,headroom:()=>2000*MB});
 expect(await plain.run(async()=>7,{kind:"k",key:"x"})).toBe(7);plain.dispose();
 const broken=createMemoryGovernorV1({workers:1,headroom:()=>2000*MB,ask:{set:()=>{throw Error("disk full");},clear:()=>{throw Error("disk full");}}});
 expect(await broken.run(async()=>8,{kind:"k",key:"y"})).toBe(8);broken.dispose();
});
